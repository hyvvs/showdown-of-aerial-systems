# Roadmap

**Current release: v0.1**. Anything not listed under the current release goes to the Parking lot at the bottom.
Effort figures are guesses at 20+ hours/week for a beginner, not predictions.

## v0.1: Subsonic fixed wing, designer, report, browser demo (about 4 to 6 months)

The hero demo is an **F-104-class supersonic jet**, targeted for v0.2 (D14). v0.1 stays subsonic and displays the explicit warning **"Mach > 0.8 not modeled"**.

### Stage 0: Learn and set up (3 to 4 weeks)
- Tasks: H1, H2, T0, T1.
- Exit: CI is green; the owner can explain lift, trim, and static margin in their own words; conventions are written.

### Stage 1: Simulation core in Rust
- Software tasks: T2 to T9b; their analytic/synthetic evidence is recorded in
  TASKS.md. Published-reference comparison T10 is **parked**, with its earlier
  performance goals retained, not claimed complete. Its source/model/D9 gates
  do not block the proposed Stage 3 designer/report work. (Owner's post-T16c
  answers and consequences; TASKS §§T2-T10; ESTIMATOR_PLAN, Current scope.)

### Stage 2: Subsonic aerodynamic estimator
- Owner-authorized start: T12 wing planform geometry, then T13 straight-wing
  Prandtl/Glauert lifting-line theory in a new soas-estimator crate. It depends
  on soas-core; the core does not depend on the estimator. This batch does not
  start T10 or T11 or satisfy their outstanding reference-validation work.
  Analytic/synthetic checks establish software evidence only; outputs list
  assumptions and mark physical validity unassessed, without invented thresholds.
- Lifting-line theory first (it has analytic test cases), then a vortex-lattice method with a compressibility correction.
- Current authorized extension: T14 unit horseshoe influence coefficients, then
  T15 planar vortex-lattice solver with mesh-convergence and moment/neutral-point
  checks. Classical lifting line assumes a straight wing of high aspect ratio;
  the hero aircraft's low-aspect-ratio swept wing is very different, motivating
  this panel method. These tasks stay incompressible/attached/small-angle and do
  not yet add the later compressibility correction or validate hero-wing results.
  Mark those results "validity unassessed" until compared with data. No T10/T11.
- Owner checkpoint answers are recorded: **park TN 1270**, retaining **NO-GO
  for a fully sourced experimental comparison** and the exact-Mach,
  continuous spanwise-prescription and matched section-input gaps. No further
  source reconstruction. Defer wing-tail searches; defer F-104 searching to
  v0.2; park R&M 2882 and RM L52G18. (Owner post-T16c answers 2-5;
  T16C_TN1270_CLOSURE.md §§3/7; parallel-track register below.)
- **v0.1 validation status:** "code-to-code cross-checked and analytic-case verified; not experimentally validated".
  The validation page and every designer HTML report must display that exact
  wording plainly, together with the actual evidence and its scope. It does
  not imply an AVL/OpenVSP run or experimental agreement. T10 and the entire
  T16-T28 chain are parked; completed audit evidence is retained. Their
  reopening conditions are in ESTIMATOR_PLAN.md, Parked work register.
  (Owner post-T16c consequences; T15b records; T16c §6.)
- Next up is **Stage 3 planning**, followed by only the task(s) the owner
  authorizes after the questions below. Experimental validation remains a
  parallel track, not a hidden Stage 3 prerequisite. Flying-wing validation
  stays after v0.2. (Owner post-T16c consequences; Questions before Stage 3.)

### Stage 3: Designer model and report
- Parametric component model, explicit mass/loading and supplied aerodynamic
  data feed isolated-wing analysis, a simulator export when complete, and an
  HTML engineering report. See the **S3-01 to S3-09 proposal below**; these
  new IDs preserve the parked T16-T28 IDs. No implementation is authorized by
  this documentation change. (Owner post-T16c planning request.)
- Proposed exit: independently checked synthetic end-to-end cases, reproducible
  export/report, and honest unavailable/conditional results after geometry or
  loading changes. The previous wingspan/stall-speed demonstration is only
  meaningful with an explicitly held-fixed CLmax assumption or newly supplied
  matching data; it cannot imply an estimated stall law. (S3-05/S3-08 below;
  estimator `vlm::ASSUMPTIONS`; NASA Lift Equation, Flow Conditions.)

