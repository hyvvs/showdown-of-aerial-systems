# S3-01: semantic aircraft inputs

## Scope and evidence

`soas-designer` is a pure in-memory library depending on core/estimator, with
no file I/O, clock, threads, random state or reverse dependency. It describes
components and validates input structure; it does not generate an aerodynamic
model, calculate mass/CG, export an aircraft, or define designer JSON. S3-02
and all later tasks remain unstarted. The owner contract is ROADMAP.md,
Recorded owner decisions / S3-01 / Machine-readable number provenance.

Tests-first commit: `7051e75 Task S3-01: tests`. It contains only tests,
their independent fixture inventory and the synthetic-value ledger. Before
implementation, the empty library scaffold gave this actual failure:

```text
error[E0425]: cannot find type `Design` in this scope
error[E0422]: cannot find struct, variant or union type `Surface` in this scope
error[E0422]: cannot find struct, variant or union type `AirfoilInput` in this scope
RED_EXIT=101
```

The tests were not changed during implementation. The initial API implementation
used owned unit strings where the tests required borrowed labels; production
`NumericField` was corrected to borrow labels, preserving every assertion.
Sixteen semantic tests pass, with exact comparisons and no new tolerance.

| Test | Independent check / failure it detects |
|---|---|
| `independent_component_inventory_and_every_numeric_path_are_preserved` | Separately authored field/path inventory, plus generic traversal of the reused engine JSON; omitted/swapped fields or lost tensor/table metadata change the exact inventory. |
| `hand_mirrored_points_preserve_forward_and_down_axes_once` | Hand-listed right/left/vertical points catch axis exchange, double reflection and duplicated symmetry-plane points. |
| `engine_placement_is_body_xyz_and_force_direction_not_exhaust` | Unequal x/y/z and signed force-direction expectations catch coordinate exchange or exhaust-direction reversal. |
| `each_control_has_its_recorded_physical_positive_direction` | Literal owner-approved D6 descriptions catch a changed physical command sign. |
| `duplicate_id_has_exact_error` | Duplicate component/control identity must return the independently specified field/reason. |
| `missing_or_nonsurface_control_parent_is_rejected` | Both missing and fuselage parent references must fail, rather than silently binding a control. |
| `dangling_engine_table_is_rejected` | A nonexistent table identity must fail with the exact table-reference path. |
| `nonpositive_nonfinite_or_unordered_geometry_and_bad_controls_fail_clearly` | Explicit zero/negative/nonfinite chord, unordered stations, negative width and inverted limits exercise separate exact errors. |
| `missing_inertia_and_nonwing_data_are_explicit_not_numeric_zero` | A legal partial draft keeps absent inertia and absent contributions, detecting invented zeros/default models. |
| `tensor_entries_each_have_provenance_and_unknown_metadata_is_rejected` | Removing one off-diagonal entry's provenance or adding an orphan numeric path must fail. |
| `empty_flags_wrong_units_or_false_missing_origins_are_rejected` | Independently invalid flag/unit/presence combinations cannot validate. |
| `template_values_are_distinct_from_user_values_and_outside_coverage_is_retained` | Explicit template and out-of-coverage flags survive validation as separate states. |
| `static_wing_overrides_and_opaque_aircraft_totals_are_rejected` | Forbidden aggregate scopes and wing-only labels on defining inputs cannot bypass the supplied/estimated split. |
| `provenance_labels_never_relabel_mixed_results_as_wing_only` | All six literal machine labels, especially DERIVED_FROM_INPUTS, are pinned independently. |
| `synthetic_template_cannot_imply_a_real_aircraft_identity` | A real-aircraft template name and explicit reference-aircraft claim must be rejected. |
| `malformed_engine_unit_vector_and_inertia_are_not_repaired` | Zero engine direction and asymmetric intrinsic tensor fail rather than being normalized/symmetrized. |

## Representation and validation

- `Design` holds a named common body datum, ordered component/table/control/
  contribution lists and a `BTreeMap` of numeric provenance. Identity values
  are stable ASCII letters/digits/hyphens/underscores; duplicates across all
  component/control/table/contribution identities are errors.
