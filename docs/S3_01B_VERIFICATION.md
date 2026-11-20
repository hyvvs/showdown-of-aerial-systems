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
