//! Every configuration/expected/tolerance number is a labeled SYNTHETIC TEST
//! VALUE or mathematical result in fixtures/planform_test_values.md. No real wing.
mod support;
use soas_estimator::planform::thin_airfoil_slope_per_rad;
fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-12,
        "{actual:.15e} != {expected:.15e}"
    );
}

#[test]
fn rectangular_hand_geometry_has_distinct_span_chord_and_correct_mac_position() {
    let g = support::planform("rectangular").geometry().unwrap();
    // b=2*3.5=7; S=7*1.25=8.75; AR=49/8.75=5.6; MAC=1.25.
    close(g.span_m, 7.0);
    close(g.area_m2, 8.75);
    close(g.aspect_ratio, 5.6);
    close(g.taper_ratio, 1.0);
    close(g.mean_aerodynamic_chord_m, 1.25);
    for (actual, expected) in g
        .mac_leading_edge_right_body_m
        .into_iter()
        .zip([0.8, 1.75, -0.4])
    {
        close(actual, expected);
    }
    close(g.mac_quarter_chord_right_body_m[0], 0.4875);
    close(g.quarter_chord_sweep_rad.value(), 0.0);
}
#[test]
fn tapered_hand_geometry_matches_standard_trapezoid_mac_and_body_axis_sweep() {
    let g = support::planform("tapered").geometry().unwrap();
    // b=10, cr=3, ct=1 -> lambda=1/3, S=20, AR=5.
    // MAC=(2/3)*3*(1+1/3+1/9)/(1+1/3)=13/6.
    // y_MAC=(10/6)*(1+2/3)/(1+1/3)=25/12.
    // x_LE=.6-.4*y_MAC=-7/30; z=-.2-.2*y_MAC=-37/60.
    // x_quarter=-7/30 - (13/6)/4=-31/40.
    // Root/tip quarter-x = -.15,-1.65: tan(aft sweep)=1.5/5=.3.
    close(g.span_m, 10.0);
    close(g.area_m2, 20.0);
    close(g.aspect_ratio, 5.0);
    close(g.taper_ratio, 1.0 / 3.0);
    close(g.mean_aerodynamic_chord_m, 13.0 / 6.0);
    for (actual, expected) in
        g.mac_leading_edge_right_body_m
            .into_iter()
            .zip([-7.0 / 30.0, 25.0 / 12.0, -37.0 / 60.0])
    {
        close(actual, expected);
    }
    close(g.mac_quarter_chord_right_body_m[0], -31.0 / 40.0);
    close(g.quarter_chord_sweep_rad.value(), 0.2914567944778671);
}
#[test]
fn sampling_interpolates_every_station_input_and_mirrors_only_y() {
    let wing = support::planform("rectangular");
    for y in [1.4, -1.4] {
        let s = wing.sample(y).unwrap();
        close(s.y_m, y);
        close(s.chord_m, 1.25);
        close(s.leading_edge_x_m, 0.8);
        close(s.z_m, -0.4);
        // f=1.4/3.5=.4: twist=.03+.4*(-.05)=.01; a0=5.5+.4*.8=5.82.
        close(s.twist_rad.value(), 0.01);
        close(s.section_lift_curve_slope_per_rad, 5.82);
        close(s.zero_lift_angle_rad.value(), -0.028);
    }
    assert_eq!(wing.sample(0.0).unwrap(), wing.stations[0]);
    assert_eq!(wing.sample(3.5).unwrap(), wing.stations[1]);
    assert!(wing.sample(3.6).is_err());
    assert!(wing.sample(f64::NAN).is_err());
}
#[test]
fn exact_linear_panel_integration_is_unchanged_by_subdivision() {
    let mut wing = support::planform("tapered");
    let original = wing.geometry().unwrap();
    let middle = wing.sample(2.0).unwrap();
    wing.stations.insert(1, middle);
    let split = wing.geometry().unwrap();
    close(split.area_m2, original.area_m2);
    close(
        split.mean_aerodynamic_chord_m,
        original.mean_aerodynamic_chord_m,
    );
    for (a, b) in split
        .mac_leading_edge_right_body_m
        .into_iter()
        .zip(original.mac_leading_edge_right_body_m)
    {
        close(a, b);
    }
    close(
        split.quarter_chord_sweep_rad.value(),
        original.quarter_chord_sweep_rad.value(),
    );
}
#[test]
fn validation_rejects_nonfinite_nonpositive_unordered_missing_and_overflow_data() {
    let original = support::planform("tapered");
    for field in 0..5 {
        for invalid in [f64::NAN, f64::INFINITY] {
            let mut wing = original.clone();
            let s = &mut wing.stations[1];
            match field {
                0 => s.y_m = invalid,
                1 => s.chord_m = invalid,
                2 => s.leading_edge_x_m = invalid,
                3 => s.z_m = invalid,
                _ => s.section_lift_curve_slope_per_rad = invalid,
            }
            let e = wing.geometry().unwrap_err().to_string();
            assert!(e.contains("stations[1]"), "{e}");
        }
    }
    for value in [0.0, -1.0] {
        let mut wing = original.clone();
        wing.stations[1].chord_m = value;
        assert!(wing.geometry().is_err());
        let mut wing = original.clone();
        wing.stations[1].section_lift_curve_slope_per_rad = value;
        assert!(wing.geometry().is_err());
    }
    for value in [-1.0, 0.0] {
        let mut wing = original.clone();
        wing.stations[1].y_m = value;
        assert!(wing.geometry().is_err());
    }
    let mut wing = original.clone();
    wing.stations[0].y_m = 0.1;
    assert!(wing.geometry().is_err());
    let mut wing = original.clone();
    wing.stations.reverse();
    assert!(wing.geometry().is_err());
    let mut wing = original.clone();
    wing.stations.truncate(1);
    assert!(wing.geometry().is_err());
    let mut wing = original.clone();
    wing.stations[1].section_source.clear();
    assert!(wing.geometry().is_err());
    let mut wing = original.clone();
    wing.data_note.clear();
    assert!(wing.geometry().is_err());
    let mut wing = original;
    wing.stations[1].y_m = f64::MAX;
    assert!(wing.geometry().is_err());
}
#[test]
fn theoretical_section_slope_default_is_explicitly_two_pi_per_radian() {
    // Independent identity, not measured/synthetic real-airfoil data: Caughey eq 2.15.
    close(thin_airfoil_slope_per_rad(), 2.0 * std::f64::consts::PI);
}
