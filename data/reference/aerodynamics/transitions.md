# Lift-limit and table-node convergence (T7, 2026-11-14)

This uses only a configuration of the existing synthetic aircraft fixture.
Every changed coefficient, state, altitude, duration and numerical bound below
is a synthetic TEST VALUE, not a real aircraft parameter. It is a test of the
implemented model and integrator, not physical validation.

The fixture is cloned. An altitude row is inserted at **999.8 geometric m**;
its cells copy the first row with CLalpha increased by 1/rad, creating a slope
change at that node. The altitude grid becomes [0,999.8,20000] m. Other aircraft
values remain from the synthetic file. Initial position is NED (0,0,-1000) m,
body velocity (20,0,6) m/s, identity attitude, zero body rates and zero controls.
Duration is 0.2 s. Atmosphere, tables and loads are evaluated at all RK4 stages.

The reference run at 1/12800 s demonstrably crosses both transitions:

- Table-node altitude crossing bracket: **[0.039609375, 0.0396875] s**.
- CLmax transition bracket: **[0.040234375, 0.0403125] s**.

These brackets come from consecutive reference samples, not exact event solves.
The test asserts an actual sign change across the altitude node and both capped
and uncapped lift states. It cannot pass merely by starting at an event.

| Step (s) | Position difference (m) | Body velocity difference (m/s) | Attitude difference (rad) | Body rate difference (rad/s) | Combined scaled norm |
|---:|---:|---:|---:|---:|---:|
| 0.01000 | 2.030041863720e-5 | 3.793764428217e-5 | 1.200781093058e-6 | 1.092533341787e-5 | 4.440919539972e-5 |
| 0.00500 | 9.197469130837e-6 | 1.631069123065e-5 | 5.371853355890e-7 | 4.948962242651e-6 | 1.937557437079e-5 |
| 0.00250 (D10) | 3.566159825141e-6 | 6.277415460996e-6 | 2.079968248123e-7 | 1.918754127500e-6 | 7.473173412235e-6 |
| 0.00125 | 7.456874123453e-7 | 1.307346751548e-6 | 4.355505274500e-8 | 4.011483428212e-7 | 1.558211244113e-6 |
| 0.000625 | 6.648466282340e-7 | 1.174857215800e-6 | 3.865670818171e-8 | 3.578080466360e-7 | 1.397079616885e-6 |
| 0.0003125 | 1.089604029229e-8 | 1.925295724858e-8 | 6.335126451207e-10 | 5.863313390364e-9 | 2.289497410026e-8 |
| 0.00015625 | 3.634233652394e-9 | 6.417325084898e-9 | 2.111748692579e-10 | 1.954511746893e-9 | 7.632458744049e-9 |

Differences are against the same model at 1/12800 s, not an analytic solution.
The combined norm divides the four norms by explicit synthetic scales of 1 m,
1 m/s, 1 rad and 1 rad/s, then combines them quadratically. It is dimensionless,
not a new engineering validation metric. The test requires a material overall
decrease (>4x from first to fifth row) and default combined difference <1e-2.

Observed successive orders, log2(error(h)/error(h/2)), are **1.196619, 1.374446,
2.261830, 0.157477, 5.931239, 1.584811**. There is no stable fourth-order regime
in these measurements. The exceptionally high and low individual values are
consistent with event phase relative to the step grid; that is an interpretation,
not an established isolated cause. Both the lift cap and interpolation-node
slopes make the right-hand side nonsmooth, so the smooth-case RK4 result from
T6 cannot simply be carried over.

A second reference at 1/25600 s changes the combined endpoint by
**3.812967747422e-9**, about **0.051%** of the default-step estimate (test bound:
less than 10%). This checks reference resolution at the default, not exactness
of the finest rows.

**Implication for D10:** retain the owner-decided 0.0025 s default, with this
explicit limitation: fourth-order accuracy is not established across these
transitions, and halving does not guarantee a 16x improvement. The measured
default differences are small for this short synthetic run only. New aircraft,
long runs and transition-sensitive scenarios need their own refinement checks;
no universal accuracy bound or new default decision is inferred.

The model, table and RK4 references remain those in the adjacent T6 README and
`data/reference/dynamics/README.md`. No external aircraft data are used.
