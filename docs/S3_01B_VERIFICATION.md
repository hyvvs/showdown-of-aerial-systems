# S3-01b verification findings

## Separate rudder fixture correction

Source: owner's endpoint-order clarification and narrow inventory exception
in this conversation. D6 in CONVENTIONS, Forces and moments, defines physical
deflection signs independently of the order in which endpoints are stored.
`physical_control_sign_is_independent_of_hinge_endpoint_order` passed before
the fixture edit, including the unchanged rudder expectation, "trailing edge
left; nose-left yaw moment". This verifies the descriptive sign contract;
S3-01 does not yet turn a designer hinge into aerodynamic loads.

Before edit: `cargo test --all`: **198 passed, 0 failed, 0 ignored**.
After edit: the same command: **198 passed, 0 failed, 0 ignored**.
TEMP and TMP were checkout-local `target/s3-01b/tmp` for both runs. Raw logs
are `target/s3-01b/fix1-all-tests.txt` and
`target/s3-01b/rudder-after-full-suite.txt` (ignored local evidence).
The existing incremental-cache hard-link warning caused Rust's automatic
copy fallback; no cache, profile or test settings were changed.

Only the rudder fixture endpoints and their independent expected inventory
were swapped. A scratch before/after inventory comparison found 161 records:
155 identical and exactly these six coordinate changes, all in metres:

| Path below `controls[rudder]` | Before | After |
|---|---:|---:|
| `hinge_start_body_m[0]` | -1 | -2 |
| `hinge_start_body_m[1]` | 0 | 3 |
| `hinge_start_body_m[2]` | 2 | 4 |
| `hinge_end_body_m[0]` | -2 | -1 |
| `hinge_end_body_m[1]` | 3 | 0 |
| `hinge_end_body_m[2]` | 4 | 2 |

These are existing labelled synthetic values, not new aircraft data. The
parent-span dot product changes from -5 to +5 under the owner's rule. See
`semantic_values.md` and `verification_values.md` in the designer test fixtures.

Exact diff **inside `support::expected_numbers`** (no other changes there):

```diff
         ] {
+            let point = match (id, name) {
+                ("rudder", "hinge_start_body_m") => [-2.0, 3.0, 4.0],
+                ("rudder", "hinge_end_body_m") => [-1.0, 0.0, 2.0],
+                _ => point,
+            };
             for (i, value) in point.iter().enumerate() {
```

No assertion, physical-sign expectation or tolerance was changed. The
original `semantic.rs` and Fix 1 `quantity_identity.rs` were byte unchanged
relative to the Fix 1 implementation commit.

### Tests using the rudder entries at the correction checkpoint

The inventory test directly compares them; the sign-independence test reads
and reverses them. The following exhaustive list also includes every shared
fixture consumer, even tests which exit validation before reaching a control:

`semantic.rs`:

- `independent_component_inventory_and_every_numeric_path_are_preserved`
- `hand_mirrored_points_preserve_forward_and_down_axes_once`
- `engine_placement_is_body_xyz_and_force_direction_not_exhaust`
- `duplicate_id_has_exact_error`
- `missing_or_nonsurface_control_parent_is_rejected`
- `dangling_engine_table_is_rejected`
- `nonpositive_nonfinite_or_unordered_geometry_and_bad_controls_fail_clearly`
- `missing_inertia_and_nonwing_data_are_explicit_not_numeric_zero`
- `tensor_entries_each_have_provenance_and_unknown_metadata_is_rejected`
- `empty_flags_wrong_units_or_false_missing_origins_are_rejected`
- `template_values_are_distinct_from_user_values_and_outside_coverage_is_retained`
- `static_wing_overrides_and_opaque_aircraft_totals_are_rejected`
- `synthetic_template_cannot_imply_a_real_aircraft_identity`
- `malformed_engine_unit_vector_and_inertia_are_not_repaired`

`quantity_identity.rs`:

- `original_casing_and_separator_bypasses_are_rejected`
- `each_computable_quantity_is_rejected_in_each_scope_origin_and_spelling`
- `physical_control_sign_is_independent_of_hinge_endpoint_order`
- `typed_computed_identity_cannot_hide_behind_an_arbitrary_display_label`

Later follow-up test files also clone this fixture; their names and coverage
are recorded below as they are added. No unrelated expected values changed.

## Fix 2: invalid inputs and actual-parent ordering

Tests were committed first as `27ab527`. Original-code runtime summary:

```text
reversed_and_zero_length_controls_have_specific_errors_for_every_kind ... FAILED
  left: Ok(())
 right: Err(DesignError { field: "controls[elevator].hinge",
   reason: "hinge endpoints must progress root to tip along parent span" })
actual_parent_span_supports_ventral_inverted_v_and_canted_controls ... FAILED
  Err(DesignError { field: "components[fin].surface.stations[1]",
    reason: "stations must progress outboard" })
test result: FAILED. 10 passed; 2 failed; 0 ignored
```

The existing ten rejection tests passed immediately; those guards were
already correct. The production change adds actual-parent hinge ordering
and permits vertical stations progressing along their actual declared span,
including a ventral fin. Horizontal surface station ordering is unchanged.
No aircraft dynamics, physical sign or original expectation is altered.

