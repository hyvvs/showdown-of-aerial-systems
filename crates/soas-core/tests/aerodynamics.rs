//! ALL aircraft parameters, control inputs, states, durations and numerical
//! bounds are SYNTHETIC TEST VALUES, not real aircraft numbers. The shared JSON
//! fixture also has a per-value ledger. Constants 0.1 m/s and Mach 0.8 are owner
//! policies D13 / TASKS.md. Gravity/ISA density are sourced by the T3 tests.
use nalgebra::{UnitQuaternion, Vector3};
use soas_core::{
    aerodynamics::{
        Controls, MIN_AIRSPEED_FOR_ANGLES_MPS, air_angles, beta_from_ratio, evaluate,
        lookup_coefficients, stability_to_body, step_aircraft, wind_to_body,
    },
    aircraft::{Aircraft, from_json},
    atmosphere::{STANDARD_GRAVITY_MPS2 as G, isa_at_geometric_altitude},
    dynamics::{FixedStepRk4, State},
    units::Radians,
};
use std::f64::consts::{FRAC_PI_2, PI};

fn fixture() -> Aircraft {
    from_json(include_str!("fixtures/synthetic_aircraft.json")).unwrap()
}
fn rad(value: f64) -> Radians {
    Radians::new(value).unwrap()
}
fn state(velocity: Vector3<f64>, altitude_m: f64) -> State {
    State {
        position_ned_m: Vector3::new(0.0, 0.0, -altitude_m),
        velocity_body_mps: velocity,
        attitude_body_to_ned: UnitQuaternion::identity(),
        angular_rates_body_radps: Vector3::zeros(),
    }
}
fn constant_coefficients() -> Aircraft {
    let mut aircraft = fixture();
    let cell = aircraft.aerodynamics.coefficients[0][0];
    for row in &mut aircraft.aerodynamics.coefficients {
        row.fill(cell);
    }
    aircraft
}
fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual:.15e} != {expected:.15e}"
    );
}
fn vector_close(actual: Vector3<f64>, expected: Vector3<f64>) {
    assert!(
        (actual - expected).norm() < 1e-11,
        "{actual:?} != {expected:?}"
    );
}

#[test]
fn table_lookup_is_exact_at_every_node() {
    let a = fixture();
    for (j, h) in [0.0, 20000.0].into_iter().enumerate() {
        for (i, mach) in [0.0, 0.8].into_iter().enumerate() {
            assert_eq!(
                lookup_coefficients(&a.aerodynamics, mach, h).unwrap(),
                a.aerodynamics.coefficients[j][i]
            );
        }
    }
}

#[test]
fn signed_zero_at_sea_level_and_zero_mach_matches_the_zero_node() {
    let a = fixture();
    assert_eq!(
        lookup_coefficients(&a.aerodynamics, -0.0, -0.0).unwrap(),
        a.aerodynamics.coefficients[0][0]
    );
    let mut sea_level = state(Vector3::zeros(), 0.0);
    sea_level.position_ned_m.z = 0.0; // h=-z produces -0.0.
    assert!(evaluate(&a, &sea_level, Controls::neutral()).is_ok());
}

#[test]
fn table_interpolation_is_linear_along_each_axis_and_bilinear_inside() {
    let a = fixture();
    for (mach, h, cl0, slope, max) in [
        (0.4, 0.0, 0.3, 5.0, 1.3),
        (0.0, 10000.0, 0.25, 4.5, 1.25),
        (0.4, 10000.0, 0.35, 5.5, 1.35),
    ] {
        let c = lookup_coefficients(&a.aerodynamics, mach, h).unwrap();
        close(c.cl0, cl0, 1e-14);
        close(c.cl_alpha_per_rad, slope, 1e-14);
        close(c.cl_max, max, 1e-14);
        close(c.cd0, 0.02, 1e-14);
        close(c.oswald_efficiency, 0.8, 1e-14);
        close(c.cm0, 0.01, 1e-14);
        close(c.cm_alpha_per_rad, -0.5, 1e-14);
        close(c.cm_elevator_per_rad, -1.0, 1e-14);
        close(c.cy_beta_per_rad, -0.2, 1e-14);
        close(c.roll_aileron_per_rad, 0.2, 1e-14);
        close(c.yaw_rudder_per_rad, -0.15, 1e-14);
    }
}