### Stage 4: Browser demo
- Sim core compiled to WASM; sliders; live "does it fly" report; fly with keyboard; time-boxed "graphics-lite" (decent sky, lighting, simple ground).
- Exit: anyone can open a link and use it.

### Stage 5: v0.1 launch
- README with GIF, a clearly labeled example gallery, validation/evidence page
  and contribution guide. The page must use the exact v0.1 validation-status
  sentence above and distinguish synthetic/code comparisons from unavailable
  experimental validation. A real power-off reference example remains subject
  to the parked source/model gates. (Owner post-T16c consequences; T10.)

## Experimental validation: parallel track, parked candidates

Revisit at v0.2 for supersonic data, or when a suitable wing-alone or wing-tail
dataset is found. This is a review trigger, not an instruction to keep searching
now. Any reopening needs a source-sufficiency audit, an explicit owner task and
the matched-model/terms/condition gates in ESTIMATOR_PLAN.md. No gap is filled
from memory and no existing audit is deleted. (Owner post-T16c answers 2-5 and
consequences; T16b §§2-9; T16c §3.)

| Candidate / disposition | Open gaps and reopening evidence |
|---|---|
| TN 1270, wing alone — **parked, NO-GO** | Exact finite-wing Mach remains unverified; continuous twist/section-transition prescriptions and independent matched section laws are partial. References/reuse are closed. Reopen only on genuinely new source evidence or a separately approved, explicitly assumption-dependent study; no further reconstruction is authorized now. (T16c §3; TN 1270 pp. 3-6/Table I/Figs. 1/3; TN 1269 p. 20/Table IV.) |
| TN 1378 / Report 678, wing-tail leads — **deferred** | TN 1378 needs actual Mach, full mounting-body geometry and matched section inputs; Report 678 additionally relies on external wing/body geometry and retains tunnel effects. A new clean, self-contained wing-tail dataset could replace these leads only after its own audit and owner selection. (T16b §4; TN 1378 pp. 4-7; Report 678 pp. 1-4.) |
| RM SL56H06, F-104 model — **v0.2 candidate** | Supersonic regime, incomplete detailed loft/profile definitions, configuration/propulsion matching and the conflicting Reynolds/Mach association must be resolved or explicitly bounded; compatible supersonic model required. (T16b §5; report pp. 2/4-5/Table I/Fig. 2.) |
| F-104 flight-data leads — **v0.2 candidates, identity qualifications retained** | Keep CR-96008's F-104 compilation and NASA MEMO 2-2-59H; the latter is F-104-type but its exact aircraft identity/configuration match remains unverified, and some damping quantities are combined/estimated. Keep MEMO 2-3-59L as flight-versus-tunnel methodology evidence, **not identified F-104 data**. Need matched variant/conditions, axes, CG/inertias, individual-versus-combined derivative definitions and propulsion state before reuse. No new search now. ([CR inventory](../data/reference/inventory/cr96008.md), F-104 section; [other sources](../data/reference/inventory/other_sources.md), Scope and MEMO sections; CR pp. 74-79; MEMO 2-2-59H pp. 2/5-10; MEMO 2-3-59L pp. 6-7/12-16.) |
| R&M 2882 / RM L52G18 — **both parked** | R&M needs report-specific reuse permission, actual Mach and a matched model/support dataset; L52G18 retains unremoved support loads, whose effect is unbounded here. Neither becomes an approved replacement or correction. (T16b §3 A2-A3; R&M cover/Part II §2; L52G18 p. 3.) |
| Navion aircraft total — **parked** | Matched variant, complete geometry/section laws, normalization/datum and configuration/power state remain incomplete; wing-only output cannot close that comparison. (T16_AUDIT §§1-4; CR pp. 100-103.) |

CRM-HL remains **excluded from the first comparison**, not an admitted clean-wing
candidate: deployed slats and missing high-lift physics fail that scope. No
Stage 3 feature silently reintroduces it. (Owner T16c Decisions; T16b §6;
ONERA guide §6.4.)

