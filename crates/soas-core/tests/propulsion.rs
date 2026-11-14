//! Every engine/configuration/query number here is SYNTHETIC TEST DATA.
//! No real engine or aircraft is represented. See fixtures/synthetic_jet_values.md.
use nalgebra::Vector3;
use serde_json::{Value, json};
use soas_core::{
    aerodynamics::Controls,
    aircraft::{Aircraft, from_json, to_json},
    dynamics::State,
    propulsion::{EngineSample, evaluate, evaluate_flight, lookup},
};

const JET: &str = include_str!("fixtures/synthetic_jet.json");
fn jet() -> Aircraft {
    from_json(JET).unwrap()
}
fn close(x: f64, y: f64) {
    assert!((x - y).abs() < 1e-11, "{x:.15e} != {y:.15e}");
}
fn constant_engine(a: &mut Aircraft, thrust: f64) {
    for row in &mut a.engines[0].table.samples {
        for column in row {
            for sample in column {
                *sample = EngineSample {
                    thrust_n: thrust,
                    fuel_flow_kgps: 0.25,
                };
            }
        }
    }
}

#[test]
fn version_one_round_trips_engines_and_version_zero_remains_explicitly_power_off() {
    let a = jet();
    assert_eq!(a.format_version, 1);
    assert_eq!(from_json(&to_json(&a).unwrap()).unwrap(), a);
    let old = from_json(include_str!("fixtures/synthetic_aircraft.json")).unwrap();
    assert!(old.engines.is_empty());
    assert!(!to_json(&old).unwrap().contains("engines"));
    for (version, engines) in [(0, true), (1, false), (2, true)] {
        let mut v: Value = serde_json::from_str(JET).unwrap();
        v["format_version"] = json!(version);
        if !engines {
            v.as_object_mut().unwrap().remove("engines");
        }
        let e = from_json(&v.to_string()).unwrap_err().to_string();
        assert!(e.contains("format_version"), "{e}");
    }
    let mut empty = jet();
    empty.engines.clear();
    assert_eq!(from_json(&to_json(&empty).unwrap()).unwrap(), empty);
}

#[test]
fn trilinear_off_centre_mixed_terms_are_hand_calculated_for_both_outputs() {
    let a = jet();
    let s = lookup(&a.engines[0].table, 0.12, 1400.0, 0.24).unwrap();
    // Local fractions Mach=.4, altitude=.7, throttle=.4; all mixed terms matter.
    // T=24+2.4+14+1.44+6.72+.168+.12096=48.84896 N.
    // f=.01+.048+.0036+.014+.001152+.00672+.000504+.00016128=.08413728 kg/s.
    close(s.thrust_n, 48.84896);
    close(s.fuel_flow_kgps, 0.08413728);
    // Unequal physical axis values and distinct slopes expose transposition.
    for (m, h, t, expected) in [
        (0.3, 0.0, 0.0, 6.0),
        (0.0, 2000.0, 0.0, 20.0),
        (0.0, 0.0, 0.6, 60.0),
    ] {
        close(
            lookup(&a.engines[0].table, m, h, t).unwrap().thrust_n,
            expected,
        );
    }
}

#[test]
fn exact_nodes_singletons_and_no_extrapolation_are_explicit() {
    let a = jet();
    let t = &a.engines[0].table;
    for (hi, h) in t.geometric_altitude_nodes_m.iter().enumerate() {
        for (mi, m) in t.mach_nodes.iter().enumerate() {
            for (ti, throttle) in t.throttle_nodes.iter().enumerate() {
                assert_eq!(lookup(t, *m, *h, *throttle).unwrap(), t.samples[hi][mi][ti]);
            }
        }
    }
    for (m, h, tv) in [
        (-0.1, 0.0, 0.0),
        (0.9, 0.0, 0.0),
        (0.0, -1.0, 0.0),
        (0.0, 2001.0, 0.0),
        (0.0, 0.0, -0.1),
        (0.0, 0.0, 1.1),
        (f64::NAN, 0.0, 0.0),
    ] {
        assert!(lookup(t, m, h, tv).is_err());
    }
    let mut singleton = t.clone();
    singleton.mach_nodes = vec![0.3];
    singleton.geometric_altitude_nodes_m = vec![0.0];
    singleton.samples = vec![vec![t.samples[0][1].clone()]];
    close(lookup(&singleton, 0.3, 0.0, 0.24).unwrap().thrust_n, 33.6);
    assert!(lookup(&singleton, 0.31, 0.0, 0.24).is_err());
    assert!(lookup(&singleton, 0.3, 1.0, 0.24).is_err());
}

