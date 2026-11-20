# Project Brief: Showdown of Aerial Systems (SOAS)

> Written 2026-11-13 as a full-context briefing for a new AI agent joining this project.
> **This brief is a snapshot.** The repo files are authoritative. If this brief conflicts with
> `AGENTS.md`, `docs/CONVENTIONS.md`, `docs/DECISIONS.md` or `docs/TASKS.md`, those files win.
> Report the discrepancy to the owner instead of silently choosing one.

---

## 0. How to use this document

**Read order at the start of every session:**
1. `AGENTS.md` (standing rules; binding)
2. `docs/CONVENTIONS.md` (axes, units, signs; binding)
3. `docs/DECISIONS.md` (what is decided and what is OPEN)
4. `docs/TASKS.md` (the task queue)
5. `docs/ROADMAP.md`, `docs/VISION.md`, `docs/ASSUMPTIONS.md`, `docs/QUESTIONS.md`
6. This brief, for the "why" behind all of the above

This brief explains context and reasoning. It is not a task list. Do only the task you are given.

---

## 1. The project in one paragraph

SOAS is an open-source, browser-friendly aircraft design and flight simulation tool written in Rust.
**One aircraft description feeds two outputs:** an engineering analysis ("does it fly?") and a flyable
simulation. The user designs an aircraft (initially fixed-wing, later supersonic jets and VTOL),
gets an honest, validated conceptual-design report (stability, stall speed, performance, mass/CG),
and can fly the design. The headline demo is an **F-104-class supersonic jet**.

```
Aircraft description (wings, tail, fuselage, engines, mass, gear)
        |
        +--> Aero estimator --> coefficients --> Validation report ("does it fly?")
        |                              |
        |                              v
        +----------------------> Flight sim core --> you fly it
```

Everything else in the project is a feature layered on that loop.

---

## 2. The owner and how to work with them

- **Background:** new to **both Rust and flight mechanics**. Learning while building.
- **Workflow:** AI agents write most of the code; the owner reviews, decides, and learns.
- **Time:** 20+ hours per week.
- **Goals:** a project they are proud of, good enough that it becomes well known and people want to use it.
  Fame is not controllable; quality, honesty, and an instant browser demo raise the odds.
- **Environment:** Windows; Codex as the coding agent; a local git repo; a **private** GitHub repo
  (being set up); license undecided (see D2).
- **Stated preference, repeatedly:** *do not assume; ask questions when something is unclear.*

**What this means for you:**
- Explain changes in plain language. Define aerospace and Rust terms on first use.
- Never guess. A question beats a wrong assumption.
- Keep diffs small and readable. The owner must be able to review your work.
- Never invent a number. Every physical constant or coefficient needs a cited source.
- Be honest about uncertainty and limits. Do not call anything "accurate" without saying what it was
  compared against and the error.
- Do not take an OPEN decision yourself. Present options with trade-offs and wait.
- Note that "you do it" from the owner is acceptable for logistics, not for decisions listed as OPEN.

---

## 3. Product concept

### Audience
- Builders, RC hobbyists, and students asking "will my idea fly, and how will it behave?"
- Players who like building things (inspiration: Flyout-style plane builders).
- Engineers wanting a quick conceptual-stage sanity check.

### The existing landscape (do not claim novelty we do not have)
- Analysis tools: OpenVSP (NASA), AVL (MIT), XFLR5.
- Flight dynamics: JSBSim (open source).
- Commercial browser tools combining design and simulation exist, e.g. SIMNET Designer
  (parametric components for drones, fixed-wing and VTOL, own physics engine, free demo).
- Academic tools pair vortex-lattice aerodynamics with flight simulators.

### Differentiator
**Open source, free to try in a browser, every number traceable and validated, a game-like feel, and one
file serving both analysis and simulation.**

### Accuracy stance (important)
- Target: **conceptual-design fidelity**: good for "does it fly, how fast does it stall, is it stable",
  with typical errors of tens of percent. The same class as AVL/OpenVSP.
- **Not** DCS-level fidelity. That depends on manufacturer data and flight-test tables for one specific aircraft.
- Every release publishes a validation page: predicted vs. reference values, error plots, and a list of
  what the model ignores (`docs/ASSUMPTIONS.md`).
