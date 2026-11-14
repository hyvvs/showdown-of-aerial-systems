//! Review Fix 1: named records must be JSON objects, never positional arrays.
//! All numeric input is from the existing labeled synthetic aircraft fixture.
use serde_json::Value;
use soas_core::aircraft::from_json;

fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/synthetic_aircraft.json")).unwrap()
}
fn sequence(value: &Value, fields: &[&str]) -> Value {
    Value::Array(fields.iter().map(|field| value[*field].clone()).collect())
}
fn rejects(value: &Value, path: &str) {
    let result = from_json(&value.to_string());
    assert!(result.is_err(), "accepted positional array at {path}");
    let message = result.unwrap_err().to_string();
    assert!(
        message.contains(path) && message.contains("object"),
        "unclear error at {path}: {message}"
    );
    println!("rejected {path}: {message}");
}

#[test]
fn aircraft_root_array_is_rejected_with_named_object_error() {
    let input = fixture();
    rejects(
        &sequence(
            &input,
            &[
                "format_version",
                "name",
                "data_note",
                "mass_kg",
                "cg_body_m",
                "inertia_body_kg_m2",
                "reference_area_m2",
                "reference_span_m",
                "reference_chord_m",
                "aerodynamic_reference_point_body_m",
                "coefficient_axes",
                "moments_about",
                "aerodynamics",
            ],
        ),
        "aircraft",
    );
}

#[test]
fn coefficient_table_array_is_rejected_with_its_field_path() {
    let mut input = fixture();
    input["aerodynamics"] = sequence(
        &input["aerodynamics"],
        &["mach_nodes", "geometric_altitude_nodes_m", "coefficients"],
    );
    rejects(&input, "aerodynamics");
}

#[test]
fn every_coefficient_cell_array_is_rejected_including_control_derivative_fields() {
    let fields = [
        "cl0",
        "cl_alpha_per_rad",
        "cl_max",
        "cd0",
        "oswald_efficiency",
        "cm0",
        "cm_alpha_per_rad",
        "cm_elevator_per_rad",
        "cy_beta_per_rad",
        "roll_aileron_per_rad",
        "yaw_rudder_per_rad",
    ];
    // One test per cell mutation, retaining all control derivative values and
    // schema order: no unrelated type/shape error may mask positional decoding.
    let mut accepted = Vec::new();
    for row in 0..2 {
        for column in 0..2 {
            let mut input = fixture();
            let cell = &input["aerodynamics"]["coefficients"][row][column];
            input["aerodynamics"]["coefficients"][row][column] = sequence(cell, &fields);
            let path = format!("aerodynamics.coefficients[{row}][{column}]");
            match from_json(&input.to_string()) {
                Ok(_) => accepted.push(path),
                Err(e) => {
                    let message = e.to_string();
                    assert!(
                        message.contains(&path) && message.contains("object"),
                        "{message}"
                    );
                }
            }
        }
    }
    assert!(
        accepted.is_empty(),
        "accepted positional cell arrays: {accepted:?}"
    );
}

#[test]
fn mass_geometry_and_control_coefficients_keep_their_current_numeric_shape() {
    // There are NO mass, geometry or control-surface record structs in v0:
    // mass/geometry are named scalar fields and controls are cell derivatives.
    // Test arrays replacing those scalars without inventing a new schema.
    for field in [
        "mass_kg",
        "reference_area_m2",
        "reference_span_m",
        "reference_chord_m",
    ] {
        let mut input = fixture();
        input[field] = Value::Array(vec![input[field].clone()]);
        assert!(
            from_json(&input.to_string()).is_err(),
            "accepted array for {field}"
        );
    }
    for field in [
        "cm_elevator_per_rad",
        "roll_aileron_per_rad",
        "yaw_rudder_per_rad",
    ] {
        let mut input = fixture();
        let cell = &mut input["aerodynamics"]["coefficients"][0][0];
        cell[field] = Value::Array(vec![cell[field].clone()]);
        assert!(
            from_json(&input.to_string()).is_err(),
            "accepted array for {field}"
        );
    }
    // Coordinate/inertia vectors and table containers ARE arrays and stay legal.
    assert!(from_json(&fixture().to_string()).is_ok());
}

#[test]
fn object_guard_keeps_unknown_duplicate_missing_and_trailing_input_errors() {
    let text = include_str!("fixtures/synthetic_aircraft.json");
    for (input, expected) in [
        (
            text.replacen("\"mass_kg\"", "\"mass_lb\"", 1),
            "unknown field `mass_lb`",
        ),
        (
            text.replacen("\"mass_kg\": 10.0,", "", 1),
            "missing field `mass_kg`",
        ),
        (
            text.replacen(
                "\"mass_kg\": 10.0,",
                "\"mass_kg\": 10.0, \"mass_kg\": 10.0,",
                1,
            ),
            "duplicate field `mass_kg`",
        ),
        (format!("{text} {{}}"), "trailing characters"),
    ] {
        let message = from_json(&input).unwrap_err().to_string();
        assert!(
            message.contains(expected) && message.contains("line"),
            "{message}"
        );
    }
}
