# Task queue

Status: [ ] todo, [~] in progress, [x] done. Do tasks **in order**, one at a time.
Later tasks are written just in time, after earlier ones teach us what we need.
`H` tasks are for the human owner; agents may help structure them but must not invent data.
H1 and H2 run alongside development. T10 and the entire T16-T28 experimental/modeling chain are **parked** by the owner's post-T16c answers; completed audits remain complete. Their reopening gates are in ESTIMATOR_PLAN.md, Parked work register. All six Stage 3 choices are answered with binding conditions in ROADMAP.md; **S3-01 is completed**, with its semantic-only scope and evidence below. S3-02 and later tasks remain unstarted and need task authorization. The parked comparison chain does not block that limited designer/report work; T0 to T9 are not blocked by owner tasks. (Owner S3-01 batch; ROADMAP; S3_01_SEMANTIC_MODEL.md.)
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

## T10: Compare against reference (parked)
**PARKED**, not completed or deleted. The owner's latest direction parks comparison against published derivatives along with the T16-T28 chain; retain this task's earlier power-off stall/glide/trim/static-margin reference goals as historical scope. Neither a Stage 3 report nor a source-readiness audit completes a published-data comparison. (Owner post-T16c consequences; ESTIMATOR_PLAN, Parked work register.)
Retained performance goal: power-off comparisons using a selected audited aircraft, H3 reference data and appropriately reviewed reference-tool evidence; no invented propulsion data or powered climb/cruise requirement. Navion/Cessna remain historical leads, not an approved complete dataset. The earlier static milestone/H3/D9 requirements remain unsatisfied where applicable; they do not block the limited Stage 3 route. (Previous T10 scope; T16_AUDIT §§1-4; owner consequences.)
Reopen only on explicit owner task/scope, a suitable audited dataset, matched definitions/configuration/condition/reuse terms, independently sourced inputs and adequate model capability, plus H3 cases and D9 if an accuracy acceptance verdict is requested. Review at v0.2 for supersonic work or when a suitable wing-alone/wing-tail dataset is found; neither trigger resumes work automatically. (ESTIMATOR_PLAN, Parked work register; ROADMAP, Experimental validation.)
Notes: Python is allowed here **only for plotting from CSV**. The owner checks JSBSim's license before any JSBSim files or derived data are committed.
Done when: a comparison table and plots are committed and docs/ASSUMPTIONS.md "Validation status" is filled in.  [ ]

## T11: Data-driven F-104-class aircraft in the headless sim
Goal: a data-driven F-104-class aircraft in the v0.2 headless simulation, using published derivatives versus Mach from NASA CR-96008 and related NTRS reports, with altitude dependence where supported by the sources.
Blocked until H4 is complete. Use the audited source coverage and missing-input list; do not invent missing aircraft data.
Deliverables/tests: trace each input to its source, conditions, units and axis system; transform source-axis forces, moments and derivatives into the project's body axes, with tests for the transformations and signs; exercise the published Mach-table nodes and interpolation in a headless scenario. Document coverage and limitations in docs/ASSUMPTIONS.md.  [ ]

## Stage 2 start: owner-authorized T12 then T13
Run only T12 and T13 in this batch; do not start T10 or T11. Their reference-data
and validation decisions remain pending. Create soas-estimator depending on
soas-core, with no reverse dependency. Pre-approved dependencies only. First
record these tasks in a docs-only commit, then tests/implementation commits per
task. Every fixture number must have a synthetic label and ledger entry.

## T12: Wing planform geometry
Goal: mirrored symmetric wing from ordered right-half span stations: y, chord,
body-axis leading-edge x and z offsets, geometric twist, section lift slope and
zero-lift angle. Section values are explicit inputs with provenance; provide
only a cited theoretical default (thin-airfoil 2*pi/rad), never guessed data.
Derive span, projected planform area, aspect/taper ratios, mean aerodynamic
chord and its position, and quarter-chord sweep. Document station interpolation,
mirroring, body-axis signs and reference definitions. Validate finite values,
positive chord/section slope and strictly increasing nonnegative station y.
Tests: independent rectangular and tapered hand cases, standard trapezoid MAC,
distinct span/chord, interpolation/mirroring and invalid inputs. [x]
Verification: six tests pass for rectangular/tapered hand geometry, trapezoid MAC and position, body-axis sweep, station interpolation/mirroring, subdivision invariance, invalid/overflow/provenance cases and the cited thin-airfoil helper. Runtime has only the one-way soas-core dependency; serde_json is test-only. Full formulas, assumptions and failing-first evidence: `data/reference/estimator/geometry.md`.

