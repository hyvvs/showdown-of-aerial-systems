# Build Phases: Showdown of Aerial Systems (SOAS)

Scope of this document: the **building phases** and **what every layer must have in each phase**.
Nothing else. Conventions, decisions, process, and rationale live in other docs.

---

## 1. Layers

Dependencies point **downward only**: a layer may depend on layers with a lower number, never higher.

| # | Layer | Responsibility | Where it lives |
|---|---|---|---|
| L1 | Foundation | Workspace, toolchain, CI, build targets | repo root |
| L2 | Math and frames | Units newtypes, rotations, frame transforms | `soas-core` |
| L3 | Environment | Atmosphere, gravity, later wind and terrain queries | `soas-core` |
| L4 | Dynamics | Rigid-body state, equations of motion, integrator | `soas-core` |
| L5 | Aerodynamics model | Forces and moments from coefficients | `soas-core` |
| L6 | Propulsion | Thrust and fuel flow models | `soas-core` |
| L7 | Control and trim | Trim solver, stability numbers, controllers | `soas-core` |
| L8 | Aircraft data | Aircraft file schema, loader, validation, versioning | `soas-core` (types), `soas-cli` (file I/O) |
| L9 | Estimator | Geometry to coefficients | `soas-estimator` |
| L10 | Designer and report | Parametric aircraft model, mass/CG, checks, report | `soas-designer` |
| L11 | Validation harness | Reference data, golden fixtures, comparisons, plots | `data/reference`, tests, tools |
| L12 | CLI | Headless runner and commands | `soas-cli` |
| L13 | Web app and UI | WASM build, controls, panels, hosting | `soas-web` |
| L14 | Rendering and audio | Viewer, visuals, sound | `soas-web` / render crate |
| L15 | Systems and cockpit | Fuel, electrical, hydraulics, instruments | `soas-core` modules, UI |
| L16 | Import and export | SolidCraft adapter, glTF/STEP, tagging | adapter crate |
| L17 | Terrain and world | Heightmaps, streaming, weather | world crate |
| L18 | Combat and AI | Weapons, damage, AI | `soas-core` modules |
| L19 | Networking and content | Netcode, missions, modding | net and content crates |

Crates are created only when a phase needs them. The mapping above may be split later.

### Requirements that every layer must meet, in every phase
1. A small, documented public API; no reaching into another layer's internals.
2. Tests that can fail: analytic cases, published values, or reference-tool fixtures (golden files).
3. Explicit error types; no panics on bad input; non-finite numbers rejected, never silently clamped.
4. Every constant and coefficient has a cited source or a `TODO(source)`.
5. Simplifications recorded in `docs/ASSUMPTIONS.md`.
6. Determinism for L2 to L7 and L15, L18: fixed time step, no unordered-map iteration, no wall-clock time, seeded randomness only.
7. Serializable state and configuration wherever a layer holds state.
8. Passes the quality gates: format check, clippy with warnings as errors, all tests.
9. A performance budget stated before the layer is built for the web target (L13).

---

## Phase 0: Foundation (release: none)

**Goal:** a compiling, tested, CI-checked workspace.

- **L1 Foundation:** Cargo workspace; pinned stable toolchain with rustfmt and clippy; Rust edition 2024; `.gitignore`; README; CI workflow running format check, clippy (`-D warnings`) and tests on push and pull request; project rename applied (`soas-core`).
- **L2 to L8:** `soas-core` library crate exists with a smoke test and no dependencies.

**Exit:** all three quality gates pass locally and in hosted CI.

---

## Phase 1: Headless simulation core (release: v0.1, Stage 1)

**Goal:** a deterministic, Mach-aware 6-DoF simulator that flies a table-based aircraft in a terminal and is compared against published data.

Task map: T2 to T4 (math, environment, dynamics), T5 to T8 (data, aerodynamics, runner, trim), T9 to T10 (propulsion, comparison), T11 (data-driven jet).

- **L2 Math and frames (T2):**
  - `Radians` and `Degrees` newtypes with tested conversions.
  - Unit quaternion (Hamilton) rotating body vectors into NED; renormalization.
  - Euler angles (3-2-1) for input and output only; documented behaviour near pitch of +/-90 degrees.
  - Frame transforms: body to NED, wind/stability axes to body axes (instantaneous axes, wind force components `(-drag, +side, -lift)`), each with sign tests.
  - Source-axis to body-axis transformation utilities for published derivatives.