## Later releases (details written when reached)
- v0.2: Supersonic (Mach-regime modules, wave drag, turbojet/afterburner, low-confidence flags); F-104-class hero demo and T11's data-driven headless aircraft, blocked until H4 is complete.
- v0.3: VTOL (rotor model, transition, hover controller).
- v0.4: Import 3D models, real-life export, landing gear, structural/mass/cost estimates.
- v0.5: Cockpit and aircraft systems.
- v0.6: Graphics and terrain.
- v1.0+: Combat and multiplayer.

## Parking lot (ideas that are not in the current release)
- After v0.2: flying-wing validation against AVL/OpenVSP output.
- T10 and T16-T28: retained experimental/modeling chain, with completed audits
  and original broader plans preserved; see ESTIMATOR_PLAN.md, Parked work
  register and retained task sections. (Owner post-T16c consequences.)

## Stage 3 proposed task plan — owner answers required before execution

**Numbering and boundaries.** Use S3-01 onward, without renumbering any T task.
Sizes below mean relative implementation/review effort, not calendar promises.
Recommended architecture: a pure `soas-designer` library depends on `soas-core`
and `soas-estimator`; CLI owns disk I/O and HTML writing. Neither existing
library depends on the designer. This is a proposal awaiting question 1, not
a crate/dependency change. Only pre-approved dependencies; D4 units/angle
types, D11 inertia signs and the body/file datum remain binding. (Owner plan
request; AGENTS, purity/dependencies; CONVENTIONS, Units/Mass properties.)

**What exists today.** The estimator provides symmetric station geometry,
straight-wing LLT and **isolated planar-wing VLM** lift/induced drag, pitch
slopes and wing neutral point, under its stated ideal section/incompressible/
attached-flow assumptions. It has **no coupled tail or fuselage aerodynamic
model**, no whole-aircraft polar/control effectiveness, and no CLmax/stall
prediction. Tail/fuselage geometry in a designer is descriptive until supplied
data or separately authorized models exist. The simulator consumes supplied
tables; its trim/static-margin routines do not estimate those tables.
(Estimator `planform::WingPlanform`, `lifting_line::solve`, `vlm::ASSUMPTIONS`
and `VlmOutput`; core `aircraft::Aircraft`, `trim::solve_trim`/`static_margin`;
AIRCRAFT_FORMAT, Coefficient fields.)

**Proposed honest data contract.** Keep draft geometry usable with missing
data, but mark affected outputs **not estimated / unavailable**, never zero.
Recommend requiring a complete user-supplied whole-aircraft coefficient bundle
for core trim/static-margin checks and simulator export. Basic quantities and
the standalone CLmax check need only their own declared inputs, not an entire
simulator-ready bundle. The bundle must state tail, fuselage
and interaction coverage (or an explicit externally supplied omission
assumption), source/configuration, reference normalization/origin and condition
coverage. Label their treatment **included in supplied total; not estimated
or separately resolved here**, rather than claiming separate component loads.
A wing
prediction is displayed separately; never add it to a bundle that already
includes the wing. Per-component aerodynamic assembly would need a separately
agreed common-reference/downwash/interaction contract, not blind addition.
This route is proposed in question 2. (Owner Stage 3 honesty requirement;
ESTIMATOR_PLAN, Static prediction budget; T16c §§4-6.)

Geometry/control/engine edits must mark incompatible supplied data and derived
outputs stale until applicability is reconfirmed; no automatic rescaling of
missing coefficients. Loading edits recompute mass/CG, force/moment transfers
and dependent results. Provenance and confidence records stay in the designer
document/report, not as unknown fields injected into the strict simulator
schema. (Proposed S3 contract; AIRCRAFT_FORMAT, Fields/versions/named objects.)

**Evidence rules for every task.** Tests first; expected answers from hand
algebra, separately authored records or an independently stated analytic case,
never by calling the adapter/calculator being tested. Mutation checks below
use an uncommitted scratch copy. Existing tolerances stay fixed; future
numerical settings need a stated source/error budget before implementation.
Use labeled synthetic fixtures and ledger every input/setting; no real
aircraft/engine numbers are invented. Report validation status, provenance,
model limitations and input completeness separately: a converged solve is not
physical validation. (AGENTS, Tests first/Batch mode; owner planning request;
T15b records; T16c §6.)

### S3-01 — Semantic aircraft components — medium, proposed

- **Purpose:** represent what the aircraft contains, independently of the
  solver mesh or simulator table layout.