## T13: Prandtl/Glauert lifting-line Fourier solution
Goal: straight symmetric, incompressible, attached-flow wing; output CL, induced
CD, span efficiency and section loading versus alpha, with Fourier term count
as a parameter. Use T12 station geometry for straight wings and an analytic
elliptic planform input for an exact benchmark (no station-tip approximation).
Cite every formula, document collocation and coefficient normalization, and
return applicable assumptions with "validity unassessed" rather than invented
physical validity thresholds. No sweep correction or real-aircraft validation.
Tests: elliptic no-twist efficiency=1, CDi=CL^2/(pi*AR), and analytic finite-wing
slope for two aspect ratios; zero lift at alpha_L0; rectangular term-count
convergence and efficiency<1; section loading/symmetry, invalid inputs; deliberate
wrong-section-slope variant fails elliptic tests in an uncommitted scratch copy.
Update ASSUMPTIONS/LEARNING; report observed convergence and limits. [x]
Verification: eight tests cover two independent elliptic hand cases (including local loading and e=1), zero lift, incidence signs, rectangular refinement/e<1, symmetry, assumptions/validity and invalid inputs. The scratch omitted-section-slope variant fails both elliptic cases with CL errors of factors 3 and 6. Rectangular CL/CDi converge by roughly 16x per term-count doubling; N=128 gives e=.953934767177. No physical validity threshold, OPEN decision or core-to-estimator dependency was added. Full sources, counts/ledgers and measured convergence: `data/reference/estimator/lifting_line.md`. This batch stops here; T10/T11 remain unstarted.

## T14: Vortex-lattice influence coefficients
Goal: unit-strength horseshoe induced velocity in soas-estimator from cited
finite-segment Biot-Savart integrals, with trailing legs along freestream to
infinity (or an explicit finite wake). State endpoint orientation, units and
an explicit on-segment singularity error, never NaN or an invented core radius.
Bound vortices occupy panel quarter chord; controls occupy three-quarter chord.
Classical lifting line is a high-aspect-ratio straight-wing approximation;
the low-aspect-ratio swept hero-wing geometry is very different. This motivates
the more general panel method, without claiming its physical validation.
Tests: independent horseshoe hand values on/off symmetry plane, all component
signs, long-bound two-dimensional limit, singularities and mirrored symmetry.
Use labeled fixture/test settings ledgers and tests/implementation commits. [x]
Verification: seven tests pass for scalar horseshoe hand values (all components, on/off symmetry), signs/orientation, long-segment 2D limit, ray limit, singularity errors, mirrored panels and invalid data. Semi-infinite wake legs remove a guessed wake-length setting; no core radius is introduced. Formula source/verification limits and ledgers: `data/reference/estimator/vortex.md`.

## T15: Planar-wing vortex-lattice solver
Goal: T12 planar geometry with parameterized spanwise/chordwise panel counts;
solve circulation from flow tangency including twist/section zero-lift angle.
Return CL, induced CD (document Trefftz-plane or Kutta-Joukowski choice), pitching
moment about an explicit reference point, and neutral-point x from dCm/dalpha.
State planar, incompressible, attached-flow, small-angle assumptions; low-AR
swept hero-wing results remain "validity unassessed" until compared with data.
Tests: one-panel hand solution; mirrored strengths/zero roll; refinement in
both mesh directions; rectangular neutral point near quarter MAC; high-AR
agreement with T13 through strictly decreasing CL/CDi relative gaps across
AR6/10/20/40 on two meshes (64x8 and 128x8). Preserve the original comparison
unchanged as an expected-difference record. Zero-compatible extrapolation
verdicts are informational only, with full/subset records. Elliptic AR6/40:
nonincreasing efficiency excess, decreasing successive VLM mesh changes
(owner clarification), and finest-three Richardson efficiency inside a
prediction band derived from the first four meshes. No physical e=1 equality
or guaranteed continuum-error band is claimed. Existing hand, spacing,
symmetry, convergence, moment/neutral-point/error tests remain. [x]
Review evidence: data/reference/estimator/vlm_gap_vs_mesh.md,
vlm_subset_fits.md and vlm_acceptance_review.md. New tests pass immediately
against unchanged production; scratch AR-drag-multiplier and zero-slope
variants fail. All 175 workspace tests pass; formatting/Clippy pass. Every
input/setting is ledgered. Original failing comparison remains in 8067b1d
and vlm_expected_difference.md. No T10/T11 work or real-aircraft validation.

