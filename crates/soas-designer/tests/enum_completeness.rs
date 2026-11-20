//! Structural catalogue settings: fixtures/registry_source_values.md, Fix 8.
use soas_designer::{ComputedWingQuantity, SuppliedQuantityKind};

// Deliberately exhaustive, with no wildcard. Rust checks every possible enum
// variant even when it is absent from ALL and never reaches this function.
fn supplied_slot(kind: SuppliedQuantityKind) -> usize {
    match kind {
        SuppliedQuantityKind::TailLiftCurveSlope => 0,
        SuppliedQuantityKind::TailNeutralPointContribution => 1,
        SuppliedQuantityKind::TailPitchingMomentSlope => 2,
        SuppliedQuantityKind::FuselagePitchingMomentContribution => 3,
        SuppliedQuantityKind::InterferenceFactor => 4,
        SuppliedQuantityKind::WholeAircraftClMax => 5,
    }
}

fn computed_slot(kind: ComputedWingQuantity) -> usize {
    match kind {
        ComputedWingQuantity::Area => 0,
        ComputedWingQuantity::Span => 1,
        ComputedWingQuantity::AspectRatio => 2,
        ComputedWingQuantity::TaperRatio => 3,
        ComputedWingQuantity::MeanAerodynamicChord => 4,
        ComputedWingQuantity::MacLeadingEdgePosition => 5,
        ComputedWingQuantity::MacQuarterChordPosition => 6,
        ComputedWingQuantity::QuarterChordSweep => 7,
        ComputedWingQuantity::LiftCoefficient => 8,
        ComputedWingQuantity::LiftSlope => 9,
        ComputedWingQuantity::InducedDragCoefficient => 10,
        ComputedWingQuantity::PitchingMomentCoefficient => 11,
        ComputedWingQuantity::PitchingMomentSlope => 12,
        ComputedWingQuantity::RollingMomentCoefficient => 13,
        ComputedWingQuantity::NeutralPointPosition => 14,
        ComputedWingQuantity::SpanEfficiency => 15,
        ComputedWingQuantity::SectionLoading => 16,
        ComputedWingQuantity::SectionLiftCoefficient => 17,
        ComputedWingQuantity::SectionChord => 18,
        ComputedWingQuantity::SpanwisePosition => 19,
        ComputedWingQuantity::CirculationOverAirspeed => 20,
        ComputedWingQuantity::LiftPerSpanOverDynamicPressure => 21,
        ComputedWingQuantity::InducedAngle => 22,
        ComputedWingQuantity::FourierCoefficient => 23,
        ComputedWingQuantity::FourierHarmonic => 24,
        ComputedWingQuantity::ReferencePointPosition => 25,
        ComputedWingQuantity::RelativeCollocationResidual => 26,
        ComputedWingQuantity::PanelBoundStartPosition => 27,
        ComputedWingQuantity::PanelBoundEndPosition => 28,
        ComputedWingQuantity::PanelControlPointPosition => 29,
        ComputedWingQuantity::PanelCirculationOverAirspeed => 30,
        ComputedWingQuantity::PanelSpanwiseIndex => 31,
        ComputedWingQuantity::PanelChordwiseIndex => 32,
        ComputedWingQuantity::SampledLeadingEdgeX => 33,
        ComputedWingQuantity::SampledLeadingEdgeZ => 34,
        ComputedWingQuantity::SampledTwist => 35,
        ComputedWingQuantity::SampledSectionLiftSlope => 36,
        ComputedWingQuantity::SampledSectionZeroLiftAngle => 37,
        ComputedWingQuantity::FiniteSegmentVelocityPerCirculation => 38,
        ComputedWingQuantity::SemiInfiniteVelocityPerCirculation => 39,
        ComputedWingQuantity::HorseshoeVelocityPerCirculation => 40,
        ComputedWingQuantity::ThinAirfoilLiftSlope => 41,
    }
}

#[test]
fn every_supplied_enum_variant_is_listed_exactly_once() {
    let mut seen = [false; 6];
    for &kind in SuppliedQuantityKind::ALL {
        let slot = supplied_slot(kind);
        assert!(!seen[slot], "duplicate supplied variant: {kind:?}");
        seen[slot] = true;
    }
    assert!(
        seen.into_iter().all(|present| present),
        "missing supplied variant"
    );
}

#[test]
fn every_computed_enum_variant_is_listed_exactly_once() {
    let mut seen = [false; 42];
    for &kind in ComputedWingQuantity::ALL {
        let slot = computed_slot(kind);
        assert!(!seen[slot], "duplicate computed variant: {kind:?}");
        seen[slot] = true;
    }
    assert!(
        seen.into_iter().all(|present| present),
        "missing computed variant"
    );
}
