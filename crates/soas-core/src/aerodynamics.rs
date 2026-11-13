//! Quasi-steady v0 aerodynamics. Equations: TASKS.md T6 and
//! data/reference/aerodynamics/README.md. No aircraft parameters are built in.
//! D6/D12/D13 specify signs, frames and the low-speed policy.

use crate::{
    aircraft::{AeroCoefficients, Aircraft, AircraftError, CoefficientTable},
    atmosphere::{AtmosphereError, dynamic_pressure_pa, isa_at_geometric_altitude, mach_number},
    dynamics::{DynamicsError, FixedStepRk4, Loads, State, StepError},
    units::Radians,
};
use nalgebra::Vector3;
use std::{error::Error, f64::consts::PI, fmt};

/// D13 owner-selected placeholder; revisit with evidence, not aircraft data.
pub const MIN_AIRSPEED_FOR_ANGLES_MPS: f64 = 0.1;
/// v0.1 envelope in TASKS.md / CONVENTIONS.md; tables do not extend it.
pub const MAX_MODELED_MACH: f64 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Controls {
    pub elevator_rad: Radians,
    pub aileron_rad: Radians,
    pub rudder_rad: Radians,
}

impl Controls {
    pub fn neutral() -> Self {
        let zero = Radians::new(0.0).expect("zero is finite");
        Self {
            elevator_rad: zero,
            aileron_rad: zero,
            rudder_rad: zero,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AirAngles {
    pub airspeed_mps: f64,
    pub alpha_rad: Radians,
    pub beta_rad: Radians,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AeroOutput {
    pub loads: Loads,
    pub angles: AirAngles,
    pub mach: f64,
    pub dynamic_pressure_pa: f64,
    pub cl: f64,
    pub cd: f64,
    pub lift_n: f64,
    pub drag_n: f64,
    pub side_n: f64,
    /// Upper lift limit reached/exceeded; not a model of post-stall motion.
    pub stalled: bool,
}

#[derive(Debug)]
pub enum AeroError {
    Aircraft(AircraftError),
    Atmosphere(AtmosphereError),
    Dynamics(DynamicsError),
    Invalid(&'static str),
    TableRange { axis: &'static str, value: f64 },
    MachNotModeled(f64),
}

impl fmt::Display for AeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aircraft(error) => error.fmt(f),
            Self::Atmosphere(error) => error.fmt(f),
            Self::Dynamics(error) => error.fmt(f),
            Self::Invalid(reason) => write!(f, "aerodynamics: {reason}"),
            Self::TableRange { axis, value } => write!(
                f,
                "aerodynamics: {axis}={value} outside coefficient table range"
            ),
            Self::MachNotModeled(mach) => write!(f, "Mach > 0.8 not modeled (Mach={mach})"),
        }
    }
}

impl Error for AeroError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Aircraft(error) => Some(error),
            Self::Atmosphere(error) => Some(error),
            Self::Dynamics(error) => Some(error),
            _ => None,
        }
    }
}

pub fn air_angles(velocity_body_mps: Vector3<f64>) -> Result<AirAngles, AeroError> {
    finite_vector(velocity_body_mps)?; // Before fallback or any clamping (D13).
    let v = velocity_body_mps;
    // hypot avoids overflowing intermediate squares for large finite inputs.
    let speed = v.x.hypot(v.y).hypot(v.z);
    finite([speed])?;
    let (alpha, beta) = if speed < MIN_AIRSPEED_FOR_ANGLES_MPS {
        (angle(0.0)?, angle(0.0)?)
    } else {
        (angle(v.z.atan2(v.x))?, beta_from_ratio(v.y / speed)?)
    };
    Ok(AirAngles {
        airspeed_mps: speed,
        alpha_rad: alpha,
        beta_rad: beta,
    })
}

/// D13 asin guard, separated so roundoff beyond +/-1 can be tested directly.
/// A non-finite ratio is always an error, not a value to clamp.
pub fn beta_from_ratio(ratio: f64) -> Result<Radians, AeroError> {
    finite([ratio])?;
    angle(ratio.clamp(-1.0, 1.0).asin())
}

pub fn stability_to_body(
    alpha_rad: Radians,
    vector_stability: Vector3<f64>,
) -> Result<Vector3<f64>, AeroError> {
    finite_vector(vector_stability)?;
    let (s, c) = alpha_rad.value().sin_cos();
    let v = vector_stability;
    finite_vector(Vector3::new(c * v.x - s * v.z, v.y, s * v.x + c * v.z))
}

pub fn wind_to_body(
    alpha_rad: Radians,
    beta_rad: Radians,
    vector_wind: Vector3<f64>,
) -> Result<Vector3<f64>, AeroError> {
    finite_vector(vector_wind)?;
    let (s, c) = beta_rad.value().sin_cos();
    let v = vector_wind;
    // Wind -> instantaneous stability is a +beta rotation about shared z.
    stability_to_body(
        alpha_rad,
        Vector3::new(c * v.x - s * v.y, s * v.x + c * v.y, v.z),
    )
}

