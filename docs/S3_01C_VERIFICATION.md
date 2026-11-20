# S3-01c2 verification: closed kinds and remaining guards

## Branch and scope

Verified `main` at merge `850f143` contains `task/s3-01b`, the designer crate
and S3_01_SEMANTIC_MODEL.md. Fresh branch `task/s3-01c2` starts there; the
old `task/s3-01c` / blocker-only `f245334` was not reused. QUESTIONS marks
that blocker RESOLVED. Scope is only Fixes 5-7, no S3-02 or new dependencies.

## Fix 5: closed supplied quantity kinds

Tests commit: `1f89e82`. First, a runtime test against the old API showed
the incompatible name-based rule:

```text
a_legitimate_tail_kind_can_have_a_computed_quantity_as_its_label ... FAILED
called Result::unwrap() on Err(DesignError {
 field: "contributions[tail-law].parameters[wing_area_m2]",
 reason: "computed wing quantities cannot be supplied" })
test result: FAILED. 0 passed; 1 failed; 0 ignored
```

The complete new API tests then failed to compile before SuppliedQuantityKind
and the Supplied/Missing/Unknown states existed. Local red evidence:
`target/s3-01c2/fix5-red-runtime.txt` and `fix5-red-api.txt`.

The closed list is the owner's five examples plus the existing tail moment
slope; exact scopes, units, restrictions and later accounting are documented
in S3_01_SEMANTIC_MODEL, Closed supplied-quantity kinds. No computed variant
belongs to that enum. The computed catalogue expands from 33 to 42 to cover
sampled leading-edge x/z, twist, section slope/zero-lift angle, the three
vortex velocity-per-circulation outputs and the thin-airfoil slope helper.
Section sampled y/chord already have identities. Defining inputs are distinct
from sampled/computed outputs. No aerodynamic formula is added here.

### Tests and design-change migration

| Test | What is independently checked / why it fails a fault |
|---|---|
| legitimate tail kind with computed label | A tail slope remains a tail slope when labeled wing_area_m2; name classification wrongly rejects it. |
| each closed kind, scope and origin | Independently authored six-kind scope/unit table; all scopes and origin flags, plus allowed coverage markers. Wrong scope/origin fails the exact error assertion. |
| unknown/missing kinds | Both invalid states reject under both supplied origins in every non-wing scope. No guessed/default kind. |
| verifier decorations | Twelve computed identities, twelve label forms including embedded names, both origins and every permitted supplied kind/scope. Computed kinds reject; legitimate non-wing kinds with the same labels pass. |
| closed/disjoint catalogue | Independent exact allowed list and cross-domain canonical-ID check detect any added computed entry. Separate enum types prevent a computed variant being put in the supplied enum. |
| absent drafts | Each legitimate kind can remain None with NOT_ESTIMATED; no implicit numeric zero. |

Existing quantity tests still reject every computed kind in every prior
scope/origin/spelling combination, now declaring that kind explicitly.
The original name classifier is intentionally superseded by the owner's
design change; the old generic SuppliedNonWing state is removed, not silently
accepted. Original unit-probe strings are retained and checked against the
declared kind's unit, not a label ending. All differing allowed-unit pairs,
non-SI/wrong-case/unknown unit probes and both draft/present states remain
covered. Unknown label endings now have no meaning, with explicit acceptance
and wrong-unit rejection tests. No numerical assertion/tolerance is weakened.
Original semantic.rs and numeric inventory values are unchanged in Fix 5;
fixture changes add only explicit kind metadata.

Designer green: **44 passed, 0 failed, 0 ignored**. New values/settings ledger:
`crates/soas-designer/tests/fixtures/closed_kind_values.md`; prior ledgers
remain historical records. Sources: owner's Fix 5, existing estimator public
outputs and D4's SI policy; supplied-contribution assembly remains S3-05.

Scratch copies separately changed a supplied canonical ID to computed wing
area, bypassed the kind/unit check, and bypassed the kind/scope check. All
compiled and failed their targeted assertions (exit 101):

```text
supplied_catalogue_is_closed_and_disjoint_from_all_computed_results: FAILED
every_model_unit_probe_is_checked_against_the_explicit_quantity_kind: FAILED
each_closed_kind_is_accepted_only_in_its_scope_and_with_supplied_origins: FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored   [each run]
```