- Transonic flight (about Mach 0.8 to 1.2) is **low confidence** and must be flagged as such in reports and UI.
- Reports carry a "conceptual estimates only, not for certification or flight safety" notice.

---

## 4. Scope

### In scope, by release (details in section 8)
v0.1 subsonic fixed-wing core, estimator, designer, report, browser demo (Mach-aware design);
v0.2 supersonic (F-104-class); v0.3 VTOL; v0.4 3D model import and real-life export;
v0.5 cockpit and systems; v0.6 graphics and terrain; v1.0+ combat and multiplayer.

### Explicitly not now
Terrain streaming, multiplayer, weapons, AI, a custom wgpu renderer, volumetric clouds, audio,
mission editor, modding. These are deferred, not forgotten. New ideas go to the **Parking lot** in
`docs/ROADMAP.md`, never into code mid-release.

### Owner's ranking of post-v0.3 work
1. Import 3D models and real-life export; 2. Cockpit and aircraft systems; 3. Graphics and terrain;
4. Combat and multiplayer.

---

## 5. Architecture

- **Language:** Rust, stable toolchain, edition 2024.
- **Workspace:** one crate exists today, `crates/soas-core`. Create other crates only when a task needs them.
  Planned later: `soas-cli` (headless runner), `soas-estimator`, `soas-designer`, `soas-web` (WASM).
- **`soas-core` is pure.** No file I/O, rendering, networking, threads, wall-clock time, or unseeded
  randomness. No iteration over `HashMap` in simulation logic. Fixed time step. Deterministic on one
  machine and binary (cross-platform bit-identical results are **not** promised).
- **State is serializable.** Inputs are the only external drivers of the simulation. (This keeps
  multiplayer possible later at almost no cost now; it is very expensive to retrofit.)
- **Rendering stays outside the core.** The viewer stack is undecided (see D5): Rust-native (Bevy) vs. a JS
  viewer over a WASM core. Decide at Stage 4, not earlier. Convert from aerospace axes to render axes
  in exactly one place.
- **The aircraft file is the contract** between designer, estimator, simulator, and (later) renderer.
  Format: **JSON** with a `format_version` field and unit suffixes in field names. It supports
  coefficient **tables indexed by Mach** (and altitude where needed) with documented interpolation.
- **Source of truth for a design is a semantic aircraft model** (wing sections, control surfaces, engines,
  tanks), from which geometry is derived. Do not derive semantics from anonymous CAD solids.
- **Dependencies pre-approved:** `nalgebra`, `serde` (with derive), `serde_json`, `thiserror`, `approx`
  (dev), `proptest` (dev). Ask before adding anything else. No `unsafe` without approval.
- **CAD:** SolidCraft (pure Rust, MIT OR Apache-2.0, desktop and web, headless CLI, MCP server) is a
  *later* dependency (v0.4), driven through its **CLI and command scripts**, not its Rust internals.
  It is early software whose file formats change between releases; pin a version.
  Mesh imports (glTF) become mesh bodies: they can be rendered, measured, moved and exported, but they do not
  support solid features. Raw model import therefore needs a "tag which parts are wing/tail/fuselage" step.

---

## 6. Conventions (summary; `docs/CONVENTIONS.md` is binding)

- **Units:** SI internally (m, kg, s, N, Pa, K, rad). Unit suffixes in names where ambiguous (`span_m`,
  `alpha_rad`). Newtypes for angles only (`Radians`, `Degrees`) with tested conversions. Non-SI units
  (deg, knots, ft) only at the UI/report boundary. Adding any other newtype or the `uom` crate needs owner approval.
- **Body frame:** x forward, y right, z down. Dynamics origin at the CG.
- **World frame:** NED, flat non-rotating Earth for v0.1; altitude `h = -z`.
- **Attitude:** unit quaternion (Hamilton) rotating body vectors into NED; renormalize every step. Euler
  angles (3-2-1) for I/O and display only.
- **Airflow angles:** `alpha = atan2(w, u)`, `beta = asin(clamp(v/V, -1, 1))`. Below
  `MIN_AIRSPEED_FOR_ANGLES_MPS` (placeholder 0.1 m/s, to be revisited) return neutral fallback angles of 0.
  Non-finite inputs are an explicit error, never silently clamped.
- **Forces:** computed in instantaneous wind/stability axes and explicitly transformed to body axes.
  Wind-axis force components are `(-drag, +side, -lift)`. The transformation has its own tests.
