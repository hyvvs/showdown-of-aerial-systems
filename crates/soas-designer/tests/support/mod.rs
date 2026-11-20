//! SYNTHETIC TEST VALUES only. Every new literal is inventoried in
//! ../fixtures/semantic_values.md. No real aircraft or section is represented.
use soas_core::{aircraft, units::Radians};
use soas_designer::*;
use std::collections::BTreeMap;

pub fn rad(value: f64) -> Radians {
    Radians::new(value).unwrap()
}

pub fn fixture() -> Design {
    let surface = |kind, root, tip, mirrored| {
        ComponentGeometry::Surface(Surface {
            kind,
            mirrored,
            incidence_rad: rad(0.02),
            stations: vec![
                Station {
                    leading_edge_body_m: root,
                    chord_m: 2.0,
                    twist_rad: rad(0.0),
                    airfoil: Some(AirfoilInput {
                        name: "synthetic section".into(),
                        source: "SYNTHETIC TEST VALUE: semantic_values.md".into(),
                        lift_curve_slope_per_rad: Some(6.0),
                        zero_lift_angle_rad: Some(rad(-0.1)),
                    }),
                },
                Station {
                    leading_edge_body_m: tip,
                    chord_m: 1.0,
                    twist_rad: rad(-0.03),
                    airfoil: None,
                },
            ],
        })
    };
    let component = |id: &str, geometry| Component {
        id: id.into(),
        name: id.into(),
        geometry,
        mass: MassInput::default(),
    };
    // Reuse the existing labeled synthetic engine table, not an invented engine.
    let jet = aircraft::from_json(include_str!(
        "../../../soas-core/tests/fixtures/synthetic_jet.json"
    ))
    .unwrap()
    .engines
    .remove(0);
    let mut design = Design {
        name: "Synthetic aircraft study".into(),
        identity: DesignIdentity::SyntheticTemplate,
        real_aircraft_claim: None,
        datum: "synthetic common body datum".into(),
        components: vec![
            component(
                "wing",
                surface(SurfaceKind::Wing, [1.0, 0.0, 3.0], [-2.0, 4.0, 5.0], true),
            ),
            component(
                "tail",
                surface(
                    SurfaceKind::HorizontalTail,
                    [-4.0, 0.0, 2.0],
                    [-5.0, 2.0, 3.0],
                    true,
                ),
            ),
            component(
                "fin",
                surface(
                    SurfaceKind::VerticalTail,
                    [-6.0, 0.0, 2.0],
                    [-7.0, 0.0, -1.0],
                    false,
                ),
            ),
            component(
                "body",
                ComponentGeometry::Fuselage(Fuselage {
                    nose_body_m: [10.0, 11.0, 12.0],
                    length_m: 13.0,
                    sections: vec![
                        BodySection {
                            aft_m: 0.0,
                            offset_y_m: 0.0,
                            offset_z_m: 0.0,
                            width_m: 2.0,
                            height_m: 3.0,
                        },
                        BodySection {
                            aft_m: 13.0,
                            offset_y_m: 1.0,
                            offset_z_m: -2.0,
                            width_m: 1.0,
                            height_m: 2.0,
                        },
                    ],
                }),
            ),
            component(
                "engine",
                ComponentGeometry::Engine(EngineInstance {
                    table_id: "jet".into(),
                    position_body_m: [7.0, 8.0, 9.0],
                    direction_body_unit: [0.0, 0.6, 0.8],
                }),
            ),
        ],
        engine_tables: vec![EngineTableDefinition {
            id: "jet".into(),
            source: jet.data_note,
            dry_throttle_max: jet.dry_throttle_max,
            table: jet.table,
        }],
        controls: vec![
            control("elevator", "tail", ControlKind::Elevator),
            control("aileron", "wing", ControlKind::Aileron),
            control("rudder", "fin", ControlKind::Rudder),
        ],
        contributions: vec![
            SuppliedContribution {
                id: "tail-law".into(),
                scope: ContributionScope::Tail,
                source: "synthetic test law".into(),
                coverage: "synthetic declared condition only".into(),
                parameters: vec![SuppliedParameter {
                    name: "cm_alpha_per_rad".into(),
                    value: Some(-0.7),
                    unit: "1/rad".into(),
                }],
            },
            SuppliedContribution {
                id: "lift-limit".into(),
                scope: ContributionScope::ClMax,
                source: "synthetic test limit".into(),
                coverage: "synthetic declared condition only".into(),
                parameters: vec![SuppliedParameter {
                    name: "cl_max".into(),
                    value: Some(1.4),
                    unit: "1".into(),
                }],
            },
        ],
        provenance: BTreeMap::new(),
    };
    design.components[0].mass = MassInput {
        mass_kg: Some(5.0),
        cg_body_m: Some([1.0, 2.0, 3.0]),
        intrinsic_inertia: Some(IntrinsicInertia {
            tensor_kg_m2: [[4.0, -0.1, 0.2], [-0.1, 5.0, -0.3], [0.2, -0.3, 6.0]],
            axes_euler_rad: [rad(0.01), rad(0.02), rad(0.03)],
        }),
    };
    // Metadata is authored from a separate fixture inventory, NEVER numbers().
    for (path, value, unit) in expected_numbers(&design) {
        design.provenance.insert(path, metadata(value, unit));
    }
    design
}