## T15b: bound checks and efficiency diagnosis (owner-authorized 2026-11-19)
- Part 1: elliptic finest relative slope improvement AR6→40 for Nx1/Nx8; exact Munk screens on 128/256/512 and 256/512/1024 Richardson estimates. Tests pass against unchanged production; wrong-AR and 0.95-drag scratch mutations fail. Owner-approved estimator-only test compilation optimization; no tolerance added. All 177 tests and fmt/Clippy pass. Reference: estimator/t15b_assertions.md. [x]
- Part 2: preserve verifier script unchanged apart from origin header; record supplied AR10..160 data/exponents and scope Nx1 prediction bands explicitly. Body bytes/AST and numeric inventory checked; see estimator/vlm_gap_vs_ar.md and independent/NUMERICAL_LEDGER.md. [x]
- Part 3: same-CL efficiency comparison, algebraic drag-gap decomposition and outer-tip refinement experiments complete using the independent verifier, with LLT term-convergence records. See estimator/vlm_efficiency_gap.md and its numerical ledger; decay-rate question remains explicitly unresolved. No assertions, physical/bug verdict or T10/T11 work. [x]

### T15b follow-up: resource evidence and taper experiments (owner-authorized 2026-11-19)
- Part 1: exact profile diff, enabled debug/overflow checks, printed 256x8 old/new outputs and bit differences; unchanged largest test 179.9444381 s / 1,081,479,168 peak resident bytes, below the owner-confirmed 240 s / 2.0 GB guards. Full cargo test --all: 194.9422389 s, 177 passed, none failed/ignored; fmt/Clippy pass. Source/settings/results ledger: data/reference/estimator/t15b_profile.md. Documentation only. [x]
- Part 2: production Rust runs at AR10/20/40/80, efficiencies agreeing with archived full-precision results to relative differences below 5.38e-14 on these grids. AR80 ran in 76.8736739 s / 1,080,778,752 peak resident bytes. AR160 was not launched: two dense matrices alone need 4,294,967,296 bytes, above the disclosed budget. No solver substitution, new assertion or tolerance. Evidence/ledger: data/reference/estimator/t15b_production_comparison.md and independent/T15B_PRODUCTION_LEDGER.md. [x]
- Part 3: all owner-specified taper/AR/mesh cases run with the archived independent VLM and its Fourier equations adapted only for tapered chord input. LLT refined through 4096 terms; both fit forms and all residuals, local tip-width/chord ratios and full numerical ledger recorded. Raw lambda .5 / AR10 e=1.001331201893 retained and flagged; no threshold verdict or causal claim. Evidence: data/reference/estimator/vlm_taper_sweep.md and independent/T15B_TAPER_LEDGER.md. Documentation only. [x]

---
## Review fixes (historical)
- T12 Gap 2: duplicate/descending interior, missing-root and equal-root/tip structured-error tests pass immediately; removed-guard scratch copy fails all four. [x]
- T13 Gap 1: independent rectangular CL/CDi/efficiency constants at 4/128 terms, immediate passing regression and failing n-squared scratch mutations; no production correction needed. [x]
- Fix 1: named JSON records reject positional arrays; existing diagnostics and legal array shapes retained. [x]
- Fix 2: off-centre mixed-difference interpolation and singleton-altitude coverage; scratch planar fault check. [x] All 12 pass in production; all 11 coefficient checks fail the deliberate planar variant. No production formula change was needed.
- Fix 3: independent moment magnitudes and scratch span/chord swap check. [x] All six pass in production and fail the deliberate length swap. Every moment coefficient and control surface is covered; no production formula change was needed.

