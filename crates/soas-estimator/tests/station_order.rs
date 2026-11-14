//! SYNTHETIC test values/settings: fixtures/station_order_values.md.
//! Owner clarified that exact existing WingError field/reason values are
//! required, and equal spans means root and tip both at y=0 (zero span).
use serde_json::Value;
use soas_core::units::Radians;
use soas_estimator::planform::{WingError, WingPlanform, WingStation, thin_airfoil_slope_per_rad};
fn check(case: &str) {
    let v: Value = serde_json::from_str(include_str!("fixtures/station_order.json")).unwrap();
    let c = &v[case];
    let wing = WingPlanform {
        data_note: v["data_note"].as_str().unwrap().into(),
        stations: c["y_m"]
            .as_array()
            .unwrap()
            .iter()
            .map(|y| WingStation {
                y_m: y.as_f64().unwrap(),
                chord_m: v["chord_m"].as_f64().unwrap(),
                leading_edge_x_m: 0.0,
                z_m: 0.0,
                twist_rad: Radians::new(0.0).unwrap(),
                zero_lift_angle_rad: Radians::new(0.0).unwrap(),
                section_lift_curve_slope_per_rad: thin_airfoil_slope_per_rad(),
                section_source: "Cited ideal 2*pi; synthetic order-validation case".into(),
            })
            .collect(),
    };
    let reason = match case {
        "missing_root" => "root must be at the symmetry plane y=0",
        _ => "right-half stations must be nonnegative and strictly increasing",
    };
    assert_eq!(reason, c["error_reason"].as_str().unwrap());
    let expected = WingError {
        field: c["error_field"].as_str().unwrap().into(),
        reason,
    };
    assert_eq!(wing.validate().unwrap_err(), expected);
    assert_eq!(wing.geometry().unwrap_err(), expected);
}
#[test]
fn duplicate_interior_station_has_the_specific_order_error() {
    check("duplicate_interior");
}
#[test]
fn descending_interior_station_has_the_specific_order_error() {
    check("descending_interior");
}
#[test]
fn first_station_not_at_root_has_the_specific_root_error() {
    check("missing_root");
}
#[test]
fn equal_root_and_tip_span_has_the_specific_order_error() {
    check("equal_root_tip");
}