#[test]
fn table_bounds_invalid_inputs_and_singleton_axes_follow_the_contract() {
    let mut a = fixture();
    for (mach, h) in [
        (-0.01, 0.0),
        (0.81, 0.0),
        (0.4, -1.0),
        (0.4, 20001.0),
        (f64::NAN, 0.0),
        (0.4, f64::INFINITY),
    ] {
        assert!(lookup_coefficients(&a.aerodynamics, mach, h).is_err());
    }
    a.aerodynamics.geometric_altitude_nodes_m = None;
    a.aerodynamics.coefficients.truncate(1);
    assert_eq!(
        lookup_coefficients(&a.aerodynamics, 0.4, 0.0).unwrap(),
        lookup_coefficients(&a.aerodynamics, 0.4, 20000.0).unwrap()
    );
    a.aerodynamics.mach_nodes.truncate(1);
    a.aerodynamics.coefficients[0].truncate(1);
    assert!(lookup_coefficients(&a.aerodynamics, 0.0, 1000.0).is_ok());
    assert!(lookup_coefficients(&a.aerodynamics, 0.001, 1000.0).is_err());
    a.aerodynamics.coefficients[0].clear();
    assert!(lookup_coefficients(&a.aerodynamics, 0.0, 0.0).is_err());
}

#[test]
fn interpolation_applies_to_every_coefficient_not_only_lift() {
    let mut a = fixture();
    for (index, c) in a.aerodynamics.coefficients.iter_mut().flatten().enumerate() {
        let n = (index + 1) as f64; // Synthetic node labels 1..4.
        c.cd0 = 0.01 * n;
        c.oswald_efficiency = 0.4 + 0.1 * n;
        c.cm0 = 0.01 * n;
        c.cm_alpha_per_rad = -0.1 * n;
        c.cm_elevator_per_rad = -0.4 - 0.1 * n;
        c.cy_beta_per_rad = -0.01 * n;
        c.roll_aileron_per_rad = 0.1 * n;
        c.yaw_rudder_per_rad = -0.1 * n;
    }
    let c = lookup_coefficients(&a.aerodynamics, 0.4, 10000.0).unwrap();
    for (actual, expected) in [
        (c.cd0, 0.025),
        (c.oswald_efficiency, 0.65),
        (c.cm0, 0.025),
        (c.cm_alpha_per_rad, -0.25),
        (c.cm_elevator_per_rad, -0.65),
        (c.cy_beta_per_rad, -0.025),
        (c.roll_aileron_per_rad, 0.25),
        (c.yaw_rudder_per_rad, -0.25),
    ] {
        close(actual, expected, 1e-14);
    }
}

#[test]
fn low_speed_angles_use_zero_fallback_strictly_below_owner_threshold() {
    assert_eq!(MIN_AIRSPEED_FOR_ANGLES_MPS, 0.1);
    for velocity in [
        Vector3::zeros(),
        Vector3::new(0.0, 0.099, 0.0),
        Vector3::new(-0.099, 0.0, 0.0),
    ] {
        let a = air_angles(velocity).unwrap();
        assert_eq!(a.alpha_rad.value(), 0.0);
        assert_eq!(a.beta_rad.value(), 0.0);
    }
    let a = air_angles(Vector3::new(0.0, 0.1, 0.0)).unwrap();
    close(a.beta_rad.value(), FRAC_PI_2, 1e-14);
    let a = air_angles(Vector3::new(0.0, 0.0, 0.1)).unwrap();
    close(a.alpha_rad.value(), FRAC_PI_2, 1e-14);
    let a = air_angles(Vector3::new(-1.0, 0.0, 1.0)).unwrap();
    close(a.alpha_rad.value(), 3.0 * PI / 4.0, 1e-14);
}

