//! Review Fix 1: scenario records must be named objects. Synthetic values only.
use serde_json::Value;
use soas_cli::parse_scenario;
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/synthetic_scenario.json")).unwrap()
}
fn sequence(value: &Value, fields: &[&str]) -> Value {
    Value::Array(fields.iter().map(|field| value[*field].clone()).collect())
}
fn rejects(value: &Value, path: &str) {
    let result = parse_scenario(&value.to_string());
    assert!(result.is_err(), "accepted positional array at {path}");
    let message = result.unwrap_err().to_string();
    assert!(
        message.contains(path) && message.contains("object"),
        "unclear error at {path}: {message}"
    );
    println!("rejected {path}: {message}");
}
#[test]
fn scenario_root_array_is_rejected_with_named_object_error() {
    let mut input = fixture();
    input["dt_s"] = Value::from(0.0025); // Existing D10 default.
    rejects(
        &sequence(
            &input,
            &[
                "format_version",
                "data_note",
                "aircraft_path",
                "initial_state",
                "controls",
                "duration_s",
                "dt_s",
                "output_interval_s",
            ],
        ),
        "scenario",
    );
}
#[test]
fn initial_state_array_is_rejected_with_its_field_path() {
    let mut input = fixture();
    input["initial_state"] = sequence(
        &input["initial_state"],
        &[
            "position_ned_m",
            "velocity_body_mps",
            "attitude_euler_deg",
            "angular_rates_body_radps",
        ],
    );
    rejects(&input, "initial_state");
}
#[test]
fn control_schedules_record_array_is_rejected_with_its_field_path() {
    let mut input = fixture();
    input["controls"] = sequence(&input["controls"], &["elevator", "aileron", "rudder"]);
    rejects(&input, "controls");
}
#[test]
fn every_control_surface_knot_array_is_rejected_with_its_field_path() {
    let mut accepted = Vec::new();
    for surface in ["elevator", "aileron", "rudder"] {
        for index in 0..fixture()["controls"][surface].as_array().unwrap().len() {
            let mut input = fixture();
            input["controls"][surface][index] = sequence(
                &input["controls"][surface][index],
                &["time_s", "deflection_deg"],
            );
            let path = format!("controls.{surface}[{index}]");
            match parse_scenario(&input.to_string()) {
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
        "accepted positional knot arrays: {accepted:?}"
    );
}
#[test]
fn scenario_guard_keeps_unknown_duplicate_missing_and_trailing_input_errors() {
    let text = include_str!("fixtures/synthetic_scenario.json");
    for (input, expected) in [
        (
            text.replacen("\"position_ned_m\"", "\"position_feet\"", 1),
            "unknown field `position_feet`",
        ),
        (
            text.replacen("\"duration_s\": 0.2,", "", 1),
            "missing field `duration_s`",
        ),
        (
            text.replacen(
                "\"duration_s\": 0.2,",
                "\"duration_s\": 0.2, \"duration_s\": 0.2,",
                1,
            ),
            "duplicate field `duration_s`",
        ),
        (format!("{text} {{}}"), "trailing characters"),
    ] {
        let message = parse_scenario(&input).unwrap_err().to_string();
        assert!(
            message.contains(expected) && message.contains("line"),
            "{message}"
        );
    }
}
