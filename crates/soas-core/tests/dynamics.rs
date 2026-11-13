//! All masses, inertias, loads, rates and starting states here are synthetic
//! mathematical test values, not parameters of a real aircraft. Analytic
//! expectations and sources are described in data/reference/dynamics/README.md.

use nalgebra::{Matrix3, Quaternion, UnitQuaternion, Vector3};
use soas_core::atmosphere::STANDARD_GRAVITY_MPS2 as G;
use soas_core::dynamics::{
    DEFAULT_STEP_S, DynamicsError, FixedStepRk4, Loads, MassProperties, State,
};

fn body(diagonal: Vector3<f64>) -> MassProperties {
    MassProperties::from_signed_tensor(2.0, Matrix3::from_diagonal(&diagonal)).unwrap()
}

fn initial_state() -> State {
    State {
        position_ned_m: Vector3::zeros(),
        velocity_body_mps: Vector3::zeros(),
        attitude_body_to_ned: UnitQuaternion::identity(),
        angular_rates_body_radps: Vector3::zeros(),
    }
}

fn no_loads(_: &State) -> Loads {
    Loads {
        force_body_n: Vector3::zeros(),
        moment_body_nm: Vector3::zeros(),
    }
}

fn close(actual: Vector3<f64>, expected: Vector3<f64>, tolerance: f64) {
    assert!(
        (actual - expected).norm() <= tolerance,
        "{actual:?} != {expected:?}"
    );
}

#[test]
fn inertia_products_negate_all_off_diagonals_preserve_diagonals_and_symmetry() {
    // Three pairs of 1 kg point masses, positions +/- (1,2,0),
    // +/- (1,0,-3), +/- (0,2,3) m about the CG at the origin.
    // Diagonals sum m*(y*y+z*z), etc.; products sum m*x*y, etc.
    let mass = MassProperties::from_products_of_inertia(
        6.0,
        Vector3::new(52.0, 40.0, 20.0),
        Vector3::new(4.0, -6.0, 12.0),
    )
    .unwrap();
    let expected = Matrix3::new(52.0, -4.0, 6.0, -4.0, 40.0, -12.0, 6.0, -12.0, 20.0);
    assert_eq!(mass.inertia_body_kg_m2(), expected);
    assert_eq!(
        mass.inertia_body_kg_m2(),
        mass.inertia_body_kg_m2().transpose()
    );
    assert_eq!(mass.mass_kg(), 6.0);
    assert_eq!(
        MassProperties::from_signed_tensor(6.0, expected).unwrap(),
        mass
    );
}

#[test]
fn invalid_mass_inertia_and_step_are_errors() {
    for mass in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(MassProperties::from_signed_tensor(mass, Matrix3::identity()).is_err());
    }
    for matrix in [
        Matrix3::zeros(),
        Matrix3::from_diagonal(&Vector3::new(-1.0, 1.0, 1.0)),
        Matrix3::new(1.0, 0.2, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
        Matrix3::repeat(f64::NAN),
    ] {
        assert!(MassProperties::from_signed_tensor(1.0, matrix).is_err());
    }
    for step in [0.0, -0.01, f64::NAN, f64::INFINITY] {
        assert!(FixedStepRk4::new(body(Vector3::repeat(2.0)), step).is_err());
    }
    assert_eq!(DEFAULT_STEP_S, 1.0 / 400.0);
    assert_eq!(
        FixedStepRk4::with_default_step(body(Vector3::repeat(2.0))).step_s(),
        DEFAULT_STEP_S
    );
}

#[test]
fn free_fall_matches_analytic_position_and_velocity_for_tilted_body() {
    let sim = FixedStepRk4::with_default_step(body(Vector3::repeat(2.0)));
    let mut state = initial_state();
    state.attitude_body_to_ned = UnitQuaternion::from_euler_angles(0.3, -0.4, 0.7);
    state.position_ned_m = Vector3::new(2.0, -3.0, -100.0);
    let initial_velocity_ned = Vector3::new(4.0, 5.0, -6.0);
    state.velocity_body_mps = state
        .attitude_body_to_ned
        .inverse_transform_vector(&initial_velocity_ned);
    let start = state;
    for _ in 0..800 {
        state = sim.step(&state, no_loads).unwrap();
    }
    let t = 2.0;
    close(
        state.position_ned_m,
        start.position_ned_m + initial_velocity_ned * t + Vector3::z() * (0.5 * G * t * t),
        1e-10,
    );
    close(
        state
            .attitude_body_to_ned
            .transform_vector(&state.velocity_body_mps),
        initial_velocity_ned + Vector3::z() * (G * t),
        1e-10,
    );
    assert!(
        state
            .attitude_body_to_ned
            .angle_to(&start.attitude_body_to_ned)
            < 1e-14
    );
}