Logs: `target/s3-01c2/mutations/5-*.txt`; no mutations committed. All runs
set TEMP/TMP within this checkout or their checkout-local scratch copy.
Final Fix 5 gates pass: fmt, Clippy with warnings denied, full workspace
**221 passed, 0 failed, 0 ignored** (`fix5-all.txt`). A nested conditional was
simplified after Clippy's first rejection; no lint allowance was added.
The existing incremental hard-link warning triggered Rust's automatic copy
fallback and was disclosed; no profile/cache setting or dependency changed.

## Fix 6: surviving verifier mutations h and i

Tests commit: `bdeb0c7`. Existing production checks were already correct,
so the unmutated new tests immediately pass (**2 passed, 0 failed**). Before
committing them, the exact verifier faults were injected in scratch copies:

```text
mutation h: positive_diagonal_indefinite_inertia_is_rejected ... FAILED
 left: Ok(())
right: Err(DesignError { field: "components[wing].mass.intrinsic_inertia",
 reason: "invalid signed intrinsic inertia tensor" })
test result: FAILED. 0 passed; 1 failed; 0 ignored; 1 filtered out

mutation i: template_default_requires_nonempty_source_for_every_numeric_input ... FAILED
 left: Ok(())
right: Err(DesignError { field: "components[body].fuselage.length_m",
 reason: "source or missing-input explanation required" })
test result: FAILED. 0 passed; 1 failed; 0 ignored; 1 filtered out
```

Mutation h retains finite/symmetry/finite-inverse checks but replaces the
Cholesky test with positive diagonal entries. The synthetic test matrix has
positive diagonals and is invertible, yet has a negative eigenvalue; a hand
quadratic-form witness is negative. A separate valid tensor in the same test
ensures positive off-diagonal entries are not simply prohibited. Mutation i
exempts TEMPLATE_DEFAULT from the nonempty numeric-source check. The new
test covers every independently inventoried present numeric path, with an
empty source list, empty string, whitespace and contribution-source blanks.
Its exact error expectation detects acceptance or a different later error.

Both mutants compiled; neither is committed. Logs:
`target/s3-01c2/mutations/6-h-diagonals-only.txt`,
`6-i-template-empty-source.txt`, and `fix6-unmutated.txt`. Tests ran with
checkout-local TEMP/TMP. Synthetic matrices, hand results and all settings
are in `tests/fixtures/closed_kind_values.md`, Fix 6. No production change
is needed; its implementation commit records evidence/docs only rather than
introducing an artificial bug or replacing a working guard.

Fix 6 gates pass: fmt, Clippy with warnings denied, and full workspace
**223 passed, 0 failed, 0 ignored**. Log:
`target/s3-01c2/fix6-before-fixture-all.txt`. This full run was completed
before any approved elevator/rudder coordinate edit and is also the
pre-correction checkpoint. The two new tests passed on the original guards;
no production source change is made in Fix 6.

## Separate fixture correction: elevator and rudder

Owner approval required a separate correction commit and full suites before
and after, preserving every assertion/physical sign/tolerance and all other
fixture values. Commit `ae36257 Fixture: contain elevator and rudder hinges
in parent spans` changes only support fixture/raw expected-number coordinates
and their ledger, with no assertion-file or production change.

Before: **223 passed, 0 failed, 0 ignored** (`fix6-before-fixture-all.txt`).
After: **223 passed, 0 failed, 0 ignored** (`hinges-after-all.txt`). Both
full `cargo test --all` runs set TEMP/TMP to checkout-local `target/s3-01c2/tmp`.
No failure occurred for another reason and no non-coordinate expectation changed.

The previous elevator start projected to -3 before a span of 0..6; rudder
endpoints projected to -10/-5 before a span of 0..10 (square metres). Those
coordinates passed order alone but violated containment. The chosen elevator
points are tail root [-4,0,2] and tip [-5,2,3] m; rudder points are fin root
[-6,0,2] and tip [-7,0,-1] m. They reuse existing parent coordinates, each
exactly on its corresponding boundary, zero distance outside. Aileron remains
unchanged. Hand projections and all coordinates are synthetic and ledgered in
semantic_values.md. They do not introduce a hinge-length/direction expectation.

