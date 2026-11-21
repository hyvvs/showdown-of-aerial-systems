# S3-02-ids: stable station identity only

## 1. What changed

Fresh task/s3-02-ids starts from main 6c63ab2, which contains approved
contract d16745a and the blocker-log commit. git ls-tree on main lists
docs/S3_02_PLAN.md; D15-D22 are present. QUESTIONS marks the blocker RESOLVED.

Station now requires an opaque String ID. The existing identity validator
rejects empty/whitespace/invalid IDs and duplicates within one surface;
another surface can reuse the ID. Station numeric paths contain the surface
ID, station ID and field, including section values. Root/tip stay first/last
positions; no names select geometry. Physical ordering, hinge ordering,
containment, signs and every existing numerical tolerance are unchanged.

Only this model prerequisite is implemented. No reader/writer, parameter
identity migration, grouped-array provenance, snapshot, simulator change,
dependency or other S3-02 child task is started.

## 2. Commits and files

| Commit | Contents |
|---|---|
| c4cb807, Task S3-02-ids: tests | New tests/station_ids.rs and fixtures/station_id_values.md; Station documentation contains the missing-ID compile-fail test. No implementation. |
| 0da98b5, Task S3-02-ids: migrate fixture station IDs | Only ID additions in tests/support/mod.rs, hinge_containment.rs and one cloned-station setup in validation_regressions.rs; corresponding semantic_values.md, closed_kind_values.md and verification_values.md ledger additions. |
| Task S3-02-ids: implementation | src/lib.rs required field; src/numbers.rs ID paths; src/validation.rs surface-local ID guard; docs/QUESTIONS, TASKS, LEARNING, S3_01_SEMANTIC_MODEL, S3_02_PLAN and this report. |

Paths in the first two rows are under crates/soas-designer/. The final
implementation commit follows the separate migration commit, so its complete
tree builds. The intentionally red tests/migration commits need the final
model implementation; no standalone green migration commit is claimed.

## 3. Tests first: actual failing output

Before any fixture migration the unmodified full cargo test --all passed:
**234 passed, 0 failed, 0 ignored**. Raw log:
target/s3-02-ids/before-migration.txt.

The missing-ID compile-fail example initially compiled, proving the model
did not require identity:

```text
test ... Station ... compile fail ... FAILED
Test compiled successfully, but it's marked `compile_fail`.
test result: FAILED. 0 passed; 1 failed; 0 ignored
```

The integration test initially failed to compile against the old model:

```text
error[E0560]: struct `soas_designer::Station` has no field named `id`
  --> crates/soas-designer/tests/station_ids.rs:75:9
error[E0609]: no field `id` on type `&soas_designer::Station`
exit: 101
```

After adding the field and ID paths but before identity guards, the same
tests executed and exposed both missing guards:

```text
test duplicate_station_ids_are_rejected_within_each_surface ... FAILED
test empty_whitespace_and_invalid_station_ids_are_rejected_including_mirrors ... FAILED
test result: FAILED. 6 passed; 2 failed; 0 ignored
```

The duplicate case reported an orphan metadata path instead of the required
stations[1].id / duplicate identity error. The empty case reported missing
station[].airfoil metadata instead of the required stations[0].id identity
error. The tests demand the specific identity rejection, not a later unrelated
provenance error. Logs: red-missing-id.txt, red-station-ids.txt and
red-id-validation.txt under target/s3-02-ids/.

### Independent expectations and why the tests can fail