#[test]
fn asin_roundoff_is_clamped_but_non_finite_inputs_are_errors() {
    close(
        beta_from_ratio(1.0 + f64::EPSILON).unwrap().value(),
        FRAC_PI_2,
        1e-14,
    );
    close(
        beta_from_ratio(-1.0 - f64::EPSILON).unwrap().value(),
        -FRAC_PI_2,
        1e-14,
    );
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(beta_from_ratio(invalid).is_err());
        for velocity in [
            Vector3::new(invalid, 0.0, 0.0),
            Vector3::new(0.0, invalid, 0.0),
            Vector3::new(0.0, 0.0, invalid),
        ] {
            assert!(air_angles(velocity).is_err());
        }
    }
    assert!(air_angles(Vector3::repeat(f64::MAX)).is_err());
}

#[test]
fn wind_force_signs_and_stability_rotations_match_each_axis() {
    vector_close(
        wind_to_body(rad(0.0), rad(0.0), Vector3::new(-2.0, 3.0, -4.0)).unwrap(),
        Vector3::new(-2.0, 3.0, -4.0),
    );
    vector_close(
        stability_to_body(rad(FRAC_PI_2), Vector3::x()).unwrap(),
        Vector3::z(),
    );
    vector_close(
        stability_to_body(rad(FRAC_PI_2), Vector3::y()).unwrap(),
        Vector3::y(),
    );
    vector_close(
        stability_to_body(rad(FRAC_PI_2), Vector3::z()).unwrap(),
        -Vector3::x(),
    );
    vector_close(
        wind_to_body(rad(0.0), rad(FRAC_PI_2), Vector3::x()).unwrap(),
        Vector3::y(),
    );
    vector_close(
        wind_to_body(rad(0.0), rad(FRAC_PI_2), Vector3::y()).unwrap(),
        -Vector3::x(),
    );
    vector_close(
        wind_to_body(rad(0.0), rad(FRAC_PI_2), Vector3::z()).unwrap(),
        Vector3::z(),
    );
    // Positive alpha: upward lift has a forward body-x component.
    vector_close(
        wind_to_body(rad(FRAC_PI_2), rad(0.0), -Vector3::z()).unwrap(),
        Vector3::x(),
    );
}

#[test]
fn wind_x_follows_velocity_and_transform_preserves_orthogonal_right_handed_axes() {
    let velocity = Vector3::new(4.0, 3.0, 2.0);
    let angles = air_angles(velocity).unwrap();
    let x = wind_to_body(angles.alpha_rad, angles.beta_rad, Vector3::x()).unwrap();
    let y = wind_to_body(angles.alpha_rad, angles.beta_rad, Vector3::y()).unwrap();
    let z = wind_to_body(angles.alpha_rad, angles.beta_rad, Vector3::z()).unwrap();
    vector_close(x, velocity / velocity.norm());
    vector_close(x.cross(&y), z);
    close(x.dot(&y), 0.0, 1e-14);
    close(x.norm(), 1.0, 1e-14);
    close(y.norm(), 1.0, 1e-14);
    close(z.norm(), 1.0, 1e-14);
    let stability_y = stability_to_body(angles.alpha_rad, Vector3::y()).unwrap();
    vector_close(stability_y, Vector3::y());
    vector_close(
        z,
        stability_to_body(angles.alpha_rad, Vector3::z()).unwrap(),
    );
    assert!(wind_to_body(rad(0.0), rad(0.0), Vector3::repeat(f64::INFINITY)).is_err());
}

#[test]
fn positive_elevator_produces_negative_pitch_moment() {
    let a = fixture();
    let s = state(Vector3::new(20.0, 0.0, 0.0), 1000.0);
    let base = evaluate(&a, &s, Controls::neutral()).unwrap();
    let out = evaluate(
        &a,
        &s,
        Controls {
            elevator_rad: rad(0.1),
            ..Controls::neutral()
        },
    )
    .unwrap();
    assert!(out.loads.moment_body_nm.y < base.loads.moment_body_nm.y);
    assert!(out.loads.moment_body_nm.y < 0.0);
    close(
        out.loads.moment_body_nm.x,
        base.loads.moment_body_nm.x,
        1e-14,
    );
}

