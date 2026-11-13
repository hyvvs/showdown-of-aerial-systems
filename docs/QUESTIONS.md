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