All twelve tests in `validation_regressions.rs` use the shared fixture and
therefore join the rudder-entry consumer list above:

| Test | Independent expected check / how a fault fails |
|---|---|
| `blank_supplied_sources_and_numeric_source_explanations_have_specific_errors` | Empty/whitespace contribution, section, engine-table and every numeric source explanation must report the exact source field/error. |
| `computed_provenance_is_rejected_on_every_supplied_and_missing_input` | Independently authored inventory paths reject wing-only and derived origins for supplied or absent inputs. |
| `nonfinite_surface_coordinates_dimensions_and_section_inputs_are_rejected` | NaN and both infinities at every present station coordinate/chord/section slope have explicit exact errors. |
| `nonfinite_fuselage_coordinates_and_dimensions_are_rejected` | Nose, length and every sampled section coordinate/dimension reject the same probes. |
| `nonfinite_mass_cg_and_every_intrinsic_tensor_entry_are_rejected` | Every component's mass, CG coordinate and tensor entry is invalidated separately; populated probe masses copy only ledgered synthetic data. |
| `every_angle_field_rejects_nonfinite_at_the_typed_constructor` | Independent angle-path list covers incidence, twist, section, controls and all component tensor axes; safe Radians construction rejects non-finite values with NonFiniteAngle. |
| `nonfinite_engine_placement_table_axes_and_every_sample_are_rejected` | Engine xyz position/direction, dry limit, all table axes and each thrust/fuel sample have exact errors. |
| `nonfinite_control_and_supplied_parameter_scalars_are_rejected` | Both hinge vectors, fractions and supplied scalar values reject each invalid probe. |
| `symmetric_non_positive_definite_inertia_is_rejected_separately_from_asymmetry` | Negative final diagonal in an otherwise symmetric tensor is invalid; the original asymmetry test stays unchanged. |
| `empty_and_unordered_stations_have_specific_errors_for_every_surface_kind` | Wing, horizontal tail and vertical tail each reject empty stations and an out-and-back interior station with exact errors. |
| `reversed_and_zero_length_controls_have_specific_errors_for_every_kind` | Each control kind separately rejects reversed and coincident endpoints with distinct exact errors. |
| `actual_parent_span_supports_ventral_inverted_v_and_canted_controls` | Hand dot products for three nonstandard orientations; positive cases pass and their reversals fail the exact order error. |

Green designer summary: **33 passed, 0 failed, 0 ignored** (original 16,
Fix 1 five, Fix 2 twelve). Red/green logs are local
`target/s3-01b/fix2-red.txt` and `fix2-green.txt`. New synthetic probes and
the owner-specified projection equation are ledgered in
`tests/fixtures/verification_values.md`; no new physical formula or tolerance.

### Fix 2 scratch mutations

Each mutation ran against the unchanged new regression tests in an ignored
copy containing designer/core/estimator sources, with checkout-local TEMP/TMP
and separate build storage. Every final mutant compiled and exited 101 with:

```text
test result: FAILED. 0 passed; 1 failed; 0 ignored; 11 filtered out
```

| Fault (local log basename under `target/s3-01b/fix2-mutations`) | Test above which fails |
|---|---|
| `blank-source`, `blank-metadata-source`: remove text / numeric-source guards | blank source test |
| `finite-coordinate`: bypass coordinate finite check | nonfinite surface test |
| `positive-infinity`: allow positive infinite dimensions | nonfinite fuselage test |
| `engine-position-finite`: bypass core finite-vector check | nonfinite engine test |
| `engine-sample-finite`: bypass sample finite/nonnegative check | nonfinite engine test |
| `present-provenance`, `absent-provenance`: bypass origin guard separately | computed provenance test |
| `minimum-stations`: bypass minimum station count | empty/unordered station test (missing root produces a panic instead of the specified error) |
| `positive-definite`: retain symmetry/finite checks but bypass Cholesky check | symmetric indefinite tensor test |
| `unordered-Wing`, `unordered-HorizontalTail`, `unordered-VerticalTail`: bypass each kind's station order separately | empty/unordered station test |
| `reversed-hinge`: bypass positive projection | reversed/zero hinge test |
| `fixed-axis-hinge`: add a fixed negative-z requirement | actual-parent ventral/inverted/canted test |
| `zero-hinge`: bypass distinct-endpoint guard | reversed/zero hinge test (gets order error instead of distinct-endpoint error) |
| `angle-constructor`: allow non-finite Radians construction | every angle field test |

The first hinge-bypass scratch edit lost array-size inference and failed to
compile; that is not counted as evidence. It was corrected only in scratch
to retain the typed projection expression, then the assertion failed as
required. The fixed-axis mutant likewise retains type inference. No mutant,
scratch helper or changed test expectation was committed.

Fix 2 gates: formatting, Clippy with warnings denied, and the full workspace
suite pass: **210 passed, 0 failed, 0 ignored**. A collapsible nested `if`
was simplified after Clippy's first rejection; no assertion or tolerance
changed. Full-suite log: `target/s3-01b/fix2-all-tests.txt`; compiler copy
fallback warnings remain disclosed. No dependency, profile or cache setting
was changed. Source: the gate outputs and owner-specified scope.
