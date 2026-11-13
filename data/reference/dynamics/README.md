# Rigid-body dynamics references and analytic checks (T4)

## Sources and verification

Verified online on 2026-11-13:

- David A. Caughey, *Introduction to Aircraft Stability and Control*, Cornell
  (2011), [section 4.1, printed pages 45-50](https://courses.cit.cornell.edu/mae5070/Caughey_2011_04.pdf#page=51).
  Equation (4.12) gives the body-velocity equations with gravity in NED.
  The implementation uses quaternions for the gravity rotation, rather than
  integrating the source's Euler-angle representation. The source specializes
  rotational components using aircraft symmetry; our full tensor is retained.
- MIT OpenCourseWare, 8.09, *Classical Mechanics III* (Fall 2014),
  [chapter 2](https://ocw.mit.edu/courses/8-09-classical-mechanics-iii-fall-2014/6fe39e8d5ce4ce746ca256dfea665eda_MIT8_09F14_Chapter_2.pdf),
  equations (2.42)-(2.43) define signed tensor entries and angular momentum;
  (2.66)-(2.68) give rotating-frame angular momentum balance. Combining
  constant body-frame inertia with that vector balance gives the tensor
  equation below without assuming zero off-diagonals.
- Joan Sola, *Quaternion kinematics for the error-state Kalman filter* (2017),
  [equation (200), section 4.5, printed page 46](https://arxiv.org/pdf/1711.02508#page=46):
  Hamilton quaternion derivative for local/body angular rates, multiplying on
  the right. The finite-rotation solution for constant rates supplies an
  independent analytic orientation in the convergence test.
- [NIST DLMF, equations (3.7.18)-(3.7.19)](https://dlmf.nist.gov/3.7#E18):
  classical four-stage RK4 weights and intermediate states. For smooth solutions,
  local error is order h^5; over a fixed duration the expected global error is
  order h^4, giving an asymptotic factor of 16 when the step is halved.

Gravity is the project's constant `g0 = 9.80665 m/s^2`, sourced in
`../atmosphere/README.md` and required by CONVENTIONS.md. The default step
`1/400 s` is owner decision D10, not a published aircraft parameter.

## Implemented equations

R maps body vectors to NED; v and omega are body-frame velocity and angular
rate; I is the constant signed inertia tensor about the CG. F excludes gravity;
M is the total applied moment about the CG. Quaternion multiplication is Hamilton.

```text
position_dot_ned = R v
v_dot_body      = F/m + R^T (0, 0, g0) - omega cross v
omega_dot_body  = I^-1 (M - omega cross (I omega))
quaternion_dot  = 0.5 quaternion * (0, omega)
```

RK4 advances all these variables together. Raw quaternion stage components
are used in its differential equation; normalized copies are used for spatial
rotations and stage load evaluation. The completed quaternion is normalized
each step. A pure load callback is reevaluated at all four stage states.
There is no wall-clock time, hidden integration state or file I/O.

`State` is a value record with explicit units and frames, no derived caches or
resources. Its quaternion has scalar w and vector i,j,k components. Actual file
serialization and its wire format are deferred; T4 adds no serde dependency.
Mass/inertia and the fixed time step live outside State. The caller tracks time
from the step count; the present load callback depends on state, without an
explicit time argument. Loads or controls held constant during a step can be
captured by that callback.

Mass must be finite and positive; signed inertia must be finite, exactly
symmetric and positive definite, with a finite computed inverse. These are
numerical prerequisites, not a complete physical-realizability/data-provenance
audit. Products-of-inertia input (Pxy,Pxz,Pyz) means integrals without leading
minus signs; all three are negated when building the symmetric tensor (D11).
No silent symmetrization is performed. Non-finite states/loads or intermediate
overflow are errors; input is borrowed, so an error cannot partially advance it.

## Tests and their failure modes

Every mass, inertia, load, initial state, duration, rate and numerical acceptance
bound below is a **synthetic mathematical test choice**, not a real aircraft
measurement, a fitted parameter, or D9's future aircraft-validation tolerance.

| Test | Independent expectation | What would make it fail |
|---|---|---|
| Inertia construction | Three pairs of 1 kg points at +/- (1,2,0), +/- (1,0,-3), +/- (0,2,3) m have CG zero. Diagonals (52,40,20) kg m^2; products (4,-6,12) kg m^2; tensor off-diagonals (-4,+6,-12). | Wrong negation, product order, symmetry or changed diagonal. |
| Invalid mass/inertia/step | Reject zero/negative/non-finite mass, singular/negative/asymmetric/non-finite tensor, and zero/negative/non-finite step; default exactly 1/400 s. | Accepting an invalid input or changing the default. |
| Tilted free fall | After 2 s, position = initial position + initial NED velocity*t + (0,0,g0*t^2/2); velocity = initial NED velocity + (0,0,g0*t). | Wrong gravity sign/frame, extra gravity, position integration or body/NED transform; vector error limit 1e-10. |
| Torque-free symmetric top | I=(2,2,3) kg m^2, initial omega=(1,0,2) rad/s: p=cos(t), q=sin(t), r=2. World angular momentum is constant over 10 s. | Wrong gyroscopic sign or missing term, quaternion rotation direction; rate error <1e-10, world momentum vector error <=1e-8, magnitude relative drift <1e-10. |
| Full tensor and moments | Moment = I*(chosen unit angular acceleration) + omega cross (I omega), with the full mixed-sign tensor above. After 1 s omega equals the chosen acceleration for each axis. | Wrong inverse, moment axis, off-diagonal use or gyroscopic coupling; rate error <=1e-12. |
| Each body force axis | A 2 N axis force on 2 kg gives unit acceleration along that body axis, rotated to NED, plus gravity. After 1 s, v=a and position=a/2. | Wrong mass division, axis, rotation or gravity; vector error <=1e-11. |
| Long-run quaternion norm | 100000 default steps (250 s) with omega=(0.7,-1.1,0.9) rad/s on spherical inertia (2,2,2) kg m^2; norm error <=5e-15 at every step. | Missing/broken normalization or non-finite integration. |
| Step-size convergence | Spinning free fall described below; compare analytic position and orientation at the same final time. | Lower integration order, incorrect stage coupling, wrong quaternion product, wrong rotating-frame velocity sign, default error exceeding bounds. |
| Explicit errors | Non-finite position/velocity/rates, invalid quaternion, non-finite force/moment, stage-two invalid load and finite-input arithmetic overflow are errors. | Invalid outputs escaping as usable state or stage loads not being checked. |

The tests were written before the module; their initial run failed because
`soas_core::dynamics` did not exist. No test expectation was weakened during
implementation.

## Step-size convergence and D10

A spherical synthetic body (mass 2 kg; I=(2,2,2) kg m^2) starts at the origin,
identity orientation, body velocity (10,0,0) m/s, body rates (0,0,8) rad/s.
There are no applied loads, only gravity. At t=1 s, exact NED position is
(10,0,g0/2) m; exact orientation is an 8 rad rotation about +z. The body velocity
must rotate while the world trajectory remains analytic free fall, exercising
coupled translation and attitude rather than only a scalar RK4 example.

Measured on this machine/build (2026-11-13):

| Step (s) | Position error norm (m) | Orientation angle error (rad) |
|---:|---:|---:|
| 0.02000 | 2.273434468942e-4 | 2.724427735524e-6 |
| 0.01000 | 1.426002981618e-5 | 1.705691513587e-7 |
| 0.00500 | 8.928390443426e-7 | 1.066514376813e-8 |
| 0.00250 (D10) | 5.585205047472e-8 | 6.666444063441e-10 |
| 0.00125 | 3.492305521814e-9 | 4.166622602497e-11 |

Position error ratios under halving: 15.942705, 15.971557, 15.985788, 15.992888.
Orientation ratios: 15.972570, 15.993141, 15.998250, 15.999635.
Each must lie between 14 and 18, a numerical check for fourth-order behaviour.
At the default, error bounds for this case are <1e-6 m and <1e-9 rad.
The evidence supports **retaining D10's 0.0025 s default for this smooth test**;
no new time-step decision is needed. It does not establish sufficient resolution
for future aerodynamic/propulsion dynamics, discontinuities, arbitrary rates or
stiff load models; those require their own convergence checks.

The symmetric-top test's maximum relative angular-momentum magnitude drift was
9.830333712e-16. Maximum quaternion norm error over 100000 steps was
2.220446049e-16 against the stated 5e-15 limit. RK4 does not promise exact
long-term momentum/energy conservation; these are finite-run measurements.
