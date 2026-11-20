//! Semantic output registry, not aerodynamic formulas. Coverage/extension rules:
//! docs/S3_01_SEMANTIC_MODEL.md, Computed wing quantity registry.

macro_rules! wing_quantities {
    ($($variant:ident => $name:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ComputedWingQuantity { $($variant),+ }
        impl ComputedWingQuantity {
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];
            pub fn canonical_name(self) -> &'static str {
                match self { $(Self::$variant => $name),+ }
            }
        }
    };
}

wing_quantities! {
    Area => "wing_area_m2",
    Span => "wing_span_m",
    AspectRatio => "wing_aspect_ratio",
    TaperRatio => "wing_taper_ratio",
    MeanAerodynamicChord => "wing_mean_aerodynamic_chord_m",
    MacLeadingEdgePosition => "wing_mac_leading_edge_position_body_m",
    MacQuarterChordPosition => "wing_mac_quarter_chord_position_body_m",
    QuarterChordSweep => "wing_quarter_chord_sweep_rad",
    LiftCoefficient => "wing_lift_coefficient",
    LiftSlope => "wing_lift_slope_per_rad",
    InducedDragCoefficient => "wing_induced_drag_coefficient",
    PitchingMomentCoefficient => "wing_pitching_moment_coefficient",
    PitchingMomentSlope => "wing_pitching_moment_slope_per_rad",
    RollingMomentCoefficient => "wing_rolling_moment_coefficient",
    NeutralPointPosition => "wing_neutral_point_position_body_m",
    SpanEfficiency => "wing_span_efficiency",
    SectionLoading => "wing_section_loading",
    SectionLiftCoefficient => "wing_section_lift_coefficient",
    SectionChord => "wing_section_chord_m",
    SpanwisePosition => "wing_spanwise_position_m",
    CirculationOverAirspeed => "wing_circulation_over_airspeed_m",
    LiftPerSpanOverDynamicPressure => "wing_lift_per_span_over_dynamic_pressure_m",
    InducedAngle => "wing_induced_angle_rad",
    FourierCoefficient => "wing_fourier_coefficient",
    FourierHarmonic => "wing_fourier_harmonic",
    ReferencePointPosition => "wing_reference_point_position_body_m",
    RelativeCollocationResidual => "wing_relative_collocation_residual",
    PanelBoundStartPosition => "wing_panel_bound_start_position_body_m",
    PanelBoundEndPosition => "wing_panel_bound_end_position_body_m",
    PanelControlPointPosition => "wing_panel_control_point_position_body_m",
    PanelCirculationOverAirspeed => "wing_panel_circulation_over_airspeed_m",
    PanelSpanwiseIndex => "wing_panel_spanwise_index",
    PanelChordwiseIndex => "wing_panel_chordwise_index",
    SampledLeadingEdgeX => "wing_sampled_leading_edge_x_m",
    SampledLeadingEdgeZ => "wing_sampled_leading_edge_z_m",
    SampledTwist => "wing_sampled_twist_rad",
    SampledSectionLiftSlope => "wing_sampled_section_lift_slope_per_rad",
    SampledSectionZeroLiftAngle => "wing_sampled_section_zero_lift_angle_rad",
    FiniteSegmentVelocityPerCirculation => "finite_segment_velocity_per_circulation_per_m",
    SemiInfiniteVelocityPerCirculation => "semi_infinite_velocity_per_circulation_per_m",
    HorseshoeVelocityPerCirculation => "horseshoe_velocity_per_circulation_per_m",
    ThinAirfoilLiftSlope => "thin_airfoil_lift_slope_per_rad",
}

// One declaration generates the enum, complete ALL list and exhaustive
// metadata mappings. There is no separately maintained supplied-variant list.
// Independent exhaustive test matches still require deliberate classification
// of every variant, even if someone edits the generated enum/list separately.
macro_rules! supplied_quantities {
    ($($variant:ident => ($name:literal, $scope:ident, $unit:literal)),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum SuppliedQuantityKind { $($variant),+ }
        impl SuppliedQuantityKind {
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];
            pub fn canonical_name(self) -> &'static str {
                match self { $(Self::$variant => $name),+ }
            }
            pub fn scope(self) -> crate::ContributionScope {
                match self { $(Self::$variant => crate::ContributionScope::$scope),+ }
            }
            pub fn unit(self) -> &'static str {
                match self { $(Self::$variant => $unit),+ }
            }
        }
    };
}

// Closed non-wing domain: owner's Fix 5 examples and existing tail-law/CLmax.
// Extension protocol: docs/S3_01_SEMANTIC_MODEL.md, Enum/list completeness.
supplied_quantities! {
    TailLiftCurveSlope => ("tail_lift_curve_slope", Tail, "1/rad"),
    TailNeutralPointContribution => ("tail_neutral_point_contribution", Tail, "m"),
    TailPitchingMomentSlope => ("tail_pitching_moment_slope", Tail, "1/rad"),
    FuselagePitchingMomentContribution => ("fuselage_pitching_moment_contribution", Fuselage, "1"),
    InterferenceFactor => ("interference_factor", Interference, "1"),
    WholeAircraftClMax => ("whole_aircraft_cl_max", ClMax, "1"),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantityIdentity {
    Supplied(SuppliedQuantityKind),
    ComputedWing(ComputedWingQuantity),
    /// Explicit invalid declarations give precise errors; neither is a default.
    Missing,
    Unknown(String),
}
