//! ALL configuration/query/expected numbers are SYNTHETIC TEST VALUES listed
//! in fixtures/synthetic_dynamic_test_values.md. Existing sourced sea-level
//! density is reused independently of the atmosphere function under test.
use nalgebra::Vector3;
use soas_core::{
    aerodynamics::{Controls, evaluate, lookup_coefficients, step_aircraft},
    aircraft::{AeroCoefficients, Aircraft, from_json},
    attitude::Attitude,
    dynamics::{FixedStepRk4, Loads, State},
    units::Radians,
};
fn rad(v: f64) -> Radians {
    Radians::new(v).unwrap()
}
fn fixture() -> Aircraft {
    from_json(include_str!("fixtures/synthetic_dynamic_aircraft.json")).unwrap()
}
fn uniform(change: impl FnOnce(&mut AeroCoefficients)) -> Aircraft {
    let mut a = fixture();
    let mut c = a.aerodynamics.coefficients[0][0];
    c.cm0 = 0.0;
    c.cm_alpha_per_rad = 0.0;
    c.cm_elevator_per_rad = 0.0;
    c.roll_aileron_per_rad = 0.0;
    c.yaw_rudder_per_rad = 0.0;
    c.cy_beta_per_rad = 0.0;
    c.roll_beta_per_rad = 0.0;
    c.yaw_beta_per_rad = 0.0;
    c.cm_q = 0.0;
    c.roll_p = 0.0;
    c.yaw_r = 0.0;
    c.roll_r = 0.0;
    c.yaw_p = 0.0;
    change(&mut c);
    for row in &mut a.aerodynamics.coefficients {
        row.fill(c);
    }
    a
}
fn state(beta: f64, rates: Vector3<f64>) -> State {
    State {
        position_ned_m: Vector3::zeros(),
        velocity_body_mps: Vector3::new(20.0 * beta.cos(), 20.0 * beta.sin(), 0.0),
        attitude_body_to_ned: Attitude::identity(),
        angular_rates_body_radps: rates,
    }
}
fn check(
    change: impl FnOnce(&mut AeroCoefficients),
    beta: f64,
    rates: Vector3<f64>,
    expected: Vector3<f64>,
) {
    let out = evaluate(&uniform(change), &state(beta, rates), Controls::neutral()).unwrap();
    assert!(
        (out.loads.moment_body_nm - expected).norm() < 1e-11,
        "{:?} != {expected:?}",
        out.loads.moment_body_nm
    );
}
// Independent scales: rho=1.224999155887712, V=20, S=2, b=6, c=0.5.
// qS=489.9996623550848; qSb=2939.9979741305088; qSc=244.9998311775424.
// p=0.8,q=0.6,r=-0.4 -> phat=0.12, qhat=0.0075, rhat=-0.06.
#[test]
fn roll_beta_has_hand_magnitude() {
    check(
        |c| c.roll_beta_per_rad = -0.12,
        0.1,
        Vector3::zeros(),
        Vector3::new(-35.279_975_689_566_11, 0.0, 0.0),
    );
}
#[test]
fn yaw_beta_has_hand_magnitude() {
    check(
        |c| c.yaw_beta_per_rad = 0.18,
        0.1,
        Vector3::zeros(),
        Vector3::new(0.0, 0.0, 52.919_963_534_349_15),
    );
}
#[test]
fn pitch_damping_has_hand_chord_and_rate_magnitude() {
    check(
        |c| c.cm_q = -3.0,
        0.0,
        Vector3::new(0.0, 0.6, 0.0),
        Vector3::new(0.0, -5.512496201494704, 0.0),
    );
}
#[test]
fn roll_damping_has_hand_span_and_rate_magnitude() {
    check(
        |c| c.roll_p = -0.4,
        0.0,
        Vector3::new(0.8, 0.0, 0.0),
        Vector3::new(-141.11990275826442, 0.0, 0.0),
    );
}
#[test]
fn yaw_damping_has_hand_span_and_rate_magnitude() {
    check(
        |c| c.yaw_r = -0.7,
        0.0,
        Vector3::new(0.0, 0.0, -0.4),
        Vector3::new(0.0, 0.0, 123.47991491348137),
    );
}
#[test]
fn roll_due_to_yaw_rate_has_hand_magnitude() {
    check(
        |c| c.roll_r = 0.09,
        0.0,
        Vector3::new(0.0, 0.0, -0.4),
        Vector3::new(-15.875989060304748, 0.0, 0.0),
    );
}
#[test]
fn yaw_due_to_roll_rate_has_hand_magnitude() {
    check(
        |c| c.yaw_p = -0.08,
        0.0,
        Vector3::new(0.8, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -28.223980551652884),
    );
}
#[test]
fn side_beta_force_has_an_independent_wind_axis_magnitude() {
    let out = evaluate(
        &uniform(|c| c.cy_beta_per_rad = -0.2),
        &state(0.1, Vector3::zeros()),
        Controls::neutral(),
    )
    .unwrap();
    // qS * Cy_beta * beta = 489.9996623550848 * (-0.2) * 0.1.
    assert!((out.side_n + 9.799993247101697).abs() < 1e-11);
}
#[test]
fn every_negative_damping_derivative_opposes_its_positive_and_negative_rate() {
    let a = uniform(|c| {
        c.cm_q = -3.0;
        c.roll_p = -0.4;
        c.yaw_r = -0.7;
    });
    for axis in 0..3 {
        for rate in [-0.5, 0.5] {
            let mut rates = Vector3::zeros();
            rates[axis] = rate;
            let out = evaluate(&a, &state(0.0, rates), Controls::neutral()).unwrap();
            assert!(out.loads.moment_body_nm[axis] * rate < 0.0);
        }
    }
}
#[test]
fn rate_terms_are_zero_below_threshold_and_active_at_threshold_nonfinite_errors() {
    let a = uniform(|c| {
        c.cm_q = -3.0;
        c.roll_p = -0.4;
        c.yaw_r = -0.7;
    });
    for speed in [0.0, 0.099] {
        let mut s = state(0.0, Vector3::new(0.8, 0.6, -0.4));
        s.velocity_body_mps = Vector3::new(speed, 0.0, 0.0);
        assert_eq!(
            evaluate(&a, &s, Controls::neutral())
                .unwrap()
                .loads
                .moment_body_nm,
            Vector3::zeros()
        );
    }
    let mut s = state(0.0, Vector3::new(0.8, 0.6, -0.4));
    s.velocity_body_mps = Vector3::new(0.1, 0.0, 0.0);
    assert!(
        evaluate(&a, &s, Controls::neutral())
            .unwrap()
            .loads
            .moment_body_nm
            .norm()
            > 0.0
    );
    s.angular_rates_body_radps.x = f64::NAN;
    assert!(evaluate(&a, &s, Controls::neutral()).is_err());
}
#[test]
fn every_new_coefficient_interpolates_off_centre_with_a_mixed_term() {
    let a = fixture();
    let c = lookup_coefficients(&a.aerodynamics, 0.2, 12000.0).unwrap();
    // Weights [0.30,0.10,0.45,0.15] on corner multipliers [1,2,3,7] give 2.9.
    for (actual, expected) in [
        (c.roll_beta_per_rad, -0.348),
        (c.yaw_beta_per_rad, 0.522),
        (c.cm_q, -8.7),
        (c.roll_p, -1.16),
        (c.yaw_r, -2.03),
        (c.roll_r, 0.261),
        (c.yaw_p, -0.232),
    ] {
        assert!((actual - expected).abs() < 1e-13, "{actual} != {expected}");
    }
    // Lower limit weights: 0.30*(-1.1)+0.10*(-1.3)+0.45*(-1.6)+0.15*(-2.4)=-1.54.
    assert!((c.cl_min.unwrap() + 1.54).abs() < 1e-13);
}
#[test]
fn both_lift_limits_include_the_boundary_and_leave_inside_values_unclipped() {
    let a = uniform(|c| {
        c.cl0 = 0.0;
        c.cl_alpha_per_rad = 4.0;
        c.cl_min = Some(-1.0);
        c.cl_max = 1.0;
    });
    for (alpha, cl, limited) in [
        (-0.251_f64, -1.0, true),
        (-0.25, -1.0, true),
        (-0.249, -0.996, false),
        (0.249, 0.996, false),
        (0.25, 1.0, true),
        (0.251, 1.0, true),
    ] {
        let mut s = state(0.0, Vector3::zeros());
        s.velocity_body_mps = Vector3::new(20.0 * alpha.cos(), 0.0, 20.0 * alpha.sin());
        let out = evaluate(&a, &s, Controls::neutral()).unwrap();
        assert!((out.cl - cl).abs() < 1e-13);
        assert_eq!(out.stalled, limited);
    }
}
#[test]
fn all_surfaces_saturate_asymmetrically_and_report_both_controls() {
    let a = fixture();
    for (command, expected) in [
        (
            Controls {
                elevator_rad: rad(1.0),
                aileron_rad: rad(-1.0),
                rudder_rad: rad(1.0),
            },
            Controls {
                elevator_rad: rad(0.18),
                aileron_rad: rad(-0.22),
                rudder_rad: rad(0.27),
            },
        ),
        (
            Controls {
                elevator_rad: rad(-1.0),
                aileron_rad: rad(1.0),
                rudder_rad: rad(-1.0),
            },
            Controls {
                elevator_rad: rad(-0.25),
                aileron_rad: rad(0.16),
                rudder_rad: rad(-0.3),
            },
        ),
    ] {
        let s = state(0.0, Vector3::zeros());
        let out = evaluate(&a, &s, command).unwrap();
        assert_eq!(out.commanded_controls, command);
        assert_eq!(out.applied_controls, expected);
        assert_eq!(out.loads, evaluate(&a, &s, expected).unwrap().loads);
    }
    let neutral = Controls::neutral();
    assert_eq!(
        evaluate(&a, &state(0.0, Vector3::zeros()), neutral)
            .unwrap()
            .applied_controls,
        neutral
    );
}
#[test]
fn isolated_roll_subsidence_matches_independent_exponential_at_fixed_airspeed() {
    let a = uniform(|c| c.roll_p = -0.4);
    // K = qSb * b/(2V) * Cl_p = -176.39987844783052 Nm/(rad/s).
    // tau = -Ixx/K = 8/176.39987844783052 s; p(t)=0.8*exp(-t/tau).
    let tau = 8.0 / 176.39987844783052;
    let mut s = state(0.0, Vector3::new(0.8, 0.0, 0.0));
    let rk = FixedStepRk4::new(a.mass_properties().unwrap(), 0.0025).unwrap();
    for _ in 0..80 {
        s = rk
            .try_step(&s, |stage| {
                // Isolate rotation: fixed V=20 and h=0 in the aero evaluation.
                // Cancel gravity; apply only aero's roll moment, no aero force.
                let mut proxy = *stage;
                proxy.velocity_body_mps = Vector3::new(20.0, 0.0, 0.0);
                proxy.position_ned_m = Vector3::zeros();
                let moment = evaluate(&a, &proxy, Controls::neutral())?
                    .loads
                    .moment_body_nm;
                Ok::<_, soas_core::aerodynamics::AeroError>(Loads {
                    force_body_n: -stage.attitude_body_to_ned.inverse_transform_vector(
                        &(Vector3::z() * a.mass_kg * soas_core::atmosphere::STANDARD_GRAVITY_MPS2),
                    ),
                    moment_body_nm: moment,
                })
            })
            .unwrap();
    }
    let expected = 0.8 * (-0.2_f64 / tau).exp();
    let error = (s.angular_rates_body_radps.x - expected).abs();
    println!(
        "roll tau={tau:0.12}, p(0.2)={:0.12}, analytic={expected:0.12}, error={error:0.12e}",
        s.angular_rates_body_radps.x
    );
    assert!(error < 5e-9);
    assert_eq!(s.angular_rates_body_radps.y, 0.0);
    assert_eq!(s.angular_rates_body_radps.z, 0.0);
}
#[test]
fn damping_inclusive_aero_coupled_step_halving_has_measured_convergence() {
    let a = fixture();
    let mut s = state(0.05, Vector3::new(0.4, -0.2, 0.3));
    s.position_ned_m.z = -1000.0;
    let run = |dt: f64| {
        let mut current = s;
        for _ in 0..(0.25 / dt).round() as usize {
            current = step_aircraft(&a, &current, Controls::neutral(), dt).unwrap();
        }
        current
    };
    let reference = run(0.000078125);
    let finer = run(0.0000390625);
    let error = |end: State| {
        let parts = [
            (end.position_ned_m - reference.position_ned_m).norm(),
            (end.velocity_body_mps - reference.velocity_body_mps).norm(),
            end.attitude_body_to_ned
                .angle_to(&reference.attitude_body_to_ned),
            (end.angular_rates_body_radps - reference.angular_rates_body_radps).norm(),
        ];
        (parts, parts.iter().map(|x| x * x).sum::<f64>().sqrt())
    };
    let mut errors = Vec::new();
    for dt in [0.01, 0.005, 0.0025, 0.00125] {
        let (parts, norm) = error(run(dt));
        println!("damped dt={dt}: components={parts:?}, norm={norm:0.12e}");
        errors.push(norm);
    }
    for pair in errors.windows(2) {
        let ratio = pair[0] / pair[1];
        println!("damped ratio={ratio:0.6}, order={:0.6}", ratio.log2());
        assert!((12.0..22.0).contains(&ratio));
    }
    assert!(errors[2] < 1e-5);
    assert!(error(finer).1 < 0.05 * errors[2]);
}