- **Inputs / outputs:** stable component identities; main-wing stations;
  horizontal/vertical tail geometry, placement and incidence; fuselage
  dimensions/cross-sections; engine instances referencing explicit thrust/
  fuel tables and application axes; control-surface parent, geometry, sign
  and travel limits. Output a validated in-memory model with a common body
  datum, SI quantities, citations/applicability and explicit missing-data
  states. Mass/loading and aerodynamic bundles are referenced, not inferred.
- **Independent tests:** manually written component trees/reference lists;
  mirrored geometry cases, distinct x/y/z coordinates, duplicate identities,
  dangling engine/control references, invalid/non-finite dimensions and legal
  geometry-only drafts. Verify exact structured errors and absence of invented
  masses/coefficients.
- **Mutations:** swap axes; mirror twice; ignore a missing parent; turn an
  absent input into zero; reverse engine force direction.
- **Risks:** descriptive geometry mistaken for aerodynamic capability;
  inconsistent datum or accidental component duplication. No aerodynamic
  tail/fuselage solver. (Proposed contract; CONVENTIONS; AIRCRAFT_FORMAT,
  Engine extension; planform `WingStation`.)

### S3-02 — Versioned designer document and loading — small, after S3-01

- **Purpose:** save the semantic model without confusing it with simulator JSON.
- **Inputs / outputs:** independently versioned designer JSON, local table
  references and source metadata; output parsed/validated model or clear
  field/path errors. Pure text conversion in the library; file access and
  relative-path resolution in CLI. Version numbers/schema choices must be
  agreed before tests; no silent migration or changed simulator format.
- **Independent tests:** hand-authored valid/invalid documents; round-trip
  comparison against separately written expected records; recursive discovery
  of every named object followed by positional-array substitution, including
  engine/control/provenance objects. Preserve legal arrays and unknown-field,
  duplicate-field, missing-reference and unsupported-version rejection.
- **Mutations:** accept an object-shaped array; drop source metadata; resolve
  paths against the process directory; ignore a version or unknown field.
- **Risks:** relative references breaking on relocation, schema drift and
  treating successful parsing as physical validity. (D3/D4; AIRCRAFT_FORMAT,
  named-object/version rules; S3-01 contract.)

### S3-03 — Component mass, CG and inertia buildup — medium, after S3-01

- **Purpose:** make loading traceable rather than estimating mass from an
  unverified material density or geometry rule.
- **Inputs / outputs:** explicit component masses, their CG positions, intrinsic
  signed inertia tensors/orientations where available, and named payload/fuel
  loading cases. Output total mass, body-datum CG and CG inertia, plus an
  inclusion ledger preventing engine/fuel double counting. Missing masses or
  intrinsic inertia remain missing; an explicitly chosen point-mass
  idealization must be flagged, never defaulted. Fuel is fixed per loading
  case during a run, matching current core assumptions.
- **Independent tests:** hand-sum asymmetric non-collinear masses; manually
  integrate a simple distributed mass; check intrinsic-tensor rotation and
  signed parallel-axis contributions, rigid translation and reordered inputs
  within a stated roundoff budget. Check incomplete/zero-total-mass cases and
  singular inertia blocking simulator export.
- **Mutations:** average positions without mass weighting; omit an engine;
  count fuel twice; use the file origin instead of component CG; lose an
  off-diagonal minus sign or rotate inertia the wrong way.
- **Risks:** unknown structure/payload distribution and point-mass singularity;
  no structural weight estimate or fuel-burn CG model. Sources: MIT L11 p. 1
  and L26 pp. 2-3/7-9 below; convert its positive products to D11 signed entries.
  (AIRCRAFT_FORMAT, inertia/Engine extension; question 3.)

### S3-04 — Model-to-isolated-wing estimator adapter — small, after S3-01

- **Purpose:** derive one traceable estimator view from the semantic wing.
- **Inputs / outputs:** selected wing, explicit section law/zero-lift/twist
  inputs, condition, mesh and reference point; output `WingPlanform` and
  separately labeled wing-only geometry/LLT/VLM results or unsupported-input
  errors. Preserve current symmetry/planarity and ideal-VLM-slope restrictions;
  tail/fuselage contributions are **not estimated**. No T17-T22 implementation.
