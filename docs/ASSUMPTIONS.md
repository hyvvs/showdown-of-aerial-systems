# Model assumptions and limitations

Keep this honest and current. Every simplification the code makes belongs here. Reports and the README link to it.

## Environment (v0.1)
- Flat, non-rotating Earth; constant gravity.
- International Standard Atmosphere, no wind, no turbulence.
- The dry-air atmosphere has two explicit APIs: `isa_at_geopotential_altitude` accepts finite geopotential H in [0, 20000] m inclusive; `isa_at_geometric_altitude` accepts finite geometric Z in [0, 20000] m inclusive and converts H = r0 Z / (r0 + Z) after checking that range. Z = 20000 corresponds to H = 19937.272279 m. No generic altitude input silently chooses a definition; out-of-range and non-finite inputs are errors.
- T3 compares temperature, pressure, density and sound speed against U.S. Standard Atmosphere 1976, Tables I and III (four geopotential rows and one geometric row). This verifies those checkpoints, not weather or real-aircraft behaviour. Sources, values, measured errors and deliberate-error checks are in `data/reference/atmosphere/README.md`.
- At H = 5000 m the PDF prints **54019 Pa**, while the equation gives **54019.912103762079 Pa**: difference **+0.912103762079 Pa** (same absolute magnitude), relative difference **1.688486944e-5**. On 2026-11-13 the owner selected **5e-5 relative tolerance for pressure and density**, accommodating the recorded printed-table discrepancy while rejecting the requested lapse-rate, gravity, height-definition and gas-constant mistakes. No other row needs a larger tolerance. All transcribed digits were rechecked and unchanged. Using the exact listed R*, M0 and g0 with 50-digit arithmetic gives 54019.912103762073475... Pa, so does not resolve the discrepancy. Tabulation/printing or undocumented calculation differences remain hypotheses; their cause is unverified. This is not a D9 aircraft-validation tolerance.

## Aircraft (v0.1)
- Rigid airframe (no flexing, no flutter).
- Mass properties constant during a run unless a task says otherwise.

## Attitude helpers
- Body-to-NED Hamilton unit quaternions are checked against analytic right-handed axis rotations. Composition renormalizes the quaternion.
- Euler angles are for I/O only. Roll/yaw output is rejected when `abs(cos(pitch)) <= 64 * f64::EPSILON`; near this singularity the individual angles are ill-conditioned. This numerical guard does not limit the quaternion orientation.
- Numerical test tolerances and synthetic angles are not real-aircraft validation tolerances.

## Aerodynamics (initial model)
- Quasi-steady aerodynamics (no unsteady effects).
- Linear lift and moment curves until a simple stall limit; post-stall behaviour is crude.
- Subsonic only until v0.2; compressibility handled by a correction below about Mach 0.7 to 0.8.
- **Mach > 0.8 is out of scope until v0.2.** v0.1 must display the explicit warning "Mach > 0.8 not modeled"; Mach-indexed input tables do not imply supersonic support.
- No ground effect, no propwash on tail, no aeroelastic effects.

## Propulsion
- To be filled in when the engine model task starts. Note: thrust is not simply `power / speed` (that diverges at zero airspeed).

## Known weak regions
- Mach > 0.8, including the transonic range: not modeled in v0.1; transonic modeling remains a low-confidence region when introduced from v0.2.
- Large angles of attack, spins, and tumbling: not modelled.

## Validation status
| Case | Reference | Quantities compared | Error | Status |
|---|---|---|---|---|
| (none yet) | | | | |