- **Moments:** roll L about x, pitch M about y, yaw N about z; positive pitching moment = nose up.
- **Control signs:** positive elevator = trailing edge down (nose-down moment); positive aileron = left
  trailing edge down and right trailing edge up (positive rolling moment); positive rudder = trailing edge
  left (nose-left yaw moment). **The owner still needs to verify these against a textbook before T6.**
  Every surface needs a sign test.
- **Inertia:** signed tensor entries about the CG in body axes (off-diagonals are true tensor entries, e.g.
  `Ixy = -integral(x*y dm)`), with a constructor from products of inertia that negates correctly, tested.
- **Atmosphere and gravity:** ISA from 0 to 20 km (error outside that range), no wind; constant
  `g0 = 9.80665 m/s^2`.
- **Numerics:** fixed time step, always a parameter, default `1/400 s`; RK4. The T4 step-size convergence
  test may justify revising the default; record any change as a new decision.

---

## 7. Decisions

`docs/DECISIONS.md` is authoritative and its IDs may have been extended beyond those below.

| Topic | Status | Summary |
|---|---|---|
| D1 Project name | DECIDED | "Showdown of Aerial Systems"; repo slug `showdown-of-aerial-systems`; crate prefix `soas` ("soas" = Showdown of Aerial Systems) |
| D2 License | **OPEN by owner choice** | Repo is private. No LICENSE file. **Must be decided before the repo is made public.** Leaning: MIT OR Apache-2.0 |
| D3 File format | DECIDED | JSON, with `format_version`, unit suffixes in field names |
| D4 Units | DECIDED | Option B (SI + suffixes + angle newtypes) |
| D5 Web viewer stack | DEFERRED | Decide at Stage 4 |
| D6 Control signs | DECIDED (owner to verify) | As in section 6 |
| D7 SolidCraft | DEFERRED | CLI-driven, v0.4, pinned |
| D8 Supersonic validation aircraft | OPEN | Candidate: F-104 via NASA CR-96008; pending owner task H4 |
| D9 Validation tolerances | OPEN | Needed before T10, after seeing reference data. Proposal only: about +/-10% on stall speed |
| Time step, inertia sign, axes, near-zero airspeed | DECIDED | Section 6 |
| Hero aircraft | DECIDED (D14 per repo) | F-104-class supersonic jet |

The batch stop rule only halts on a decision **that the current task requires**. D2, D8, D9 do not block T1 to T9.

---

## 8. Roadmap and task plan

Effort figures are **guesses** at 20+ hours/week for a beginner, not predictions.

| Release | Scope | Guess |
|---|---|---|
| **v0.1** | Subsonic core, estimator, designer, report, browser demo; Mach-aware; "Mach > 0.8 not modeled" warning | 4-6 months |
| **v0.2** | Supersonic, F-104-class | 2-4 months |
| **v0.3** | VTOL (quadplane) | 3+ months |
| **v0.4** | Import 3D models, real-life export, landing gear, structural/mass/cost estimates | 3-4 months |
| **v0.5** | Cockpit and aircraft systems | 4-6 months |
| **v0.6** | Graphics and terrain (against a written visual target) | 3-6 months |
| **v1.0+** | Combat, multiplayer, mission editor, modding | 6-12+ months |

Rules: every release stands alone and is worth showing; no new features mid-release; the whole vision is
likely 2.5 to 3.5+ years.

### v0.1 stages
- **Stage 0 (learn and set up):** tooling, learning, conventions, repo, CI. T0, T1, T0b done.
- **Stage 1 (simulation core in Rust):** batches below.
- **Stage 2 (subsonic estimator):** written after Stage 1.
- **Stage 3 (designer model and report).**
- **Stage 4 (browser demo, "graphics-lite").**
- **Stage 5 (launch).**

### Task queue (authoritative detail: `docs/TASKS.md`)
Human tasks: **H1** tooling; **H2** learning; **H3** reference data (candidate: Navion from NASA CR-96008,
fallback Cessna 172; propulsion parameters not required for v0.1); **H4** verify that NASA CR-96008's F-104
and Navion sections contain geometry, mass, inertia and derivatives versus Mach.

