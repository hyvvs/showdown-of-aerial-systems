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
