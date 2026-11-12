# Vision

Working title: **aero** (placeholder).

## One sentence
An open-source, browser-friendly tool where you design an aircraft (fixed wing, flying wing, supersonic jet, VTOL), get an honest engineering analysis, and fly it, with the same aircraft file feeding both.

## The core loop
```
Aircraft description (wings, tail, fuselage, engines, mass, gear)
        |
        +--> Aero estimator --> coefficients --> Validation report ("does it fly?")
        |                              |
        |                              v
        +----------------------> Flight sim core --> you fly it
```
Everything else in this project is a feature layered on that loop.

## Who it is for
- Builders, RC hobbyists, students: "will my idea fly, and how will it behave?"
- Players who like building things (inspired by Flyout-style plane builders).
- Engineers who want a fast conceptual-stage sanity check.

## What exists already (we should not claim novelty we do not have)
- Analysis tools: OpenVSP, AVL, XFLR5.
- Flight dynamics: JSBSim (open source).
- Commercial browser tools combining design and simulation exist (e.g. SIMNET Designer).
- Academic tools combine vortex-lattice aerodynamics with flight simulators.

Our angle: **open source, free to try in a browser, every number traceable and validated, game-like feel, one file for both analysis and simulation.**

## Accuracy stance
- Target is **conceptual-design fidelity**: good for "does it fly, how fast does it stall, is it stable", with typical errors of tens of percent. That is the same class as AVL/OpenVSP.
- It is **not** DCS-level fidelity, which depends on manufacturer data and flight-test tables for a specific aircraft.
- Every release publishes a validation page: predicted vs. reference values, with error plots and a list of what the model ignores (`docs/ASSUMPTIONS.md`).
- Transonic flight (about Mach 0.8 to 1.2) is low-confidence and is flagged as such in reports and UI.
- Reports include a clear "conceptual estimates only, not for certification or flight safety" notice.

## Aircraft scope (in order of release)
1. v0.1: subsonic fixed wing, including conventional and flying-wing layouts.
2. v0.2: supersonic (separate Mach-regime estimator modules behind one interface).
3. v0.3: VTOL (rotor model, transition, hover controller).

## Releases after v0.3 (owner's ranking)
1. v0.4: Import 3D models; real-life export; landing gear; structural, mass, and cost estimates.
2. v0.5: Cockpit and aircraft systems (fuel, electrical, hydraulics, instruments).
3. v0.6: Graphics and terrain, against a defined visual target.
4. v1.0+: Combat and multiplayer.

## Principles
1. Every release stands alone and is worth showing.
2. Every number has a source.
3. Verify before trusting (analytic tests, reference tools, published data).
4. Keep the sim core free of rendering code; keep state serializable; fixed time step (keeps multiplayer possible later).
5. No feature creep inside a release; extra ideas go to the Parking lot.
6. Be honest about limits.

## Success
A project the owner is proud of, that people can try in one click, that is honest about its accuracy, and that others want to use and contribute to. Fame is not controllable; quality and honesty raise the odds.
