//! Synthetic mathematical cases, not aircraft data. Quarter-turn expectations
//! follow the body/NED right-handed axes in docs/CONVENTIONS.md.

use nalgebra::Vector3;
use soas_core::attitude::{
    Euler321, RotationError, body_to_ned, compose_body_rotation, from_euler321, ned_to_body,
    to_euler321,
};
use soas_core::units::{Degrees, Radians};
use std::f64::consts::{FRAC_PI_2, PI};

// Numerical comparison tolerances for analytic, synthetic cases; not a physical
// accuracy claim or an aircraft validation tolerance (D9).
const TOL: f64 = 1.0e-12;

fn rad(value: f64) -> Radians {
    Radians::new(value).unwrap()
}

fn euler(roll: f64, pitch: f64, yaw: f64) -> Euler321 {
    Euler321 {
        roll_rad: rad(roll),
        pitch_rad: rad(pitch),
        yaw_rad: rad(yaw),
    }
}

fn close_vector(actual: Vector3<f64>, expected: Vector3<f64>) {
    assert!(
        (actual - expected).norm() < TOL,
        "{actual:?} != {expected:?}"
    );
}

#[test]
fn explicit_angle_conversions_match_known_values_and_round_trip() {
    for (deg, radians) in [
        (0.0, 0.0),
        (90.0, FRAC_PI_2),
        (-180.0, -PI),
        (360.0, 2.0 * PI),
    ] {
        let converted = Radians::try_from(Degrees::new(deg).unwrap()).unwrap();
        assert!((converted.value() - radians).abs() < TOL);
        let back = Degrees::try_from(converted).unwrap();
        assert!((back.value() - deg).abs() < TOL);
    }
}

#[test]
fn invalid_angles_and_conversion_overflow_are_errors() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Radians::new(value).is_err());
        assert!(Degrees::new(value).is_err());
    }
    assert!(Degrees::try_from(rad(f64::MAX)).is_err());
}

#[test]
fn quarter_turns_follow_body_to_ned_axis_signs() {
    let yaw = from_euler321(euler(0.0, 0.0, FRAC_PI_2));
    close_vector(body_to_ned(&yaw, Vector3::x()).unwrap(), Vector3::y());
    close_vector(body_to_ned(&yaw, Vector3::y()).unwrap(), -Vector3::x());
    let pitch = from_euler321(euler(0.0, FRAC_PI_2, 0.0));
    close_vector(body_to_ned(&pitch, Vector3::x()).unwrap(), -Vector3::z());
    let roll = from_euler321(euler(FRAC_PI_2, 0.0, 0.0));
    close_vector(body_to_ned(&roll, Vector3::y()).unwrap(), Vector3::z());
    close_vector(ned_to_body(&yaw, Vector3::y()).unwrap(), Vector3::x());
}

#[test]
fn body_increment_composition_uses_hamilton_order() {
    let yaw = from_euler321(euler(0.0, 0.0, FRAC_PI_2));
    let roll = from_euler321(euler(FRAC_PI_2, 0.0, 0.0));
    let combined = compose_body_rotation(&yaw, &roll);
    // Roll acts first in body coordinates, then the existing yaw maps to NED.
    close_vector(body_to_ned(&combined, Vector3::x()).unwrap(), Vector3::y());
    close_vector(body_to_ned(&combined, Vector3::y()).unwrap(), Vector3::z());
    close_vector(body_to_ned(&combined, Vector3::z()).unwrap(), Vector3::x());
}

#[test]
fn euler_quaternion_euler_round_trips_away_from_singular_pitch() {
    for angles in [
        euler(0.2, -0.4, 0.7),
        euler(-1.0, 0.8, -2.0),
        euler(0.0, 0.0, 0.0),
    ] {
        let rotation = from_euler321(angles);
        let back = to_euler321(&rotation).unwrap();
        assert!((back.roll_rad.value() - angles.roll_rad.value()).abs() < TOL);
        assert!((back.pitch_rad.value() - angles.pitch_rad.value()).abs() < TOL);
        assert!((back.yaw_rad.value() - angles.yaw_rad.value()).abs() < TOL);
    }
}

#[test]
fn singular_euler_output_is_explicit_and_nearby_rotation_is_preserved() {
    for sign in [-1.0, 1.0] {
        let singular = from_euler321(euler(0.3, sign * FRAC_PI_2, 0.7));
        assert_eq!(to_euler321(&singular), Err(RotationError::SingularEuler));
        assert!((singular.quaternion().norm() - 1.0).abs() < TOL);
        let near = from_euler321(euler(0.3, sign * (FRAC_PI_2 - 1.0e-4), 0.7));
        let recovered = from_euler321(to_euler321(&near).unwrap());
        assert!(near.angle_to(&recovered) < 1.0e-10);
    }
}

#[test]
fn many_compositions_keep_the_quaternion_unit_length() {
    let increment = from_euler321(euler(0.001, -0.002, 0.003));
    let mut attitude = from_euler321(euler(0.0, 0.0, 0.0));
    for _ in 0..100_000 {
        attitude = compose_body_rotation(&attitude, &increment);
        assert!((attitude.quaternion().norm() - 1.0).abs() < TOL);
    }
}

#[test]
fn vector_transforms_reject_non_finite_inputs() {
    let attitude = from_euler321(euler(0.0, 0.0, 0.0));
    assert!(body_to_ned(&attitude, Vector3::new(f64::NAN, 0.0, 0.0)).is_err());
    assert!(ned_to_body(&attitude, Vector3::new(0.0, f64::INFINITY, 0.0)).is_err());
}
