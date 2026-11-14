mod support {
    pub mod object_shapes;
}
#[test]
fn every_discovered_aircraft_fixture_object_rejects_positional_values() {
    support::object_shapes::check_directory(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"),
        |text| {
            soas_core::aircraft::from_json(text)
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
    );
}