#[test]
fn positive_aileron_produces_positive_roll_moment() {
    let a = fixture();
    let s = state(Vector3::new(20.0, 0.0, 0.0), 1000.0);
    let out = evaluate(
        &a,
        &s,
        Controls {
            aileron_rad: rad(0.1),
            ..Controls::neutral()
        },
    )
    .unwrap();
    assert!(out.loads.moment_body_nm.x > 0.0);
    close(out.loads.moment_body_nm.z, 0.0, 1e-14);
}

#[test]
fn positive_rudder_produces_negative_yaw_moment() {
    let a = fixture();
    let s = state(Vector3::new(20.0, 0.0, 0.0), 1000.0);
    let out = evaluate(
        &a,
        &s,
        Controls {
            rudder_rad: rad(0.1),
            ..Controls::neutral()
        },
    )
    .unwrap();
    assert!(out.loads.moment_body_nm.z < 0.0);
    close(out.loads.moment_body_nm.x, 0.0, 1e-14);
}

#[test]
fn side_force_and_cg_moment_transfer_have_the_expected_signs() {
    let mut a = constant_coefficients();
    let s = state(Vector3::new(20.0, 2.0, 0.0), 0.0);
    let base = evaluate(&a, &s, Controls::neutral()).unwrap();
    assert!(base.side_n < 0.0); // CY_beta=-0.2, beta>0: wind-left.
    // Synthetic aerodynamic point (1,2,3), CG (-1,0,1) in the same datum.
    a.aerodynamic_reference_point_body_m = [1.0, 2.0, 3.0];
    a.cg_body_m = [-1.0, 0.0, 1.0];
    let shifted = evaluate(&a, &s, Controls::neutral()).unwrap();
    vector_close(
        shifted.loads.moment_body_nm - base.loads.moment_body_nm,
        Vector3::new(2.0, 2.0, 2.0).cross(&base.loads.force_body_n),
    );
}

#[test]
fn hand_constructed_lift_equals_weight_at_stated_alpha_and_airspeed() {
    let a = constant_coefficients(); // mass=10 kg, S=2 m^2, CL0=.2, CLalpha=4/rad.
    let alpha: f64 = 0.1;
    let cl = 0.6;
    // Independent T3 sea-level equation value, from the published constants;
    // this is an atmosphere reference, not an aircraft parameter.
    let density = 1.224999155887712;
    let speed = (2.0 * 10.0 * G / (density * 2.0 * cl)).sqrt();
    let s = state(
        Vector3::new(speed * alpha.cos(), 0.0, speed * alpha.sin()),
        0.0,
    );
    let out = evaluate(&a, &s, Controls::neutral()).unwrap();
    println!(
        "hand balance: mass=10 kg, alpha=0.1 rad, V={speed:.12} m/s, expected lift=98.0665 N, actual lift={:.12} N",
        out.lift_n
    );
    close(out.cl, cl, 1e-14);
    close(out.lift_n, 98.0665, 1e-9);
    assert!(out.drag_n > 0.0); // This hand check is not a trim solution (T8).
}

