# S3-01 numeric ledger — SYNTHETIC TEST VALUES ONLY

No number represents any real aircraft, airfoil, loading or engine. `support::expected_numbers`
is the per-path inventory; its explicit paths and the tables below jointly label EVERY
new numeric value. The fixture's provenance is populated from that independently authored
inventory, not from the production numeric walker. Indices/counts are structural test
settings, not measured data. All comparisons are exact; no new tolerance is selected.

| Numeric path / setting | Synthetic value(s), meaning |
|---|---|
| components[wing].surface.stations[*].leading_edge_body_m | root [1,0,3], right tip [-2,4,5] m, deliberately distinct axes; hand left tip [-2,-4,5] |
| components[tail].surface.stations[*].leading_edge_body_m | root [-4,0,2], tip [-5,2,3] m |
| components[fin].surface.stations[*].leading_edge_body_m | root [-6,0,2], tip [-7,0,-1] m; vertical span extends up, negative body z |
| each surface.incidence_rad | 0.02 rad |
| each surface.stations[*].chord_m / twist_rad | root 2 m / 0 rad; tip 1 m / -0.03 rad |
| each root.airfoil.lift_curve_slope_per_rad / zero_lift_angle_rad | 6 per rad / -0.1 rad, arbitrary labeled section TEST VALUES, not empirical airfoil data or a default |
| components[body].fuselage.nose_body_m / length_m | [10,11,12] m / 13 m |
| body sections[0]: aft, lateral offset, down offset, width, height | [0,0,0,2,3] m |
| body sections[1]: same quantities | [13,1,-2,1,2] m |
| components[engine].engine.position_body_m / direction_body_unit | [7,8,9] m / [0,0.6,0.8], force-on-aircraft unit vector, hand expectation unchanged |
| components[wing].mass.mass_kg / cg_body_m | 5 kg / [1,2,3] m |
| wing.mass.intrinsic_inertia.tensor_kg_m2 | [[4,-0.1,0.2],[-0.1,5,-0.3],[0.2,-0.3,6]] kg m², symmetric positive definite test matrix |
| wing.mass.intrinsic_inertia.axes_euler_rad | [0.01,0.02,0.03] rad, orientation input only, not transformed in S3-01 |
| all other mass/CG/inertia fields | absent, not numeric zero; NOT_ESTIMATED |
| controls[elevator/aileron/rudder].hinge_start/end_body_m | [-1,0,2] / [-2,3,4] m |
| each control.chord_fraction / min_rad / max_rad | 0.2 / -0.3 rad / 0.4 rad |
| contributions[tail-law].parameters[cm_alpha_per_rad] | -0.7 per rad, declared synthetic tail law only |
| contributions[lift-limit].parameters[cl_max] | 1.4, synthetic supplied whole-aircraft lift cap, never a wing estimate |
| engine_tables[jet].dry_throttle_max / table.* | Reused exactly from soas-core/tests/fixtures/synthetic_jet.json, with its existing synthetic_jet_values.md per-value ledger; no new engine data |
| invalid geometry probes | chord 0, -1, NaN, +infinity; zero tip y; fuselage width -1 m; control max -0.4 rad |
| invalid numeric/provenance probes | orphan numeric value 1; nonsymmetric tensor [0][1]=0; engine direction [0,0,0] |
| index/test settings | component slots 0..4 = wing/tail/fin/body/engine; station/row/column/flag slots 0..2 address explicitly inventoried entries; one copied engine at index 0; no numerical perturbation step or tolerance |

Only formulas in this task are coordinate reflection `(x,y,z)->(x,-y,z)`
and preservation of body force direction: CONVENTIONS, Coordinate frames/Forces
and moments; data/reference/estimator/geometry.md, Geometry contract and formulas;
core propulsion Engine contract. Tensor and engine numeric validation reuse the
existing core rules, including its unchanged unit-vector roundoff bound. No
mass buildup, inertia shift/rotation, aerodynamic estimation or JSON design format
is implemented by this task.