| Batch | Tasks | Gate before next batch |
|---|---|---|
| done | T0 workspace; T1 CI; T0b rename | Hosted CI run not yet observed |
| **C** | T2 rotations + `Radians`/`Degrees`; T3 ISA atmosphere; T4 rigid-body dynamics, RK4, convergence test | Owner checks ISA values against a table; reads convergence report |
| **D** | T5 JSON aircraft format with Mach tables; T6 Mach-aware aero model, table interpolation, per-surface sign tests, axis-transformation tests; T7 headless runner + determinism test; T8 trim solver + static margin | Owner verifies control signs |
| **E** | T9 turbojet + afterburner thrust tables; T10 compare to reference (power-off cases) | Needs H3, H4, D9 |
| **F** | T11 data-driven F-104-class aircraft in the headless sim, with source-axis to body-axis transformations | Needs H4 |

### Stage 2 plan (tasks to be written when Stage 1 finishes)
1. Parametric geometry structs (wing sections, taper, sweep, twist, tail, fuselage).
2. Lifting-line theory, tested against the elliptic-wing analytic case.
3. Compressibility correction (Prandtl-Glauert) up to roughly Mach 0.7-0.8.
4. Vortex-lattice method for arbitrary lifting-surface layouts.
5. Tail contribution and neutral point (downwash, tail efficiency).
6. Drag buildup (skin friction, form factors, wetted areas).
7. Validation against the reference aircraft, with AVL/OpenVSP output as extra references.
8. Published error plots. Aim for "within X%, and here is where it is worse."

### Stage 3-5 plan
Parametric parts as source of truth; mass/CG buildup; validation checks (static margin, stall speed, wing
loading, thrust-to-weight, Mach warnings); CLI producing the aircraft JSON and an HTML report; template
aircraft (a light plane and an F-104-class jet). Browser demo: core compiled to WASM, sliders, live report,
keyboard/gamepad flight, free static hosting. Launch: README with GIF, gallery, validation page,
contribution guide, license decision.

---

## 9. Technical background for later releases

### Why Mach-aware from the start
The hero is supersonic. The aerodynamic model takes Mach and altitude from T6 onward and the aircraft
file supports coefficient tables versus Mach, so a data-driven F-104-class aircraft can fly in the sim
**before the estimator exists**. The estimator's goal then becomes: reproduce published data from geometry.

### Engines
Jets are simpler than propellers here: T9 is a turbojet with afterburner, with **thrust as a table of Mach,
altitude and throttle**, plus fuel flow (cite sources, no invented numbers). A piston-plus-propeller model
is deferred. Note for any future propeller model: thrust is not `power / speed` (diverges at zero airspeed).

### Supersonic (v0.2)
- The estimator becomes **separate Mach-regime modules behind one interface**: subsonic, a transonic blend
  flagged **low confidence**, and supersonic.
- Supersonic methods: linearized supersonic theory (Ackeret-type for airfoils, supersonic wing theory),
  wave drag via the area rule (Sears-Haack body as the minimum-drag baseline), aft shift of the aerodynamic
  center with Mach, control effectiveness varying with Mach.
- Sim needs: variable stability and control effectiveness vs. Mach, afterburning turbojet with simple inlet
  effects, ISA to operating altitudes.
- Known hard part: supersonic directional stability. NASA flight tests of an F-104-type airplane reported that
  a ventral fin was needed for directional stability in the supersonic range.
- Vortex-lattice methods are subsonic and attached-flow only. Do not extrapolate them past their validity.

### VTOL (v0.3)
Blade-element or actuator-disk rotor model, hover-to-forward transition, and a **hover attitude controller**
(hovering aircraft are unstable without one), plus ground effect basics. Quadplane is the likely hero.
Validation data is thinner, so rely on analytic cases and published rotor data and state the limits.

### Later (v0.4 to v1.0+)
v0.4: SolidCraft CLI integration, tagging UI, landing-gear spring-damper (a notorious source of numerical
stiffness; add only after the in-air model is trustworthy), empirical structural weight and rough cost.
"Worth building in real life" needs more than aerodynamics: structures, power/fuel budgets, cost, buildability.
v0.5: systems framework (fuel, electrical, hydraulics) mapped to designer parts. v0.6: heightmap terrain,
sky, PBR; check licensing for any real-world elevation or imagery data. v1.0+: client-server multiplayer
with snapshots (do not rely on cross-platform bit-identical lockstep).

