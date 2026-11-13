//! JSON contract tests. ALL fixture numbers are synthetic test values; the
//! fixture's warning and per-value ledger identify them individually.
use soas_core::aircraft::{from_json, to_json};

const FIXTURE: &str = include_str!("fixtures/synthetic_aircraft.json");

fn invalid(text: &str, expected: &str) {
    let error = from_json(text).unwrap_err().to_string();
    assert!(
        error.contains(expected),
        "expected {expected:?} in {error:?}"
    );
}

#[test]
fn synthetic_json_round_trips_without_losing_values_or_units() {
    let aircraft = from_json(FIXTURE).unwrap();
    let saved = to_json(&aircraft).unwrap();
    assert_eq!(from_json(&saved).unwrap(), aircraft);
    for key in [
        "format_version",
        "mass_kg",
        "cg_body_m",
        "inertia_body_kg_m2",
        "reference_area_m2",
        "reference_span_m",
        "reference_chord_m",
        "geometric_altitude_nodes_m",
        "cl_alpha_per_rad",
    ] {
        assert!(saved.contains(&format!("\"{key}\"")));
    }
    assert!(saved.contains("SYNTHETIC"));
    assert_eq!(aircraft.mass_properties().unwrap().mass_kg(), 10.0);
}

#[test]
fn malformed_json_missing_fields_and_unknown_fields_are_errors() {
    invalid("{", "JSON");
    invalid(&FIXTURE.replace("\"mass_kg\": 10.0,", ""), "mass_kg");
    invalid(&FIXTURE.replace("\"mass_kg\"", "\"mass_lb\""), "mass_lb");
    invalid(
        &FIXTURE.replace("\"cl0\": 0.2", "\"invented_coefficient\": 0.2"),
        "invented_coefficient",
    );
}

#[test]
fn format_version_is_required_and_unsupported_versions_are_rejected() {
    invalid(
        &FIXTURE.replace("\"format_version\": 0,", ""),
        "format_version",
    );
    invalid(
        &FIXTURE.replace("\"format_version\": 0", "\"format_version\": 1"),
        "format_version",
    );
}

#[test]
fn mass_geometry_and_coordinate_validation_identifies_the_field() {
    for (before, after, field) in [
        ("\"mass_kg\": 10.0", "\"mass_kg\": 0.0", "mass_kg"),
        (
            "\"reference_area_m2\": 2.0",
            "\"reference_area_m2\": -2.0",
            "reference_area_m2",
        ),
        (
            "\"reference_span_m\": 4.0",
            "\"reference_span_m\": 0.0",
            "reference_span_m",
        ),
        (
            "\"reference_chord_m\": 0.5",
            "\"reference_chord_m\": 0.0",
            "reference_chord_m",
        ),
    ] {
        invalid(&FIXTURE.replace(before, after), field);
    }
    invalid(&FIXTURE.replace("[0.0, 0.0, 0.0]", "[0.0, 0.0]"), "JSON");
}

#[test]
fn inertia_must_be_symmetric_positive_definite_and_about_the_cg() {
    invalid(
        &FIXTURE.replace("[[8.0, 0.0, 0.0]", "[[8.0, 1.0, 0.0]"),
        "inertia_body_kg_m2",
    );
    invalid(
        &FIXTURE.replace("[[8.0, 0.0, 0.0]", "[[-8.0, 0.0, 0.0]"),
        "inertia_body_kg_m2",
    );
    let mass = from_json(FIXTURE).unwrap().mass_properties().unwrap();
    assert_eq!(
        mass.inertia_body_kg_m2().diagonal().as_slice(),
        &[8.0, 10.0, 12.0]
    );
}

#[test]
fn axes_and_moment_reference_are_explicit_and_unsupported_ones_fail() {
    invalid(
        &FIXTURE.replace("wind_forces_body_moments", "source_axes_unknown"),
        "coefficient_axes",
    );
    invalid(
        &FIXTURE.replace(
            "\"moments_about\": \"aerodynamic_reference_point\"",
            "\"moments_about\": \"datum\"",
        ),
        "moments_about",
    );
}

