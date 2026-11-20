//! Synthetic label/probe settings: fixtures/registry_source_values.md, Fix 9.
mod support;
use soas_designer::*;

fn parameter(
    label: &str,
    kind: SuppliedQuantityKind,
    scope: ContributionScope,
    unit: &str,
) -> Design {
    let mut d = support::fixture();
    let old = "contributions[tail-law].parameters[cm_alpha_per_rad]";
    let mut meta = d.provenance.remove(old).unwrap();
    meta.flags = vec![ProvenanceFlag::UserSupplied];
    meta.unit = unit.into();
    let c = &mut d.contributions[0];
    c.scope = scope;
    let a = &mut c.parameters[0];
    a.quantity = QuantityIdentity::Supplied(kind);
    a.name = label.into();
    a.unit = unit.into();
    // Keep the existing ledgered synthetic value; no aircraft coefficient
    // or new kind is invented by changing the display label for this probe.
    d.provenance
        .insert(format!("contributions[tail-law].parameters[{label}]"), meta);
    d
}

fn requires_sources(d: &Design, label: &str) {
    // Prove the label and legitimate declared kind are otherwise accepted.
    d.validate().unwrap();
    let path = format!("contributions[tail-law].parameters[{label}]");
    for sources in [
        vec![],
        vec![String::new()],
        vec![" \t\r\n ".into()],
        vec!["\u{2003}\u{00a0}".into()],
        vec!["SYNTHETIC TEST SOURCE".into(), String::new()],
    ] {
        let mut bad = d.clone();
        bad.provenance.get_mut(&path).unwrap().source_refs = sources;
        assert_eq!(
            bad.validate(),
            Err(DesignError {
                field: path.clone(),
                reason: "source or missing-input explanation required",
            })
        );
    }
    for blank in ["", " \t\r\n ", "\u{2003}\u{00a0}"] {
        let mut bad = d.clone();
        bad.contributions[0].source = blank.into();
        assert_eq!(
            bad.validate(),
            Err(DesignError {
                field: "contributions[tail-law].source".into(),
                reason: "must not be empty",
            })
        );
    }
}

#[test]
fn long_user_label_requires_nonempty_sources() {
    let label = "x".repeat(256);
    requires_sources(
        &parameter(
            &label,
            SuppliedQuantityKind::TailPitchingMomentSlope,
            ContributionScope::Tail,
            "1/rad",
        ),
        &label,
    );
}

#[test]
fn long_kind_label_requires_nonempty_user_sources() {
    // Both the genuine longest existing kind ID and a long label made from it.
    // The kind remains a legitimate closed enum variant, never a new string kind.
    let canonical = "fuselage_pitching_moment_contribution";
    assert_eq!(
        SuppliedQuantityKind::FuselagePitchingMomentContribution.canonical_name(),
        canonical
    );
    for label in [canonical.into(), canonical.repeat(8)] {
        requires_sources(
            &parameter(
                &label,
                SuppliedQuantityKind::FuselagePitchingMomentContribution,
                ContributionScope::Fuselage,
                "1",
            ),
            &label,
        );
    }
}

#[test]
fn unicode_user_label_requires_nonempty_sources() {
    // UTF-8 byte length exceeds the verifier's cutoff while character count
    // does not. Source requirements cannot depend on either measurement.
    let label = "\u{7ffc}".repeat(64);
    assert_eq!(label.len(), 192);
    assert_eq!(label.chars().count(), 64);
    requires_sources(
        &parameter(
            &label,
            SuppliedQuantityKind::TailPitchingMomentSlope,
            ContributionScope::Tail,
            "1/rad",
        ),
        &label,
    );
}
