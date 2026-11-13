//! Version-zero JSON aircraft descriptions. Pure text in/text out: filesystem
//! access belongs to a future runner, not soas-core. Contract: docs/AIRCRAFT_FORMAT.md.
//! Serialization API: https://docs.rs/serde_json/1/serde_json/;
//! strict fields: https://serde.rs/container-attrs.html#deny_unknown_fields.

use crate::dynamics::MassProperties;
use nalgebra::Matrix3;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

pub const FORMAT_VERSION: u32 = 0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Aircraft {
    pub format_version: u32,
    pub name: String,
    pub data_note: String,
    pub mass_kg: f64,
    pub cg_body_m: [f64; 3],
    /// Signed tensor about the CG, expressed in body axes, row then column.
    pub inertia_body_kg_m2: [[f64; 3]; 3],
    pub reference_area_m2: f64,
    pub reference_span_m: f64,
    pub reference_chord_m: f64,
    /// Relative to the SAME file datum as cg_body_m; not a CG-relative offset.
    pub aerodynamic_reference_point_body_m: [f64; 3],
    pub coefficient_axes: String,
    pub moments_about: String,
    pub aerodynamics: CoefficientTable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoefficientTable {
    pub mach_nodes: Vec<f64>,
    /// None: Mach-only table, with exactly one row, independent of altitude.
    pub geometric_altitude_nodes_m: Option<Vec<f64>>,
    /// Outer index is altitude; inner index is Mach. No unordered maps.
    pub coefficients: Vec<Vec<AeroCoefficients>>,
}

/// Dimensionless coefficients; all angle derivatives are per RADIAN.
/// CL/CD/CY use wind axes, moments use body axes about the aerodynamic point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AeroCoefficients {
    pub cl0: f64,
    pub cl_alpha_per_rad: f64,
    pub cl_max: f64,
    pub cd0: f64,
    pub oswald_efficiency: f64,
    pub cm0: f64,
    pub cm_alpha_per_rad: f64,
    pub cm_elevator_per_rad: f64,
    pub cy_beta_per_rad: f64,
    pub roll_aileron_per_rad: f64,
    pub yaw_rudder_per_rad: f64,
}

#[derive(Debug)]
pub enum AircraftError {
    Json(serde_json::Error),
    Validation { field: String, reason: &'static str },
}

impl fmt::Display for AircraftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(f, "aircraft JSON: {error}"),
            Self::Validation { field, reason } => write!(f, "aircraft {field}: {reason}"),
        }
    }
}

impl Error for AircraftError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Validation { .. } => None,
        }
    }
}

pub fn from_json(text: &str) -> Result<Aircraft, AircraftError> {
    let aircraft: Aircraft = serde_json::from_str(text).map_err(AircraftError::Json)?;
    aircraft.validate()?;
    Ok(aircraft)
}

pub fn to_json(aircraft: &Aircraft) -> Result<String, AircraftError> {
    // serde_json serializes some non-finite numbers as null; reject them BEFORE
    // serialization so an invalid in-memory model cannot silently change value.
    aircraft.validate()?;
    serde_json::to_string_pretty(aircraft).map_err(AircraftError::Json)
}

impl Aircraft {
    pub fn validate(&self) -> Result<(), AircraftError> {
        require(
            self.format_version == FORMAT_VERSION,
            "format_version",
            "only version 0 is supported",
        )?;
        require(!self.name.trim().is_empty(), "name", "must not be empty")?;
        require(
            !self.data_note.trim().is_empty(),
            "data_note",
            "must identify provenance or synthetic test data",
        )?;
        positive(self.mass_kg, "mass_kg")?;
        positive(self.reference_area_m2, "reference_area_m2")?;
        positive(self.reference_span_m, "reference_span_m")?;
        positive(self.reference_chord_m, "reference_chord_m")?;
        finite(self.cg_body_m.into_iter(), "cg_body_m")?;
        finite(
            self.aerodynamic_reference_point_body_m.into_iter(),
            "aerodynamic_reference_point_body_m",
        )?;
        self.mass_properties()?;
        require(
            self.coefficient_axes == "wind_forces_body_moments",
            "coefficient_axes",
            "v0 requires wind_forces_body_moments (D12)",
        )?;
        require(
            self.moments_about == "aerodynamic_reference_point",
            "moments_about",
            "v0 requires aerodynamic_reference_point",
        )?;
        self.aerodynamics.validate()
    }

