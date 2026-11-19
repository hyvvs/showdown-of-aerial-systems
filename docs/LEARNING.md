# Learning plan and log

You are building this while learning. The aim: be able to review what the agent writes.

## Weeks 1 to 2: foundations
- Rust: *The Rust Programming Language* (the free "Rust Book"), chapters 1 to 10; then work through Rustlings exercises.
- Flight basics: NASA Glenn "Beginner's Guide to Aeronautics" (search for it by name): forces on an aircraft, lift, drag, stability, control surfaces.

## Weeks 3 to 4: flight dynamics
- Beard & McLain, *Small Unmanned Aircraft: Theory and Practice*: chapters on coordinate frames, kinematics and dynamics, and forces and moments. I believe it is freely available from the authors; verify. Suitable for UAV and odd-configuration thinking.
- As needed: a standard atmosphere explainer for T3.

## Self-check questions (answer in your own words, in the log below)
1. Why does lift depend on the square of airspeed?
2. What does a negative `Cm_alpha` mean physically, and why does it make an aircraft stable?
3. What is the neutral point, and what is static margin?
4. Why must the CG be ahead of the neutral point?
5. What does "trim" mean?
6. Why use a quaternion for attitude instead of Euler angles?
7. Why is thrust not just `power / speed`?
8. What does the stall speed formula assume?

## Log
After each task, the agent adds the date, task and **Concepts introduced** only, leaving **Understood** and **Still unclear** blank. The owner fills in those two fields in their own words, including any questions for the agent.

| Date | Task | Concepts introduced | Understood | Still unclear |
|---|---|---|---|---|
| 2026-11-12 | T0 | Cargo workspace: a group of Rust packages; library crate: reusable code without an executable entry point; package-identity test: checks the manifest's package name. |  |  |
| 2026-11-13 | Documentation corrections and owner decisions | Task dependencies: what must be ready before a task starts; hand-constructed force checks versus trim (a solved steady-flight balance); angle newtypes: distinct Rust types for radians and degrees with explicit conversions. |  |  |
| 2026-11-13 | Binding conventions and batch-mode documentation | Signed tensor entries versus products of inertia; low-speed fallback angles and explicit errors for non-finite inputs; batch stop conditions and local per-task commits. |  |  |
| 2026-11-13 | Hero-aircraft and release-scope documentation | Mach-indexed coefficient tables and interpolation; source-axis derivatives must be transformed before use in body axes; a future hero demo and source-data availability do not establish current model coverage. |  |  |
| 2026-11-13 | Batch-rule clarification and documentation baseline | A task's prerequisites determine which open decisions block it; stable, unique decision IDs keep references unambiguous; a local documentation commit records an agreed baseline. |  |  |
| 2026-11-13 | T1 | Continuous integration runs checks on pushes and pull requests; rustfmt checks formatting, Clippy checks code warnings, and tests check expected behaviour; required gates must fail the workflow when a check fails. |  |  |
| 2026-11-13 | T0b | Project display names, repository slugs and Rust package names are separate identifiers; the Cargo workspace, package manifest and generated lockfile must agree; a package-identity test detects an incomplete rename. |  |  |
| 2026-11-13 | T2 | Radians and degrees are distinct types with checked conversions; a body-to-NED quaternion rotates body vectors into world coordinates, and composition order matters; at singular pitch, Euler roll and yaw cannot be determined separately even though the quaternion remains valid. |  |  |
| 2026-11-13 | T3 | Geometric height and geopotential height are different atmosphere inputs; Mach compares airspeed with sound speed, while dynamic pressure scales with density and speed squared; a relative comparison tolerance accommodates a documented table discrepancy and must still reject deliberate physical mistakes. |  |  |
| 2026-11-13 | T4 | Six degrees of freedom couple position, body velocity, orientation and body angular rates; inertia links turning moments to changes in spin, with signed cross-axis entries; RK4 samples four intermediate rates and step-size convergence checks how numerical error decreases. |  |  |
| 2026-11-14 | T5 | A versioned JSON schema gives saved data an explicit contract; validation rejects wrong units, frames and table shapes before use; synthetic test fixtures exercise software without pretending to describe real aircraft. |  |  |
| 2026-11-14 | T6 | Dimensionless coefficients scale aerodynamic forces with density, speed squared and reference area; wind-axis lift/drag/side forces must be rotated and moments transferred to the CG; interpolation supplies coefficients at the current Mach/altitude, and coupled convergence checks reevaluate those loads throughout each RK4 step. |  |  |
| 2026-11-14 | T7 | A headless runner translates files into pure simulation inputs and writes histories; piecewise-linear controls must be sampled at intermediate integration times; byte determinism and round-trip number formatting differ from numerical accuracy, which can lose its smooth-case order at lift/table transitions. |  |  |
| 2026-11-14 | T8 | Trim balances force along/across the flight path and pitching moment about the CG; body-axis thrust has both path-parallel and normal components; static margin compares small lift and CG-moment changes with elevator held fixed, so an aircraft can be trimmed yet statically unstable. |  |  |
| 2026-11-14 | Review Fix 1 | A general serializer can accept representations that a particular file format forbids; shape checks distinguish named objects from legitimate vector/table arrays; parsing original text preserves duplicate-field and source-location diagnostics. |  |  |
| 2026-11-14 | Review Fix 2 | Bilinear interpolation includes a mixed term that planar corner data cannot expose; an off-centre hand calculation can distinguish algorithms; injecting a deliberate fault in an isolated copy demonstrates that each regression test can fail. |  |  |
| 2026-11-14 | Review Fix 3 | A moment coefficient needs dynamic pressure, area and the correct reference length to become a turning moment; roll/yaw use span while pitch uses mean chord; sign checks alone miss magnitude errors, so distinct lengths and isolated hand-calculated terms expose swaps. |  |  |