- **L3 Environment (T3):**
  - International Standard Atmosphere, 0 to 20 km: temperature, pressure, density, speed of sound.
  - Mach number and dynamic pressure helpers.
  - Out-of-range altitude returns an error.
  - Constant gravity `g0`; flat, non-rotating Earth; NED with `h = -z`.
- **L4 Dynamics (T4):**
  - Serializable rigid-body state: position, body velocity, quaternion, body rates, mass properties.
  - Inertia as signed tensor entries about the CG; constructor from products of inertia (negation tested).
  - 6-DoF equations of motion in body axes.
  - Fixed-step RK4; default step `1/400 s`, always a parameter.
  - Tests: free fall, torque-free rotation, quaternion-norm drift bound, step-size convergence order.
- **L5 Aerodynamics model (T6):**
  - Inputs: state, atmosphere, control deflections, Mach, altitude.
  - Airflow angles `alpha` and `beta` with the near-zero-airspeed rule (`MIN_AIRSPEED_FOR_ANGLES_MPS`, neutral fallback of 0).
  - Lift, drag, side force and three moments from coefficients; coefficient tables indexed by Mach (and altitude where needed) with documented interpolation and out-of-range behaviour.
  - Lift limit at CLmax (crude stall); drag polar; pitching moment with elevator effect.
  - Control surface signs: elevator, aileron, rudder, each with a sign test.
  - A hand-constructed lift-equals-weight test.
  - Mach above 0.8 reported as "not modeled".
- **L6 Propulsion (T9):**
  - Turbojet with afterburner: thrust as a table of Mach, altitude and throttle; fuel flow.
  - Thrust position and direction in body axes; forces and moments into the dynamics.
  - Sources cited for every table; no invented values.
- **L7 Control and trim (T8):**
  - Trim solver for a given airspeed and altitude (alpha and elevator, throttle if powered).
  - Static margin and neutral-point report from the coefficients.
  - Tests against a hand-made aircraft with an analytic answer; force and moment balance to a tight tolerance.
- **L8 Aircraft data (T5):**
  - JSON schema v0 with `format_version`, unit suffixes in field names, mass properties, reference geometry, coefficients, tables indexed by Mach, control surfaces, engines.
  - Load and save with round-trip tests; clear validation errors; unknown versions rejected.
- **L11 Validation harness (T10, T11):**
  - `data/reference/` with sources for every number.
  - Synthetic test aircraft clearly labelled as test values.
  - Comparison tests against published data for power-off cases: stall, glide, trim, static margin.
  - Reference-tool fixtures (JSBSim outputs generated by the owner), plotting helper (plotting only).
  - Data-driven F-104-class aircraft built from published derivatives versus Mach, including source-axis transformations with tests.
  - Chosen tolerances recorded; validation status table in `docs/ASSUMPTIONS.md` filled in.
- **L12 CLI (T7):**
  - `run` command: scenario in, CSV time history out.
  - Determinism test: identical inputs produce byte-identical output on one machine.

**Exit:** a reference aircraft and the data-driven F-104-class jet fly headlessly; comparison tables and plots committed; deterministic outputs; `ASSUMPTIONS.md` current.

---

## Phase 2: Subsonic aerodynamic estimator (release: v0.1, Stage 2)

**Goal:** coefficients computed from geometry, validated against data.

- **L9 Estimator:**
  - Geometry structs: wing sections with airfoil, chord, taper, sweep, twist, dihedral; tails; fuselage cross-sections.
  - Lifting-line theory, tested against the elliptic-wing analytic case (induced-drag efficiency 1).
  - Compressibility correction (Prandtl-Glauert) for roughly Mach 0.7 to 0.8 and below.
  - Vortex-lattice method for arbitrary lifting-surface layouts.
  - Tail contribution and neutral point (downwash, tail efficiency).
  - Drag buildup: skin friction, form factors, wetted areas, per component.
  - Control surface effectiveness estimates.
  - Output matches the aircraft-file coefficient schema, with confidence flags.