A before/after numeric snapshot compared all 161 paths including units and
provenance: **nine raw hinge-coordinate rows changed, 152 records identical**.
Those nine changes are:

```text
controls[elevator].hinge_end_body_m[0]: Some(-2.0) -> Some(-5.0)
controls[elevator].hinge_end_body_m[1]: Some(3.0) -> Some(2.0)
controls[elevator].hinge_end_body_m[2]: Some(4.0) -> Some(3.0)
controls[elevator].hinge_start_body_m[0]: Some(-1.0) -> Some(-4.0)
controls[rudder].hinge_end_body_m[0]: Some(-1.0) -> Some(-7.0)
controls[rudder].hinge_end_body_m[2]: Some(2.0) -> Some(-1.0)
controls[rudder].hinge_start_body_m[0]: Some(-2.0) -> Some(-6.0)
controls[rudder].hinge_start_body_m[1]: Some(3.0) -> Some(0.0)
controls[rudder].hinge_start_body_m[2]: Some(4.0) -> Some(2.0)
```

### Exact fixture and expected_numbers diff

```diff
diff --git a/crates/soas-designer/tests/support/mod.rs b/crates/soas-designer/tests/support/mod.rs
index cc35aa4..475573b 100644
--- a/crates/soas-designer/tests/support/mod.rs
+++ b/crates/soas-designer/tests/support/mod.rs
@@ -175,11 +175,16 @@ fn control(id: &str, parent: &str, kind: ControlKind) -> ControlSurface {
         min_rad: rad(-0.3),
         max_rad: rad(0.4),
     };
-    if kind == ControlKind::Rudder {
-        std::mem::swap(
-            &mut control.hinge_start_body_m,
-            &mut control.hinge_end_body_m,
-        );
+    match kind {
+        ControlKind::Elevator => {
+            control.hinge_start_body_m = [-4.0, 0.0, 2.0];
+            control.hinge_end_body_m = [-5.0, 2.0, 3.0];
+        }
+        ControlKind::Rudder => {
+            control.hinge_start_body_m = [-6.0, 0.0, 2.0];
+            control.hinge_end_body_m = [-7.0, 0.0, -1.0];
+        }
+        ControlKind::Aileron => {}
     }
     control
 }
@@ -320,8 +325,10 @@ pub fn expected_numbers(_design: &Design) -> Vec<(String, Option<f64>, &'static
             ("hinge_end_body_m", [-2.0, 3.0, 4.0]),
         ] {
             let point = match (id, name) {
-                ("rudder", "hinge_start_body_m") => [-2.0, 3.0, 4.0],
-                ("rudder", "hinge_end_body_m") => [-1.0, 0.0, 2.0],
+                ("elevator", "hinge_start_body_m") => [-4.0, 0.0, 2.0],
+                ("elevator", "hinge_end_body_m") => [-5.0, 2.0, 3.0],
+                ("rudder", "hinge_start_body_m") => [-6.0, 0.0, 2.0],
+                ("rudder", "hinge_end_body_m") => [-7.0, 0.0, -1.0],
                 _ => point,
             };
             for (i, value) in point.iter().enumerate() {
```

### Every test consuming elevator/rudder shared-fixture values

The following exhaustive list conservatively includes all shared-fixture
consumers, even error cases that exit validation before reaching the hinge.
The inventory test compares raw coordinates; the sign test reverses them but
asserts the same physical D6 descriptions. Existing orientation dot-product
assertions use their own explicitly assigned endpoints and are unchanged.
There are 43 fixture consumers; all assertions were byte unchanged for this
separate correction. The three tests without fixture access are excluded.

### semantic.rs

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

### quantity_identity.rs

- `original_casing_and_separator_bypasses_are_rejected`
- `each_computable_quantity_is_rejected_in_each_scope_origin_and_spelling`
- `physical_control_sign_is_independent_of_hinge_endpoint_order`
- `typed_computed_identity_cannot_hide_behind_an_arbitrary_display_label`

### parameter_units.rs

- `every_model_unit_probe_is_checked_against_the_explicit_quantity_kind`
- `every_different_kind_unit_pair_is_rejected_in_both_directions`
- `non_si_and_wrong_case_units_do_not_bypass_the_si_boundary`
- `unknown_label_suffixes_have_no_meaning_but_declared_units_still_must_match`
- `a_long_or_short_label_suffix_never_selects_the_quantity_unit`

