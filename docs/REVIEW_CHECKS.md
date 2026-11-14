# Review regression checks (2026-11-14)

## Fix 1: named JSON objects

The aircraft v0 format has flat numeric mass/geometry fields, a table object,
and coefficient-cell objects (including numeric control derivatives). It does
not have mass/geometry/control-surface sub-record types. The tests cover those
scalars, every actual object type and every cell. Scenario tests cover the root,
initial state, controls record and every knot on all three surfaces.

Before the fix, core tests had 3 failures / 2 passes and CLI tests 4 failures /
1 pass: valid positional arrays were accepted. After the fix all 10 pass, and
unknown/duplicate/missing fields, trailing input and legal array shapes remain
covered. Object errors identify their field/cell/knot path.

Options considered:

- Custom Deserialize implementations or mirrored wire structs: enforce object
  input at each type, but duplicate field declarations/visitors and risk drifting
  from the existing derived schema and its diagnostics.
- A recursively wrapped Deserializer/Visitor adapter: retain derives and reject
  sequence-backed records, but add substantial generic forwarding machinery.
- **Chosen: object-shape preflight, then typed parsing of the original text.**
  This adds small checks at the two validated file entry points, keeps the schema,
  serde attributes and diagnostics, and adds no dependency. The cost is a second
  parse/allocation when loading. Generic Serde decoding alone still does not
  provide the file loader's shape and numeric validation; callers must use
  `aircraft::from_json` / `parse_scenario`.

## Fix 2: distinguish bilinear from planar interpolation

`mixed_interpolation.rs` has one independent test for each of the eleven
coefficients. Every mutated corner set has a nonzero mixed difference, and each
expected result is hand-calculated in a comment. These are all synthetic TEST
VALUES, not real aircraft data. No existing tolerance was changed.

At Mach 0.2 on [0,0.8] and geometric height 12000 m on [0,20000], normalized
coordinates are x=1/4, y=3/5. Corner weights [C00,C01,C10,C11] are
[0.30,0.10,0.45,0.15], where row is altitude and column is Mach.

The base pattern [1,2,3,7] has mixed difference 1-2-3+7=3. Its hand result is
0.30*1+0.10*2+0.45*3+0.15*7=2.9. A plane through C00,C01,C10 instead gives
1+(2-1)*0.25+(3-1)*0.6=2.45, missing the mixed term 3*0.25*0.6=0.45.
Tests scale/sign this pattern to respect every coefficient's input bounds.

| Coefficient | Hand bilinear result | Deliberate planar result |
|---|---:|---:|
| cl0 | 0.29 | 0.245 |
| cl_alpha_per_rad | 2.9 | 2.45 |
| cl_max | 2.9 | 2.45 |
| cd0 | 0.029 | 0.0245 |
| oswald_efficiency | 0.29 | 0.245 |
| cm0 | 0.029 | 0.0245 |
| cm_alpha_per_rad | -0.29 | -0.245 |
| cm_elevator_per_rad | -2.9 | -2.45 |
| cy_beta_per_rad | -0.029 | -0.0245 |
| roll_aileron_per_rad | 0.29 | 0.245 |
| yaw_rudder_per_rad | -0.29 | -0.245 |

A separate test makes the altitude axis explicitly `Some([1234])`, with one
row. It checks exact-node retrieval, Mach interpolation (0.75*0.2+0.25*0.4=0.25)
and errors immediately below/above that sole altitude. This differs from a
Mach-only table whose altitude axis is absent.

The working implementation passed all 12 tests. An isolated source copy outside
the repository replaced the bilinear lookup with C00+x(C01-C00)+y(C10-C00), for
every coefficient. Its result was:

```text
cl0: actual=2.450000000000000e-1, expected=2.900000000000000e-1
cm_elevator_per_rad: actual=-2.450000000000000e0, expected=-2.900000000000000e0
test result: FAILED. 1 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
```

