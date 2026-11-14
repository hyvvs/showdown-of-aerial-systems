//! All engine, geometry and target-condition numbers are SYNTHETIC TEST VALUES.
use nalgebra::Vector3;
use soas_core::{
    aerodynamics::Controls,
    aircraft::{Aircraft, from_json},
    atmosphere::{STANDARD_GRAVITY_MPS2 as G, isa_at_geometric_altitude},
    propulsion::evaluate_flight,
    trim::{TrimError, TrimMode, TrimOptions, TrimRequest, solve_trim},
};
use std::f64::consts::PI;
fn case(inclined: bool) -> (Aircraft, TrimRequest, f64, f64) {
    let mut a = from_json(include_str!("fixtures/synthetic_jet.json")).unwrap();
    let c = a.aerodynamics.coefficients[0][0];
    for row in &mut a.aerodynamics.coefficients {
        row.fill(c);
    }
    for row in &mut a.engines[0].table.samples {
        for column in row {
            for (sample, t) in column.iter_mut().zip([0.0, 0.6, 1.0]) {
                sample.thrust_n = 100.0 * t;
                sample.fuel_flow_kgps = 0.2 * t;
            }
        }
    }
    let alpha: f64 = 0.1;
    let qs = 0.5 * isa_at_geometric_altitude(1000.0).unwrap().density_kg_m3 * 400.0 * 2.0;
    let drag = qs * (0.02 + 0.36 / (PI * 0.8 * 8.0));
    let (along, up, moment_arm) = if inclined {
        a.engines[0].position_body_m = [2.0, 0.0, 0.3];
        a.engines[0].direction_body_unit = [0.8, 0.0, -0.6];
        (
            0.8 * alpha.cos() - 0.6 * alpha.sin(),
            0.8 * alpha.sin() + 0.6 * alpha.cos(),
            1.44,
        )
    } else {
        (alpha.cos(), alpha.sin(), 0.0)
    };
    let thrust = drag / along;
    a.mass_kg = (qs * 0.6 + thrust * up) / G;
    // M_engine=(.3*.8 - 2*(-.6))*T=1.44*T for inclined case.
    let elevator = -0.04 + moment_arm * thrust / (qs * 0.5);
    (
        a,
        TrimRequest {
            airspeed_mps: 20.0,
            geometric_altitude_m: 1000.0,
            mode: TrimMode::Level,
        },
        thrust / 100.0,
        elevator,
    )
}
#[test]
fn level_trim_solves_throttle_and_all_loads_for_cg_and_inclined_offset_engines() {
    for inclined in [false, true] {
        let (a, r, throttle, elevator) = case(inclined);
        let s = solve_trim(&a, r, TrimOptions::default()).unwrap();
        assert!((s.condition.alpha_rad.value() - 0.1).abs() < 1e-10);
        assert!((s.condition.elevator_rad.value() - elevator).abs() < 1e-10);
        assert!((s.condition.throttle - throttle).abs() < 1e-10);
        assert_eq!(s.condition.gamma_rad.value(), 0.0);
        let out = evaluate_flight(
            &a,
            &s.state,
            Controls {
                elevator_rad: s.condition.elevator_rad,
                ..Controls::neutral()
            },
            s.condition.throttle,
        )
        .unwrap();
        let force = out.loads.force_body_n
            + s.state
                .attitude_body_to_ned
                .inverse_transform_vector(&(Vector3::z() * a.mass_kg * G));
        assert!(force.norm() < 1e-8, "{force:?}");
        assert!(out.loads.moment_body_nm.norm() < 1e-8);
        assert!(s.residuals.parallel_force_n.abs() < 1e-8);
        assert!(s.residuals.normal_force_n.abs() < 1e-8);
        assert!(s.residuals.pitch_moment_nm.abs() < 1e-8);
        println!(
            "inclined={inclined}: throttle={:.12}, alpha={:.12}, elevator={:.12}, force={:.12e} N, moment={:.12e} Nm",
            s.condition.throttle,
            s.condition.alpha_rad.value(),
            s.condition.elevator_rad.value(),
            force.norm(),
            out.loads.moment_body_nm.norm()
        );
    }
}
#[test]
fn insufficient_thrust_no_engines_and_lateral_imbalance_never_return_best_effort() {
    let (a, r, _, _) = case(false);
    let mut weak = a.clone();
    for row in &mut weak.engines[0].table.samples {
        for col in row {
            for s in col {
                s.thrust_n *= 0.01;
            }
        }
    }
    for bad in [&weak, &{
        let mut empty = a.clone();
        empty.engines.clear();
        empty
    }] {
        assert!(matches!(
            solve_trim(bad, r, TrimOptions::default()),
            Err(TrimError::NonConvergence { .. })
        ));
    }
    let mut lateral = a;
    lateral.engines[0].position_body_m[1] = 0.5;
    assert!(matches!(
        solve_trim(&lateral, r, TrimOptions::default()),
        Err(TrimError::NonConvergence { .. })
    ));
}
#[test]
fn throttle_boundaries_are_valid_initial_guesses_and_nonfinite_guesses_are_errors() {
    let (a, r, expected, _) = case(false);
    for throttle in [0.0, 0.6, 1.0] {
        let mut opts = TrimOptions::default();
        opts.initial.throttle = throttle;
        let s = solve_trim(&a, r, opts).unwrap();
        assert!((s.condition.throttle - expected).abs() < 1e-10);
    }
    for throttle in [f64::NAN, -0.1, 1.1] {
        let mut opts = TrimOptions::default();
        opts.initial.throttle = throttle;
        assert!(solve_trim(&a, r, opts).is_err());
    }
}
