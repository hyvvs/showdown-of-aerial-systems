//! All numeric scenario/engine data are labeled SYNTHETIC TEST VALUES.
use serde_json::{Value, json};
use soas_cli::{csv_text, parse_scenario, simulate};
use soas_core::{aircraft::from_json, dynamics::FixedStepRk4, propulsion::evaluate_flight};
const SCENARIO: &str = include_str!("fixtures/synthetic_powered_scenario.json");
fn jet() -> soas_core::aircraft::Aircraft {
    from_json(include_str!(
        "../../soas-core/tests/fixtures/synthetic_jet.json"
    ))
    .unwrap()
}
#[test]
fn throttle_is_interpolated_at_rk4_stages_and_csv_reports_thrust_and_fuel_deterministically() {
    let a = jet();
    let s = parse_scenario(SCENARIO).unwrap();
    assert_eq!(s.throttle_at(0.0).unwrap(), 0.2);
    assert!((s.throttle_at(0.05).unwrap() - 0.45).abs() < 1e-14);
    let out = simulate(&s, &a).unwrap();
    let again = simulate(&s, &a).unwrap();
    let csv = csv_text(&out);
    assert_eq!(csv, csv_text(&again));
    for field in [
        "throttle_unitless",
        "thrust_n",
        "fuel_flow_kgps",
        "afterburner_engines_unitless",
    ] {
        assert!(csv.lines().next().unwrap().contains(field));
    }
    assert!(out.iter().all(|x| x.propulsion.fuel_flow_kgps > 0.0));
    assert_eq!(out[2].propulsion.afterburner_engines, 1);
    assert_eq!(a.mass_kg, 10.0);
    // Independent RK4 call for the first step using T(t)=.2+5*t;
    // freezing throttle at the step start must differ from the runner result.
    let initial = s.initial_state.to_state().unwrap();
    let rk = FixedStepRk4::new(a.mass_properties().unwrap(), s.dt_s).unwrap();
    let expected = rk
        .try_step_with_time(&initial, |t, stage| {
            evaluate_flight(&a, stage, s.controls.at(t).unwrap(), 0.2 + 5.0 * t).map(|x| x.loads)
        })
        .unwrap();
    let mut one = s.clone();
    one.duration_s = s.dt_s;
    one.output_interval_s = s.dt_s;
    let sample = simulate(&one, &a).unwrap()[1];
    assert!((sample.state.velocity_body_mps - expected.velocity_body_mps).norm() < 1e-13);
    let frozen = rk
        .try_step(&initial, |stage| {
            evaluate_flight(&a, stage, s.controls.at(0.0).unwrap(), 0.2).map(|x| x.loads)
        })
        .unwrap();
    assert!((sample.state.velocity_body_mps - frozen.velocity_body_mps).norm() > 1e-5);
}
#[test]
fn throttle_knots_are_named_objects_and_versions_ranges_and_coverage_are_checked() {
    let original: Value = serde_json::from_str(SCENARIO).unwrap();
    for i in 0..3 {
        let mut v = original.clone();
        v["throttle"][i] = json!([v["throttle"][i]["time_s"], v["throttle"][i]["throttle"]]);
        let e = parse_scenario(&v.to_string()).unwrap_err().to_string();
        assert!(
            e.contains(&format!("throttle[{i}]")) && e.contains("object"),
            "{e}"
        );
    }
    for (path, value) in [
        ("/format_version", json!(0)),
        ("/format_version", json!(2)),
        ("/throttle", json!([])),
        ("/throttle/0/throttle", json!(-0.1)),
        ("/throttle/1/throttle", json!(1.1)),
        ("/throttle/1/time_s", json!(0.0)),
        ("/throttle/2/time_s", json!(0.15)),
    ] {
        let mut v = original.clone();
        *v.pointer_mut(path).unwrap() = value;
        assert!(parse_scenario(&v.to_string()).is_err(), "accepted {path}");
    }
    let mut missing = original;
    missing.as_object_mut().unwrap().remove("throttle");
    assert!(
        parse_scenario(&missing.to_string())
            .unwrap_err()
            .to_string()
            .contains("format_version")
    );
    let legacy = parse_scenario(include_str!("fixtures/synthetic_scenario.json")).unwrap();
    assert!(
        simulate(&legacy, &jet())
            .unwrap_err()
            .to_string()
            .contains("version 0")
    );
    let mut malformed = parse_scenario(SCENARIO).unwrap();
    malformed.throttle[1].throttle = f64::NAN;
    assert!(simulate(&malformed, &jet()).is_err());
}
