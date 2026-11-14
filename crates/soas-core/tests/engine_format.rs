//! T9 file contract; every fixture number is SYNTHETIC TEST DATA.
#[test]
fn version_one_synthetic_engine_file_loads() {
    let result = soas_core::aircraft::from_json(include_str!("fixtures/synthetic_jet.json"));
    assert!(
        result.is_ok(),
        "version 1 engine file rejected: {}",
        result.unwrap_err()
    );
}