- **Independent tests:** hand-derived rectangular/tapered area, MAC, station
  locations and moment arms with unequal dimensions; existing analytic elliptic
  case as a separately specified expectation; unsupported non-planar/non-ideal
  requests and semantic input edits invalidate cached results.
- **Mutations:** replace MAC with average chord; swap span/chord; drop twist;
  accept unsupported geometry; label wing neutral point as aircraft neutral
  point. Zero-lift/offset tests check values as well as slopes.
- **Risks:** section-input provenance and mapping coordinate origins; convergence
  evidence remains specific to its mesh/method. (Estimator APIs/assumptions;
  geometry/lifting_line/vlm reference records; T16c §5.)

### S3-05 — Supplied aircraft data and simulator export — medium, after S3-02/03/04

- **Purpose:** connect the same design/loading to a flyable file only when its
  complete aerodynamic/propulsion inputs are supplied and applicable.
- **Inputs / outputs:** S3 model, mass/CG/inertia, declared aerodynamic
  references, complete supplied whole-aircraft tables, control limits and
  referenced engine tables; output the existing **aircraft format 2** plus a
  provenance/coverage report. Call existing validation and JSON text APIs;
  CLI writes files. A missing tail/fuselage/control/drag/damping contribution
  blocks dependent whole-aircraft results/export, unless its treatment is
  explicitly supplied as part of the bundle; absence is never implicit zero.
- **Independent tests:** separately hand-authored expected aircraft JSON;
  distinct datums, coefficients and engine arms; correct mass/reference/control
  fields; old/new configuration mismatch; missing data fails with exact
  paths. Cross-check loads using hand forces and cross products, not another
  call through the exporter. Read exported text through the existing parser
  as an additional compatibility check, not the sole expected answer.
- **Mutations:** add wing lift twice; fill a missing derivative with zero;
  swap a control sign; shift a moment twice; detach an engine from its table;
  copy stale coefficients after changing geometry.
- **Risks:** a complete file can still contain poor supplied data. Do not
  collapse data completeness, model assumptions and validation into one flag.
  No new simulator schema or estimated whole-aircraft model. (AIRCRAFT_FORMAT,
  Fields/Coefficients/Dynamic derivatives/Engine extension; question 2.)

### S3-06 — Loading, thrust and domain checks — small, after S3-03/05

- **Purpose:** report basic design quantities and condition coverage.
- **Inputs / outputs:** explicit loading/condition/throttle, wing area,
  table-supported net thrust and an optional **user-supplied** allowable CG
  range. Output weight, wing loading W/S, thrust-to-weight T/W, CG position
  and range membership, table/atmosphere coverage and **"Mach > 0.8 not modeled"**.
  State whether T is scalar installed thrust or forward resultant; recommend
  forward body-x resultant for the design check, reporting both when inclined.
  Warn/refuse dependent aero checks above the ceiling; never infer climb or
  trim feasibility from T/W alone. No guessed CG bounds or warning thresholds.
- **Independent tests:** manual mass-times-gravity, division, unequal engine
  force-vector sums and boundary inclusivity; hand-selected in/out-of-range
  CG/table/Mach cases; missing declared CG range returns unavailable.
- **Mutations:** report kg/m² as N/m²; use scalar thrust as forward thrust;
  invert T/W; reverse a CG inequality; hide the Mach warning at its boundary.
- **Risks:** condition-dependent thrust, idle versus engine-off, and treating a
  designer constraint as a certified CG envelope. (Definitions/formulas below;
  core propulsion output; CONVENTIONS, Atmosphere; question 4.)

### S3-07 — Trim, static margin and sampled CG feasibility — medium, after S3-05/06

- **Purpose:** ask whether the supplied model balances and has restoring
  aerodynamic pitch tendency at the requested condition.
- **Inputs / outputs:** complete supplied aircraft, explicit speed/geometric
  altitude, powered-level or all-engines-off glide mode, solver options and
  optional owner-defined longitudinal CG samples/loading cases. Reuse
  `solve_trim`/`static_margin`; report residuals, controls/throttle, saturation,
  failures and local **aerodynamic control-fixed** static margin. Display the
  derived wing-only neutral point separately. The existing margin excludes
  propulsive derivatives; this is not powered/dynamic-stability certification.
  Report sampled CG feasibility, not an invented continuous safe envelope.
  A virtual CG sensitivity case must be distinguished from a physical
  component loading case and cannot be exported as one without matching mass
  properties. Each check needs only its declared inputs; missing aero does
  not suppress independent mass/wing-loading results.
