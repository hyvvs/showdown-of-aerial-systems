//! All aircraft configurations and conditions below are SYNTHETIC TEST VALUES,
//! derived from the existing synthetic fixture. No real aircraft data. Gravity
//! and atmosphere use the sourced T3 model; analytic force/moment expectations
//! are constructed independently before the solver exists.
use nalgebra::Vector3;
use soas_core::{
    aerodynamics::{Controls, evaluate, lookup_coefficients},
    aircraft::{Aircraft, from_json},
    atmosphere::{STANDARD_GRAVITY_MPS2 as G, isa_at_geometric_altitude},
    trim::{TrimCondition, TrimError, TrimOptions, TrimRequest, solve_trim, static_margin},
    units::Radians,
};
use std::f64::consts::PI;
fn rad(x: f64) -> Radians {
    Radians::new(x).unwrap()
}
fn fixture() -> Aircraft {
    from_json(include_str!("fixtures/synthetic_aircraft.json")).unwrap()
}
fn constant_aircraft() -> Aircraft {
    let mut a = fixture();
    let c = a.aerodynamics.coefficients[0][0];
    for row in &mut a.aerodynamics.coefficients {
        row.fill(c);
    }
    a
}
fn request(thrust: Option<f64>) -> TrimRequest {
    TrimRequest {
        airspeed_mps: 20.0,
        geometric_altitude_m: 1000.0,
        applied_thrust_n: thrust,
    }
}
fn analytic_case(level: bool) -> (Aircraft, TrimRequest, f64, f64, f64) {
    let mut a = constant_aircraft();
    let alpha: f64 = 0.1;
    let elevator = -0.04;
    let q_s =
        0.5 * isa_at_geometric_altitude(1000.0).unwrap().density_kg_m3 * 20.0_f64.powi(2) * 2.0;
    let cl = 0.6;
    let cd = 0.02 + cl * cl / (PI * 0.8 * 8.0);
    let lift = q_s * cl;
    let drag = q_s * cd;
    if level {
        let thrust = drag / alpha.cos();
        a.mass_kg = (lift + thrust * alpha.sin()) / G;
        (a, request(Some(thrust)), alpha, elevator, 0.0)
    } else {
        a.mass_kg = lift.hypot(drag) / G;
        (a, request(None), alpha, elevator, -(drag / lift).atan())
    }
}
fn close(x: f64, y: f64, tolerance: f64) {
    assert!((x - y).abs() < tolerance, "{x:.15e} != {y:.15e}");
}

#[test]
fn analytic_power_off_glide_solves_alpha_elevator_and_gamma() {
    let (a, r, alpha, elevator, gamma) = analytic_case(false);
    let solution = solve_trim(&a, r, TrimOptions::default()).unwrap();
    close(solution.condition.alpha_rad.value(), alpha, 1e-10);
    close(solution.condition.elevator_rad.value(), elevator, 1e-10);
    close(solution.condition.gamma_rad.value(), gamma, 1e-10);
    assert_eq!(solution.applied_thrust_n, 0.0);
    assert!(gamma < 0.0);
    println!(
        "power-off analytic trim: alpha={:.12} rad, elevator={:.12} rad, gamma={:.12} rad, iterations={}",
        solution.condition.alpha_rad.value(),
        solution.condition.elevator_rad.value(),
        solution.condition.gamma_rad.value(),
        solution.iterations
    );
}

#[test]
fn body_x_thrust_chosen_for_level_flight_solves_gamma_zero() {
    let (a, r, alpha, elevator, _) = analytic_case(true);
    let s = solve_trim(&a, r, TrimOptions::default()).unwrap();
    close(s.condition.alpha_rad.value(), alpha, 1e-10);
    close(s.condition.elevator_rad.value(), elevator, 1e-10);
    close(s.condition.gamma_rad.value(), 0.0, 1e-10);
    close(s.applied_thrust_n, r.applied_thrust_n.unwrap(), 1e-14);
    println!(
        "level trim: thrust={:.12} N, gamma={:.12e} rad",
        s.applied_thrust_n,
        s.condition.gamma_rad.value()
    );
}

