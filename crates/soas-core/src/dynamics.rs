//! Constant-mass, six-degree-of-freedom rigid-body motion about the CG.
//!
//! Equations and conventions: data/reference/dynamics/README.md. In particular,
//! Caughey (2011), eq. 4.12; MIT 8.09 ch. 2, eq. 2.66; Sola (2017), eq. 200;
//! NIST DLMF 3.7.18-19 (classical RK4). No aerodynamics or propulsion is assumed.

use crate::{atmosphere::STANDARD_GRAVITY_MPS2, attitude::Attitude};
use nalgebra::{Matrix3, Quaternion, UnitQuaternion, Vector3};
use std::{convert::Infallible, error::Error, fmt};

/// Owner decision D10; this is a numerical default, not an aircraft constant.
pub const DEFAULT_STEP_S: f64 = 1.0 / 400.0;

/// Complete instantaneous state, without caches, callbacks or I/O resources.
/// All vectors have explicit frames and units. This record can acquire serde
/// support later without making an integrator or load model serializable.
/// The Hamilton quaternion maps body vectors to NED; its components are
/// scalar w and vector (i,j,k). No Euler angles are stored or integrated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
    pub position_ned_m: Vector3<f64>,
    pub velocity_body_mps: Vector3<f64>,
    pub attitude_body_to_ned: Attitude,
    pub angular_rates_body_radps: Vector3<f64>,
}

/// Applied force EXCLUDING gravity, and total moment about the CG, both in
/// body axes. Gravity is added once by the dynamics, with NED down positive.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Loads {
    pub force_body_n: Vector3<f64>,
    pub moment_body_nm: Vector3<f64>,
}

/// Constant mass and signed inertia tensor about the CG in body axes (D11).
/// Private fields keep the cached inverse consistent with the input tensor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassProperties {
    mass_kg: f64,
    inertia_body_kg_m2: Matrix3<f64>,
    inverse_inertia_body: Matrix3<f64>,
}

impl MassProperties {
    /// Require finite positive mass and a finite, exactly symmetric,
    /// positive-definite inertia tensor. Never silently symmetrize input.
    /// This numerical check does not establish that aircraft data are genuine.
    pub fn from_signed_tensor(
        mass_kg: f64,
        inertia_body_kg_m2: Matrix3<f64>,
    ) -> Result<Self, DynamicsError> {
        if !mass_kg.is_finite() || mass_kg <= 0.0 {
            return Err(DynamicsError::InvalidMass);
        }
        if !all_finite(inertia_body_kg_m2.iter())
            || inertia_body_kg_m2 != inertia_body_kg_m2.transpose()
        {
            return Err(DynamicsError::InvalidInertia);
        }
        let inverse_inertia_body = inertia_body_kg_m2
            .cholesky()
            .ok_or(DynamicsError::InvalidInertia)?
            .inverse();
        if !all_finite(inverse_inertia_body.iter()) {
            return Err(DynamicsError::InvalidInertia);
        }
        Ok(Self {
            mass_kg,
            inertia_body_kg_m2,
            inverse_inertia_body,
        })
    }

    /// `diagonal_kg_m2` = (Ixx,Iyy,Izz); `products_kg_m2` = (Pxy,Pxz,Pyz),
    /// where Pxy = integral(x*y dm), WITHOUT a leading minus sign.
    /// The three products can themselves be positive or negative. Negate
    /// them to obtain signed off-diagonal tensor entries, symmetrically.
    pub fn from_products_of_inertia(
        mass_kg: f64,
        diagonal_kg_m2: Vector3<f64>,
        products_kg_m2: Vector3<f64>,
    ) -> Result<Self, DynamicsError> {
        let d = diagonal_kg_m2;
        let p = products_kg_m2;
        Self::from_signed_tensor(
            mass_kg,
            Matrix3::new(d.x, -p.x, -p.y, -p.x, d.y, -p.z, -p.y, -p.z, d.z),
        )
    }

    pub fn mass_kg(&self) -> f64 {
        self.mass_kg
    }