#[test]
fn cg_thrust_has_zero_moment_and_offset_inclined_thrust_has_signed_hand_moments() {
    let mut a = jet();
    constant_engine(&mut a, 90.0);
    a.cg_body_m = [1.0, -2.0, 3.0];
    a.engines[0].position_body_m = a.cg_body_m;
    a.engines[0].direction_body_unit = [2.0 / 3.0, -1.0 / 3.0, 2.0 / 3.0];
    let cg = evaluate(&a, 0.12, 1400.0, 0.24).unwrap();
    assert_eq!(cg.loads.force_body_n, Vector3::new(60.0, -30.0, 60.0));
    assert_eq!(cg.loads.moment_body_nm, Vector3::zeros());
    a.engines[0].position_body_m = [3.0, 1.0, -1.0]; // r=[2,3,-4] FROM CG.
    let inclined = evaluate(&a, 0.12, 1400.0, 0.24).unwrap();
    // r x F = [3*60-(-4)*(-30), (-4)*60-2*60, 2*(-30)-3*60].
    assert_eq!(
        inclined.loads.moment_body_nm,
        Vector3::new(60.0, -360.0, -240.0)
    );
    a.engines[0].direction_body_unit = [1.0, 0.0, 0.0];
    assert_eq!(
        evaluate(&a, 0.12, 1400.0, 0.24)
            .unwrap()
            .loads
            .moment_body_nm,
        Vector3::new(0.0, -360.0, -270.0)
    );
}

#[test]
fn afterburner_boundary_sums_and_no_engine_outputs_are_defined() {
    let mut a = jet();
    assert_eq!(evaluate(&a, 0.0, 0.0, 0.6).unwrap().afterburner_engines, 0);
    assert_eq!(
        evaluate(&a, 0.0, 0.0, 0.60001).unwrap().afterburner_engines,
        1
    );
    assert_eq!(evaluate(&a, 0.0, 0.0, 1.0).unwrap().afterburner_engines, 1);
    let one = evaluate(&a, 0.12, 1400.0, 0.24).unwrap();
    a.engines.push(a.engines[0].clone());
    let two = evaluate(&a, 0.12, 1400.0, 0.24).unwrap();
    close(two.thrust_n, 2.0 * one.thrust_n);
    close(two.fuel_flow_kgps, 2.0 * one.fuel_flow_kgps);
    assert_eq!(two.loads.force_body_n, 2.0 * one.loads.force_body_n);
    a.engines.clear();
    let none = evaluate(&a, 0.12, 1400.0, 0.24).unwrap();
    assert_eq!(none.thrust_n, 0.0);
    assert_eq!(none.fuel_flow_kgps, 0.0);
    assert!(evaluate(&a, 0.0, 0.0, f64::NAN).is_err());
}