- `Surface` describes main wings and horizontal/vertical tails through
  leading-edge coordinates, chord, incidence, twist and optional independently
  attributed airfoil inputs. Horizontal stations progress toward increasing
  body y; vertical stations toward decreasing z (up). Mirroring declares only
  nonnegative-y input and reflects about the common body x-z plane. The public
  point view returns the declared points followed by reflected off-plane points,
  retaining symmetry-plane points once. It is not an estimator adapter/mesh.
- `Fuselage` records nose position, length and ordered positive sampled
  cross-sections with lateral/down offsets. Aft station distances lie within
  declared length. Sampled sections do not infer an aerodynamic loft or end caps.
- `EngineInstance` references an explicit `EngineTableDefinition`; placements
  retain body x/y/z and force-on-aircraft direction. Existing core `Engine`
  and table validators enforce the same table/direction/dry-throttle rules,
  including the unchanged unit-direction roundoff bound. No thrust evaluation
  or force/moment assembly is added.
- `ControlSurface` refers to a lifting-surface parent, with distinct hinge
  endpoints, positive chord fraction not exceeding one, explicit travel limits
  and the binding elevator/aileron/rudder physical sign descriptions. These
  descriptions are command conventions, not geometry-derived effectiveness.
- `MassInput` keeps mass, CG and intrinsic tensor optional independently.
  Tensor axes have explicit roll/pitch/yaw orientation inputs. Present tensors
  use signed entries, finite exact symmetry, positive definiteness and finite
  inverse, following existing core numeric validation. No rotation, parallel-axis
  shift or mass buildup is performed. Missing tensors stay missing in valid
  drafts; later S3-05 must block export when required inputs are absent. There
  is no export/readiness API in S3-01.
- `SuppliedContribution` accepts only tail/fuselage/interference/whole-aircraft
  CLmax scopes with source and declared coverage. Explicit Wing/WholeAircraft
  scope requests and computed wing quantity identities are rejected. ClMax accepts
  only positive, optionally missing `cl_max`. This is an input ledger, not
  coefficient assembly or a physical interpretation of arbitrary parameter names.

Drafts need not contain every aircraft component or any aerodynamic contribution.
Optional airfoil/mass/contribution values remain explicit `Option` values; no
physical default is invented. An absent aggregate CG/tensor has one absent
metadata path; a present vector/tensor has metadata for every actual element.
The owner-prescribed recomputation rules remain for S3-04; no wing area, AR,
aggregate slope, induced drag or neutral point field is stored as a defining input.

Public structs allow editing, as existing core input structs do. Call `validate`
after edits. Placement/point views validate before returning data. Validation
returns deterministic `DesignError { field, reason }`, with physical/reference
checks before numeric-metadata checks. It establishes declared consistency,
not that supplied citations or physical aircraft properties are true.

## Numeric provenance and identity

`numbers()` traverses all typed numeric inputs, including engine table nodes/
samples and each tensor element, and sorts paths. Every path requires parallel
metadata with matching unit, nonempty source/explanation and ordered unique
flags. Orphan metadata and missing flags are errors. Present semantic inputs
require USER_SUPPLIED or TEMPLATE_DEFAULT, optionally OUTSIDE_COVERAGE; absent
inputs require NOT_ESTIMATED. Out-of-coverage inputs may be recorded as drafts;
S3-01 performs no analysis that could use them outside coverage.

ESTIMATED_WING_ONLY and DERIVED_FROM_INPUTS have the exact owner-approved public
labels for future result records; they cannot relabel defining inputs. Source
dependencies must reference existing numeric paths and cannot self-reference.
Geometry revision/approximate metadata is retained without inventing an estimate.
Report rendering, dependency evaluation, numeric coverage assessment and cycle
evaluation are later work, not claims of this validator.

Synthetic templates use the registered neutral labels `Synthetic aircraft study`
or `Synthetic wing study`; restricting the registry avoids guessing whether an
arbitrary template label denotes a real aircraft. User-design text is not proof
of reference-aircraft identity. Explicit real-aircraft claims are unsupported
until a separately specified validated-data contract exists. S3-01 does not
certify arbitrary descriptive/source text as truthful.

## Formula sources and fixture ledger

