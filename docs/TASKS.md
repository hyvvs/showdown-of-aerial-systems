# Task queue

Status: [ ] todo, [~] in progress, [x] done. Do tasks **in order**, one at a time.
Later tasks are written just in time, after earlier ones teach us what we need.
`H` tasks are for the human owner; agents may help structure them but must not invent data.

---
## H1 (owner): Tooling
Install Rust via rustup, git, and a GitHub account. Create the repo, copy this kit in, commit.
Done when: `cargo --version` and `git --version` work.  [ ]

## H2 (owner): Learning, first pass
See docs/LEARNING.md, weeks 1 to 2.
Done when: you can answer the self-check questions there in your own words.  [ ]

## H3 (owner): Reference data for the Cessna 172
Collect published Cessna 172 numbers (stall speeds, climb rate, cruise speed, geometry, weights) into `data/reference/c172/README.md`, with a **source for every number**. Also choose validation tolerances (decision D9).
The agent may help format the file but must not supply numbers from memory.  [ ]

---
## T0: Workspace skeleton
Goal: a compiling Rust workspace with one library crate.
Deliverables: root `Cargo.toml` (workspace), `crates/aero-core` (lib, no dependencies yet), `rust-toolchain.toml` (stable), `.gitignore`, a short `README.md` saying the name is a placeholder. **No LICENSE file** (decision D2 is open).
Acceptance: the three quality gates in AGENTS.md pass, with at least one trivial test.  [x]

## T1: Continuous integration
Goal: GitHub Actions runs fmt, clippy (`-D warnings`), and tests on every push and pull request.
Acceptance: workflow file added; explain in plain language what each step does.  [ ]

## T2: Rotation and attitude helpers
Goal: body-to-NED rotation with quaternions; Euler angle conversion (3-2-1) for I/O only.
Tests (must be able to fail): known 90-degree rotations map axes as expected; Euler to quaternion to Euler round-trips; quaternion stays unit length after many composed rotations; document behaviour near pitch of +/-90 degrees.
Notes: use the conventions in docs/CONVENTIONS.md exactly. Ask if anything is ambiguous.  [ ]

## T3: ISA atmosphere
Goal: temperature, pressure, density, speed of sound as functions of altitude (0 to 20 km), plus Mach number and dynamic pressure.
Tests: compare against a **cited** ISA table (e.g. US Standard Atmosphere 1976 / ICAO). Sanity values the owner will check by hand: sea level 288.15 K, 101325 Pa, 1.225 kg/m^3; at 11 km about 216.65 K, about 22632 Pa, about 0.364 kg/m^3. Altitudes beyond range return an error, not a guess.  [ ]

## T4: Rigid-body dynamics (no aerodynamics yet)
Goal: 6-DoF equations of motion in body axes, with a fixed-step RK4 integrator and a state struct that can be serialized later.
Tests: free fall matches the analytic solution; torque-free rotation of a symmetric body conserves angular momentum magnitude; quaternion norm drift stays within a stated tolerance over very many steps; halving the step size reduces integration error by roughly the expected factor for RK4.
Notes: explain the equations of motion in docs/LEARNING.md in plain language.  [ ]

## T5: Aircraft file format v0 (requires decisions D3 and D4)
Goal: define and load a minimal aircraft description: mass, inertia, CG, reference area/span/chord, simple linear aerodynamic coefficients.
First step: agent proposes options for D3/D4 with trade-offs and **waits**. Then implement.
Tests: round-trip load/save; invalid files give clear errors; units are explicit in field names.  [ ]

## T6: Simple aerodynamic model (requires decision D6)
Goal: forces and moments from `CL = CL0 + CLalpha*alpha` (limited at CLmax), `CD = CD0 + CL^2/(pi*e*AR)`, `Cm = Cm0 + Cmalpha*alpha + Cmde*de`; transform wind to body axes; apply to the rigid body.
Tests: steady glide gives the expected lift-to-drag ratio; level-flight force balance at the computed trim; the stall-speed formula matches the simulated 1-g stall onset within a stated tolerance; transformation tests for each axis sign.
Update docs/ASSUMPTIONS.md.  [ ]

## T7: Headless runner
Goal: `aero-cli run scenario --out run.csv` writes time histories.
Tests: same input produces byte-identical output on one machine (determinism test).  [ ]

## T8: Trim solver and static stability numbers
Goal: find trim alpha and elevator for a given airspeed; report static margin.
Tests: hand-made aircraft with a known analytic answer; trim results make forces and moments balance to a tight tolerance.  [ ]

## T9: Engine and propeller model v0
Goal: simple piston engine plus fixed-pitch propeller giving thrust versus airspeed (not `power / speed`).
Notes: agent must cite its propeller model and list parameters needing real data; no invented numbers.  [ ]

## T10: Compare against reference (needs H3)
Goal: tests and a small plotting helper comparing C172 results to the reference data and to JSBSim output the owner generates.
Notes: Python is allowed here **only for plotting from CSV**. The owner checks JSBSim's license before any JSBSim files or derived data are committed.
Done when: a comparison table and plots are committed and docs/ASSUMPTIONS.md "Validation status" is filled in.  [ ]

---
Stage 2 tasks (lifting line, vortex lattice, validation) will be written when T10 is done.