#[test]
fn engine_records_tables_and_every_sample_reject_positional_arrays_with_paths() {
    let original: Value = serde_json::from_str(JET).unwrap();
    for (path, fields) in [
        (
            "/engines/0",
            vec![
                "name",
                "data_note",
                "position_body_m",
                "direction_body_unit",
                "dry_throttle_max",
                "table",
            ],
        ),
        (
            "/engines/0/table",
            vec![
                "mach_nodes",
                "geometric_altitude_nodes_m",
                "throttle_nodes",
                "samples",
            ],
        ),
    ] {
        let mut v = original.clone();
        let value = v.pointer(path).unwrap();
        let array = Value::Array(fields.iter().map(|k| value[*k].clone()).collect());
        *v.pointer_mut(path).unwrap() = array;
        let e = from_json(&v.to_string()).unwrap_err().to_string();
        assert!(e.contains("object") && e.contains("engines[0]"), "{e}");
    }
    for h in 0..2 {
        for m in 0..3 {
            for t in 0..3 {
                let mut v = original.clone();
                let cell = &mut v["engines"][0]["table"]["samples"][h][m][t];
                *cell = json!([cell["thrust_n"], cell["fuel_flow_kgps"]]);
                let e = from_json(&v.to_string()).unwrap_err().to_string();
                assert!(
                    e.contains("object") && e.contains(&format!("samples[{h}][{m}][{t}]")),
                    "{e}"
                );
            }
        }
    }
}

#[test]
fn invalid_engine_values_shapes_unknown_duplicate_and_missing_fields_remain_errors() {
    let original: Value = serde_json::from_str(JET).unwrap();
    for (path, value, field) in [
        (
            "/engines/0/direction_body_unit",
            json!([2, 0, 0]),
            "direction_body_unit",
        ),
        ("/engines/0/position_body_m", json!([0, 0]), "JSON"),
        ("/engines/0/dry_throttle_max", json!(1), "dry_throttle_max"),
        (
            "/engines/0/dry_throttle_max",
            json!(0.5),
            "dry_throttle_max",
        ),
        (
            "/engines/0/table/mach_nodes",
            json!([0.0, 0.0, 0.8]),
            "mach_nodes",
        ),
        (
            "/engines/0/table/throttle_nodes",
            json!([0.1, 0.6, 1.0]),
            "throttle_nodes",
        ),
        ("/engines/0/table/samples/0/0", json!([]), "samples"),
        (
            "/engines/0/table/samples/0/0/0/fuel_flow_kgps",
            json!(-1),
            "fuel_flow_kgps",
        ),
        (
            "/engines/0/table/samples/1/2/2/thrust_n",
            json!(-1),
            "thrust_n",
        ),
    ] {
        let mut v = original.clone();
        *v.pointer_mut(path).unwrap() = value;
        let e = from_json(&v.to_string()).unwrap_err().to_string();
        assert!(e.contains(field), "{e}");
    }
    for input in [
        JET.replacen("\"thrust_n\"", "\"thrust_lbf\"", 1),
        JET.replacen(
            "\"dry_throttle_max\": 0.6,",
            "\"dry_throttle_max\": 0.6, \"dry_throttle_max\": 0.6,",
            1,
        ),
    ] {
        let e = from_json(&input).unwrap_err().to_string();
        assert!(e.contains("line"), "{e}");
    }
    let mut a = jet();
    a.engines[0].table.samples[1][2][2].fuel_flow_kgps = f64::NAN;
    assert!(to_json(&a).is_err());
    let mut v = original;
    v["engines"][0].as_object_mut().unwrap().remove("table");
    assert!(
        from_json(&v.to_string())
            .unwrap_err()
            .to_string()
            .contains("missing field `table`")
    );
}

#[test]
fn coupled_loads_add_engine_force_moment_and_preserve_constant_mass() {
    let a = jet();
    let before = a.clone();
    let state = State {
        position_ned_m: Vector3::new(0.0, 0.0, -1000.0),
        velocity_body_mps: Vector3::new(20.0, 0.0, 2.0),
        attitude_body_to_ned: Default::default(),
        angular_rates_body_radps: Vector3::zeros(),
    };
    let out = evaluate_flight(&a, &state, Controls::neutral(), 0.7).unwrap();
    assert_eq!(
        out.loads.force_body_n,
        out.aero.loads.force_body_n + out.propulsion.loads.force_body_n
    );
    assert_eq!(
        out.loads.moment_body_nm,
        out.aero.loads.moment_body_nm + out.propulsion.loads.moment_body_nm
    );
    assert!(out.propulsion.fuel_flow_kgps > 0.0);
    assert_eq!(a, before);
}
