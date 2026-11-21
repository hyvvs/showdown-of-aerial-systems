# Questions for the owner

When a batch stops, append an entry with the task, what blocked it, and options with trade-offs. Do not guess an answer or continue to further tasks.

## 2026-11-21 — S3-02a contract questions — ANSWERED

- **Task:** S3-02a, documentation only; no implementation authorized.
- **What needed an answer:** S3_02_PLAN Q1-Q8 (version, tags, missing
  values, provenance, engine tables, replacement, templates and loading),
  plus how to bind metadata when equal-unit arrays are edited and when
  vertical-tail station ordering permits a full reversal.
- **Options/trade-offs recorded:** path-only maps can silently reattach
  sources; per-number snapshots add duplication/manual edits; inline
  metadata changes the layout. The owner chose the ID/order/aggregate
  hierarchy with snapshots only where necessary. Stable IDs require
  coordinated renames; ordered arrays reject permutations; grouped sources
  are less granular; snapshots need matching hand edits.
- **Answer:** Q1-Q8 adopted with the owner's conditions, recorded as
  DECISIONS D15-D22 and S3_02_PLAN section 4. Record-ID plus field binding
  comes first; validated strict order/index paths second; whole-array or
  table-axis/sample-block records third. No per-cell engine-table metadata.
  The owner approved stable station IDs for all surfaces after the
  vertical-tail ordering limitation was reported. Automatic array edit and
  path/ID-rename checks remain mandatory; report impracticality instead of
  weakening them. The universal snapshot-first suggestion was not adopted.
- **Status:** ANSWERED; S3-02a is the approved documentation contract.
  S3-02b1/b2 and later implementation remain PROPOSED and need separate
  authorization; S3-02e is DEFERRED, not dropped. No code/tests are started.

## Entry format

- **Task:** task ID and title.
- **What blocked:** the open decision, missing source/specification, failing gate and fix attempts, test weakening that would be needed, or non-approved dependency.
- **Options with trade-offs:** the choices the owner can make and the consequences of each.

## 2026-11-13 — Batch stopped before T1

- **Status:** ANSWERED on 2026-11-13. The owner clarified that only an OPEN decision required by the current task stops a batch. AGENTS.md now records that rule and explicitly states that D2, D8 and D9 do not block T1 to T9. Those decisions remain OPEN; no license or validation choice was made here.
- **Task:** T1: Continuous integration, first in the requested sequence T1, T0b, T2, T3, T4, T5, T6, T7, T8. No implementation or later task was started.
- **What blocked:** AGENTS.md Batch mode says to "Stop immediately, with no further tasks, if: a decision is OPEN". D2 (license) and D9 (validation tolerances) remain OPEN in DECISIONS.md. TASKS.md explicitly requires D9 only before T10, but the batch stop rule does not say whether it applies only to decisions required by the current task. The agent has not changed either decision or assumed an exception.
- **Options with trade-offs:**
  - Clarify that the OPEN-decision stop applies only when the current task depends on that decision. D2 can stay OPEN while no LICENSE file is added, and D9 can stay OPEN until T10 as already planned; this permits unrelated tasks to proceed while retaining their actual decision gates.
  - Keep the unqualified stop rule. The batch remains stopped until the owner resolves the remaining OPEN decisions; this delays unrelated work, and D9 would still need the H3 reference evidence.

## 2026-11-13 — T3 pressure-reference discrepancy

