# Local trim and control-fixed static margin (T8)

## Sources and definitions

- [MIT 16.100, Equations of Aircraft Motion, pages 1-2, equations (2)-(3)](https://ocw.mit.edu/courses/16-100-aerodynamics-fall-2005/678fcdd7f52560f148826442b6885f3e_16100lecture8_cg.pdf)
  resolves forces along and normal to the flight path, including inclined
  thrust. Its flight-path symbol theta is called **gamma** here; its thrust
  inclination alpha_T equals our alpha for thrust along body x.
- [Caughey, Introduction to Aircraft Stability and Control (2011), printed
  page 28, equations (3.20)-(3.21)](https://courses.cit.cornell.edu/mae5070/Caughey_2011_04.pdf#page=34)
  relates pitching-moment/lift slope to the control-fixed static margin. The
  chain rule gives the requested ratio below. Positive margin is restoring
  longitudinal static stability when the lift slope is positive; negative
  margin is destabilizing. It is not a complete dynamic-stability assessment.

These passages were verified online 2026-11-14. No real-aircraft numbers from
either source are used. The atmosphere/gravity sources remain T3; aerodynamic
equations and signed moment transfer remain T6. All aircraft configurations and
conditions in the tests are explicitly SYNTHETIC TEST VALUES from configurations
of the sole existing synthetic JSON fixture.

## What trim means here

At the requested airspeed V and **geometric** height h, solve three unknowns:
alpha, elevator and flight-path angle gamma. Gamma is positive climb. In the
zero-bank, zero-sideslip, zero-body-rate condition, body pitch = alpha + gamma.
The geometric-atmosphere API and the actual Mach/altitude tables are evaluated
at that condition; coefficients are not silently replaced with sea-level values.

```text
parallel force = T cos(alpha) - D - W sin(gamma) = 0
upward-normal force = L + T sin(alpha) - W cos(gamma) = 0
pitching moment about CG = 0
W = mass * g0
```

`TrimRequest.applied_thrust_n` is an optional **temporary constant body-x force
through the CG**, default **0 N**. It is not an engine model, and includes no
fuel flow, throttle law or propulsive interaction. Its sign follows +body x.
The result always states the applied force explicitly, including zero. T7 run
scenarios remain power-off; T8 does not add an engine or a CLI thrust command.

This is local steady straight-flight balance at the specified atmosphere. A
descending trim state changes altitude if integrated; constant density and
coefficients are not promised along a whole glide trajectory. The solver
searches the forward, nonvertical branch |alpha|, |gamma| < pi/2. It does not
search vertical/inverted branches or optimize control limits. Elevator limits
are unspecified, so no physical actuator bounds are invented. Roll/yaw moments
and lateral force must also meet the returned solution's tolerances; otherwise
the three-variable longitudinal solver reports failure.

## Solver and failure contract

`solve_trim` is pure core code. It uses central-difference Newton steps with
backtracking. Both forces are scaled by weight and pitching moment by q*S*chord
for the numerical linear solve; acceptance checks use the requested SI residual
tolerances directly, not only a normalized objective or a small step.

Default numerical settings (not physical parameters or D9 tolerances): initial
(alpha,elevator,gamma) = (0.05,0,0) rad; 50 Newton iterations; force tolerance
1e-8 N; moment tolerance 1e-8 Nm; difference step 1e-5 rad; at most 24 halvings
per line search. Options expose the initial guess, iteration limit, tolerances
and difference step. The derivative test also checks steps 1e-4 and 1e-6 rad.

**There is no best-effort success result.** Singular Jacobian, exhausted
iterations, failed residual reduction or unbalanced lateral loads returns
`TrimError::NonConvergence` with the reason, iteration count and last residuals,
but no purported trim solution. Invalid conditions/options and model-domain
errors are distinct errors. Speed must reach D13's angle threshold; finite
inputs, table bounds, geometric [0,20000] m and Mach <=0.8 remain mandatory.

## Static margin

```text
CL_alpha = (CL(alpha+h) - CL(alpha-h)) / (2h)
Cm_alpha = (Cm_CG(alpha+h) - Cm_CG(alpha-h)) / (2h)
static margin = -Cm_alpha / CL_alpha
Cm_CG = aerodynamic pitching moment about CG / (q*S*reference_chord)
```

The returned value is a **chord fraction**, e.g. 0.125 = 12.5% chord. The
derivatives are per radian. Each perturbed evaluation looks up the same
Mach/altitude table condition and evaluates its coefficient laws at the perturbed
alpha. These tables are indexed by Mach/altitude, not alpha: we perturb the
alpha-dependent coefficient evaluation, not the Mach index. Speed, geometric
altitude and elevator stay fixed; elevator is NOT retrimmed at each perturbation.
Reference-to-CG moment transfer is included. A stencil touching the CLmax cap,
nonpositive/unresolved lift slope or non-finite derivative returns an explicit
undefined-margin error instead of an infinite or misleading margin.

The formula/report ignores elevator-free hinge motion, control feedback,
rate-dependent and unsteady aerodynamics, structural flexibility, changing
mass/loading, thrust derivatives/propulsive interaction and atmospheric/Mach
changes during the perturbation. Effects not represented in the input tables
cannot be inferred. Near stall, this cap model cannot establish physical static
margin; a positive value does not prove dynamic stability or flight safety.

## Analytic synthetic cases and results

The existing synthetic fixture is cloned and all table cells are set to its
first coefficient cell for the analytic cases. Thus CL0=0.2, CLalpha=4/rad,
CLmax=1.2, CD0=0.02, e=0.8, Cm0=0.01, Cmalpha=-0.5/rad and Cmde=-1/rad;
S=2 m^2, span=4 m and chord=0.5 m. All these are TEST VALUES. At V=20 m/s and
h=1000 geometric m, choose alpha=0.1 rad, hence CL=0.6,
CD=0.02+0.6^2/(pi*0.8*8), and elevator=-0.04 rad. Let qS be evaluated from the
sourced ISA density, L=qS*CL and D=qS*CD.

- **Power off:** construct mass=sqrt(L^2+D^2)/g0. Exact gamma=-atan(D/L).
  The solver converged in 3 iterations to alpha=0.100000000000 rad,
  elevator=-0.040000000000 rad, gamma=-0.063091040800 rad. Full body force
  residual norm was 2.289190454985e-10 N; moment norm 1.542736750926e-15 Nm.
- **Level with temporary thrust:** construct T=D/cos(alpha) and
  mass=(L+T*sin(alpha))/g0. Exact gamma=0. Measured T=16.939570186044 N and
  gamma=9.739682727790e-13 rad. Full body force residual norm was
  2.629883488135e-10 N; moment norm was zero in this build. This checks the
  thrust-normal component, not merely T=D.
- **Stable margin:** finite differences give CLalpha=4/rad, Cmalpha=-0.5/rad,
  hence +0.125 chord. Steps 1e-4, 1e-5 and 1e-6 rad agree within 1e-9 chord.
- **Unstable configuration:** change the synthetic pitching slope to +0.5/rad;
  trim still solves, but the returned margin is -0.125 chord.

Further independent checks use a CG shift (-0.05,0,-0.02) m while the reference
point stays at zero: analytic derivatives of the force-transfer moment must
match the finite difference, preventing a report that uses the reference-point
Cm slope instead of CG Cm. Another case uses the original varying table at
Mach 0.4, h=10000 m; the margin must equal 0.5/5.5 chord from that interpolated
lift slope rather than the first node's 0.5/4.

All ten tests and their failure modes:

| Test | What it detects |
|---|---|
| Analytic power-off glide | Wrong alpha/elevator/gamma signs, force equation or zero-thrust default. |
| Level thrust | Missing T*sin(alpha), treating body thrust as wind-aligned, or failing to solve gamma=0. |
| Full force/moment balance | Misleading small solver residuals despite wrong state orientation, gravity frame or CG moment; all residual norms must be <1e-8 SI. |
| Stable finite differences | Wrong derivative sign/scale, perturbing elevator with alpha, or derivative-step sensitivity; expected +0.125 chord. |
| Unstable configuration | Clamping/reversing a negative margin or confusing trim with stability. |
| CG transfer | Missing or reversed reference-to-CG moment transfer in trim/margin. |
| Current table/atmosphere | Substituting sea-level coefficients or ignoring Mach/altitude interpolation. |
| Limit/unsolvable moment | Returning best effort when iterations are exhausted or elevator cannot cancel moment. |
| Impossible/lateral balance | Returning success below available lift or with unbalanced roll/yaw moments. |
| Invalid/undefined inputs | Non-finite/out-of-domain conditions, invalid tolerances, zero perturbation or capped lift producing a bogus margin. |

The tests-only run initially failed because `soas_core::trim` did not exist.
No test tolerance was weakened to obtain these results.