### validation_regressions.rs

- `blank_supplied_sources_and_numeric_source_explanations_have_specific_errors`
- `computed_provenance_is_rejected_on_every_supplied_and_missing_input`
- `nonfinite_surface_coordinates_dimensions_and_section_inputs_are_rejected`
- `nonfinite_fuselage_coordinates_and_dimensions_are_rejected`
- `nonfinite_mass_cg_and_every_intrinsic_tensor_entry_are_rejected`
- `every_angle_field_rejects_nonfinite_at_the_typed_constructor`
- `nonfinite_engine_placement_table_axes_and_every_sample_are_rejected`
- `nonfinite_control_and_supplied_parameter_scalars_are_rejected`
- `symmetric_non_positive_definite_inertia_is_rejected_separately_from_asymmetry`
- `empty_and_unordered_stations_have_specific_errors_for_every_surface_kind`
- `reversed_and_zero_length_controls_have_specific_errors_for_every_kind`
- `actual_parent_span_supports_ventral_inverted_v_and_canted_controls`

### closed_kinds.rs

- `a_legitimate_tail_kind_can_have_a_computed_quantity_as_its_label`
- `each_closed_kind_is_accepted_only_in_its_scope_and_with_supplied_origins`
- `unknown_and_missing_kinds_are_rejected_in_every_nonwing_scope`
- `verifier_decorations_never_override_computed_or_legitimate_supplied_kinds`
- `supplied_catalogue_is_closed_and_disjoint_from_all_computed_results`
- `absent_drafts_keep_not_estimated_without_an_implicit_numeric_zero`

### remaining_guards.rs

- `positive_diagonal_indefinite_inertia_is_rejected`
- `template_default_requires_nonempty_source_for_every_numeric_input`

The Fix 7 root/tip-boundary and slightly-outside tests are deliberately separate
functions in hinge_containment.rs, with their own Design/stations/endpoints.
That file never calls support::fixture or reads shared hinge coordinates.
Correcting this fixture cannot make those independent tests pass.

## Fix 7: inclusive span containment

Tests first: `8287fd6`. The original rule accepted outside endpoints:

```text
inside_hinges_are_accepted_in_actual_parent_span_directions ... ok
root_and_tip_boundaries_are_accepted_on_independent_geometry ... ok
chordwise_position_is_explicitly_not_checked ... ok
either_partly_or_fully_outside_hinges_are_rejected ... FAILED
slightly_outside_both_boundaries_is_rejected_on_independent_geometry ... FAILED
span_check_scales_with_geometry_instead_of_a_fixed_distance_tolerance ... FAILED
 left: Ok(())
right: Err(DesignError { field: "controls[control].hinge_start_body_m",
 reason: "hinge endpoint must lie within parent span" })
test result: FAILED. 3 passed; 3 failed; 0 ignored
```

All six now pass; designer total **52 passed, 0 failed, 0 ignored**.
Local red/green logs: `target/s3-01c2/fix7-red.txt`, `fix7-green.txt`.

| Independent test | Why it can fail |
|---|---|
| Inside hinges | Hand quarter/three-quarter span points must remain valid across horizontal, dorsal, ventral, canted and swept parents. |
| Root/tip boundaries | Exact independently constructed endpoints on both boundaries must pass; exclusive bounds fail. |
| Slightly outside | A relative sqrt(EPSILON) displacement beyond either plane must reject with the exact endpoint error. This offset is a test input, not the acceptance tolerance. |
| Partly/fully outside | Both endpoints are checked independently; positive order alone does not contain a segment. |
| Geometry scaling | Boundary and outside checks at three ledgered scales prevent substituting a fixed metric allowance. |
| No chordwise rule | Large transverse offsets inside the span slab remain accepted; this does not establish attachment or aerodynamic effectiveness. |

The allowance is derived from outward-rounded enclosures of actual coordinate
subtractions, endpoint products and projection sums. Root/tip planes are
inclusive; comparisons indeterminate only within those arithmetic bounds
are accepted. There is no fixed distance/percentage/empirical multiplier.
Non-finite bounds error. Exact formulas, limitations and primary sources are
in S3_01_SEMANTIC_MODEL, Span containment and rounding allowance.

