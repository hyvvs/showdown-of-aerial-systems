//! All fixture numbers are SYNTHETIC TEST VALUES in the companion ledger.
use soas_core::aircraft::{from_json, to_json};
const FIXTURE: &str = include_str!("fixtures/synthetic_dynamic_aircraft.json");
#[test]
fn version_two_loads_and_round_trips_explicit_dynamic_data_and_limits() {
    let result = from_json(FIXTURE);
    assert!(
        result.is_ok(),
        "version 2 rejected: {}",
        result.unwrap_err()
    );
    let a = from_json(FIXTURE).unwrap();
    assert_eq!(from_json(&to_json(&a).unwrap()).unwrap(), a);
}