- **Independent tests:** prescribe a synthetic linear lift/moment law and
  engine with an analytic equilibrium; hand-check forces and reference-to-CG
  moments, negative/zero/positive margin and CG shift; cases with inadequate
  thrust/control travel, a lift-cap stencil and deliberate solver failure.
  Expected roots/residuals cannot be obtained from the solver being tested.
- **Mutations:** return the last iterate as success; bypass control limits;
  call idle engine-off; reverse the CG arm; use a wing slope with an aircraft
  moment slope; describe a sampled interval as fully feasible.
- **Risks:** local non-convergence does not prove no possible trim; missing
  effects and finite-difference/lift-limit boundaries. No static-margin target
  or CG sampling settings are invented. (Core trim APIs; trim README,
  Static margin/Solver and failure contract; Caughey §3.1 p. 28/§3.4 pp. 41-42;
  question 4.)

### S3-08 — Conditional CLmax stall-speed check — small, after S3-03/05/06

- **Purpose:** calculate a transparent lift-limit speed without pretending
  that attached-flow VLM predicts stall.
- **Inputs / outputs:** weight, density at geometric altitude, the same
  reference area and an independently supplied **whole-aircraft CLmax** with
  configuration/condition coverage. For the declared constant-CLmax,
  unaccelerated level lift-balance approximation, report
  `Vs = sqrt(2*W/(rho*S*CLmax))` and its assumptions. Missing/mismatched CLmax
  means unavailable. If CLmax varies with Mach/Reynolds across the solution,
  do not silently freeze a table value; require a separately specified solve
  or mark unsupported. This is not a demonstrated trim/minimum controllable
  speed, stall dynamics or a wing-section clmax substitution.
- **Independent tests:** hand algebra with a labeled synthetic constant CLmax;
  analytic square-root scaling with weight/area/density/CLmax; invalid inputs,
  section-versus-aircraft normalization and coverage failures. An edited span
  cannot reuse CLmax silently; any held-fixed sensitivity is explicitly chosen.
- **Mutations:** omit the square root; use mass as weight or section clmax as
  aircraft CLmax; substitute induced-drag efficiency for CLmax; hide a stale
  configuration or out-of-range condition.
- **Risks:** supplied CLmax dominates usefulness; no error band is invented.
  Formula is algebra from NASA Lift Equation with L=W at the declared cap,
  not a new stall model. (Formula sources below; VLM `ASSUMPTIONS`; question 5.)

### S3-09 — Headless HTML report and validation page — medium, after S3-02/04/06/07/08

- **Purpose:** present inputs, results and limitations from the same design.
- **Inputs / outputs:** CLI-owned document/table paths and typed analysis
  records; a standalone HTML report and export when complete, plus the v0.1
  validation/evidence page. Include component/source/loading/condition tables,
  wing-only versus supplied-aircraft results, units, references, assumptions,
  solver residuals, stale/missing inputs and qualitative confidence flags.
  Show **"code-to-code cross-checked and analytic-case verified; not experimentally validated"**
  verbatim on both outputs, alongside the conceptual-use notice from VISION.
  No aggregate “safe to fly” verdict; export failure must not suppress a
  useful geometry-only report. CLI command spelling awaits implementation scope.
- **Independent tests:** hand-authored complete/partial/failed-result records,
  fixed expected text/units/disclaimer and HTML-escaped malicious labels;
  parsed report assertions for missing/stale data, source links, negative
  margin, trim failure and Mach warning. A separately written synthetic
  end-to-end expected record checks model-to-report/export agreement.
- **Mutations:** omit/alter the disclaimer; hide a missing tail contribution;
  claim an external/experimental comparison that never ran; print unavailable
  as zero or green success; omit escaping; show stale data as current.
- **Risks:** persuasive presentation overstating conditional outputs; no
  arbitrary confidence percentages or pass thresholds. Validate result content
  as well as rendering. (Owner disclaimer requirement; VISION Accuracy stance;
  evidence policy above; question 6.)

### Formula and evidence sources for the proposed work

