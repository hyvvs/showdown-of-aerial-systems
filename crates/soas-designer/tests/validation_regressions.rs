//! SYNTHETIC TEST VALUES/settings: fixtures/verification_values.md.
mod support;
use soas_core::units::{NonFiniteAngle, Radians};
use soas_designer::*;
use support::fixture;

fn rejects(d: &Design, field: &str, reason: &'static str) {
    assert_eq!(
        d.validate(),
        Err(DesignError {
            field: field.into(),
            reason
        })
    );
}
fn surface(d: &mut Design, i: usize) -> &mut Surface {
    let ComponentGeometry::Surface(s) = &mut d.components[i].geometry else {
        panic!()
    };
    s
}
fn body(d: &mut Design) -> &mut Fuselage {
    let ComponentGeometry::Fuselage(f) = &mut d.components[3].geometry else {
        panic!()
    };
    f
}
fn engine(d: &mut Design) -> &mut EngineInstance {
    let ComponentGeometry::Engine(e) = &mut d.components[4].geometry else {
        panic!()
    };
    e
}
fn refresh_provenance(d: &mut Design) {
    d.provenance = d
        .numbers()
        .into_iter()
        .map(|n| (n.path, support::metadata(n.value, n.unit)))
        .collect();
}
fn probe(d: &Design, set: impl Fn(&mut Design, f64), field: &str, reason: &'static str) {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut bad = d.clone();
        set(&mut bad, invalid);
        rejects(&bad, field, reason);
    }
}

#[test]
fn blank_supplied_sources_and_numeric_source_explanations_have_specific_errors() {
    for blank in ["", " \t\n "] {
        let mut d = fixture();
        d.engine_tables[0].source = blank.into();
        rejects(&d, "engine_tables[jet].source", "must not be empty");
        for (i, id) in ["wing", "tail", "fin"].into_iter().enumerate() {
            let mut d = fixture();
            surface(&mut d, i).stations[0]
                .airfoil
                .as_mut()
                .unwrap()
                .source = blank.into();
            rejects(
                &d,
                &format!("components[{id}].surface.stations[0].airfoil.source"),
                "must not be empty",
            );
        }
        for i in 0..fixture().contributions.len() {
            let mut d = fixture();
            d.contributions[i].source = blank.into();
            rejects(
                &d,
                &format!("contributions[{}].source", d.contributions[i].id),
                "must not be empty",
            );
        }
        for (path, _, _) in support::expected_numbers(&fixture()) {
            let mut d = fixture();
            d.provenance.get_mut(&path).unwrap().source_refs = vec![blank.into()];
            rejects(&d, &path, "source or missing-input explanation required");
        }
    }
}

#[test]
fn computed_provenance_is_rejected_on_every_supplied_and_missing_input() {
    for (path, value, _) in support::expected_numbers(&fixture()) {
        for flag in [
            ProvenanceFlag::EstimatedWingOnly,
            ProvenanceFlag::DerivedFromInputs,
        ] {
            let mut d = fixture();
            d.provenance.get_mut(&path).unwrap().flags = vec![flag];
            rejects(
                &d,
                &path,
                if value.is_some() {
                    "present input requires supplied or template provenance"
                } else {
                    "absent input requires NOT_ESTIMATED"
                },
            );
        }
    }
}

#[test]
fn nonfinite_surface_coordinates_dimensions_and_section_inputs_are_rejected() {
    let d = fixture();
    for (i, id) in ["wing", "tail", "fin"].into_iter().enumerate() {
        for j in 0..2 {
            let p = format!("components[{id}].surface.stations[{j}]");
            for axis in 0..3 {
                probe(
                    &d,
                    |d, v| surface(d, i).stations[j].leading_edge_body_m[axis] = v,
                    &format!("{p}.leading_edge_body_m"),
                    "must be finite",
                );
            }
            probe(
                &d,
                |d, v| surface(d, i).stations[j].chord_m = v,
                &format!("{p}.chord_m"),
                "must be finite and positive",
            );
        }
        probe(
            &d,
            |d, v| {
                surface(d, i).stations[0]
                    .airfoil
                    .as_mut()
                    .unwrap()
                    .lift_curve_slope_per_rad = Some(v)
            },
            &format!("components[{id}].surface.stations[0].airfoil.lift_curve_slope_per_rad"),
            "must be finite and positive",
        );
    }
}