    pub fn inertia_body_kg_m2(&self) -> Matrix3<f64> {
        self.inertia_body_kg_m2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicsError {
    InvalidMass,
    InvalidInertia,
    InvalidStep,
    InvalidAttitude,
    NonFinite,
}

impl fmt::Display for DynamicsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidMass => "mass must be finite and positive",
            Self::InvalidInertia => {
                "inertia must be finite, symmetric, positive definite and invertible"
            }
            Self::InvalidStep => "time step must be finite and positive",
            Self::InvalidAttitude => "attitude must be a finite unit quaternion",
            Self::NonFinite => "state, loads or intermediate calculation is not finite",
        })
    }
}

impl Error for DynamicsError {}

/// Distinguish a rigid-body calculation error from a caller's load-model error.
#[derive(Debug)]
pub enum StepError<E> {
    Dynamics(DynamicsError),
    Loads(E),
}

impl<E: fmt::Display> fmt::Display for StepError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dynamics(error) => error.fmt(f),
            Self::Loads(error) => write!(f, "load model: {error}"),
        }
    }
}

impl<E: Error + 'static> Error for StepError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Dynamics(error) => Some(error),
            Self::Loads(error) => Some(error),
        }
    }
}

/// The step size is fixed for this integrator. Construct another instance to
/// compare a different step; no adaptive stepping or wall-clock time is used.
#[derive(Debug, Clone, Copy)]
pub struct FixedStepRk4 {
    mass_properties: MassProperties,
    step_s: f64,
}

impl FixedStepRk4 {
    pub fn new(mass_properties: MassProperties, step_s: f64) -> Result<Self, DynamicsError> {
        if !step_s.is_finite() || step_s <= 0.0 {
            return Err(DynamicsError::InvalidStep);
        }
        Ok(Self {
            mass_properties,
            step_s,
        })
    }

    pub fn with_default_step(mass_properties: MassProperties) -> Self {
        Self {
            mass_properties,
            step_s: DEFAULT_STEP_S,
        }
    }

    pub fn step_s(&self) -> f64 {
        self.step_s
    }

    /// Evaluate a pure, state-based load model at ALL FOUR RK4 stages. Callers
    /// must not introduce side effects, clocks or randomness into the callback.
    /// A constant-load callback is also valid. Errors return without mutating
    /// the supplied state. Completed steps always normalize the quaternion.
    pub fn step(
        &self,
        state: &State,
        loads: impl Fn(&State) -> Loads,
    ) -> Result<State, DynamicsError> {
        self.try_step(state, |stage| Ok::<_, Infallible>(loads(stage)))
            .map_err(|error| match error {
                StepError::Dynamics(error) => error,
                StepError::Loads(never) => match never {},
            })
    }

    /// Fallible load model, evaluated at every stage. A failure aborts the
    /// entire step without changing the input or substituting a zero load.
    pub fn try_step<E>(
        &self,
        state: &State,
        loads: impl Fn(&State) -> Result<Loads, E>,
    ) -> Result<State, StepError<E>> {
        let norm = state.attitude_body_to_ned.quaternion().norm();
        // Numerical invariant guard, not a new physical tolerance. nalgebra's
        // UnitQuaternion normally guarantees this; detect unchecked misuse.
        if !norm.is_finite() || (norm - 1.0).abs() > 64.0 * f64::EPSILON {
            return Err(StepError::Dynamics(DynamicsError::InvalidAttitude));
        }
        let start = Stage::from(state);
        let h = self.step_s;
        let k1 = self.derivative(&start, &loads)?;
        let k2 = self.derivative(&start.add_scaled(&k1, h / 2.0), &loads)?;
        let k3 = self.derivative(&start.add_scaled(&k2, h / 2.0), &loads)?;
        let k4 = self.derivative(&start.add_scaled(&k3, h), &loads)?;
        start
            .add_scaled(&k1, h / 6.0)
            .add_scaled(&k2, h / 3.0)
            .add_scaled(&k3, h / 3.0)
            .add_scaled(&k4, h / 6.0)
            .to_state()
            .map_err(StepError::Dynamics)
    }

