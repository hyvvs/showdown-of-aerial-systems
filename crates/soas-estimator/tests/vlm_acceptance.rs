//! All settings are SYNTHETIC and ledgered in fixtures/vlm_trend_values.md
//! and fixtures/vlm_acceptance_test_values.md. Formula/acceptance derivation:
//! data/reference/estimator/vlm_diagnosis.md; no fixed inter-method tolerance.
use serde_json::Value;
use soas_core::units::Radians;
use soas_estimator::{
    lifting_line::{self, StraightWing},
    planform::{WingPlanform, WingStation, thin_airfoil_slope_per_rad},
    vlm::{SpanwiseSpacing, VlmRequest, solve},
};
use std::f64::consts::PI;
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/vlm_trend.json")).unwrap()
}
fn number(v: &Value, key: &str) -> f64 {
    v[key].as_f64().unwrap()
}
fn count(v: &Value, key: &str) -> usize {
    v[key].as_u64().unwrap().try_into().unwrap()
}
fn wing(span: f64, chord: f64) -> WingPlanform {
    WingPlanform {
        data_note: "SYNTHETIC trend wing; no aircraft calibration".into(),
        stations: [0.0, span / 2.0]
            .into_iter()
            .map(|y| WingStation {
                y_m: y,
                chord_m: chord,
                leading_edge_x_m: 0.0,
                z_m: 0.0,
                twist_rad: Radians::new(0.0).unwrap(),
                section_lift_curve_slope_per_rad: thin_airfoil_slope_per_rad(),
                zero_lift_angle_rad: Radians::new(0.0).unwrap(),
                section_source: "Caughey ideal 2*pi/rad; synthetic geometry".into(),
            })
            .collect(),
    }
}
fn request(v: &Value, ny: usize, nx: usize) -> VlmRequest {
    VlmRequest {
        spanwise_panels: ny,
        chordwise_panels: nx,
        spanwise_spacing: SpanwiseSpacing::default(),
        alpha_rad: Radians::new(number(v, "alpha_rad")).unwrap(),
        reference_x_body_m: 0.0,
    }
}
// NIST least-squares normal equations, with the intercept expressed as weights:
// https://www.itl.nist.gov/div898/handbook/pmd/section4/pmd431.htm
// Weight propagation and leave-one-out sensitivity are our explicitly empirical
// acceptance construction, not a sourced physical/statistical confidence bound.
fn fit(x: &[f64], y: &[f64]) -> (f64, f64, Vec<f64>) {
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let den = x.iter().map(|a| (a - mx).powi(2)).sum::<f64>();
    let b = x
        .iter()
        .zip(y)
        .map(|(a, c)| (a - mx) * (c - my))
        .sum::<f64>()
        / den;
    let weights = x.iter().map(|a| 1.0 / n - mx * (a - mx) / den).collect();
    (my - b * mx, b, weights)
}
fn zero_limit_band(x: &[f64], gap: &[f64], uncertainty: &[f64]) -> (f64, f64) {
    let (intercept, _, weights) = fit(x, gap);
    let propagated = weights
        .iter()
        .zip(uncertainty)
        .map(|(w, u)| w.abs() * u)
        .sum::<f64>();
    let curvature = (0..x.len())
        .map(|omit| {
            let keep_x: Vec<_> = x
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != omit)
                .map(|(_, a)| *a)
                .collect();
            let keep_y: Vec<_> = gap
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != omit)
                .map(|(_, a)| *a)
                .collect();
            (fit(&keep_x, &keep_y).0 - intercept).abs()
        })
        .fold(0.0, f64::max);
    println!(
        "free intercept={intercept:.12e}, propagated mesh={propagated:.12e}, leave-one-out={curvature:.12e}"
    );
    (intercept, propagated + curvature)
}
#[test]
fn rectangular_method_gaps_decrease_and_empirical_infinite_ar_limit_is_zero_compatible() {
    let v = fixture();
    let ars: Vec<_> = v["aspect_ratios"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a.as_f64().unwrap())
        .collect();
    let mut gaps = [Vec::new(), Vec::new()];
    let mut uncertainties = [Vec::new(), Vec::new()];
    for ar in &ars {
        let c = number(&v, "chord_m");
        let w = wing(ar * c, c);
        let r = request(
            &v,
            count(&v, "fine_span_panels"),
            count(&v, "fine_chord_panels"),
        );
        let fine = solve(&w, r).unwrap();
        let span = solve(
            &w,
            VlmRequest {
                spanwise_panels: count(&v, "coarse_span_panels"),
                ..r
            },
        )
        .unwrap();
        let chord = solve(
            &w,
            VlmRequest {
                chordwise_panels: count(&v, "coarse_chord_panels"),
                ..r
            },
        )
        .unwrap();
        let ll = lifting_line::solve(
            StraightWing::Stations(&w),
            r.alpha_rad,
            count(&v, "fine_fourier_terms"),
        )
        .unwrap();
        let ll_coarse = lifting_line::solve(
            StraightWing::Stations(&w),
            r.alpha_rad,
            count(&v, "coarse_fourier_terms"),
        )
        .unwrap();
        let e = fine.cl.powi(2) / (PI * ar * fine.induced_drag_coefficient);
        println!(
            "AR={ar}: VLM slope={}, CDi={}, e={e}; LLT slope={}, CDi={}, e={}",
            fine.d_cl_d_alpha_per_rad,
            fine.induced_drag_coefficient,
            ll.cl / r.alpha_rad.value(),
            ll.induced_drag_coefficient,
            ll.span_efficiency.unwrap()
        );
        for (i, (a, s, c, b, old)) in [
            (fine.cl, span.cl, chord.cl, ll.cl, ll_coarse.cl),
            (
                fine.induced_drag_coefficient,
                span.induced_drag_coefficient,
                chord.induced_drag_coefficient,
                ll.induced_drag_coefficient,
                ll_coarse.induced_drag_coefficient,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            gaps[i].push((a - b).abs() / b.abs());
            uncertainties[i].push(((a - s).abs() + (a - c).abs() + (b - old).abs()) / b.abs());
        }
    }
    for quantity in 0..gaps.len() {
        let delta = &gaps[quantity];
        assert!(delta.iter().all(|d| d.is_finite() && *d > 0.0));
        for (a, d) in ars.windows(2).zip(delta.windows(2)) {
            assert!(d[1] < d[0], "nondecreasing relative gap: {delta:?}");
            let p = (d[0] / d[1]).ln() / (a[1] / a[0]).ln();
            assert!(p > 0.0, "nondecaying local power law");
            println!("quantity={quantity}, AR={a:?}, gap={d:?}, local p={p}");
        }
        let log_ar: Vec<_> = ars.iter().map(|a| a.ln()).collect();
        let log_delta: Vec<_> = delta.iter().map(|d| d.ln()).collect();
        let fitted_p = -fit(&log_ar, &log_delta).1;
        assert!(
            fitted_p > 0.0,
            "empirical continuation does not tend to zero"
        );
        // Lift: Tuck constant-term difference / AR. Relative drag: log-based
        // tip hypothesis, explicitly unverified beyond these four measurements.
        let x: Vec<_> = ars
            .iter()
            .map(|a| if quantity == 0 { 1.0 / a } else { 1.0 / a.ln() })
            .collect();
        let (intercept, bound) = zero_limit_band(&x, delta, &uncertainties[quantity]);
        assert!(
            intercept.abs() <= bound,
            "nonzero-limit evidence: intercept {intercept}, empirical mesh/range band {bound}"
        );
    }
}
#[test]
fn extrapolation_rejects_an_independently_known_nonzero_floor() {
    let v = fixture();
    let x: Vec<_> = v["aspect_ratios"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| 1.0 / a.as_f64().unwrap())
        .collect();
    // Hand law delta=.02+.6/AR has strictly decreasing gaps but an exact .02
    // infinite-AR floor. This must fail a zero-limit check despite its trend.
    let delta: Vec<_> = x
        .iter()
        .map(|x| number(&v, "nonzero_floor_fault") + number(&v, "inverse_ar_fault_scale") * x)
        .collect();
    let uncertainty = vec![0.0; x.len()];
    assert!(delta.windows(2).all(|d| d[1] < d[0]));
    let (intercept, band) = zero_limit_band(&x, &delta, &uncertainty);
    assert!(
        intercept.abs() > band,
        "constant-floor fault was not rejected"
    );
}
#[test]
fn spacing_parameter_has_hand_calculated_edges_and_cosine_default() {
    let v = fixture();
    let w = wing(number(&v, "spacing_case_span_m"), number(&v, "chord_m"));
    let r = request(
        &v,
        count(&v, "spacing_case_span_panels"),
        count(&v, "spacing_case_chord_panels"),
    );
    assert_eq!(SpanwiseSpacing::default(), SpanwiseSpacing::Cosine);
    let cosine = solve(&w, r).unwrap();
    let uniform = solve(
        &w,
        VlmRequest {
            spanwise_spacing: SpanwiseSpacing::Uniform,
            ..r
        },
    )
    .unwrap();
    // b=8, Ny=4: uniform [-4,-2,0,2,4], cosine [-4,-2sqrt2,0,2sqrt2,4].
    let expected_uniform = [-4.0, -2.0, 0.0, 2.0, 4.0];
    let expected_cosine = [-4.0, -2.0 * 2.0_f64.sqrt(), 0.0, 2.0 * 2.0_f64.sqrt(), 4.0];
    for (out, edges) in [(&uniform, expected_uniform), (&cosine, expected_cosine)] {
        for (i, panel) in out.panels.iter().enumerate() {
            assert!(
                (panel.bound_start_m.y - edges[i]).abs() < number(&v, "coordinate_tolerance_m")
            );
            assert!(
                (panel.bound_end_m.y - edges[i + 1]).abs() < number(&v, "coordinate_tolerance_m")
            );
        }
    }
    assert_ne!(cosine.cl, uniform.cl, "spacing parameter was ignored");
}