- Generic object discovery: recursive fixture tests include engine/table/sample objects and throttle knots without hand-listed paths. Existing guards pass; scratch bypasses fail. [x]

- CSV contract review: exact schema-1 header test, 33-column documentation, two-sided lift-flag wording and identical-bit literal audit recorded. [x]

## Documentation inventory (owner-authorized 2026-11-19)

Goal: inventory CR-96008's F-104/Navion sections and the three exact supplementary
NASA reports supplied by the owner, with page references, coverage, units,
source axes/control signs, H3/H4 gaps and power-off comparison limitations.
Documentation only; no numerical fixture transcription, code or tests. [x]

Evidence: `data/reference/inventory/cr96008.md` and `other_sources.md`.
All four exact PDFs opened; title/author/date/aircraft scope and relevant
scanned pages checked. The absent PROJECT_BRIEF.md was not reconstructed;
the owner supplied NTRS 19630004010, 19660022438 and 20090026345 explicitly.
The two-fighter comparison covers labeled swept-wing airplanes A/B, not an
identified F-104/Navion data set. CR discrepancies and incomplete physical
control-sign evidence remain flagged. No engine-off validation set was
established. This inventory does not complete owner H3/H4 or decide D8/D9.
CONVENTIONS.md is unchanged. Documentation diff/whitespace checks only;
no test run, as the owner directed for this documentation task.

## Documentation go/no-go and sign audit (owner-authorized 2026-11-19)

- Part A: compact F-104/Navion input matrix, current estimator/simulator limits,
  ranked source blockers and Navion-first recommendation, with the comparison
  report's derivative-specific disagreement and uncertainty kept separate.
  Evidence: `data/reference/inventory/go_no_go.md`; cited CR pp. 74-79,
  100-103, MEMO 2-3-59L pp. 12-16, and repository sections/lines. [x]
- Part B: source-by-source elevator/aileron/rudder evidence, including the
  JSBSim manual's opposite right-aileron scalar sign, Caughey's explicit D6
  agreement and opened author-linked slides; the published textbook passage
  remains unverified. Evidence: `docs/SIGN_EVIDENCE.md`, JSBSim manual
  Flight Control chapter §Conventions, Caughey §1.2.2 p. 5, Beard/McLain
  chapter 4 slide 8, and the NASA page references in the audit. [x]

Documentation only; no model-input values transcribed, no code, dependencies,
tests or CONVENTIONS.md changes. Checks: documentation whitespace, local links,
input-field coverage and tracked-change scope. No tests run, as the owner
directed. H3/H4, D8/D9 and T10/T11 statuses remain unchanged; the requested
derivative audit does not replace queued T10's power-off goal (H3/H4/T10/T11
above; AIRCRAFT_FORMAT.md sections cited in the decision sheet).

## Documentation estimator plan (owner-authorized 2026-11-19)

Historical initial draft; superseded by the owner's static-only planning
decisions below. Its broad proposals do not authorize later comparisons.

Goal: identify the estimator contributions needed for a published Navion
derivative comparison and propose their implementation order, without starting
that implementation or transcribing aircraft values. [x]

Evidence: `docs/ESTIMATOR_PLAN.md`. All X-B/C derivative names and the
recoverable elevator pitch derivative are mapped to current capabilities,
missing physics and proposed tasks T16-T28. Each proposal includes method
sources/verification limits, independent analytic or hand tests, planned
mutations, risks and relative size. Static-slope milestones are separated
from controls, rotating-flow, compressibility, total drag, unsteady and
conditional propulsion work; source/owner gates precede comparison claims
(CR pp. 100-103, A-1-A-10/B-1; method sections cited in the plan).

