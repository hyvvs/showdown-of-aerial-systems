# Task queue

Status: [ ] todo, [~] in progress, [x] done. Do tasks **in order**, one at a time.
Later tasks are written just in time, after earlier ones teach us what we need.
`H` tasks are for the human owner; agents may help structure them but must not invent data.
H1 and H2 run alongside development. H3 and D9 are required only before T10; T0 to T9 are not blocked by owner tasks.
H4 is required before T11. v0.1 remains subsonic, with an explicit "Mach > 0.8 not modeled" warning; the F-104-class supersonic hero demo belongs to v0.2.

---
## H1 (owner): Tooling
Install Rust via rustup, git, and a GitHub account. Create the repo, copy this kit in, commit.
Done when: `cargo --version` and `git --version` work.  [ ]

## H2 (owner): Learning, first pass
See docs/LEARNING.md, weeks 1 to 2.
Done when: you can answer the self-check questions there in your own words.  [ ]

## H3 (owner): Power-off reference data
Candidate first reference aircraft: the Navion from [NASA CR-96008](https://ntrs.nasa.gov/citations/19690022405); fallback: Cessna 172. Collect published geometry, loading and aerodynamic data plus power-off stall, glide, trim and static-margin comparison cases into `data/reference/<selected-aircraft>/README.md`, with a **source for every number** and missing inputs identified. Also choose validation tolerances (decision D9). Propulsion parameters are not required for v0.1.
The agent may help format the file but must not supply numbers from memory.  [ ]

## H4 (owner): Audit F-104 and Navion source coverage
Check whether the F-104 and Navion sections of Gary L. Teper's [NASA CR-96008, *Aircraft Stability and Control Data* (1969)](https://ntrs.nasa.gov/citations/19690022405) contain geometry, mass, inertia and derivatives versus Mach; list what is missing and identify any related NTRS reports needed to fill gaps. Record the aircraft variant, loading, flight conditions, Mach/altitude coverage, units and source axes for each usable item.
The introduction states that derivatives use the "axis system of the data source"; do not assume those axes match this project's body or instantaneous stability axes. The source's introduction and contents were verified online on 2026-11-13; completeness and suitability remain for the owner to assess.
Done when: the owner records a source/page coverage checklist for both aircraft, including missing inputs and required source-axis transformations. D8 stays OPEN until H4 is complete; T11 is blocked until then.  [ ]

## Reference model inputs needed (Navion candidate; C172 fallback)
Beyond performance numbers, H3 must collect the following, with sources, units, applicability and any unavailable inputs explicitly identified:

- **Variant:** exact aircraft variant, model year, airframe configuration and modifications; matching report, pilot operating handbook or flight manual edition.
- **Loading:** empty and test mass, occupants, payload and fuel distribution, centre of gravity and datum, inertia tensor about the centre of gravity, and the source's axes and off-diagonal sign convention.
- **Flight conditions:** altitude and its definition, pressure, temperature or density, wind, airspeed definition, flap and gear configuration, and the documented power-off condition for each reference result.
- **Aerodynamic parameters:** reference wing area, span and chord, aerodynamic reference point, lift intercept and slope, lift limit and stall treatment, drag polar and efficiency factor, pitching-moment coefficients, elevator effectiveness, and lateral/directional and rate derivatives where the model needs them; include Mach/altitude indexing, angle units, signs, source axes and valid ranges.
- **Propulsion parameters:** not required for v0.1 power-off comparisons. Published turbojet/afterburner thrust and fuel-flow data are needed separately for T9 and powered v0.2 work.
- **Comparison scenarios:** reproducible power-off stall, glide, trim and static-margin cases, with matching loading, atmosphere and configuration; reference procedures, measured outputs and airspeed definitions; matching JSBSim aircraft/version and owner-generated runs. Collect evidence for the owner's D9 tolerance choice without selecting tolerances here.

---
## T0: Workspace skeleton
Goal: a compiling Rust workspace with one library crate.
Original deliverables (historical): root `Cargo.toml` (workspace), `crates/aero-core` (lib, no dependencies yet), `rust-toolchain.toml` (stable), `.gitignore`, a short `README.md` using the former placeholder name. T0b renamed the crate to `crates/soas-core`. **No LICENSE file** (decision D2 is open).
Acceptance: the three quality gates in AGENTS.md pass, with at least one trivial test.  [x]

## T1: Continuous integration
Goal: GitHub Actions runs fmt, clippy (`-D warnings`), and tests on every push and pull request.
Acceptance: workflow file added; explain in plain language what each step does.  [x]
Verification: `.github/workflows/ci.yml` has both triggers and all three mandatory gates; the commands pass locally. A hosted GitHub Actions run has not been observed because nothing has been pushed.

## T0b: Rename to the project name
Goal: apply the project name "Showdown of Aerial Systems" across the repo, crates, README and docs.
Deliverables: use repo slug `showdown-of-aerial-systems` and crate prefix `soas` (D1), with the first crate named `soas-core`; update manifests, imports, tests, README and docs to match, preserving historical references to the former placeholder.
Notes: the local crate and documentation names are updated. The working folder was left unchanged as requested. The owner creates or renames the private GitHub repository to the chosen slug and may rename the checkout folder manually if desired; no Git remote is configured here or changed by this task.
Acceptance: naming is consistent and the three quality gates in AGENTS.md pass.  [x]
Verification: the package-name test failed with the old manifest name and passed after renaming; all three gates pass for `soas-core`. Incremental compilation was disabled for the final local checks because this workspace filesystem cannot hardlink cache files.

## T2: Rotation and attitude helpers
Goal: body-to-NED rotation with quaternions; Euler angle conversion (3-2-1) for I/O only.
Deliverables: `Radians` and `Degrees` newtypes with conversion tests.
Tests (must be able to fail): known 90-degree rotations map axes as expected; Euler to quaternion to Euler round-trips; quaternion stays unit length after many composed rotations; document behaviour near pitch of +/-90 degrees.
Notes: use the conventions in docs/CONVENTIONS.md exactly. Ask if anything is ambiguous.  [x]
Verification: angle conversions, independent body-to-NED axis checks, composition order, Euler round-trips, singular-pitch errors and 100,000 normalized compositions pass. All three quality gates pass with `CARGO_INCREMENTAL=0`; the filesystem hardlink warning was reported explicitly.

## T3: ISA atmosphere
Goal: temperature, pressure, density, speed of sound as functions of altitude (0 to 20 km), plus Mach number and dynamic pressure.
Tests: compare against a **cited** ISA table (e.g. US Standard Atmosphere 1976 / ICAO). Sanity values the owner will check by hand: sea level 288.15 K, 101325 Pa, 1.225 kg/m^3; at 11 km about 216.65 K, about 22632 Pa, about 0.364 kg/m^3. Altitudes beyond range return an error, not a guess.  [x]
Verification: all five published checkpoints pass with the owner's 5e-5 relative pressure/density tolerance; all digits were rechecked against the scanned pages. The documented 5 km discrepancy persists with exact listed constants. Deliberate lapse-rate, gravity, altitude-definition and gas-constant mistakes are rejected. Both explicitly named altitude APIs accept [0, 20000] m inclusive in their own height systems. See `data/reference/atmosphere/README.md` and the answered question in docs/QUESTIONS.md.

## T4: Rigid-body dynamics (no aerodynamics yet)
Goal: 6-DoF equations of motion in body axes, with a fixed-step RK4 integrator and a state struct that can be serialized later.
Deliverables: parameterised time step with default `1/400 s` (`0.0025 s`, D10); signed inertia tensor entries about the CG in body axes and a constructor from products of inertia (D11), with tests for off-diagonal negation, symmetry and unchanged diagonal moments.
Tests: free fall matches the analytic solution; torque-free rotation of a symmetric body conserves angular momentum magnitude; quaternion norm drift stays within a stated tolerance over very many steps; halving the step size reduces integration error by roughly the expected factor for RK4.
Notes: explain the equations of motion in docs/LEARNING.md in plain language.  [x]
Verification: tilted free fall, torque-free symmetric-top rates/angular momentum, all force/moment axes, full signed inertia construction/use, invalid inputs and 100000-step quaternion normalization pass. Step-halving ratios approach 16; at default 0.0025 s, the one-second synthetic spinning free-fall case has position error 5.585205047472e-8 m and orientation error 6.666444063441e-10 rad. D10 remains unchanged; this is not a general aircraft-accuracy claim. See `data/reference/dynamics/README.md`.

## T5: Aircraft file format v0 (requires decisions D3 and D4)
Goal: define and load a minimal aircraft description: mass, inertia, CG, reference area/span/chord, aerodynamic coefficients and coefficient tables indexed by Mach (and altitude where needed).
First step: use the decided D3 JSON format with a `format_version` field and unit suffixes in field names; apply the D4 units policy. `serde_json` is pre-approved.
Deliverables: support the Mach/altitude coefficient tables and document their interpolation rules, units, source axes and supported ranges, including the policy outside tabulated ranges.
Tests: round-trip load/save; invalid files give clear errors; units are explicit in field names.  [x]
Verification: 10 tests cover JSON round-trip, required version/units/axes, mass/inertia/geometry, table shapes/nodes/control signs and non-finite save errors. Synthetic fixture labels every numeric value in its companion ledger. Pure text conversion and the linear/bilinear interpolation/error-outside-range contract are documented in `docs/AIRCRAFT_FORMAT.md`; lookup is T6.

## T6: Simple aerodynamic model (requires decision D6)
Goal: forces and moments from `CL = CL0 + CLalpha*alpha` (limited at CLmax), `CD = CD0 + CL^2/(pi*e*AR)`, `Cm = Cm0 + Cmalpha*alpha + Cmde*de`; transform wind to body axes; apply to the rigid body.
Inputs/deliverables: take Mach and altitude; look up the aerodynamic coefficients in T5's tables with linear interpolation between nodes and documented behaviour outside the tabulated range. Supporting tables does not extend v0.1 beyond its "Mach > 0.8 not modeled" limit.
Tests: steady glide gives the expected lift-to-drag ratio; a hand-constructed case gives lift = weight at a stated alpha and airspeed, with hand-calculated expected lift and specified mass (no trim solver); the stall-speed formula matches the simulated 1-g stall onset within a stated tolerance; transformation tests for each axis sign.
Table tests: exact coefficient values at nodes; linear values between nodes along each indexed dimension; behaviour outside the range matches the documented policy.
Additional deliverables/tests: apply D12's instantaneous stability/wind frames and wind force components; apply D13's named low-speed threshold, zero fallback angles, clamped asin and explicit non-finite input errors, including boundary tests; test each D6 surface sign separately (elevator: negative pitch, aileron: positive roll, rudder: negative yaw).
Update docs/ASSUMPTIONS.md.  [x]
Verification: 19 tests cover every table coefficient and node/bounds policy, signed zero, low-speed/clamped-asin/non-finite rules, all control signs, wind/stability axes, side force/CG moment transfer, hand lift=weight, local glide and one-g upper-lift-limit boundary. The synthetic aircraft is coupled through fallible RK4 stage loads and geometric ISA. Aero-coupled halving ratios are about 16; D10 stays 0.0025 s. Full scenario, differences and limits: `data/reference/aerodynamics/README.md`. T7 has not started.

## T7: Headless runner
Goal: `soas-cli run scenario --out run.csv` writes time histories.
Tests: same input produces byte-identical output on one machine (determinism test).  [x]
Verification: soas-cli runs versioned scenarios with timed piecewise-linear controls and unit-labeled, shortest-round-trip CSV. Seven tests cover process determinism, scenario/argument errors, file boundaries, time/angle conversions, stage times, cadence and CLmax/table-node convergence. No new external packages; only CLI file I/O. The transition case has irregular observed order, so D10 does not imply fourth order across events. See `docs/SCENARIOS.md` and `data/reference/aerodynamics/transitions.md`.

## T8: Trim solver and static stability numbers
Goal: local steady straight trim at given airspeed/geometric altitude: solve alpha, elevator and gamma from path-parallel/normal force and pitching moment about CG; report finite-difference control-fixed static margin. Historical T8 temporary CG-thrust input was replaced in T9 by engine-based level trim; explicit power-off glide remains available.
Tests: hand-made aircraft with a known analytic answer; level-flight force balance at the computed trim, with forces and moments balancing to a tight tolerance; explicit non-convergence and negative margin for an unstable configuration.  [x]
Verification: 10 tests cover analytic glide/level trim, tight full-vector balances, stable/unstable finite-difference margins, CG moment transfer, Mach/altitude lookup and explicit invalid/non-convergence/undefined-margin errors. No best-effort result or engine model. Synthetic margins are +0.125/-0.125 chord; full force residuals <2.63e-10 N. Sources, definitions and limitations: `data/reference/trim/README.md`. These measurements describe T8; T9 adds engine-based level trim below.

## T9: Turbojet with afterburner model v0
Goal: simple turbojet with afterburner: thrust as a table of Mach, altitude and throttle, plus fuel flow.
Owner-authorized scope for this batch: SYNTHETIC engine data only, with every numeric fixture path labeled in a ledger. Real thrust/fuel-flow tables still require cited sources; none were invented for a real engine. This is not required for the power-off v0.1 comparisons in T10. [x]
Verification: separate tests/implementation commits; version-1 engine/scenario contracts with explicit version-0 power-off compatibility, named-object guards, trilinear thrust/fuel lookup, afterburner boundary, body force and CG r x F moments, stage-timed throttle, deterministic CSV fuel reporting with constant mass, and engine-based level trim pass. Hand checks include mixed off-centre cells, unequal axis values, zero CG moment and signed inclined/offset moments. Level trim force residuals are below 1.6e-10 N in the synthetic cases; no real-engine validation is claimed. No new dependencies or OPEN decisions were selected. Sources, failing-first evidence and limits: `data/reference/propulsion/README.md`. Stop after T9; T10 remains unstarted.

## T9b: Dynamic derivatives and limits
Goal: extend the table-driven aerodynamic model with Cy_beta, Cl_beta, Cn_beta,
Cm_q, Cl_p and Cn_r (optional cross-rate Cl_r/Cn_p), using nondimensional body
rates p*b/(2V), q*c/(2V), r*b/(2V) with cited definitions/sign conventions.
Below MIN_AIRSPEED_FOR_ANGLES_MPS all rate terms are zero; non-finite inputs error.
Add a lower lift limit alongside CLmax, and aircraft-file control deflection
limits that saturate commands. Report commanded and applied deflections in CSV;
actuator rate limits remain out of scope. Bump the aircraft format version with
explicit legacy handling, named-object parsing and documented interpolation.
Tests: independent hand magnitudes with distinct span/chord for every added term;
damping signs; fixed-speed analytic roll subsidence with the stated exponential
time constant; pitch-damping magnitude; both lift boundaries; control saturation;
Mach/altitude interpolation; damping-inclusive aero-coupled convergence.
Every new test value has a synthetic label and ledger entry; no new dependencies.
Use separate tests/implementation commits, update assumptions/learning and report
sources, failures, convergence results and remaining limits. [x]
Verification: separate docs-only task entry, tests and implementation commits; all required derivatives plus optional Cl_r/Cn_p, explicit aircraft format 2, two-sided lift caps, surface saturation and commanded/applied CSV are covered. Independent magnitudes, both-direction damping signs, analytic fixed-V roll subsidence, generic object-shape tests and smooth damped coupled convergence pass. Default-step roll error is 3.455e-9 rad/s; coupled halving ratios approach 16. No dependency, physical convention or default-step decision changed. Full report and sources: `data/reference/aerodynamics/dynamics_and_limits.md`. T10 remains unstarted.

## T10: Compare against reference (needs H3 and D9)
Goal: v0.1 power-off comparisons only: stall, glide, trim and static margin, using the Navion candidate from NASA CR-96008 or the fallback Cessna 172, the H3 reference data, and JSBSim output the owner generates. Propulsion parameters and powered climb/cruise comparisons are not required.
Notes: Python is allowed here **only for plotting from CSV**. The owner checks JSBSim's license before any JSBSim files or derived data are committed.
Done when: a comparison table and plots are committed and docs/ASSUMPTIONS.md "Validation status" is filled in.  [ ]

## T11: Data-driven F-104-class aircraft in the headless sim
Goal: a data-driven F-104-class aircraft in the v0.2 headless simulation, using published derivatives versus Mach from NASA CR-96008 and related NTRS reports, with altitude dependence where supported by the sources.
Blocked until H4 is complete. Use the audited source coverage and missing-input list; do not invent missing aircraft data.
Deliverables/tests: trace each input to its source, conditions, units and axis system; transform source-axis forces, moments and derivatives into the project's body axes, with tests for the transformations and signs; exercise the published Mach-table nodes and interpolation in a headless scenario. Document coverage and limitations in docs/ASSUMPTIONS.md.  [ ]

---
## Review fixes (current batch; no T9 work)
- Fix 1: named JSON records reject positional arrays; existing diagnostics and legal array shapes retained. [x]
- Fix 2: off-centre mixed-difference interpolation and singleton-altitude coverage; scratch planar fault check. [x] All 12 pass in production; all 11 coefficient checks fail the deliberate planar variant. No production formula change was needed.
- Fix 3: independent moment magnitudes and scratch span/chord swap check. [x] All six pass in production and fail the deliberate length swap. Every moment coefficient and control surface is covered; no production formula change was needed.

- Generic object discovery: recursive fixture tests include engine/table/sample objects and throttle knots without hand-listed paths. Existing guards pass; scratch bypasses fail. [x]

Stage 2 tasks (lifting line, vortex lattice, validation) will be written when T10 is done.
