# Initial aerodynamic model and numerical checks (T6)

## Scope and provenance

This is the TASKS.md T6 model, using D3/D4/D6/D11/D12/D13 and the geometric
altitude convention. There are NO built-in real-aircraft parameters. Every
aircraft number comes from the caller's JSON. All aircraft parameters and
scenarios in these tests are synthetic mathematical TEST VALUES, not measurements
or estimates of any real aircraft. The shared JSON fixture and per-value ledger
are under `crates/soas-core/tests/fixtures/`.

New formula references verified online on 2026-11-14:

- [NASA Glenn, Lift Equation](https://www1.grc.nasa.gov/beginners-guide-to-aeronautics/lift-equation/):
  lift = dynamic pressure * reference area * lift coefficient. Rearranging this
  relation with lift = weight and CL = CLmax gives the one-g stall-speed formula.
- [NASA Glenn, Drag Coefficient](https://www1.grc.nasa.gov/beginners-guide-to-aeronautics/drag-coefficient/):
  induced-drag coefficient CL^2/(pi*AR*e), aspect ratio span^2/area and the Mach
  definition. No example aircraft parameters from that page were copied.
- [JSBSim manual, Frames of reference](https://jsbsim-team.github.io/jsbsim-reference-manual/user/concepts/frames-of-reference/):
  frame descriptions; D12 explicitly selects instantaneous stability axes.
  Wind forces here follow the owner's chosen (-drag,+side,-lift) components.

Moment coefficient normalization and body-axis notation follow
[Caughey, Introduction to Aircraft Stability and Control (2011), section 4.3](https://courses.cit.cornell.edu/mae5070/Caughey_2011_04.pdf#page=57).
Control directions and their verification notes are already recorded in
CONVENTIONS.md: Beard & McLain (2012), chapter 4, could not be directly verified;
Caughey section 1.2.2 was verified as the explicit cross-check. The owner-selected
signs remain binding. Moment transfer uses the definition of a moment, r cross F;
r runs from the CG to the aerodynamic reference point, in body axes.

## Equations and interfaces

`evaluate(aircraft, state, controls)` computes geometric h = -NED z, obtains
the geometric-altitude ISA atmosphere, calculates Mach and looks up coefficients
using Mach and geometric altitude. `lookup_coefficients` also exposes that lookup
directly. Exact nodes return stored values; between nodes every coefficient is
linearly/bilinearly interpolated. Explicit-axis bounds are errors, with no
extrapolation/clamping. A Mach-only table is altitude-independent. See
`docs/AIRCRAFT_FORMAT.md` for shapes, singleton axes and validation.

The finite-input check precedes the D13 low-speed fallback and asin clamping.
Below 0.1 m/s, alpha and beta are zero; at/above it, alpha = atan2(w,u),
beta = asin(clamp(v/V,-1,1)). The threshold is the owner's placeholder.
The beta helper rejects non-finite ratios before clamping. Airspeed uses hypot
to avoid overflow from squaring finite components; an unrepresentable result
still returns an error.

```text
q = rho V^2 / 2                       AR = span^2 / area
CLraw = CL0 + CLalpha * alpha         CL = min(CLraw, CLmax)
CD = CD0 + CL^2 / (pi e AR)           CY = CYbeta * beta
Cm = Cm0 + Cmalpha*alpha + Cmde*elevator
Cl_roll = Clda * aileron              Cn_yaw = Cndr * rudder
wind force = q area * (-CD, +CY, -CL)
body moment at reference = q area * (span*Cl_roll, chord*Cm, span*Cn_yaw)
body moment at CG = body moment at reference + (reference - CG) cross body force
```

Angles/controls are radians. The wind-to-body rotation is
R_body_from_stability(alpha) * R_stability_from_wind(beta):

```text
[ cos(alpha)  0  -sin(alpha) ] [ cos(beta) -sin(beta) 0 ]
[     0       1       0     ] [ sin(beta)  cos(beta) 0 ]
[ sin(alpha)  0   cos(alpha) ] [     0          0     1 ]
```

`step_aircraft` reevaluates these loads and atmosphere at every RK4 stage and
checks the final combined state too. `FixedStepRk4::try_step` adds fallible loads
to the existing dynamics API; any load/dynamics failure aborts the whole step,
without changing its borrowed input. No zero-load substitution, file I/O,
wall-clock time or extra integration scheme is introduced.

Mach > 0.8 returns the explicit error **"Mach > 0.8 not modeled"**. Geometric
heights outside [0,20000] m are errors. Source coefficients and intermediate
results must remain finite. Negative lift-stall behaviour, physical control
limits, rate derivatives, aerodynamic damping and nonlinear post-stall physics
are not supplied. The requested CLmax is implemented as an upper cap, not a
fitted post-stall curve or an invented negative lift limit. At zero airspeed,
loads vanish; below the angle threshold but above zero, q still uses actual V.
Mach dependence is supplied only by the input table; no additional compressibility
correction is claimed. Test convergence below does not validate any aircraft.

## Tests and failure modes

The tests-only commit precedes implementation. Initial output was:

```text
error[E0432]: unresolved import `soas_core::aerodynamics`
error[E0599]: no method named `try_step` found for struct `FixedStepRk4`
error: could not compile `soas-core` (test "aerodynamics") due to 2 previous errors
```

Boundary review added a regression BEFORE its fix and included it in the
tests-only commit. The interim implementation produced:

```text
test signed_zero_at_sea_level_and_zero_mach_matches_the_zero_node ... FAILED
panicked at crates\soas-core\src\aerodynamics.rs:194:25:
attempt to subtract with overflow
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out
```

Total float ordering distinguishes -0.0/+0.0. The fix uses numeric equality
for finite table values, including h=-z at sea level. No test was weakened.

| Test | Check / failure it detects |
|---|---|
| Exact table nodes | All four cells must match exactly; wrong indexing or node rounding fails. |
| Signed zero | Negative-zero Mach/height equals the zero node; sea-level h=-z must not panic. |
| Linear/bilinear interpolation | Independent Mach, altitude and interior averages; wrong fractions or axis ordering fail. |
| All coefficients interpolate | Independently vary every non-lift coefficient too; copying only lift fields or one endpoint fails. |
| Bounds/singletons/Mach-only | Reject outside/non-finite coordinates and malformed tables; singleton only accepts its node, Mach-only data stay altitude-independent. |
| Low-speed threshold | Zero fallback strictly below 0.1 m/s; the exact boundary uses angles; atan2 quadrants tested. |
| Asin and non-finite input | +/-1 plus roundoff clamps to +/-pi/2, but NaN/Inf in any velocity component must error. |
| Axis signs | Independent quarter-turn basis expectations and (-drag,+side,-lift) at zero angles catch sign/transposition mistakes. |
| Combined axes | Wind x follows velocity, stability y equals body y, shared z and right-handed orthonormal axes catch composition/order mistakes. |
| Elevator | Positive deflection must reduce pitch moment and make it negative in the fixture case. |
| Aileron | Positive deflection must make roll positive without yaw. |
| Rudder | Positive deflection must make yaw negative without roll. |
| Side force/CG transfer | Positive beta with the synthetic negative derivative produces negative side force; all-axis r cross F uses reference minus CG. |
| Hand lift = weight | Mass 10 kg, alpha 0.1 rad, S=2 m^2, CL0=0.2, CLalpha=4/rad: CL=0.6. With the T3 sea-level density, V=11.550926433078 m/s yields 98.0665 N. Limit 1e-9 N; wrong coefficient/area/q/lift scaling fails. This is not a trim solve. |
| Local steady glide | Choose CL=0.6, CD=0.02+0.6^2/(pi*0.8*8), tan(descent angle)=CD/CL and mass from the resultant. The local aerodynamic/gravity forces and moments balance; L/D equals the independent formula. This is a local equilibrium at the chosen atmosphere, not an indefinitely constant descent through varying density. |
| One-g stall boundary | With synthetic CLmax=1.2, max model lift crosses weight at 8.167738409816 m/s versus formula 8.167738409817 m/s. Below/above cases and a force-model bisection must agree within 1e-10 relative. Checks the upper-lift-limit boundary, not dynamic stall. |
| Geometric altitude/envelope | 20 km geometric q matches the T3 geometric density; wrong API, unsupported Mach/height, NaN velocity or nonzero zero-speed loads fail. |
| Coupling errors | Crossing the atmosphere boundary within a stage aborts; caller input stays unchanged and the fallible API retains the original error. |
| Aero-coupled convergence | Shared JSON aircraft, nonzero alpha/beta/all controls and angular rates, evolving atmosphere/table coefficients and loads at all stages. Wrong order/coupling or unresolved reference errors fail. |

All bounds and synthetic choices in these checks are numerical software-test
criteria, not D9 aircraft-validation tolerances.

## Aero-coupled step-size convergence (2026-11-14)

Uses the shared synthetic JSON unchanged. Start at NED (0,0,-1000) m with body
velocity (20,1,2) m/s, body rates (0.05,-0.03,0.02) rad/s and Euler I/O angles
(roll,pitch,yaw) = (0.03,0.08,0.2) rad. Fixed controls are elevator -0.02,
aileron +0.01 and rudder -0.01 rad. Integrate 0.5 s with the actual aerodynamic
model, geometric ISA and table lookup at every stage. No endpoint is stalled.

There is no claimed analytic solution. Differences below are against the same
model at reference step **1/6400 s = 0.00015625 s**. A second reference at
**1/12800 s** checks reference resolution.

| Step (s) | Position norm difference (m) | Body velocity norm difference (m/s) | Attitude angle difference (rad) | Body rate norm difference (rad/s) |
|---:|---:|---:|---:|---:|
| 0.02000 | 1.191385226526e-6 | 2.327093445401e-6 | 2.789815836192e-8 | 2.109308053946e-7 |
| 0.01000 | 7.124693883673e-8 | 1.374209349731e-7 | 1.690381254780e-9 | 1.248416930924e-8 |
| 0.00500 | 4.353146294017e-9 | 8.347513564578e-9 | 1.039449047103e-10 | 7.591555068419e-10 |
| 0.00250 (D10) | 2.682651359844e-10 | 5.143524015678e-10 | 6.439822566515e-12 | 4.680008398830e-11 |
| 0.00125 | 1.626941912417e-11 | 3.186659243118e-11 | 3.984738801259e-13 | 2.903767503488e-12 |

All successive halving ratios are 16.117 to 16.934 (test interval 14 to 18).
At D10 the test bounds are respectively 1e-5 m, 1e-4 m/s, 1e-6 rad and
1e-5 rad/s. Both finer references differ by 2.388472827992e-12 m,
3.389243282775e-13 m/s, 7.763939559583e-15 rad and 8.169055794461e-15 rad/s;
each must be <5% of the default-step difference. The position difference is
about 0.89% of the default estimate; the finest results approach roundoff limits.

This supports retaining **1/400 s** for this smooth, short, synthetic aero-coupled
case. D10 is unchanged. It does not establish a universal step for real aircraft,
long runs, stall transitions, table derivative discontinuities or stiff future
models. Such changes need new convergence evidence, and any default change
must be a new owner decision.