No active T16-T28 queue entries or implementation authorization are created.
H3/H4, D8/D9 and queued T10/T11 remain unchanged. Documentation checks only;
no code, dependency, aircraft data, test, CONVENTIONS.md or VISION.md changes.
The plan ends with the questions needed before implementation starts.

## Static derivative milestone (parked after T16c owner answers)

The source checkpoint is answered: park TN 1270 with its NO-GO and remaining
gaps; defer wing-tail work and F-104 searching (to v0.2); park both wing
fallbacks. T16/T16b/T16c audits remain complete, but further T16-T28 work and
the comparison milestone are **parked**, not deleted or marked successful.
First experimental scope remains wing-alone if suitable data are found later.
(Owner post-T16c answers 2-5; ESTIMATOR_PLAN, Parked work register.) [ ]

**v0.1 validation status:** "code-to-code cross-checked and analytic-case verified; not experimentally validated".
The validation page and report disclaimer must use this wording plainly,
with actual evidence and omissions. Experimental validation is a parallel
track; candidate gaps and reopening conditions remain documented. Stage 3
does not imply a complete aircraft estimator: current VLM is wing-only, with
no tail/fuselage model. Missing whole-aircraft data are not invented to enable
trim/static-margin/export checks. (Owner post-T16c consequences; ROADMAP,
Experimental validation/Stage 3 data contract; ESTIMATOR_PLAN, Current scope.)

Other simulator derivatives remain future candidates, each requiring its
own approval after the checkpoint. Drag, Mach-derivative and
angle-of-attack-rate comparisons are outside the planning horizon.
No propeller or engine model is to be built for this milestone.

## T16: Navion source and comparison-definition audit (completed; modeling NO-GO)

Documentation/source audit completed on 2026-11-20; no solver implementation.
The required power-off search and source/definition audit are recorded in
`docs/T16_AUDIT.md`, §§1-5. The Navion verdict is NO-GO for estimation against
CR X-B: later sources add geometry and a propeller-removed case, but do not
close the aircraft/configuration, normalization, complete geometry/section
or propulsion-state match (audit §§2-4; CR pp. 100-103; NASA TN D-5857
pp. 3-6, Fig. 4 p. 12; NASA TN D-6643 pp. 6-7, 19-21).
Audit exact variant/configuration, wing planform and airfoil/section data,
tail geometry/position, fuselage dimensions, CG/datum and aerodynamic reference
definitions, flight condition and power/propeller state. Resolve or list
every missing input, source discrepancy and axis/reference ambiguity with
source/page evidence (CR pp. 100-103, A-1-A-10/B-1; audit §§1-2). [x]

Audit completion does not satisfy the modeling gate. The historical alternative proposal
was CRM-HL / ONERA LRM-WBSHV, HLPW-5 Case 2.2: public matching CAD/static data
were opened and checked, with section/slat/deformation/conversion and
CC BY-NC-ND reuse questions explicitly retained (audit §3; ONERA guide
§§3, 6.4, 8.2.3, 8.3.1). No substitution is selected. The PG arithmetic is
confirmed at the source Mach, including the proposed alternative's condition;
the share of an unknown total discrepancy remains unavailable (audit §5).
The first-comparison contract, T17's limited role and pre-modeling questions
are documented without thresholds (audit §§6-7; ESTIMATOR_PLAN.md §§T17, T23).
Documentation checks only; no code, tests, coefficient-table values or source
artifacts committed. T17/T10 remain unstarted (audit scope and §6).

Required ending: GO or NO-GO for the first whole-aircraft lift-slope estimate,
distinguishing input readiness from physical accuracy. If geometry is
insufficient, say so plainly and propose an alternative aircraft with complete
public, mutually matching geometry/static-reference data, citing and checking
each required input. Do not assume the queued Cessna fallback is complete;
do not fill gaps from memory or silently switch the selected reference.

If power-on or silent, report propulsion as unmodeled, with sourced expected
size/direction and assumptions. State explicitly when those cannot be
established; do not invent a range or net sign. No engine/propeller model.

