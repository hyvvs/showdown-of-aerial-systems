//! Local steady straight-flight trim and control-fixed static margin.
//! Sources/equations/limits: data/reference/trim/README.md. All load evaluation
//! reuses the actual table-driven T6 model and geometric-altitude atmosphere.
use crate::{
    aerodynamics::{AeroError, AeroOutput, Controls, MIN_AIRSPEED_FOR_ANGLES_MPS, evaluate},
    aircraft::Aircraft,
    atmosphere::STANDARD_GRAVITY_MPS2,
    attitude::{Euler321, from_euler321},
    dynamics::State,
    units::Radians,
};
use nalgebra::{Matrix3, Vector3};
use std::{error::Error, f64::consts::FRAC_PI_2, fmt};

#[derive(Debug, Clone, Copy)]
pub struct TrimRequest {
    pub airspeed_mps: f64,
    pub geometric_altitude_m: f64,
    /// Temporary constant body-x force through the CG, NOT a propulsion model.
    /// None means zero; the sign follows +body x. No fuel flow is implied.
    pub applied_thrust_n: Option<f64>,
}
#[derive(Debug, Clone, Copy)]
pub struct TrimCondition {
    pub alpha_rad: Radians,
    pub elevator_rad: Radians,
    pub gamma_rad: Radians,
}
#[derive(Debug, Clone, Copy)]
pub struct TrimOptions {
    pub initial: TrimCondition,
    pub max_iterations: usize,
    pub force_tolerance_n: f64,
    pub moment_tolerance_nm: f64,
    pub difference_step_rad: Radians,
}
impl Default for TrimOptions {
    fn default() -> Self {
        // Numerical settings, not physical aircraft parameters. See reference
        // documentation and derivative-step sensitivity tests.
        Self {
            initial: TrimCondition {
                alpha_rad: rad(0.05),
                elevator_rad: rad(0.0),
                gamma_rad: rad(0.0),
            },
            max_iterations: 50,
            force_tolerance_n: 1e-8,
            moment_tolerance_nm: 1e-8,
            difference_step_rad: rad(1e-5),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TrimResiduals {
    pub parallel_force_n: f64,
    pub normal_force_n: f64,
    pub pitch_moment_nm: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct TrimSolution {
    pub condition: TrimCondition,
    pub state: State,
    pub residuals: TrimResiduals,
    pub iterations: usize,
    /// Always explicit in the result, including the zero-thrust default.
    pub applied_thrust_n: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct StaticMargin {
    pub margin_chord_fraction: f64,
    pub d_cm_d_alpha_per_rad: f64,
    pub d_cl_d_alpha_per_rad: f64,
    pub difference_step_rad: Radians,
}
#[derive(Debug)]
pub enum TrimError {
    Invalid(&'static str),
    Model(AeroError),
    NonConvergence {
        iterations: usize,
        residuals: TrimResiduals,
        reason: &'static str,
    },
    UndefinedMargin(&'static str),
}
impl fmt::Display for TrimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(reason) => write!(f, "invalid trim input: {reason}"),
            Self::Model(e) => e.fmt(f),
            Self::NonConvergence {
                iterations,
                residuals,
                reason,
            } => write!(
                f,
                "trim non-convergence after {iterations} iterations: {reason}; parallel={} N, normal={} N, pitch={} Nm",
                residuals.parallel_force_n, residuals.normal_force_n, residuals.pitch_moment_nm
            ),
            Self::UndefinedMargin(reason) => write!(f, "static margin undefined: {reason}"),
        }
    }
}
impl Error for TrimError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Model(e) => Some(e),
            _ => None,
        }
    }
}

struct Evaluation {
    state: State,
    aero: AeroOutput,
    residuals: TrimResiduals,
}

/// Gamma is positive climb; body pitch = alpha + gamma. Solve the forward,
/// nonvertical branch |alpha|,|gamma| < pi/2, with zero bank/sideslip/body rates.
/// Return success ONLY when force and CG moment residuals meet the requested
/// tolerances. Singular Jacobian, failed line search or iteration exhaustion
/// returns an error, never a best-effort TrimSolution.
pub fn solve_trim(
    aircraft: &Aircraft,
    request: TrimRequest,
    options: TrimOptions,
) -> Result<TrimSolution, TrimError> {
    validate_request(request)?;
    if !options.force_tolerance_n.is_finite()
        || options.force_tolerance_n <= 0.0
        || !options.moment_tolerance_nm.is_finite()
        || options.moment_tolerance_nm <= 0.0
        || options.difference_step_rad.value() <= 0.0
        || options.difference_step_rad.value() >= FRAC_PI_2
    {
        return Err(TrimError::Invalid(
            "tolerances and difference step must be finite and positive, difference step below pi/2",
        ));
    }
    let mut x = vector(options.initial);
    if !forward_branch(x) {
        return Err(TrimError::Invalid(
            "initial alpha and gamma must be on the forward nonvertical branch",
        ));
    }
    let mut current = at_condition(aircraft, request, options.initial)?;
    let weight = aircraft.mass_kg * STANDARD_GRAVITY_MPS2;
    let moment_scale =
        current.aero.dynamic_pressure_pa * aircraft.reference_area_m2 * aircraft.reference_chord_m;
    if !weight.is_finite() || !moment_scale.is_finite() || weight <= 0.0 || moment_scale <= 0.0 {
        return Err(TrimError::Invalid(
            "force/moment scales are not finite and positive",
        ));
    }
    let scales = Vector3::new(weight, weight, moment_scale);
    for iteration in 0..=options.max_iterations {
        let r = current.residuals;
        if r.parallel_force_n.abs() <= options.force_tolerance_n
            && r.normal_force_n.abs() <= options.force_tolerance_n
            && r.pitch_moment_nm.abs() <= options.moment_tolerance_nm
        {
            // The requested three equations describe longitudinal trim. Ensure
            // an asymmetric moment arm has not left unbalanced lateral loads.
            if current.aero.loads.force_body_n.y.abs() > options.force_tolerance_n
                || current.aero.loads.moment_body_nm.x.abs() > options.moment_tolerance_nm
                || current.aero.loads.moment_body_nm.z.abs() > options.moment_tolerance_nm
            {
                return Err(failed(
                    iteration,
                    r,
                    "lateral force or roll/yaw moment is unbalanced",
                ));
            }
            return Ok(TrimSolution {
                condition: condition(x)?,
                state: current.state,
                residuals: r,
                iterations: iteration,
                applied_thrust_n: request.applied_thrust_n.unwrap_or(0.0),
            });
        }
        if iteration == options.max_iterations {
            return Err(failed(iteration, r, "iteration limit reached"));
        }
        let base = scaled(r, scales);
        let h = options.difference_step_rad.value();
        let mut jacobian = Matrix3::zeros();
        for column in 0..3 {
            let mut plus = x;
            plus[column] += h;
            let mut minus = x;
            minus[column] -= h;
            if !forward_branch(plus) || !forward_branch(minus) {
                return Err(failed(
                    iteration,
                    r,
                    "finite-difference stencil leaves the forward branch",
                ));
            }
            let p = at_condition(aircraft, request, condition(plus)?)?;
            let m = at_condition(aircraft, request, condition(minus)?)?;
            jacobian.set_column(
                column,
                &((scaled(p.residuals, scales) - scaled(m.residuals, scales)) / (2.0 * h)),
            );
        }
        let Some(direction) = jacobian.lu().solve(&(-base)) else {
            return Err(failed(iteration, r, "singular trim Jacobian"));
        };
        if !direction.iter().all(|v| v.is_finite()) {
            return Err(failed(iteration, r, "non-finite Newton direction"));
        }
        let mut accepted = None;
        let mut fraction = 1.0;
        // Bounded backtracking is a numerical search policy, not a control
        // deflection limit. No unspecified physical actuator bounds are added.
        for _ in 0..24 {
            let trial = x + direction * fraction;
            if forward_branch(trial) {
                let evaluated = at_condition(aircraft, request, condition(trial)?)?;
                if scaled(evaluated.residuals, scales).norm() < base.norm() {
                    accepted = Some((trial, evaluated));
                    break;
                }
            }
            fraction *= 0.5;
        }
        let Some((next, evaluated)) = accepted else {
            return Err(failed(
                iteration,
                r,
                "line search did not reduce the residual",
            ));
        };
        x = next;
        current = evaluated;
    }
    unreachable!("the inclusive loop returns at the iteration limit")
}

/// Control-fixed local static margin, chord fraction, from central differences
/// of TABLE-EVALUATED coefficients: -Cm_alpha / CL_alpha (Caughey eq. 3.21).
/// Speed/Mach, geometric altitude, elevator and all other controls stay fixed.
/// Cm uses the actual aerodynamic moment about the CG divided by q*S*chord,
/// including reference-point transfer. Constant CG thrust adds no moment.
pub fn static_margin(
    aircraft: &Aircraft,
    request: TrimRequest,
    trim: TrimCondition,
    difference_step_rad: Radians,
) -> Result<StaticMargin, TrimError> {
    validate_request(request)?;
    let h = difference_step_rad.value();
    if h <= 0.0 || h >= FRAC_PI_2 {
        return Err(TrimError::Invalid(
            "static-margin difference step must be positive and below pi/2",
        ));
    }
    let mut plus = trim;
    plus.alpha_rad = checked_rad(trim.alpha_rad.value() + h)?;
    let mut minus = trim;
    minus.alpha_rad = checked_rad(trim.alpha_rad.value() - h)?;
    let p = at_condition(aircraft, request, plus)?;
    let m = at_condition(aircraft, request, minus)?;
    if p.aero.stalled || m.aero.stalled {
        return Err(TrimError::UndefinedMargin(
            "finite-difference stencil reaches the CLmax cap",
        ));
    }
    let cm = |value: &Evaluation| {
        value.aero.loads.moment_body_nm.y
            / (value.aero.dynamic_pressure_pa
                * aircraft.reference_area_m2
                * aircraft.reference_chord_m)
    };
    let cl_derivative = (p.aero.cl - m.aero.cl) / (2.0 * h);
    let cm_derivative = (cm(&p) - cm(&m)) / (2.0 * h);
    let margin = -cm_derivative / cl_derivative;
    if cl_derivative <= 0.0
        || ![cl_derivative, cm_derivative, margin]
            .into_iter()
            .all(f64::is_finite)
    {
        return Err(TrimError::UndefinedMargin(
            "nonpositive/unresolved lift slope or non-finite derivative",
        ));
    }
    Ok(StaticMargin {
        margin_chord_fraction: margin,
        d_cl_d_alpha_per_rad: cl_derivative,
        d_cm_d_alpha_per_rad: cm_derivative,
        difference_step_rad,
    })
}

fn at_condition(
    aircraft: &Aircraft,
    request: TrimRequest,
    c: TrimCondition,
) -> Result<Evaluation, TrimError> {
    if !forward_branch(vector(c)) {
        return Err(TrimError::Invalid(
            "alpha/gamma outside forward nonvertical branch",
        ));
    }
    let alpha = c.alpha_rad.value();
    let gamma = c.gamma_rad.value();
    let (sin, cos) = alpha.sin_cos();
    let state = State {
        position_ned_m: Vector3::new(0.0, 0.0, -request.geometric_altitude_m),
        velocity_body_mps: Vector3::new(
            request.airspeed_mps * cos,
            0.0,
            request.airspeed_mps * sin,
        ),
        attitude_body_to_ned: from_euler321(Euler321 {
            roll_rad: rad(0.0),
            pitch_rad: checked_rad(alpha + gamma)?,
            yaw_rad: rad(0.0),
        }),
        angular_rates_body_radps: Vector3::zeros(),
    };
    let aero = evaluate(
        aircraft,
        &state,
        Controls {
            elevator_rad: c.elevator_rad,
            ..Controls::neutral()
        },
    )
    .map_err(TrimError::Model)?;
    let thrust = request.applied_thrust_n.unwrap_or(0.0);
    let weight = aircraft.mass_kg * STANDARD_GRAVITY_MPS2;
    let residuals = TrimResiduals {
        parallel_force_n: thrust * cos - aero.drag_n - weight * gamma.sin(),
        normal_force_n: aero.lift_n + thrust * sin - weight * gamma.cos(),
        pitch_moment_nm: aero.loads.moment_body_nm.y,
    };
    if !residual_vector(residuals).iter().all(|v| v.is_finite()) {
        return Err(TrimError::Invalid("non-finite trim residual"));
    }
    Ok(Evaluation {
        state,
        aero,
        residuals,
    })
}
fn validate_request(request: TrimRequest) -> Result<(), TrimError> {
    if !request.airspeed_mps.is_finite()
        || request.airspeed_mps < MIN_AIRSPEED_FOR_ANGLES_MPS
        || !request.geometric_altitude_m.is_finite()
        || !request.applied_thrust_n.unwrap_or(0.0).is_finite()
    {
        return Err(TrimError::Invalid(
            "finite inputs required; speed must reach the D13 angle threshold",
        ));
    }
    Ok(())
}
fn residual_vector(r: TrimResiduals) -> Vector3<f64> {
    Vector3::new(r.parallel_force_n, r.normal_force_n, r.pitch_moment_nm)
}
fn scaled(r: TrimResiduals, scales: Vector3<f64>) -> Vector3<f64> {
    residual_vector(r).component_div(&scales)
}
fn vector(c: TrimCondition) -> Vector3<f64> {
    Vector3::new(
        c.alpha_rad.value(),
        c.elevator_rad.value(),
        c.gamma_rad.value(),
    )
}
fn condition(x: Vector3<f64>) -> Result<TrimCondition, TrimError> {
    Ok(TrimCondition {
        alpha_rad: checked_rad(x.x)?,
        elevator_rad: checked_rad(x.y)?,
        gamma_rad: checked_rad(x.z)?,
    })
}
fn forward_branch(x: Vector3<f64>) -> bool {
    x.iter().all(|v| v.is_finite()) && x.x.abs() < FRAC_PI_2 && x.z.abs() < FRAC_PI_2
}
fn failed(iterations: usize, residuals: TrimResiduals, reason: &'static str) -> TrimError {
    TrimError::NonConvergence {
        iterations,
        residuals,
        reason,
    }
}
fn rad(value: f64) -> Radians {
    Radians::new(value).expect("finite numerical constant")
}
fn checked_rad(value: f64) -> Result<Radians, TrimError> {
    Radians::new(value).map_err(|_| TrimError::Invalid("non-finite solver angle"))
}