For the independently constructed 8 m span at [0,2,0] m, an instrumented
scratch copy of the Rust helper returned Qroot =
[15.999999999999991,16.000000000000018] and Qtip =
[-48.00000000000004,-47.999999999999964] m². The documented formulas give
outward allowances 1.7763568394002505e-14 / 4.263256414560601e-14 m², or
2.220446049250313e-15 / 5.329070518200751e-15 m after division by span 8.
These are geometry-dependent observations, not selected tolerances or values
used by tests. The source function and its numeric inputs are unchanged;
the trace helper was added only to an ignored scratch copy. Trace and full
settings ledger: `target/s3-01c2/span-roundoff-trace.txt` and
closed_kind_values.md, Fix 7.

Four additional isolated scratch faults all compiled and failed assertions
(exit 101): omit start check; omit end check; exclude boundaries by demanding
strict interior; use body y alone. Each summary was:

```text
test result: FAILED. 0 passed; 1 failed; 0 ignored; 5 filtered out
```

Evidence: `target/s3-01c2/mutations/7-*.txt`. No mutant or scratch helper was
committed. All new numbers/settings are labeled synthetic in
`tests/fixtures/closed_kind_values.md`. Chordwise position is not checked.

## Final batch gates, files and limits

Final fmt and Clippy-with-warnings-denied gates pass. Full unfiltered
`cargo test --all`: **229 passed, 0 failed, 0 ignored**. Logs:
`target/s3-01c2/final-fmt.txt`, `final-clippy.txt`, `final-all.txt`.
All tests, snapshots and scratch runs use checkout-local TEMP/TMP. The
incremental-cache hard-link warning and Rust's automatic copy fallback
were disclosed; no cache, profile or dependency setting changed.

| Fix | Files touched (paths relative to checkout) |
|---|---|
| 5 | `crates/soas-designer/src/{lib,quantities,validation}.rs`; `tests/{closed_kinds,quantity_identity,parameter_units}.rs`, `tests/support/mod.rs` (kind metadata only), `tests/fixtures/closed_kind_values.md`; `docs/{S3_01_SEMANTIC_MODEL,S3_01C_VERIFICATION,ASSUMPTIONS,QUESTIONS,TASKS,LEARNING}.md` |
| 6 | `crates/soas-designer/tests/remaining_guards.rs`, `tests/fixtures/closed_kind_values.md`; `docs/{S3_01C_VERIFICATION,ASSUMPTIONS,TASKS,LEARNING}.md`. Production guards unchanged. |
| Separate fixture | `crates/soas-designer/tests/support/mod.rs` and `tests/fixtures/semantic_values.md` only; raw elevator/rudder coordinates and independent expected entries. |
| 7 | `crates/soas-designer/src/validation.rs`, `tests/hinge_containment.rs`, `tests/fixtures/closed_kind_values.md`; `docs/{S3_01_SEMANTIC_MODEL,S3_01C_VERIFICATION,ASSUMPTIONS,QUESTIONS,TASKS,LEARNING}.md` |

Unprefixed `tests/` paths in the table are under `crates/soas-designer/`.
No core/CLI/estimator source, manifest, lockfile, dependency, original semantic
assertion, control sign or existing tolerance changed. Classification/unit
tests were migrated under the owner's explicit Fix 5 design change as
described above; original probe strings and stronger kind checks remain.

**Least confident about:** these validate declarations and numerical bounds,
not supplied-source truth, whole-aircraft aerodynamic accuracy or physical
hinge attachment. A point whose plane comparison is indeterminate within
roundoff is accepted by the documented allowance. Chordwise position is
explicitly unchecked, not implicitly validated. No assembly/export/modeling
claim follows from accepting the six supplied kinds.

**Assumptions made:** root/tip mean the first/last parent stations; arithmetic
bounds concern the exact stored f64 coordinates, not measurement uncertainty.
The supplied-kind list is exactly the owner's examples plus the existing
tail moment slope, without a generic escape hatch. Source/reference and
combination rules for contributions remain S3-05 specification work.

**Questions:** none pending for this batch. Both blocker and fixture entries
are RESOLVED in QUESTIONS. S3-02 remains unstarted. Nothing was pushed,
merged or changed in remotes; the old task/s3-01c branch was not reused.