For the actual published condition, compute and cite the Prandtl-Glauert
factor, its percentage change and the share of the expected/reported
discrepancy it could explain, with consistent denominators and clear limits.
CR X-A/B p. 101 gives M=0.158: direct calculation gives factor approximately
1.01272 (1.272% above incompressible; 1.256% deficit relative to the corrected
value). The formula is in the AVL primer, Compressibility section. This
diagnostic is not an AVL run or a uniform three-dimensional aircraft correction.
Without a whole-aircraft estimate/expected-gap budget, its fraction of the
total gap is unavailable; report that limitation and the calculation at the
first comparison. Source and derivation: `docs/ESTIMATOR_PLAN.md`.

Owner hold point: if actual Mach is above about 0.3 or the factor increase is
above about 5%, stop and ask whether finite-Mach treatment must come first.
This is a review trigger, not a physical-validity or D9 accuracy threshold.

Later pinned AVL cross-check: secondary evidence only, after license review
of the actual release, inputs and intended artifacts. Check permission before
committing anything derived from AVL. No binaries or raw output unless clearly
permitted with applicable conditions satisfied; our own sourced summary tables
are the preferred deliverable. Official release conditions/GPL §0 were checked
for this plan; a future pinned distribution/artifact review remains required.

## T16b: Layered public-reference search (documentation completed; selection gaps remain)

Completed on 2026-11-20 in `docs/T16B_REFERENCE_SEARCH.md`: three wing-alone
candidates (§3), two wing-tail candidates (§4), the F-104 model in NACA
RM SL56H06 (§5), and CRM-HL / ONERA Case 2.2 (§6), each against the owner's
six criteria, with source/page or section evidence and uncertain readings. [x]

The ranked recommendation is to prepare TN 1270's smooth Fig. 4 wing
conditionally, after geometry, exact condition and independent section-input
closure; no candidate is certified ready for an unqualified comparison
(reference search §§3/7/8; TN 1270 pp. 4-6, Tables I-II, Figs. 1/3/4).
No fully cleared wing-tail reference was established (§4). RM SL56H06 fails
the low-Mach requirement: its tests are supersonic (§5; report pp. 2-5).
The previous CRM-HL proposal is withdrawn for this narrower scope: deployed
slats and missing component physics fail criteria (4) and (6)
(§6; ONERA guide §6.4 p. 3; VLM `ASSUMPTIONS`).

Audit §§8-9 distinguish possible AVL/OpenVSP code cross-checks from experimental
validation and list the questions before modeling. No alternative/reference
selection, D8/D9 decision, code, tests, coefficient-table transcription or
modeling task is completed or authorized here. T17/T10/T11 remain unstarted
(audit §1; ESTIMATOR_PLAN.md opening scope and §§T17/T23).

## T16c: TN 1270 bounded closure (documentation completed; modeling NO-GO)

Completed on 2026-11-20 in [T16C_TN1270_CLOSURE.md](T16C_TN1270_CLOSURE.md):
the five T16b gaps have explicit closure methods/statuses (§3), the exact
future measured-quantity contract is recorded without data values (§4),
and candidate assumptions have sourced support with unknown effects stated
rather than invented bounds (§5). No coefficient-table transcription,
code, tests, modeling or external-program run. (Closure §§1-5.) [x]

Reference definitions/reuse are closed; spanwise prescriptions and section
inputs are partial; exact finite-wing Mach remains unverified. TN 1269's
published sample section plots are new useful evidence for a different
worked wing, not a complete matched input set. Verdict: NO-GO for a fully
sourced experimental comparison. (Closure §3; TN 1270 pp. 3-6/Tables I-II;
TN 1269 p. 20/Table IV/Figs. 1-3.)

The five T16b questions are restated with options and recommendations and
are now **ANSWERED** by the owner's T16c decisions and follow-up answers 2-5.
TN 1270 is parked with no further reconstruction. Stage 3 is the next planned
direction, with new implementation questions in ROADMAP.md; no modeling or
designer task is started. T17/T10/T11 remain unstarted and D8/D9 unchanged.
(Closure §§6-7; ESTIMATOR_PLAN, Current scope; QUESTIONS, T16c/Stage 3 entries.)

## Stage 3: recorded designer/report queue (S3-01 completed)

