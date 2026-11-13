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
- Lifting-line theory first (it has analytic test cases), then a vortex-lattice method with a compressibility correction.
- Validate: the selected Navion or fallback Cessna 172 against power-off reference data and applicable AVL/OpenVSP output, within the v0.1 Mach limit. Flying-wing validation is parked until after v0.2.
- Exit: published error plots; ASSUMPTIONS.md updated.

### Stage 3: Designer model and report
- Parametric parts (wing sections, tail, fuselage, engine) as the source of truth; headless CLI producing the aircraft file and an HTML validation report.
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
