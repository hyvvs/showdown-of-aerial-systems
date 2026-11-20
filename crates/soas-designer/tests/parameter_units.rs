//! Synthetic parameter probes/settings: fixtures/verification_values.md.
mod support;
use soas_designer::*;

// Independently listed contract, not read from the implementation's suffix map.
const PARAMETERS: &[(&str, &str)] = &[
    ("nonwing_inertia_kg_m2", "kg*m^2"),
    ("nonwing_slope_per_rad", "1/rad"),
    ("nonwing_fuel_flow_kgps", "kg/s"),
    ("nonwing_area_m2", "m^2"),
    ("nonwing_angle_rad", "rad"),
    ("nonwing_mass_kg", "kg"),
    ("nonwing_length_m", "m"),
    ("nonwing_thrust_n", "N"),
    ("nonwing_direction_unit", "1"),
    ("nonwing_chord_fraction", "1"),
    ("cl_max", "1"),
];

fn parameter(name: &str, unit: &str, present: bool) -> Design {
    let mut d = support::fixture();
    let old = "contributions[tail-law].parameters[cm_alpha_per_rad]";
    d.provenance.remove(old).unwrap();
    let a = &mut d.contributions[0].parameters[0];
    a.name = name.into();
    a.unit = unit.into();
    if !present {
        a.value = None;
    }
    // Make metadata agree with the declared unit. Otherwise the old metadata
    // check would mask the missing name/unit consistency guard.
    d.provenance.insert(
        format!("contributions[tail-law].parameters[{name}]"),
        support::metadata(a.value, unit),
    );
    d
}

fn rejects(d: &Design, name: &str, reason: &'static str) {
    assert_eq!(
        d.validate(),
        Err(DesignError {
            field: format!("contributions[tail-law].parameters[{name}]"),
            reason
        })
    );
}

#[test]
fn every_model_unit_suffix_accepts_only_its_declared_si_unit() {
    for &(name, unit) in PARAMETERS {
        for present in [true, false] {
            parameter(name, unit, present).validate().unwrap();
        }
    }
}

#[test]
fn every_different_unit_pair_is_rejected_in_both_directions() {
    for &(name, expected) in PARAMETERS {
        for &(_, wrong) in PARAMETERS {
            if expected == wrong {
                continue;
            }
            for present in [true, false] {
                rejects(
                    &parameter(name, wrong, present),
                    name,
                    "parameter suffix and declared unit disagree",
                );
            }
        }
    }
}

#[test]
fn matching_non_si_spelling_does_not_bypass_the_si_boundary() {
    for (name, unit) in [
        ("nonwing_length_m", "mm"),
        ("nonwing_length_m", "ft"),
        ("nonwing_angle_rad", "deg"),
        ("nonwing_thrust_n", "n"),
    ] {
        rejects(
            &parameter(name, unit, true),
            name,
            "parameter suffix and declared unit disagree",
        );
    }
}

#[test]
fn unknown_suffixes_are_rejected_even_when_metadata_agrees() {
    for (name, unit) in [
        ("nonwing_length_mm", "mm"),
        ("nonwing_length_mm", "m"),
        ("nonwing_length_ft", "ft"),
        ("nonwing_length_ft", "m"),
        ("nonwing_angle_deg", "deg"),
        ("nonwing_angle_deg", "rad"),
        ("nonwing_value_furlong", "m"),
        ("nonwing_speed_mps", "m"),
        ("nonwing_length_M", "m"),
    ] {
        rejects(
            &parameter(name, unit, true),
            name,
            "unknown supplied-parameter unit suffix",
        );
    }
}

#[test]
fn a_long_suffix_cannot_be_misread_as_its_shorter_suffix() {
    // kg_m2 must not be interpreted as area; per_rad is not an angle.
    for (name, wrong) in [
        ("nonwing_inertia_kg_m2", "m^2"),
        ("nonwing_slope_per_rad", "rad"),
    ] {
        rejects(
            &parameter(name, wrong, true),
            name,
            "parameter suffix and declared unit disagree",
        );
    }
}