#[test]
fn local_steady_glide_balances_forces_and_gives_the_hand_calculated_lift_drag_ratio() {
    let mut a = constant_coefficients();
    for row in &mut a.aerodynamics.coefficients {
        for c in row {
            c.cl0 = 0.6;
            c.cm0 = 0.0;
        }
    }
    let cd = 0.02 + 0.6_f64.powi(2) / (PI * 0.8 * 8.0);
    let gamma = (cd / 0.6).atan();
    let density = isa_at_geometric_altitude(1000.0).unwrap().density_kg_m3;
    let q_s = 0.5 * density * 20.0_f64.powi(2) * 2.0;
    a.mass_kg = q_s * (0.6_f64.powi(2) + cd * cd).sqrt() / G;
    let mut s = state(Vector3::new(20.0, 0.0, 0.0), 1000.0);
    s.attitude_body_to_ned = UnitQuaternion::from_euler_angles(0.0, -gamma, 0.0);
    let out = evaluate(&a, &s, Controls::neutral()).unwrap();
    close(out.lift_n / out.drag_n, 0.6 / cd, 1e-12);
    close(gamma.tan(), out.drag_n / out.lift_n, 1e-14);
    vector_close(
        out.loads.force_body_n / a.mass_kg
            + s.attitude_body_to_ned
                .inverse_transform_vector(&(Vector3::z() * G)),
        Vector3::zeros(),
    );
    vector_close(out.loads.moment_body_nm, Vector3::zeros());
}

