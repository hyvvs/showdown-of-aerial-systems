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
