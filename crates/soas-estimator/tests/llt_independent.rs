//! Every numeric input/setting is a SYNTHETIC TEST VALUE, ledgered in
//! fixtures/llt_independent_values.md. Fixed expectations are owner-supplied
//! independent Gaussian-elimination results, not outputs of the Rust solver.
//! Equations/provenance: data/reference/estimator/llt_independent.md; MIT Lecture 9.
use serde_json::Value;
use soas_core::units::Radians;
use soas_estimator::{
    lifting_line::{StraightWing, solve},
    planform::{WingPlanform, WingStation, thin_airfoil_slope_per_rad},
};
#[test]
fn rectangular_higher_modes_match_independent_fixed_lift_drag_and_efficiency() {
    let v: Value = serde_json::from_str(include_str!("fixtures/llt_independent.json")).unwrap();
    let number = |key: &str| v[key].as_f64().unwrap();
    let wing = WingPlanform {
        data_note: v["data_note"].as_str().unwrap().into(),
        stations: [0.0, number("span_m") / 2.0]
            .into_iter()
            .map(|y| WingStation {
                y_m: y,
                chord_m: number("chord_m"),
                leading_edge_x_m: 0.0,
                z_m: 0.0,
                twist_rad: Radians::new(0.0).unwrap(),
                section_lift_curve_slope_per_rad: thin_airfoil_slope_per_rad(),
                zero_lift_angle_rad: Radians::new(number("zero_lift_angle_rad")).unwrap(),
                section_source: "MIT Lecture 9 / ideal thin-airfoil 2*pi; synthetic test".into(),
            })
            .collect(),
    };
    for reference in v["checks"].as_array().unwrap() {
        let terms = reference["odd_terms"].as_u64().unwrap().try_into().unwrap();
        let out = solve(
            StraightWing::Stations(&wing),
            Radians::new(number("alpha_rad")).unwrap(),
            terms,
        )
        .unwrap();
        for (name, actual) in [
            ("cl", out.cl),
            ("cdi", out.induced_drag_coefficient),
            ("efficiency", out.span_efficiency.unwrap()),
        ] {
            let expected = reference[name].as_f64().unwrap();
            assert!(
                (actual - expected).abs() <= number("absolute_tolerance"),
                "N={terms} {name}: {actual} versus independent {expected}"
            );
        }
    }
}