Only coordinate reflection `(x,y,z) -> (x,-y,z)` is introduced here: the
binding body frame is CONVENTIONS.md, Coordinate frames; the same reflection
is in [geometry reference](../data/reference/estimator/geometry.md), Geometry
contract and formulas. Engine direction preserves the core Engine contract
([aircraft format](AIRCRAFT_FORMAT.md), Engine extension). Control sign strings
are CONVENTIONS, Forces and moments / D6. Tensor checks reuse core
`dynamics::MassProperties`, not an inertia approximation or new formula.

All new numbers are labeled SYNTHETIC TEST VALUES in
[semantic_values.md](../crates/soas-designer/tests/fixtures/semantic_values.md),
with per-path inventory `tests/support::expected_numbers`. Expected coordinates
and scalar fields are independently authored; expected engine numbers are
discovered generically from the previously labeled core JSON fixture, not from
production `Design::numbers`. No real-aircraft or engine data are introduced.

## Computed wing quantity registry — S3-01b Fix 1

`SuppliedParameter.quantity` is a mandatory `QuantityIdentity`: either
`SuppliedNonWing` or `ComputedWing(ComputedWingQuantity)`. The latter is always
rejected in every scope, regardless of its display name. A known computed-wing
label cannot be laundered by declaring `SuppliedNonWing`: `from_name` resolves
canonical labels case-insensitively, ignoring underscores, hyphens and
whitespace, then rejects that computed identity too. This replaces the old
case-sensitive prefix test. Quantity validation precedes scope and label
syntax checks, preserving one specific parameter-path error across spellings:
`computed wing quantities cannot be supplied`.

The registry is defined in
[quantities.rs](../crates/soas-designer/src/quantities.rs). Its 33 identities cover
the owner's named outputs and existing geometry/VLM/LLT numeric outputs:

| Group | Registered identities |
|---|---|
| Planform | Area, Span, AspectRatio, TaperRatio, MeanAerodynamicChord, MacLeadingEdgePosition, MacQuarterChordPosition, QuarterChordSweep |
| Wing coefficients and stability | LiftCoefficient, LiftSlope, InducedDragCoefficient, PitchingMomentCoefficient, PitchingMomentSlope, RollingMomentCoefficient, NeutralPointPosition, SpanEfficiency |
| Section/loading/Fourier results | SectionLoading, SectionLiftCoefficient, SectionChord, SpanwisePosition, CirculationOverAirspeed, LiftPerSpanOverDynamicPressure, InducedAngle, FourierCoefficient, FourierHarmonic |
| Reference/mesh/diagnostic outputs | ReferencePointPosition, RelativeCollocationResidual, PanelBoundStartPosition, PanelBoundEndPosition, PanelControlPointPosition, PanelCirculationOverAirspeed, PanelSpanwiseIndex, PanelChordwiseIndex |

Canonical names come from `canonical_name()`; every variant appears in
`ComputedWingQuantity::ALL`. These are identities, not new aerodynamic
calculations/defaults or a serializer. Defining coordinates/section inputs
remain inputs; sampled wing-only results have distinct output identities.
Declared non-wing semantics do not prove a human's physical claim is true,
but display text cannot override an explicit computed identity or bypass
a recognized canonical spelling.

**Extension rule:** for a new computable estimator output, add its variant and
canonical label, update this coverage table, and update the independently
authored expected catalogue in `tests/quantity_identity.rs`. Add the same
four-scope/two-origin/four-spelling rejection cases and opaque-display-label
typed-identity case. Expected coverage must not be generated from the registry:
exact catalogue equality detects accidentally dropped entries. Future loading
must preserve the mandatory identity, not infer authority from display text.
S3-02 remains unstarted.

Tests were committed first as `12968e4`. Original-code runtime evidence:

```text
physical_control_sign_is_independent_of_hinge_endpoint_order ... ok
each_computable_quantity_is_rejected_in_each_scope_origin_and_spelling ... FAILED
original_casing_and_separator_bypasses_are_rejected ... FAILED
accepted forbidden quantity: Wing_area_m2, UserSupplied
test result: FAILED. 1 passed; 2 failed; 0 ignored
```

Typed-registry tests also failed to compile before that API existed. All five
new tests then passed alongside the original 16 assertions. The test matrix
checks all 33 quantities under four scopes, two origins and four spellings;
another matrix uses typed quantities with arbitrary display names. Fixture
parameters gained only `SuppliedNonWing` identity metadata; no numeric value
or existing expected value changed for Fix 1.

