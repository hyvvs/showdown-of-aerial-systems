//! All benchmark dimensions, angles, counts and tolerances are labeled synthetic
//! test values/settings in lifting_line_cases_values.md / lifting_line_test_values.md.
//! 2*pi is the CITED thin-airfoil theoretical input, not real-airfoil data.
use serde_json::Value;
use soas_core::units::Radians;
use soas_estimator::{
    lifting_line::{EllipticWing, StraightWing, solve},
    planform::{WingPlanform, WingStation, thin_airfoil_slope_per_rad},
};
use std::f64::consts::PI;
fn rad(x: f64) -> Radians {
    Radians::new(x).unwrap()
}
fn cases() -> Value {
    serde_json::from_str(include_str!("fixtures/lifting_line_cases.json")).unwrap()
}
fn elliptic(index: usize) -> EllipticWing {
    let v = cases();
    let c = &v["elliptic"][index];
    let b = c["span_m"].as_f64().unwrap();
    let ar = c["aspect_ratio"].as_f64().unwrap();
    // Analytic ellipse area = pi*b*c0/4 -> c0=4*b/(pi*AR).
    EllipticWing {
        span_m: b,
        root_chord_m: 4.0 * b / (PI * ar),
        twist_rad: rad(0.0),
        section_lift_curve_slope_per_rad: thin_airfoil_slope_per_rad(),
        zero_lift_angle_rad: rad(c["zero_lift_angle_rad"].as_f64().unwrap()),
        section_source: "Caughey eq 2.15, ideal thin-airfoil 2*pi/rad; synthetic benchmark".into(),
        data_note: "SYNTHETIC elliptic benchmark, not a real wing; see numeric ledger".into(),
    }
}
fn rectangle() -> WingPlanform {
    let v = cases();
    let c = &v["rectangular"];
    let span = c["span_m"].as_f64().unwrap();
    let chord = c["chord_m"].as_f64().unwrap();
    WingPlanform {
        data_note: "SYNTHETIC rectangular test wing; fixture ledger".into(),
        stations: [0.0, span / 2.0]
            .into_iter()
            .map(|y| WingStation {
                y_m: y,
                chord_m: chord,
                leading_edge_x_m: chord / 4.0,
                z_m: 0.0,
                twist_rad: rad(0.0),
                section_lift_curve_slope_per_rad: thin_airfoil_slope_per_rad(),
                zero_lift_angle_rad: rad(c["zero_lift_angle_rad"].as_f64().unwrap()),
                section_source: "Cited thin-airfoil 2*pi/rad, synthetic benchmark".into(),
            })
            .collect(),
    }
}
fn check_elliptic(
    index: usize,
    expected_slope: f64,
    expected_cl: f64,
    expected_cdi: f64,
    expected_induced: f64,
) {
    let wing = elliptic(index);
    let alpha = cases()["elliptic"][index]["alpha_rad"].as_f64().unwrap();
    for terms in [1, 4, 12, 24] {
        let out = solve(StraightWing::Elliptic(&wing), rad(alpha), terms).unwrap();
        assert!(
            (out.cl - expected_cl).abs() < 2e-12,
            "AR case {index}, N={terms}: {} != {expected_cl}",
            out.cl
        );
        assert!((out.cl / alpha - expected_slope).abs() < 2e-11);
        assert!((out.induced_drag_coefficient - expected_cdi).abs() < 2e-13);
        assert!((out.span_efficiency.unwrap() - 1.0).abs() < 2e-13);
        assert_eq!(out.fourier_terms.len(), terms);
        assert!(out.relative_collocation_residual < 1e-12);
        for (j, t) in out.fourier_terms.iter().enumerate() {
            assert_eq!(t.harmonic, 2 * j + 1);
            if j > 0 {
                assert!(t.coefficient.abs() < 2e-13);
            }
        }
        for s in &out.sections {
            assert!((s.section_cl - expected_cl).abs() < 2e-11);
            assert!((s.induced_angle_rad.value() - expected_induced).abs() < 2e-12);
            let shape = (1.0 - (2.0 * s.y_m / wing.span_m).powi(2)).sqrt();
            let expected_gamma_over_v = wing.root_chord_m * expected_cl * shape / 2.0;
            assert!((s.circulation_over_airspeed_m - expected_gamma_over_v).abs() < 2e-12);
            assert!(
                (s.lift_per_span_over_dynamic_pressure_m - 2.0 * expected_gamma_over_v).abs()
                    < 2e-12
            );
        }
    }
}
#[test]
fn elliptic_ar_four_matches_independent_hand_slope_lift_drag_and_loading() {
    // a0=2*pi, AR=4: a=2*pi/(1+1/2)=4*pi/3.
    // alpha=1/20 -> CL=pi/15; CDi=(pi/15)^2/(4*pi)=pi/900; induced=1/60.
    check_elliptic(0, 4.0 * PI / 3.0, PI / 15.0, PI / 900.0, 1.0 / 60.0);
}
#[test]
fn elliptic_ar_ten_matches_independent_hand_slope_lift_drag_and_loading() {
    // a0=2*pi, AR=10: a=2*pi/(1+1/5)=5*pi/3.
    // alpha=1/20 -> CL=pi/12; CDi=(pi/12)^2/(10*pi)=pi/1440; induced=1/120.
    check_elliptic(1, 5.0 * PI / 3.0, PI / 12.0, PI / 1440.0, 1.0 / 120.0);
}
#[test]
fn untwisted_wings_have_zero_loading_at_the_section_zero_lift_angle() {
    let mut ellipse = elliptic(0);
    ellipse.zero_lift_angle_rad = rad(-0.03);
    let rectangle = rectangle();
    for wing in [
        StraightWing::Elliptic(&ellipse),
        StraightWing::Stations(&rectangle),
    ] {
        let alpha = if matches!(wing, StraightWing::Elliptic(_)) {
            -0.03
        } else {
            -0.02
        };
        let out = solve(wing, rad(alpha), 12).unwrap();
        assert_eq!(out.cl, 0.0);
        assert_eq!(out.induced_drag_coefficient, 0.0);
        assert_eq!(out.span_efficiency, None); // 0/0 efficiency is undefined, not guessed 1.
        assert!(
            out.sections
                .iter()
                .all(|s| s.section_cl == 0.0 && s.circulation_over_airspeed_m == 0.0)
        );
    }
}
#[test]
fn positive_twist_and_zero_lift_angle_signs_have_analytic_effects() {
    let mut wing = elliptic(0);
    wing.twist_rad = rad(0.02);
    let twisted = solve(StraightWing::Elliptic(&wing), rad(0.0), 12).unwrap();
    // AR4 slope 4*pi/3 times .02 = 2*pi/75.
    assert!((twisted.cl - 2.0 * PI / 75.0).abs() < 2e-12);
    wing.twist_rad = rad(0.0);
    wing.zero_lift_angle_rad = rad(-0.03);
    let cambered = solve(StraightWing::Elliptic(&wing), rad(0.0), 12).unwrap();
    assert!((cambered.cl - PI / 25.0).abs() < 2e-12);
}
#[test]
fn rectangular_wing_converges_with_terms_and_efficiency_is_below_one() {
    let wing = rectangle();
    let alpha = rad(cases()["rectangular"]["alpha_rad"].as_f64().unwrap());
    let reference = solve(StraightWing::Stations(&wing), alpha, 128).unwrap();
    let mut previous_cl_error = f64::INFINITY;
    let mut previous_cd_error = f64::INFINITY;
    for n in [2, 4, 8, 16, 32, 64] {
        let out = solve(StraightWing::Stations(&wing), alpha, n).unwrap();
        let cl_error = (out.cl - reference.cl).abs();
        let cd_error = (out.induced_drag_coefficient - reference.induced_drag_coefficient).abs();
        println!(
            "rectangle N={n}: CL={:.12}, CDi={:.12}, e={:.12}, CL_error={cl_error:.12e}, CDi_error={cd_error:.12e}, error_ratios=({:.6},{:.6})",
            out.cl,
            out.induced_drag_coefficient,
            out.span_efficiency.unwrap(),
            previous_cl_error / cl_error,
            previous_cd_error / cd_error
        );
        assert!(cl_error < previous_cl_error && cd_error < previous_cd_error);
        assert!(out.span_efficiency.is_some_and(|e| e > 0.0 && e < 1.0));
        previous_cl_error = cl_error;
        previous_cd_error = cd_error;
        if n == 64 {
            assert!(cl_error / reference.cl.abs() < 2e-6);
            assert!(cd_error / reference.induced_drag_coefficient < 2e-6);
        }
    }
    println!(
        "rectangle N=128 reference: CL={:.12}, CDi={:.12}, e={:.12}",
        reference.cl,
        reference.induced_drag_coefficient,
        reference.span_efficiency.unwrap()
    );
}
#[test]
fn section_loading_is_mirrored_and_scaling_alpha_reverses_lift_not_drag() {
    let wing = rectangle();
    let plus = solve(StraightWing::Stations(&wing), rad(0.08), 16).unwrap();
    let minus = solve(StraightWing::Stations(&wing), rad(-0.12), 16).unwrap();
    // Relative incidences .08-(-.02)=+.1, -.12-(-.02)=-.1.
    assert!((plus.cl + minus.cl).abs() < 2e-12);
    assert!((plus.induced_drag_coefficient - minus.induced_drag_coefficient).abs() < 2e-13);
    for (left, right) in plus.sections.iter().zip(plus.sections.iter().rev()) {
        assert!((left.y_m + right.y_m).abs() < 1e-13);
        assert!((left.section_cl - right.section_cl).abs() < 2e-12);
    }
    assert!(plus.sections.windows(2).all(|s| s[0].y_m < s[1].y_m));
}
#[test]
fn assumptions_are_returned_and_physical_validity_is_unassessed() {
    let wing = elliptic(0);
    let out = solve(StraightWing::Elliptic(&wing), rad(0.8), 4).unwrap();
    assert_eq!(out.validity, "validity unassessed");
    for assumption in [
        "straight symmetric wing",
        "incompressible flow",
        "attached flow",
        "no sweep correction",
    ] {
        assert!(out.assumptions.contains(&assumption));
    }
}
#[test]
fn invalid_geometry_provenance_term_counts_and_nonstraight_wings_error() {
    let original = elliptic(0);
    for field in 0..3 {
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut wing = original.clone();
            match field {
                0 => wing.span_m = invalid,
                1 => wing.root_chord_m = invalid,
                _ => wing.section_lift_curve_slope_per_rad = invalid,
            }
            assert!(solve(StraightWing::Elliptic(&wing), rad(0.05), 4).is_err());
        }
    }
    for terms in [0, usize::MAX] {
        assert!(solve(StraightWing::Elliptic(&original), rad(0.05), terms).is_err());
    }
    let mut wing = original.clone();
    wing.section_source.clear();
    assert!(solve(StraightWing::Elliptic(&wing), rad(0.05), 4).is_err());
    let mut wing = original;
    wing.data_note.clear();
    assert!(solve(StraightWing::Elliptic(&wing), rad(0.05), 4).is_err());
    let mut swept = rectangle();
    swept.stations[1].leading_edge_x_m += 0.1;
    assert!(
        solve(StraightWing::Stations(&swept), rad(0.05), 4)
            .unwrap_err()
            .to_string()
            .contains("straight")
    );
    let mut bent = rectangle();
    bent.stations[1].z_m = 0.1;
    assert!(solve(StraightWing::Stations(&bent), rad(0.05), 4).is_err());
}