    pub fn mass_properties(&self) -> Result<MassProperties, AircraftError> {
        positive(self.mass_kg, "mass_kg")?;
        let i = self.inertia_body_kg_m2;
        let tensor = Matrix3::new(
            i[0][0], i[0][1], i[0][2], i[1][0], i[1][1], i[1][2], i[2][0], i[2][1], i[2][2],
        );
        MassProperties::from_signed_tensor(self.mass_kg, tensor).map_err(|_| {
            validation(
                "inertia_body_kg_m2",
                "must be finite, symmetric and positive definite about the CG in body axes",
            )
        })
    }
}

impl CoefficientTable {
    pub fn validate(&self) -> Result<(), AircraftError> {
        validate_nodes(&self.mach_nodes, "aerodynamics.mach_nodes")?;
        let expected_rows = if let Some(nodes) = &self.geometric_altitude_nodes_m {
            validate_nodes(nodes, "aerodynamics.geometric_altitude_nodes_m")?;
            nodes.len()
        } else {
            1
        };
        require(
            self.coefficients.len() == expected_rows,
            "aerodynamics.coefficients",
            "row count must equal altitude-node count (one for a Mach-only table)",
        )?;
        for (row_index, row) in self.coefficients.iter().enumerate() {
            let path = format!("aerodynamics.coefficients[{row_index}]");
            require(
                row.len() == self.mach_nodes.len(),
                &path,
                "column count must equal Mach-node count",
            )?;
            for (column, coefficients) in row.iter().enumerate() {
                coefficients.validate(&format!("{path}[{column}]"))?;
            }
        }
        Ok(())
    }
}

impl AeroCoefficients {
    pub(crate) fn validate(&self, path: &str) -> Result<(), AircraftError> {
        for (field, value) in self.fields() {
            finite([value].into_iter(), &format!("{path}.{field}"))?;
        }
        for (field, value) in [
            ("cl_alpha_per_rad", self.cl_alpha_per_rad),
            ("cl_max", self.cl_max),
            ("oswald_efficiency", self.oswald_efficiency),
        ] {
            positive(value, &format!("{path}.{field}"))?;
        }
        require(
            self.cd0 >= 0.0,
            &format!("{path}.cd0"),
            "must be nonnegative",
        )?;
        for (field, correct_sign) in [
            ("cm_elevator_per_rad", self.cm_elevator_per_rad <= 0.0),
            ("roll_aileron_per_rad", self.roll_aileron_per_rad >= 0.0),
            ("yaw_rudder_per_rad", self.yaw_rudder_per_rad <= 0.0),
        ] {
            require(
                correct_sign,
                &format!("{path}.{field}"),
                "sign contradicts owner decision D6",
            )?;
        }
        Ok(())
    }

    fn fields(&self) -> [(&'static str, f64); 11] {
        [
            ("cl0", self.cl0),
            ("cl_alpha_per_rad", self.cl_alpha_per_rad),
            ("cl_max", self.cl_max),
            ("cd0", self.cd0),
            ("oswald_efficiency", self.oswald_efficiency),
            ("cm0", self.cm0),
            ("cm_alpha_per_rad", self.cm_alpha_per_rad),
            ("cm_elevator_per_rad", self.cm_elevator_per_rad),
            ("cy_beta_per_rad", self.cy_beta_per_rad),
            ("roll_aileron_per_rad", self.roll_aileron_per_rad),
            ("yaw_rudder_per_rad", self.yaw_rudder_per_rad),
        ]
    }
}

fn validate_nodes(nodes: &[f64], field: &str) -> Result<(), AircraftError> {
    require(!nodes.is_empty(), field, "must contain at least one node")?;
    finite(nodes.iter().copied(), field)?;
    require(
        nodes.iter().all(|v| *v >= 0.0) && nodes.windows(2).all(|pair| pair[0] < pair[1]),
        field,
        "must be nonnegative and strictly increasing",
    )
}

fn positive(value: f64, field: &str) -> Result<(), AircraftError> {
    require(
        value.is_finite() && value > 0.0,
        field,
        "must be finite and positive",
    )
}

fn finite(values: impl Iterator<Item = f64>, field: &str) -> Result<(), AircraftError> {
    require(
        values.into_iter().all(f64::is_finite),
        field,
        "must contain only finite values",
    )
}

fn require(condition: bool, field: &str, reason: &'static str) -> Result<(), AircraftError> {
    if condition {
        Ok(())
    } else {
        Err(validation(field, reason))
    }
}

fn validation(field: &str, reason: &'static str) -> AircraftError {
    AircraftError::Validation {
        field: field.to_owned(),
        reason,
    }
}
