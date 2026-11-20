# Estimator plan: static derivative milestone

## Current scope and pre-modeling checkpoint — T16c

The owner's T16c request supersedes the earlier whole-aircraft-first sequence: the first experimental comparison is **wing-alone**, with **TN 1270 prepared conditionally**. RM SL56H06 is excluded from that comparison and retained as a candidate source for v0.2 because its tests are supersonic; CRM-HL is excluded because of deployed slats. These are owner decisions, not selections made by the audit. (Owner T16c request, Decisions so far; [T16b](T16B_REFERENCE_SEARCH.md) §§3/5-6; RM SL56H06 pp. 2-5; ONERA guide §6.4 p. 3.)

**STOP before modeling:** T16/T16b/T16c are documentation audits, not authorization for T17 or another model task. The [T16c closure](T16C_TN1270_CLOSURE.md) ends NO-GO for a fully sourced experimental comparison; its §§3-5 identify the remaining condition, spanwise-prescription and section-input gaps. The owner must answer the remaining questions in its §7 at this pre-modeling checkpoint. No assumptions are adopted by listing them. (Owner T16c request; closure §§1/3/5/7.)

For this NO-GO, the owner's v0.1 reporting category is **“code-to-code cross-checked, not experimentally validated”**, with only the actual existing cross-check evidence claimed. The next direction is Stage 3, subject to this checkpoint and a separately scoped task; it does not authorize implementation now. The closure's §6 states the required report/interface scope and evidence limits. If an experimental comparison is later reopened, stop again after its first lift-slope report before any pitch/neutral-point work. The earlier Navion whole-aircraft sequence below is retained as **deferred context**, not the current first-comparison requirement. (Owner T16c requirement 4; closure §§4/6-7; previous checkpoint in §T23 below; ROADMAP Stage 3.)

The static derivative milestone is separate from reshaped [T10](TASKS.md#t10-compare-against-reference-reshaped-and-blocked). T10 retains its power-off stall/glide/trim/static-margin goal and is blocked on this milestone as well as H3/D9. T11 stays separate. Completing the source audits does not complete that milestone, H3/H4 or D8/D9. No accuracy pass/fail claim is permitted before D9. (TASKS §§T10/T11/T16-T16c; DECISIONS §§D8-D9.)

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

No modeling task is executable under the T16c authorization. The following preserves a deferred static build sequence for later owner decisions; each implementation needs independent hand/analytic expectations first and mutation evidence. Rough sizes are relative, not time estimates. Incompressibility/propulsion disclosures do not turn the current wing result into a whole-aircraft estimate. All source gaps remain gates. (Owner T16c request; Current scope section above; AGENTS.md, Batch mode.)

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

## Later AVL cross-check and artifact policy

A version-pinned AVL cross-check is approved **in principle for later**, after license review, as secondary evidence. Record the actual release/hash, input provenance, axes/reference conventions and condition before use. Analytic/hand expectations and published reference data remain the primary evidence. No AVL run or generated result was produced in this update.

**License check performed for this documentation:** the official [Release Conditions](https://web.mit.edu/drela/Public/web/avl/) identify GPL licensing and link [GNU GPL version 2](https://web.mit.edu/drela/Public/web/gpl.txt). Its §0 makes output coverage depend on the output's contents; §§1-3 impose conditions on redistribution of covered material. This does not establish blanket permission for every file/result or a particular future release/input set. Review that pinned release's actual notices and the intended artifact/input licenses before committing anything derived from AVL.

Owner policy: do not commit AVL binaries or raw output unless the license clearly permits the particular artifact and all applicable conditions are satisfied. The preferred repository deliverable is our own summary tables with provenance after the license check. Unclear permission means leave the artifact uncommitted and ask. This is not a change to the project's open D2 license decision.

## What is already meaningful, and what happens next

Current synthetic verification can use the analytic elliptic LLT case, VLM's independent one-panel load/moment solution and hand symmetry/reference/limiting cases. These verify software within its idealizations, not the Navion. Existing finite-aspect-ratio VLM/LLT differences are not an equality oracle; preserve their evidence and tolerances ([LLT tests/reference](../data/reference/estimator/lifting_line.md), [VLM tests/reference](../data/reference/estimator/vlm.md), [T15b records](../data/reference/estimator/t15b_assertions.md)).

The inventories contain no independently separated Navion wing derivative. A wing-only prediction cannot be compared as though it were the published Navion aircraft total. The owner has instead selected a separate wing-alone first comparison, with TN 1270 conditional and presently NO-GO, as recorded in Current scope and T16c §§1/4 (CR pp. 100-103; owner T16c Decisions).

**Next action is the owner's pre-modeling checkpoint response**, covering the five restated T16b questions and their already-answered parts in T16c §7. No modeling, published-data comparison, T10 or T11 starts from this documentation update. (Owner T16c request; Current scope section above; QUESTIONS.md, T16c entry.)