#[test]
fn modelled_one_g_stall_boundary_matches_the_stall_speed_formula() {
    let a = constant_coefficients();
    let rho = isa_at_geometric_altitude(0.0).unwrap().density_kg_m3;
    let analytic = (2.0 * 10.0 * G / (rho * 2.0 * 1.2)).sqrt();
    let alpha: f64 = 0.30; // Beyond upper CL limit (.2+4*.30 > 1.2).
    let max_lift = |speed: f64| {
        let out = evaluate(
            &a,
            &state(
                Vector3::new(speed * alpha.cos(), 0.0, speed * alpha.sin()),
                0.0,
            ),
            Controls::neutral(),
        )
        .unwrap();
        assert!(out.stalled);
        close(out.cl, 1.2, 1e-14);
        out.lift_n
    };
    assert!(max_lift(analytic * 0.99) < 10.0 * G);
    assert!(max_lift(analytic * 1.01) > 10.0 * G);
    let (mut lo, mut hi) = (analytic * 0.9, analytic * 1.1);
    for _ in 0..40 {
        let mid = (lo + hi) / 2.0;
        if max_lift(mid) < 10.0 * G {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    close((lo + hi) / 2.0, analytic, analytic * 1e-10);
    println!(
        "1-g upper-lift-limit boundary: formula={analytic:.12} m/s, model={:.12} m/s",
        (lo + hi) / 2.0
    );
    let below = evaluate(
        &a,
        &state(Vector3::new(20.0, 0.0, 0.0), 0.0),
        Controls::neutral(),
    )
    .unwrap();
    assert!(!below.stalled);
}

#[test]
fn simulator_uses_geometric_height_and_rejects_unsupported_conditions() {
    let a = fixture();
    let s = state(Vector3::new(20.0, 0.0, 0.0), 20000.0);
    let out = evaluate(&a, &s, Controls::neutral()).unwrap();
    let rho = 0.088909915089; // T3 geometric 20 km equation result (rounded).
    close(out.dynamic_pressure_pa, 0.5 * rho * 400.0, 1e-9);
    for h in [-1.0, 20001.0] {
        assert!(evaluate(&a, &state(Vector3::x(), h), Controls::neutral()).is_err());
    }
    let error = evaluate(
        &a,
        &state(Vector3::new(300.0, 0.0, 0.0), 0.0),
        Controls::neutral(),
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("Mach > 0.8 not modeled"));
    let sound = isa_at_geometric_altitude(0.0).unwrap().speed_of_sound_mps;
    assert!(
        evaluate(
            &a,
            &state(Vector3::x() * (0.8 * sound), 0.0),
            Controls::neutral()
        )
        .is_ok()
    );
    let mut invalid = s;
    invalid.velocity_body_mps.x = f64::NAN;
    assert!(evaluate(&a, &invalid, Controls::neutral()).is_err());
    let zero = evaluate(&a, &state(Vector3::zeros(), 0.0), Controls::neutral()).unwrap();
    vector_close(zero.loads.force_body_n, Vector3::zeros());
    vector_close(zero.loads.moment_body_nm, Vector3::zeros());
}

#[test]
fn coupling_propagates_intermediate_aerodynamic_errors_without_advancing_input() {
    let a = fixture();
    let mut s = state(Vector3::new(20.0, 0.0, -10.0), 20000.0);
    s.position_ned_m.z = -19999.999;
    let start = s;
    assert!(step_aircraft(&a, &s, Controls::neutral(), 0.0025).is_err());
    assert_eq!(s, start);
    // Direct fallible RK4 API must also retain the caller's own load error.
    let integrator = FixedStepRk4::with_default_step(a.mass_properties().unwrap());
    let error = integrator
        .try_step(&s, |_| {
            Err::<soas_core::dynamics::Loads, _>("test load error")
        })
        .unwrap_err();
    assert!(error.to_string().contains("test load error"));
}

#[test]
fn aero_coupled_step_halving_converges_with_the_shared_aircraft_and_geometric_atmosphere() {
    let a = fixture();
    let mut start = state(Vector3::new(20.0, 1.0, 2.0), 1000.0);
    start.attitude_body_to_ned = UnitQuaternion::from_euler_angles(0.03, 0.08, 0.2);
    start.angular_rates_body_radps = Vector3::new(0.05, -0.03, 0.02);
    let controls = Controls {
        elevator_rad: rad(-0.02),
        aileron_rad: rad(0.01),
        rudder_rad: rad(-0.01),
    };
    let run = |count: u32| {
        let dt = 0.5 / f64::from(count);
        let mut s = start;
        for _ in 0..count {
            s = step_aircraft(&a, &s, controls, dt).unwrap();
            assert!(!evaluate(&a, &s, controls).unwrap().stalled);
        }
        s
    };
    // Same aerodynamic MODEL at a finer reference step; this is numerical
    // convergence, not real-aircraft validation. No exact solution is claimed.
    let reference = run(3200);
    let mut errors = Vec::new();
    for count in [25, 50, 100, 200, 400] {
        let s = run(count);
        let error = [
            (s.position_ned_m - reference.position_ned_m).norm(),
            (s.velocity_body_mps - reference.velocity_body_mps).norm(),
            s.attitude_body_to_ned
                .angle_to(&reference.attitude_body_to_ned),
            (s.angular_rates_body_radps - reference.angular_rates_body_radps).norm(),
        ];
        println!(
            "aero dt={:.7}: position={:.12e} m, velocity={:.12e} m/s, attitude={:.12e} rad, rates={:.12e} rad/s",
            0.5 / f64::from(count),
            error[0],
            error[1],
            error[2],
            error[3]
        );
        errors.push(error);
    }
    for pair in errors.windows(2) {
        for (i, (coarse, fine)) in pair[0].iter().zip(&pair[1]).enumerate() {
            let ratio = coarse / fine;
            println!("aero halving quantity {i}: {ratio:.6}");
            assert!((14.0..18.0).contains(&ratio), "quantity {i}, ratio={ratio}");
        }
    }
    for (error, bound) in errors[3].into_iter().zip([1e-5, 1e-4, 1e-6, 1e-5]) {
        assert!(error < bound, "default-step error {error} >= {bound}");
    }
    let finer_reference = run(6400);
    let reference_deltas = [
        (reference.position_ned_m - finer_reference.position_ned_m).norm(),
        (reference.velocity_body_mps - finer_reference.velocity_body_mps).norm(),
        reference
            .attitude_body_to_ned
            .angle_to(&finer_reference.attitude_body_to_ned),
        (reference.angular_rates_body_radps - finer_reference.angular_rates_body_radps).norm(),
    ];
    println!(
        "aero finer-reference differences: position={:.12e}, velocity={:.12e}, attitude={:.12e}, rates={:.12e}",
        reference_deltas[0], reference_deltas[1], reference_deltas[2], reference_deltas[3]
    );
    for (delta, default_error) in reference_deltas.into_iter().zip(errors[3]) {
        assert!(
            delta < 0.05 * default_error,
            "reference difference {delta} is too large compared with default error {default_error}"
        );
    }
}