---

## 10. Validation philosophy

**Ground-truth hierarchy:** analytic cases > published data > reference tools (JSBSim, AVL, OpenVSP) > nothing.
Never trust plausible-looking output.

- Analytic tests: free fall; constant spin preserves quaternion norm; torque-free symmetric-body rotation
  conserves angular momentum magnitude; RK4 error scales as expected with step size; elliptic wing gives
  induced-drag efficiency 1; Ackeret theory; Sears-Haack minimum wave drag.
- Reference tools: run them, save outputs as golden test files. Check each tool's license before committing
  any of its files or data. Copying their code is not acceptable without a license review.
- Published data: for v0.1, power-off cases only (stall, glide, trim, static margin).
- ISA sanity values (cite a table when testing): sea level 288.15 K, 101325 Pa, 1.225 kg/m^3; at 11 km about
  216.65 K, about 22632 Pa, about 0.364 kg/m^3.
- Tests must be able to fail. Show red then green. Do not weaken a test to make it pass without explaining
  why the test was wrong.

### Candidate public data sources (all from NASA NTRS unless noted; **sufficiency not yet verified, see H4**)
- **NASA CR-96008**, *Aircraft Stability and Control Data* (Teper, 1969; NTRS 19690022405): stability and
  control data for several aircraft including the F-104, T-38, B-58 and Navion, with derivatives versus Mach
  number and altitude and geometry. **Derivatives are given in the axis system of the data source**, so
  explicit axis transformations are needed.
- **NACA RM SL56H06**: longitudinal and lateral stability of a scale model of the F-104A (NTRS 20090026345).
- Flight-determined derivatives of a supersonic airplane with a low-aspect-ratio unswept wing and tee tail,
  Mach 0.88 to 2.08 (NTRS 19630004010). This appears to be the F-104 configuration; confirm.
- Comparison of flight and wind-tunnel derivatives for two supersonic fighters (NTRS 19660022438).
- NASA CR-2144, *Aircraft handling qualities data*, ten airplanes (NTRS 19730003312).
- Wind-tunnel stability derivatives of an unswept-wing model with high horizontal tail, Mach 0.65 to 2.2
  (NTRS 19980228050).
- The F-104 is attractive as a target: straight trapezoidal wing and T-tail rather than a complex delta or
  blended layout. Transonic mismatch between estimator and data is expected and must be documented.

---

## 11. Working agreement (details in `AGENTS.md` and `docs/WORKFLOW.md`)

- **One task at a time**, in order, from `docs/TASKS.md`. Tests first. Quality gates before "done":
  `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`.
- **Batch mode:** for each task: tests first, implement, run gates, one local commit `Task <ID>: <title>`,
  update status in TASKS.md, add a "Concepts introduced" entry to LEARNING.md (owner columns stay blank).
  Never push, merge, or change remotes.
- **Stop immediately** (no further tasks) if: a decision the current task requires is OPEN; a number has no
  source; a gate still fails after 3 honest fix attempts; a test would have to be weakened; the task needs
  something not specified; or a non-approved dependency is needed. Append task, blocker, and options to
  `docs/QUESTIONS.md`.
- **Report format:** what changed; files touched; tests (what each checks and why it can fail); assumptions
  and unknowns; sources cited; concepts to understand (2-3 bullets); questions for the owner. For batches,
  one section per task, then "Least confident about", "Assumptions I made", "Commands for the owner to verify".
- **Owner loop per batch:** read the report and QUESTIONS.md, run the gates, skim the code, run a read-only
  review pass in a fresh session (look for sign errors, frame mix-ups, unit mistakes, uncited numbers, tests
  that cannot fail, determinism hazards, new dependencies, scope creep, unsupported accuracy claims),
  fill in LEARNING.md, push, confirm CI.
- **Never:** add features "while you're there"; hide failures or skip tests; commit secrets, large binaries,
  or files whose license you have not checked; touch `VISION.md` or `CONVENTIONS.md` without being asked.

---

## 12. Environment notes

- Windows. The repo lived on an `A:` drive where hardlinks are unsupported; incremental compilation had to be
  disabled (`CARGO_INCREMENTAL=0`). The owner was advised to move the project to an internal NTFS drive.
  If builds behave oddly, say so rather than working around it silently.