- **Mass/CG:** Peraire/Widnall, MIT 16.07 **L11**, [Center of Mass, p. 1,
  eq. (1)](https://ocw.mit.edu/courses/16-07-dynamics-fall-2009/5b68bc0342763cf099422af5a075940a_MIT16_07F09_Lec11.pdf):
  total mass is the component sum; CG is the mass-weighted position sum divided
  by that total. **L26**, [The Inertia Tensor, pp. 2-3, Parallel Axis Theorem
  p. 7, Rotation of Axes pp. 8-9](https://ocw.mit.edu/courses/16-07-dynamics-fall-2009/dd277ec654440f4c2b5b07d6c286c3fd_MIT16_07F09_Lec26.pdf):
  rotate each intrinsic tensor to body axes and shift to assembled CG before
  summing. The notes call unsigned-integral off-diagonal quantities products;
  D11 stores their negatives as tensor entries. No model mass values are supplied.
- **Weight, ratios and lift limit:** W=m*g0 uses the existing sourced gravity
  in CONVENTIONS; W/S and T/W are the declared diagnostic definitions above.
  NASA Glenn, [Lift Equation, Lift Coefficient/Flow Conditions](https://www1.grc.nasa.gov/beginners-guide-to-aeronautics/lift-equation/)
  gives L=CL*rho*V²*S/2 and emphasizes condition dependence. S3-08 rearranges
  this equation under the explicitly stated assumptions; NASA does not supply
  this project's CLmax or a validation tolerance.
- **Trim/margin/CG:** [trim reference](../data/reference/trim/README.md), Sources
  and definitions, What trim means here and Static margin; [MIT 16.100,
  Equations of Aircraft Motion pp. 1-2](https://ocw.mit.edu/courses/16-100-aerodynamics-fall-2005/678fcdd7f52560f148826442b6885f3e_16100lecture8_cg.pdf);
  [Caughey](https://courses.cit.cornell.edu/mae5070/Caughey_2011_04.pdf),
  §3.1 p. 28 eqs. (3.20)-(3.21), §3.4 pp. 41-42. Existing static margin is
  `-dCm_CG/dalpha / dCL/dalpha`; neither static margin nor a sampled trim
  survey supplies all real CG-envelope limits.
- **Available software evidence:** [T15b production comparison](../data/reference/estimator/t15b_production_comparison.md),
  Measured comparison/Verification and limits; existing geometry/LLT/VLM
  analytic records. These support named verification/cross-check claims, not
  experimental validation, a universal tolerance or a whole-aircraft estimate.

### Questions before Stage 3 starts

These are proposed implementation choices, not reopened reference decisions.
No task starts until the owner answers the applicable questions and names the
first authorized task. (Owner post-T16c request; AGENTS, Batch mode.)

1. **Architecture and first task:** approve the S3 numbering and pure
   `soas-designer`/CLI boundary, then authorize **S3-01 only** (recommended),
   or request a different module boundary/task split before implementation?
2. **Aerodynamic data route:** require a complete supplied whole-aircraft
   bundle for core trim/static-margin checks and export, while allowing incomplete
   geometry-only
   drafts (recommended), or first specify per-component contribution assembly,
   including reference/downwash/interference accounting, as additional work?
3. **Mass/inertia scope:** use explicitly entered component masses/CG/intrinsic
   tensors and fixed loading cases, with incomplete inertia blocking export
   (recommended), or add a separate declared aggregate-inertia override mode?
   No density-based weight estimate or implicit point-mass default is proposed.
4. **Condition and CG policy:** require explicit analysis conditions, loading
   cases, CG bounds/samples and numerical solver settings, reporting separate
   availability/feasibility results without an aggregate safety verdict
   (recommended), or supply a different documented acceptance policy? No
   default CG range, margin target or new numerical tolerance is selected.
5. **Initial stall scope:** accept only an explicitly supplied constant
   whole-aircraft CLmax within its stated condition/configuration coverage
   for the first algebraic check (recommended), or separately specify a
   Mach/Reynolds-dependent implicit stall-speed solve first?
6. **First end-to-end evidence:** reuse/extend labeled synthetic inputs and
   show categorical provenance/missing-data/assumption/domain flags, with
   no confidence percentages (recommended), or nominate a fully sourced
   user-supplied example and its reporting requirements before coding?
