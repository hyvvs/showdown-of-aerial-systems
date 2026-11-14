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
- T5's version-zero JSON stores SI values with explicit frames, a signed inertia tensor about the CG, and moments about a separately identified aerodynamic reference point. JSON conversion is text-only; filesystem access stays outside the pure core. Unknown fields/versions, invalid shapes, non-finite numbers and invalid required bounds are errors on load and save. Numeric validation is not proof of physical realizability or provenance.
- Coefficients use rectangular Mach/optional-geometric-altitude tables. T6 implements linear/bilinear interpolation for every coefficient, exact at nodes and errors outside tabulated ranges; a Mach-only table is altitude-independent within the simulator's supported range. Full format: `docs/AIRCRAFT_FORMAT.md`.
- Every value in the test aircraft is explicitly synthetic, with a per-value ledger. It is not a real-aircraft model and supplies no evidence toward H3/D9/T10 validation.
- Review Fix 1 restricts named aircraft/scenario records to JSON objects at the validated file-loading entry points. Positional struct arrays are rejected with field paths; actual vectors, matrices and table/schedule containers remain arrays. A shape preflight followed by typed parsing of the original text preserves unknown/duplicate/missing-field and line/column errors, at the cost of parsing twice when loading. Direct generic Serde decoding is not a substitute for the validated file-loading API.

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

## Headless scenarios (T7)
- Only soas-cli performs file I/O. Version-zero scenarios use geometric NED position, body velocity/rates, Euler degrees converted at the file boundary, and piecewise-linear per-surface degree schedules sampled at every RK4 stage time. All external packages were already present; argument parsing uses std.
- Duration and output interval must align with the fixed step; no partial final integration step is substituted. Initial and final states are output. CSV uses a fixed unit-labeled column order, scalar-first quaternion and unrounded Rust f64 Display. Byte determinism is checked for the same inputs on one binary/machine only.
- The runner buffers its samples and CSV in memory. Invalid scenarios/aircraft/simulation steps leave prior output untouched; disk writes themselves are not atomic. Scenario/aircraft input paths cannot also be output paths. Full contract: `docs/SCENARIOS.md`.
- A synthetic run crossing CLmax and an internal altitude-table node gives irregular observed orders (1.1966, 1.3744, 2.2618, 0.1575, 5.9312, 1.5848), not stable fourth order. At 0.0025 s, endpoint differences versus 1/12800 s are 3.56616e-6 m position, 6.27742e-6 m/s velocity, 2.07997e-7 rad attitude and 1.91875e-6 rad/s rates. A second fine reference changes the combined estimate by 0.051% of the default estimate. D10 stays unchanged; do not assume a 16x gain on halving across nonsmooth transitions. Full scenario, event brackets and bounds: `data/reference/aerodynamics/transitions.md`.

## Aerodynamics (initial model)
- Quasi-steady aerodynamics (no unsteady effects).
- Linear lift and pitching-moment coefficients per T6, an upper CLmax cap, and quadratic induced drag. The cap is not a validated post-stall model; no negative-stall curve is specified. Side force is linear in beta; rolling/yawing control moments use linear derivatives. No rate derivatives, aerodynamic damping or physical control stops are added.
- Subsonic only until v0.2. Mach dependence comes from the supplied coefficient table; no additional compressibility correction is implemented or claimed by T6.
- **Mach > 0.8 is out of scope until v0.2.** v0.1 must display the explicit warning "Mach > 0.8 not modeled"; Mach-indexed input tables do not imply supersonic support.
- No ground effect, no propwash on tail, no aeroelastic effects.
- The coupled model uses geometric h=-z and the geometric-altitude atmosphere API. It transforms wind forces (-drag,+side,-lift) to body axes and transfers reference-point moments to the CG using (reference-CG) cross force. Control signs follow D6. D13's 0.1 m/s placeholder, zero fallback and clamped asin are implemented; non-finite inputs error before fallback/clamping.
- Every RK4 stage reevaluates aerodynamic loads, atmosphere and coefficients. Fallible loads abort the whole step; final-state validity is checked too. No silent zero-load substitution is used.
- The shared synthetic aircraft's 0.5 s aero-coupled step-halving check gave ratios 16.117 to 16.934. At 1/400 s, differences against a 1/6400 s reference were 2.682651359844e-10 m position, 5.143524015678e-10 m/s body velocity, 6.439822566515e-12 rad attitude and 4.680008398830e-11 rad/s body rates. A 1/12800 s reference check stayed below 5% of each default-step estimate. This supports retaining D10 for this smooth synthetic case only; it is not aircraft validation or evidence about stall/table-boundary transitions. Inputs, sources, numerical bounds and full results: `data/reference/aerodynamics/README.md`.

## Propulsion
- To be filled in when the engine model task starts. Note: thrust is not simply `power / speed` (that diverges at zero airspeed).
- T8 optionally accepts a temporary constant applied body-x force through the CG, default 0 N, only for the trim calculation. This is not a propulsion model or a new powered-run scenario input; it includes no fuel flow or aerodynamic interaction.

## Trim and static margin (T8)
- Local steady straight longitudinal flight at a specified airspeed and geometric altitude: solve alpha, elevator and gamma (positive climb) from path-parallel force, upward-normal force and pitching moment about the CG. Body pitch=alpha+gamma; bank, sideslip and rates are zero. Both T*cos(alpha) and T*sin(alpha) are included. A glide trim is local at that atmosphere, not a guarantee of constant speed along changing altitude/density.
- The numerical solver searches the forward nonvertical branch |alpha|,|gamma|<pi/2. No physical actuator limits are invented. Success requires explicit force/moment tolerances, including no residual lateral imbalance. Failed iteration limits, singular Jacobians and failed searches return non-convergence errors, never best-effort solutions. Defaults 1e-8 N/Nm and 1e-5 rad difference step are numerical settings, not D9 aircraft tolerances.
- Control-fixed static margin is -(dCm_CG/dalpha)/(dCL/dalpha), evaluated by central differences of the table-driven coefficient laws at the trim condition with speed/Mach, geometric altitude and elevator fixed. Moment transfer to the CG is included; the result is a chord fraction. The stencil reaching CLmax or an unresolved/nonpositive lift slope is an explicit undefined-margin error.
- This report ignores elevator-free motion/control feedback, rate/unsteady effects, structural flexibility, loading changes, thrust derivatives/propulsive interactions and changes in atmosphere/Mach during the perturbation. It does not establish dynamic stability or near-stall physical validity. Stable/unstable synthetic configurations give +0.125/-0.125 chord; full force residuals in the analytic glide/level cases are below 2.63e-10 N. Sources, cases and limits: `data/reference/trim/README.md`.

## Known weak regions
- Mach > 0.8, including the transonic range: not modeled in v0.1; transonic modeling remains a low-confidence region when introduced from v0.2.
- Large angles of attack, spins, and tumbling: not modelled.

## Validation status
| Case | Reference | Quantities compared | Error | Status |
|---|---|---|---|---|
| (none yet) | | | | |
