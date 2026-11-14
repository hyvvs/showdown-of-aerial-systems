//! All fixture values and every numeric test setting are synthetic/derived,
//! individually ledgered in vlm_cases_values.md and vlm_test_values.md.
use serde_json::Value;
use soas_core::units::Radians;
use soas_estimator::{
    lifting_line::{self, StraightWing},
    planform::{WingPlanform, WingStation, thin_airfoil_slope_per_rad},
    vlm::{SpanwiseSpacing, VlmRequest, solve},
};
use std::f64::consts::PI;
fn rad(x: f64) -> Radians {
    Radians::new(x).unwrap()
}
fn wing(name: &str) -> WingPlanform {
    let cases: Value = serde_json::from_str(include_str!("fixtures/vlm_cases.json")).unwrap();
    let v = &cases[name];
    let b = v["span_m"].as_f64().unwrap();
    let c = v["chord_m"].as_f64().unwrap();
    WingPlanform {
        data_note: "SYNTHETIC VLM TEST WING - see fixture ledger; not a real aircraft".into(),
        stations: [0.0, b / 2.0]
            .into_iter()
            .map(|y| WingStation {
                y_m: y,
                chord_m: c,
                leading_edge_x_m: 0.0,
                z_m: 0.0,
                twist_rad: rad(0.0),
                section_lift_curve_slope_per_rad: thin_airfoil_slope_per_rad(),
                zero_lift_angle_rad: rad(0.0),
                section_source: "Cited ideal thin-airfoil 2*pi/rad; synthetic case".into(),
            })
            .collect(),
    }
}
fn request(ny: usize, nx: usize, alpha: f64) -> VlmRequest {
    VlmRequest {
        spanwise_panels: ny,
        spanwise_spacing: SpanwiseSpacing::default(),
        chordwise_panels: nx,
        alpha_rad: rad(alpha),
        reference_x_body_m: 0.0,
    }
}
#[test]
fn one_panel_has_independent_hand_circulation_lift_drag_moment_and_neutral_point() {
    let w = wing("one_panel");
    let out = solve(&w, request(1, 1, 0.1)).unwrap();
    // b=4,c=2,S=8; bound x=-.5,y=[-2,2], control=(-1.5,0,0).
    // At control, T14 scalar integral W=(1+sqrt(5))/(4*pi), Gamma/V=.1/W.
    let gamma = 0.4 * PI / (1.0 + 5.0_f64.sqrt());
    assert!((out.panels[0].circulation_over_airspeed_m - gamma).abs() < 1e-12);
    // CL=2*gamma*b/S=gamma. Trefftz midpoint: w_far/V=gamma/(2*pi).
    // CDi=gamma*w_far*b/S=gamma^2/(4*pi); Cm_LE=-CL/4.
    assert!((out.cl - gamma).abs() < 1e-12);
    assert!((out.induced_drag_coefficient - gamma * gamma / (4.0 * PI)).abs() < 1e-13);
    assert!((out.pitching_moment_coefficient + gamma / 4.0).abs() < 1e-12);
    assert!((out.neutral_point_x_body_m + 0.5).abs() < 1e-12);
    assert!(out.rolling_moment_coefficient.abs() < 1e-13);
    assert_eq!(out.panels[0].bound_start_m.as_slice(), &[-0.5, -2.0, 0.0]);
    assert_eq!(out.panels[0].bound_end_m.as_slice(), &[-0.5, 2.0, 0.0]);
    assert_eq!(out.panels[0].control_point_m.as_slice(), &[-1.5, 0.0, 0.0]);
    assert_eq!(out.reference_point_body_m, [0.0, 0.0, 0.0]);
}
#[test]
fn mirrored_panels_have_equal_circulation_and_zero_rolling_moment() {
    let w = wing("rectangular");
    let out = solve(&w, request(12, 3, 0.05)).unwrap();
    assert!(out.rolling_moment_coefficient.abs() < 2e-12);
    for p in &out.panels {
        let mirror = out
            .panels
            .iter()
            .find(|q| {
                q.spanwise_index == 11 - p.spanwise_index && q.chordwise_index == p.chordwise_index
            })
            .unwrap();
        assert!((p.circulation_over_airspeed_m - mirror.circulation_over_airspeed_m).abs() < 2e-12);
    }
}
#[test]
fn rectangular_spanwise_refinement_is_measured_at_fixed_chordwise_count() {
    let w = wing("rectangular");
    let reference = solve(&w, request(128, 4, 0.05)).unwrap();
    let mut errors = Vec::new();
    for ny in [8, 16, 32, 64] {
        let out = solve(&w, request(ny, 4, 0.05)).unwrap();
        let e = [
            (out.cl - reference.cl).abs(),
            (out.induced_drag_coefficient - reference.induced_drag_coefficient).abs(),
        ];
        println!(
            "VLM span ny={ny}, nx=4: CL={:.12}, CDi={:.12}, Cm={:.12}, NP={:.12}, errors={e:?}",
            out.cl,
            out.induced_drag_coefficient,
            out.pitching_moment_coefficient,
            out.neutral_point_x_body_m
        );
        errors.push(e);
    }
    for component in 0..2 {
        assert!(
            errors[3][component] < errors[0][component] / 4.0,
            "span refinement: {errors:?}"
        );
    }
    println!(
        "VLM span reference 128x4: CL={:.12}, CDi={:.12}, NP={:.12}",
        reference.cl, reference.induced_drag_coefficient, reference.neutral_point_x_body_m
    );
}
#[test]
fn rectangular_chordwise_refinement_is_measured_at_fixed_spanwise_count() {
    let w = wing("rectangular");
    let reference = solve(&w, request(32, 16, 0.05)).unwrap();
    let mut errors = Vec::new();
    for nx in [1, 2, 4, 8] {
        let out = solve(&w, request(32, nx, 0.05)).unwrap();
        let e = [
            (out.cl - reference.cl).abs(),
            (out.induced_drag_coefficient - reference.induced_drag_coefficient).abs(),
        ];
        println!(
            "VLM chord ny=32, nx={nx}: CL={:.12}, CDi={:.12}, Cm={:.12}, NP={:.12}, errors={e:?}",
            out.cl,
            out.induced_drag_coefficient,
            out.pitching_moment_coefficient,
            out.neutral_point_x_body_m
        );
        errors.push(e);
    }
    for component in 0..2 {
        assert!(
            errors[3][component] < errors[0][component] / 4.0,
            "chord refinement: {errors:?}"
        );
    }
    println!(
        "VLM chord reference 32x16: CL={:.12}, CDi={:.12}, NP={:.12}",
        reference.cl, reference.induced_drag_coefficient, reference.neutral_point_x_body_m
    );
}
#[test]
fn high_ar_rectangular_neutral_point_is_near_quarter_mac_and_derivative_is_correct() {
    let w = wing("high_ar");
    let r = request(32, 4, 0.05);
    let out = solve(&w, r).unwrap();
    // Thin-airfoil quarter chord: NASA Glenn/Caughey; 1% chord is a prewritten
    // synthetic numerical comparison bound, not a real-wing validity threshold.
    assert!((out.neutral_point_x_body_m + 0.25).abs() < 0.01);
    let plus = solve(
        &w,
        VlmRequest {
            alpha_rad: rad(0.0501),
            ..r
        },
    )
    .unwrap();
    let minus = solve(
        &w,
        VlmRequest {
            alpha_rad: rad(0.0499),
            ..r
        },
    )
    .unwrap();
    assert!(
        ((plus.pitching_moment_coefficient - minus.pitching_moment_coefficient) / 0.0002
            - out.d_cm_d_alpha_per_rad)
            .abs()
            < 1e-9
    );
    let shifted = solve(
        &w,
        VlmRequest {
            reference_x_body_m: 1.5,
            ..r
        },
    )
    .unwrap();
    assert!((shifted.neutral_point_x_body_m - out.neutral_point_x_body_m).abs() < 1e-12);
    assert!(
        (shifted.pitching_moment_coefficient - (out.pitching_moment_coefficient - 1.5 * out.cl))
            .abs()
            < 1e-12
    );
}
#[test]
fn twist_zero_lift_offset_and_planar_sweep_keep_explicit_assumptions() {
    let mut w = wing("rectangular");
    for s in &mut w.stations {
        s.twist_rad = rad(0.02);
        s.zero_lift_angle_rad = rad(-0.03);
    }
    let out = solve(&w, request(8, 2, -0.05)).unwrap();
    assert!(out.cl.abs() < 1e-13);
    assert!(out.induced_drag_coefficient.abs() < 1e-13);
    let mut swept = wing("one_panel");
    swept.stations[1].leading_edge_x_m = -1.0;
    let out = solve(&swept, request(8, 2, 0.05)).unwrap();
    assert_eq!(out.validity, "validity unassessed");
    for a in [
        "planar wing",
        "incompressible flow",
        "attached flow",
        "small angles",
    ] {
        assert!(out.assumptions.contains(&a));
    }
    assert!(out.drag_method.contains("Trefftz"));
}
#[test]
fn invalid_counts_reference_nonplanarity_and_unsupported_section_slope_error() {
    let w = wing("rectangular");
    for r in [
        request(0, 1, 0.05),
        request(1, 0, 0.05),
        request(usize::MAX, 2, 0.05),
        VlmRequest {
            reference_x_body_m: f64::NAN,
            ..request(4, 2, 0.05)
        },
    ] {
        assert!(solve(&w, r).is_err());
    }
    let mut bent = w.clone();
    bent.stations[1].z_m = 0.1;
    assert!(solve(&bent, request(4, 2, 0.05)).is_err());
    let mut other = w;
    other.stations[1].section_lift_curve_slope_per_rad = 5.0;
    assert!(
        solve(&other, request(4, 2, 0.05))
            .unwrap_err()
            .to_string()
            .contains("2*pi")
    );
}