| New test | Independent expectation / fault caught |
|---|---|
| station_numeric_paths_use_the_declared_id_for_every_surface_and_field | Manually authored path/value/unit inventory for wing, horizontal tail, dorsal fin and ventral fin; catches positional paths or omitted station/airfoil fields. |
| full_reversal_changes_endpoint_ids_without_moving_station_provenance | First/last ID pair changes from tip/root to root/tip, while each ID keeps its original numeric inventory and source map; horizontal order still rejects, existing vertical rule still permits full reversal without controls. |
| duplicate_station_ids_are_rejected_within_each_surface | Exact duplicate identity error, for every surface case; catches missing surface-local uniqueness guard. |
| empty_whitespace_and_invalid_station_ids_are_rejected_including_mirrors | Exact identity error for empty, space, tab/newline and invalid punctuation, including mirrored surfaces. |
| moving_a_provenance_record_to_a_different_station_id_is_rejected | Move one required record to an unrelated ID; original station's record is missing, so rejection identifies its exact path. |
| the_same_station_ids_are_valid_on_two_different_surfaces | Separately authored inventories under two surface IDs coexist; catches a global station-ID uniqueness rule or lost surface namespace. |
| mirroring_keeps_opaque_station_identity_and_the_existing_physical_points | Hand-listed reflected points plus declared ID/source retention; catches ignored IDs on mirrored surfaces or changed reflection. |
| root_tip_direction_uses_positions_not_the_meaning_of_id_names | Valid hinge runs from first station named tip to last station named root; catches using the names to choose a physical span. |
| Station compile-fail example | All old fields are specified but ID is omitted; E0063 after implementation proves missing identity cannot construct a Station. |

Expected numeric records and metadata are authored in the test, never
obtained from Design::numbers. All new test values/settings are labelled
synthetic in fixtures/station_id_values.md. Shared fixture numbers do not
serve as the expected answers for these new tests.

## 4. Fixture migration: exact diffs and preservation

The shared fixture adds only these explicit identities:

```diff
--- tests/support/mod.rs
+++ tests/support/mod.rs
@@ first Station
+                    id: "0".into(),
                     leading_edge_body_m: root,
@@ second Station
+                    id: "1".into(),
                     leading_edge_body_m: tip,
```

The independent containment geometry adds the same opaque strings:

```diff
--- tests/hinge_containment.rs
+++ tests/hinge_containment.rs
@@ first Station
+                        id: "0".into(),
                         leading_edge_body_m: [0.0; 3],
@@ second Station
+                        id: "1".into(),
                         leading_edge_body_m: tip,
```

The existing intentionally unordered clone needs its own identity so the
original ordering assertion continues to exercise ordering:

```diff
--- tests/validation_regressions.rs
+++ tests/validation_regressions.rs
@@ empty_and_unordered_stations_have_specific_errors_for_every_surface_kind
         let mut middle = s.stations[1].clone();
+        middle.id = "extra".into();
         for axis in 0..3 {
```

Exact expected_numbers diff:

```text
support::expected_numbers: NO DIFFERENCES
```

The strings 0/1 are manually declared opaque IDs, not generated positions,
and preserve every existing fixture path verbatim. The independent inventory
function is unchanged. Its normalized source SHA-256 before and after is
4bda83db1480acd5df793bbf3a5890ea8fe4e43c5ba04676cc0f4dbcbcde8693.
Comparing the three migrated source files after removing only the added ID
lines gives identical source text (normalizing line endings). Thus no value,
expected number, assertion, sign, coordinate or tolerance changed. The new
alphabetic-ID tests and index-path mutation prove the implementation does
not rely on these legacy-looking ID spellings.

Before migration: **234 passed, 0 failed, 0 ignored**. After migration with
the model implementation present in the working tree: **243 passed, 0 failed,
0 ignored**. The increase is the eight integration tests and one compile-fail
doctest. The old assertions, including complete independent numeric inventory,
run unchanged. Log: target/s3-02-ids/after-migration.txt.

### Every test reading a migrated fixture

There are 45 shared-fixture consumers and six independent containment-fixture
consumers. The table below lists all 51, including helper-mediated calls.
It was checked by tracing local helper calls from each test to fixture(),
support::fixture()/expected_numbers(), or containment's own_design(). The
new station_ids tests use their own data and do not read migrated fixtures.