| 2026-11-14 | T9 | Trilinear interpolation retains interactions between Mach, altitude and throttle; an offset thrust force turns the aircraft according to the CG-to-engine arm crossed with force; level trim solves throttle to balance drag while reported fuel flow leaves mass unchanged in this model. |  |  |

| 2026-11-14 | Generic object regression | Recursive discovery covers newly added records without a path checklist; preserving source field order avoids unrelated type errors hiding positional decoding; deliberate faults establish that regression tests can fail. |  |  |

| 2026-11-14 | T9b | Nondimensional rates compare rotation over a wing/chord travel time; negative direct rate derivatives oppose spin and give exponential roll decay in an isolated fixed-speed case; lift caps and surface travel saturation bound the model, so commanded and applied angles can differ. |  |  |

| 2026-11-14 | T12 | A symmetric wing can be described by right-half stations and mirroring; area and MAC come from chord integrals, with MAC differing from average chord; body-axis geometry and cited section inputs must be explicit before estimating forces. |  |  |

| 2026-11-14 | T13 | A Fourier series represents circulation and induced downwash across a wing; non-elliptic loading adds induced drag through higher modes; analytic benchmarks and term refinement check equations/numerics while physical validity remains a separate question. |  |  |

| 2026-11-14 | CSV contract review | A versioned header is a data contract while consumers select fields by name; decimal spellings can differ yet round to identical binary floating-point values; pinning exact output catches accidental schema drift. |  |  |

| 2026-11-14 | T14 | A directed vortex filament induces velocity by the Biot-Savart law; horseshoe legs must have consistent incoming/bound/outgoing orientation; a mathematical filament is singular on its centerline, so explicit errors differ from a physical vortex-core model. |  |  |

| 2026-11-14 | T15 | A vortex lattice solves circulation so flow follows a wing surface; force positions determine pitching moment and neutral point while the far wake estimates induced drag; mesh error and method differences require separate evidence, and finite-data extrapolation is conditional even when a zero limit is compatible with the observations. |  |  |

| 2026-11-14 | T13 Gap 1 | Higher Fourier modes affect induced drag even when lift stays unchanged; fixed results from an independent solver can detect a wrong mode weight; decimal reference rounding and numerical roundoff need an explicit small comparison budget. |  |  |

| 2026-11-14 | T12 Gap 2 | Interior ordering errors need cases beyond root and tip; exact structured errors protect both rejection and diagnostics; removing independent root and ordering guards demonstrates which boundary each regression protects. |  |  |

| 2026-11-14 | T15 acceptance review | A fit verdict that changes with the subset is weak acceptance evidence; convergence to a method's own limit differs from agreement with another theory; a prediction band from coarser meshes can be checked with a held-out finer result without claiming a physical error bound. |  |  |

| 2026-11-19 | T15b Part 1 | Relative error compares methods with different lift slopes; the planar minimum-drag bound gives an exact efficiency ceiling within its assumptions; an extrapolation can be screened without assigning it a tolerance or claiming physical validation. |  |  |

| 2026-11-19 | T15b Part 2 | A numerical prediction band belongs to its specific discretization; provenance headers can preserve an independent program without changing its logic; finite-range fitted exponents describe observations without establishing their cause or an asymptotic law. |  |  |

