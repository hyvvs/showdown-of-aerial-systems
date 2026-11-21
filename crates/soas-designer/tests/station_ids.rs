//! SYNTHETIC TEST VALUES/settings only: fixtures/station_id_values.md.
//! Expected paths and values are written independently of Design::numbers.
use soas_core::units::Radians;
use soas_designer::*;
use std::collections::BTreeMap;

const ROOT: [f64; 3] = [1.0, 0.0, 2.0];
const WING_TIP: [f64; 3] = [-2.0, 3.0, 4.0];
const DORSAL_TIP: [f64; 3] = [-2.0, 0.0, -4.0];
const VENTRAL_TIP: [f64; 3] = [-2.0, 0.0, 8.0];
const CASES: &[(SurfaceKind, [f64; 3], bool)] = &[
    (SurfaceKind::Wing, WING_TIP, true),
    (SurfaceKind::HorizontalTail, WING_TIP, false),
    (SurfaceKind::VerticalTail, DORSAL_TIP, false),
    (SurfaceKind::VerticalTail, VENTRAL_TIP, false),
];

fn rad(value: f64) -> Radians {
    Radians::new(value).unwrap()
}

fn expected(id: &str, tip: [f64; 3]) -> Vec<(String, Option<f64>, &'static str)> {
    let prefix = format!("components[{id}]");
    let mut out = vec![
        (format!("{prefix}.surface.incidence_rad"), Some(0.05), "rad"),
        (format!("{prefix}.mass.mass_kg"), None, "kg"),
        (format!("{prefix}.mass.cg_body_m"), None, "m"),
        (format!("{prefix}.mass.intrinsic_inertia"), None, "kg*m^2"),
    ];
    // These IDs deliberately contradict the geometric root/tip roles.
    for (station, point, chord, twist, slope, zero) in [
        ("tip", ROOT, 2.5, 0.125, 6.0, -0.1),
        ("root", tip, 1.25, -0.25, 5.0, -0.2),
    ] {
        let prefix = format!("{prefix}.surface.stations[{station}]");
        for (axis, coordinate) in point.into_iter().enumerate() {
            out.push((
                format!("{prefix}.leading_edge_body_m[{axis}]"),
                Some(coordinate),
                "m",
            ));
        }
        for (field, value, unit) in [
            ("chord_m", chord, "m"),
            ("twist_rad", twist, "rad"),
            ("airfoil.lift_curve_slope_per_rad", slope, "1/rad"),
            ("airfoil.zero_lift_angle_rad", zero, "rad"),
        ] {
            out.push((format!("{prefix}.{field}"), Some(value), unit));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn metadata(path: &str, value: Option<f64>, unit: &str) -> NumberProvenance {
    NumberProvenance {
        flags: vec![if value.is_some() {
            ProvenanceFlag::UserSupplied
        } else {
            ProvenanceFlag::NotEstimated
        }],
        unit: unit.into(),
        source_refs: vec![format!(
            "SYNTHETIC TEST VALUE or missing-input explanation: station_id_values.md; {path}"
        )],
        input_paths: vec![],
        geometry_revision: None,
        approximate: false,
    }
}

fn design(kind: SurfaceKind, tip: [f64; 3], mirrored: bool) -> Design {
    let station = |id: &str, point, chord, twist, slope, zero| Station {
        id: id.into(),
        leading_edge_body_m: point,
        chord_m: chord,
        twist_rad: rad(twist),
        airfoil: Some(AirfoilInput {
            name: "synthetic section input".into(),
            source: "SYNTHETIC TEST VALUES: station_id_values.md".into(),
            lift_curve_slope_per_rad: Some(slope),
            zero_lift_angle_rad: Some(rad(zero)),
        }),
    };
    Design {
        name: "Synthetic station identity test".into(),
        identity: DesignIdentity::UserDesign,
        real_aircraft_claim: None,
        datum: "synthetic common body datum".into(),
        components: vec![Component {
            id: "surface-a".into(),
            name: "synthetic surface".into(),
            geometry: ComponentGeometry::Surface(Surface {
                kind,
                mirrored,
                incidence_rad: rad(0.05),
                stations: vec![
                    station("tip", ROOT, 2.5, 0.125, 6.0, -0.1),
                    station("root", tip, 1.25, -0.25, 5.0, -0.2),
                ],
            }),
            mass: MassInput::default(),
        }],
        engine_tables: vec![],
        controls: vec![],
        contributions: vec![],
        provenance: expected("surface-a", tip)
            .into_iter()
            .map(|(path, value, unit)| {
                let meta = metadata(&path, value, unit);
                (path, meta)
            })
            .collect(),
    }
}

fn surface(d: &Design) -> &Surface {
    let ComponentGeometry::Surface(s) = &d.components[0].geometry else {
        panic!("independent fixture is a surface")
    };
    s
}

fn surface_mut(d: &mut Design) -> &mut Surface {
    let ComponentGeometry::Surface(s) = &mut d.components[0].geometry else {
        panic!("independent fixture is a surface")
    };
    s
}

fn endpoint_ids(d: &Design) -> (&str, &str) {
    let stations = &surface(d).stations;
    (&stations.first().unwrap().id, &stations.last().unwrap().id)
}

#[test]
fn station_numeric_paths_use_the_declared_id_for_every_surface_and_field() {
    for &(kind, tip, mirrored) in CASES {
        let d = design(kind, tip, mirrored);
        d.validate().unwrap();
        let actual: Vec<_> = d
            .numbers()
            .into_iter()
            .map(|n| (n.path, n.value, n.unit))
            .collect();
        assert_eq!(actual, expected("surface-a", tip));
    }
}

#[test]
fn full_reversal_changes_endpoint_ids_without_moving_station_provenance() {
    for &(kind, tip, mirrored) in CASES {
        let mut d = design(kind, tip, mirrored);
        assert_eq!(endpoint_ids(&d), ("tip", "root"));
        let original_provenance = d.provenance.clone();
        surface_mut(&mut d).stations.reverse();
        assert_eq!(endpoint_ids(&d), ("root", "tip"));
        let actual: Vec<_> = d
            .numbers()
            .into_iter()
            .map(|n| (n.path, n.value, n.unit))
            .collect();
        assert_eq!(actual, expected("surface-a", tip));
        assert_eq!(d.provenance, original_provenance);
        // Preserve the existing ordering rule, including its vertical-tail limit.
        if kind == SurfaceKind::VerticalTail {
            d.validate().unwrap();
        } else {
            assert_eq!(
                d.validate(),
                Err(DesignError {
                    field: "components[surface-a].surface.stations[1]".into(),
                    reason: "stations must progress outboard",
                })
            );
        }
    }
}

#[test]
fn duplicate_station_ids_are_rejected_within_each_surface() {
    for &(kind, tip, mirrored) in CASES {
        let mut d = design(kind, tip, mirrored);
        surface_mut(&mut d).stations[1].id = "tip".into();
        assert_eq!(
            d.validate(),
            Err(DesignError {
                field: "components[surface-a].surface.stations[1].id".into(),
                reason: "duplicate identity",
            })
        );
    }
}

#[test]
fn empty_whitespace_and_invalid_station_ids_are_rejected_including_mirrors() {
    for &(kind, tip, mirrored) in CASES {
        for id in ["", " ", "\t\n", "bad.id"] {
            let mut d = design(kind, tip, mirrored);
            surface_mut(&mut d).stations[0].id = id.into();
            assert_eq!(
                d.validate(),
                Err(DesignError {
                    field: "components[surface-a].surface.stations[0].id".into(),
                    reason: "identity must contain only ASCII letters, digits, hyphens or underscores",
                })
            );
        }
    }
}

#[test]
fn moving_a_provenance_record_to_a_different_station_id_is_rejected() {
    let mut d = design(SurfaceKind::VerticalTail, VENTRAL_TIP, false);
    let path = "components[surface-a].surface.stations[tip].chord_m";
    let meta = d.provenance.remove(path).unwrap();
    d.provenance.insert(
        "components[surface-a].surface.stations[other-id].chord_m".into(),
        meta,
    );
    assert_eq!(
        d.validate(),
        Err(DesignError {
            field: path.into(),
            reason: "numeric provenance missing",
        })
    );
}

#[test]
fn the_same_station_ids_are_valid_on_two_different_surfaces() {
    let mut d = design(SurfaceKind::Wing, WING_TIP, true);
    let mut other = design(SurfaceKind::VerticalTail, VENTRAL_TIP, false)
        .components
        .remove(0);
    other.id = "surface-b".into();
    d.components.push(other);
    for (path, value, unit) in expected("surface-b", VENTRAL_TIP) {
        let meta = metadata(&path, value, unit);
        d.provenance.insert(path, meta);
    }
    d.validate().unwrap();
    let actual: BTreeMap<_, _> = d.numbers().into_iter().map(|n| (n.path, n.value)).collect();
    let expected: BTreeMap<_, _> = expected("surface-a", WING_TIP)
        .into_iter()
        .chain(expected("surface-b", VENTRAL_TIP))
        .map(|(path, value, _)| (path, value))
        .collect();
    assert_eq!(actual, expected);
}

#[test]
fn mirroring_keeps_opaque_station_identity_and_the_existing_physical_points() {
    let d = design(SurfaceKind::Wing, WING_TIP, true);
    let original_provenance = d.provenance.clone();
    assert_eq!(
        d.surface_points_body_m("surface-a").unwrap(),
        [ROOT, WING_TIP, [-2.0, -3.0, 4.0]]
    );
    assert_eq!(endpoint_ids(&d), ("tip", "root"));
    assert_eq!(d.provenance, original_provenance);
    assert!(
        d.numbers()
            .iter()
            .any(|n| n.path == "components[surface-a].surface.stations[tip].chord_m")
    );
}

#[test]
fn root_tip_direction_uses_positions_not_the_meaning_of_id_names() {
    for &(kind, tip, mirrored) in CASES {
        let mut d = design(kind, tip, mirrored);
        d.controls.push(ControlSurface {
            id: "control".into(),
            parent_surface_id: "surface-a".into(),
            kind: ControlKind::Aileron,
            hinge_start_body_m: ROOT,
            hinge_end_body_m: tip,
            chord_fraction: 0.2,
            min_rad: rad(-0.3),
            max_rad: rad(0.4),
        });
        for (field, value, unit) in [
            ("hinge_start_body_m[0]", ROOT[0], "m"),
            ("hinge_start_body_m[1]", ROOT[1], "m"),
            ("hinge_start_body_m[2]", ROOT[2], "m"),
            ("hinge_end_body_m[0]", tip[0], "m"),
            ("hinge_end_body_m[1]", tip[1], "m"),
            ("hinge_end_body_m[2]", tip[2], "m"),
            ("chord_fraction", 0.2, "1"),
            ("min_rad", -0.3, "rad"),
            ("max_rad", 0.4, "rad"),
        ] {
            let path = format!("controls[control].{field}");
            let meta = metadata(&path, Some(value), unit);
            d.provenance.insert(path, meta);
        }
        assert_eq!(endpoint_ids(&d), ("tip", "root"));
        d.validate().unwrap();
    }
}

// Fix 10 uses the existing independently specified geometry/values, changing
// only opaque ID labels. Every added label/digest example is in the ledger.
fn assert_distinct_station_id_pair(first: &str, last: &str) {
    for &(kind, tip, mirrored) in CASES {
        let mut d = design(kind, tip, mirrored);
        let stations = &mut surface_mut(&mut d).stations;
        stations[0].id = first.into();
        stations[1].id = last.into();
        let mut records: Vec<_> = expected("surface-a", tip)
            .into_iter()
            .map(|(path, value, unit)| {
                let path = path
                    .replace(".stations[tip]", &format!(".stations[{first}]"))
                    .replace(".stations[root]", &format!(".stations[{last}]"));
                (path, value, unit)
            })
            .collect();
        records.sort_by(|a, b| a.0.cmp(&b.0));
        d.provenance = records
            .iter()
            .map(|(path, value, unit)| (path.clone(), metadata(path, *value, unit)))
            .collect();
        d.validate().unwrap();
        assert_eq!(endpoint_ids(&d), (first, last));
        let actual: Vec<_> = d
            .numbers()
            .into_iter()
            .map(|n| (n.path, n.value, n.unit))
            .collect();
        assert_eq!(actual, records);
        assert_ne!(
            d.provenance[&format!("components[surface-a].surface.stations[{first}].chord_m")]
                .source_refs,
            d.provenance[&format!("components[surface-a].surface.stations[{last}].chord_m")]
                .source_refs
        );
    }
}

#[test]
fn fix10_uppercase_a_and_lowercase_a_are_distinct_station_ids() {
    assert_distinct_station_id_pair("A", "a");
}

#[test]
fn fix10_ab_and_ba_remain_distinct_despite_byte_sum_collision() {
    assert_distinct_station_id_pair("ab", "ba");
}

#[test]
fn fix10_ac_and_ca_remain_distinct_despite_byte_sum_collision() {
    assert_distinct_station_id_pair("ac", "ca");
}

#[test]
fn fix10_abc_and_cba_remain_distinct_despite_byte_sum_collision() {
    assert_distinct_station_id_pair("abc", "cba");
}

#[test]
fn fix10_aa_and_az_remain_distinct_despite_first_character_collision() {
    assert_distinct_station_id_pair("aa", "az");
}

#[test]
fn fix10_pa_and_pb_remain_distinct_despite_first_character_collision() {
    assert_distinct_station_id_pair("pa", "pb");
}
