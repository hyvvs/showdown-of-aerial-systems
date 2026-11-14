//! All new fixture numbers are SYNTHETIC TEST VALUES in the companion ledger.
use serde_json::Value;
use soas_core::aircraft::{from_json, to_json};
const FIXTURE: &str = include_str!("fixtures/synthetic_dynamic_aircraft.json");
#[test]
fn required_v2_fields_and_legacy_rejection_are_explicit() {
    for field in [
        "cl_min",
        "roll_beta_per_rad",
        "yaw_beta_per_rad",
        "cm_q",
        "roll_p",
        "yaw_r",
    ] {
        let mut v: Value = serde_json::from_str(FIXTURE).unwrap();
        v["aerodynamics"]["coefficients"][0][0]
            .as_object_mut()
            .unwrap()
            .remove(field);
        let e = from_json(&v.to_string()).unwrap_err().to_string();
        assert!(e.contains(field), "{e}");
    }
    let mut v: Value = serde_json::from_str(FIXTURE).unwrap();
    v.as_object_mut().unwrap().remove("control_limits");
    assert!(
        from_json(&v.to_string())
            .unwrap_err()
            .to_string()
            .contains("control_limits")
    );
    for version in [0, 1, 3] {
        v["format_version"] = Value::from(version);
        assert!(
            from_json(&v.to_string())
                .unwrap_err()
                .to_string()
                .contains("format_version")
        );
    }
}
#[test]
fn invalid_limits_coefficients_and_nonfinite_programmatic_values_error() {
    for (path, value) in [
        ("/control_limits/elevator/min_rad", Value::from(1.0)),
        ("/aerodynamics/coefficients/0/0/cl_min", Value::from(0.0)),
    ] {
        let mut v: Value = serde_json::from_str(FIXTURE).unwrap();
        *v.pointer_mut(path).unwrap() = value;
        assert!(from_json(&v.to_string()).is_err());
    }
    let mut a = from_json(FIXTURE).unwrap();
    a.aerodynamics.coefficients[0][0].cm_q = f64::NAN;
    assert!(to_json(&a).is_err());
    // Both optional cross-rate fields may be absent, explicitly meaning zero.
    let mut v: Value = serde_json::from_str(FIXTURE).unwrap();
    for row in v["aerodynamics"]["coefficients"].as_array_mut().unwrap() {
        for c in row.as_array_mut().unwrap() {
            c.as_object_mut().unwrap().remove("roll_r");
            c.as_object_mut().unwrap().remove("yaw_p");
        }
    }
    let a = from_json(&v.to_string()).unwrap();
    assert_eq!(a.aerodynamics.coefficients[0][0].roll_r, 0.0);
    assert_eq!(a.aerodynamics.coefficients[0][0].yaw_p, 0.0);
}
