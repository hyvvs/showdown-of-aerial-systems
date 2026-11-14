//! Every fixture number and numeric test setting is labeled SYNTHETIC TEST DATA
//! or a derived expectation in horseshoe_values.md / vortex_test_values.md.
use nalgebra::Vector3;
use serde_json::Value;
use soas_estimator::vortex::{
    Horseshoe, finite_segment_velocity_per_circulation, semi_infinite_velocity_per_circulation,
};
use std::f64::consts::PI;
fn vector(v: &Value) -> Vector3<f64> {
    Vector3::new(
        v[0].as_f64().unwrap(),
        v[1].as_f64().unwrap(),
        v[2].as_f64().unwrap(),
    )
}
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/horseshoe.json")).unwrap()
}
fn horse() -> Horseshoe {
    let v = fixture();
    Horseshoe {
        bound_start_m: vector(&v["bound_start_m"]),
        bound_end_m: vector(&v["bound_end_m"]),
        wake_direction_unit: vector(&v["wake_direction_unit"]),
    }
}
#[test]
fn unit_horseshoe_matches_independent_hand_values_on_and_off_symmetry_plane() {
    // Directed bound (0,-1,0)->(0,1,0); wake along -x.
    // P=(-1,0,0): vz=(1+sqrt(2))/(2*pi).
    // P=(-1,0,1): vx=1/(4*pi*sqrt(3)), vz=(1+2/sqrt(3))/(4*pi).
    // Off-plane/off-symmetry terms are expanded individually in the ledger/reference.
    let h = horse();
    let v = fixture();
    for case in v["cases"].as_array().unwrap() {
        let actual = h
            .velocity_per_circulation(vector(&case["point_m"]))
            .unwrap();
        let expected = vector(&case["expected_per_m"]);
        assert!(
            (actual - expected).norm() < 2e-14,
            "{}: {actual:?} != {expected:?}",
            case["name"]
        );
    }
}
#[test]
fn component_signs_and_reversed_orientation_are_independent_checks() {
    let h = horse();
    let off = h
        .velocity_per_circulation(Vector3::new(-1.0, 2.0, 2.0))
        .unwrap();
    assert!(off.x > 0.0 && off.y > 0.0 && off.z > 0.0);
    assert!(
        h.velocity_per_circulation(Vector3::new(-1.0, 2.0, 0.0))
            .unwrap()
            .z
            < 0.0
    );
    let reversed = Horseshoe {
        bound_start_m: h.bound_end_m,
        bound_end_m: h.bound_start_m,
        ..h
    };
    let p = Vector3::new(-1.0, 2.0, 2.0);
    assert!((reversed.velocity_per_circulation(p).unwrap() + off).norm() < 2e-14);
}
#[test]
fn a_long_bound_segment_approaches_the_two_dimensional_line_vortex() {
    let p = Vector3::new(2.0, 0.0, 0.0);
    let expected = -1.0 / (4.0 * PI); // Gamma/(2*pi*r), r=2.
    let mut previous = f64::INFINITY;
    for half_length in [100.0, 1000.0, 10000.0, 1000000.0] {
        let v = finite_segment_velocity_per_circulation(
            Vector3::new(0.0, -half_length, 0.0),
            Vector3::new(0.0, half_length, 0.0),
            p,
        )
        .unwrap();
        // Angular integral: -L/(2*pi*r*sqrt(L^2+r^2)).
        let hand = -half_length / (4.0 * PI * (half_length * half_length + 4.0).sqrt());
        assert!((v.z - hand).abs() < 2e-14);
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        let relative = (v.z / expected - 1.0).abs();
        assert!(relative < previous);
        previous = relative;
    }
    assert!(previous < 1e-10);
}
#[test]
fn singular_points_are_explicit_errors_and_collinear_extensions_are_zero() {
    let h = horse();
    for p in [
        h.bound_start_m,
        h.bound_end_m,
        Vector3::zeros(),
        Vector3::new(-1.0, -1.0, 0.0),
    ] {
        let e = h.velocity_per_circulation(p).unwrap_err().to_string();
        assert!(e.contains("singular"), "{e}");
    }
    let v = finite_segment_velocity_per_circulation(
        h.bound_start_m,
        h.bound_end_m,
        Vector3::new(0.0, 2.0, 0.0),
    )
    .unwrap();
    assert_eq!(v, Vector3::zeros());
    assert_eq!(
        semi_infinite_velocity_per_circulation(
            Vector3::zeros(),
            Vector3::x(),
            Vector3::new(-1.0, 0.0, 0.0)
        )
        .unwrap(),
        Vector3::zeros()
    );
}
#[test]
fn mirrored_panel_and_field_point_preserve_x_z_and_reverse_y_velocity() {
    let right = Horseshoe {
        bound_start_m: Vector3::new(0.0, 1.0, 0.0),
        bound_end_m: Vector3::new(0.0, 3.0, 0.0),
        wake_direction_unit: -Vector3::x(),
    };
    // Mirror geometry AND reverse endpoint order to keep positive bound direction +y.
    let left = Horseshoe {
        bound_start_m: Vector3::new(0.0, -3.0, 0.0),
        bound_end_m: Vector3::new(0.0, -1.0, 0.0),
        wake_direction_unit: -Vector3::x(),
    };
    let v = right
        .velocity_per_circulation(Vector3::new(-2.0, 0.5, 1.0))
        .unwrap();
    let mirror = left
        .velocity_per_circulation(Vector3::new(-2.0, -0.5, 1.0))
        .unwrap();
    assert!((mirror - Vector3::new(v.x, -v.y, v.z)).norm() < 2e-14);
}
#[test]
fn semi_infinite_result_agrees_with_hand_angle_integral_and_finite_leg_limit() {
    let a = Vector3::zeros();
    let d = Vector3::x();
    let p = Vector3::new(1.0, 1.0, 0.0);
    let expected = (1.0 + 1.0 / 2.0_f64.sqrt()) / (4.0 * PI);
    let v = semi_infinite_velocity_per_circulation(a, d, p).unwrap();
    assert!((v.z - expected).abs() < 2e-14);
    let finite =
        finite_segment_velocity_per_circulation(a, Vector3::new(1000000.0, 0.0, 0.0), p).unwrap();
    assert!((finite - v).norm() < 1e-12);
}
#[test]
fn degenerate_nonfinite_and_nonunit_inputs_are_rejected() {
    assert!(
        finite_segment_velocity_per_circulation(Vector3::zeros(), Vector3::zeros(), Vector3::x())
            .is_err()
    );
    for d in [
        Vector3::zeros(),
        Vector3::new(2.0, 0.0, 0.0),
        Vector3::new(f64::NAN, 0.0, 0.0),
    ] {
        assert!(semi_infinite_velocity_per_circulation(Vector3::zeros(), d, Vector3::y()).is_err());
    }
    assert!(
        horse()
            .velocity_per_circulation(Vector3::new(0.0, f64::INFINITY, 0.0))
            .is_err()
    );
}