#[test]
fn nonfinite_fuselage_coordinates_and_dimensions_are_rejected() {
    let d = fixture();
    let p = "components[body].fuselage";
    for axis in 0..3 {
        probe(
            &d,
            |d, v| body(d).nose_body_m[axis] = v,
            &format!("{p}.nose_body_m"),
            "must be finite",
        );
    }
    probe(
        &d,
        |d, v| body(d).length_m = v,
        &format!("{p}.length_m"),
        "must be finite and positive",
    );
    for j in 0..2 {
        for name in ["aft_m", "offset_y_m", "offset_z_m", "width_m", "height_m"] {
            let positive = matches!(name, "width_m" | "height_m");
            let field = if positive {
                format!("{p}.sections[{j}].{name}")
            } else {
                format!("{p}.sections[{j}]")
            };
            probe(
                &d,
                |d, v| {
                    let s = &mut body(d).sections[j];
                    match name {
                        "aft_m" => s.aft_m = v,
                        "offset_y_m" => s.offset_y_m = v,
                        "offset_z_m" => s.offset_z_m = v,
                        "width_m" => s.width_m = v,
                        "height_m" => s.height_m = v,
                        _ => unreachable!(),
                    }
                },
                &field,
                if positive {
                    "must be finite and positive"
                } else {
                    "must be finite"
                },
            );
        }
    }
}

#[test]
fn nonfinite_mass_cg_and_every_intrinsic_tensor_entry_are_rejected() {
    let mut d = fixture();
    let complete = d.components[0].mass.clone();
    for c in &mut d.components {
        c.mass = complete.clone();
    }
    refresh_provenance(&mut d);
    d.validate().unwrap();
    for (i, id) in ["wing", "tail", "fin", "body", "engine"]
        .into_iter()
        .enumerate()
    {
        let p = format!("components[{id}].mass");
        probe(
            &d,
            |d, v| d.components[i].mass.mass_kg = Some(v),
            &format!("{p}.mass_kg"),
            "must be finite and positive",
        );
        for axis in 0..3 {
            probe(
                &d,
                |d, v| d.components[i].mass.cg_body_m.as_mut().unwrap()[axis] = v,
                &format!("{p}.cg_body_m"),
                "must be finite",
            );
        }
        for row in 0..3 {
            for col in 0..3 {
                probe(
                    &d,
                    |d, v| {
                        d.components[i]
                            .mass
                            .intrinsic_inertia
                            .as_mut()
                            .unwrap()
                            .tensor_kg_m2[row][col] = v
                    },
                    &format!("{p}.intrinsic_inertia"),
                    "invalid signed intrinsic inertia tensor",
                );
            }
        }
    }
}

#[test]
fn every_angle_field_rejects_nonfinite_at_the_typed_constructor() {
    let mut d = fixture();
    let complete = d.components[0].mass.clone();
    for c in &mut d.components {
        c.mass = complete.clone();
    }
    // Radians has no public raw constructor: invalid values cannot enter any
    // angle field. Exercise that exact error for every independently described
    // angle location, including optional section and component inertia angles.
    let mut paths = vec![];
    for id in ["wing", "tail", "fin"] {
        paths.push(format!("components[{id}].surface.incidence_rad"));
        for j in 0..2 {
            paths.push(format!("components[{id}].surface.stations[{j}].twist_rad"));
        }
        paths.push(format!(
            "components[{id}].surface.stations[0].airfoil.zero_lift_angle_rad"
        ));
    }
    for id in ["wing", "tail", "fin", "body", "engine"] {
        for axis in 0..3 {
            paths.push(format!(
                "components[{id}].mass.intrinsic_inertia.axes_euler_rad[{axis}]"
            ));
        }
    }
    for id in ["elevator", "aileron", "rudder"] {
        for name in ["min_rad", "max_rad"] {
            paths.push(format!("controls[{id}].{name}"));
        }
    }
    let actual: Vec<_> = d
        .numbers()
        .into_iter()
        .filter(|n| n.unit == "rad")
        .map(|n| n.path)
        .collect();
    paths.sort();
    assert_eq!(actual, paths);
    for path in paths {
        for v in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(Radians::new(v), Err(NonFiniteAngle), "{path}");
        }
    }
}

