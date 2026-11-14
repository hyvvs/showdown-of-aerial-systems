//! Directed unit-circulation Biot-Savart filaments. Formula sources, scalar
//! hand checks and singularity policy: data/reference/estimator/vortex.md.
use nalgebra::Vector3;
use std::{error::Error, f64::consts::PI, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VortexError {
    Invalid(&'static str),
    Singular,
    Numerical,
}
impl fmt::Display for VortexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(s) => write!(f, "vortex input: {s}"),
            Self::Singular => {
                f.write_str("vortex singularity: point lies on a filament segment/ray")
            }
            Self::Numerical => f.write_str("vortex arithmetic overflow/underflow"),
        }
    }
}
impl Error for VortexError {}
fn finite(v: Vector3<f64>) -> Result<Vector3<f64>, VortexError> {
    if v.iter().all(|x| x.is_finite()) {
        Ok(v)
    } else {
        Err(VortexError::Numerical)
    }
}
fn norm(v: Vector3<f64>) -> Result<f64, VortexError> {
    finite(v)?;
    let n = v.x.hypot(v.y).hypot(v.z);
    if n.is_finite() {
        Ok(n)
    } else {
        Err(VortexError::Numerical)
    }
}
/// Finite filament start -> end. Output units 1/m: multiply by Gamma (m^2/s)
/// to get velocity (m/s). NASA CR-3208 Appendix A, eqs A-1/A-2, in unit-direction
/// form: (u x r1)/h^2 * [u.r1/|r1| - u.r2/|r2|] / (4*pi).
pub fn finite_segment_velocity_per_circulation(
    start_m: Vector3<f64>,
    end_m: Vector3<f64>,
    point_m: Vector3<f64>,
) -> Result<Vector3<f64>, VortexError> {
    finite(start_m)?;
    finite(end_m)?;
    finite(point_m)?;
    let segment = finite(end_m - start_m)?;
    let length = norm(segment)?;
    if length == 0.0 {
        return Err(VortexError::Invalid("segment must have nonzero length"));
    }
    let u = segment / length;
    let r1 = finite(point_m - start_m)?;
    let r2 = finite(point_m - end_m)?;
    let (n1, n2) = (norm(r1)?, norm(r2)?);
    if n1 == 0.0 || n2 == 0.0 {
        return Err(VortexError::Singular);
    }
    let cross = finite(u.cross(&r1))?;
    let h = norm(cross)?;
    if h == 0.0 {
        let along = u.dot(&r1);
        return if (0.0..=length).contains(&along) {
            Err(VortexError::Singular)
        } else {
            Ok(Vector3::zeros())
        };
    }
    let angle_factor = u.dot(&(r1 / n1)) - u.dot(&(r2 / n2));
    // Normalizing once avoids squaring a tiny perpendicular distance.
    finite((cross / h) * ((angle_factor / (4.0 * PI)) / h))
}
/// Ray start -> infinity along a unit direction. NASA CR-3208 eq A-4, obtained
/// from the finite-segment limit: (u x r)/h^2 * (1+u.r/|r|)/(4*pi).
pub fn semi_infinite_velocity_per_circulation(
    start_m: Vector3<f64>,
    direction_unit: Vector3<f64>,
    point_m: Vector3<f64>,
) -> Result<Vector3<f64>, VortexError> {
    finite(start_m)?;
    finite(point_m)?;
    finite(direction_unit)?;
    if (norm(direction_unit)? - 1.0).abs() > 64.0 * f64::EPSILON {
        return Err(VortexError::Invalid("ray direction must be unit length"));
    }
    let r = finite(point_m - start_m)?;
    let distance = norm(r)?;
    if distance == 0.0 {
        return Err(VortexError::Singular);
    }
    let cross = finite(direction_unit.cross(&r))?;
    let h = norm(cross)?;
    let cosine = direction_unit.dot(&(r / distance));
    if h == 0.0 {
        return if cosine >= 0.0 {
            Err(VortexError::Singular)
        } else {
            Ok(Vector3::zeros())
        };
    }
    // For an upstream point, avoid cancellation in 1+cos(theta) by the
    // equivalent sin^2(theta)/(1-cos(theta)); this adds no physical cutoff.
    let angle_factor = if cosine < 0.0 {
        (h / distance).powi(2) / (1.0 - cosine)
    } else {
        1.0 + cosine
    };
    finite((cross / h) * ((angle_factor / (4.0 * PI)) / h))
}

#[derive(Debug, Clone, Copy)]
pub struct Horseshoe {
    pub bound_start_m: Vector3<f64>,
    pub bound_end_m: Vector3<f64>,
    pub wake_direction_unit: Vector3<f64>,
}
impl Horseshoe {
    /// Incoming ray infinity -> start, directed bound start -> end, outgoing
    /// ray end -> infinity. Body-axis wing convention: bound left -> right,
    /// wake -x. Unit positive circulation then gives downward induced z aft.
    pub fn velocity_per_circulation(
        &self,
        point_m: Vector3<f64>,
    ) -> Result<Vector3<f64>, VortexError> {
        finite(
            finite_segment_velocity_per_circulation(self.bound_start_m, self.bound_end_m, point_m)?
                - semi_infinite_velocity_per_circulation(
                    self.bound_start_m,
                    self.wake_direction_unit,
                    point_m,
                )?
                + semi_infinite_velocity_per_circulation(
                    self.bound_end_m,
                    self.wake_direction_unit,
                    point_m,
                )?,
        )
    }
}
