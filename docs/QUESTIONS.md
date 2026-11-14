# Questions for the owner

When a batch stops, append an entry with the task, what blocked it, and options with trade-offs. Do not guess an answer or continue to further tasks.

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