| Test file | Test reading migrated fixture | Fixture |
|---|---|---|
| closed_kinds.rs | a_legitimate_tail_kind_can_have_a_computed_quantity_as_its_label | shared support::fixture |
| closed_kinds.rs | each_closed_kind_is_accepted_only_in_its_scope_and_with_supplied_origins | shared support::fixture |
| closed_kinds.rs | unknown_and_missing_kinds_are_rejected_in_every_nonwing_scope | shared support::fixture |
| closed_kinds.rs | verifier_decorations_never_override_computed_or_legitimate_supplied_kinds | shared support::fixture |
| closed_kinds.rs | absent_drafts_keep_not_estimated_without_an_implicit_numeric_zero | shared support::fixture |
| hinge_containment.rs | inside_hinges_are_accepted_in_actual_parent_span_directions | independent own_design |
| hinge_containment.rs | root_and_tip_boundaries_are_accepted_on_independent_geometry | independent own_design |
| hinge_containment.rs | slightly_outside_both_boundaries_is_rejected_on_independent_geometry | independent own_design |
| hinge_containment.rs | either_partly_or_fully_outside_hinges_are_rejected | independent own_design |
| hinge_containment.rs | span_check_scales_with_geometry_instead_of_a_fixed_distance_tolerance | independent own_design |
| hinge_containment.rs | chordwise_position_is_explicitly_not_checked | independent own_design |
| parameter_units.rs | every_model_unit_probe_is_checked_against_the_explicit_quantity_kind | shared support::fixture |
| parameter_units.rs | every_different_kind_unit_pair_is_rejected_in_both_directions | shared support::fixture |
| parameter_units.rs | non_si_and_wrong_case_units_do_not_bypass_the_si_boundary | shared support::fixture |
| parameter_units.rs | unknown_label_suffixes_have_no_meaning_but_declared_units_still_must_match | shared support::fixture |
| parameter_units.rs | a_long_or_short_label_suffix_never_selects_the_quantity_unit | shared support::fixture |
| quantity_identity.rs | original_casing_and_separator_bypasses_are_rejected | shared support::fixture |
| quantity_identity.rs | each_computable_quantity_is_rejected_in_each_scope_origin_and_spelling | shared support::fixture |
| quantity_identity.rs | physical_control_sign_is_independent_of_hinge_endpoint_order | shared support::fixture |
| quantity_identity.rs | typed_computed_identity_cannot_hide_behind_an_arbitrary_display_label | shared support::fixture |
| remaining_guards.rs | positive_diagonal_indefinite_inertia_is_rejected | shared support::fixture |
| remaining_guards.rs | template_default_requires_nonempty_source_for_every_numeric_input | shared support::fixture |
| semantic.rs | independent_component_inventory_and_every_numeric_path_are_preserved | shared support::fixture |
| semantic.rs | hand_mirrored_points_preserve_forward_and_down_axes_once | shared support::fixture |
| semantic.rs | engine_placement_is_body_xyz_and_force_direction_not_exhaust | shared support::fixture |
| semantic.rs | duplicate_id_has_exact_error | shared support::fixture |
| semantic.rs | missing_or_nonsurface_control_parent_is_rejected | shared support::fixture |
| semantic.rs | dangling_engine_table_is_rejected | shared support::fixture |
| semantic.rs | nonpositive_nonfinite_or_unordered_geometry_and_bad_controls_fail_clearly | shared support::fixture |
| semantic.rs | missing_inertia_and_nonwing_data_are_explicit_not_numeric_zero | shared support::fixture |
| semantic.rs | tensor_entries_each_have_provenance_and_unknown_metadata_is_rejected | shared support::fixture |
| semantic.rs | empty_flags_wrong_units_or_false_missing_origins_are_rejected | shared support::fixture |
| semantic.rs | template_values_are_distinct_from_user_values_and_outside_coverage_is_retained | shared support::fixture |
| semantic.rs | static_wing_overrides_and_opaque_aircraft_totals_are_rejected | shared support::fixture |
| semantic.rs | synthetic_template_cannot_imply_a_real_aircraft_identity | shared support::fixture |
| semantic.rs | malformed_engine_unit_vector_and_inertia_are_not_repaired | shared support::fixture |
| source_label_guards.rs | long_user_label_requires_nonempty_sources | shared support::fixture |
| source_label_guards.rs | long_kind_label_requires_nonempty_user_sources | shared support::fixture |
| source_label_guards.rs | unicode_user_label_requires_nonempty_sources | shared support::fixture |
| validation_regressions.rs | blank_supplied_sources_and_numeric_source_explanations_have_specific_errors | shared support::fixture |
| validation_regressions.rs | computed_provenance_is_rejected_on_every_supplied_and_missing_input | shared support::fixture |
| validation_regressions.rs | nonfinite_surface_coordinates_dimensions_and_section_inputs_are_rejected | shared support::fixture |
| validation_regressions.rs | nonfinite_fuselage_coordinates_and_dimensions_are_rejected | shared support::fixture |
| validation_regressions.rs | nonfinite_mass_cg_and_every_intrinsic_tensor_entry_are_rejected | shared support::fixture |
| validation_regressions.rs | every_angle_field_rejects_nonfinite_at_the_typed_constructor | shared support::fixture |
| validation_regressions.rs | nonfinite_engine_placement_table_axes_and_every_sample_are_rejected | shared support::fixture |
| validation_regressions.rs | nonfinite_control_and_supplied_parameter_scalars_are_rejected | shared support::fixture |
| validation_regressions.rs | symmetric_non_positive_definite_inertia_is_rejected_separately_from_asymmetry | shared support::fixture |
| validation_regressions.rs | empty_and_unordered_stations_have_specific_errors_for_every_surface_kind | shared support::fixture |
| validation_regressions.rs | reversed_and_zero_length_controls_have_specific_errors_for_every_kind | shared support::fixture |
| validation_regressions.rs | actual_parent_span_supports_ventral_inverted_v_and_canted_controls | shared support::fixture |