All details, independent tests, scratch mutations, risks, sizes and recorded
owner conditions are in [ROADMAP.md](ROADMAP.md#stage-3-task-plan--recorded-owner-decisions).
Use S3 IDs; **no T16-T28 renumbering**. All six choices are ANSWERED; the owner
authorized **S3-01 only**, now completed, not an automatic batch. The
implemented architecture is a pure `soas-designer` depending on core/estimator,
with no reverse dependency or library file I/O. CLI integration is later work;
S3-02 remains unstarted. Only pre-approved dependencies. (Owner choices 1-6
and S3-01 batch; AGENTS, Batch mode; S3_01_SEMANTIC_MODEL.md.)

**Binding data rules:** save incomplete drafts, but missing intrinsic inertia
blocks export. Every computable wing result (including area, aspect ratio,
lift slope, induced drag and wing neutral point) is recomputed from current
geometry; supplied/template aerodynamic values are restricted to the permitted
tail, fuselage, interference and CLmax contributions. No opaque supplied
aircraft total or stored static wing result bypasses that split. Whole-aircraft
trim/static-margin/export require a complete assembled bundle; basic checks
remain available with their own required inputs. Detailed assembly and any
remaining simulator fields are later S3-05 gates, not an excuse to guess or
to block S3-01. (Owner choice 2; ROADMAP, Binding estimated-versus-supplied contract.)

Every numeric input/result/setting and vector/tensor entry has machine-readable
provenance, shown beside its report value: `ESTIMATED_WING_ONLY`,
`USER_SUPPLIED`, `TEMPLATE_DEFAULT`, `NOT_ESTIMATED`, `OUTSIDE_COVERAGE`.
The single added category **`DERIVED_FROM_INPUTS`** identifies calculated
mass/CG, ratios and mixed-result quantities without falsely calling them
supplied or wing-only. Dependency paths retain their origins. Later shape
helpers also carry `approximate: true`. Templates use neutral names and
synthetic labels; no real-aircraft identity or confidence percentage is
implied. (Owner choices 3/6; ROADMAP, Machine-readable number provenance.)

**Stage 3 acceptance:** the **11 m to 9 m wingspan demonstration** is required.
With normalized chord distribution, component mass/loading, atmosphere and
supplied constant CLmax held fixed within coverage, independent expectations
are area/aspect-ratio ratios `9/11`, wing-loading ratio `11/9` and stall-speed
ratio `sqrt(11/9)`. Recompute all wing outputs and show current-revision
provenance. Every stall result says **"conditional on supplied CLmax"** and
**"flaps and high-lift devices are not modelled"**; state geometry affects
this stall calculation through area and weight only. No implied change to
CLmax. (Owner choice 5/acceptance instruction; ROADMAP, Geometry-change acceptance test.)

| ID | Task / dependencies | Status |
|---|---|---|
| S3-01 | Semantic aircraft components: wings, tails, fuselage, engines/table references, controls, datum and per-number provenance; no static derived wing inputs. | [x] Completed; 16 tests and eight detected scratch faults |
| S3-02 | Independently versioned designer JSON/text conversion and CLI loading; after S3-01. | [ ] Planned; not authorized |
| S3-03 | Explicit component mass/CG/inertia and fixed loading-case buildup; after S3-01. | [ ] Planned; not authorized |
| S3-04 | Current-geometry wing recomputation with provenance and explicit unsupported/not-estimated results; after S3-01. | [ ] Planned; not authorized |
| S3-05 | Complete assembled wing-plus-allowed-supplied inputs and freshly generated simulator export; after S3-02/03/04. | [ ] Planned; not authorized |
| S3-06 | Wing loading, thrust-to-weight, declared CG range and domain checks; after S3-03/05. | [ ] Planned; not authorized |
| S3-07 | Existing trim/static-margin wrappers and sampled CG feasibility; after S3-05/06. | [ ] Planned; not authorized |
| S3-08 | Conditional stall speed from independently supplied whole-aircraft CLmax; after S3-03/05/06. | [ ] Planned; not authorized |
| S3-09 | Headless HTML report and validation/evidence page with exact disclaimer; after S3-02/04/06/07/08. | [ ] Planned; not authorized |
| S3-10 | Later box/cylinder/ellipsoid inertia helper with rotation/parallel-axis accounting and explicit approximate qualifier; after S3-03 and report support. | [ ] Later; not authorized |

### S3-01 completion evidence

#### S3-01b verification follow-up (in order)

- Fix 1: completed typed registry and spelling-independent computed-wing
  rejection. Tests first (`12968e4`), original-case runtime failures recorded,
  restored-prefix scratch mutation rejected. All three gates pass; full suite
  before any rudder correction: **198 passed, 0 failed, 0 ignored**. [x]
- Separate rudder-fixture correction: narrowly authorized after the sign-order
  preflight; before/after full suites both 198 passed; only six swapped rudder
  coordinates changed. Exact inventory diff and fixture-reader list recorded
  in S3_01B_VERIFICATION.md. [x]
- Fix 2: source/non-finite/provenance/station/inertia/control-order verification,
  including actual-parent-span ventral/inverted/canted cases. [ ]
- Fix 3: supplied-parameter suffix/unit reconciliation. [ ]
- Fix 4: template/gallery naming and future loading/missing-mass acceptance
  conditions (documentation only). [ ]

S3-02 stays unstarted. The original S3-01 evidence below is retained; current
registry/follow-up details are in S3_01_SEMANTIC_MODEL.md and QUESTIONS.md.

Tests committed first as `7051e75 Task S3-01: tests`, with the missing-API
compile failure recorded before implementation. Sixteen tests pass unchanged;
independent coordinate/field expectations and the per-number synthetic ledger
cover semantic components, references, draft missing states, every tensor/table
numeric path, provenance and neutral template identity. All eight listed
scratch mutations compiled and failed their targeted assertions; no mutant
is committed. All three gates pass: **193 workspace tests passed, 0 failed,
0 ignored**, with no tolerance/profile change. TEMP/TMP were inside the
checkout. Rust's hard-link cache warning and automatic copy fallback were
disclosed; no filesystem/cache workaround was applied. Details and actual
failure evidence: [S3_01_SEMANTIC_MODEL.md](S3_01_SEMANTIC_MODEL.md).

Scope remains in-memory descriptive inputs/validation. No designer JSON,
mass buildup, estimator adapter, aircraft export, report implementation,
shape approximation or S3-02 work is included. The future export task must
enforce required missing inputs; it is not claimed implemented here.

### Documentation: six Stage 3 choices adopted with conditions (completed)

Recorded all six owner choices, automatic geometry-driven wing results,
the permitted supplied-input boundary, per-number provenance/adjacent report
flags, conditional stall notices, neutral template names and the required
11 m to 9 m acceptance test. Added the later approximate shape helper. No
implementation, numeric fixture or test is created; S3-01 remains authorized
for the next implementation task without repeat approval. (Owner adoption
and documentation-only scope; ROADMAP; QUESTIONS, answered Stage 3 entry.) [x]

### Documentation: checkpoint answers and Stage 3 proposal (completed)

Recorded the owner's answers, exact v0.1 status, parked T10/T16-T28 scopes,
parallel validation candidates/gaps and deliberate reopening conditions;
proposed S3 tasks without implementation or invented aircraft numbers.
Original T25-T28 plans are restored as parked context, with earlier T23/T24
scope history identified. Documentation checks only; owner learning columns
remain blank. (ROADMAP/ESTIMATOR_PLAN, current sections; owner post-T16c
request; LEARNING log.) [x]

## Documentation: static milestone decisions recorded (2026-11-19)

Updated ESTIMATOR_PLAN.md and this queue with the owner's scope, T16 audit,
PG assessment/hold point, power-state treatment, neutral-point requirement,
license policy and mandatory first-comparison checkpoint. Broad candidate
comparison task plans were removed from the current planning horizon. [x]
Documentation checks only; no solver, dependency, aircraft fixture, test,
CONVENTIONS.md, VISION.md or T11 change. LEARNING.md records concepts only.
This commit does not complete T16 or start a modeling/comparison task.