- **L8 Aircraft data:** semantic geometry section added to the schema; version bump and migration path; round-trip tests.
- **L11 Validation harness:**
  - Analytic cases; AVL/OpenVSP golden fixtures; NASA data comparisons.
  - Error report generator: predicted versus reference, per quantity, with plots.
  - Documented list of where the estimator is worse.
- **L12 CLI:** `estimate` command: geometry JSON in, coefficients JSON and error report out.
- **L4 to L7:** no structural change; the simulator flies an estimator-generated aircraft.

**Exit:** estimator output within the chosen tolerances on the subsonic reference cases; error plots committed; the sim flies an estimated aircraft.

---

## Phase 3: Designer and report (release: v0.1, Stage 3)

**Goal:** edit a parametric aircraft, get an engineering report, fly the result.

- **L10 Designer and report:**
  - Semantic aircraft model as the source of truth: wing sections, airfoils, control surfaces, tails, fuselage, engines (referencing thrust tables), tanks, payload.
  - Parameter editing with immediate rebuild of derived geometry.
  - Component-based mass, CG and inertia buildup with stated assumptions.
  - Checks: static margin, stall speed, wing loading, thrust-to-weight, trim feasibility, CG range, "Mach above 0.8 not modeled" warning.
  - HTML report: inputs, results, assumptions, confidence flags, disclaimer ("conceptual estimates only, not for certification or flight safety").
  - Templates: a light plane and an F-104-class jet.
- **L8 Aircraft data:** design file (semantic) and derived aircraft file, both versioned.
- **L12 CLI:** `design` and `report` commands; deterministic output.
- **L9 Estimator, L4 to L7:** stable APIs consumed by the designer; no behavioural changes.
- **L11 Validation harness:** designer regression tests (same parameters, same outputs).

**Exit:** change wingspan from 11 m to 9 m via the CLI; the report shows higher stall speed and a warning; the sim behaves accordingly.

---

## Phase 4: Browser demo (release: v0.1, Stage 4)

**Goal:** anyone opens a link, edits a design, sees the report update, and flies it.

- **L13 Web app and UI:**
  - Core, estimator and designer compiled to WASM; fixed-timestep accumulator loop; clean JS boundary.
  - Parameter controls, template picker, live report panel, save and load of design JSON, shareable design links.
  - CI check that the WASM target builds; static hosting build pipeline.
  - Size and frame-time budgets measured and enforced.
- **L14 Rendering ("graphics-lite"):**
  - Viewer stack chosen here; aircraft mesh generated from the design.
  - Sky, lighting, ground plane; chase camera.
  - HUD: airspeed, altitude, Mach, throttle, control deflections.
  - Axis conversion from aerospace to render axes in exactly one place.
- **L13 Input:** keyboard and gamepad mapping; inputs are the only external drivers of the core.
- **L4 to L7:** unchanged; core remains render-free.

**Exit:** works in a current desktop browser at a stable frame rate; deterministic stepping; designs round-trip through save and load.

---

## Phase 5: v0.1 release packaging (release: v0.1)

**Goal:** a shippable first release.

- **L11 Validation harness:** validation page generated from harness outputs (predicted versus published, error plots, assumptions list).
- **L10 Designer:** example gallery of design files (light plane, F-104-class jet).
- **L1 Foundation:** version tag, changelog, release build of the web app, license file present.
- **L13 Web app:** published build reachable by a link.

**Exit:** tagged release; web build live; validation page built from current data.

---

## Phase 6: Supersonic (release: v0.2)

**Goal:** supersonic design and flight for an F-104-class jet, validated against NASA data.

- **L3 Environment:** atmosphere coverage extended only if validation cases require it.
- **L5 Aerodynamics model:**
  - Mach-regime handling across subsonic, transonic and supersonic ranges.
  - Coefficient tables across the Mach range; wave drag; aerodynamic-center shift with Mach; control effectiveness versus Mach.
  - Directional stability including vertical and ventral fin effects.
  - A low-confidence flag in the transonic band, propagated to reports and UI.
- **L9 Estimator:**
  - Separate regime modules behind one interface: subsonic (Phase 2), transonic blend (flagged), supersonic.
  - Supersonic: linearized supersonic theory, area-rule wave drag from the cross-sectional area distribution, Sears-Haack body as the minimum-drag baseline test.