/// Linear Mach / bilinear Mach-geometric-altitude interpolation. Exact nodes
/// return their stored values. Outside an explicit axis: error, never clamp.
pub fn lookup_coefficients(
    table: &CoefficientTable,
    mach: f64,
    geometric_altitude_m: f64,
) -> Result<AeroCoefficients, AeroError> {
    table.validate().map_err(AeroError::Aircraft)?;
    finite([mach, geometric_altitude_m])?;
    if geometric_altitude_m < 0.0 {
        return Err(AeroError::Invalid("geometric altitude must be nonnegative"));
    }
    let (m0, m1, mix_m) = bracket(&table.mach_nodes, mach, "Mach")?;
    let (h0, h1, mix_h) = if let Some(nodes) = &table.geometric_altitude_nodes_m {
        bracket(nodes, geometric_altitude_m, "geometric altitude (m)")?
    } else {
        (0, 0, 0.0)
    };
    let low = interpolate(
        table.coefficients[h0][m0],
        table.coefficients[h0][m1],
        mix_m,
    );
    let high = interpolate(
        table.coefficients[h1][m0],
        table.coefficients[h1][m1],
        mix_m,
    );
    let result = interpolate(low, high, mix_h);
    // Convex interpolation preserves sign/positive bounds; still reject any
    // numerical overflow instead of returning a usable invalid coefficient.
    result
        .validate("interpolated coefficients")
        .map_err(AeroError::Aircraft)?;
    Ok(result)
}

fn bracket(
    nodes: &[f64],
    value: f64,
    axis: &'static str,
) -> Result<(usize, usize, f64), AeroError> {
    if value < nodes[0] || value > nodes[nodes.len() - 1] {
        return Err(AeroError::TableRange { axis, value });
    }
    // Numeric equality treats -0.0 and +0.0 as the same physical node. A total
    // float ordering distinguishes them and could report an insertion before
    // the first node even though the range check accepted the value.
    match nodes.binary_search_by(|node| {
        node.partial_cmp(&value)
            .expect("finite validated nodes and input")
    }) {
        Ok(i) => Ok((i, i, 0.0)),
        Err(upper) => {
            let lower = upper - 1;
            Ok((
                lower,
                upper,
                (value - nodes[lower]) / (nodes[upper] - nodes[lower]),
            ))
        }
    }
}

fn interpolate(a: AeroCoefficients, b: AeroCoefficients, t: f64) -> AeroCoefficients {
    if t == 0.0 {
        return a;
    }
    if t == 1.0 {
        return b;
    }
    // Weighted form avoids overflowing b-a for finite opposite-sign endpoints.
    let mix = |x: f64, y: f64| (1.0 - t) * x + t * y;
    AeroCoefficients {
        cl0: mix(a.cl0, b.cl0),
        cl_alpha_per_rad: mix(a.cl_alpha_per_rad, b.cl_alpha_per_rad),
        cl_max: mix(a.cl_max, b.cl_max),
        cd0: mix(a.cd0, b.cd0),
        oswald_efficiency: mix(a.oswald_efficiency, b.oswald_efficiency),
        cm0: mix(a.cm0, b.cm0),
        cm_alpha_per_rad: mix(a.cm_alpha_per_rad, b.cm_alpha_per_rad),
        cm_elevator_per_rad: mix(a.cm_elevator_per_rad, b.cm_elevator_per_rad),
        cy_beta_per_rad: mix(a.cy_beta_per_rad, b.cy_beta_per_rad),
        roll_aileron_per_rad: mix(a.roll_aileron_per_rad, b.roll_aileron_per_rad),
        yaw_rudder_per_rad: mix(a.yaw_rudder_per_rad, b.yaw_rudder_per_rad),
    }
}