    fn derivative<E>(
        &self,
        stage: &Stage,
        loads: &impl Fn(&State) -> Result<Loads, E>,
    ) -> Result<Stage, StepError<E>> {
        let state = stage.to_state().map_err(StepError::Dynamics)?;
        let applied = loads(&state).map_err(StepError::Loads)?;
        if !all_finite(
            applied
                .force_body_n
                .iter()
                .chain(applied.moment_body_nm.iter()),
        ) {
            return Err(StepError::Dynamics(DynamicsError::NonFinite));
        }
        let attitude = state.attitude_body_to_ned;
        let velocity = state.velocity_body_mps;
        let omega = state.angular_rates_body_radps;
        let mass = &self.mass_properties;
        let gravity_body =
            attitude.inverse_transform_vector(&Vector3::new(0.0, 0.0, STANDARD_GRAVITY_MPS2));
        // Rotating body coordinates need -omega cross velocity, even if the
        // inertial trajectory is a straight line. Gravity is not an input load.
        let velocity_dot =
            applied.force_body_n / mass.mass_kg + gravity_body - omega.cross(&velocity);
        // Euler's equation in tensor form retains ALL signed off-diagonals.
        let omega_dot = mass.inverse_inertia_body
            * (applied.moment_body_nm - omega.cross(&(mass.inertia_body_kg_m2 * omega)));
        // Body-local angular rates multiply the body-to-NED quaternion on the
        // RIGHT. Keep its raw RK4 stage components for this differential equation;
        // use the normalized attitude only when transforming spatial vectors.
        let quaternion_dot = stage.quaternion * Quaternion::from_parts(0.0, omega) * 0.5;
        let derivative = Stage {
            position: attitude.transform_vector(&velocity),
            velocity: velocity_dot,
            quaternion: quaternion_dot,
            omega: omega_dot,
        };
        derivative.validate_finite().map_err(StepError::Dynamics)?;
        Ok(derivative)
    }
}

/// Raw quaternion components are needed for additive RK4 stage arithmetic.
/// This type is private: a non-unit intermediate is never returned as State.
#[derive(Clone, Copy)]
struct Stage {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    quaternion: Quaternion<f64>,
    omega: Vector3<f64>,
}

impl From<&State> for Stage {
    fn from(state: &State) -> Self {
        Self {
            position: state.position_ned_m,
            velocity: state.velocity_body_mps,
            quaternion: *state.attitude_body_to_ned.quaternion(),
            omega: state.angular_rates_body_radps,
        }
    }
}

impl Stage {
    fn add_scaled(&self, derivative: &Self, scale: f64) -> Self {
        Self {
            position: self.position + derivative.position * scale,
            velocity: self.velocity + derivative.velocity * scale,
            quaternion: self.quaternion + derivative.quaternion * scale,
            omega: self.omega + derivative.omega * scale,
        }
    }

    fn validate_finite(&self) -> Result<(), DynamicsError> {
        if all_finite(
            self.position
                .iter()
                .chain(self.velocity.iter())
                .chain(self.quaternion.coords.iter())
                .chain(self.omega.iter()),
        ) {
            Ok(())
        } else {
            Err(DynamicsError::NonFinite)
        }
    }

    fn to_state(self) -> Result<State, DynamicsError> {
        self.validate_finite()?;
        let norm = self.quaternion.norm();
        if !norm.is_finite() || norm <= 0.0 {
            return Err(DynamicsError::InvalidAttitude);
        }
        Ok(State {
            position_ned_m: self.position,
            velocity_body_mps: self.velocity,
            attitude_body_to_ned: UnitQuaternion::new_normalize(self.quaternion),
            angular_rates_body_radps: self.omega,
        })
    }
}

fn all_finite<'a>(values: impl Iterator<Item = &'a f64>) -> bool {
    values.into_iter().all(|value| value.is_finite())
}