#[test]
fn coefficient_table_nodes_must_be_finite_nonnegative_and_strictly_increasing() {
    for nodes in ["[]", "[0.8, 0.0]", "[0.0, 0.0]", "[-0.1, 0.8]"] {
        invalid(&FIXTURE.replace("[0.0, 0.8]", nodes), "mach_nodes");
    }
    for nodes in ["[]", "[20000.0, 0.0]", "[0.0, 0.0]", "[-1.0, 20000.0]"] {
        invalid(
            &FIXTURE.replace("[0.0, 20000.0]", nodes),
            "geometric_altitude_nodes_m",
        );
    }
}

#[test]
fn table_dimensions_must_match_both_axes_and_mach_only_tables_are_supported() {
    invalid(
        &FIXTURE.replace("[0.0, 0.8]", "[0.0, 0.4, 0.8]"),
        "coefficients",
    );
    invalid(
        &FIXTURE.replace("[0.0, 20000.0]", "[0.0, 10000.0, 20000.0]"),
        "coefficients",
    );
    let mut aircraft = from_json(FIXTURE).unwrap();
    aircraft.aerodynamics.geometric_altitude_nodes_m = None;
    aircraft.aerodynamics.coefficients.truncate(1);
    let saved = to_json(&aircraft).unwrap();
    assert_eq!(from_json(&saved).unwrap(), aircraft);
    aircraft.aerodynamics.coefficients[0].pop();
    assert!(
        to_json(&aircraft)
            .unwrap_err()
            .to_string()
            .contains("coefficients")
    );
}

#[test]
fn aerodynamic_bounds_and_owner_control_signs_are_validated_at_every_node() {
    for (before, after, field) in [
        (
            "\"cl_alpha_per_rad\": 4.0",
            "\"cl_alpha_per_rad\": 0.0",
            "cl_alpha_per_rad",
        ),
        ("\"cl_max\": 1.2", "\"cl_max\": 0.0", "cl_max"),
        ("\"cd0\": 0.02", "\"cd0\": -0.02", "cd0"),
        (
            "\"oswald_efficiency\": 0.8",
            "\"oswald_efficiency\": 0.0",
            "oswald_efficiency",
        ),
        (
            "\"cm_elevator_per_rad\": -1.0",
            "\"cm_elevator_per_rad\": 1.0",
            "cm_elevator_per_rad",
        ),
        (
            "\"roll_aileron_per_rad\": 0.2",
            "\"roll_aileron_per_rad\": -0.2",
            "roll_aileron_per_rad",
        ),
        (
            "\"yaw_rudder_per_rad\": -0.15",
            "\"yaw_rudder_per_rad\": 0.15",
            "yaw_rudder_per_rad",
        ),
    ] {
        invalid(&FIXTURE.replacen(before, after, 1), field);
    }
    // Check a later node too, rather than only validating the first node.
    invalid(
        &FIXTURE.replace("\"cl_max\": 1.5", "\"cl_max\": -1.5"),
        "cl_max",
    );
}

#[test]
fn saving_also_rejects_non_finite_programmatic_values() {
    let original = from_json(FIXTURE).unwrap();
    for field in 0..5 {
        let mut aircraft = original.clone();
        match field {
            0 => aircraft.mass_kg = f64::NAN,
            1 => aircraft.cg_body_m[1] = f64::INFINITY,
            2 => aircraft.aerodynamic_reference_point_body_m[2] = f64::NAN,
            3 => aircraft.aerodynamics.mach_nodes[1] = f64::INFINITY,
            _ => aircraft.aerodynamics.coefficients[1][1].cl0 = f64::NAN,
        }
        assert!(to_json(&aircraft).is_err());
    }
    invalid(
        &FIXTURE.replace("\"mass_kg\": 10.0", "\"mass_kg\": 1e400"),
        "JSON",
    );
}
