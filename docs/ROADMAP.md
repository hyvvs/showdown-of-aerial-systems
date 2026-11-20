# Roadmap

**Current release: v0.1**. Anything not listed under the current release goes to the Parking lot at the bottom.
Effort figures are guesses at 20+ hours/week for a beginner, not predictions.

## v0.1: Subsonic fixed wing, designer, report, browser demo (about 4 to 6 months)

The hero demo is an **F-104-class supersonic jet**, targeted for v0.2 (D14). v0.1 stays subsonic and displays the explicit warning **"Mach > 0.8 not modeled"**.

### Stage 0: Learn and set up (3 to 4 weeks)
- Tasks: H1, H2, T0, T1.
- Exit: CI is green; the owner can explain lift, trim, and static margin in their own words; conventions are written.

### Stage 1: Simulation core in Rust
- Tasks: T2 to T10.
- Exit: a hand-written reference aircraft flies headlessly, with the Navion from NASA CR-96008 as the candidate (fallback: Cessna 172); power-off stall, glide, trim and static-margin comparisons match the reference data within the tolerances the owner chose (D9); plots and comparisons committed. Propulsion parameters and powered comparisons are not required for v0.1 validation.

### Stage 2: Subsonic aerodynamic estimator
- Owner-authorized start: T12 wing planform geometry, then T13 straight-wing
  Prandtl/Glauert lifting-line theory in a new soas-estimator crate. It depends
  on soas-core; the core does not depend on the estimator. This batch does not
  start T10 or T11 or satisfy their outstanding reference-validation work.
  Analytic/synthetic checks establish software evidence only; outputs list
  assumptions and mark physical validity unassessed, without invented thresholds.
- Lifting-line theory first (it has analytic test cases), then a vortex-lattice method with a compressibility correction.
- Current authorized extension: T14 unit horseshoe influence coefficients, then
  T15 planar vortex-lattice solver with mesh-convergence and moment/neutral-point
  checks. Classical lifting line assumes a straight wing of high aspect ratio;
  the hero aircraft's low-aspect-ratio swept wing is very different, motivating
  this panel method. These tasks stay incompressible/attached/small-angle and do
  not yet add the later compressibility correction or validate hero-wing results.
  Mark those results "validity unassessed" until compared with data. No T10/T11.
- Current reference direction: wing-alone first, TN 1270 conditional; the T16c
  source-closure verdict is NO-GO for a fully sourced experimental comparison.
  For this outcome the owner's v0.1 category is **"code-to-code cross-checked,
  not experimentally validated"**, tied to actual existing evidence, not an
  implied AVL/OpenVSP run. RM SL56H06 is a v0.2 source candidate; CRM-HL is
  excluded from the first comparison. (Owner T16c Decisions/requirement 4;
  T16C_TN1270_CLOSURE.md §§1/3/6; T16B_REFERENCE_SEARCH.md §§5-6/8.)
- Next direction is Stage 3 after the owner's pre-modeling checkpoint and a
  separately scoped task. Experimental error plots remain unavailable while
  source gates are open; no experimental-validation exit claim is made.
  Flying-wing validation remains parked until after v0.2. (Owner T16c
  requirement 4; ESTIMATOR_PLAN.md, Current scope; closure §§4/6-7.)

### Stage 3: Designer model and report
- Parametric parts (wing sections, tail, fuselage, engine) as the source of truth; headless CLI producing the aircraft file and an HTML validation report.
- T16c route: first agree the input/report interface, provenance, evidence
  labels and unavailable outputs at the checkpoint. Existing isolated-wing
  capability cannot silently supply missing whole-aircraft or stall outputs;
  the report must preserve that distinction. No Stage 3 implementation or
  prerequisite model task is authorized by the audit. (Owner T16c request;
  T16C_TN1270_CLOSURE.md §6; ESTIMATOR_PLAN.md, Current scope.)
- Exit: change wingspan 11 m to 9 m, report shows higher stall speed and a warning.

### Stage 4: Browser demo
- Sim core compiled to WASM; sliders; live "does it fly" report; fly with keyboard; time-boxed "graphics-lite" (decent sky, lighting, simple ground).
- Exit: anyone can open a link and use it.

### Stage 5: v0.1 launch
- README with GIF, example gallery for the selected power-off reference aircraft, validation page, contribution guide.

## Later releases (details written when reached)
- v0.2: Supersonic (Mach-regime modules, wave drag, turbojet/afterburner, low-confidence flags); F-104-class hero demo and T11's data-driven headless aircraft, blocked until H4 is complete.
- v0.3: VTOL (rotor model, transition, hover controller).
- v0.4: Import 3D models, real-life export, landing gear, structural/mass/cost estimates.
- v0.5: Cockpit and aircraft systems.
- v0.6: Graphics and terrain.
- v1.0+: Combat and multiplayer.

## Parking lot (ideas that are not in the current release)
- After v0.2: flying-wing validation against AVL/OpenVSP output.
