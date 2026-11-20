//! All values/settings are synthetic: fixtures/closed_kind_values.md.
mod support;
use soas_designer::*;

// Independent closed contract, not generated from the implementation registry.
const SUPPLIED: &[(SuppliedQuantityKind, &str, ContributionScope, &str)] = &[
    (
        SuppliedQuantityKind::TailLiftCurveSlope,
        "tail_lift_curve_slope",
        ContributionScope::Tail,
        "1/rad",
    ),
    (
        SuppliedQuantityKind::TailNeutralPointContribution,
        "tail_neutral_point_contribution",
        ContributionScope::Tail,
        "m",
    ),
    (
        SuppliedQuantityKind::TailPitchingMomentSlope,
        "tail_pitching_moment_slope",
        ContributionScope::Tail,
        "1/rad",
    ),
    (
        SuppliedQuantityKind::FuselagePitchingMomentContribution,
        "fuselage_pitching_moment_contribution",
        ContributionScope::Fuselage,
        "1",
    ),
    (
        SuppliedQuantityKind::InterferenceFactor,
        "interference_factor",
        ContributionScope::Interference,
        "1",
    ),
    (
        SuppliedQuantityKind::WholeAircraftClMax,
        "whole_aircraft_cl_max",
        ContributionScope::ClMax,
        "1",
    ),
];

pub fn parameter(
    kind: QuantityIdentity,
    scope: ContributionScope,
    label: &str,
    unit: &str,
    flags: Vec<ProvenanceFlag>,
) -> Design {
    let mut d = support::fixture();
    let positive = d.contributions[1].parameters[0].value;
    let c = &mut d.contributions[0];
    c.scope = scope;
    let a = &mut c.parameters[0];
    a.quantity = kind;
    a.name = label.into();
    a.unit = unit.into();
    // Reuse existing ledgered synthetic values, not a new aerodynamic default.
    if scope == ContributionScope::ClMax {
        a.value = positive;
    }
    let mut meta = support::metadata(a.value, unit);
    meta.flags = flags;
    d.provenance
        .remove("contributions[tail-law].parameters[cm_alpha_per_rad]")
        .unwrap();
    d.provenance
        .insert(format!("contributions[tail-law].parameters[{label}]"), meta);
    d
}

fn rejects(d: &Design, label: &str, reason: &'static str) {
    assert_eq!(
        d.validate(),
        Err(DesignError {
            field: format!("contributions[tail-law].parameters[{label}]"),
            reason
        })
    );
}

#[test]
fn a_legitimate_tail_kind_can_have_a_computed_quantity_as_its_label() {
    let mut d = support::fixture();
    let old = "contributions[tail-law].parameters[cm_alpha_per_rad]";
    let meta = d.provenance.remove(old).unwrap();
    d.contributions[0].parameters[0].name = "wing_area_m2".into();
    d.provenance.insert(
        "contributions[tail-law].parameters[wing_area_m2]".into(),
        meta,
    );
    // The fixture parameter is a tail pitching-moment slope. Its label must
    // not turn it into wing area or select square metres as its units.
    d.validate().unwrap();
}

#[test]
fn each_closed_kind_is_accepted_only_in_its_scope_and_with_supplied_origins() {
    for &(kind, _, allowed, unit) in SUPPLIED {
        for scope in [
            ContributionScope::Tail,
            ContributionScope::Fuselage,
            ContributionScope::Interference,
            ContributionScope::ClMax,
            ContributionScope::Wing,
            ContributionScope::WholeAircraft,
        ] {
            for flag in [
                ProvenanceFlag::UserSupplied,
                ProvenanceFlag::TemplateDefault,
                ProvenanceFlag::EstimatedWingOnly,
                ProvenanceFlag::DerivedFromInputs,
                ProvenanceFlag::NotEstimated,
                ProvenanceFlag::OutsideCoverage,
            ] {
                let d = parameter(
                    QuantityIdentity::Supplied(kind),
                    scope,
                    "free label",
                    unit,
                    vec![flag],
                );
                if matches!(
                    scope,
                    ContributionScope::Wing | ContributionScope::WholeAircraft
                ) {
                    assert_eq!(
                        d.validate(),
                        Err(DesignError {
                            field: "contributions[tail-law].scope".into(),
                            reason: "static wing or opaque aircraft totals are forbidden"
                        })
                    );
                } else if scope != allowed {
                    rejects(
                        &d,
                        "free label",
                        "quantity kind does not belong to contribution scope",
                    );
                } else if matches!(
                    flag,
                    ProvenanceFlag::UserSupplied | ProvenanceFlag::TemplateDefault
                ) {
                    d.validate().unwrap();
                } else {
                    rejects(
                        &d,
                        "free label",
                        "present input requires supplied or template provenance",
                    );
                }
            }
        }
        for flag in [
            ProvenanceFlag::UserSupplied,
            ProvenanceFlag::TemplateDefault,
        ] {
            parameter(
                QuantityIdentity::Supplied(kind),
                allowed,
                "free label",
                unit,
                vec![flag, ProvenanceFlag::OutsideCoverage],
            )
            .validate()
            .unwrap();
        }
    }
}

