# T9 synthetic turbojet/afterburner checks

## Provenance and sources

The owner explicitly authorized **SYNTHETIC engine data** for T9. No real engine
numbers, tables, operating envelope or validation claim are supplied. Every
numeric JSON path is labeled TEST VALUE in these fixture ledgers:

- `crates/soas-core/tests/fixtures/synthetic_jet_values.md`
- `crates/soas-cli/tests/fixtures/synthetic_powered_scenario_values.md`

The original aircraft fixture remains a supported version-0 power-off file.
The new jet fixture copies its synthetic airframe and adds version-1 engines.
All modified engine/geometry/query values in Rust tests are explicitly labeled
synthetic too. Test tolerances, normalized control endpoints and numerical
solver settings are software contracts, not sourced real-engine measurements.

References verified online 2026-11-14:

- [MIT 8.01SC, Calculating Torque transcript, page 1](https://ocw.mit.edu/courses/8-01sc-classical-mechanics-fall-2016/cf03c05682d1451bc928a4dba047b705_1BU28txGAFI.pdf):
  the position vector runs from the reference point to the force application
  point; torque is its vector cross product with force. Here that reference
  point is the CG, in the project's right-handed body axes.
- [JSBSim reference manual, Math / Tables](https://jsbsim-team.github.io/jsbsim-reference-manual/user/concepts/math/#tables):
  documents one-, two- and three-dimensional lookup tables in flight dynamics.
  This is context for tabular modeling, **not** a source of engine values or a
  claim that this implementation duplicates JSBSim's engine/interpolation/
  out-of-range policies. T9 explicitly uses tensor-product linear interpolation
  and rejects extrapolation as documented in `docs/AIRCRAFT_FORMAT.md`.
- Existing trim force-projection and aerodynamic static-margin sources remain
  in `data/reference/trim/README.md`. The aerodynamic margin continues to
  exclude engine derivatives, even when evaluated at a powered trim condition.

Real engines still need cited net-thrust and fuel-flow tables versus Mach,
geometric altitude and throttle, dry/afterburner calibration, applicable
conditions, units, source axes and installation locations/directions. No
missing real data were filled with plausible estimates.

## Engine/table contract and limits

Aircraft version 1 requires an engines array, with named engine/table/sample
objects; version 0 forbids it and remains engine-less. Scenario version 1
requires named throttle knots; version 0 has none and cannot run an installed
engine. Unknown versions and misspelled/duplicate/missing fields error.

Common throttle is dimensionless [0,1]; each engine declares an interior
`dry_throttle_max` node. The boundary itself is dry; commands above it are
afterburning. The fixture uses SYNTHETIC 0.6, not a real throttle calibration.
There is no additional afterburner multiplier. Zero command returns the first
table entry, including any idle thrust/fuel, rather than switching off.
Explicit `PowerOffGlide` trim mode bypasses all engines.

Samples are `[geometric altitude][Mach][throttle]`; both thrust N and fuel kg/s
are interpolated in all three dimensions. Exact nodes return exact samples;
out-of-range/non-finite queries error; singleton Mach/altitude permits only its
node. The flight envelope remains geometric ISA [0,20000] m and Mach <=0.8.
Position uses the same datum as CG; direction is a unit vector along aircraft
force, not exhaust. Sum force and `(position-CG) cross force` in declaration
order with aerodynamic loads at each RK4 stage. No gravity double-counting.

Fuel flow is **reported only**: mass, CG and inertia never change. No spool
dynamics, engine-off scenario switch, starts/flameouts, burner ignition delay,
hysteresis, thermal limits, gyroscopic moment or aerodynamic interaction is
modeled. The supplied thrust is already net thrust; no separate ram-drag or
intake/nozzle model is added. These limitations preclude real-engine claims.

## Independent hand calculations and wrong alternatives

All numbers in this section are SYNTHETIC TEST VALUES or mathematical results
from them. Fixture-generating expressions (m=Mach, h=geometric m, t=throttle):

```text
thrust N = 100t + 20m + .01h + 50mt + .02ht + .001hm + .003hmt
fuel kg/s = .01 + .2t + .03m + .00001h + .04mt + .00002ht
            + .000003hm + .000004hmt
```

The tests query m=.12, h=1400, t=.24, strictly inside the first 3D cell. The
local fractions are .4, .7, .4, and the hand-calculated values are:

```text
T = 24 + 2.4 + 14 + 1.44 + 6.72 + .168 + .12096 = 48.84896 N
f = .01 + .048 + .0036 + .014 + .001152 + .00672
    + .000504 + .00016128 = .08413728 kg/s
```

With the mixed terms omitted, the anchor-plane results would be 40.4 N and
.0756 kg/s: discrepancies 8.44896 N and .00853728 kg/s, far above the 1e-11
absolute check. Unequal physical nodes and slopes distinguish the axes: at
(m,h,t)=(.3,0,0), (0,2000,0), (0,0,.6), thrust must be 6,20,60 N respectively.
Swapping Mach/throttle cell indices would interchange 6 and 60 N. The test
also visits every exact node and an explicitly singleton altitude/Mach table.

For CG=(1,-2,3), application point=(3,1,-1), r=(2,3,-4), direction=
(2/3,-1/3,2/3), T=90 N gives F=(60,-30,60) N and:

```text
r x F = (3*60 - (-4)*(-30), (-4)*60 - 2*60, 2*(-30) - 3*60)
      = (60, -360, -240) Nm
```

Moving the application point to the CG gives exactly zero moment even with
this inclined direction. With the offset retained and direction=(1,0,0),
moment=(0,-360,-270) Nm. Thus omitted moment, reversed cross product,
origin-relative arm and swapped force axes cannot satisfy these checks.

## Powered trim and runner evidence

Level mode solves alpha/elevator/throttle with gamma=0. PowerOffGlide keeps
the existing alpha/elevator/gamma solve, engines off. No temporary
`applied_thrust_n` input remains. Engine direction and offset affect both
force projection equations and CG pitch moment. Throttle trials stay in [0,1],
with one-sided derivative stencils at the bounds; a failed solve returns an
error rather than a best-effort condition. Existing force/moment tolerances
remain 1e-8 N/Nm. The old supplied-force echo check at 1e-14 now checks the
returned engine output against evaluation at the solved command at 1e-14.

Independent analytic targets: V=20 m/s, geometric h=1000 m, alpha=.1 rad;
the original fixture's first aero cell is held constant, T=100t N, f=.2t kg/s.
With qS from the sourced ISA, CL=.6, CD=.02+.36/(pi*.8*8), L=qS*CL, D=qS*CD:

- CG body-x engine: T=D/cos(alpha), W=L+T*sin(alpha), elevator=-.04 rad.
- Inclined engine: direction=(.8,0,-.6), r=(2,0,.3),
  T=D/(.8*cos(alpha)-.6*sin(alpha));
  W=L+T*(.8*sin(alpha)+.6*cos(alpha));
  M_engine=1.44*T, elevator=-.04+1.44*T/(qS*.5).

Measured in this build (software checks, not aircraft validation):

| Engine | Solved throttle | Elevator rad | Full force norm N | Moment norm Nm |
|---|---:|---:|---:|---:|
| Through CG | .169395701860 | -.040000000000 | 1.514990622339e-10 | 0 |
| Inclined/offset | .228975244370 | .108302832220 | 1.642654366435e-11 | 0 |

Alpha solves to .100000000000 rad and gamma is fixed at exactly zero. Both
cases compare throttle/elevator/alpha with analytic targets within 1e-10.
Insufficient thrust, no engines and unbalanced lateral moments explicitly
fail. Initial throttle 0, .6 and 1 converges to the same target; invalid or
non-finite throttle guesses error.

The runner test compares its first step with an independent stage-time
callback using throttle=.2+5*time. It also verifies that a frozen start-of-step
command produces a different result, checks byte-identical repeated CSV,
afterburner reporting, positive fuel flow and unchanged aircraft mass.
CSV appends throttle, summed thrust, fuel flow and afterburner-engine count.
No default step change or real-engine integration-accuracy claim is made.

## Tests first: observed failure before implementation

New regression tests and failure mechanisms (all numerical data synthetic):

| Test | Check / plausible fault detected |
|---|---|
| `version_one_synthetic_engine_file_loads` | Engine fixture accepted; missing version-1 schema support fails. |
| `version_one_synthetic_throttle_scenario_loads` | Throttle fixture accepted; missing scenario extension fails. |
| `version_one_round_trips_engines_and_version_zero_remains_explicitly_power_off` | Values survive saving; wrong version handling, implicit migration or omitted empty engines fails. |
| `trilinear_off_centre_mixed_terms_are_hand_calculated_for_both_outputs` | Independent thrust/fuel values and unequal-axis node values; planar interpolation or swapped axes fails. |
| `exact_nodes_singletons_and_no_extrapolation_are_explicit` | Every sample, singleton Mach/altitude and query bounds; wrong node or extrapolation fails. |
| `cg_thrust_has_zero_moment_and_offset_inclined_thrust_has_signed_hand_moments` | Hand body forces and all three moments; wrong datum, exhaust direction, sign, cross-product order or ignored offset fails. |
| `afterburner_boundary_sums_and_no_engine_outputs_are_defined` | Inclusive dry/exclusive burner boundary, multiple-engine sums, empty-engine totals and NaN command; wrong boundary or aggregation fails. |
| `engine_records_tables_and_every_sample_reject_positional_arrays_with_paths` | Engine/table and all 18 sample records reject arrays; unchecked Serde sequence acceptance fails. |
| `invalid_engine_values_shapes_unknown_duplicate_and_missing_fields_remain_errors` | Geometry, direction, nodes, boundary, shapes, signed/non-finite outputs and parser diagnostics; permissive validation fails. |
| `coupled_loads_add_engine_force_moment_and_preserve_constant_mass` | Actual combined force/moment and unmodified aircraft; omitted propulsion or mutation fails. |
| `level_trim_solves_throttle_and_all_loads_for_cg_and_inclined_offset_engines` | Two analytic conditions and full-vector balance; omitted thrust-normal component or engine pitch moment fails. |
| `insufficient_thrust_no_engines_and_lateral_imbalance_never_return_best_effort` | Unsolvable cases error; unconstrained throttle or best-effort success fails. |
| `throttle_boundaries_are_valid_initial_guesses_and_nonfinite_guesses_are_errors` | Valid initial 0/boundary/1 commands converge, invalid guesses error; out-of-range stencils or silent clamping fails. |
| `throttle_is_interpolated_at_rk4_stages_and_csv_reports_thrust_and_fuel_deterministically` | Hand time law, independent RK4 step, frozen-command contrast, CSV repetition/columns and fuel/mass reporting; stage-time or output omissions fail. |
| `throttle_knots_are_named_objects_and_versions_ranges_and_coverage_are_checked` | Every knot object, schedule versions/ranges/coverage and legacy-engine mismatch; positional knots, implicit schedule or permissive limits fail. |

The tests-only commit contains only test sources/fixtures/ledgers (including
porting the existing trim tests to the required new engine interface).
Before implementation, the two independently runnable boundary tests failed:

```text
test version_one_synthetic_engine_file_loads ... FAILED
version 1 engine file rejected: aircraft JSON: unknown field `engines`
at line 106 column 11
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

test version_one_synthetic_throttle_scenario_loads ... FAILED
version 1 throttle scenario rejected: scenario JSON: unknown field `throttle`
at line 65 column 12
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

The later targeted implementation run passes all 15 new tests and all ten
ported trim tests. No new external dependencies, OPEN-decision choices, pushes
or remote changes are part of T9. The known filesystem hardlink limitation
remains explicitly handled with `CARGO_INCREMENTAL=0` for local checks.

Final workspace gates: fmt check passed; Clippy with `-D warnings` passed;
all 116 tests passed, 0 failed/ignored. Two actual powered CLI runs produced
byte-identical 2761-byte CSV files, each with five samples. A numeric-path audit
found ledger entries for all 119 jet-fixture numbers and all 35 powered-scenario
numbers. These checks do not validate an engine or establish a new time-step
accuracy bound; D10 stays unchanged.

## Files changed in T9

Repository-relative paths, grouped for review:

```text
crates/soas-core/src/{aircraft,lib,propulsion,trim}.rs
crates/soas-cli/src/lib.rs
crates/soas-core/tests/{engine_format,powered_trim,propulsion,trim}.rs
crates/soas-core/tests/fixtures/synthetic_jet.json
crates/soas-core/tests/fixtures/synthetic_jet_values.md
crates/soas-cli/tests/{powered,throttle_format}.rs
crates/soas-cli/tests/fixtures/synthetic_powered_scenario.json
crates/soas-cli/tests/fixtures/synthetic_powered_scenario_values.md
docs/{AIRCRAFT_FORMAT,ASSUMPTIONS,LEARNING,SCENARIOS,TASKS}.md
data/reference/{propulsion,trim}/README.md
```
