//! Pure semantic inputs, not an aircraft estimator or a file format.
//! Contract, formula sources and limitations: docs/S3_01_SEMANTIC_MODEL.md.
mod numbers;
mod quantities;
mod validation;

pub use quantities::{ComputedWingQuantity, QuantityIdentity, SuppliedQuantityKind};

use soas_core::{propulsion::EngineTable, units::Radians};
use std::{collections::BTreeMap, error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProvenanceFlag {
    EstimatedWingOnly,
    UserSupplied,
    TemplateDefault,
    NotEstimated,
    OutsideCoverage,
    DerivedFromInputs,
}
impl ProvenanceFlag {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EstimatedWingOnly => "ESTIMATED_WING_ONLY",
            Self::UserSupplied => "USER_SUPPLIED",
            Self::TemplateDefault => "TEMPLATE_DEFAULT",
            Self::NotEstimated => "NOT_ESTIMATED",
            Self::OutsideCoverage => "OUTSIDE_COVERAGE",
            Self::DerivedFromInputs => "DERIVED_FROM_INPUTS",
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct NumberProvenance {
    pub flags: Vec<ProvenanceFlag>,
    pub unit: String,
    /// A citation, synthetic ledger or explicit missing-input explanation.
    pub source_refs: Vec<String>,
    pub input_paths: Vec<String>,
    pub geometry_revision: Option<String>,
    pub approximate: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct NumericField<'a> {
    pub path: String,
    pub value: Option<f64>,
    pub unit: &'a str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignIdentity {
    SyntheticTemplate,
    UserDesign,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Design {
    pub name: String,
    pub identity: DesignIdentity,
    /// No real-aircraft claim is admitted without a future validated-data contract.
    pub real_aircraft_claim: Option<String>,
    /// Every coordinate is in body x-forward/y-right/z-down at this common datum.
    pub datum: String,
    pub components: Vec<Component>,
    pub engine_tables: Vec<EngineTableDefinition>,
    pub controls: Vec<ControlSurface>,
    pub contributions: Vec<SuppliedContribution>,
    /// Parallel metadata; numeric fields remain SI primitives / existing angles.
    pub provenance: BTreeMap<String, NumberProvenance>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub geometry: ComponentGeometry,
    pub mass: MassInput,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentGeometry {
    Surface(Surface),
    Fuselage(Fuselage),
    Engine(EngineInstance),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceKind {
    Wing,
    HorizontalTail,
    VerticalTail,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Surface {
    pub kind: SurfaceKind,
    /// Reflect declared side about the common body x-z plane, not its own root.
    pub mirrored: bool,
    pub incidence_rad: Radians,
    pub stations: Vec<Station>,
}
/// Station identity must be supplied explicitly, never synthesized from position.
/// This example intentionally omits it and must not compile. All example
/// numbers are synthetic structural values: tests/fixtures/station_id_values.md.
///
/// ```compile_fail,E0063
/// use soas_core::units::Radians;
/// use soas_designer::Station;
/// let _missing_id = Station {
///     leading_edge_body_m: [0.0; 3],
///     chord_m: 1.0,
///     twist_rad: Radians::new(0.0).unwrap(),
///     airfoil: None,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Station {
    /// Opaque identity, unique within the surface. Root/tip are list positions.
    pub id: String,
    /// Defining coordinates, not derived span/area/MAC values.
    pub leading_edge_body_m: [f64; 3],
    pub chord_m: f64,
    pub twist_rad: Radians,
    pub airfoil: Option<AirfoilInput>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AirfoilInput {
    pub name: String,
    pub source: String,
    /// Explicit section inputs, not a supplied aggregate wing lift slope.
    pub lift_curve_slope_per_rad: Option<f64>,
    pub zero_lift_angle_rad: Option<Radians>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Fuselage {
    pub nose_body_m: [f64; 3],
    pub length_m: f64,
    /// Positive sampled sections; no inference of an aerodynamic loft or end cap.
    pub sections: Vec<BodySection>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BodySection {
    pub aft_m: f64,
    pub offset_y_m: f64,
    pub offset_z_m: f64,
    pub width_m: f64,
    pub height_m: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct EngineInstance {
    pub table_id: String,
    pub position_body_m: [f64; 3],
    /// Direction of force ON the aircraft, never exhaust direction.
    pub direction_body_unit: [f64; 3],
}
#[derive(Debug, Clone, PartialEq)]
pub struct EngineTableDefinition {
    pub id: String,
    pub source: String,
    pub dry_throttle_max: f64,
    pub table: EngineTable,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MassInput {
    pub mass_kg: Option<f64>,
    pub cg_body_m: Option<[f64; 3]>,
    pub intrinsic_inertia: Option<IntrinsicInertia>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct IntrinsicInertia {
    /// Signed entries about component CG in the explicitly oriented axes.
    pub tensor_kg_m2: [[f64; 3]; 3],
    /// Existing 3-2-1 roll/pitch/yaw convention; rotation is deferred to S3-03.
    pub axes_euler_rad: [Radians; 3],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlKind {
    Elevator,
    Aileron,
    Rudder,
}
impl ControlKind {
    /// Physical command definitions from CONVENTIONS, D6, not predicted loads.
    pub fn positive_deflection(self) -> &'static str {
        match self {
            Self::Elevator => "trailing edge down; nose-down moment",
            Self::Aileron => "left trailing edge down, right up; positive roll",
            Self::Rudder => "trailing edge left; nose-left yaw moment",
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ControlSurface {
    pub id: String,
    pub parent_surface_id: String,
    pub kind: ControlKind,
    pub hinge_start_body_m: [f64; 3],
    pub hinge_end_body_m: [f64; 3],
    pub chord_fraction: f64,
    pub min_rad: Radians,
    pub max_rad: Radians,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContributionScope {
    Tail,
    Fuselage,
    Interference,
    ClMax,
    /// Explicit invalid requests make validation reject attempted overrides.
    Wing,
    WholeAircraft,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SuppliedContribution {
    pub id: String,
    pub scope: ContributionScope,
    pub source: String,
    /// Required declared applicability, not a machine-certified coverage model.
    pub coverage: String,
    pub parameters: Vec<SuppliedParameter>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SuppliedParameter {
    /// Semantic identity is independent of the display label. No implicit default.
    pub quantity: QuantityIdentity,
    /// Free-text label; never selects kind, scope, units or allowed origin.
    pub name: String,
    pub value: Option<f64>,
    pub unit: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct EnginePlacement {
    pub position_body_m: [f64; 3],
    pub force_direction_body_unit: [f64; 3],
    pub table_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignError {
    pub field: String,
    pub reason: &'static str,
}
impl fmt::Display for DesignError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "design {}: {}", self.field, self.reason)
    }
}
impl Error for DesignError {}

impl Design {
    /// Return each explicitly defining point once. Reflection is (x,-y,z),
    /// CONVENTIONS Coordinate frames / estimator geometry reference.
    /// This is descriptive mirroring, not a mesh or aerodynamic adapter.
    pub fn surface_points_body_m(&self, id: &str) -> Result<Vec<[f64; 3]>, DesignError> {
        self.validate()?;
        let surface = self
            .components
            .iter()
            .find_map(|c| match &c.geometry {
                ComponentGeometry::Surface(s) if c.id == id => Some(s),
                _ => None,
            })
            .ok_or_else(|| validation::error(id, "unknown lifting surface"))?;
        let mut points: Vec<_> = surface
            .stations
            .iter()
            .map(|s| s.leading_edge_body_m)
            .collect();
        if surface.mirrored {
            for station in &surface.stations {
                let mut point = station.leading_edge_body_m;
                if point[1] != 0.0 {
                    point[1] = -point[1];
                    points.push(point);
                }
            }
        }
        Ok(points)
    }
    /// Preserve common-datum body coordinates and declared force direction.
    pub fn engine_placement(&self, id: &str) -> Result<EnginePlacement, DesignError> {
        self.validate()?;
        let engine = self
            .components
            .iter()
            .find_map(|c| match &c.geometry {
                ComponentGeometry::Engine(e) if c.id == id => Some(e),
                _ => None,
            })
            .ok_or_else(|| validation::error(id, "unknown engine instance"))?;
        Ok(EnginePlacement {
            position_body_m: engine.position_body_m,
            force_direction_body_unit: engine.direction_body_unit,
            table_id: engine.table_id.clone(),
        })
    }
}