#[test]
fn unknown_and_missing_kinds_are_rejected_in_every_nonwing_scope() {
    for scope in [
        ContributionScope::Tail,
        ContributionScope::Fuselage,
        ContributionScope::Interference,
        ContributionScope::ClMax,
    ] {
        for (kind, reason) in [
            (
                QuantityIdentity::Missing,
                "supplied quantity kind is required",
            ),
            (
                QuantityIdentity::Unknown("not_a_registered_kind".into()),
                "unknown supplied quantity kind",
            ),
        ] {
            for origin in [
                ProvenanceFlag::UserSupplied,
                ProvenanceFlag::TemplateDefault,
            ] {
                rejects(
                    &parameter(kind.clone(), scope, "cl_max", "1", vec![origin]),
                    "cl_max",
                    reason,
                );
            }
        }
    }
}

fn labels(name: &str) -> Vec<String> {
    vec![
        name.into(),
        name.to_uppercase(),
        name.replace('_', "-"),
        name.replace('_', " "),
        format!("{name}_unit"),
        format!("{name}_m"),
        format!("{name}_m2"),
        format!("est_{name}"),
        format!("calc_{name}"),
        format!("user_{name}"),
        format!("free text embeds {name} here"),
        format!("tail_{name}_scale_unit"),
    ]
}

#[test]
fn verifier_decorations_never_override_computed_or_legitimate_supplied_kinds() {
    let computed = [
        ComputedWingQuantity::Area,
        ComputedWingQuantity::Span,
        ComputedWingQuantity::AspectRatio,
        ComputedWingQuantity::TaperRatio,
        ComputedWingQuantity::MeanAerodynamicChord,
        ComputedWingQuantity::QuarterChordSweep,
        ComputedWingQuantity::LiftCoefficient,
        ComputedWingQuantity::LiftSlope,
        ComputedWingQuantity::InducedDragCoefficient,
        ComputedWingQuantity::PitchingMomentSlope,
        ComputedWingQuantity::NeutralPointPosition,
        ComputedWingQuantity::SpanEfficiency,
    ];
    for q in computed {
        for label in labels(q.canonical_name()) {
            for &(kind, _, scope, unit) in SUPPLIED {
                for origin in [
                    ProvenanceFlag::UserSupplied,
                    ProvenanceFlag::TemplateDefault,
                ] {
                    rejects(
                        &parameter(
                            QuantityIdentity::ComputedWing(q),
                            scope,
                            &label,
                            unit,
                            vec![origin],
                        ),
                        &label,
                        "computed wing quantities cannot be supplied",
                    );
                    parameter(
                        QuantityIdentity::Supplied(kind),
                        scope,
                        &label,
                        unit,
                        vec![origin],
                    )
                    .validate()
                    .unwrap();
                }
            }
        }
    }
}

#[test]
fn supplied_catalogue_is_closed_and_disjoint_from_all_computed_results() {
    let actual: Vec<_> = SuppliedQuantityKind::ALL
        .iter()
        .map(|k| (*k, k.canonical_name(), k.scope(), k.unit()))
        .collect();
    assert_eq!(actual, SUPPLIED);
    for &(_, name, _, _) in SUPPLIED {
        assert!(
            !ComputedWingQuantity::ALL
                .iter()
                .any(|q| q.canonical_name() == name),
            "computed quantity in supplied list: {name}"
        );
    }
    // Also inspect production names, so an implementation edit cannot hide
    // behind the independently authored allowed list.
    for kind in SuppliedQuantityKind::ALL {
        assert!(
            !ComputedWingQuantity::ALL
                .iter()
                .any(|q| q.canonical_name() == kind.canonical_name())
        );
    }
}

#[test]
fn absent_drafts_keep_not_estimated_without_an_implicit_numeric_zero() {
    for &(kind, _, scope, unit) in SUPPLIED {
        let mut d = parameter(
            QuantityIdentity::Supplied(kind),
            scope,
            "draft label",
            unit,
            vec![ProvenanceFlag::NotEstimated],
        );
        d.contributions[0].parameters[0].value = None;
        d.validate().unwrap();
        assert_eq!(d.contributions[0].parameters[0].value, None);
    }
}
