//! Review Fix 3: all geometry, coefficients and control inputs are SYNTHETIC
//! TEST VALUES, not real aircraft data. The independent density is the T3
//! sea-level equation value from its sourced USSA constants (also used by T6).
use nalgebra::Vector3;
use soas_core::{
    aerodynamics::{Controls, evaluate},
    aircraft::{AeroCoefficients, Aircraft, from_json},
    attitude::Attitude,
    dynamics::State,
    units::Radians,
};

fn rad(x: f64) -> Radians {
    Radians::new(x).unwrap()
}
fn aircraft(change: impl FnOnce(&mut AeroCoefficients)) -> Aircraft {
    let mut a = from_json(include_str!("fixtures/synthetic_aircraft.json")).unwrap();
    // Deliberately different span b=6 m and chord c=.5 m; S=2 m^2.
    a.reference_span_m = 6.0;
    a.reference_chord_m = 0.5;
    a.reference_area_m2 = 2.0;
    a.cg_body_m = [0.0; 3];
    a.aerodynamic_reference_point_body_m = [0.0; 3];
    let mut c = a.aerodynamics.coefficients[0][0];
    c.cm0 = 0.0;
    c.cm_alpha_per_rad = 0.0;
    c.cm_elevator_per_rad = 0.0;
    c.roll_aileron_per_rad = 0.0;
    c.yaw_rudder_per_rad = 0.0;
    change(&mut c);
    for row in &mut a.aerodynamics.coefficients {
        row.fill(c);
    }
    a
}
fn moment(a: &Aircraft, alpha: f64, controls: Controls) -> Vector3<f64> {
    let state = State {
        position_ned_m: Vector3::zeros(),
        velocity_body_mps: Vector3::new(20.0 * alpha.cos(), 0.0, 20.0 * alpha.sin()),
        attitude_body_to_ned: Attitude::identity(),
        angular_rates_body_radps: Vector3::zeros(),
    };
    evaluate(a, &state, controls).unwrap().loads.moment_body_nm
}
fn close(label: &str, actual: Vector3<f64>, expected: Vector3<f64>) {
    println!("{label}: actual={actual:?} Nm, hand expected={expected:?} Nm");
    assert!(
        (actual - expected).norm() <= 1e-11,
        "{label}: {actual:?} != {expected:?}"
    );
}

// Independent hand scale: rho=1.224999155887712 kg/m^3, V=20 m/s.
// q = rho*400/2 = 244.9998311775424 Pa; q*S=489.9996623550848 N.
// q*S*b=2939.9979741305088 Nm; q*S*c=244.9998311775424 Nm.
#[test]
fn pitch_intercept_has_hand_calculated_q_s_chord_magnitude() {
    let a = aircraft(|c| c.cm0 = 0.03);
    // M = q*S*c*.03 = 7.349994935326272 Nm; L=N=0.
    close(
        "Cm0",
        moment(&a, 0.0, Controls::neutral()),
        Vector3::new(0.0, 7.349994935326272, 0.0),
    );
}
#[test]
fn pitch_alpha_derivative_has_hand_calculated_q_s_chord_magnitude() {
    let a = aircraft(|c| c.cm_alpha_per_rad = -0.6);
    for sign in [-1.0, 1.0] {
        // alpha=+/-.1 rad: Cm=-.6*(+/-.1)=-/+.06.
        // M=q*S*c*(-/+.06)=-/+14.699989870652544 Nm; L=N=0.
        close(
            "Cm_alpha",
            moment(&a, sign * 0.1, Controls::neutral()),
            Vector3::new(0.0, -sign * 14.699989870652544, 0.0),
        );
    }
}
#[test]
fn elevator_has_hand_calculated_q_s_chord_magnitude() {
    let a = aircraft(|c| c.cm_elevator_per_rad = -1.5);
    for sign in [-1.0, 1.0] {
        // elevator=+/-.2 rad: Cm=-1.5*(+/-.2)=-/+.3.
        // M=q*S*c*(-/+.3)=-/+73.49994935326272 Nm; L=N=0.
        close(
            "elevator",
            moment(
                &a,
                0.0,
                Controls {
                    elevator_rad: rad(sign * 0.2),
                    ..Controls::neutral()
                },
            ),
            Vector3::new(0.0, -sign * 73.49994935326272, 0.0),
        );
    }
}
#[test]
fn aileron_has_hand_calculated_q_s_span_magnitude() {
    let a = aircraft(|c| c.roll_aileron_per_rad = 0.25);
    for sign in [-1.0, 1.0] {
        // aileron=+/-.2 rad: Cl_roll=.25*(+/-.2)=+/-.05.
        // L=q*S*b*(+/-.05)=+/-146.99989870652544 Nm; M=N=0.
        close(
            "aileron",
            moment(
                &a,
                0.0,
                Controls {
                    aileron_rad: rad(sign * 0.2),
                    ..Controls::neutral()
                },
            ),
            Vector3::new(sign * 146.99989870652544, 0.0, 0.0),
        );
    }
}
#[test]
fn rudder_has_hand_calculated_q_s_span_magnitude() {
    let a = aircraft(|c| c.yaw_rudder_per_rad = -0.4);
    for sign in [-1.0, 1.0] {
        // rudder=+/-.25 rad: Cn=-.4*(+/-.25)=-/+.1.
        // N=q*S*b*(-/+.1)=-/+293.99979741305088 Nm; L=M=0.
        close(
            "rudder",
            moment(
                &a,
                0.0,
                Controls {
                    rudder_rad: rad(sign * 0.25),
                    ..Controls::neutral()
                },
            ),
            Vector3::new(0.0, 0.0, -sign * 293.999_797_413_050_9),
        );
    }
}
#[test]
fn simultaneous_moment_terms_have_hand_calculated_sum_and_separate_reference_lengths() {
    let a = aircraft(|c| {
        c.cm0 = 0.03;
        c.cm_alpha_per_rad = -0.6;
        c.cm_elevator_per_rad = -1.5;
        c.roll_aileron_per_rad = 0.25;
        c.yaw_rudder_per_rad = -0.4;
    });
    // alpha=.1, elevator=.2: Cm=.03-.6*.1-1.5*.2=-.33;
    // M=q*S*c*(-.33)=-80.849944288588992 Nm.
    // Cl_roll=.25*.2=.05 -> L=q*S*b*.05=146.99989870652544 Nm.
    // Cn=-.4*.25=-.1 -> N=q*S*b*(-.1)=-293.99979741305088 Nm.
    close(
        "combined",
        moment(
            &a,
            0.1,
            Controls {
                elevator_rad: rad(0.2),
                aileron_rad: rad(0.2),
                rudder_rad: rad(0.25),
            },
        ),
        Vector3::new(
            146.99989870652544,
            -80.84994428858899,
            -293.999_797_413_050_9,
        ),
    );
}