#[test]
fn torque_free_symmetric_body_precesses_and_conserves_angular_momentum() {
    // Symmetric top I=(2,2,3): r=2, p=cos(t), q=sin(t) from Euler's equations.
    let mass = body(Vector3::new(2.0, 2.0, 3.0));
    let inertia = mass.inertia_body_kg_m2();
    let sim = FixedStepRk4::with_default_step(mass);
    let mut state = initial_state();
    state.angular_rates_body_radps = Vector3::new(1.0, 0.0, 2.0);
    let initial_h = inertia * state.angular_rates_body_radps;
    let mut max_relative_drift: f64 = 0.0;
    for _ in 0..4000 {
        state = sim.step(&state, no_loads).unwrap();
        let h_ned = state
            .attitude_body_to_ned
            .transform_vector(&(inertia * state.angular_rates_body_radps));
        max_relative_drift =
            max_relative_drift.max((h_ned.norm() - initial_h.norm()).abs() / initial_h.norm());
        close(h_ned, initial_h, 1e-8);
    }
    close(
        state.angular_rates_body_radps,
        Vector3::new(10.0_f64.cos(), 10.0_f64.sin(), 2.0),
        1e-10,
    );
    println!(
        "symmetric top: max relative angular-momentum magnitude drift={max_relative_drift:.9e}"
    );
    assert!(max_relative_drift < 1e-10);
}

#[test]
fn full_signed_tensor_and_each_moment_axis_drive_the_expected_acceleration() {
    let mass = MassProperties::from_products_of_inertia(
        6.0,
        Vector3::new(52.0, 40.0, 20.0),
        Vector3::new(4.0, -6.0, 12.0),
    )
    .unwrap();
    let inertia = mass.inertia_body_kg_m2();
    let sim = FixedStepRk4::with_default_step(mass);
    for expected_acceleration in [Vector3::x(), Vector3::y(), Vector3::z()] {
        // Cancel gyroscopic torque at every stage, leaving a known constant
        // angular acceleration. This exercises every signed tensor entry.
        let mut state = initial_state();
        for _ in 0..400 {
            state = sim
                .step(&state, |s| Loads {
                    force_body_n: Vector3::zeros(),
                    moment_body_nm: inertia * expected_acceleration
                        + s.angular_rates_body_radps
                            .cross(&(inertia * s.angular_rates_body_radps)),
                })
                .unwrap();
        }
        close(state.angular_rates_body_radps, expected_acceleration, 1e-12);
    }
}

#[test]
fn each_body_force_axis_uses_mass_and_body_to_ned_rotation() {
    let sim = FixedStepRk4::with_default_step(body(Vector3::repeat(2.0)));
    for force in [Vector3::x() * 2.0, Vector3::y() * 2.0, Vector3::z() * 2.0] {
        let mut state = initial_state();
        state.attitude_body_to_ned = UnitQuaternion::from_euler_angles(0.4, 0.2, -0.6);
        let acceleration =
            state.attitude_body_to_ned.transform_vector(&(force / 2.0)) + Vector3::z() * G;
        for _ in 0..400 {
            state = sim
                .step(&state, |_| Loads {
                    force_body_n: force,
                    ..no_loads(&state)
                })
                .unwrap();
        }
        close(
            state
                .attitude_body_to_ned
                .transform_vector(&state.velocity_body_mps),
            acceleration,
            1e-11,
        );
        close(state.position_ned_m, acceleration * 0.5, 1e-11);
    }
}