## 5. Scratch mutations (never committed)

Only isolated-build logs below are accepted evidence. Every copy contains
one mutation; each Cargo invocation compiles its own soas-designer source,
with a distinct build directory under target/s3-02-ids/isolated-build-NAME.
TEMP/TMP remain inside this checkout. No mutation/helper is committed.

| Mutation | Change in scratch copy | Actual failed test (representative) | Result |
|---|---|---|---|
| index_paths | Numeric traversal uses list index again | full_reversal_changes_endpoint_ids_without_moving_station_provenance | 2 passed, 6 failed; exit 101 |
| duplicate_ids | Fresh uniqueness set for each station, allowing duplicate IDs through the identity guard | duplicate_station_ids_are_rejected_within_each_surface | 7 passed, 1 failed; exit 101 |
| mirrored_ids | Numeric traversal uses index only on mirrored surfaces | mirroring_keeps_opaque_station_identity_and_the_existing_physical_points | 3 passed, 5 failed; exit 101 |
| name_based_endpoints | Span helper chooses stations named root/tip rather than first/last positions | root_tip_direction_uses_positions_not_the_meaning_of_id_names | 4 passed, 4 failed; exit 101 |
| empty_ids | Skip identity guard for empty string | empty_whitespace_and_invalid_station_ids_are_rejected_including_mirrors | 7 passed, 1 failed; exit 101 |

Actual failing-output excerpts:

