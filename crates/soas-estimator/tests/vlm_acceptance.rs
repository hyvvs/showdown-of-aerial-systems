//! All numerical settings are SYNTHETIC TEST VALUES, ledgered in fixtures/
//! vlm_trend_values.md and vlm_acceptance_test_values.md. No zero-limit verdict.
//! Sources/derivation: data/reference/estimator/vlm_acceptance_review.md.
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
        data_note: "SYNTHETIC acceptance wing; no aircraft calibration".into(),
        stations: [0.0, span / 2.0]
            .into_iter()
            .map(|y| WingStation {
                y_m: y,
                chord_m: chord,
                leading_edge_x_m: 0.0,
                z_m: 0.0,
                twist_rad: Radians::new(0.0).unwrap(),
                zero_lift_angle_rad: Radians::new(0.0).unwrap(),
                section_lift_curve_slope_per_rad: thin_airfoil_slope_per_rad(),
                section_source: "Cited ideal thin-airfoil 2*pi; synthetic geometry".into(),
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
#[test]
fn rectangular_lift_and_drag_gaps_strictly_decrease_on_both_meshes() {
    let v = fixture();
    for ny in [
        count(&v, "coarse_span_panels"),
        count(&v, "fine_span_panels"),
    ] {
        let mut gaps = [Vec::new(), Vec::new()];
        for ar in v["aspect_ratios"].as_array().unwrap() {
            let ar = ar.as_f64().unwrap();
            let c = number(&v, "chord_m");
            let w = wing(ar * c, c);
            let r = request(&v, ny, count(&v, "fine_chord_panels"));
            let out = solve(&w, r).unwrap();
            let ll = lifting_line::solve(
                StraightWing::Stations(&w),
                r.alpha_rad,
                count(&v, "fine_fourier_terms"),
            )
            .unwrap();
            let cl_gap = (out.cl - ll.cl).abs() / ll.cl.abs();
            let cd_gap = (out.induced_drag_coefficient - ll.induced_drag_coefficient).abs()
                / ll.induced_drag_coefficient.abs();
            println!("Ny={ny} AR={ar} relative gaps CL={cl_gap:.12e}, CDi={cd_gap:.12e}");
            gaps[0].push(cl_gap);
            gaps[1].push(cd_gap);
        }
        for g in gaps {
            assert!(g.iter().all(|a| a.is_finite()));
            assert!(
                g.windows(2).all(|a| a[1] < a[0]),
                "nondecreasing relative gaps at Ny={ny}: {g:?}"
            );
        }
    }
}
// MIT ellipse chord and T12 area integral; normalize polygon area, not forces.
// Same span/area/AR at every mesh; positive machine-epsilon endpoint is a test
// geometry regularization required by T12, not a physical tip parameter.
fn elliptic_polygon(ar: f64, root: f64, ny: usize) -> WingPlanform {
    let b = ar * PI * root / 4.0;
    let mut w = wing(b, root);
    let template = w.stations[0].clone();
    w.stations = (ny / 2..=ny)
        .map(|j| {
            let y = if j == ny / 2 {
                0.0
            } else if j == ny {
                b / 2.0
            } else {
                -b / 2.0 * (PI * j as f64 / ny as f64).cos()
            };
            let c = (root * (1.0 - (2.0 * y / b).powi(2)).max(0.0).sqrt()).max(root * f64::EPSILON);
            WingStation {
                y_m: y,
                chord_m: c,
                leading_edge_x_m: c / 4.0,
                ..template.clone()
            }
        })
        .collect();
    let scale = (b * b / ar) / w.geometry().unwrap().area_m2;
    for s in &mut w.stations {
        s.chord_m *= scale;
        s.leading_edge_x_m = s.chord_m / 4.0;
    }
    w
}
// NASA grid-convergence tutorial: f(h)=f0+C*h^p, ratio=2.
// No fixed observed order or inflated tolerance is selected.
fn richardson(f: &[f64]) -> (f64, f64) {
    let d0 = f[1] - f[0];
    let d1 = f[2] - f[1];
    let ratio = d0 / d1;
    assert!(
        ratio.is_finite() && ratio > 1.0,
        "insufficient contracting, same-sign changes: {f:?}"
    );
    let p = ratio.log2();
    let limit = f[2] + d1 / (2.0_f64.powf(p) - 1.0);
    assert!(limit.is_finite());
    (limit, p)
}
fn check_ellipse(index: usize) {
    let v = fixture();
    let b = &v["ellipse_benchmarks"];
    let ar = b["aspect_ratios"][index].as_f64().unwrap();
    let mut efficiencies = Vec::new();
    let mut slopes = Vec::new();
    let target = 2.0 * PI / (1.0 + 2.0 / ar); // MIT/Caughey elliptic LLT, information ONLY.
    for ny in b["full_span_panels"].as_array().unwrap() {
        let ny = ny.as_u64().unwrap().try_into().unwrap();
        let w = elliptic_polygon(ar, number(b, "root_chord_m"), ny);
        let out = solve(&w, request(&v, ny, count(b, "chordwise_panels"))).unwrap();
        let e = out.cl.powi(2)
            / (PI * w.geometry().unwrap().aspect_ratio * out.induced_drag_coefficient);
        println!(
            "ellipse AR={ar} Ny={ny} e={e:.15} slope={:.15} LLT slope error={:.15} (information only)",
            out.d_cl_d_alpha_per_rad,
            (out.d_cl_d_alpha_per_rad - target).abs()
        );
        efficiencies.push(e);
        slopes.push(out.d_cl_d_alpha_per_rad);
    }
    let excess: Vec<_> = efficiencies.iter().map(|e| (e - 1.0).max(0.0)).collect();
    assert!(
        excess.windows(2).all(|p| p[1] <= p[0]),
        "efficiency excess grows: {excess:?}"
    );
    for series in [&efficiencies, &slopes] {
        let changes: Vec<_> = series.windows(2).map(|p| (p[1] - p[0]).abs()).collect();
        assert!(
            changes.windows(2).all(|d| d[1] < d[0]),
            "mesh changes fail to decrease: {changes:?}"
        );
    }
    let extrapolated: Vec<_> = efficiencies.windows(3).map(richardson).collect();
    // A prediction band from the first FOUR meshes, before the finest mesh:
    // center=R(N1,N2,N3); width=abs(center-R(N0,N1,N2)). Test the held-out
    // finest-three result R(N2,N3,N4) against continued estimate contraction.
    // Empirical self-consistency band, NOT a bound around physical e=1.
    let center = extrapolated[1].0;
    let width = (center - extrapolated[0].0).abs();
    let finest = extrapolated[2].0;
    println!(
        "ellipse AR={ar}: Richardson={extrapolated:?}; pre-finest prediction band=[{},{}], finest estimate={finest}",
        center - width,
        center + width
    );
    assert!(
        (finest - center).abs() <= width,
        "finest Richardson estimate outside pre-finest empirical band"
    );
}
#[test]
fn elliptic_ar6_has_decreasing_mesh_changes_and_richardson_prediction() {
    check_ellipse(0);
}
#[test]
fn elliptic_ar40_has_decreasing_mesh_changes_and_richardson_prediction() {
    check_ellipse(1);
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
