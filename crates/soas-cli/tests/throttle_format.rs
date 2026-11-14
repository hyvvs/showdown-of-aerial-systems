//! T9 scenario contract; every fixture number is SYNTHETIC TEST DATA.
#[test]
fn version_one_synthetic_throttle_scenario_loads() {
    let result = soas_cli::parse_scenario(include_str!("fixtures/synthetic_powered_scenario.json"));
    assert!(
        result.is_ok(),
        "version 1 throttle scenario rejected: {}",
        result.unwrap_err()
    );
}