The singleton test still passed that fault, while EVERY mixed-coefficient test
failed. This is the red phase for a test-coverage finding; production already
used the correct bilinear formula. No planar code is committed and no numerical
formula change is needed. `Fix 2: implementation` records the validated coverage
and limitations, rather than changing correct simulation mathematics.

## Fix 3: control/moment magnitudes

`moment_magnitudes.rs` configures the SAME synthetic fixture with S=2 m^2,
span b=6 m and mean chord c=0.5 m (b/c=12). The aerodynamic reference and CG
coincide so force-arm moments cannot obscure coefficient scaling. All geometry,
coefficients, angles and control deflections are labeled synthetic TEST VALUES.

Independent density is the already sourced T3 sea-level equation value
1.224999155887712 kg/m^3. At V=20 m/s, q=rho*V^2/2=244.9998311775424 Pa,
q*S=489.9996623550848 N, q*S*b=2939.9979741305088 Nm and
q*S*c=244.9998311775424 Nm. Expected SI magnitudes are calculated explicitly in
comments and compared as literal numbers, not recovered from the model's q or
moment outputs. Each isolated case sets the other moment coefficients to zero.

| Isolated term / positive input | Hand coefficient | Expected moment (Nm) |
|---|---:|---:|
| Cm0=0.03 | Cm=0.03 | Pitch +7.349994935326272 |
| Cmalpha=-0.6/rad, alpha=0.1 rad | Cm=-0.06 | Pitch -14.699989870652544 |
| Cmde=-1.5/rad, elevator=0.2 rad | Cm=-0.3 | Pitch -73.49994935326272 |
| Clda=0.25/rad, aileron=0.2 rad | Cl_roll=0.05 | Roll +146.99989870652544 |
| Cndr=-0.4/rad, rudder=0.25 rad | Cn_yaw=-0.1 | Yaw -293.99979741305088 |

The derivative/control cases also check negative inputs. A sixth test combines
all terms: Cm=0.03-0.6*0.1-1.5*0.2=-0.33, giving pitch
-80.84994428858899 Nm, with the roll/yaw values above. Each vector comparison
uses a new 1e-11 Nm numerical test bound; no existing test/tolerance is changed.
All five moment coefficients and all three surfaces are independently exercised.

The actual implementation passed all six tests. A NEW isolated source copy
exchanged span and chord only in moment normalization: roll/yaw used q*S*c and
pitch used q*S*b. Its output included:

```text
Cm0: actual=[[0.0, 88.19993922391528, 0.0]] Nm,
     hand expected=[[0.0, 7.349994935326272, 0.0]] Nm
aileron: actual=[[-12.249991558877126, 0.0, 0.0]] Nm,
         hand expected=[[-146.99989870652544, 0.0, 0.0]] Nm
test result: FAILED. 0 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out
```

The swapped formula preserves signs, demonstrating why sign-only tests are
insufficient. Pitch magnitude is 12x too large and roll/yaw 12x too small.
The faulted source was never committed; no production scaling change is needed.
`Fix 3: implementation` records coverage/evidence and the assumptions update.
The formulas remain q*S*b for roll/yaw and q*S*c for pitch, as required by the
owner and the existing T6 moment-normalization references.

Both scratch copies and their build outputs were removed after their isolated
runs. The working checkout never contained either faulted source variant.

References: the existing table contract in `docs/AIRCRAFT_FORMAT.md`; the
bilinear weighted sum above is direct arithmetic. Fix 1's software references
are the existing Serde data model / serde_json documentation. No new physical
constants, aircraft data or dependencies are introduced by these checks.

## Generic object discovery (Part 1)