fn control(id: &str, parent: &str, kind: ControlKind) -> ControlSurface {
    ControlSurface {
        id: id.into(),
        parent_surface_id: parent.into(),
        kind,
        hinge_start_body_m: [-1.0, 0.0, 2.0],
        hinge_end_body_m: [-2.0, 3.0, 4.0],
        chord_fraction: 0.2,
        min_rad: rad(-0.3),
        max_rad: rad(0.4),
    }
}

pub fn metadata(value: Option<f64>, unit: &str) -> NumberProvenance {
    NumberProvenance {
        flags: vec![if value.is_some() {
            ProvenanceFlag::UserSupplied
        } else {
            ProvenanceFlag::NotEstimated
        }],
        unit: unit.into(),
        source_refs: vec!["SYNTHETIC TEST VALUE: semantic_values.md".into()],
        input_paths: vec![],
        geometry_revision: None,
        approximate: false,
    }
}

/// Independently authored inventory of the fixture above. Missing tuples denote
/// an absent value, not a numeric zero. Core engine JSON is walked generically.
pub fn expected_numbers(_design: &Design) -> Vec<(String, Option<f64>, &'static str)> {
    let mut out = vec![];
    let mut push = |path: String, value, unit| out.push((path, value, unit));
    for (id, root, tip) in [
        ("wing", [1.0, 0.0, 3.0], [-2.0, 4.0, 5.0]),
        ("tail", [-4.0, 0.0, 2.0], [-5.0, 2.0, 3.0]),
        ("fin", [-6.0, 0.0, 2.0], [-7.0, 0.0, -1.0]),
    ] {
        let p = format!("components[{id}].surface");
        push(format!("{p}.incidence_rad"), Some(0.02), "rad");
        for (i, point) in [root, tip].iter().enumerate() {
            let s = format!("{p}.stations[{i}]");
            for (axis, value) in point.iter().enumerate() {
                push(
                    format!("{s}.leading_edge_body_m[{axis}]"),
                    Some(*value),
                    "m",
                );
            }
            push(
                format!("{s}.chord_m"),
                Some(if i == 0 { 2.0 } else { 1.0 }),
                "m",
            );
            push(
                format!("{s}.twist_rad"),
                Some(if i == 0 { 0.0 } else { -0.03 }),
                "rad",
            );
            if i == 0 {
                push(
                    format!("{s}.airfoil.lift_curve_slope_per_rad"),
                    Some(6.0),
                    "1/rad",
                );
                push(
                    format!("{s}.airfoil.zero_lift_angle_rad"),
                    Some(-0.1),
                    "rad",
                );
            }
        }
    }
    for (i, value) in [10.0, 11.0, 12.0].iter().enumerate() {
        push(
            format!("components[body].fuselage.nose_body_m[{i}]"),
            Some(*value),
            "m",
        );
    }
    push("components[body].fuselage.length_m".into(), Some(13.0), "m");
    for (i, values) in [[0.0, 0.0, 0.0, 2.0, 3.0], [13.0, 1.0, -2.0, 1.0, 2.0]]
        .iter()
        .enumerate()
    {
        for (name, value) in ["aft_m", "offset_y_m", "offset_z_m", "width_m", "height_m"]
            .iter()
            .zip(values)
        {
            push(
                format!("components[body].fuselage.sections[{i}].{name}"),
                Some(*value),
                "m",
            );
        }
    }
    for (name, values, unit) in [
        ("position_body_m", [7.0, 8.0, 9.0], "m"),
        ("direction_body_unit", [0.0, 0.6, 0.8], "1"),
    ] {
        for (i, value) in values.iter().enumerate() {
            push(
                format!("components[engine].engine.{name}[{i}]"),
                Some(*value),
                unit,
            );
        }
    }
    for id in ["wing", "tail", "fin", "body", "engine"] {
        let p = format!("components[{id}].mass");
        push(
            format!("{p}.mass_kg"),
            if id == "wing" { Some(5.0) } else { None },
            "kg",
        );
        if id != "wing" {
            push(format!("{p}.cg_body_m"), None, "m");
            push(format!("{p}.intrinsic_inertia"), None, "kg*m^2");
            continue;
        }
        for (i, value) in [1.0, 2.0, 3.0].iter().enumerate() {
            push(format!("{p}.cg_body_m[{i}]"), Some(*value), "m");
        }
        for (row, values) in [[4.0, -0.1, 0.2], [-0.1, 5.0, -0.3], [0.2, -0.3, 6.0]]
            .iter()
            .enumerate()
        {
            for (col, value) in values.iter().enumerate() {
                push(
                    format!("{p}.intrinsic_inertia.tensor_kg_m2[{row}][{col}]"),
                    Some(*value),
                    "kg*m^2",
                );
            }
        }
        for (i, value) in [0.01, 0.02, 0.03].iter().enumerate() {
            push(
                format!("{p}.intrinsic_inertia.axes_euler_rad[{i}]"),
                Some(*value),
                "rad",
            );
        }
    }
    for id in ["elevator", "aileron", "rudder"] {
        for (name, point) in [
            ("hinge_start_body_m", [-1.0, 0.0, 2.0]),
            ("hinge_end_body_m", [-2.0, 3.0, 4.0]),
        ] {
            for (i, value) in point.iter().enumerate() {
                push(format!("controls[{id}].{name}[{i}]"), Some(*value), "m");
            }
        }
        for (name, value, unit) in [
            ("chord_fraction", 0.2, "1"),
            ("min_rad", -0.3, "rad"),
            ("max_rad", 0.4, "rad"),
        ] {
            push(format!("controls[{id}].{name}"), Some(value), unit);
        }
    }
    push(
        "contributions[tail-law].parameters[cm_alpha_per_rad]".into(),
        Some(-0.7),
        "1/rad",
    );
    push(
        "contributions[lift-limit].parameters[cl_max]".into(),
        Some(1.4),
        "1",
    );
    // Reused engine table: expected numbers discovered from the separately
    // authored JSON, not the semantic model's own numeric walker.
    let original: serde_json::Value = serde_json::from_str(include_str!(
        "../../../soas-core/tests/fixtures/synthetic_jet.json"
    ))
    .unwrap();
    fn walk(
        value: &serde_json::Value,
        path: &str,
        out: &mut Vec<(String, Option<f64>, &'static str)>,
    ) {
        match value {
            serde_json::Value::Number(n) => {
                let unit = if path.ends_with("thrust_n") {
                    "N"
                } else if path.ends_with("fuel_flow_kgps") {
                    "kg/s"
                } else if path.contains("geometric_altitude_nodes_m") {
                    "m"
                } else {
                    "1"
                };
                out.push((path.into(), n.as_f64(), unit));
            }
            serde_json::Value::Array(a) => {
                for (i, v) in a.iter().enumerate() {
                    walk(v, &format!("{path}[{i}]"), out);
                }
            }
            serde_json::Value::Object(o) => {
                for (k, v) in o {
                    walk(v, &format!("{path}.{k}"), out);
                }
            }
            _ => (),
        }
    }
    walk(
        &original["engines"][0]["table"],
        "engine_tables[jet].table",
        &mut out,
    );
    out.push((
        "engine_tables[jet].dry_throttle_max".into(),
        original["engines"][0]["dry_throttle_max"].as_f64(),
        "1",
    ));
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}
