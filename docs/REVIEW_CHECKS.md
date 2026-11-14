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

References: the existing table contract in `docs/AIRCRAFT_FORMAT.md`; the
bilinear weighted sum above is direct arithmetic. Fix 1's software references
are the existing Serde data model / serde_json documentation. No new physical
constants, aircraft data or dependencies are introduced by these checks.
