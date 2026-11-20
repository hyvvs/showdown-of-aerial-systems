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
}

fn normalize(name: &str) -> String {
    name.chars()
        .filter(|c| *c != '_' && *c != '-' && !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

impl ComputedWingQuantity {
    /// Compatibility spelling recognition prevents a misleading SuppliedNonWing
    /// declaration from laundering a known wing-result label. Display text does
    /// not override an explicit ComputedWing identity, even if text is arbitrary.
    pub fn from_name(name: &str) -> Option<Self> {
        let normalized = normalize(name);
        Self::ALL
            .iter()
            .copied()
            .find(|q| normalize(q.canonical_name()) == normalized)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityIdentity {
    /// Allowed non-wing parameter, subject to scope, source and unit validation.
    SuppliedNonWing,
    /// Explicit forbidden request, retained for a precise validation error.
    ComputedWing(ComputedWingQuantity),
}
