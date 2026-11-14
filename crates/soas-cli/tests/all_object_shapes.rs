#[path = "../../soas-core/tests/support/object_shapes.rs"]
mod object_shapes;
#[test]
fn every_discovered_scenario_fixture_object_rejects_positional_values() {
    object_shapes::check_directory(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"),
        |text| {
            soas_cli::parse_scenario(text)
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
    );
}
