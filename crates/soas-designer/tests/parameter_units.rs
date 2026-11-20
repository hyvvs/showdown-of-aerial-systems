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

fn parameter(kind: SuppliedQuantityKind, name: &str, unit: &str, present: bool) -> Design {
    let mut d = support::fixture();
    d.provenance
        .remove("contributions[tail-law].parameters[cm_alpha_per_rad]")
        .unwrap();
    d.contributions[0].scope = kind.scope();
    let positive = d.contributions[1].parameters[0].value;
    let a = &mut d.contributions[0].parameters[0];
    a.quantity = QuantityIdentity::Supplied(kind);
    a.name = name.into();
    a.unit = unit.into();
    if kind == SuppliedQuantityKind::WholeAircraftClMax {
        a.value = positive;
    }
    if !present {
        a.value = None;
    }
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
fn every_model_unit_probe_is_checked_against_the_explicit_quantity_kind() {
    for kind in SuppliedQuantityKind::ALL {
        for &(name, unit) in PARAMETERS {
            for present in [true, false] {
                let d = parameter(*kind, name, unit, present);
                if unit == kind.unit() {
                    d.validate().unwrap();
                } else {
                    rejects(&d, name, "quantity kind and declared unit disagree");
                }
            }
        }
    }
}
#[test]
fn every_different_kind_unit_pair_is_rejected_in_both_directions() {
    for kind in SuppliedQuantityKind::ALL {
        for other in SuppliedQuantityKind::ALL {
            if kind.unit() == other.unit() {
                continue;
            }
            for present in [true, false] {
                rejects(
                    &parameter(*kind, "free label", other.unit(), present),
                    "free label",
                    "quantity kind and declared unit disagree",
                );
            }
        }
    }
}
#[test]
fn non_si_and_wrong_case_units_do_not_bypass_the_si_boundary() {
    for kind in SuppliedQuantityKind::ALL {
        for unit in ["mm", "ft", "deg", "n", "furlong"] {
            rejects(
                &parameter(*kind, "free label", unit, true),
                "free label",
                "quantity kind and declared unit disagree",
            );
        }
    }
}
#[test]
fn unknown_label_suffixes_have_no_meaning_but_declared_units_still_must_match() {
    for name in [
        "nonwing_length_mm",
        "nonwing_length_ft",
        "nonwing_angle_deg",
        "nonwing_value_furlong",
        "nonwing_speed_mps",
        "nonwing_length_M",
    ] {
        for kind in SuppliedQuantityKind::ALL {
            parameter(*kind, name, kind.unit(), true)
                .validate()
                .unwrap();
            rejects(
                &parameter(*kind, name, "mm", true),
                name,
                "quantity kind and declared unit disagree",
            );
        }
    }
}
#[test]
fn a_long_or_short_label_suffix_never_selects_the_quantity_unit() {
    for name in ["nonwing_inertia_kg_m2", "nonwing_slope_per_rad"] {
        for kind in SuppliedQuantityKind::ALL {
            parameter(*kind, name, kind.unit(), true)
                .validate()
                .unwrap();
            rejects(
                &parameter(*kind, name, "m^2", true),
                name,
                "quantity kind and declared unit disagree",
            );
        }
    }
}