#[test]
fn nonfinite_engine_placement_table_axes_and_every_sample_are_rejected() {
    let d = fixture();
    for axis in 0..3 {
        probe(
            &d,
            |d, v| engine(d).position_body_m[axis] = v,
            "components[engine].engine.position_body_m",
            "must contain only finite values",
        );
        probe(
            &d,
            |d, v| engine(d).direction_body_unit[axis] = v,
            "components[engine].engine.direction_body_unit",
            "must contain only finite values",
        );
    }
    let p = "engine_tables[jet]";
    probe(
        &d,
        |d, v| d.engine_tables[0].dry_throttle_max = v,
        &format!("{p}.dry_throttle_max"),
        "must be an explicit throttle node strictly between 0 and 1",
    );
    for name in ["mach_nodes", "geometric_altitude_nodes_m", "throttle_nodes"] {
        let table = &d.engine_tables[0].table;
        let count = match name {
            "mach_nodes" => table.mach_nodes.len(),
            "geometric_altitude_nodes_m" => table.geometric_altitude_nodes_m.len(),
            _ => table.throttle_nodes.len(),
        };
        for i in 0..count {
            probe(
                &d,
                |d, v| {
                    let table = &mut d.engine_tables[0].table;
                    match name {
                        "mach_nodes" => table.mach_nodes[i] = v,
                        "geometric_altitude_nodes_m" => table.geometric_altitude_nodes_m[i] = v,
                        _ => table.throttle_nodes[i] = v,
                    }
                },
                &format!("{p}.table.{name}"),
                "must contain only finite values",
            );
        }
    }
    for (h, row) in d.engine_tables[0].table.samples.iter().enumerate() {
        for (m, column) in row.iter().enumerate() {
            for t in 0..column.len() {
                for name in ["thrust_n", "fuel_flow_kgps"] {
                    probe(
                        &d,
                        |d, v| {
                            let s = &mut d.engine_tables[0].table.samples[h][m][t];
                            if name == "thrust_n" {
                                s.thrust_n = v
                            } else {
                                s.fuel_flow_kgps = v
                            }
                        },
                        &format!("{p}.table.samples[{h}][{m}][{t}].{name}"),
                        "must be finite and nonnegative",
                    );
                }
            }
        }
    }
}

#[test]
fn nonfinite_control_and_supplied_parameter_scalars_are_rejected() {
    let d = fixture();
    for (i, id) in ["elevator", "aileron", "rudder"].into_iter().enumerate() {
        for axis in 0..3 {
            for name in ["hinge_start_body_m", "hinge_end_body_m"] {
                probe(
                    &d,
                    |d, v| {
                        let c = &mut d.controls[i];
                        if name == "hinge_start_body_m" {
                            c.hinge_start_body_m[axis] = v
                        } else {
                            c.hinge_end_body_m[axis] = v
                        }
                    },
                    &format!("controls[{id}].{name}"),
                    "must be finite",
                );
            }
        }
        probe(
            &d,
            |d, v| d.controls[i].chord_fraction = v,
            &format!("controls[{id}].chord_fraction"),
            "must be a finite fraction in (0,1]",
        );
    }
    for (i, c) in d.contributions.iter().enumerate() {
        for (j, a) in c.parameters.iter().enumerate() {
            probe(
                &d,
                |d, v| d.contributions[i].parameters[j].value = Some(v),
                &format!("contributions[{}].parameters[{}]", c.id, a.name),
                "must be finite",
            );
        }
    }
}

