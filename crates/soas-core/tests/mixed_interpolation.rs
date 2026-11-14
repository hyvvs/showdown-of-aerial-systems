//! Review Fix 2. Every changed value is a SYNTHETIC TEST VALUE, not aircraft
//! data. Expected off-centre values are hand calculations, not model output.
use serde_json::Value;
use soas_core::{aerodynamics::lookup_coefficients, aircraft::from_json};

fn check(field: &str, corners: [f64; 4], expected: f64) {
    let mut json: Value =
        serde_json::from_str(include_str!("fixtures/synthetic_aircraft.json")).unwrap();
    for (index, value) in corners.into_iter().enumerate() {
        json["aerodynamics"]["coefficients"][index / 2][index % 2][field] = Value::from(value);
    }
    // Ordering [C00,C01,C10,C11]: altitude row then Mach column.
    assert_ne!(
        corners[0] - corners[1] - corners[2] + corners[3],
        0.0,
        "mixed difference for {field} must be nonzero"
    );
    let a = from_json(&json.to_string()).unwrap();
    // Mach .2 on [0,.8] -> x=1/4; height 12000 on [0,20000] -> y=3/5.
    // Weights (1-x)(1-y), x(1-y), (1-x)y, xy = .30,.10,.45,.15.
    let c = lookup_coefficients(&a.aerodynamics, 0.2, 12000.0).unwrap();
    let actual = serde_json::to_value(c).unwrap()[field].as_f64().unwrap();
    assert!(
        (actual - expected).abs() <= 1e-14,
        "{field}: actual={actual:.15e}, expected={expected:.15e}"
    );
}
macro_rules! mixed_case {
    ($name:ident,$field:literal,$corners:expr,$expected:expr) => {
        #[test]
        fn $name() {
            check($field, $corners, $expected);
        }
    };
}

// .30*.1 + .10*.2 + .45*.3 + .15*.7 = .29; mixed difference = .3.
mixed_case!(mixed_cl0, "cl0", [0.1, 0.2, 0.3, 0.7], 0.29);
// .30*1 + .10*2 + .45*3 + .15*7 = 2.9; mixed difference = 3.
mixed_case!(
    mixed_cl_alpha,
    "cl_alpha_per_rad",
    [1.0, 2.0, 3.0, 7.0],
    2.9
);
// .30*1 + .10*2 + .45*3 + .15*7 = 2.9; mixed difference = 3.
mixed_case!(mixed_cl_max, "cl_max", [1.0, 2.0, 3.0, 7.0], 2.9);
// .30*.01 + .10*.02 + .45*.03 + .15*.07 = .029; mixed difference = .03.
mixed_case!(mixed_cd0, "cd0", [0.01, 0.02, 0.03, 0.07], 0.029);
// .30*.1 + .10*.2 + .45*.3 + .15*.7 = .29; mixed difference = .3.
mixed_case!(
    mixed_efficiency,
    "oswald_efficiency",
    [0.1, 0.2, 0.3, 0.7],
    0.29
);
// .30*.01 + .10*.02 + .45*.03 + .15*.07 = .029; mixed difference = .03.
mixed_case!(mixed_cm0, "cm0", [0.01, 0.02, 0.03, 0.07], 0.029);
// .30*(-.1) + .10*(-.2) + .45*(-.3) + .15*(-.7) = -.29; mixed = -.3.
mixed_case!(
    mixed_cm_alpha,
    "cm_alpha_per_rad",
    [-0.1, -0.2, -0.3, -0.7],
    -0.29
);
// .30*(-1) + .10*(-2) + .45*(-3) + .15*(-7) = -2.9; mixed = -3.
mixed_case!(
    mixed_elevator,
    "cm_elevator_per_rad",
    [-1.0, -2.0, -3.0, -7.0],
    -2.9
);
// .30*(-.01) + .10*(-.02) + .45*(-.03) + .15*(-.07) = -.029; mixed = -.03.
mixed_case!(
    mixed_side_beta,
    "cy_beta_per_rad",
    [-0.01, -0.02, -0.03, -0.07],
    -0.029
);
// .30*.1 + .10*.2 + .45*.3 + .15*.7 = .29; mixed difference = .3.
mixed_case!(
    mixed_aileron,
    "roll_aileron_per_rad",
    [0.1, 0.2, 0.3, 0.7],
    0.29
);
// .30*(-.1) + .10*(-.2) + .45*(-.3) + .15*(-.7) = -.29; mixed = -.3.
mixed_case!(
    mixed_rudder,
    "yaw_rudder_per_rad",
    [-0.1, -0.2, -0.3, -0.7],
    -0.29
);

#[test]
fn singleton_altitude_accepts_only_its_node_and_still_interpolates_mach() {
    let mut a = from_json(include_str!("fixtures/synthetic_aircraft.json")).unwrap();
    a.aerodynamics.geometric_altitude_nodes_m = Some(vec![1234.0]);
    a.aerodynamics.coefficients.truncate(1);
    let c = lookup_coefficients(&a.aerodynamics, 0.2, 1234.0).unwrap();
    // .75*.2 + .25*.4 = .25, while the single altitude is exactly 1234 m.
    assert!((c.cl0 - 0.25).abs() <= 1e-14);
    assert_eq!(
        lookup_coefficients(&a.aerodynamics, 0.8, 1234.0).unwrap(),
        a.aerodynamics.coefficients[0][1]
    );
    for height in [1234.0 - 1e-9, 1234.0 + 1e-9] {
        let message = lookup_coefficients(&a.aerodynamics, 0.2, height)
            .unwrap_err()
            .to_string();
        assert!(
            message.contains("geometric altitude") && message.contains("outside"),
            "{message}"
        );
    }
}
