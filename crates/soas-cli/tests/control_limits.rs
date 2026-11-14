//! All values are SYNTHETIC TEST DATA; fixture and Rust-value ledgers accompany them.
use soas_cli::{csv_text, parse_scenario, simulate};
#[test]
fn csv_reports_commanded_and_applied_deflection_for_each_surface() {
    let scenario =
        parse_scenario(include_str!("fixtures/synthetic_limited_scenario.json")).unwrap();
    let a = soas_core::aircraft::from_json(include_str!(
        "../../soas-core/tests/fixtures/synthetic_dynamic_aircraft.json"
    ))
    .unwrap();
    let samples = simulate(&scenario, &a).unwrap();
    let csv = csv_text(&samples);
    let mut lines = csv.lines();
    let names: Vec<_> = lines.next().unwrap().split(',').collect();
    let first: Vec<f64> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();
    for (surface, commanded_deg, applied) in [
        ("elevator", 30.0_f64, 0.18),
        ("aileron", 40.0, 0.16),
        ("rudder", -50.0, -0.3),
    ] {
        let ci = names
            .iter()
            .position(|x| *x == format!("commanded_{surface}_rad"))
            .unwrap();
        let ai = names
            .iter()
            .position(|x| *x == format!("applied_{surface}_rad"))
            .unwrap();
        assert!((first[ci] - commanded_deg.to_radians()).abs() < 1e-14);
        assert_eq!(first[ai], applied);
    }
}