#[test]
fn symmetric_non_positive_definite_inertia_is_rejected_separately_from_asymmetry() {
    let mut d = fixture();
    d.components[0]
        .mass
        .intrinsic_inertia
        .as_mut()
        .unwrap()
        .tensor_kg_m2[2][2] = -6.0;
    rejects(
        &d,
        "components[wing].mass.intrinsic_inertia",
        "invalid signed intrinsic inertia tensor",
    );
}

#[test]
fn empty_and_unordered_stations_have_specific_errors_for_every_surface_kind() {
    for (i, id) in ["wing", "tail", "fin"].into_iter().enumerate() {
        let mut d = fixture();
        surface(&mut d, i).stations.clear();
        rejects(
            &d,
            &format!("components[{id}].surface"),
            "need at least root and tip stations",
        );
        let mut d = fixture();
        let s = surface(&mut d, i);
        let mut middle = s.stations[1].clone();
        middle.id = "extra".into();
        for axis in 0..3 {
            middle.leading_edge_body_m[axis] +=
                s.stations[1].leading_edge_body_m[axis] - s.stations[0].leading_edge_body_m[axis];
        }
        s.stations.insert(1, middle);
        rejects(
            &d,
            &format!("components[{id}].surface.stations[2]"),
            "stations must progress outboard",
        );
    }
}

#[test]
fn reversed_and_zero_length_controls_have_specific_errors_for_every_kind() {
    for (i, id) in ["elevator", "aileron", "rudder"].into_iter().enumerate() {
        let mut d = fixture();
        let c = &mut d.controls[i];
        std::mem::swap(&mut c.hinge_start_body_m, &mut c.hinge_end_body_m);
        rejects(
            &d,
            &format!("controls[{id}].hinge"),
            "hinge endpoints must progress root to tip along parent span",
        );
        let mut d = fixture();
        d.controls[i].hinge_end_body_m = d.controls[i].hinge_start_body_m;
        rejects(
            &d,
            &format!("controls[{id}].hinge"),
            "hinge endpoints must differ",
        );
    }
}

#[test]
fn actual_parent_span_supports_ventral_inverted_v_and_canted_controls() {
    // Owner-specified dot-product rule, not an axis selected by surface kind.
    // Hand-derived positive dot products: ventral 10, inverted V 14, canted 13.
    for (component, control, tip, end, expected) in [
        (2, 2, [-7.0, 0.0, 5.0], [-7.0, 0.0, 5.0], 10.0),
        (1, 0, [-5.0, 2.0, 5.0], [-5.0, 2.0, 5.0], 14.0),
        (2, 2, [-7.0, 4.0, -1.0], [-6.0, 4.0, 3.0], 13.0),
    ] {
        let mut d = fixture();
        let s = surface(&mut d, component);
        let root = s.stations[0].leading_edge_body_m;
        s.stations[1].leading_edge_body_m = tip;
        let c = &mut d.controls[control];
        c.hinge_start_body_m = root;
        c.hinge_end_body_m = end;
        let dot: f64 = (0..3)
            .map(|i| (end[i] - root[i]) * (tip[i] - root[i]))
            .sum();
        assert_eq!(dot, expected);
        refresh_provenance(&mut d);
        d.validate().unwrap();
        let id = d.controls[control].id.clone();
        let c = &mut d.controls[control];
        std::mem::swap(&mut c.hinge_start_body_m, &mut c.hinge_end_body_m);
        rejects(
            &d,
            &format!("controls[{id}].hinge"),
            "hinge endpoints must progress root to tip along parent span",
        );
    }
}
