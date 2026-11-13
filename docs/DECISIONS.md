# Decisions log

Agents: do not make an open decision yourself. Present options with trade-offs and wait for the owner.

| ID | Decision | Status | Notes / recommendation |
|---|---|---|---|
| D1 | Project name | DECIDED | "Showdown of Aerial Systems". "aero" was only a placeholder. Repo slug: `showdown-of-aerial-systems`; crate prefix: `soas`; first crate: `soas-core`. Renaming is deferred to T0b. |
| D2 | License | OPEN | Remains OPEN by owner choice. The repo stays private; no LICENSE file. Must be decided before the repo is made public. |
| D3 | Aircraft file format | DECIDED | JSON, with a `format_version` field and unit suffixes in field names. Required before T5; `serde_json` is pre-approved in AGENTS.md. |
| D4 | Units strategy | DECIDED | Required before T2; owner chose option B: SI internally, unit suffixes where ambiguous, and angle newtypes only for now. Full policy below. |
| D5 | Web viewer stack | DEFERRED | Decide at Stage 4. Options: Bevy (Rust-native, heavy on web) or a JS viewer (e.g. Three.js) on a WASM core. Do not decide earlier. |
| D6 | Control-surface sign conventions | DECIDED | Positive elevator: trailing edge down, nose-down moment; positive aileron: left trailing edge down and right trailing edge up, positive rolling moment; positive rudder: trailing edge left, nose-left yaw moment. Each surface needs a sign test. Textbook source and verification limits are in CONVENTIONS.md. Required before T6. |
| D7 | SolidCraft integration | DEFERRED | Plan: drive its CLI/command scripts, not its Rust internals; v0.4. Pin a version. Its project moves fast. |
| D8 | First supersonic validation aircraft | OPEN | Candidate: F-104, using [NASA CR-96008](https://ntrs.nasa.gov/citations/19690022405) and related NTRS reports. Remains OPEN until the owner completes H4; data completeness and suitability are not yet established. |
| D9 | Validation tolerances | OPEN | Proposal only: about +/-10% for stall speed. Owner confirms after seeing the reference data (task H3). |
| D10 | Default time step | DECIDED | Owner-selected default `1/400 s` (`0.0025 s`), always a parameter. T4's step-size convergence test may justify revising it; record any change as a new decision. |
| D11 | Inertia off-diagonal signs | DECIDED | Option A: store signed tensor entries about the CG in body axes. Provide a constructor from products of inertia that negates off-diagonal terms correctly, with tests. |
| D12 | Wind and stability axes | DECIDED | Option A: instantaneous stability and wind axes as defined in CONVENTIONS.md; wind-axis force components `(-drag, +side, -lift)`. |
| D13 | Near-zero airspeed | DECIDED | Option A: one named constant `MIN_AIRSPEED_FOR_ANGLES_MPS`, initially `0.1 m/s`, an owner-selected placeholder to revisit. Below it, alpha and beta are zero; otherwise alpha is `atan2(w, u)` and beta is `asin(clamp(v/V, -1, 1))`. Non-finite inputs are an explicit error, never silently clamped. |
| D14 | Hero aircraft | DECIDED | Supersonic jet, F-104-class. The hero demo is targeted for v0.2; v0.1 stays subsonic. |

## D4: Units strategy — option B

- SI units internally: metres, kilograms, seconds, newtons, pascals, kelvin, radians.
- Unit suffixes in names wherever a unit is ambiguous, e.g. `span_m`, `mass_kg`, `alpha_rad`.
- Newtype wrappers for angles only for now: `Radians` and `Degrees`, with explicit, tested conversions.
- Non-SI units (degrees, knots, feet) appear only at the UI/report boundary, converted by tested functions.
- Adding any further newtype, or the `uom` crate, requires the owner's approval.
- Revisit `uom` only if unit bugs keep recurring.