- **L6 Propulsion:** afterburning turbojet with simple inlet and ram effects; thrust table generation from engine parameters; fuel flow.
- **L7 Control and trim:** trim and stability reporting across a Mach sweep.
- **L10 Designer:** area-distribution view, Mach range in checks and warnings, supersonic template.
- **L11 Validation harness:** F-104 comparisons against NASA data across Mach; error plots; documented known mismatches (transonic).
- **L13/L14:** Mach readout and supersonic template in the web demo.

**Exit:** estimator versus F-104 data across Mach with published error plots; supersonic flight in the sim and in the browser.

---

## Phase 7: VTOL (release: v0.3)

**Goal:** a quadplane hovers, transitions, flies and lands.

- **L5 Aerodynamics model:** high-angle-of-attack and edgewise-flow handling needed for transition; rotor wake interference kept simple and documented.
- **L6 Propulsion:**
  - Rotor and propeller model (actuator disk or blade-element): thrust and torque versus RPM and inflow.
  - Multiple thrust sources with positions and spin directions; tilt mechanisms.
  - Motor, speed controller and battery models (simple, cited).
- **L4 Dynamics:** multiple force and moment sources; rotor gyroscopic effects if needed; minimal ground contact (simple spring-damper contact points) so takeoff and landing can be tested.
- **L7 Control and trim:**
  - Hover attitude and altitude controllers; control allocation and mixer.
  - Transition scheduler between hover and forward flight.
  - Trim for hover and transition; linearization-based stability analysis.
- **L9 Estimator:** rotor sizing, disk loading, hover power, transition-corridor estimates.
- **L10 Designer:** rotor, motor and battery parts; VTOL checks (thrust margin, hover power, endurance); quadplane template.
- **L11 Validation harness:** momentum-theory analytic cases; published rotor data; hover and transition scenario tests with stated limits.
- **L13/L14:** VTOL control mapping and flight-mode indicator.

**Exit:** the quadplane hovers stably, transitions, cruises, and lands in the sim; analytic checks pass.

---

## Phase 8: Import, export, gear and structures (release: v0.4)

**Goal:** bring in an existing model, analyze it, and produce build-oriented output.

- **L16 Import and export:**
  - SolidCraft adapter driven through its CLI and command scripts, with a pinned version and recorded-output tests; clear behaviour when SolidCraft is absent.
  - glTF and STEP import; mesh-body handling.
  - Tagging data model mapping mesh parts to roles (wing, tail, fuselage, control surface, engine).
  - Mass properties from mesh volume and material density.
  - glTF and STEP export; project file format.
- **L9 Estimator:** geometry extraction from tagged meshes (planform, sections, areas) with extraction checks.
- **L5/L4 Landing gear:**
  - Gear parts: position, travel, spring-damper, tire friction, brakes, steering, retraction.
  - Stable ground-contact integration; takeoff and landing scenarios.
  - Checks: CG within the gear polygon, tip-back.
- **L10 Designer and report:**
  - Import and tagging workflow; mismatch warnings.
  - Structures: empirical structural weight, load-limit (V-n) basics, wing bending estimate, material library.
  - Rough cost estimate with stated ranges; "build readiness" checklist in the report.
- **L11 Validation harness:** import round-trip tests; gear-model analytic tests; structural estimates compared to published aircraft data.

**Exit:** import a model, tag its parts, analyze it, fly it, and export it.

---

## Phase 9: Cockpit and aircraft systems (release: v0.5)

**Goal:** designs carry working systems and instruments.

- **L15 Systems and cockpit:**
  - Systems framework: component graph for fuel (tanks, pumps, valves), electrical (buses, loads, generator, battery), hydraulic (pumps, accumulators, actuators); deterministic update tick; failure injection.
  - Control-surface actuator models with rate and position limits.
  - Instrument model: airspeed, altimeter, attitude, vertical speed, heading, Mach, engine gauges; warnings and cautions.
  - Instrument panels defined by data in the aircraft file.
