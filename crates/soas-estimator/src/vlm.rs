//! Linear planar-wing vortex lattice. Formulas/signs, comparison budget and
//! scope: data/reference/estimator/vlm.md. Runtime remains pure.
use crate::{
    planform::{WingPlanform, thin_airfoil_slope_per_rad},
    vortex::Horseshoe,
};
use nalgebra::{DMatrix, DVector, Vector3};
use soas_core::units::Radians;
use std::{error::Error, f64::consts::PI, fmt};

const ASSUMPTIONS: &[&str] = &[
    "planar wing",
    "incompressible flow",
    "attached flow",
    "small angles",
    "ideal thin-airfoil 2*pi section slope",
    "flat wake along linearized freestream",
    "no thickness/profile drag",
    "zero intrinsic section pitching moment",
];
/// Full-span edge distribution. Cosine is the default because the T15
/// diagnosis resolves wing-tip loading better at equal panel count; it is a
/// numerical choice, not a physical validity criterion. AVL primer, spacing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpanwiseSpacing {
    Uniform,
    #[default]
    Cosine,
}
#[derive(Debug, Clone, Copy)]
pub struct VlmRequest {
    /// Count over the FULL wing, including both mirrored halves.
    pub spanwise_panels: usize,
    pub chordwise_panels: usize,
    pub spanwise_spacing: SpanwiseSpacing,
    pub alpha_rad: Radians,
    /// Centerline reference x; y=0, z=wing reference plane.
    pub reference_x_body_m: f64,
}
#[derive(Debug, Clone)]
pub struct VlmPanel {
    pub spanwise_index: usize,
    pub chordwise_index: usize,
    pub bound_start_m: Vector3<f64>,
    pub bound_end_m: Vector3<f64>,
    pub control_point_m: Vector3<f64>,
    pub circulation_over_airspeed_m: f64,
}
#[derive(Debug, Clone)]
pub struct VlmOutput {
    pub cl: f64,
    pub induced_drag_coefficient: f64,
    pub pitching_moment_coefficient: f64,
    pub rolling_moment_coefficient: f64,
    pub reference_point_body_m: [f64; 3],
    pub d_cl_d_alpha_per_rad: f64,
    pub d_cm_d_alpha_per_rad: f64,
    pub neutral_point_x_body_m: f64,
    pub panels: Vec<VlmPanel>,
    pub relative_collocation_residual: f64,
    pub drag_method: &'static str,
    pub assumptions: &'static [&'static str],
    pub validity: &'static str,
}
#[derive(Debug, Clone)]
pub struct VlmError(pub String);
impl fmt::Display for VlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VLM: {}", self.0)
    }
}
impl Error for VlmError {}
fn require(ok: bool, reason: &str) -> Result<(), VlmError> {
    if ok {
        Ok(())
    } else {
        Err(VlmError(reason.into()))
    }
}
fn residual(matrix: &DMatrix<f64>, x: &DVector<f64>, rhs: &DVector<f64>) -> Result<f64, VlmError> {
    // Netlib Templates Criterion 1, same normwise check as T13.
    let max_abs = |v: &DVector<f64>| v.iter().map(|x| x.abs()).fold(0.0, f64::max);
    let matrix_norm = (0..matrix.nrows())
        .map(|i| matrix.row(i).iter().map(|v| v.abs()).sum::<f64>())
        .fold(0.0, f64::max);
    let r = matrix * x - rhs;
    let denominator = matrix_norm * max_abs(x) + max_abs(rhs);
    require(denominator.is_finite(), "linear residual scaling overflow")?;
    let value = if denominator == 0.0 {
        max_abs(&r)
    } else {
        max_abs(&r) / denominator
    };
    require(
        value.is_finite() && value <= 128.0 * f64::EPSILON * matrix.nrows() as f64,
        "linear solve failed numerical residual check",
    )?;
    Ok(value)
}
/// Selectable span edges, uniform chord panels; quarter-bound and three-quarter
/// controls are fractions of each panel, not of the whole wing. See Mason 6.7,
/// Drela JVL 3.1. Free airflow is -body x, with normal component -alpha.
pub fn solve(wing: &WingPlanform, request: VlmRequest) -> Result<VlmOutput, VlmError> {
    require(
        request.spanwise_panels > 0 && request.chordwise_panels > 0,
        "both panel counts must be positive",
    )?;
    require(
        request.reference_x_body_m.is_finite(),
        "reference x must be finite",
    )?;
    let count = request
        .spanwise_panels
        .checked_mul(request.chordwise_panels)
        .ok_or_else(|| VlmError("panel count overflow".into()))?;
    let cells = count
        .checked_mul(count)
        .ok_or_else(|| VlmError("influence matrix size overflow".into()))?;
    let geometry = wing.geometry().map_err(|e| VlmError(e.to_string()))?;
    let z = wing.stations[0].z_m;
    let a0 = thin_airfoil_slope_per_rad();
    for station in &wing.stations {
        let scale = geometry.span_m.max(z.abs()).max(station.z_m.abs());
        require(
            (station.z_m - z).abs() <= 64.0 * f64::EPSILON * scale,
            "requires a planar wing; dihedral/nonplanarity not modeled",
        )?;
        require(
            (station.section_lift_curve_slope_per_rad - a0).abs() <= 64.0 * f64::EPSILON * a0,
            "classical quarter/three-quarter VLM requires ideal 2*pi section slope; no slope correction implemented",
        )?;
    }
    let ny = request.spanwise_panels;
    let nx = request.chordwise_panels;
    let half_span = geometry.span_m / 2.0;
    let mut edges = Vec::with_capacity(ny + 1);
    for j in 0..=ny {
        let y = if j == 0 {
            -half_span
        } else if j == ny {
            half_span
        } else if 2 * j == ny {
            0.0
        } else {
            match request.spanwise_spacing {
                SpanwiseSpacing::Cosine => -half_span * (PI * j as f64 / ny as f64).cos(),
                SpanwiseSpacing::Uniform => -half_span + geometry.span_m * j as f64 / ny as f64,
            }
        };
        edges.push(y);
    }
    let mut panels = Vec::new();
    panels
        .try_reserve_exact(count)
        .map_err(|_| VlmError("unable to allocate panels".into()))?;
    let mut rhs_values = Vec::with_capacity(count);
    for strip in 0..ny {
        let left = wing
            .sample(edges[strip])
            .map_err(|e| VlmError(e.to_string()))?;
        let right = wing
            .sample(edges[strip + 1])
            .map_err(|e| VlmError(e.to_string()))?;
        let width = right.y_m - left.y_m;
        require(
            width > 0.0 && width.is_finite(),
            "unresolved span panel width",
        )?;
        let mid_y = 0.5 * left.y_m + 0.5 * right.y_m;
        let section = wing.sample(mid_y).map_err(|e| VlmError(e.to_string()))?;
        for chord in 0..nx {
            let bound_fraction = (chord as f64 + 0.25) / nx as f64;
            let control_fraction = (chord as f64 + 0.75) / nx as f64;
            let a = Vector3::new(
                left.leading_edge_x_m - bound_fraction * left.chord_m,
                left.y_m,
                z,
            );
            let b = Vector3::new(
                right.leading_edge_x_m - bound_fraction * right.chord_m,
                right.y_m,
                z,
            );
            let mid_le = 0.5 * left.leading_edge_x_m + 0.5 * right.leading_edge_x_m;
            let mid_chord = 0.5 * left.chord_m + 0.5 * right.chord_m;
            let cp = Vector3::new(mid_le - control_fraction * mid_chord, mid_y, z);
            require(
                a.iter()
                    .chain(b.iter())
                    .chain(cp.iter())
                    .all(|v| v.is_finite()),
                "panel coordinate overflow",
            )?;
            let incidence = request.alpha_rad.value() + section.twist_rad.value()
                - section.zero_lift_angle_rad.value();
            require(incidence.is_finite(), "section incidence overflow")?;
            // Linearized effective normal=(twist-alpha_L0,0,1), airflow
            // (-1,0,-alpha), planar induced velocity only in z -> W*gamma=incidence.
            rhs_values.push(incidence);
            panels.push(VlmPanel {
                spanwise_index: strip,
                chordwise_index: chord,
                bound_start_m: a,
                bound_end_m: b,
                control_point_m: cp,
                circulation_over_airspeed_m: 0.0,
            });
        }
    }
    let mut entries = Vec::new();
    entries
        .try_reserve_exact(cells)
        .map_err(|_| VlmError("unable to allocate influence matrix".into()))?;
    entries.resize(cells, 0.0);
    for (j, p) in panels.iter().enumerate() {
        let h = Horseshoe {
            bound_start_m: p.bound_start_m,
            bound_end_m: p.bound_end_m,
            wake_direction_unit: -Vector3::x(),
        };
        for (i, control) in panels.iter().enumerate() {
            let v = h
                .velocity_per_circulation(control.control_point_m)
                .map_err(|e| VlmError(format!("influence[{i}][{j}]: {e}")))?;
            entries[i + j * count] = v.z;
        }
    }
    let matrix = DMatrix::from_vec(count, count, entries);
    let rhs = DVector::from_vec(rhs_values);
    let lu = matrix.clone().lu();
    let gamma = lu
        .solve(&rhs)
        .ok_or_else(|| VlmError("singular circulation system".into()))?;
    // Geometry/fixed linearized wake do not depend on alpha, so differentiation
    // is exact: W*d(gamma)/dalpha=1. No finite-difference step is invented.
    let derivative_rhs = DVector::from_element(count, 1.0);
    let gamma_alpha = lu
        .solve(&derivative_rhs)
        .ok_or_else(|| VlmError("singular circulation derivative system".into()))?;
    require(
        gamma
            .iter()
            .chain(gamma_alpha.iter())
            .all(|g| g.is_finite()),
        "non-finite circulation/derivative",
    )?;
    let relative_residual =
        residual(&matrix, &gamma, &rhs)?.max(residual(&matrix, &gamma_alpha, &derivative_rhs)?);
    let mut lift = 0.0;
    let mut pitch = 0.0;
    let mut roll = 0.0;
    let mut lift_alpha = 0.0;
    let mut pitch_alpha = 0.0;
    let mut strip_gamma = vec![0.0; ny];
    for (i, p) in panels.iter_mut().enumerate() {
        p.circulation_over_airspeed_m = gamma[i];
        let dy = p.bound_end_m.y - p.bound_start_m.y;
        let x = 0.5 * p.bound_start_m.x + 0.5 * p.bound_end_m.x;
        let y = 0.5 * p.bound_start_m.y + 0.5 * p.bound_end_m.y;
        // First-order Kutta-Joukowski: Fz/(rho*V^2)=-gamma*dy.
        // r x F gives My=(x-xref)*gamma*dy, Mx=-y*gamma*dy.
        lift += gamma[i] * dy;
        pitch += (x - request.reference_x_body_m) * gamma[i] * dy;
        roll -= y * gamma[i] * dy;
        lift_alpha += gamma_alpha[i] * dy;
        pitch_alpha += (x - request.reference_x_body_m) * gamma_alpha[i] * dy;
        strip_gamma[p.spanwise_index] += gamma[i];
    }
    // Drela JVL eqs 123-127: sum chordwise horseshoes into strip circulation;
    // the far wake has twice the wing-plane downwash. Midpoint energy drag
    // D/(rho*V^2)=.5*sum gamma*w_far*dy -> CDi=sum(...)/S.
    let mut drag = 0.0;
    for i in 0..ny {
        let y = 0.5 * edges[i] + 0.5 * edges[i + 1];
        let dy = edges[i + 1] - edges[i];
        let w_far = (0..ny)
            .map(|j| {
                strip_gamma[j] * (1.0 / (y - edges[j]) - 1.0 / (y - edges[j + 1])) / (2.0 * PI)
            })
            .sum::<f64>();
        drag += strip_gamma[i] * w_far * dy;
    }
    let area = geometry.area_m2;
    let mac = geometry.mean_aerodynamic_chord_m;
    let cl = 2.0 * lift / area;
    let cm = 2.0 * pitch / (area * mac);
    let cr = 2.0 * roll / (area * geometry.span_m);
    let cd = drag / area;
    let cla = 2.0 * lift_alpha / area;
    let cma = 2.0 * pitch_alpha / (area * mac);
    require(cla != 0.0, "neutral point undefined: unresolved lift slope")?;
    // Body x is FORWARD: Cm_ref=CL*(x_ac-x_ref)/MAC. Hence the plus sign.
    let neutral = request.reference_x_body_m + mac * cma / cla;
    require(
        [cl, cm, cr, cd, cla, cma, neutral]
            .into_iter()
            .all(f64::is_finite)
            && cd >= 0.0,
        "non-finite/negative coefficient or neutral point",
    )?;
    Ok(VlmOutput {
        cl,
        induced_drag_coefficient: cd,
        pitching_moment_coefficient: cm,
        rolling_moment_coefficient: cr,
        reference_point_body_m: [request.reference_x_body_m, 0.0, z],
        d_cl_d_alpha_per_rad: cla,
        d_cm_d_alpha_per_rad: cma,
        neutral_point_x_body_m: neutral,
        panels,
        relative_collocation_residual: relative_residual,
        drag_method: "Trefftz plane strip-midpoint quadrature",
        assumptions: ASSUMPTIONS,
        validity: "validity unassessed",
    })
}
