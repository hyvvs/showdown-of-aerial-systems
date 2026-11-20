//! All numbers/indices/invalid probes: fixtures/semantic_values.md.
mod support;
use soas_designer::*;
use support::{fixture, rad};

fn rejects(d: &Design, field: &str, reason: &'static str) {
    assert_eq!(
        d.validate().unwrap_err(),
        DesignError {
            field: field.into(),
            reason
        }
    );
}

#[test]
fn independent_component_inventory_and_every_numeric_path_are_preserved() {
    let d = fixture();
    d.validate().unwrap();
    let actual: Vec<_> = d
        .numbers()
        .into_iter()
        .map(|n| (n.path, n.value, n.unit))
        .collect();
    assert_eq!(actual, support::expected_numbers(&d));
    assert_eq!(d.datum, "synthetic common body datum");
    assert_eq!(
        d.components
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        ["wing", "tail", "fin", "body", "engine"]
    );
    assert!(d.components[1].mass.intrinsic_inertia.is_none());
}

#[test]
fn hand_mirrored_points_preserve_forward_and_down_axes_once() {
    let d = fixture();
    assert_eq!(
        d.surface_points_body_m("wing").unwrap(),
        [[1.0, 0.0, 3.0], [-2.0, 4.0, 5.0], [-2.0, -4.0, 5.0]]
    );
    assert_eq!(
        d.surface_points_body_m("fin").unwrap(),
        [[-6.0, 0.0, 2.0], [-7.0, 0.0, -1.0]]
    );
}

#[test]
fn engine_placement_is_body_xyz_and_force_direction_not_exhaust() {
    let d = fixture();
    let p = d.engine_placement("engine").unwrap();
    assert_eq!(p.position_body_m, [7.0, 8.0, 9.0]);
    assert_eq!(p.force_direction_body_unit, [0.0, 0.6, 0.8]);
    assert_eq!(p.table_id, "jet");
}

#[test]
fn each_control_has_its_recorded_physical_positive_direction() {
    assert_eq!(
        ControlKind::Elevator.positive_deflection(),
        "trailing edge down; nose-down moment"
    );
    assert_eq!(
        ControlKind::Aileron.positive_deflection(),
        "left trailing edge down, right up; positive roll"
    );
    assert_eq!(
        ControlKind::Rudder.positive_deflection(),
        "trailing edge left; nose-left yaw moment"
    );
}

#[test]
fn duplicate_id_has_exact_error() {
    let mut d = fixture();
    d.controls[0].id = "wing".into();
    rejects(&d, "controls[wing].id", "duplicate identity");
}

#[test]
fn missing_or_nonsurface_control_parent_is_rejected() {
    for parent in ["missing", "body"] {
        let mut d = fixture();
        d.controls[0].parent_surface_id = parent.into();
        rejects(
            &d,
            "controls[elevator].parent_surface_id",
            "must reference a lifting surface",
        );
    }
}

#[test]
fn dangling_engine_table_is_rejected() {
    let mut d = fixture();
    let ComponentGeometry::Engine(e) = &mut d.components[4].geometry else {
        panic!()
    };
    e.table_id = "absent".into();
    rejects(
        &d,
        "components[engine].engine.table_id",
        "unknown engine table",
    );
}

#[test]
fn nonpositive_nonfinite_or_unordered_geometry_and_bad_controls_fail_clearly() {
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let mut d = fixture();
        let ComponentGeometry::Surface(s) = &mut d.components[0].geometry else {
            panic!()
        };
        s.stations[0].chord_m = value;
        rejects(
            &d,
            "components[wing].surface.stations[0].chord_m",
            "must be finite and positive",
        );
    }
    let mut d = fixture();
    let ComponentGeometry::Surface(s) = &mut d.components[0].geometry else {
        panic!()
    };
    s.stations[1].leading_edge_body_m[1] = 0.0;
    rejects(
        &d,
        "components[wing].surface.stations[1]",
        "stations must progress outboard",
    );
    let mut d = fixture();
    let ComponentGeometry::Fuselage(f) = &mut d.components[3].geometry else {
        panic!()
    };
    f.sections[0].width_m = -1.0;
    rejects(
        &d,
        "components[body].fuselage.sections[0].width_m",
        "must be finite and positive",
    );
    let mut d = fixture();
    d.controls[0].max_rad = rad(-0.4);
    rejects(
        &d,
        "controls[elevator].limits",
        "minimum must not exceed maximum",
    );
}

#[test]
fn missing_inertia_and_nonwing_data_are_explicit_not_numeric_zero() {
    let d = fixture();
    d.validate().unwrap();
    let n = d
        .numbers()
        .into_iter()
        .find(|n| n.path == "components[tail].mass.intrinsic_inertia")
        .unwrap();
    assert_eq!(n.value, None);
    assert_eq!(d.provenance[&n.path].flags, [ProvenanceFlag::NotEstimated]);
    let mut draft = d.clone();
    draft.contributions.clear();
    draft
        .provenance
        .retain(|p, _| !p.starts_with("contributions["));
    draft.validate().unwrap();
    assert!(draft.contributions.is_empty());
}