An uncommitted scratch copy restored the old case-sensitive prefix guard in
place of the typed/normalized guard. It compiled and reproduced the assertion
failure (exit 101):

```text
original_casing_and_separator_bypasses_are_rejected ... FAILED
accepted forbidden quantity: Wing_area_m2, UserSupplied
test result: FAILED. 0 passed; 1 failed; 0 ignored; 4 filtered out
```

Logs are under `target/s3-01b`: `fix1-red-runtime.txt`, `fix1-red-api.txt`,
`fix1-green.txt` and `scratch/prefix-mutation.txt`. No mutant is committed.

Fix 1 quality gates pass: formatting, Clippy with warnings denied and the full
workspace suite, **198 passed, 0 failed, 0 ignored**. This is also the recorded
full-suite result **before** any rudder endpoint correction. The unchanged VLM
acceptance executable took 335.44 s on this run, slower than the previous run;
no test/profile/cache setting was changed to work around it. Compiler cache
hard-link failures again triggered its automatic copy fallback and were
disclosed. No fixture number or original assertion changed for Fix 1.

## Scratch mutation evidence — original S3-01

Each fault ran in its own ignored `target/s3-01/mutations/<case>` crate copy,
with unchanged tests, isolated Cargo target storage and TEMP/TMP under that
copy. Only the targeted production-source fault differed. No scratch source,
mutant or helper script is committed. Each run compiled and failed an actual
assertion (not merely a compiler error), with exit 101 and:

```text
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out
```

| Case / source fault | Actual failing test | Failure evidence |
|---|---|---|
| axes: exchange engine placement x/y | `engine_placement_is_body_xyz_and_force_direction_not_exhaust` | actual [8,7,9], expected [7,8,9] |
| mirror: negate y twice | `hand_mirrored_points_preserve_forward_and_down_axes_once` | repeated right tip [-2,4,5], expected left [-2,-4,5] |
| parent: bypass parent lookup | `missing_or_nonsurface_control_parent_is_rejected` | `unwrap_err()` received `Ok(())` |
| absent: replace absent numeric value with zero | `missing_inertia_and_nonwing_data_are_explicit_not_numeric_zero` | legal missing-input draft rejected because numeric zero contradicts NOT_ESTIMATED |
| force: negate declared force direction | `engine_placement_is_body_xyz_and_force_direction_not_exhaust` | actual [-0,-0.6,-0.8], expected [0,0.6,0.8] |
| provenance: accept a removed flag list | `empty_flags_wrong_units_or_false_missing_origins_are_rejected` | `unwrap_err()` received `Ok(())` |
| label: print DERIVED_FROM_INPUTS as wing-only | `provenance_labels_never_relabel_mixed_results_as_wing_only` | actual final label ESTIMATED_WING_ONLY, expected DERIVED_FROM_INPUTS |
| identity: bypass neutral-template identity guard | `synthetic_template_cannot_imply_a_real_aircraft_identity` | a forbidden real-aircraft template name validated as `Ok(())` |

The coordinate examples above are the already ledgered synthetic test values,
not added aircraft data. Full local logs are `target/s3-01/red-tests.txt`,
`semantic-tests.txt`, and each mutation copy's `test-output.txt`. Reproduce a
fault by copying the crate/tests and reused core fixture, changing the source
as in this table, then running its named test with `--exact`; never edit the
working production source or commit a mutant.

## Quality gates and filesystem observation

All three required gates pass: `cargo fmt --all -- --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all`.
Aggregated workspace summary: **193 passed, 0 failed, 0 ignored**, including
the existing large VLM acceptance cases and all 16 new semantic tests. The
unchanged VLM acceptance executable finished in 213.64 s; this is an observed
run time, not a new test setting or fixture value. All test
runs, including the initial red run and mutations, explicitly set TEMP and TMP
inside this checkout. No profile, incremental-compilation setting, assertion
or tolerance was changed.

Rust warned that incremental cache hard links failed on this drive and it was
copying instead. This compiler fallback was disclosed to the owner; no cache
was relocated or disabled. Semantic compilation/tests succeeded with that
warning; Clippy's gate completed successfully without lint warnings.