/// Evaluate the aircraft at a rigid-body state. No wind in v0.1, so the body
/// translational velocity equals air-relative velocity. Altitude is GEOMETRIC
/// h=-z; local sound speed supplies Mach, then Mach and h index the table.
pub fn evaluate(
    aircraft: &Aircraft,
    state: &State,
    controls: Controls,
) -> Result<AeroOutput, AeroError> {
    aircraft.validate().map_err(AeroError::Aircraft)?;
    finite_vector(state.position_ned_m)?;
    finite_vector(state.angular_rates_body_radps)?;
    let norm = state.attitude_body_to_ned.quaternion().norm();
    if !norm.is_finite() || (norm - 1.0).abs() > 64.0 * f64::EPSILON {
        return Err(AeroError::Dynamics(DynamicsError::InvalidAttitude));
    }
    let angles = air_angles(state.velocity_body_mps)?;
    let height_m = -state.position_ned_m.z;
    let atmosphere = isa_at_geometric_altitude(height_m).map_err(AeroError::Atmosphere)?;
    let mach = mach_number(angles.airspeed_mps, atmosphere.speed_of_sound_mps)
        .map_err(AeroError::Atmosphere)?;
    if mach > MAX_MODELED_MACH {
        return Err(AeroError::MachNotModeled(mach));
    }
    let c = lookup_coefficients(&aircraft.aerodynamics, mach, height_m)?;
    let q = dynamic_pressure_pa(atmosphere.density_kg_m3, angles.airspeed_mps)
        .map_err(AeroError::Atmosphere)?;
    let q_s = q * aircraft.reference_area_m2;
    let ar = aircraft.reference_span_m * aircraft.reference_span_m / aircraft.reference_area_m2;
    let raw_cl = c.cl0 + c.cl_alpha_per_rad * angles.alpha_rad.value();
    finite([q_s, ar, raw_cl])?;
    if ar <= 0.0 {
        return Err(AeroError::Invalid("aspect ratio underflowed"));
    }
    // T6 specifies an UPPER CLmax limit. No negative-stall law or fitted
    // post-stall curve is invented; large-angle flight remains unvalidated.
    let cl = raw_cl.min(c.cl_max);
    let induced_drag_denominator = PI * c.oswald_efficiency * ar;
    finite([induced_drag_denominator])?;
    if induced_drag_denominator <= 0.0 {
        return Err(AeroError::Invalid("induced-drag denominator underflowed"));
    }
    let cd = c.cd0 + cl * cl / induced_drag_denominator;
    let cy = c.cy_beta_per_rad * angles.beta_rad.value();
    let cm = c.cm0
        + c.cm_alpha_per_rad * angles.alpha_rad.value()
        + c.cm_elevator_per_rad * controls.elevator_rad.value();
    let roll = c.roll_aileron_per_rad * controls.aileron_rad.value();
    let yaw = c.yaw_rudder_per_rad * controls.rudder_rad.value();
    let (lift_n, drag_n, side_n) = (q_s * cl, q_s * cd, q_s * cy);
    finite([cl, cd, cy, cm, roll, yaw, lift_n, drag_n, side_n])?;
    let force = wind_to_body(
        angles.alpha_rad,
        angles.beta_rad,
        Vector3::new(-drag_n, side_n, -lift_n),
    )?;
    let moment_at_reference = Vector3::new(
        q_s * aircraft.reference_span_m * roll,
        q_s * aircraft.reference_chord_m * cm,
        q_s * aircraft.reference_span_m * yaw,
    );
    let reference = Vector3::from(aircraft.aerodynamic_reference_point_body_m);
    let cg = Vector3::from(aircraft.cg_body_m);
    // Newton's moment transfer: r runs FROM the CG TO the force application point.
    let moment = finite_vector(moment_at_reference + (reference - cg).cross(&force))?;
    Ok(AeroOutput {
        loads: Loads {
            force_body_n: force,
            moment_body_nm: moment,
        },
        angles,
        mach,
        dynamic_pressure_pa: q,
        cl,
        cd,
        lift_n,
        drag_n,
        side_n,
        stalled: raw_cl >= c.cl_max,
    })
}

/// Advance with the same aerodynamic model/atmosphere at every RK4 stage.
/// Any stage error aborts the step. Mass properties always come from this file.
pub fn step_aircraft(
    aircraft: &Aircraft,
    state: &State,
    controls: Controls,
    step_s: f64,
) -> Result<State, AeroError> {
    aircraft.validate().map_err(AeroError::Aircraft)?;
    let mass = aircraft.mass_properties().map_err(AeroError::Aircraft)?;
    let integrator = FixedStepRk4::new(mass, step_s).map_err(AeroError::Dynamics)?;
    let next = integrator
        .try_step(state, |stage| {
            evaluate(aircraft, stage, controls).map(|output| output.loads)
        })
        .map_err(|error| match error {
            StepError::Dynamics(error) => AeroError::Dynamics(error),
            StepError::Loads(error) => error,
        })?;
    // The weighted final state is distinct from the last trial stage. Check
    // its flight envelope too, so callers never receive an unsupported state.
    evaluate(aircraft, &next, controls)?;
    Ok(next)
}

fn angle(value: f64) -> Result<Radians, AeroError> {
    Radians::new(value).map_err(|_| AeroError::Invalid("non-finite angle"))
}
fn finite<const N: usize>(values: [f64; N]) -> Result<(), AeroError> {
    if values.into_iter().all(f64::is_finite) {
        Ok(())
    } else {
        Err(AeroError::Invalid("non-finite input or calculation"))
    }
}
fn finite_vector(vector: Vector3<f64>) -> Result<Vector3<f64>, AeroError> {
    finite([vector.x, vector.y, vector.z])?;
    Ok(vector)
}