#[test]
fn tensor_entries_each_have_provenance_and_unknown_metadata_is_rejected() {
    let mut d = fixture();
    let path = "components[wing].mass.intrinsic_inertia.tensor_kg_m2[0][1]";
    d.provenance.remove(path);
    rejects(&d, path, "numeric provenance missing");
    let mut d = fixture();
    d.provenance
        .insert("invented".into(), support::metadata(Some(1.0), "1"));
    rejects(&d, "invented", "orphan numeric provenance");
}

#[test]
fn empty_flags_wrong_units_or_false_missing_origins_are_rejected() {
    let path = "components[wing].surface.stations[0].chord_m";
    let mut d = fixture();
    d.provenance.get_mut(path).unwrap().flags.clear();
    rejects(&d, path, "provenance flags must not be empty");
    let mut d = fixture();
    d.provenance.get_mut(path).unwrap().unit = "kg".into();
    rejects(&d, path, "provenance unit mismatch");
    let mut d = fixture();
    d.provenance.get_mut(path).unwrap().flags = vec![ProvenanceFlag::NotEstimated];
    rejects(
        &d,
        path,
        "present input requires supplied or template provenance",
    );
    let mut d = fixture();
    let p = "components[tail].mass.intrinsic_inertia";
    d.provenance.get_mut(p).unwrap().flags = vec![ProvenanceFlag::UserSupplied];
    rejects(&d, p, "absent input requires NOT_ESTIMATED");
}

#[test]
fn template_values_are_distinct_from_user_values_and_outside_coverage_is_retained() {
    let mut d = fixture();
    let p = "contributions[lift-limit].parameters[cl_max]";
    d.provenance.get_mut(p).unwrap().flags = vec![
        ProvenanceFlag::TemplateDefault,
        ProvenanceFlag::OutsideCoverage,
    ];
    d.validate().unwrap();
    assert_eq!(d.provenance[p].flags[0].as_str(), "TEMPLATE_DEFAULT");
    assert_eq!(d.provenance[p].flags[1].as_str(), "OUTSIDE_COVERAGE");
}

#[test]
fn static_wing_overrides_and_opaque_aircraft_totals_are_rejected() {
    for scope in [ContributionScope::Wing, ContributionScope::WholeAircraft] {
        let mut d = fixture();
        d.contributions[0].scope = scope;
        rejects(
            &d,
            "contributions[tail-law].scope",
            "static wing or opaque aircraft totals are forbidden",
        );
    }
    let mut d = fixture();
    let p = "components[wing].surface.stations[0].chord_m";
    d.provenance.get_mut(p).unwrap().flags = vec![ProvenanceFlag::EstimatedWingOnly];
    rejects(
        &d,
        p,
        "present input requires supplied or template provenance",
    );
}

#[test]
fn provenance_labels_never_relabel_mixed_results_as_wing_only() {
    let flags = [
        ProvenanceFlag::EstimatedWingOnly,
        ProvenanceFlag::UserSupplied,
        ProvenanceFlag::TemplateDefault,
        ProvenanceFlag::NotEstimated,
        ProvenanceFlag::OutsideCoverage,
        ProvenanceFlag::DerivedFromInputs,
    ];
    assert_eq!(
        flags.map(|f| f.as_str()),
        [
            "ESTIMATED_WING_ONLY",
            "USER_SUPPLIED",
            "TEMPLATE_DEFAULT",
            "NOT_ESTIMATED",
            "OUTSIDE_COVERAGE",
            "DERIVED_FROM_INPUTS"
        ]
    );
}

#[test]
fn synthetic_template_cannot_imply_a_real_aircraft_identity() {
    let mut d = fixture();
    d.name = "F-104".into();
    rejects(
        &d,
        "name",
        "synthetic template requires a registered neutral name",
    );
    let mut d = fixture();
    d.real_aircraft_claim = Some("real aircraft".into());
    rejects(
        &d,
        "real_aircraft_claim",
        "real aircraft identity requires a future validated-data contract",
    );
}

#[test]
fn malformed_engine_unit_vector_and_inertia_are_not_repaired() {
    let mut d = fixture();
    let ComponentGeometry::Engine(e) = &mut d.components[4].geometry else {
        panic!()
    };
    e.direction_body_unit = [0.0, 0.0, 0.0];
    assert!(
        d.validate()
            .unwrap_err()
            .field
            .ends_with("direction_body_unit")
    );
    let mut d = fixture();
    d.components[0]
        .mass
        .intrinsic_inertia
        .as_mut()
        .unwrap()
        .tensor_kg_m2[0][1] = 0.0;
    rejects(
        &d,
        "components[wing].mass.intrinsic_inertia",
        "invalid signed intrinsic inertia tensor",
    );
}
