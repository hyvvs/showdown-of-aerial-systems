# S3-01b verification ledger — synthetic test values only

No new physical number is introduced for Fix 1: every design is cloned from
the S3-01 fixture, whose complete numeric-path ledger is semantic_values.md.
Changing a quantity identity, label, scope or provenance flag does not change
its prescribed synthetic value, units or other expected numeric output.

- Parameter/component/flag indices 0 and the ordinal iteration order refer to
  the existing tail-law parameter and fixture controls; they are structural
  test settings, not aircraft data.
- Four spellings per independently listed quantity are canonical underscore,
  upper case, hyphen and space forms. These are label settings, not geometry.
- Wing/tail/fuselage/interference scopes and USER_SUPPLIED/TEMPLATE_DEFAULT
  origins form the exhaustive test cross-product. Registry coverage is checked
  against the independently authored QUANTITIES catalogue, not itself.
- Control sign preflight reverses the existing hinge endpoints with no new
  coordinates. Expected physical sign strings are unchanged from D6 and the
  original S3-01 assertions. No fixture correction is made by this test.
- No tolerance, physical constant, numerical solver setting or new formula
  is introduced. The control sign definitions come from CONVENTIONS, Forces
  and moments; quantity coverage comes from the owner's Fix 1 requirement and
  planform::WingGeometry, VLM/LLT outputs, Fourier terms and section loadings.
- Fix 1 adds only explicit `SuppliedNonWing` identity metadata to the original
  two fixture parameters. Their numbers and the entire expected numeric inventory
  remain unchanged. The scratch control compatibility experiment uses the
  original rudder endpoints reversed in memory, not a committed fixture edit.
  Its dot products -5 and +5 are derived observations from those existing
  endpoint/parent coordinates under the owner's clarified ordering formula,
  not additional physical inputs or tolerances.

## Fix 2 — exhaustive invalid-input and ordering probes

All fixture-derived coordinates, dimensions, masses, tensors, section inputs,
engine tables, controls and parameters retain the synthetic provenance in
semantic_values.md and the reused core engine ledger. For per-component mass
probes only, the existing wing mass/CG/tensor/angle tuple is cloned into each
component; it is synthetic probe data, not a mass estimate. Metadata is
refreshed solely to make these probe designs valid before invalidation.

| Test setting/value | Meaning and source |
|---|---|
| NaN, positive infinity, negative infinity | Deliberate invalid f64 probes, not physical values; each primitive numeric field is checked. Angle fields reject these at the existing Radians constructor, without unsafe construction. |
| Structural indices 0, 1, 2, 3, 4; lengths 2 and 3 | Existing root/tip, xyz/tensor indices, five-component order and three-control order. No numerical tolerance. |
| Empty string and whitespace including tab/newline | Deliberately invalid source text and provenance explanations. |
| Last inertia diagonal -6 | Sign reversal of existing synthetic +6; symmetric, indefinite, separately from the unchanged asymmetry test. |
| Interior station = tip + (tip - root) | Existing synthetic coordinates extended once, then the original tip follows; deliberate descending interior order for each surface kind. |
| Empty station list, coincident hinge endpoints, reversed endpoints | Deliberate topology/order invalidations, no new physical values. |
| Ventral fin tip/end [-7,0,5], root [-6,0,2] | Synthetic test geometry, actual-parent projection 10 from (-1,0,3) dot itself. |
| Inverted V-tail tip/end [-5,2,5], root [-4,0,2] | Synthetic test geometry, projection 14 from (-1,2,3) dot itself. |
| Canted fin tip [-7,4,-1], hinge start [-6,0,2], end [-6,4,3] | Synthetic test geometry, projection 13 from (0,4,1) dot (-1,4,-3); positive even though the hinge ends downward. |

The projection equation and root-to-tip ordering requirement are prescribed
by the owner's clarification. No physical force formula, solver setting or
tolerance is introduced. Provenance rejection loops use the independently
authored expected inventory, not the production numeric walker. Testing the
existing angle type's constructor is necessary because its private value
cannot represent a non-finite angle through the safe public API.

## Fix 3 — supplied parameter units

Every parameter probe clones the existing labelled synthetic tail-law value
(-0.7) and its source/coverage. Renaming it to a unit-test parameter does not
give it a physical interpretation. Absent variants use None, never a numeric
zero. All units and settings below are schema tests, not aircraft values.

- The independent suffix/unit catalogue is `_kg_m2`/`kg*m^2`,
  `_per_rad`/`1/rad`, `_kgps`/`kg/s`, `_m2`/`m^2`, `_rad`/`rad`, `_kg`/`kg`,
  `_m`/`m`, `_n`/`N`, `_unit`/`1`, `_fraction`/`1`, and the existing explicit
  dimensionless name `cl_max`/`1`. These cover the current model's numeric
  unit-bearing suffixes and existing coefficient exception. D4 prescribes
  internal SI; unit spellings match the existing numeric provenance inventory.
- Pairwise cross-product checks every differing declared unit in both
  directions, for present and absent values. Metadata intentionally uses the
  wrong declared unit too, so the name check must catch the inconsistency.
- `mm`, `ft`, `deg`, lowercase force `n`, unknown `furlong`, unsupported `mps`
  and uppercase suffix `M` are intentional spelling/mismatch probes. `_mps`
  is not a unit-bearing suffix used in this semantic model; future additions
  need a documented suffix/unit pair and independent tests.
- Longest suffix takes precedence: `_kg_m2` is not `_m2`, and `_per_rad`
  is not `_rad`. This introduces no conversion formula or tolerance.
- Parameter/component index 0 is the existing tail-law parameter; true/false
  selects present versus absent state. All original fixture data, expectations
  and tolerances remain unchanged.
## S3-02-ids identity-only setup migration

The existing unordered-interior-station regression clones a station solely
to make an invalid ordering. Assign the clone the unique opaque ID "extra"
so that its original ordering error remains the error under test. This adds
identity only; all coordinates, indices, expected error assertions and
numeric values remain unchanged. No duplicate-ID guard is bypassed.