No generic regression existed before this batch; prior tests hand-listed paths.
`crates/soas-core/tests/support/object_shapes.rs` now discovers every object
recursively in every JSON file in each crate's tests/fixtures directory. The
core/CLI `all_object_shapes.rs` tests replace each discovered object with its
values in source order, independently, and require a named-object rejection.
Source order is retained by a generic Serde visitor without new dependencies;
alphabetizing values could hide array acceptance behind unrelated type errors.
No object paths or field names are hand-listed. New fixture objects are included
automatically, including engines, cells and throttle knots. All numeric values
are reused unchanged from existing labeled fixture ledgers; no data were added.

Baseline: aircraft fixtures contain 6 and 26 objects; scenario fixtures contain
10 and 13. Both tests pass. A scratch copy bypassing only engine object guards
failed with 20 accepted positional engine/table/sample objects; bypassing only
throttle-knot guards failed with all 3 throttle records accepted:

```text
test every_discovered_aircraft_fixture_object_rejects_positional_values ... FAILED
test result: FAILED. 0 passed; 1 failed

test every_discovered_scenario_fixture_object_rejects_positional_values ... FAILED
test result: FAILED. 0 passed; 1 failed
```

The scratch fault is not committed. Existing production guards already handle
all current records, so no parser change was needed. This is regression
coverage, not new schema behavior or physics validation.

## Expected-number spelling audit: ab6e9e3

The T9b implementation commit `ab6e9e3` changed two hand-expected literal
spellings at Clippy's excessive_precision request. The exact diff was rechecked:

| Before | After | Identical IEEE-754 f64 bits (hex) |
|---|---|---|
| `-35.279975689566105` | `-35.279_975_689_566_11` | `c041a3d63e4f2e45` |
| `52.919963534349154` | `52.919_963_534_349_15` | `404a75c15d76c566` |

No expected binary value, tolerance or assertion changed. This is a spelling
correction, not a relaxed numerical comparison.

Part 0 also pins the exact schema-1 CSV header in the CLI test; it already
passes against production. No production output change was necessary. The
33-column table is in SCENARIOS.md; consumers must select fields by header.
The test adds no numeric fixture data or numerical test setting.

## T13 Gap 1 — higher-mode induced-drag weight

Owner-supplied independent rectangular CL/CDi/e constants at 4 and 128 odd
terms are pinned with absolute tolerance 2e-12; derivation/provenance and all
numerical ledgers are in data/reference/estimator/llt_independent.md. The new
test passes immediately: production already weights energy by n. Scratch n^2
mutations fail at both counts; no mutation is committed. All three gates pass
(170 workspace tests). No existing tolerance or production formula changed.

## T12 Gap 2 — exact station-order/root errors

The owner confirmed that the existing WingError struct (field/reason) should
be asserted, without adding an enum, and equal spans means root and tip at
y=0. Four standalone tests pin validation AND geometry entry-point errors:

| Synthetic case | Expected field | Expected reason |
|---|---|---|
| duplicate interior [0,1,1,4] | stations[2].y_m | right-half stations must be nonnegative and strictly increasing |
| descending interior [0,2,1,4] | stations[2].y_m | right-half stations must be nonnegative and strictly increasing |
| first station not root [1,2,4] | stations[0].y_m | root must be at the symmetry plane y=0 |
| equal root/tip [0,0] | stations[1].y_m | right-half stations must be nonnegative and strictly increasing |

Every number is a SYNTHETIC TEST VALUE with its JSON/source-line ledger in
tests/fixtures/station_order_values.md. All four pass immediately; production
already validates correctly. The isolated target/t15-review/gap2 copy removes
the order guard and the separate root guard; all four then fail because
validate() incorrectly returns Ok(()). The root test specifically protects
the root guard, not just the ordering guard. No mutation is committed.

```text
test result: FAILED. 0 passed; 4 failed; 0 ignored
```

No production correction, new error API, dependency, or tolerance change is
needed. Test TEMP/TMP is checkout/target/batch-temp.

Gap 2 final gates: formatting and Clippy pass; 174 workspace tests pass,
0 failed, 0 ignored.
