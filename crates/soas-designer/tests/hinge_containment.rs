//! Independent SYNTHETIC geometry, never the shared aircraft fixture.
//! Every value/setting: fixtures/closed_kind_values.md, Fix 7.
use soas_core::units::Radians;
use soas_designer::*;
use std::collections::BTreeMap;

fn along(tip: [f64; 3], fraction: f64) -> [f64; 3] {
    tip.map(|v| v * fraction)
}
fn own_design(kind: SurfaceKind, tip: [f64; 3], start: [f64; 3], end: [f64; 3]) -> Design {
    let zero = Radians::new(0.0).unwrap();
    let mut d = Design {
        name: "Independent synthetic span test".into(),
        identity: DesignIdentity::UserDesign,
        real_aircraft_claim: None,
        datum: "synthetic body origin at root".into(),
        components: vec![Component {
            id: "parent".into(),
            name: "independent surface".into(),
            geometry: ComponentGeometry::Surface(Surface {
                kind,
                mirrored: false,
                incidence_rad: zero,
                stations: vec![
                    Station {
                        leading_edge_body_m: [0.0; 3],
                        chord_m: 2.0,
                        twist_rad: zero,
                        airfoil: None,
                    },
                    Station {
                        leading_edge_body_m: tip,
                        chord_m: 2.0,
                        twist_rad: zero,
                        airfoil: None,
                    },
                ],
            }),
            mass: MassInput::default(),
        }],
        engine_tables: vec![],
        contributions: vec![],
        controls: vec![ControlSurface {
            id: "control".into(),
            parent_surface_id: "parent".into(),
            kind: ControlKind::Aileron,
            hinge_start_body_m: start,
            hinge_end_body_m: end,
            chord_fraction: 0.2,
            min_rad: Radians::new(-0.3).unwrap(),
            max_rad: Radians::new(0.4).unwrap(),
        }],
        provenance: BTreeMap::new(),
    };
    // Metadata construction does not calculate an expected geometric answer.
    d.provenance = d
        .numbers()
        .into_iter()
        .map(|n| {
            let flags = vec![if n.value.is_some() {
                ProvenanceFlag::UserSupplied
            } else {
                ProvenanceFlag::NotEstimated
            }];
            (
                n.path,
                NumberProvenance {
                    flags,
                    unit: n.unit.into(),
                    source_refs: vec!["SYNTHETIC TEST: closed_kind_values.md, Fix 7".into()],
                    input_paths: vec![],
                    geometry_revision: None,
                    approximate: false,
                },
            )
        })
        .collect();
    d
}
const PARENTS: &[(SurfaceKind, [f64; 3])] = &[
    (SurfaceKind::Wing, [0.0, 8.0, 0.0]),
    (SurfaceKind::VerticalTail, [0.0, 0.0, -8.0]),
    (SurfaceKind::VerticalTail, [0.0, 0.0, 8.0]),
    (SurfaceKind::HorizontalTail, [0.0, 6.0, 8.0]),
    (SurfaceKind::Wing, [6.0, 8.0, 0.0]),
];
fn rejects(d: &Design, endpoint: &str) {
    assert_eq!(
        d.validate(),
        Err(DesignError {
            field: format!("controls[control].{endpoint}"),
            reason: "hinge endpoint must lie within parent span",
        })
    );
}

#[test]
fn inside_hinges_are_accepted_in_actual_parent_span_directions() {
    // Fractions 1/4 and 3/4 give projections 16/48 for squared span 64,
    // and 25/75 for squared span 100: strictly inside, by hand.
    for &(kind, tip) in PARENTS {
        own_design(kind, tip, along(tip, 0.25), along(tip, 0.75))
            .validate()
            .unwrap();
    }
}
#[test]
fn root_and_tip_boundaries_are_accepted_on_independent_geometry() {
    for &(kind, tip) in PARENTS {
        own_design(kind, tip, [0.0; 3], tip).validate().unwrap();
    }
}
#[test]
fn slightly_outside_both_boundaries_is_rejected_on_independent_geometry() {
    // Tiny relative to geometry, yet much larger than basic-operation
    // roundoff. This is a labelled probe offset, not an acceptance tolerance.
    let delta = f64::EPSILON.sqrt();
    for &(kind, tip) in PARENTS {
        rejects(
            &own_design(kind, tip, along(tip, -delta), along(tip, 0.75)),
            "hinge_start_body_m",
        );
        rejects(
            &own_design(kind, tip, along(tip, 0.25), along(tip, 1.0 + delta)),
            "hinge_end_body_m",
        );
    }
}
#[test]
fn either_partly_or_fully_outside_hinges_are_rejected() {
    for &(kind, tip) in PARENTS {
        for (a, b, endpoint) in [
            (-0.25, 0.75, "hinge_start_body_m"),
            (0.25, 1.25, "hinge_end_body_m"),
            (-0.5, -0.25, "hinge_start_body_m"),
            (1.25, 1.5, "hinge_start_body_m"),
        ] {
            rejects(
                &own_design(kind, tip, along(tip, a), along(tip, b)),
                endpoint,
            );
        }
    }
}
#[test]
fn span_check_scales_with_geometry_instead_of_a_fixed_distance_tolerance() {
    for scale in [1.0 / 1024.0, 1.0, 1024.0] {
        let tip = [0.0, 8.0 * scale, 0.0];
        own_design(SurfaceKind::Wing, tip, [0.0; 3], tip)
            .validate()
            .unwrap();
        rejects(
            &own_design(
                SurfaceKind::Wing,
                tip,
                along(tip, 0.25),
                along(tip, 1.0 + f64::EPSILON.sqrt()),
            ),
            "hinge_end_body_m",
        );
    }
}
#[test]
fn chordwise_position_is_explicitly_not_checked() {
    // Body x/z far from the chord still lies inside the y-directed span slab.
    own_design(
        SurfaceKind::Wing,
        [0.0, 8.0, 0.0],
        [16.0, 2.0, 32.0],
        [-16.0, 6.0, -32.0],
    )
    .validate()
    .unwrap();
}
