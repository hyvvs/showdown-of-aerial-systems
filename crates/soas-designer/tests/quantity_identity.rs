//! Verification fixtures are synthetic; all numbers/settings are inventoried in
//! fixtures/verification_values.md. No real aircraft quantity is supplied.
mod support;
use soas_designer::*;

/// Independent expected catalogue from the owner request and existing estimator
/// outputs. This list is not obtained from the production registry.
const QUANTITIES: &[&str] = &[
    "wing_area_m2",
    "wing_span_m",
    "wing_aspect_ratio",
    "wing_taper_ratio",
    "wing_mean_aerodynamic_chord_m",
    "wing_mac_leading_edge_position_body_m",
    "wing_mac_quarter_chord_position_body_m",
    "wing_quarter_chord_sweep_rad",
    "wing_lift_coefficient",
    "wing_lift_slope_per_rad",
    "wing_induced_drag_coefficient",
    "wing_pitching_moment_coefficient",
    "wing_pitching_moment_slope_per_rad",
    "wing_rolling_moment_coefficient",
    "wing_neutral_point_position_body_m",
    "wing_span_efficiency",
    "wing_section_loading",
    "wing_section_lift_coefficient",
    "wing_section_chord_m",
    "wing_spanwise_position_m",
    "wing_circulation_over_airspeed_m",
    "wing_lift_per_span_over_dynamic_pressure_m",
    "wing_induced_angle_rad",
    "wing_fourier_coefficient",
    "wing_fourier_harmonic",
    "wing_reference_point_position_body_m",
    "wing_relative_collocation_residual",
    "wing_panel_bound_start_position_body_m",
    "wing_panel_bound_end_position_body_m",
    "wing_panel_control_point_position_body_m",
    "wing_panel_circulation_over_airspeed_m",
    "wing_panel_spanwise_index",
    "wing_panel_chordwise_index",
    "wing_sampled_leading_edge_x_m",
    "wing_sampled_leading_edge_z_m",
    "wing_sampled_twist_rad",
    "wing_sampled_section_lift_slope_per_rad",
    "wing_sampled_section_zero_lift_angle_rad",
    "finite_segment_velocity_per_circulation_per_m",
    "semi_infinite_velocity_per_circulation_per_m",
    "horseshoe_velocity_per_circulation_per_m",
    "thin_airfoil_lift_slope_per_rad",
];

fn relabel_parameter(d: &mut Design, name: &str, origin: ProvenanceFlag) {
    let old = "contributions[tail-law].parameters[cm_alpha_per_rad]";
    let mut meta = d.provenance.remove(old).unwrap();
    meta.flags = vec![origin];
    d.contributions[0].parameters[0].name = name.into();
    d.contributions[0].parameters[0].quantity =
        QuantityIdentity::ComputedWing(ComputedWingQuantity::Area);
    d.provenance
        .insert(format!("contributions[tail-law].parameters[{name}]"), meta);
}

#[test]
fn original_casing_and_separator_bypasses_are_rejected() {
    for name in ["Wing_area_m2", "wing-area_m2"] {
        for origin in [
            ProvenanceFlag::UserSupplied,
            ProvenanceFlag::TemplateDefault,
        ] {
            let mut d = support::fixture();
            relabel_parameter(&mut d, name, origin);
            assert!(
                d.validate().is_err(),
                "accepted forbidden quantity: {name}, {origin:?}"
            );
        }
    }
}

#[test]
fn each_computable_quantity_is_rejected_in_each_scope_origin_and_spelling() {
    for (q, canonical) in ComputedWingQuantity::ALL.iter().zip(QUANTITIES) {
        let spellings = [
            canonical.to_string(),
            canonical.to_uppercase(),
            canonical.replace('_', "-"),
            canonical.replace('_', " "),
        ];
        for scope in [
            ContributionScope::Wing,
            ContributionScope::Tail,
            ContributionScope::Fuselage,
            ContributionScope::Interference,
        ] {
            for origin in [
                ProvenanceFlag::UserSupplied,
                ProvenanceFlag::TemplateDefault,
            ] {
                for name in &spellings {
                    let mut d = support::fixture();
                    relabel_parameter(&mut d, name, origin);
                    d.contributions[0].scope = scope;
                    d.contributions[0].parameters[0].quantity = QuantityIdentity::ComputedWing(*q);
                    assert_eq!(
                        d.validate().unwrap_err(),
                        DesignError {
                            field: format!("contributions[tail-law].parameters[{name}]"),
                            reason: "computed wing quantities cannot be supplied",
                        },
                        "quantity={canonical}, scope={scope:?}, origin={origin:?}, spelling={name}"
                    );
                }
            }
        }
    }
}

#[test]
fn physical_control_sign_is_independent_of_hinge_endpoint_order() {
    let d = support::fixture();
    let expected = [
        "trailing edge down; nose-down moment",
        "left trailing edge down, right up; positive roll",
        "trailing edge left; nose-left yaw moment",
    ];
    for (control, sign) in d.controls.iter().zip(expected) {
        let mut reversed = control.clone();
        std::mem::swap(
            &mut reversed.hinge_start_body_m,
            &mut reversed.hinge_end_body_m,
        );
        assert_eq!(control.kind.positive_deflection(), sign);
        assert_eq!(reversed.kind.positive_deflection(), sign);
    }
}

#[test]
fn registry_matches_independent_estimator_output_catalogue() {
    let actual: Vec<_> = ComputedWingQuantity::ALL
        .iter()
        .map(|q| q.canonical_name())
        .collect();
    assert_eq!(actual, QUANTITIES);
}

#[test]
fn typed_computed_identity_cannot_hide_behind_an_arbitrary_display_label() {
    for q in ComputedWingQuantity::ALL {
        for scope in [
            ContributionScope::Wing,
            ContributionScope::Tail,
            ContributionScope::Fuselage,
            ContributionScope::Interference,
        ] {
            for origin in [
                ProvenanceFlag::UserSupplied,
                ProvenanceFlag::TemplateDefault,
            ] {
                let mut d = support::fixture();
                relabel_parameter(&mut d, "arbitrary_display_label", origin);
                d.contributions[0].scope = scope;
                d.contributions[0].parameters[0].quantity = QuantityIdentity::ComputedWing(*q);
                assert_eq!(
                    d.validate().unwrap_err(),
                    DesignError {
                        field: "contributions[tail-law].parameters[arbitrary_display_label]".into(),
                        reason: "computed wing quantities cannot be supplied",
                    }
                );
            }
        }
    }
}
