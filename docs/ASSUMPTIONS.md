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

## Rigid-body integration (T4)
- Six degrees of freedom about the CG: three translations and three rotations; constant mass and a full signed body-axis inertia tensor. Applied forces exclude gravity; moments are about the CG. Constant NED-down gravity is added exactly once. No aerodynamic, propulsion, contact or fuel-burn model is added by T4.
- Classical fixed-step RK4, parameterized with the unchanged D10 default 0.0025 s. All four stages evaluate the state-based load callback; callers must keep it pure and deterministic. Quaternion components are advanced with the coupled state and normalized after each step; Euler angles are never integrated. Non-finite inputs or intermediates are explicit errors.
- State contains only position, body velocity, body-to-NED attitude and body angular rates, with units/frames in names; serialization support is deferred. Mass properties and step size are separate. The caller tracks time by step count; the present load callback has no explicit time argument.
- Inertia validation checks finite, exactly symmetric, positive-definite entries and a finite inverse. It does not prove physical realizability or correct source axes; those still require data validation. Products-of-inertia construction negates all off-diagonals (D11).
- Synthetic spinning free fall at 1 s gave fourth-order step-halving ratios about 16. At 0.0025 s, measured position error was 5.585205047472e-8 m and orientation error 6.666444063441e-10 rad. This supports retaining the default for this smooth test, not arbitrary future aircraft dynamics; repeat convergence checks for new force models. Maximum quaternion norm error over 100000 steps was 2.220446049e-16 (limit 5e-15); the symmetric-top angular-momentum magnitude drift was 9.830333712e-16 relative over 10 s. These are numerical analytic checks, not real-aircraft validation or exact long-term conservation guarantees. Sources, synthetic inputs and test limits: `data/reference/dynamics/README.md`.

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
