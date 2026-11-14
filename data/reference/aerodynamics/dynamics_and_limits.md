# T9b dynamic derivatives and limits: synthetic checks

## Sources and frame definitions

[Caughey, Introduction to Aircraft Stability and Control (Cornell, 2011)](https://courses.cit.cornell.edu/mae5070/Caughey_2011_04.pdf):
section 1.2 / Fig. 1.1, printed pp. 3-5, and section 4.1 / Fig. 4.1, printed
pp. 45-46, define right-handed body axes, velocity/rate and moment signs.
Equations 4.37-4.38, printed p. 54 (PDF p. 60), define pitch rate normalized by
chord and twice reference speed. Equations 4.77-4.78, printed p. 60 (PDF p. 66),
give the corresponding span-based roll/yaw definitions and dimensional roll
damping. These passages and the mathematical PDF pages were checked online
2026-11-14. No aircraft numbers from the notes were copied.

The user explicitly requires instantaneous V, replacing the source's reference
speed u0 in this model. Positive p/q/r and L/M/N follow the project's body axes:
right-wing-down roll, nose-up pitch, nose-right yaw. With positive q*S and
positive reference lengths/speed, a negative direct derivative produces a
moment opposing its rate. Signs are retained as data, not constrained stable.
Force coefficients still use the project's WIND axes, moments BODY axes about
the aerodynamic point before transfer to CG. Source/stability-axis derivatives
must be converted before import; this task supplies no real-aircraft data.

The rate threshold, two-sided cap and saturation policies are the owner's
T9b requirements, not textbook physical-validity claims. The existing T3
atmosphere/gravity sources are unchanged. Numerical test settings and fixture
data are synthetic; no real-aircraft calibration is implied.

## Supported derivatives before and after T9b

Before: CL_alpha, Cm_alpha, Cm_elevator, Cy_beta, Cl_aileron and Cn_rudder;
drag depends on lift through the existing polar, rather than a separate CD_alpha
table entry. No sideslip roll/yaw or angular-rate derivatives were implemented.

After: retain those terms and add Cl_beta, Cn_beta, Cm_q, Cl_p, Cn_r, plus the
optional cross-rate Cl_r and Cn_p. All seven new derivatives and cl_min use the
same exact-node / linear-Mach / bilinear-Mach-altitude lookup and range errors.
Table names use `roll_*` / `yaw_*` to distinguish roll Cl from lift CL.

```text
phat=p*b/(2V), qhat=q*c/(2V), rhat=r*b/(2V)
Cl=Cl_beta*beta + Cl_p*phat + Cl_r*rhat + Cl_da*applied_aileron
Cm=Cm0 + Cm_alpha*alpha + Cm_q*qhat + Cm_de*applied_elevator
Cn=Cn_beta*beta + Cn_r*rhat + Cn_p*phat + Cn_dr*applied_rudder
CY=Cy_beta*beta
L=q_dyn*S*b*Cl; M=q_dyn*S*c*Cm; N=q_dyn*S*b*Cn
```

Below MIN_AIRSPEED_FOR_ANGLES_MPS, all normalized rate terms are zero, after
finite-input checks; at the threshold they are active. Version 2 applies the
negative cl_min and positive cl_max caps, flagging either boundary inclusive.
This is not a separated-flow/post-stall model. Each commanded deflection
saturates independently to its file min_rad/max_rad. Both angles are reported.
No actuator rate limits, lag, hysteresis, alpha-dot/unsteady terms or additional
rate-force derivatives (CY_p/CY_r, CL_q, etc.) are included.

Format 2 requires engines (possibly empty), control_limits and every new
direct/beta/lower-cap entry. Cross-rate entries may be absent, meaning omitted
terms. Versions 0/1 preserve prior behavior and reject the new fields; saving
them does not invent derivative/limit data. Scenario versions remain 0/1.
The commanded CSV columns now have explicit commanded_ prefixes, followed by
applied_ columns; header-based consumers should be updated. The core remains
pure, with file access in the CLI and no added dependencies.

## Ledgers and independent magnitudes

Every new number is either inherited sourced atmosphere/gravity, a structural
mathematical definition, or an explicitly labeled synthetic test value/numerical
setting. Ledgers:

- `crates/soas-core/tests/fixtures/synthetic_dynamic_aircraft_values.md`: every
  numeric JSON path, including derivatives, caps, geometry and travel bounds.
- `crates/soas-core/tests/fixtures/synthetic_dynamic_test_values.md`: new Rust
  test literals/derived expectations, tolerances, version/index metadata.
- `crates/soas-cli/tests/fixtures/synthetic_limited_scenario_values.md`: every
  numeric scenario path. These examples model no real aircraft.

The independent existing T3 sea-level density is 1.224999155887712 kg/m^3.
Synthetic V=20 m/s, S=2 m^2, b=6 m, c=.5 m give qS=489.9996623550848 N,
qSb=2939.9979741305088 Nm and qSc=244.9998311775424 Nm. Each coefficient is
isolated in its own test; the reference point and CG coincide.

| Term | Synthetic input | Hand expected output |
|---|---|---|
| Cy_beta | -.2/rad, beta=.1 rad | Side force -9.799993247101697 N |
| Cl_beta | -.12/rad, beta=.1 rad | Roll -35.27997568956611 Nm |
| Cn_beta | .18/rad, beta=.1 rad | Yaw 52.91996353434915 Nm |
| Cm_q | -3, q=.6 rad/s, qhat=.0075 | Pitch -5.512496201494704 Nm |
| Cl_p | -.4, p=.8 rad/s, phat=.12 | Roll -141.11990275826442 Nm |
| Cn_r | -.7, r=-.4 rad/s, rhat=-.06 | Yaw 123.47991491348137 Nm |
| Cl_r | .09, r=-.4 rad/s, rhat=-.06 | Roll -15.875989060304748 Nm |
| Cn_p | -.08, p=.8 rad/s, phat=.12 | Yaw -28.223980551652884 Nm |

Distinct span/chord expose length swaps, and independent rates expose missing
normalization or factors of two. Moment/side checks use absolute 1e-11 SI.
Each negative direct derivative is also tested with both signs of rate.
The low-speed check covers zero, below and at the existing threshold, and NaN.

Off-centre interpolation uses Mach=.2, h=12000, corner multipliers [1,2,3,7]
and weights [.30,.10,.45,.15], giving 2.9 times each base derivative. Lower-cap
corners [-1.1,-1.3,-1.6,-2.4] give -1.54. The mixed difference is nonzero,
so a planar approximation cannot pass. Control tests cover both asymmetric
travel ends, exact bounds and neutral; commands beyond travel must have the
same loads as commands at the boundary. Lift tests cover just inside, exactly
on, and just outside BOTH caps. The CSV test checks all three commanded/applied
angles against the degree commands and radian bounds independently.

## Analytic roll subsidence

With pure roll and diagonal inertia, the inertial cross-coupling term vanishes:

```text
Ixx * dp/dt = q_dyn*S*b * (b/(2V)) * Cl_p * p = K*p
tau = -Ixx/K
p(t) = p0*exp(-t/tau)
```

This follows directly from the sourced normalization and rigid-body moment
equation; the expected curve does not call the model's force/moment function.
Synthetic Ixx=8 kg*m^2 and Cl_p=-.4 with the above fixed conditions give
K=-176.39987844783052 Nm/(rad/s), tau=0.045351505173 s. Starting at p0=.8
rad/s, eighty 0.0025 s steps give at .2 s:

```text
analytic p = .009724172214 rad/s
computed p = .009724175669 rad/s
absolute error = 3.455146092846e-9 rad/s (test bound 5e-9)
```

The callback fixes aero airspeed/altitude, applies only the evaluated roll
moment and cancels gravity for rotational isolation. Pitch/yaw rates stay zero.
This tests damping and integration, not a free-flight stability mode.

## Full aero-coupled damped convergence

Separate case: use the new varying-coefficient fixture, geometric altitude
1000 m, V=20 m/s, beta=.05 rad, body rates (.4,-.2,.3) rad/s, identity attitude,
neutral controls, duration .25 s. Every RK4 stage reevaluates full aero loads,
including damping, body motion, atmosphere, tables, gravity and CG transfer.
No speed constraint is applied here. References: 1/12800 and 1/25600 s.

| Step s | Position error m | Velocity error m/s | Attitude error rad | Rate error rad/s | Combined norm |
|---:|---:|---:|---:|---:|---:|
| .01 | 8.337492738067e-8 | 2.680808526696e-7 | 2.614093239499e-8 | 2.832212499739e-7 | 3.996452762977e-7 |
| .005 | 4.860863729059e-9 | 1.551366366333e-8 | 1.509004417585e-9 | 1.604777815956e-8 | 2.289344960119e-8 |
| .0025 | 2.879740396480e-10 | 9.332338011422e-10 | 9.060332792833e-11 | 9.551249943978e-10 | 1.369060660840e-9 |
| .00125 | 1.371105601856e-11 | 5.720187284539e-11 | 5.549705561500e-12 | 5.825396005905e-11 | 8.297210620771e-11 |

Combined norm uses synthetic unit scales 1 m, 1 m/s, 1 rad and 1 rad/s,
not a physical mixed-unit quantity. Ratios are 17.456752,16.722013,16.500252;
observed orders 4.125713,4.063677,4.044416. The second fine reference stays
below 5% of the default-step combined estimate. These support keeping D10
unchanged for this smooth synthetic case. No claim covers stall/saturation,
rate-threshold or interpolation-node transitions, or arbitrary aircraft.

## Tests first, gates and review notes

Before implementation:

```text
version 2 rejected: aircraft JSON: unknown field `cl_min`
test version_two_loads_and_round_trips_explicit_dynamic_data_and_limits ... FAILED
test result: FAILED. 0 passed; 1 failed
```

Derivative tests also initially failed to compile because the fields did not
exist. The implementation run passes 15 dynamic tests, the file round-trip,
two validation tests and the CLI saturation CSV test. The generic object walk
also automatically includes control_limits and all three bound objects.
Existing versions' tests and tolerances remain. Clippy requested two shorter
literal spellings for hand expectations; their f64 bit patterns were checked
identical and recorded in the ledger. No numerical tolerance was weakened.

New tests cover every hand magnitude above, damping signs, threshold errors,
mixed interpolation, two-sided lift boundaries, per-surface saturation,
analytic decay, coupled convergence, strict version/required-field/non-finite
validation, optional cross-rate omission, and commanded/applied CSV reporting.
This is software/analytic evidence, not real-aircraft validation.

Final gates: formatting passed; Clippy with warnings denied passed; workspace
total 137 tests passed, 0 failed/ignored. Checks use the previously disclosed
`CARGO_INCREMENTAL=0` setting for this filesystem. The generic discovery tests
now mutate all 76 objects across six fixtures (42 aircraft, 34 scenario),
including engines, throttle and the new limits, without path lists. Numeric-path
audit: 106 dynamic-aircraft and 31 limited-scenario numbers have ledger entries;
the Rust ledger lists 211 numeric literal occurrences including metadata.
An actual CLI run confirms elevator/aileron/rudder commands
(.5235987755982988, .6981317007977318, -.8726646259971648) rad apply as
(.18, .16, -.3) rad. These are results of the ledgered synthetic degree commands
and bounds, not additional aircraft inputs.

## Files changed for T9b

```text
docs/TASKS.md (initial separate docs-only task entry, then completed status)
crates/soas-core/src/{aircraft,aerodynamics}.rs
crates/soas-cli/src/lib.rs
crates/soas-core/tests/{dynamic_file,dynamic_format,dynamic_derivatives}.rs
crates/soas-cli/tests/control_limits.rs
crates/soas-core/tests/fixtures/synthetic_dynamic_aircraft.json
crates/soas-core/tests/fixtures/synthetic_dynamic_aircraft_values.md
crates/soas-core/tests/fixtures/synthetic_dynamic_test_values.md
crates/soas-cli/tests/fixtures/synthetic_limited_scenario.json
crates/soas-cli/tests/fixtures/synthetic_limited_scenario_values.md
docs/{AIRCRAFT_FORMAT,SCENARIOS,ASSUMPTIONS,LEARNING}.md
data/reference/aerodynamics/dynamics_and_limits.md
```