```text
index_paths:
test full_reversal_changes_endpoint_ids_without_moving_station_provenance ... FAILED
test result: FAILED. 2 passed; 6 failed; 0 ignored

duplicate_ids:
test duplicate_station_ids_are_rejected_within_each_surface ... FAILED
left: Err(... reason: "orphan numeric provenance")
right: Err(... field: "components[surface-a].surface.stations[1].id", reason: "duplicate identity")
test result: FAILED. 7 passed; 1 failed; 0 ignored

mirrored_ids:
test mirroring_keeps_opaque_station_identity_and_the_existing_physical_points ... FAILED
test result: FAILED. 3 passed; 5 failed; 0 ignored

name_based_endpoints:
test root_tip_direction_uses_positions_not_the_meaning_of_id_names ... FAILED
Err(DesignError { field: "controls[control].hinge", reason: "hinge endpoints must progress root to tip along parent span" })
test result: FAILED. 4 passed; 4 failed; 0 ignored

empty_ids:
test empty_whitespace_and_invalid_station_ids_are_rejected_including_mirrors ... FAILED
left: Err(... reason: "numeric provenance missing")
right: Err(... field: "components[surface-a].surface.stations[0].id", reason: "identity must contain only ASCII letters, digits, hyphens or underscores")
test result: FAILED. 7 passed; 1 failed; 0 ignored
```

Full logs are target/s3-02-ids/mutation-isolated-NAME.txt. The first attempted
driver shared a build directory across copies; Cargo reused the first
mutation's executable for later copies. Those later results were invalid,
disclosed and discarded, then all five were rerun with independent build
directories. Original logs remain target/s3-02-ids/mutation-NAME.txt and
are not evidence for the later mutations. Production-suite builds were
separate and unaffected. No profile/incremental setting was changed.

## 6. Quality gates, assumptions and sources

All three gates pass on the final code/fixtures:

- cargo fmt --all -- --check: exit 0.
- cargo clippy --all-targets --all-features -- -D warnings: exit 0.
- Full unfiltered cargo test --all: exit 0, **243 passed, 0 failed, 0 ignored**.

The designer contributes 65 integration tests plus the new compile-fail
doctest; the other crates contribute the unchanged 177 tests. Logs under
target/s3-02-ids/: fmt.txt, clippy.txt and after-migration.txt. Every test
and scratch run sets TEMP and TMP to target/s3-02-ids/temp. Rust reported
incremental-cache hard-link warnings and automatically copied files instead;
this was disclosed. No production cache/profile setting, dependency, existing
tolerance, assertion or numeric expectation changed. Scratch build isolation
was corrected as described above.

Sources are the owner's original station-ID scope and merge confirmation,
DECISIONS D18, S3_02_PLAN section 3, S3_01_SEMANTIC_MODEL's identity/hinge
contract, and CONVENTIONS Coordinate frames for the unchanged reflection.
No new physical formula is introduced. New test numbers are exclusively
labelled synthetic, with their ledger cited above.

## 7. Concepts, questions and practical limits

- Opaque IDs preserve which station a source describes; list position
  separately defines its current root/tip role.
- Surface-local uniqueness permits reuse on another surface without
  merging either station's provenance namespace.
- Exact error checks distinguish an identity guard from a later unrelated
  missing/orphan metadata rejection. The separate fixture commit adds only
  required identities, preserving all original physical expectations.

Questions: none pending. The prerequisite blocker is RESOLVED. Only this
station-model task is completed; no reader/writer or other child starts.

**Least confident about:** path coverage cannot prove a source statement is
true, nor does stable identity impose a new physical ordering rule. Future
wire-format/group-binding tests remain necessary and are not claimed here.

**Assumptions made:** the existing model address prefix stays unchanged
except for station ID substitution. The approved file wrapper/aggregate
format remains separate work. Missing IDs are prevented by required String
construction, with no optional/default fallback or designer loader.

**Commands for the owner to verify:** set TEMP/TMP to an existing directory
inside the checkout, then run the three quality gates above. Inspect commits
c4cb807 and 0da98b5, the final implementation commit, and git diff main..HEAD
-- crates/soas-designer/tests/support/mod.rs to see the identity-only migration.