#[test]
fn complete_body_force_and_all_moment_balances_are_tight_at_the_solution() {
    for level in [false, true] {
        let (a, r, _, _, _) = analytic_case(level);
        let s = solve_trim(&a, r, TrimOptions::default()).unwrap();
        let output = evaluate(
            &a,
            &s.state,
            Controls {
                elevator_rad: s.condition.elevator_rad,
                ..Controls::neutral()
            },
        )
        .unwrap();
        let gravity = s
            .state
            .attitude_body_to_ned
            .inverse_transform_vector(&(Vector3::z() * (a.mass_kg * G)));
        let force = output.loads.force_body_n + Vector3::x() * s.applied_thrust_n + gravity;
        assert!(force.norm() < 1e-8, "{force:?}");
        assert!(output.loads.moment_body_nm.norm() < 1e-8);
        assert!(s.residuals.parallel_force_n.abs() < 1e-8);
        assert!(s.residuals.normal_force_n.abs() < 1e-8);
        assert!(s.residuals.pitch_moment_nm.abs() < 1e-8);
        let world = s
            .state
            .attitude_body_to_ned
            .transform_vector(&s.state.velocity_body_mps);
        close(
            world.z,
            -r.airspeed_mps * s.condition.gamma_rad.value().sin(),
            1e-11,
        );
        println!(
            "trim balance: force norm={:.12e} N, moment norm={:.12e} Nm",
            force.norm(),
            output.loads.moment_body_nm.norm()
        );
    }
}

#[test]
fn static_margin_from_finite_differences_matches_the_analytic_stable_value() {
    let (a, r, _, _, _) = analytic_case(false);
    let s = solve_trim(&a, r, TrimOptions::default()).unwrap();
    for h in [1e-4, 1e-5, 1e-6] {
        let margin = static_margin(&a, r, s.condition, rad(h)).unwrap();
        close(margin.d_cl_d_alpha_per_rad, 4.0, 1e-8);
        close(margin.d_cm_d_alpha_per_rad, -0.5, 1e-8);
        close(margin.margin_chord_fraction, 0.125, 1e-9);
    }
}

#[test]
fn an_unstable_configuration_reports_negative_static_margin() {
    let (mut a, r, _, _, _) = analytic_case(false);
    for c in a.aerodynamics.coefficients.iter_mut().flatten() {
        c.cm_alpha_per_rad = 0.5;
    }
    let s = solve_trim(&a, r, TrimOptions::default()).unwrap();
    let m = static_margin(&a, r, s.condition, rad(1e-5)).unwrap();
    close(m.margin_chord_fraction, -0.125, 1e-9);
    assert!(m.margin_chord_fraction < 0.0);
    println!(
        "unstable synthetic configuration: margin={:.12} chord",
        m.margin_chord_fraction
    );
}

#[test]
fn cg_moment_transfer_is_included_in_trim_and_static_margin_derivatives() {
    let (mut a, r, alpha, _, gamma) = analytic_case(false);
    a.cg_body_m = [-0.05, 0.0, -0.02];
    let s = solve_trim(&a, r, TrimOptions::default()).unwrap();
    close(s.condition.alpha_rad.value(), alpha, 1e-10);
    close(s.condition.gamma_rad.value(), gamma, 1e-10);
    let cl = 0.6;
    let cd = 0.02 + cl * cl / (PI * 0.8 * 8.0);
    let cl_prime = 4.0;
    let cd_prime = 2.0 * cl * cl_prime / (PI * 0.8 * 8.0);
    let (sin, cos) = alpha.sin_cos();
    let cx = -cd * cos + cl * sin;
    let cz = -cd * sin - cl * cos;
    let expected_elevator = 0.01 - 0.5 * alpha + (0.02 * cx - 0.05 * cz) / 0.5;
    close(s.condition.elevator_rad.value(), expected_elevator, 1e-10);
    let cx_prime = -cd_prime * cos + cd * sin + cl_prime * sin + cl * cos;
    let cz_prime = -cd_prime * sin - cd * cos - cl_prime * cos + cl * sin;
    let expected_cm_prime = -0.5 + (0.02 * cx_prime - 0.05 * cz_prime) / 0.5;
    let m = static_margin(&a, r, s.condition, rad(1e-5)).unwrap();
    close(m.d_cm_d_alpha_per_rad, expected_cm_prime, 1e-8);
    close(m.margin_chord_fraction, -expected_cm_prime / cl_prime, 1e-8);
}

