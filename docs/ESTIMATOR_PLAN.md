# Estimator plan: static derivative milestone

## Current scope and pre-modeling checkpoint — T16c

The owner has answered T16c questions 2-5: **park TN 1270**, with **NO-GO for a fully sourced experimental comparison** and no further source reconstruction; **defer wing-tail reference work**; **defer F-104 searching to v0.2**, retaining RM SL56H06 and the inventoried flight-data leads; **park R&M 2882 and RM L52G18**. First experimental scope remains wing-alone if a suitable dataset later becomes available. CRM-HL remains excluded from that first comparison. (Owner post-T16c answers; [closure](T16C_TN1270_CLOSURE.md) §§3/7; [T16b](T16B_REFERENCE_SEARCH.md) §§3-6/9.)

**Source checkpoint answered; implementation still not started.** The T16c gaps remain: exact finite-wing Mach is unverified; continuous spanwise twist/section transition and a matched independently sourced section-law set are partial. Definitions/reuse are closed, not the missing inputs. No candidate assumption is adopted. T16/T16b/T16c documentation is complete; further work in the entire T16-T28 chain is **parked**, preserving the audits and plans below. (Owner answers 2-5; closure §§3/5/7.)

**v0.1 validation status:** "code-to-code cross-checked and analytic-case verified; not experimentally validated". The validation page and HTML report disclaimer must both say this plainly and identify actual evidence, assumptions and missing capabilities; no AVL/OpenVSP run or experimental agreement is implied. The next development direction is **Stage 3, designer model and report**, planned as S3-01 onward in [ROADMAP.md](ROADMAP.md#stage-3-proposed-task-plan--owner-answers-required-before-execution). Only documentation is authorized now; the new Stage 3 questions must be answered and a task named before implementation. The parked comparison chain is not a prerequisite to this limited designer/report route. (Owner post-T16c consequences; ROADMAP Stage 3/Evidence rules/Questions.)

**T10 is parked**, including the owner's published-derivative comparison direction and the retained earlier power-off stall/glide/trim/static-margin reference goals. Completed audits are not completed comparisons. H3/H4, D8/D9 and T11's v0.2 source gates are unchanged; no experimental accuracy pass/fail claim before D9. This supersedes the earlier sequencing that made the static milestone a blocker to all further v0.1 designer work. (Owner post-T16c consequences; [TASKS §T10](TASKS.md#t10-compare-against-reference-parked); DECISIONS §§D8-D9.)

## Parked work register and reopening conditions

Experimental validation continues as a **parallel track**. The candidate/gap
register is [ROADMAP.md, Experimental validation](ROADMAP.md#experimental-validation-parallel-track-parked-candidates):
TN 1270, TN 1378/Report 678, Navion, the parked wing fallbacks, and v0.2
RM SL56H06/F-104 flight-data leads. Revisit at v0.2 for supersonic data or when
a suitable wing-alone/wing-tail dataset is found, without an ongoing source
reconstruction task now. MEMO 2-2-59H's exact F-104 identity remains unverified;
MEMO 2-3-59L is comparative methodology evidence, not identified F-104 data.
(Owner answers 2-5; T16b §§3-6; inventory/other_sources, Scope and MEMO sections.)

| Retained ID | Parked scope and what would reopen it |
|---|---|
| T10 | Published derivative/reference comparisons, retaining legacy performance goals. Need a selected audited dataset, matched inputs/definitions, capable model, H3/reference cases as applicable, D9 where an accuracy verdict is sought, and an explicitly authorized comparison scope. No Stage 3 result silently completes it. (TASKS §T10; owner consequences.) |
| T16 (including T16b/T16c) | Completed source/definition audits stay complete; further audit/search is parked. Reopen on new suitable evidence or a specifically authorized new search, not to repeat the rejected TN 1270 reconstruction automatically. (Closure §§3/7; owner answers.) |
| T17 | Sourced section lift/moment inputs. Reopen on a named model need, independent section data or explicitly approved idealization, a checked formulation and owner task authorization. (Retained §T17.) |
| T18 | Arbitrary surface orientation/full vector loads. Reopen for a separately specified non-planar/tail capability with independent rotated-load tests. (Retained §T18.) |
| T19 | Coupled wing-tail/downwash. Need suitable geometry/section inputs and a checked coupled formulation; a new clean wing-tail dataset can motivate review, not automatic implementation. (Retained §T19; T16b §4.) |
| T20 | Fuselage contribution. Need sourced geometry, applicable body method/limitations and independent body tests. Describing a fuselage in Stage 3 does not implement this task. (Retained §T20.) |
| T21 | Wing-body interference. Need an explicitly scoped, sourced interaction treatment with double-counting checks. Supplying aircraft totals in Stage 3 does not claim this model exists. (Retained §T21.) |
| T22 | Finite-Mach treatment. Need an owner-selected regime/condition and verified method; a supersonic v0.2 case needs its own regime-appropriate formulation, not a PG extrapolation. (Retained §T22; A Compressibility; RM SL56H06 pp. 2-5.) |
| T23 | Current static plan: first lift-slope comparison/checkpoint; original broader control-effectiveness proposal remains in 692c4d0. Reopen only with an explicit choice of that scope, ready source/model inputs and the retained stop after the first comparison. (Retained §T23; historical registry note below.) |
| T24 | Current static plan: pitch slope/neutral point after checkpoint; original quasi-steady rotation-rate proposal remains in 692c4d0. Need explicit scope, matched references, independent derivative data and approval after any required lift checkpoint. (Retained §T24; historical registry note.) |
| T25 | Total drag/drag derivatives: independently sourced profile/body/junction data and an applicable integration model; induced drag alone is insufficient. (Restored original plan below.) |
| T26 | Unsteady terms/omission assessment: independently specified history/rate definitions, applicability and data, not substitution of a quasi-steady rate derivative. (Restored original plan below.) |
| T27 | Propulsion/reference-state closure: documented power/propeller/inlet state and independently sourced method/data; no real reference-state model is inferred from existing synthetic jet tables. (Restored original plan below.) |
| T28 | Staged published comparisons: frozen source-matched predictions, traceable omissions and explicit owner selection of derivative stages, after dependent source/model gates. (Restored original plan below.) |

**Reopening is a deliberate owner action, not a date-based resume.** For model
work, first agree the missing capability, sourced method, independent tests,
mutation evidence and data/assumption contract. For experimental comparisons,
also audit geometry, section inputs, units/axes/reference area/chord/moment
origin, configuration/control/power state, actual condition and reuse terms;
match the compared observable and state omissions. D9 precedes an accuracy
acceptance verdict. Reopening one task does not resume the whole chain or
authorize coefficient transcription without scope. (AGENTS, Batch mode;
T16b §2; closure §§3-5; owner consequences.)

The original broad T16-T28 plan is retained in commit **692c4d0**; the later
static rescope is in **5577ebb**. No IDs are reassigned: detailed current
T17-T24 sections remain below; original T25-T28 text is restored below as
parked context. The original control/rate scopes associated with T23/T24 are
preserved in that history and this register, not silently reactivated. S3 IDs
refer only to the new proposed designer/report tasks. (Repository history;
owner request to park rather than delete.)

## Stage 3 interface and honest whole-aircraft handling

The detailed proposal and pre-start questions live in ROADMAP.md, S3-01-S3-09.
Today the estimator supplies **wing-only planar VLM**, with no tail/fuselage
model. The designer may store their geometry but must mark their aerodynamic
contributions **not estimated**. Recommend supplied complete aircraft
coefficient data, with explicit tail/body/interaction coverage and provenance,
before aircraft trim/static-margin checks or simulator export; missing terms
remain unavailable, not zero. Basic mass/wing-loading and standalone CLmax
checks require only their own inputs. Effects included in a supplied total
are not separately resolved component estimates. A supplied total must not have the wing estimate
added again. Per-component assembly is an alternative requiring its own
reference/interaction specification, not an approved new estimator task.
(Owner Stage 3 requirement; VLM `ASSUMPTIONS`/`VlmOutput`; core `Aircraft`;
ROADMAP, Proposed honest data contract/S3-04/S3-05/question 2.)

Geometry-only drafts can receive geometry/wing-only results and a partial
report. Whole-aircraft stall speed needs independently supplied matched
CLmax; VLM cannot provide it. Existing core static margin is aerodynamic,
control-fixed and excludes propulsive derivatives; local trim failure is
reported, not converted to a categorical impossibility claim. No whole-aircraft
validity, dynamic-stability or safe-flight claim follows from these checks.
(ROADMAP S3-07/S3-08; core `trim::static_margin`/`TrimError`; C §3.1 p. 28.)

## Scope and references

CL_alpha is lift slope; Cm_alpha is pitching-moment slope, both for the complete aircraft at the matched source condition with the agreed controls held fixed. The neutral point is the location about which the control-fixed pitch slope vanishes. Current planar VLM/lifting line describe an isolated wing; their results are not published Navion aircraft totals (CR pp. 100-103; C §3.1 pp. 25-28; repository references below).

| Key | Source / evidence |
|---|---|
| **CR** | Teper, *Aircraft Stability and Control Data*, NASA CR-96008, April 1969, [PDF](https://ntrs.nasa.gov/api/citations/19690022405/downloads/19690022405.pdf?attachment=true): Navion pp. 100-103, source definitions A-1-A-10 and transformations B-1. The condition Mach was rechecked visually in X-A/B, p. 101 / PDF 107. |
| **C** | Caughey, *Introduction to Aircraft Stability and Control*, Cornell, 2011, [PDF](https://courses.cit.cornell.edu/mae5070/Caughey_2011_04.pdf): section inputs/body/downwash §§2.3-2.6 pp. 14-20; static slopes, tail dynamic-pressure ratio and neutral point §3.1 pp. 25-28; force-axis differentiation §4.3 pp. 56-58. |
| **J** | Drela, *JVL 2.20 Formulation*, April 2024, [PDF](https://web.mit.edu/drela/Public/web/jvl/jvl.pdf): geometry/velocities/tangency §§2-5 pp. 5-18, body approximation §4.3 pp. 9-10, forces §6 pp. 19-24 and sensitivities §7 pp. 27-28. Only the unblown attached-flow subset is a method candidate here. |
| **A** | Drela/Youngren, *AVL 3.40 User Primer*, February 2022, [text](https://web.mit.edu/drela/Public/web/avl/avl_doc.txt), §§Configurations, Compressibility, CLAF and Stability derivatives. Read as method documentation; no AVL calculation was run. |
| **D** | *The USAF Stability and Control Digital DATCOM*, AFFDL-TR-79-3032, Volume I, April 1979, [scan](https://www.pdas.com/refs/AFFDL-TR-79-3032-Digdat-manual-DGS-scan.pdf), §§2.2.1-2.2.2 pp. 8-9. The earlier review checked applicability, not the underlying body/interference charts; no empirical factor is selected. |
| **P** | MIT, *Thermodynamics and Propulsion*, [§11.7.2-11.7.3](https://web.mit.edu/16.unified/www/SPRING/propulsion/notes/node86.html): momentum/energy and ideal slipstream assumptions, used only to discuss omitted propulsion effects. No propeller or engine model is planned for this milestone. |
| **Inventories / implementation** | [Decision sheet](../data/reference/inventory/go_no_go.md), [CR inventory](../data/reference/inventory/cr96008.md), [other sources](../data/reference/inventory/other_sources.md), [sign evidence](SIGN_EVIDENCE.md); [VLM](../crates/soas-estimator/src/vlm.rs) and [lifting line](../crates/soas-estimator/src/lifting_line.rs), assumptions/outputs; [aircraft format](AIRCRAFT_FORMAT.md), coefficient fields. Repository material is unpaginated. |

These sources were opened in the documentation work; numerical Navion fixture data have not been transcribed. The Mach and derived PG diagnostic below are expressly requested exceptions. Existing source discrepancies remain unresolved. An exact three-dimensional PG implementation and Navion-specific interference/slipstream magnitudes are not verified here. Source absence is not permission to fill a gap from memory.

The simulator's other derivative fields remain **future candidates only**: Cm_delta_e, CY_beta, Cl_beta, Cn_beta, Cl_delta_a, Cn_delta_r, Cm_q, Cl_p, Cn_r, Cl_r and Cn_p. Each needs its own approval after the lift-slope checkpoint. There are no task plans for their comparisons in this revision. Drag, Mach-derivative and angle-of-attack-rate comparisons are outside the planning horizon. Evaluating a PG factor at the fixed reference condition does not add a Mach-derivative comparison (CR pp. 101-102; AIRCRAFT_FORMAT §§Coefficient fields / Dynamic derivatives and limits).

## Static prediction budget

This is a budget of effects to assess, not a measured decomposition of CR's aircraft totals. Downwash and interference change component loading; do not add them twice as independent loads (CR p. 101; C §§2.5-2.6; J §4.3).

| Target | Physical contributions | Current capability | Missing / disposition |
|---|---|---|---|
| CL_alpha | Wing and horizontal-tail lift, wing-wake downwash, fuselage loading and wing/body/tail interaction; finite-Mach and possible propulsion effects. | Isolated wing slope from VLM; LLT through its linear lift law. | Matched section/geometry inputs and coupled airframe model; incompressible assessment and sourced omitted-propulsion discussion. |
| Cm_alpha | Wing load moment, tail load/arm and downwash, fuselage pitching moment and interference; finite-Mach and possible propulsion effects. | Planar wing load-moment slope from VLM at its declared reference; LLT has no moment output. | Complete static airframe moment budget, source-axis/reference conversion and the same omission assessments. |
| Neutral-point location | The whole-aircraft slopes and agreed moment/geometry references. | VLM's isolated-wing neutral point only. | Derive aircraft/reference locations consistently when Cm_alpha is compared; a slope-derived published reference location is not an independent measured datum. |

## T16 audit contract — historical whole-aircraft scope

This is the earlier audit contract; its completed result is [T16_AUDIT.md](T16_AUDIT.md) §§1-7. The current wing-alone direction and pre-modeling checkpoint are stated above; the Navion requirements below are retained for deferred whole-aircraft work. (Owner T16c Decisions; TASKS §T16; Current scope section above.)

**Purpose / size:** establish source sufficiency and the comparison definitions; medium. Search for documented power-off evidence first. No geometry inference from an unlabeled sketch, no aircraft-number transcription beyond separately authorized audit requirements, and no propeller/engine implementation.

The audit must end with **GO or NO-GO for the first whole-aircraft lift-slope estimate**, with a source/page for every required input and every omission. GO means input/model readiness for a scoped estimate, not an accuracy verdict. Identify whether any GO is qualified by an unmodeled propulsion contribution. When evidence cannot support the expected size/direction of an omission, mark that assessment unverified and say how it limits the verdict.

The existing inventory establishes **insufficient inputs from the inspected CR Navion section alone**. This is a planning baseline, not a completed search or the final T16 verdict:

| Required input | Existing coverage and missing items |
|---|---|
| Exact variant and configuration | Family name only; traceable variant, modifications, flap/gear/control state and configuration match are missing (CR pp. 99-103). |
| Wing planform and airfoil | Three-view and reference dimensions, but no complete station coordinates/chords, sweep/twist/incidence, airfoil identity/coordinates or independently sourced section law (CR pp. 100-101). |
| Tail geometry and position | No complete horizontal-tail planform/airfoil/incidence or wing-to-tail coordinates/lever arm in a common datum; vertical-tail geometry is also incomplete and its relevance to the chosen longitudinal configuration must be assessed (CR pp. 100-101). |
| Fuselage dimensions | No sourced centerline, length/cross-section distribution or wing/body junction definition sufficient for the proposed body/interference model (CR pp. 100-101). |
| CG and reference definitions | Relative CG and reference area/span/chord are present; common absolute datum, reference-point coordinates and complete moment-reference provenance are missing. The two printed reference areas disagree; no normalization has been selected (CR pp. 100-101, A-1, B-1). |
| Flight condition and power state | A nominal condition is given, but documented power-off/propeller state, configuration and atmospheric/altitude provenance are incomplete; an unlabeled power state is not engine-off evidence (CR pp. 100-103). |

T16 must seek the missing items, list each unresolved one explicitly, and distinguish unavailable data from unclear OCR/source conflicts. Preserve the blank lateral trim-angle cell and the absent axes note on X-D/E rather than silently borrowing definitions (CR pp. 101-102; inventory).

**Required NO-GO follow-up:** if Navion geometry/reference/configuration coverage remains insufficient, say so plainly and propose an alternative reference aircraft with complete, public, mutually matching geometry and static reference data. Cite sources/pages for its wing/airfoil, tail geometry/position, fuselage, CG/reference definitions, configuration, condition and power state; open/check them rather than assuming completeness. A geometry file alone, a generic aircraft name or the queued Cessna fallback is not proof of such coverage. Include known omissions and the alternative's actual-Mach PG assessment. Propose the substitution to the owner; do not select it silently. No alternative is certified complete by this planning update.

**Method/source:** define source axes, units, moment origin, geometric datum, fixed controls and held-fixed flight quantities. Transform forces/moments and differentiate the full transformation, retaining baseline-force terms where the rotating force axes require them (CR A-1/A-6-A-10/B-1; C §4.3 pp. 56-58; J §7.3 p. 28).

**Independent checks / planned mutations:** hand-rotate a prescribed force and differentiate it; hand-shift its moment between distinct origins; check angular/slope units. Mutate rotation direction, omit the baseline-force term, reverse the reference arm or confuse angle units. These are future checks of any implemented conversion, not tests run during this documentation update. Main risk: source definitions may not permit a unique matched aircraft model.

## Incompressible assessment and PG diagnostic

Owner decision: an assessed incompressible approximation is acceptable for reporting, with no accuracy pass/fail before D9. At **every actual published comparison condition**, compute and report

    F_PG = 1 / sqrt(1 - M²).

The source for the formula and its linear small-disturbance limitations is A, §Compressibility. The owner's approximate hold point is **Mach above about 0.3 or a factor increase above about 5%**: stop and ask whether finite-Mach treatment must precede the comparison. This is an owner review trigger, not a universal physical-validity threshold. Ask at a borderline case rather than inventing a sharper rule.

**Current CR Navion condition:** M = 0.158 (CR X-A/B p. 101 / PDF 107). Direct evaluation gives F_PG ≈ **1.01272**, an increase of **1.272% relative to the incompressible value**. Equivalently, an uncorrected value is **1.256% below the PG-corrected value**, using that corrected value as denominator. This condition is below the owner's hold point. These are derived diagnostic numbers, not new aircraft coefficients or validation tolerances (CR cited page; A §Compressibility).

For a positive lifting-surface slope, the simple PG diagnostic raises the incompressible slope; it can therefore explain an underprediction component, not an existing overprediction. Under that uniform-scaling assumption, omission alone would give the 1.256% shortfall relative to the corrected slope stated above. It is **not** a demonstrated uniform correction to the whole aircraft: three-dimensional loading, body effects and downwash need assessment (A §Compressibility; C §§2.5-2.6).

**Share of the expected discrepancy:** no whole-aircraft estimate or sourced total discrepancy budget exists yet, so a numerical fraction of that total cannot honestly be given now. T16 must establish a sourced expected-effect budget, stating unknowns; at the first comparison report the signed diagnostic share using consistent units/denominators:

    delta_total = a_reference - a_incompressible
    delta_PG = (F_PG - 1) * a_incompressible
    diagnostic_share = delta_PG / delta_total, if delta_total is nonzero.

Here a denotes the selected whole-aircraft lift slope; applying this uniform diagnostic is an explicit approximation, not a three-dimensional PG solution. If the total gap is zero/near zero or lacks a defensible expectation, report the absolute slope increment and its percentage scale instead of a misleading share. Identify whether the PG direction reduces or increases the observed discrepancy. Do not call this causal attribution or use it to tune the estimator.

## Power-off search and unmodeled propulsion

Seek a source documenting power-off and the propeller's actual state/configuration. If the source is power-on or silent, label propulsion **unmodeled**; do not build a propeller or engine model for this milestone. T16 and each comparison must give a sourced expected size and direction for the resulting discrepancy, identifying assumptions and uncertainty. If evidence cannot establish a size or net direction, state that explicitly and carry the unresolved assessment into the go/no-go and checkpoint. Do not substitute a plausible range or silently set the effect to zero.

A source-based conditional mechanism is available: C §3.1 pp. 25-27 expresses the tail contribution through its local dynamic-pressure ratio and downwash; P §11.7.3 describes ideal axial slipstream acceleration. If only tail dynamic pressure increases, with a positive effective tail lift slope aft of CG and geometry/downwash otherwise fixed, the tail adds more positive lift slope and more stabilizing pitch slope. Under those assumptions, omitting that powered-tail contribution biases lift slope low and pitch slope toward less negative values. This is an inference from the cited model, **not a verified net sign or magnitude for the Navion**. Wing slipstream, changed downwash and direct propulsive moments can alter the net result. The inventoried section supplies neither the needed tail geometry nor a matched propulsion-state/slipstream increment, so the Navion-specific size and overall sign remain unverified (CR pp. 100-103; C cited pages; P cited section).

## Proposed static enabling work — separate approval required

**All T17-T28 work below is parked by the owner's post-T16c answers.** This preserves a deferred build sequence for later owner decisions; each implementation needs independent hand/analytic expectations first and mutation evidence. Rough sizes are relative, not time estimates. Incompressibility/propulsion disclosures do not turn a wing result into a whole-aircraft estimate. Stage 3 is separately proposed in ROADMAP.md; none of this chain starts through it. (Owner consequences; Parked work register; AGENTS.md, Batch mode.)

### T17 — Sourced section lift and moment inputs — medium

- **Purpose/contribution:** replace the VLM's mandatory ideal section slope where justified by independent section data; retain zero-lift angle, incidence and section pitching-moment law with provenance. This is wing/tail section physics, not a fit to Navion aircraft slopes. LLT already accepts section slope/zero-lift inputs; current VLM rejects slopes outside its ideal assumption (repository references; A `CLAF`; C §2.3-2.4).
- **Method/source:** a linear section lift law with a checked boundary-condition treatment, such as A's `CLAF`-style control-point adjustment, plus independently supplied section-moment dependence where known. The primer describes the slope mechanism but the detailed mapping and its effect on moments must be derived/verified before coding. Do not count the same section moment in both the lattice loading and an added moment. Do not scale final aircraft coefficients to match the target. Preserve the existing ideal-section mode and its placement assumptions (A `CLAF`; C §2.4 pp. 18-19; J §§4.5, 5.1-5.2).
- **Independent tests:** prescribed linear section lift/zero-lift/moment laws integrated over a rectangular strip; the analytic elliptic LLT slope with independently specified section slope; high-span two-dimensional limiting behavior. Cross-reference existing ideal-wing hand cases without changing their expectations.
- **Mutations:** ignore section slope, reverse zero-lift angle/incidence, drop the section moment or apply section area normalization twice.
- **Risks / use:** section data, Reynolds dependence and camber may be missing; potential-flow thickness correction is not a viscous model. Needed to match the selected Navion wing/tail assumptions; independent inputs remain a source gate.

### T18 — Arbitrary surface orientation and full vector loads — large

- **Purpose/contribution:** permit the non-planar wing and independently positioned horizontal-tail surfaces required by the selected static configuration, with full force/moment vectors about an explicit datum. Existing planar restrictions cannot simply be bypassed. No sideslip/control derivative extension is planned here (repository VLM/planform; J §§2.1, 3.1, 4.1-4.2, 6.1).
- **Method/source:** horseshoe-vortex influence projected onto actual panel normals, with vector Kutta-Joukowski loads and reference-arm moments. Convert source geometry conventions explicitly; trailing wakes follow downstream flow, which is along body negative x in the existing linearized convention. Keep the small-angle/attached-flow scope (J cited sections; existing [vortex convention](../data/reference/estimator/vortex.md)).
- **Independent tests:** rigidly rotate a hand-solved isolated surface, all vortex endpoints and its freestream and recover the rotated load; rotate a horizontal lifting surface into a vertical one and check side force/moments; manually evaluate an off-axis horseshoe and cross-product moment; zero-dihedral mode recovers the existing planar hand solution.
- **Mutations:** keep a global vertical normal, discard a force or moment-arm component, mirror the wing incorrectly, or send wakes upstream.
- **Risks / use:** intersecting surfaces/wakes, near-singular geometry and panel orientation need explicit diagnostics, not an invented core radius. For this milestone use the wing/horizontal-tail geometry needed for longitudinal loading. General lateral derivatives and their vertical-tail extensions remain candidates outside this planning horizon.

### T19 — Coupled wing and horizontal tail, including downwash — large

- **Purpose/contribution:** solve the wing and horizontal tail together for the selected static configuration. Their influence must enter the same tangency problem; summing independent lifting-surface coefficients misses wake interaction. Assess any other configuration component for a justified omission instead of expanding into lateral derivative work (C §2.6 p. 20; J §§4.2, 5.2-5.3).
- **Method/source:** include the velocities induced by every lifting surface and its wake at every other surface; integrate loads separately and as an aircraft total. Obtain tail flow from the modeled wake, not an adjustable downwash factor fitted to the Navion. A flat prescribed wake is the initial approximation, not a roll-up/separation model (J cited sections; A “Configurations”).
- **Independent tests:** solve a deliberately small wing/tail linear system by hand using independently evaluated vortex influences; compare a prescribed wake's induced velocity at an isolated tail with a direct hand Biot-Savart integral; check zero coupling and removed-tail limits. A scalar prescribed-downwash tail-lift/lever-arm calculation tests response, without treating that approximation as an exact oracle for the full lattice.
- **Mutations:** omit cross-surface influence, reverse downwash, discard the tail arm, normalize the tail by its own area when reporting aircraft coefficients, or omit its wake.
- **Risks / use:** wake placement near tails, tail dynamic-pressure loss and viscous wakes may exceed this model. Needed for Navion `CL_alpha`/`Cm_alpha` at the selected source condition.

### T20 — Isolated fuselage contribution — medium

- **Purpose/contribution:** add static body loads and pitch stiffness separately from lifting surfaces, with a checkable isolated-body baseline (C §2.5 pp. 19-20; J §§2.2, 3.2, 4.3, 6.1.3).
- **Method/source:** slender-body source/transverse-doublet representation from a sourced centerline and cross-section distribution; initially verify a smooth closed body of revolution. C's closed-body volume result is a useful analytic limit, not a complete model of a real cabin/fuselage or wing-body interference. A explicitly cautions about limited experience with slender-body modeling.
- **Independent tests:** integrate an analytic closed-body area distribution by hand: opposite end loads cancel in the stated ideal limit, while its pitch couple has the source-derived volume dependence and is independent of moment origin. A coordinate rotation must rotate the prescribed force/moment consistently; shrinking body cross-section removes its load contribution.
- **Mutations:** replace area by width, reverse the streamwise derivative, lose an end contribution, reverse the pitch sign or apply a CG shift to a pure couple incorrectly.
- **Risks / use:** noncircular/blunt geometry, separation and crossflow are limited or absent. Required assessment for Navion whole-aircraft static slopes; its lift contribution may vanish only in the tested ideal closed-body limit, not by assumption for the real aircraft. No lateral body-derivative comparison is planned.

### T21 — Wing-body interference without double counting — large

- **Purpose/contribution:** capture body-induced flow at the wing/tails and lifting-surface-induced changes in body loads. Adding isolated wing/body numbers does not do this (C §2.5 p. 19; J §4.3 pp. 9-10, §6.1.3 p. 21).
- **Method/source:** start with J's explicitly approximate treatment at zero body rotation: body source/doublet strengths use the steady freestream, their field enters surface velocities, and surface-induced flow enters body load evaluation. This is **not fully coupled body-strength determination**; J states that omission. Do not describe it as exact wing-body coupling. A junction/viscous empirical correction needs a separately verified method/data set; DATCOM's underlying correlations and Nelson's example have not been verified here (J p. 10; D pp. 8-9; C p. 19).
- **Independent tests:** hand-evaluate a simple body's source/doublet velocity at a lifting element and that element's velocity at a body station; use those prescribed fields to hand-compute load changes. Removing body strength or the surface must recover the respective isolated case. Check the total moment against a separate force/arm sum and retain a ledger distinguishing isolated and interaction loads.
- **Mutations:** disable either interaction direction, add isolated-body force twice, omit wing-root surface geometry or reverse the body-induced velocity.
- **Risks / use:** root/body intersections, slender-body approximations and viscous junction effects remain. The Navion static comparison requires an assessment; this task supplies an inviscid approximation, not proof that the Navion interference error is small.

### T22 — Conditional finite-Mach treatment — medium, not authorized

Purpose/contribution: a static finite-Mach correction only if the owner requires it at the PG hold point or later checkpoint. It does not include Mach-derivative comparisons. Method candidate: a consistent three-dimensional PG transformation of geometry, boundary conditions and force recovery (A §Compressibility). The exact mapping in project conventions remains unverified and needs a checked derivation before implementation.

Independent tests: zero-Mach recovery, the analytic high-span two-dimensional PG limit and a hand transformed-geometry force problem. Planned mutations: reverse or double the factor, transform loads without geometry, omit a transformation term or accept an unsupported regime. Main risks: PG does not model shocks/transonic separation or establish a universal aircraft correction. This task stays deferred when the owner permits the assessed incompressible comparison.

### T23 — First published lift-slope comparison and mandatory checkpoint — medium, not authorized

**Historical whole-aircraft route:** the first-comparison scope in the following paragraphs is superseded by Current scope above and the conditional wing-alone contract in T16c §4. The stop after a first comparison is retained; neither route is authorized to run now. (Owner T16c Decisions; T16C_TN1270_CLOSURE.md §§4/6-7.)

After separately approved prerequisites and T16's GO, freeze independent geometry/section inputs and produce the **first whole-aircraft Navion CL_alpha comparison** at the matched published condition. An alternative reference requires the owner's explicit approval and a correspondingly identified checkpoint. Report source/prediction definitions, mesh/section-input sensitivity, contributions and omissions, propulsion assessment, the actual-Mach PG factor and its diagnostic share/limits. Do not fit the model to the published target. No accuracy pass/fail before D9 (CR p. 101; C §§2.5-2.6; A §§Compressibility/Stability derivatives).

Independent checks: a prescribed synthetic comparison record with hand-derived unit/reference conversions and signed discrepancy; missing geometry or a mismatched variant/condition must remain unavailable. Mutations: substitute the prediction for the reference, compare an isolated wing with the aircraft total, change normalization/CG or silently zero a missing contribution.

**STOP after reporting this first comparison.** The owner reviews its discrepancy, source completeness, omitted effects and usefulness, and decides whether to revise the model, change reference aircraft, require finite-Mach work or authorize the next task. No later task, no pitching-moment/neutral-point comparison and no candidate derivative work starts automatically. If T16 is NO-GO, return that audit and alternative proposal to the owner before estimation; a wing-only substitute does not satisfy this checkpoint.

### T24 — Pitching-moment slope and neutral point — medium, after checkpoint approval

Compare whole-aircraft Cm_alpha, then report the neutral-point location from the matched whole-aircraft slopes/reference definitions. Give the common datum, body-axis coordinate and, where supported, the position along mean aerodynamic chord with its positive direction stated. Also report CG/moment reference, control-fixed definition, configuration/power caveats and sensitivity to both slopes. Convert C's aft-positive chord convention explicitly to the project's forward-positive body x; missing datum information must not be invented (C §3.1 pp. 27-28; CONVENTIONS §Coordinate frames; CR pp. 100-101).

A reference neutral point inferred from CR's slopes is a **derived comparison**, not an independently measured third target. Do not claim a power-off aircraft neutral point if the source power state is unresolved. Method: shift the full moment/load derivative consistently and locate the reference where the control-fixed pitch slope vanishes, under the stated linear approximation (C cited pages; J §§6.3/7.3).

Independent tests: prescribed lift and moment laws about distinct origins, with a hand-calculated neutral point; shifting the reporting origin must preserve its physical location. Mutations: reverse the moment arm, use wing-only lift slope with aircraft pitch slope, mix MAC leading-edge and CG datums or invert the ratio/coordinate direction. Risks: cancellation between body and tail moments, uncertain CG/MAC and correlated slope/reference errors. No accuracy pass/fail before D9.

## Retained original T25-T28 plans — parked

The task text below is restored from 692c4d0 without changing its methods,
tests or risks. It is historical planning, not authorization or a renewed
source search; the Parked work register above controls reopening. Its broader
comparison stages do not override the later owner checkpoints. (Owner
post-T16c consequences; repository history.)

### T25 — Total drag and drag-dependent derivatives — large

- **Purpose/contribution:** add independently sourced section/profile, body/appendage and junction drag to coupled induced drag; support `CD_alpha`, `CD_delta_e`, `CD_M` and drag-sensitive yaw derivatives. Current `CDi` is not total `CD` (repository VLM assumptions/output; J §§6.1.5, 6.2.5; A `CDCL`).
- **Method/source:** integrate explicitly supplied section polars and a separate body/interference drag model, with Mach/Reynolds/control indexing where supported. J supplies viscous load integration and A a section-polar interface; neither is Navion drag data. A Navion body/junction correlation and off/windmilling propeller drag are **not verified**; no default drag factor or total-aircraft calibration is proposed (CR pp. 100-103; D §2.2.1).
- **Independent tests:** hand-integrate a prescribed analytic section polar over a simple wing at prescribed loading; differentiate it analytically with respect to angle/control/Mach; add a known synthetic body drag load and check its moment. Compare differential left/right drag moments with a hand cross-product sum. A zero supplied profile term must recover induced-only accounting.
- **Mutations:** omit profile drag, count junction drag twice, use total `CD` as `CD0`, mix local/global reference areas or differentiate a polar while incorrectly holding its lift argument fixed.
- **Risks / use:** transition, Reynolds dependence, separation and control drag require data beyond inviscid VLM. Needed for the Navion drag/adverse-yaw targets and eventually glide; not all are required for a first lift-slope comparison. F-104 additionally needs regime-specific wave/base/inlet drag treatment, not this subsonic build-up alone.

### T26 — Separate unsteady terms and bound quasi-steady omissions — large

- **Purpose/contribution:** handle `CL_alpha-dot`/`Cm_alpha-dot` separately from rotation-rate derivatives and decide whether the latter need further unsteady corrections.
- **Method/source:** begin with C's low-frequency wing-to-tail convection-delay expansion, §4.3 pp. 58-59. It supplies a leading tail-downwash-lag contribution, **not a complete unsteady wing/body solution**. A quasi-steady wake has no such history. General unsteady circulation/added-mass, frequency dependence and an appropriate validation source remain **unverified**; require a further sourced specification if those terms are material or the owner wants the full published target set (A “Unsteady flow”; C cited pages; CR A-6, p. 101).
- **Independent tests:** prescribe an analytic time-varying downwash and delay, expand it by hand, and derive tail lift and moment rate terms; zero delay removes that particular term. Independently vary pitch rotation and alpha history so a test cannot pass by conflating them. The moment/lift ratio follows the prescribed tail arm, independent of the solver.
- **Mutations:** replace delayed by instantaneous downwash, reverse delay sign, call `Cm_q` an alpha-dot derivative, apply the delay contribution twice or normalize alpha-dot as roll rate.
- **Risks / use:** a local compiled derivative does not establish the relevant frequency/method or prove omitted unsteady terms negligible. Needed if Navion alpha-dot rows are selected; it is not necessary to implement wake history for a static `CL_alpha` milestone. F-104 dynamic comparisons have the same distinction. If the complete budget cannot be supported, mark that derivative unsupported rather than declaring this leading model complete.

### T27 — Conditional propulsion/reference-state closure — large if needed

- **Purpose/contribution:** reproduce the source's actual propulsion condition, or obtain a separately documented engine-off reference. Decide whether this task is needed in T16; if it is, it blocks acceptance of dependent comparisons even though it appears after the basic airframe modules in this development order (CR pp. 100-103; TASKS H3/T10).
- **Method/source:** combine verified propeller thrust/torque/operating data with an actuator-disk inflow baseline and explicit application-point moments, then assess slipstream effects on wing/tail loads. P §§11.7.2-11.7.3 verifies momentum/energy accounting and the ideal non-swirling assumptions; it does not supply the Navion's data, finite-blade swirl, asymmetric propeller effects or detailed local slipstream field. Those need a verified extension before claiming lateral/control derivative closure. D p. 8 likewise restricts its power-effect capability to longitudinal characteristics; it cannot establish all required lateral terms.
- **Independent tests:** hand-check disk momentum/energy balances, a prescribed local-speed change's effect on a synthetic tail lift law and the thrust force/arm moment. A genuinely non-working ideal disk removes its modeled thrust/induction contribution; this is not evidence that a real stopped/windmilling propeller has no drag.
- **Mutations:** add thrust twice, omit slipstream's dynamic-pressure effect, reverse the thrust arm or equate idle throttle with engine-off.
- **Risks / use:** source state/propeller data may be unobtainable; no matching powered derivative can then be validated. Not required for a genuinely documented engine-off Navion airframe comparison, though the corresponding propeller drag/configuration still belongs in T25. Powered F-104 work needs its own jet/inlet/thrust model rather than this propeller model.

### T28 — Stage the published comparison, with traceable omissions — medium

- **Purpose/contribution:** compare the selected aircraft derivatives with independent published targets, with no new physics hidden in the comparison layer. First `CL_alpha`, then `Cm_alpha`; afterward the selected control/lateral/rate subsets, and only then any expanded drag/Mach/unsteady set.
- **Method/source:** use T16's contract, independently transcribed/checked source records when authorized, per-contribution diagnostics, mesh/perturbation/section-input sensitivity and a frozen prediction before examining a fit. Publish differences and limitations; apply an acceptance threshold only after the owner's D9 choice. Resolve propulsion dependence before accepting any affected stage (CR pp. 100-103 and appendices; TASKS H3/H4/T10; [decision sheet](../data/reference/inventory/go_no_go.md)).
- **Independent tests:** a separate hand-written comparison record with deliberately different prediction/reference units and reference origins must yield the manually derived discrepancy; mismatched variant, condition, derivative definition or missing input must produce an unavailable comparison. The analytic cases from earlier tasks remain independent anchors. A reference-tool run, if later authorized/licensed and pinned, is a secondary cross-check, not the Navion oracle.
- **Mutations:** compare a wing contribution with an aircraft total, compare different CGs, substitute a predicted value for the published target, silently zero a missing term or label a combined derivative as an individual one.
- **Risks / use:** agreement after fitting to the same target is calibration, not independent validation; correlated source errors and missing provenance remain. This milestone does not by itself satisfy queued T10's power-off performance comparisons or T11's complete aircraft requirements.

## Later AVL cross-check and artifact policy

A version-pinned AVL cross-check is approved **in principle for later**, after license review, as secondary evidence. Record the actual release/hash, input provenance, axes/reference conventions and condition before use. Analytic/hand expectations and published reference data remain the primary evidence. No AVL run or generated result was produced in this update.

**License check performed for this documentation:** the official [Release Conditions](https://web.mit.edu/drela/Public/web/avl/) identify GPL licensing and link [GNU GPL version 2](https://web.mit.edu/drela/Public/web/gpl.txt). Its §0 makes output coverage depend on the output's contents; §§1-3 impose conditions on redistribution of covered material. This does not establish blanket permission for every file/result or a particular future release/input set. Review that pinned release's actual notices and the intended artifact/input licenses before committing anything derived from AVL.

Owner policy: do not commit AVL binaries or raw output unless the license clearly permits the particular artifact and all applicable conditions are satisfied. The preferred repository deliverable is our own summary tables with provenance after the license check. Unclear permission means leave the artifact uncommitted and ask. This is not a change to the project's open D2 license decision.

## What is already meaningful, and what happens next

Current synthetic verification can use the analytic elliptic LLT case, VLM's independent one-panel load/moment solution and hand symmetry/reference/limiting cases. These verify software within its idealizations, not the Navion. Existing finite-aspect-ratio VLM/LLT differences are not an equality oracle; preserve their evidence and tolerances ([LLT tests/reference](../data/reference/estimator/lifting_line.md), [VLM tests/reference](../data/reference/estimator/vlm.md), [T15b records](../data/reference/estimator/t15b_assertions.md)).

The inventories contain no independently separated Navion wing derivative. A wing-only prediction cannot be compared as though it were the published Navion aircraft total. The first experimental scope remains wing-alone, but TN 1270 is now parked at NO-GO with no further reconstruction; the owner has chosen the limited-evidence Stage 3 route above. (CR pp. 100-103; owner post-T16c answers; closure §§3/7.)

**Next action is the owner's response to the new Stage 3 planning questions in ROADMAP.md**, then explicit authorization of a named S3 task. The T16c source checkpoint answers are recorded, not asked again. No implementation, published-data comparison, T10, T11 or T17-T28 work starts from this documentation update. (Owner post-T16c request; ROADMAP, Questions before Stage 3; QUESTIONS.md, Stage 3 entry.)