- CI: `.github/workflows/ci.yml` (Ubuntu, stable Rust: fmt check, clippy `-D warnings`, tests) on push and
  pull request. **A hosted run has not been observed yet.**
- Git: local commits so far: documentation (`524ed44`), T1 (`cf3969c`), T0b (`75797f0`) at time of writing.
  Nothing is pushed yet. The GitHub repo is private and being created.
- Codex cloud, if used: agent internet access is off by default after setup; Cargo needs crates, so use the
  allowlist or a setup script. Non-interactive runs cannot ask questions, so use `docs/QUESTIONS.md`.

---

## 13. Status at time of writing (2026-11-13)

- **Done:** project docs; decisions listed in section 7; Rust 2024 workspace with `soas-core` (one smoke
  test); CI workflow file; rename to `soas-core`; batch-mode rules; direction change to an F-104-class jet.
- **Next:** Batch C (T2, T3, T4), sent on its own and reviewed before Batch D.
- **Owner's pending items:** verify local work; push to private GitHub and confirm CI; H4 (read NASA
  CR-96008); H3 (reference data); Rust and flight-mechanics reading; verify control-sign conventions against
  a textbook before T6; decide D9 before T10.

---

## 14. Domain pitfalls to guard against

1. **Frame and sign errors** (body vs. NED vs. render axes; wind vs. stability vs. body axes; control
   deflection signs). The most common cause of "the plane flips". One convention, tests per axis.
2. **Source-axis mismatch** in published data (CR-96008 states derivatives are in the source's own axes).
3. **Units** (degrees vs. radians especially; knots/feet only at the boundary).
4. **Quaternion drift** and Euler-angle singularities near pitch +/-90 degrees.
5. **Numerical issues:** integrator choice, step size, stiff springs (landing gear), floating-point at high speed.
6. **Determinism:** same-machine reproducibility is realistic; cross-platform bit-identity is not.
7. **Zero-airspeed behaviour:** division by `V`, `asin` domain overshoot from round-off.
8. **Post-stall, spins, large angles of attack:** crudely modelled in v0.1; document as an assumption.
9. **Transonic region:** low confidence; never present numbers there as precise.
10. **Overclaiming:** especially for the "worth building in real life" feature.
11. **Dependency churn and licenses:** SolidCraft evolves quickly; JSBSim/AVL/OpenVSP terms need checking.
12. **Scope creep:** the vision is huge; the roadmap exists to protect the first release.

---

## 15. What is verified vs. not

**Verified (via sources read during planning):** SolidCraft is pure Rust, dual MIT/Apache-2.0, with CLI, web and
MCP, and describes itself as early but usable for simple parts; the NASA reports above exist and cover the
aircraft described; SIMNET Designer exists as described; CR-96008's introduction states the source-axis caveat.

**Not verified (treat as assumptions):**
- That the F-104 and Navion data are *sufficient* for a full validation (H4).
- JSBSim's exact license terms and whether data derived from it may be committed.
- Availability and terms of Beard & McLain's UAV textbook ("believed free from the authors").
- Exact Codex CLI flags and defaults (they change between versions).
- The effort estimates and the "famous" outcome.
- The control-surface sign conventions until the owner checks them against a textbook.
- The `1/400 s` default time step (to be informed by the T4 convergence test).
- The `0.1 m/s` near-zero-airspeed threshold (placeholder).

---

## 16. Glossary of project terms

- **SOAS:** Showdown of Aerial Systems. **soas-core:** the pure simulation crate.
- **H-tasks:** human/owner tasks. **T-tasks:** agent tasks in `docs/TASKS.md`. **D-items:** decisions in `docs/DECISIONS.md`.
- **Batch mode:** the multi-task run protocol with stop rules (see `AGENTS.md`).
- **Parking lot:** the end of `docs/ROADMAP.md`, where out-of-scope ideas go.
- **Hero aircraft:** the F-104-class supersonic jet used for the main demo and the v0.2 validation.
- **Conceptual-design fidelity:** accuracy good enough to decide "does it fly and roughly how well", not to certify.
- **6-DoF:** six degrees of freedom (3 translations, 3 rotations).
- **ISA:** International Standard Atmosphere.
- **VLM:** vortex-lattice method (subsonic lifting-surface aerodynamics).
- **Static margin:** distance between the CG and the neutral point as a fraction of mean chord; positive means pitch-stable.