#[test]
fn quaternion_norm_drift_stays_below_5e_minus_15_for_100000_steps() {
    let sim = FixedStepRk4::with_default_step(body(Vector3::repeat(2.0)));
    let mut state = initial_state();
    state.angular_rates_body_radps = Vector3::new(0.7, -1.1, 0.9);
    let mut maximum: f64 = 0.0;
    for _ in 0..100_000 {
        state = sim.step(&state, no_loads).unwrap();
        maximum = maximum.max((state.attitude_body_to_ned.quaternion().norm() - 1.0).abs());
    }
    println!("100000 steps: maximum quaternion norm drift={maximum:.9e}");
    assert!(maximum <= 5e-15);
}

#[test]
fn step_halving_shows_fourth_order_convergence_in_coupled_translation_and_attitude() {
    // A spherical body spins at r=8 rad/s while moving north at 10 m/s.
    // No external force: its NED trajectory is analytic free fall, while its
    // BODY velocity changes as the body rotates. This exercises coupled RK4
    // stages, the -omega cross velocity term and quaternion multiplication.
    let mut position_errors = Vec::new();
    let mut attitude_errors = Vec::new();
    for count in [50, 100, 200, 400, 800] {
        let dt = 1.0 / f64::from(count);
        let sim = FixedStepRk4::new(body(Vector3::repeat(2.0)), dt).unwrap();
        let mut state = initial_state();
        state.velocity_body_mps = Vector3::new(10.0, 0.0, 0.0);
        state.angular_rates_body_radps = Vector3::new(0.0, 0.0, 8.0);
        for _ in 0..count {
            state = sim.step(&state, no_loads).unwrap();
        }
        let position_error = (state.position_ned_m - Vector3::new(10.0, 0.0, G / 2.0)).norm();
        let exact_attitude = UnitQuaternion::from_scaled_axis(Vector3::new(0.0, 0.0, 8.0));
        let attitude_error = state.attitude_body_to_ned.angle_to(&exact_attitude);
        println!(
            "dt={dt:.5}: position error={position_error:.12e} m, attitude error={attitude_error:.12e} rad"
        );
        position_errors.push(position_error);
        attitude_errors.push(attitude_error);
    }
    for errors in [&position_errors, &attitude_errors] {
        for pair in errors.windows(2) {
            let ratio = pair[0] / pair[1];
            println!("step-halving error ratio={ratio:.6}");
            assert!((14.0..18.0).contains(&ratio), "ratio={ratio}");
        }
    }
    // Numerical acceptance for THIS synthetic case, not a universal aircraft
    // accuracy claim: default dt=1/400 is the fourth entry.
    assert!(position_errors[3] < 1e-6);
    assert!(attitude_errors[3] < 1e-9);
}

#[test]
fn non_finite_states_loads_and_intermediate_overflow_are_explicit_errors() {
    let sim = FixedStepRk4::with_default_step(body(Vector3::repeat(2.0)));
    for field in 0..4 {
        let mut state = initial_state();
        match field {
            0 => state.position_ned_m.x = f64::NAN,
            1 => state.velocity_body_mps.y = f64::INFINITY,
            2 => state.angular_rates_body_radps.z = f64::NAN,
            _ => {
                state.attitude_body_to_ned =
                    UnitQuaternion::new_unchecked(Quaternion::new(0.0, 0.0, 0.0, 0.0))
            }
        }
        assert!(sim.step(&state, no_loads).is_err());
    }
    for field in 0..2 {
        let mut loads = no_loads(&initial_state());
        if field == 0 {
            loads.force_body_n.x = f64::NAN;
        } else {
            loads.moment_body_nm.y = f64::INFINITY;
        }
        assert_eq!(
            sim.step(&initial_state(), |_| loads),
            Err(DynamicsError::NonFinite)
        );
    }
    let mut state = initial_state();
    state.angular_rates_body_radps = Vector3::repeat(f64::MAX);
    assert!(sim.step(&state, no_loads).is_err());
    // Fn, rather than FnMut, keeps the callback interface state-based. A Cell
    // is used only by this test to inject an error after the first stage.
    let calls_at_stage = std::cell::Cell::new(0);
    assert!(
        sim.step(&initial_state(), |_| {
            let count = calls_at_stage.get() + 1;
            calls_at_stage.set(count);
            Loads {
                force_body_n: Vector3::repeat(if count == 2 { f64::NAN } else { 0.0 }),
                ..no_loads(&initial_state())
            }
        })
        .is_err()
    );
}