#[test]
fn trim_uses_current_mach_altitude_table_values_and_geometric_atmosphere() {
    let mut a = fixture();
    let h = 10000.0;
    let atmosphere = isa_at_geometric_altitude(h).unwrap();
    let speed = 0.4 * atmosphere.speed_of_sound_mps;
    let c = lookup_coefficients(&a.aerodynamics, 0.4, h).unwrap();
    let alpha = 0.05;
    let cl = c.cl0 + c.cl_alpha_per_rad * alpha;
    let cd = c.cd0 + cl * cl / (PI * c.oswald_efficiency * 8.0);
    let q_s = 0.5 * atmosphere.density_kg_m3 * speed * speed * a.reference_area_m2;
    a.mass_kg = q_s * cl.hypot(cd) / G;
    let r = TrimRequest {
        airspeed_mps: speed,
        geometric_altitude_m: h,
        applied_thrust_n: None,
    };
    let s = solve_trim(&a, r, TrimOptions::default()).unwrap();
    close(s.condition.alpha_rad.value(), alpha, 1e-10);
    let margin = static_margin(&a, r, s.condition, rad(1e-5)).unwrap();
    close(margin.margin_chord_fraction, 0.5 / 5.5, 1e-9);
}

#[test]
fn iteration_limit_and_unsolvable_moment_are_explicit_non_convergence() {
    let (mut a, r, _, _, _) = analytic_case(false);
    assert!(matches!(
        solve_trim(
            &a,
            r,
            TrimOptions {
                max_iterations: 0,
                ..TrimOptions::default()
            }
        ),
        Err(TrimError::NonConvergence { .. })
    ));
    for c in a.aerodynamics.coefficients.iter_mut().flatten() {
        c.cm0 = 0.1;
        c.cm_alpha_per_rad = 0.0;
        c.cm_elevator_per_rad = 0.0;
    }
    let error = solve_trim(&a, r, TrimOptions::default()).unwrap_err();
    assert!(matches!(error, TrimError::NonConvergence { .. }));
    assert!(error.to_string().contains("non-convergence"));
}

#[test]
fn impossible_lift_or_unbalanced_lateral_moments_cannot_return_best_effort_trim() {
    let (a, r, _, _, _) = analytic_case(false);
    let slow = TrimRequest {
        airspeed_mps: 1.0,
        ..r
    };
    assert!(matches!(
        solve_trim(&a, slow, TrimOptions::default()),
        Err(TrimError::NonConvergence { .. })
    ));
    let mut lateral = a;
    lateral.aerodynamic_reference_point_body_m[1] = 0.1;
    assert!(solve_trim(&lateral, r, TrimOptions::default()).is_err());
}

#[test]
fn invalid_conditions_options_and_undefined_stall_margin_are_errors() {
    let (a, r, _, _, _) = analytic_case(false);
    for invalid in [
        TrimRequest {
            airspeed_mps: 0.0,
            ..r
        },
        TrimRequest {
            airspeed_mps: f64::NAN,
            ..r
        },
        TrimRequest {
            geometric_altitude_m: -1.0,
            ..r
        },
        TrimRequest {
            geometric_altitude_m: 20001.0,
            ..r
        },
        TrimRequest {
            applied_thrust_n: Some(f64::INFINITY),
            ..r
        },
        TrimRequest {
            airspeed_mps: 400.0,
            ..r
        },
    ] {
        assert!(solve_trim(&a, invalid, TrimOptions::default()).is_err());
    }
    assert!(
        solve_trim(
            &a,
            r,
            TrimOptions {
                force_tolerance_n: f64::NAN,
                ..TrimOptions::default()
            }
        )
        .is_err()
    );
    let condition = TrimCondition {
        alpha_rad: rad(0.4),
        elevator_rad: rad(0.0),
        gamma_rad: rad(0.0),
    };
    assert!(static_margin(&a, r, condition, rad(1e-5)).is_err());
    assert!(static_margin(&a, r, condition, rad(0.0)).is_err());
}