- **L4 Dynamics:** variable mass, CG and inertia as fuel burns.
- **L8 Aircraft data:** systems and panel sections added to the schema; version bump and migration.
- **L10 Designer:** systems editor mapped to designer parts; checks for fuel range and electrical load balance.
- **L14 Rendering:** cockpit view, instrument rendering.
- **L13 Web app:** cockpit toggle; performance budgets re-checked.
- **L11 Validation harness:** conservation tests (fuel, electrical load); failure-scenario tests.

**Exit:** a design's systems drive its mass and performance; working instruments in a cockpit view; failures produce the expected effects.

---

## Phase 10: Graphics and terrain (release: v0.6)

**Goal:** the written visual target is met on target hardware.

- **L17 Terrain and world:**
  - Heightmap loading, tiling, level-of-detail streaming, ground-height query API consumed by the core as pure data.
  - Data-source and license record for every dataset.
  - World frame upgrade (geodetic or curved-Earth handling) if map scale requires it.
  - Weather: steady wind, shear, turbulence; time of day.
- **L3 Environment:** wind and turbulence feed the dynamics through a deterministic, seeded interface.
- **L14 Rendering and audio:**
  - A written visual target and benchmark scenes first.
  - Physically based materials, shadows, sky model, post-processing (tone mapping, bloom), clouds (billboards first), mesh level of detail, quality settings.
  - Basic audio (engine, wind), only after the visual target is met.
- **L13 Web app:** asset streaming; graphics-API fallback path; load-time budget.
- **L11 Validation harness:** terrain-collision tests; deterministic-wind tests; rendering benchmark scenes tracked over time.

**Exit:** terrain streams; the sim collides with terrain; the visual target is met at the stated frame rate.

---

## Phase 11: Combat and multiplayer (release: v1.0+)

**Goal:** two players fight over terrain; content can be authored and shared.

- **L18 Combat and AI:**
  - Guns with ballistic drag and dispersion; bombs.
  - Missiles reusing the estimator and propulsion layers (solid-rocket thrust curves), proportional-navigation guidance, simple seekers, fuzing.
  - Stores and hardpoints in the designer, with mass and drag effects.
  - Hit detection against collision meshes; damage feeding systems and aerodynamics.
  - AI: flight plans, intercept and engagement behaviours, traffic; seeded randomness.
- **L19 Networking and content:**
  - Authoritative server; client prediction and reconciliation using serializable state; snapshot protocol; browser-compatible transport; lag compensation; lobby; replay recording.
  - Mission editor and campaign data format.
  - Modding API: aircraft packs, validation of third-party files, safe loading.
- **L14 Audio:** combat sound set.
- **L15 Systems:** weapon systems and countermeasure hooks.
- **L11 Validation harness:** ballistics analytic cases; guidance engagement tests; network determinism and desync tests; mod-file fuzzing.

**Exit:** a two-player dogfight over streamed terrain; a mission loads from data; a third-party aircraft pack loads and validates.

---

## Layer-by-phase matrix

`+` means new or extended work in that phase. Blank means no change.

| Layer | P0 | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 | P9 | P10 | P11 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| L1 Foundation | + | | | | + | + | | | | | | |
| L2 Math and frames | | + | | | | | | | | | | |
| L3 Environment | | + | | | | | + | | | | + | |
| L4 Dynamics | | + | | | | | | + | + | + | | |
| L5 Aerodynamics model | | + | | | | | + | + | + | | | |
| L6 Propulsion | | + | | | | | + | + | | | | |
| L7 Control and trim | | + | | | | | + | + | | | | |
| L8 Aircraft data | | + | + | + | | | | | | + | | |
| L9 Estimator | | | + | | | | + | + | + | | | |
| L10 Designer and report | | | | + | | + | + | + | + | + | | + |
| L11 Validation harness | | + | + | + | | + | + | + | + | + | + | + |
| L12 CLI | | + | + | + | | | | | | | | |
| L13 Web app and UI | | | | | + | + | + | + | | + | + | |
| L14 Rendering and audio | | | | | + | | + | + | | + | + | + |
| L15 Systems and cockpit | | | | | | | | | | + | | + |
| L16 Import and export | | | | | | | | | + | | | |
| L17 Terrain and world | | | | | | | | | | | + | |
| L18 Combat and AI | | | | | | | | | | | | + |
| L19 Networking and content | | | | | | | | | | | | + |