| 2026-11-19 | T15b Part 3 | Linear rescaling compares span efficiency at the same lift coefficient; a drag-gap decomposition depends on the chosen counterfactual order; local mesh refinement measures sensitivity without identifying a physical cause or proving global convergence. |  |  |

| 2026-11-19 | T15b follow-up Part 1 | Compiler optimization can change the last floating-point digits while retaining runtime checks; exact bit comparisons differ from tolerance comparisons; wall time and process peak memory measure different resource costs. |  |  |

| 2026-11-19 | T15b follow-up Part 2 | Comparing full-precision references separates rounding from solver differences; a dense matrix's memory grows with the square of its panel count; matching independent implementations does not validate the shared aerodynamic assumptions. |  |  |

| 2026-11-19 | T15b follow-up Part 3 | Mean geometric chord and local tip chord measure different mesh scales; a log-space fit and an original-space fit minimize different residuals; finite Fourier refinement and visible fit residuals describe numerical sensitivity without proving a physical cause or an infinite-aspect-ratio law. |  |  |

| 2026-11-19 | Documentation inventory | A local stability derivative does not establish a stall or engine-off glide comparison; fixed stability axes, body axes and moment-reference locations must stay attached to source quantities; physical control travel, moment-sign definitions and separate yaw-damper surfaces are different evidence. |  |  |

| 2026-11-19 | Documentation go/no-go, Part A | Wing-only lift and pitch slopes are not whole-aircraft derivatives (soas-estimator VLM assumptions/output; Caughey §§2.5-2.7 pp. 19-21); a coefficient at one operating point does not supply an aircraft's full polar or engine-off validation case (CR pp. 76-79, 101-103); flight-identification uncertainty differs from flight-versus-tunnel disagreement (MEMO 2-3-59L pp. 12-16). |  |  |

| 2026-11-19 | Documentation go/no-go, Part B | Right-surface and paired-aileron variables can have opposite signs (JSBSim Flight Control chapter §Conventions; Caughey §1.2.2 p. 5); a moment-sign definition does not prove physical trailing-edge travel (CR A-4); accessible author slides are separate evidence from an unopened published textbook passage (Beard/McLain chapter 4 slide 8; SIGN_EVIDENCE.md access record). |  |  |

| 2026-11-19 | Documentation estimator plan | Wing, tail and body loads interact through downwash/sidewash rather than forming independent corrections (Caughey §§2.5-2.6, pp. 19-20; JVL §4.3 pp. 9-10); quasi-steady rotation rates and delayed alpha-dot effects require different models (Caughey §4.3 pp. 58-59, 65-69); an independently specified hand problem verifies a method, while comparing an aircraft total requires matching source definitions and configuration (CR pp. 100-103, A-1/B-1; ESTIMATOR_PLAN.md). |  |  |

## T4 concepts introduced: equations of motion in plain language

An aircraft can move in three directions and rotate about three axes. These
are its six degrees of freedom. The state records where its centre of gravity
is, how fast it moves, which way it points, and how fast it turns. It contains
values that can later be saved, rather than references to a running program.

The position equation says that velocity changes position. We store velocity
along the aircraft's forward, right and down axes, so orientation first converts
it into North, East and Down directions before updating world position.

The velocity equation says that force divided by mass changes speed and
direction. Gravity points down in the world and must be rotated into the
aircraft's axes before adding it. There is also a correction for the fact that
the aircraft's axes turn: a velocity that stays fixed in the world can have
changing forward and sideways components as the nose swings around. That
correction describes changing coordinates; it is not another applied force.

The rotation equation is the turning counterpart of force equals mass times
acceleration. A moment is a turning influence. Inertia describes resistance to
changes in rotation and how rotation about different axes couples together.
Off-diagonal tensor entries carry signs and are the negatives of the products
of inertia defined in D11. Even without a turning moment, a non-spherical body
can have changing body-axis angular rates while its angular momentum stays
fixed in the world. The equation includes that coupling.

The attitude equation uses the body angular rates to change the quaternion,
our four-component orientation representation. Its length must remain one to
represent a rotation, so it is normalized after every step. Euler angles are
useful for display, but are not the variables being advanced here.

RK4 estimates the changing rates at the start, twice near the middle, and at
the end of each step, then combines them. Forces are recalculated at those
intermediate states. For smooth motion, halving the step should reduce the
accumulated error by about sixteen times. Checking that pattern against an
analytic answer tests the integrator; it does not validate an aircraft model.
Sources and mathematical details: `data/reference/dynamics/README.md`.