- **Status:** ANSWERED on 2026-11-13. The owner chose neither earlier option: use **5e-5 relative tolerance for pressure and density**; stop rather than widen if any other row requires more. All five rows pass this limit. T3 can proceed to its quality gates and commit.
- **Task:** T3: ISA atmosphere.
- **What blocked:** [U.S. Standard Atmosphere 1976](https://ntrs.nasa.gov/citations/19770009539), Table I, printed page 54, gives pressure `5.4019 * 10^2 mb = 54019 Pa` at geopotential height `5000 m`. Equation (33a), using `T0 = 288.15 K`, `P0 = 101325 Pa`, `g0 = 9.80665 m/s^2`, `R* = 8314.32 J/(kmol K)`, `M0 = 28.9644 kg/kmol` and lapse rate `-0.0065 K/m`, gives `54019.912103762 Pa`. An independent calculation reproduces the implementation. The difference is `0.912103762 Pa`, above the test's `0.5 Pa` limit based on assumed nearest rounding to the printed precision. The NASA PDF's appended errata correct unrelated constant-printing errors, not this checkpoint. The comparison has not been weakened.
- **Options with trade-offs:**
  - Authorize a `1.0 Pa` tolerance for this pressure checkpoint, retaining and documenting the discrepancy. This allows one unit in the last printed pressure digit, with no change to the equation, but relaxes the original test and therefore needs owner approval under Batch mode.
  - Keep the `0.5 Pa` limit and select a corrected, independently published reference table. This retains the original comparison strictness but requires further reference verification; a passing value must not be invented from our own implementation.
- **Build environment:** the incremental-cache hardlink warning was reported to the owner. Subsequent checks explicitly use `CARGO_INCREMENTAL=0`; no compiler warning or test criterion was suppressed.
- **Recorded resolution:** the printed 5 km value remains **54019 Pa** after checking every digit against the PDF. The equation gives **54019.912103762079 Pa**, difference **+0.912103762079 Pa**, absolute difference **0.912103762079 Pa**, relative difference **1.688486944e-5**. The chosen **5e-5** accommodates this discrepancy, and the requested deliberate-error tests still reject each mistake (the smallest pressure error is about 20 times the tolerance). Exact listed R*, M0 and g0 with 50-digit decimal arithmetic give **54019.912103762073475... Pa**; binary floating-point rounding of these constants does not explain this residual in that check. A tabulation/printing issue or undocumented calculation difference remains a hypothesis, not a verified cause. Full row errors and checks: `data/reference/atmosphere/README.md`.

## 2026-11-14 — T15 high-aspect-ratio comparison

- **Status:** ANSWERED on 2026-11-14. The owner authorized diagnosis, retention of the original comparison as an expected-difference record, and replacement acceptance using decreasing AR gaps and zero-limit extrapolation evidence. The diagnosis is committed as d5d0d01; replacement tests were committed first as 88dfc0e. T15 now passes all gates; T10/T11 remain unstarted.
- **Task:** T15: Planar-wing vortex-lattice solver.
- **What blocked:** Seven T15 tests pass. The predeclared high-AR rectangular comparison fails: VLM CL=0.2909122489637976 versus T13 CL=0.29278072785693654, gap 0.001868478893139 (0.638184% of T13), allowance 0.001258745458735. Induced CD=0.0007977879080538741 versus 0.0008624499010484641, gap 0.00006466199299459 (7.497478%), allowance 0.000008910394977062. Both computed gaps exceed the rule committed before implementation in 8067b1d; the CL assertion stops execution before the CD assertion. Tolerances/tests were not changed. The provisional method allowance uses Caughey's approximate finite-AR slope relations and holds span efficiency fixed when propagating into drag; neither is an established bound for this rectangular comparison. A numerical or implementation defect is also possible; no cause is established. Detailed equations, settings ledgers and refinement results: data/reference/estimator/vlm.md.
- **Options with trade-offs:**
  - Continue T15 diagnosis using an independently published VLM benchmark or a separately checked reference implementation, retaining the current failing comparison. This can identify a code/discretization error without changing acceptance, but takes further reference and numerical work and may not reconcile the two theories at this planform/aspect ratio.
  - Review the inter-method acceptance design first, requiring a source-supported, planform-specific treatment of finite-chord and span-loading differences plus mesh error. This may require an explicitly approved replacement test; it cannot be an allowance selected from the observed discrepancy. It improves the justification but requires more theory/reference evidence before implementation can be accepted.
- **Historical stop rule:** AGENTS.md required stopping before an unauthorized test weakening; the initial batch did stop. The subsequent owner instruction explicitly authorizes this acceptance redesign. Original allowances/inputs are unchanged in the historical record. The quarter-chord-control scratch fault now fails as required, and formatting, Clippy and all 169 tests pass. No dependency or T10/T11 work was added.
- **Historical resolution (superseded below):** All requested spacing, AR, ellipse and two-drag-method experiments are in data/reference/estimator/vlm_diagnosis.md. No production bug was established; mesh effects and finite-chord/tip-loading theory explain evidence, not a proof of correctness. The trend test requires strictly decreasing relative lift/CDi gaps, positive local/fitted power decay, and a free-intercept extrapolation compatible with zero within measured mesh plus leave-one-out range sensitivity. Its drag inverse-log hypothesis is explicitly conditional, not a rigorous physical bound. An analytic nonzero-floor fault is rejected. The original comparison/function/measurements remain in data/reference/estimator/vlm_expected_difference.md and 8067b1d. AVL was not run; no output existed to compare. Owner input/license/run instructions are in data/reference/avl/README.md.

## 2026-11-14 — T15 robust acceptance review

- **Status:** ANSWERED. The owner withdrew zero-compatible extrapolation acceptance and approved strict AR-gap trends on two meshes. Subset fits are informational; their drag verdict changes with subset.
- **T12 clarification:** The owner confirmed exact existing WingError field/reason assertions and equal spans as root and tip at y=0. No enum or guessed geometry was added.
- **Task:** T15 acceptance and T12/T13 regressions; T10/T11 remain unstarted.
- **What needed clarification:** Elliptic decreasing slope error against LLT contradicted the no-assertion Nx8 probe. AR6 error rises from 0.253622488807 to 0.315173010845 and AR40 from 0.006640783021 to 0.021675893522 under Ny32 to 512 refinement.
- **Owner answer:** Use decreasing mesh changes; keep lifting-line differences informational. Current tests follow that answer and retain efficiency-excess monotonicity. No interval or tolerance was adjusted to manufacture the rejected assertion.
- **Resolution:** Independent LLT constants and exact station errors pass against unchanged production; scratch mutations fail. Mesh experiments and subset fits are recorded. Richardson bands are derived from four coarser meshes before checking the finest-three estimate. They test numerical self-consistency, not physical equality to e=1 or certified continuum bounds. Stronger bands are unsupported by the finite data. All three gates pass: 175 tests, none failed/ignored. The original comparison record is unchanged. Full evidence: data/reference/estimator/vlm_acceptance_review.md.

## 2026-11-19 — T15b efficiency-gap decay

- **Status:** UNRESOLVED. The requested experiments are complete; a causal explanation is not established and was not required to complete this observational task.
- **Task:** T15b Part 3.
- **Open question:** why does the CDi gap decay as AR^-0.25 while the CL gap decays as AR^-1?
- **Evidence:** The owner-supplied finite-range fits over AR20..160 are CL p=1.00897 and CDi p=0.25445 on the specified growing meshes. Same-CL efficiency differences persist in the independent calculations; algebraic CL-squared/efficiency decomposition is recorded. Outer5%-semispan refinement at AR20/40 changes efficiency differences only slightly on the two studied levels. These observations do not identify a physical mechanism or a bug and do not prove universal asymptotic exponents.
- **What remains unknown:** how much is due to the mathematical model definitions, wake/quadrature discretization, other unresolved mesh effects or implementation details. The decomposition is an identity, not a causal attribution; more Fourier terms alone changed little at the reported precision.
- **Options with trade-offs:** A future controlled mesh/quadrature and model-definition study could isolate numerical/model differences but needs additional computation and independent references; comparison with a separately established continuum result could constrain interpretation but requires suitable sourced cases. Neither investigation is started here; T10/T11 remain unstarted.
- **Records:** data/reference/estimator/vlm_gap_vs_ar.md, vlm_efficiency_gap.md and independent/T15B_EFFICIENCY_LEDGER.md.
- **Answered build preference:** The owner approved estimator-only test-profile optimization for Part 1. It changes compilation settings, not model inputs, assertions or tolerances.

## 2026-11-19 — T15b profile resource limit

- **Status:** ANSWERED. The owner confirmed **4 minutes** for the existing largest test function, including both 8192-panel cases; the disclosed memory guard is **2.0 GB** (2,000,000,000 bytes).
- **Task:** T15b follow-up Part 1, test-profile evidence.
- **What needed clarification:** "A few minutes" needed an explicit wall-time limit before measuring the unchanged test. No meshes, assertions or tolerances were adjusted.
- **Resolution:** The function passed unchanged in 179.9444381 s, with peak resident memory 1,081,479,168 bytes and peak paged/commit counter 1,078,722,560 bytes. Neither guard tripped, so this resource condition does not block the later requested experiments. Settings, printed old/new profile outputs and measurement limitations are ledgered in data/reference/estimator/t15b_profile.md.

## 2026-11-19 — T15b follow-up observations, cause still unresolved

- **Status:** UNRESOLVED. The requested profile, direct production and independent taper experiments are complete; identifying a cause was not a completion condition.
- **Task:** T15b follow-up Parts 2 and 3. T10/T11 remain unstarted.
- **Open question retained:** why does the CDi gap decay as AR^-0.25 while the CL gap decays as AR^-1?
- **New evidence:** Direct production Rust efficiency agrees with the archived independent result on the rectangular AR10/20/40/80 grids, with relative differences below 5.38e-14. AR160 production was not launched because two dense matrices alone exceed the disclosed memory budget. These observations limit one implementation-disagreement hypothesis on those grids; they do not validate the shared model or identify the LLT-gap mechanism.
- **Taper evidence:** The independent sweep covers all four requested tapers and five aspect ratios. Fourier sensitivity through 4096 terms, both empirical fit forms and all residuals are recorded. The fitted **efficiency-gap** p depends on taper and is negative at lambda .5; it must not be confused with the earlier **CDi-gap** exponent. Tip-width/local-chord ratios support geometric sampling but do not prove aerodynamic mesh convergence. Raw lambda .5 / AR10 e=1.001331201893 is retained and flagged; whether it comes from mesh/quadrature, modeling or implementation remains unestablished.
- **Options with trade-offs:** A future joint VLM refinement and force/wake-definition comparison could isolate numerical sensitivities but costs further computation, especially for production dense solves. An independently established continuum or external-solver benchmark could constrain the explanation but requires suitable matched data and provenance. Neither follow-on is started, and neither causal explanation is asserted.
- **Records and complete numerical provenance:** data/reference/estimator/t15b_production_comparison.md, vlm_taper_sweep.md, independent/T15B_PRODUCTION_LEDGER.md and independent/T15B_TAPER_LEDGER.md. No tolerance was loosened, no dependency added, and no threshold-based verdict chosen.

## 2026-11-20 — T16 completed; modeling remains NO-GO

- **Status:** UNRESOLVED for modeling. The documentation audit itself is complete; T17/T10 were not started. The full questions and source/section evidence are in [T16_AUDIT.md](T16_AUDIT.md), §§1-7; authorization remains limited by ESTIMATOR_PLAN.md's opening scope.
- **Task:** T16 Navion source and comparison-definition audit.
- **What blocks modeling:** Teper's Navion row is not matched to complete geometry/section inputs, a resolved area/chord/CG datum, actual fixed controls/configuration or a documented power/propeller state. Later Navion sources only partly close these and do not authenticate CR's condition (audit §§1-4; CR pp. 100-103; NASA TN D-5857 pp. 3-6, 10, 12; NASA TN D-6643 pp. 6-7, 19-21, 35).
- **Options with trade-offs:** Continue the original-source/geometry search while keeping CR estimation unavailable; this preserves the intended aircraft but has unresolved source availability. Alternatively, review the proposed CRM-HL / ONERA LRM-WBSHV Case 2.2 package, whose matching public CAD/static data were checked; it changes the aircraft and adds slat/section/deformation, axis-conversion and CC BY-NC-ND reuse questions. Neither option is selected and neither authorizes implementation (audit §3 and §§6-7; ONERA guide §§3, 6.4, 8.2.3, 8.3.1; ESTIMATOR_PLAN.md §Required NO-GO follow-up).
- **Later scope resolution:** The owner's T16c request selects a conditional wing-alone first reference and excludes CRM-HL from that comparison. The options above are historical; the current checkpoint and remaining choices are in the T16c entry below (T16B_REFERENCE_SEARCH.md §§6/9; T16C_TN1270_CLOSURE.md §§1/7).

## 2026-11-20 — T16c closure and pre-modeling checkpoint

- **Status:** ANSWERED by the owner's post-T16c acceptance of recommendations 2-5; question 1 was already answered. T10/T16-T28 are parked. No modeling task, T11 or Stage 3 implementation is started. The new Stage 3 questions below are separate. (Owner post-T16c answers; ESTIMATOR_PLAN, Current scope.)
- **Task / blocker:** TN 1270 bounded closure. Reference definitions and report reuse are closed, but continuous spanwise prescriptions and matched section inputs are partial; exact finite-wing Mach is unverified. The formal experimental-source verdict is NO-GO. Published sample section curves in TN 1269 concern a different worked wing and do not close the matched-input gap. (T16C_TN1270_CLOSURE.md §§3-5; TN 1270 pp. 3-6/Table I/Fig. 3; TN 1269 p. 20/Table IV/Figs. 1-3.)
- **Five original question subjects, with full options/trade-offs in closure §7:**
  1. First reference: **ANSWERED** — wing-alone first, TN 1270 conditional; no modeling before the plan's checkpoint. The alternative complete-aircraft-first route has extra coupling/data prerequisites and is deferred. Recommend retaining the owner's answer. (Owner T16c Decisions; closure §7 question 1.)
  2. Source closure: **ANSWERED — park TN 1270, NO-GO for a fully sourced experimental comparison; no further reconstruction.** The alternative records/curve reconstruction and assumption review is not selected; gaps remain listed, not silently closed. (Owner answer 2; closure §§3/5-7.)
  3. Layer B: **ANSWERED — defer clean wing-tail source work.** No new bounded search now; existing mounting/Mach gaps remain. (Owner answer 3; T16b §4; closure §7.)
  4. F-104: **ANSWERED — defer searching to v0.2**, retaining RM SL56H06 and inventoried flight-data leads, with identity/method qualifications intact. It remains excluded first; D8 is unchanged. (Owner answer 4; closure §7; ROADMAP, candidate register.)
  5. Fallbacks: **ANSWERED — park R&M 2882 and RM L52G18.** Neither source reconstruction nor the support-contaminated diagnostic alternative is selected. No correction/permission is inferred. (Owner answer 5; closure §7; T16b §3 A2-A3.)
- **Consequence:** Exact v0.1 status is **"code-to-code cross-checked and analytic-case verified; not experimentally validated"**. Both validation page and report disclaimer must state it. Stage 3 is next; experimental validation remains parallel and T10/T16-T28 stay parked with reopening gates. No further source search or implementation occurs in this documentation task. (Owner post-T16c consequences; ROADMAP/ESTIMATOR_PLAN, current sections.)

## 2026-11-20 — Stage 3 plan: six owner choices answered

- **Status:** ANSWERED. The owner adopted all six recommendations with the conditions below. No repeat approval is needed for **S3-01 only after recording**; the current request is a docs-only commit, no code. No later S3 batch or parked T task is authorized. (Owner Stage 3 adoption, choices 1-6 and final instruction.)
- **Task / scope:** semantic designer model first; current estimator remains wing-only with no tail/fuselage aerodynamic model. Detailed current rules in ROADMAP.md supersede the broader entirely supplied aircraft-bundle proposal in 94ae26f/ESTIMATOR_PLAN's earlier Stage 3 interface discussion. No physical model or schema is implemented here. (Owner choice 2; ROADMAP, Binding estimated-versus-supplied contract.)
- **Recorded answers and retained alternatives:**
  1. **Architecture — ANSWERED:** pure `soas-designer` depending on core/estimator, CLI-owned I/O; **S3-01 only authorized after recording**. The alternate module/task split is not selected. (Owner choice 1.)
  2. **Aerodynamic inputs — ANSWERED WITH CONDITION:** incomplete drafts allowed; trim/static-margin/export need a complete assembled bundle. User/template aerodynamic values cover only permitted tail, fuselage, interference and CLmax contributions. Everything computable from geometry, including wing area/AR/lift slope/induced drag/neutral point, is recomputed on geometry changes and never stored as an authoritative static input. Neither an opaque all-supplied total nor an override of a computed wing result is selected. (Owner choice 2; ROADMAP contract/S3-04/S3-05.)
  3. **Mass/inertia — ANSWERED:** component masses, CGs and intrinsic tensors; incomplete drafts may be saved, missing inertia blocks export. The aggregate-inertia override alternative is not selected. Record **later S3-10** for box/cylinder/ellipsoid helpers and parallel-axis accounting, with results explicitly approximate. (Owner choice 3; ROADMAP S3-03/S3-10.)
  4. **Conditions/CG — ANSWERED:** explicit conditions, loading cases and CG bounds/samples; separate check results. No invented acceptance thresholds, range or aggregate flight-safety verdict. (Owner choice 4.)
  5. **Stall — ANSWERED WITH CONDITION:** supplied constant whole-aircraft CLmax within declared coverage. Every result says **"conditional on supplied CLmax"** and **"flaps and high-lift devices are not modelled"**. State stall speed responds to geometry changes only through area and weight, not an estimated CLmax change. The **11 m to 9 m wingspan demonstration is a required Stage 3 acceptance test** with the explicitly controlled assumptions in ROADMAP. The implicit Mach/Reynolds-dependent stall model is not selected. (Owner choice 5/acceptance instruction.)
  6. **Evidence/provenance — ANSWERED WITH CONDITION:** labeled synthetic examples, neutral template names, no confidence percentages or implied real-aircraft identity until validated data exist. Every number has machine-readable `provenance_flags` and displays them beside its report value: `ESTIMATED_WING_ONLY`, `USER_SUPPLIED`, `TEMPLATE_DEFAULT`, `NOT_ESTIMATED`, `OUTSIDE_COVERAGE`. Add **`DERIVED_FROM_INPUTS` only because calculated CG/ratios/mixed totals/trim/stall values fit neither supplied nor wing-only origins**; preserve source/dependency paths. The later shape helper adds `approximate: true` with its declared shape assumptions. (Owner choice 6, permitted necessary additions; ROADMAP provenance contract.)
- **Later-task specification gates, not unanswered S3-01 choices:** S3-05 must inventory required simulator coefficients and specify wing/non-wing assembly, reference normalization, moment shifts and coverage. A required profile/control/rate field outside the permitted supplied categories stays missing until explicitly specified; no new allowed category or zero coefficient is inferred. S3-10 must verify the selected shape/distribution formulas and sources before implementation. These do not block the authorized semantic model. (ROADMAP S3-05/S3-10; AGENTS, current-task stop condition.)
- **Execution boundary:** record decisions in this docs-only commit, then S3-01 is the authorized next implementation task. This turn writes no code or tests and starts no later task. (Owner choices 1-6 and final documentation-only instruction.)

## S3-01b Fix 2 — rudder fixture versus unchanged inventory expectation

- **Status:** ANSWERED with a narrow exception. The owner authorizes swapping
  only the rudder's hinge-endpoint entries in the fixture, expected-number
  inventory and ledger, preserving all assertions/sign expectations/other
  values/tolerances. Full-suite results must be recorded before and after;
  the correction has its own commit. No correction was made before approval.
- **Confirmed before any fixture correction:** `ControlKind::positive_deflection`
  is defined physically by D6, not hinge order. A test cloned each control,
  reversed endpoints and checked the same literal physical sign; it passed
  against original production code. The original sign assertions remain intact.
- **Owner's clarified ordering:** `(endpoint2-endpoint1) dot (parent tip-root)`
  must be positive, using actual body-axis geometry, including canted/inverted/
  ventral surfaces. The original rudder gave -5; reversed endpoints give +5. These
  are derived from the existing synthetic fixture, not new aircraft numbers.
- **What blocks:** the existing independent expected-number inventory also
  pins rudder coordinates. An isolated in-memory reversal, without altering any
  expected data, fails for `controls[rudder].hinge_end_body_m[2]`: actual
  `Some(2.0)`, expected `Some(4.0)`. The owner explicitly required STOP if any
  assertion's expected value would change; that includes this inventory
  assertion, not only the sign strings. The fixture was untouched at that checkpoint.
- **Options / trade-offs:** authorize changing only the rudder's expected
  inventory entries together with its fixture/ledger, preserving every
  assertion, sign expectation, other fixture number and tolerance (recommended);
  or stop after Fix 1 with the current fixture unchanged. There is no proposal
  to weaken/drop the inventory check or accept wrongly ordered hinges.
- **Actual scratch evidence:** `target/s3-01b/scratch/fixture-preflight.txt`:
  `rudder_sign_and_projection_preflight ... ok`;
  `unchanged_inventory_expectation_conflicts_with_rudder_correction ... FAILED`;
  `test result: FAILED. 1 passed; 1 failed; 0 ignored`.
- **Execution requirement:** show the exact expected_numbers diff and list all
  tests consuming the rudder fixture. If an unrelated assertion/expectation
  would change or a test fails for another reason, stop again without further
  edits. Source: owner's follow-up clarification to this entry.

- **Resolution:** the separately committed correction passed both complete
  suites (198 tests each); only six rudder coordinate entries changed, with
  155 other numeric records identical. Exact diff and all fixture consumers
  are recorded in S3_01B_VERIFICATION.md. Assertions and tolerances are intact.
  Fix 2 implements the actual-parent rule and tests ventral/inverted/canted
  cases. This question is answered; no S3-02 work is authorized or started.

## S3-01b Fix 4 — naming and later acceptance policy

ANSWERED by the owner's explicit policy: templates/gallery use neutral names
with registry plus review; user designs may name themselves freely; a
denylist cannot be complete and the report disclaimer limits claims. The
automatic every-object array-substitution test is mandatory for S3-02;
single-missing-component mass/CG rejection and its zero-substitution mutation
are mandatory for S3-03. ROADMAP, TASKS and ASSUMPTIONS record these as later
acceptance conditions; no loading, mass calculation, gallery or report task
starts in this batch. No further owner decision is needed for these fixes.

## 2026-11-20 — S3-01c prerequisite branch blocker — RESOLVED

The blocker recorded on the old `task/s3-01c` branch (docs-only commit
`f245334`) was the absence of S3-01/S3-01b from main. The owner chose to wait
for the reviewed work to be merged. Main now contains merge `850f143`, on
top of `1c62d85`, and both `crates/soas-designer` and
`docs/S3_01_SEMANTIC_MODEL.md` are tracked there. Ancestor verification also
confirms main contains `task/s3-01b`. **Status: RESOLVED.**

Per the owner's new instruction, a fresh `task/s3-01c2` starts at that main;
the old branch and its commit are not reused. Continue only Fixes 5-7,
tests then implementation per fix. S3-02 remains unstarted. Source: owner's
merge confirmation, local git tree/history and ancestor checks.

## S3-01c2 Fix 7 — shared-fixture containment correction — RESOLVED

The owner authorized changing only elevator/rudder raw hinge endpoints,
matching independent expected-number entries and ledger, in a separate
commit, preserving all other values/assertions/signs/tolerances. The old
points passed hinge order but lay before their parent root planes. Chosen
new points are exactly each parent's existing root and tip, on the inclusive
boundaries; aileron is unchanged. No non-coordinate expectation changed.

Both full suites passed (223 tests each), and the numeric snapshot changed
only nine raw coordinate rows out of 161. The correction is isolated in
`ae36257`; exact fixture/inventory diffs and all 43 fixture consumers are
in S3_01C_VERIFICATION. Root/tip and slightly-outside tests use independently
constructed geometry and do not depend on the corrected shared fixture.
Status: RESOLVED. No S3-02 work starts.

## 2026-11-21 — S3-02 stable station IDs: prerequisite base — RESOLVED

- **Task:** task/s3-02-ids, stable station ID model change only.
- **What blocked originally:** the owner requested a branch from main using the
  approved S3-02a contract. Main was still at 72ac5bf: it contained
  soas-designer but not S3_02_PLAN.md or DECISIONS D15-D22. The contract
  commit d16745a is on docs/s3-02-plan and is not an ancestor of main.
- **Options with trade-offs:** wait for the owner to merge the reviewed
  documentation into main (recommended; preserves the requested base), or
  explicitly authorize creating task/s3-02-ids from docs/s3-02-plan
  (available contract immediately, but changes the requested base).
- **Status:** RESOLVED after the owner's merge confirmation. Verified main
  at 6c63ab2 contains d16745a as an ancestor, S3_02_PLAN.md in its tree and
  DECISIONS D15-D22; 6c63ab2 also commits this original blocker log. Created
  fresh task/s3-02-ids from that main. Continue only the authorized station-ID
  model/tests and separate fixture/ledger migration; no reader/writer or other
  S3-02 child is authorized.
