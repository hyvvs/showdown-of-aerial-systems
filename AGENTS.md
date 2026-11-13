# AGENTS.md — standing instructions for AI coding agents

Read this file at the start of every session. It is short on purpose.

## What this project is
Project name: **Showdown of Aerial Systems** (see docs/DECISIONS.md D1).
An open-source, browser-friendly aircraft design + flight simulation tool, written in Rust.
One aircraft description produces (a) an engineering analysis ("does it fly?") and (b) a flyable simulation.
Full vision: `docs/VISION.md`. Current plan: `docs/ROADMAP.md`. Task queue: `docs/TASKS.md`.

## Who you are working with
The owner is **new to both Rust and flight mechanics** and is learning while building.
Therefore:
- Explain what you did in plain language. Define any aerospace or Rust term the first time you use it.
- **Never assume.** If a requirement, convention, number, or intent is unclear, stop and ask. A question is better than a guess.
- Prefer simple, readable code over clever code. Comment the *why*, especially for physics.
- The owner must be able to review your work. Keep each change small enough to read.

## Session start
Read, in order: this file, `docs/CONVENTIONS.md`, `docs/DECISIONS.md`, `docs/TASKS.md`, then `docs/ROADMAP.md` and `docs/VISION.md` for context.

## Hard rules
1. **One task at a time.** Do only the task you were given from `docs/TASKS.md`. Anything else you notice goes in the "Parking lot" at the bottom of `docs/ROADMAP.md`, not into code.
2. **Never invent numbers.** Every physical constant, coefficient, or reference value must have a source cited in a comment or in `data/reference/`. If you do not have a source, write `TODO(source)` and tell the owner. Do not "fill in plausible values".
3. **Tests first, and tests that can fail.** Write the test (analytic result, published value, or reference-tool output) before the implementation. A test that cannot fail is worthless.
4. **Conventions are law.** Follow `docs/CONVENTIONS.md` exactly (axes, units, signs, angle units). If you think a convention is wrong or missing, ask; do not silently deviate.
5. **Dependencies.** Pre-approved: `nalgebra`, `serde` (with derive), `serde_json`, `thiserror`, `approx` (dev), `proptest` (dev). Ask before adding anything else, and say why.
6. **No `unsafe`** unless the owner approves. If approved, every block gets a `// SAFETY:` comment.
7. **Keep `soas-core` pure.** No file I/O, rendering, networking, threads, wall-clock time, or unseeded randomness in the simulation core. No iteration over `HashMap` in simulation logic (non-deterministic order). Use fixed time steps.
8. **Do not overclaim.** When you add a model simplification, add it to `docs/ASSUMPTIONS.md`. Never describe results as "accurate" without saying what they were compared against and the error.
9. **Do not touch** `docs/VISION.md` or `docs/CONVENTIONS.md` without asking; propose edits instead.

## Quality gates (all must pass before you say a task is done)
```
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```
Rust stable, edition 2024. Fix warnings; do not silence them without explanation.

## Repo layout (create crates only when a task needs them)
```
crates/soas-core/    simulation core: math helpers, atmosphere, rigid body, aerodynamic model (pure)
crates/soas-cli/     headless runner (later task)
data/reference/      reference data and its sources (never fabricated)
docs/                project docs
```
Later crates (do not create early): `soas-estimator`, `soas-designer`, `soas-web`.

## Batch mode
- Run the listed tasks in order. For each task: write tests first, implement, run the three quality gates, make one local git commit `Task <ID>: <title>`, set its status in `docs/TASKS.md`, and add a **Concepts introduced** entry to `docs/LEARNING.md` (the owner's **Understood** and **Still unclear** columns stay blank).
- Never push, merge, or change remotes.
- Stop immediately, with no further tasks, if: a decision that the current task requires is OPEN; a number has no source; a gate still fails after 3 honest fix attempts; a test would have to be weakened; the task needs something not specified; or a non-approved dependency is needed. Never guess to keep going. D2 (license), D8 and D9 are not required by T1 to T9 and must not stop those tasks.
- When stopping, append to `docs/QUESTIONS.md`: task, what blocked, and options with trade-offs.
- At the end of a batch, give one report with a section per task in the usual format below, then: **Least confident about**, **Assumptions I made**, and **Commands for the owner to verify**.

## When you finish a task, report in this format
1. **What changed** (plain language, 5 lines max)
2. **Files touched**
3. **Tests**: what each new test checks and why it can fail; paste the test summary
4. **Assumptions and unknowns** (anything you were unsure about)
5. **Sources cited** (new references added)
6. **Concepts to understand**: 2 to 3 bullets explaining the key ideas in this change, so the owner can learn from it. In `docs/LEARNING.md`, the agent lists **Concepts introduced** only; the owner fills in **Understood** and **Still unclear**.
7. **Questions for the owner** (if any)
Then update the status in `docs/TASKS.md` and add a short entry to `docs/LEARNING.md`.

## Never
- Add features "while you're there".
- Change a test to make it pass without explaining why the test was wrong.
- Hide failures, skip tests, or weaken tolerances silently.
- Commit secrets, large binaries, or files whose license you have not checked.
