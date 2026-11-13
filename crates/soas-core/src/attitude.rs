//! Hamilton unit-quaternion rotations from body axes to North-East-Down (NED).
//! Euler angles are a 3-2-1 yaw/pitch/roll representation for I/O only.
//!
//! The body-to-NED matrix is Rz(yaw) * Ry(pitch) * Rx(roll). This is the
//! `nalgebra` roll/pitch/yaw convention, verified by independent axis tests:
//! https://docs.rs/nalgebra/0.34.2/nalgebra/geometry/type.UnitQuaternion.html

use crate::units::Radians;
use nalgebra::{UnitQuaternion, Vector3};
use std::{error::Error, fmt};

pub type Attitude = UnitQuaternion<f64>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Euler321 {
    pub roll_rad: Radians,
    pub pitch_rad: Radians,
    pub yaw_rad: Radians,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationError {
    SingularEuler,
    NonFiniteVector,
}

impl fmt::Display for RotationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::SingularEuler => "roll and yaw are not independently defined at singular pitch",
            Self::NonFiniteVector => "rotation input or output is not finite",
        })
    }
}

impl Error for RotationError {}

pub fn from_euler321(angles: Euler321) -> Attitude {
    let mut attitude = Attitude::from_euler_angles(
        angles.roll_rad.value(),
        angles.pitch_rad.value(),
        angles.yaw_rad.value(),
    );
    attitude.renormalize();
    attitude
}

/// Return canonical pitch in [-pi/2, pi/2] and roll/yaw in [-pi, pi].
/// Near +/-90 degrees, roll and yaw are ill-conditioned. If |cos(pitch)|
/// is at most 64 machine epsilons, return `SingularEuler` instead of inventing
/// a unique roll/yaw pair. This is a numerical guard, not a physical limit;
/// the quaternion itself remains valid at singular pitch.
pub fn to_euler321(attitude: &Attitude) -> Result<Euler321, RotationError> {
    let rotation = attitude.to_rotation_matrix();
    let matrix = rotation.matrix();
    let cos_pitch = matrix[(0, 0)].hypot(matrix[(1, 0)]);
    if cos_pitch <= 64.0 * f64::EPSILON {
        return Err(RotationError::SingularEuler);
    }
    let angle = |value| Radians::new(value).map_err(|_| RotationError::NonFiniteVector);
    Ok(Euler321 {
        roll_rad: angle(matrix[(2, 1)].atan2(matrix[(2, 2)]))?,
        pitch_rad: angle((-matrix[(2, 0)]).atan2(cos_pitch))?,
        yaw_rad: angle(matrix[(1, 0)].atan2(matrix[(0, 0)]))?,
    })
}

/// Post-multiply an increment expressed in body coordinates, then renormalize.
pub fn compose_body_rotation(attitude: &Attitude, increment: &Attitude) -> Attitude {
    let mut composed = attitude * increment;
    composed.renormalize();
    composed
}

pub fn body_to_ned(
    attitude: &Attitude,
    vector_body: Vector3<f64>,
) -> Result<Vector3<f64>, RotationError> {
    finite_vector(vector_body)?;
    finite_vector(attitude.transform_vector(&vector_body))
}

pub fn ned_to_body(
    attitude: &Attitude,
    vector_ned: Vector3<f64>,
) -> Result<Vector3<f64>, RotationError> {
    finite_vector(vector_ned)?;
    finite_vector(attitude.inverse_transform_vector(&vector_ned))
}

fn finite_vector(vector: Vector3<f64>) -> Result<Vector3<f64>, RotationError> {
    if vector.iter().all(|component| component.is_finite()) {
        Ok(vector)
    } else {
        Err(RotationError::NonFiniteVector)
    }
}
