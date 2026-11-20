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
- Stage 3 choices are recorded with the owner's conditions; **S3-01 is now
  completed**, limited to the pure semantic model/validation. All later S3
  tasks remain unstarted and need separate task
  authorization. Experimental validation remains a
  parallel track, not a hidden Stage 3 prerequisite. Flying-wing validation
  stays after v0.2. (Owner post-T16c consequences; Questions before Stage 3.)

### Stage 3: Designer model and report
- Parametric component model, explicit mass/loading and supplied aerodynamic
  data feed isolated-wing analysis, a simulator export when complete, and an
  HTML engineering report. See **S3-01 to S3-09 below**, plus the later
  approximate-inertia helper S3-10. These IDs preserve the parked T16-T28 IDs.
  S3-01 alone has been implemented and verified; S3-02 and the later analysis/
  export/report tasks are not started. (Owner Stage 3 adoption and S3-01 batch;
  S3_01_SEMANTIC_MODEL.md.)
- Acceptance: independently checked synthetic end-to-end cases, reproducible
  export/report, and honest unavailable/conditional results after geometry or
  loading changes. **The 11 m to 9 m wingspan demonstration is a required
  Stage 3 acceptance test**, with the controlled assumptions and independent
  expected relationships below. Stall results say **"conditional on supplied
  CLmax"**; flaps and high-lift devices are not modelled. Geometry changes
  affect this stall calculation only through area and weight, without an
  estimated change in CLmax. (Owner adoption, choice 5/acceptance instruction;
  S3-08 and Geometry-change acceptance test below.)

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

<a id="stage-3-proposed-task-plan--owner-answers-required-before-execution"></a>

## Stage 3 task plan — recorded owner decisions

**Numbering and boundaries.** Use S3-01 onward, without renumbering any T task.
Sizes below mean relative implementation/review effort, not calendar promises.
Adopted architecture: a pure `soas-designer` library depends on `soas-core`
and `soas-estimator`; CLI owns disk I/O and HTML writing. Neither existing
library depends on the designer. All six choices below are **ANSWERED**, with
the owner's conditions binding. **S3-01 only was authorized and is now
completed**; the later task plans below remain unstarted, not automatically
authorized. Only pre-approved dependencies; D4 units/angle
types, D11 inertia signs and the body/file datum remain binding. (Owner plan
adoption, choice 1; AGENTS, purity/dependencies; CONVENTIONS, Units/Mass properties.)

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

**Binding estimated-versus-supplied contract.** Incomplete drafts may be saved;
missing data stay unavailable, never implicit zero. Trim, static margin and
export need a **complete assembled aircraft bundle**. This supersedes the
earlier proposal to accept an opaque, entirely supplied aircraft total:
supplied/template aerodynamic values may cover only effects the estimator
cannot produce, specifically **tail, fuselage, interference and CLmax** under
the owner's stated allowance. They cannot replace a computable wing result.
Engine thrust/fuel tables and defining geometry, mass and loading inputs
remain explicit inputs under choices 1/3/4; this restriction concerns supplying
aerodynamic contributions, not those defining inputs. (Owner adoption,
choices 1-4; prior proposal in 94ae26f is superseded by this contract.)

| Quantity | Required origin / refresh behavior |
|---|---|
| Wing area, span/derived aspect ratio, MAC and other computable geometry | Recompute from current defining geometry; report `ESTIMATED_WING_ONLY`. No user/template override of a derived result. |
| Wing lift slope, induced drag, pitch outputs and wing neutral point available from the estimator | Fresh current-geometry evaluation at the declared condition, section assumptions, mesh and reference; report `ESTIMATED_WING_ONLY`. A wing neutral point is never labeled as an aircraft neutral point. |
| Tail, fuselage and interference aerodynamic contributions | Explicit supplied or template inputs, with separate identity, source/assumption, coverage and reference definitions; `USER_SUPPLIED` or `TEMPLATE_DEFAULT`. They are not estimated by today's wing solver. |
| Whole-aircraft CLmax | Supplied constant within declared configuration/condition coverage, `USER_SUPPLIED` or `TEMPLATE_DEFAULT`; never inferred from attached-flow VLM or a section clmax. |
| Aircraft totals, CG, weight/ratios, trim/static margin and conditional stall speed | Derived from current eligible inputs and computed wing outputs; `DERIVED_FROM_INPUTS` with dependency provenance. No aircraft-total flag claims a wing-only estimate covers the whole aircraft. |
| Missing or unsupported contributions / out-of-coverage requests | `NOT_ESTIMATED` and/or `OUTSIDE_COVERAGE`; dependent results/export remain unavailable. No guessed defaults or percentages. |

The origin/refresh rules are owner requirements; the available wing quantities
are listed by `planform::WingGeometry`, `vlm::VlmOutput` and LLT `solve`.
Coverage and reference normalization must be preserved when combining wing
and supplied non-wing contributions. **No opaque aircraft total containing a
static wing contribution may bypass this split.** S3-05 must specify the
assembly/moment-reference accounting before implementation; blind addition or
double counting is forbidden. Required simulator fields not covered by the
approved categories, such as an unresolved profile/control/rate term, stay
explicitly missing until separately specified; do not silently expand the
allowance or fill them with zero. This later S3-05 specification gate does
not block the authorized semantic-model task S3-01. (Owner choice 2;
AIRCRAFT_FORMAT, Coefficient fields; ESTIMATOR_PLAN, Static prediction budget.)

**Geometry changes always recompute everything the estimator can compute**,
including all quantities named above. Save defining geometry and permitted
supplied inputs, not authoritative static wing results. Report snapshots may
show computed values with their geometry revision, but cannot be loaded back
as defining inputs or reused for changed geometry. A fresh result may have
the same numeric value; freshness is required, not an invented requirement
that every quantity must change. Geometry/control/engine edits also recheck
supplied-contribution coverage; no automatic invention of missing physics.
Loading edits recompute mass/CG, force/moment transfers
and dependent results. Provenance and confidence records stay in the designer
document/report, not as unknown fields injected into the strict simulator
schema. (Owner choice 2; AIRCRAFT_FORMAT, Fields/versions/named objects.)

### Machine-readable number provenance

Every numeric input, result, array/tensor element and displayed numerical
setting must have metadata keyed by its stable numeric path, including a
nonempty, deterministically ordered **`provenance_flags`** list, unit, source
references and input/dependency paths. Computed wing records also identify
the geometry revision and applicable condition/mesh. This specifies a
machine-readable metadata contract, not a new Rust unit-wrapper type or an
implemented schema. The report displays the exact flag labels **beside each
value**, including template/derived values and all vector/tensor entries;
a page-wide disclaimer is insufficient. (Owner choice 6; D4; S3-01/S3-02/S3-09.)

| Flag | Meaning |
|---|---|
| `ESTIMATED_WING_ONLY` | Current-geometry wing estimator output, including derived wing geometry; never a whole-aircraft claim. |
| `USER_SUPPLIED` | Explicit user input, with source/applicability or declared assumption; no experimental-validation implication. |
| `TEMPLATE_DEFAULT` | Explicit template/policy input with its source or synthetic ledger, never a hidden wing-output override. |
| `NOT_ESTIMATED` | Missing/unsupported quantity; value is absent/null, never a fabricated numeric zero. |
| `OUTSIDE_COVERAGE` | Stated geometry/condition/data limits are exceeded. Preserve any input-origin flags, but do not use that value to produce an in-coverage result. |
| `DERIVED_FROM_INPUTS` | **One necessary additional flag:** mass/CG buildup, unit conversions, ratios, mixed aircraft totals, trim results and stall calculations are computed from inputs, rather than supplied or wing-only. Dependency paths retain all contributing origins; no confidence percentage is implied. |

Only that one additional provenance category is introduced. The later
simple-shape inertia helper also records **`approximate: true`**, with the
shape/distribution assumptions displayed beside its `DERIVED_FROM_INPUTS`
result: this qualifier is required because an ideal shape approximates an
actual component, not an additional claim of aerodynamic estimation. Missing
provenance is a validation error, not a reason to silently label a value user
supplied. (Owner choices 3/6; later S3-10.)

Templates and fixtures use **neutral names** and explicit synthetic labels;
no name, image, descriptive text or performance claim implies an F-104 or
another real aircraft until validated matching data exist. The parked real
source inventory is not a template-data licence or validation result.
(Owner choice 6; experimental parallel-track register.)

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

### S3-01 — Semantic aircraft components — medium, completed

Implementation/evidence: [S3_01_SEMANTIC_MODEL.md](S3_01_SEMANTIC_MODEL.md).
Sixteen unchanged semantic tests pass; all eight listed scratch faults fail
their targeted assertions; 193 workspace tests and fmt/Clippy pass. Missing
data remain draft inputs, not invented coefficients. No S3-02 or estimator/
export/report implementation is included. The requirements below are retained.

- **Purpose:** represent what the aircraft contains, independently of the
  solver mesh or simulator table layout.
- **Inputs / outputs:** stable component identities; main-wing stations;
  horizontal/vertical tail geometry, placement and incidence; fuselage
  dimensions/cross-sections; engine instances referencing explicit thrust/
  fuel tables and application axes; control-surface parent, geometry, sign
  and travel limits. Output a validated in-memory model with a common body
  datum, SI quantities, citations/applicability and explicit missing-data
  states, per-number provenance metadata and neutral template identities.
  Mass/loading and allowed non-wing aerodynamic inputs are referenced, not
  inferred. No authoritative stored wing-area/slope/drag/neutral-point numbers;
  those belong to later current-geometry computations. A missing intrinsic
  tensor is a valid draft state and blocks export in the later export task.
  Scope now: pure semantic model/validation, not the later adapters or report.
- **Independent tests:** manually written component trees/reference lists;
  mirrored geometry cases, distinct x/y/z coordinates, duplicate identities,
  dangling engine/control references, invalid/non-finite dimensions and legal
  geometry-only drafts. Verify exact structured errors and absence of invented
  masses/coefficients; every numeric path, including tensor elements, has
  provenance. Distinguish user/template inputs from derived wing results and
  reject a static supplied wing-result override. Keep missing inertia and
  missing non-wing contributions explicit in drafts.
- **Mutations:** swap axes; mirror twice; ignore a missing parent; turn an
  absent input into zero; reverse engine force direction; strip a numeric
  provenance flag; relabel a whole-aircraft total as wing-only; imply a real
  aircraft identity for a synthetic template.
- **Risks:** descriptive geometry mistaken for aerodynamic capability;
  inconsistent datum or accidental component duplication. No aerodynamic
  tail/fuselage solver. (Recorded owner choices; CONVENTIONS; AIRCRAFT_FORMAT,
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
  Saving an incomplete draft retains missing inertia; injecting a static
  wing-derived result or dropping any numeric-path provenance is rejected.
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
  idealization must be flagged, never defaulted. Missing intrinsic tensors
  are allowed in saved drafts but block export; no aggregate-inertia override
  is selected. Simple-shape inertia help is later S3-10, explicitly approximate.
  Fuel is fixed per loading case during a run, matching current core assumptions.
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
  tail/fuselage contributions are **not estimated**. Recompute every available
  wing quantity after geometry changes and attach `ESTIMATED_WING_ONLY`, input
  paths and current geometry revision. No supplied/template wing result is
  authoritative. No T17-T22 implementation.
- **Independent tests:** hand-derived rectangular/tapered area, MAC, station
  locations and moment arms with unequal dimensions; existing analytic elliptic
  case as a separately specified expectation; unsupported non-planar/non-ideal
  requests and semantic input edits invalidate cached results. The Stage 3
  11 m to 9 m acceptance case below must refresh all listed wing quantities,
  including quantities whose numeric value happens to remain unchanged.
- **Mutations:** replace MAC with average chord; swap span/chord; drop twist;
  accept unsupported geometry; label wing neutral point as aircraft neutral
  point; freeze an old wing slope, induced drag or neutral point after a
  geometry edit. Zero-lift/offset tests check values as well as slopes.
- **Risks:** section-input provenance and mapping coordinate origins; convergence
  evidence remains specific to its mesh/method. (Estimator APIs/assumptions;
  geometry/lifting_line/vlm reference records; T16c §5.)

### S3-05 — Assembled aircraft data and simulator export — medium, after S3-02/03/04

- **Purpose:** connect the same design/loading to a flyable file only when its
  complete wing-plus-allowed-supplied aerodynamic inputs and propulsion tables
  are available and applicable. An opaque supplied total cannot replace wing
  computations.
- **Inputs / outputs:** S3 model, mass/CG/inertia, declared aerodynamic
  references, freshly computed wing results, allowed tail/fuselage/interference
  inputs and supplied constant CLmax, control limits and
  referenced engine tables; output the existing **aircraft format 2** plus a
  provenance/coverage report. Each export is a newly generated snapshot of the
  current geometry and eligible inputs, never a static designer input that
  overrides wing computations. Call existing validation and JSON text APIs;
  CLI writes files. A missing tail/fuselage/control/drag/damping contribution
  blocks dependent whole-aircraft results/export, unless its treatment is
  explicitly supplied within the owner's allowed categories; absence is never
  implicit zero. Before implementation, inventory every required simulator
  coefficient and specify current-reference assembly and coverage handling.
  Any remaining field outside the allowance is a later-task specification
  question, not permission to add a static wing number or invent zero.
- **Independent tests:** separately hand-authored expected aircraft JSON;
  distinct datums, coefficients and engine arms; correct mass/reference/control
  fields; old/new configuration mismatch; missing data fails with exact
  paths. Cross-check loads using hand forces and cross products, not another
  call through the exporter. Read exported text through the existing parser
  as an additional compatibility check, not the sole expected answer.
- **Mutations:** add wing lift twice; fill a missing derivative with zero;
  swap a control sign; shift a moment twice; detach an engine from its table;
  copy stale coefficients after changing geometry; accept a static template
  wing slope/drag/neutral point; conceal a supplied non-wing origin flag.
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
- **Inputs / outputs:** complete assembled aircraft, explicit speed/geometric
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
  means unavailable. Every stall result is labeled **"conditional on supplied CLmax"**
  and states **"flaps and high-lift devices are not modelled"**. Within the
  declared constant-CLmax coverage and at fixed atmosphere, geometry edits
  change this calculation **only through area and weight**; no span/sweep/
  airfoil change in CLmax is estimated. If CLmax varies with Mach/Reynolds
  across the solution,
  do not silently freeze a table value; require a separately specified solve
  or mark unsupported. This is not a demonstrated trim/minimum controllable
  speed, stall dynamics or a wing-section clmax substitution.
- **Independent tests:** hand algebra with a labeled synthetic constant CLmax;
  analytic square-root scaling with weight/area/density/CLmax; invalid inputs,
  section-versus-aircraft normalization and coverage failures. An edited span
  must recheck the declared CLmax coverage; the controlled 11 m to 9 m
  acceptance test explicitly holds its supplied constant CLmax fixed and
  checks the independent area/weight scaling below. Both required notices and
  the adjacent provenance flag must be present on every stall result.
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
  as zero or green success; omit escaping; show stale data as current;
  remove a flag beside any numeric value or either mandatory stall notice;
  name or imply a synthetic template is a real aircraft.
- **Risks:** persuasive presentation overstating conditional outputs; no
  confidence percentages or pass thresholds. Every number displays its exact
  machine-readable provenance flag(s), including individual vector/tensor
  entries, rather than relying on a blanket source note. Validate result content
  as well as rendering. (Owner disclaimer requirement; VISION Accuracy stance;
  evidence policy above; question 6.)

### S3-10 — Approximate simple-shape inertia helper — later, small/medium

- **Purpose:** assist explicit component-inertia entry for a box, cylinder or
  ellipsoid, without claiming the actual mass distribution is known.
- **Inputs / outputs:** explicitly supplied mass, dimensions, declared
  solid/shell or other distribution assumption, orientation and component CG;
  output intrinsic tensor and its rotated/parallel-axis contribution to the
  assembled CG. Every result is `DERIVED_FROM_INPUTS` with
  `approximate: true`, shape/distribution assumptions and source references
  shown beside it. No geometry-to-mass or material-density guess.
- **Independent tests:** independently integrate each declared simple shape
  by hand; check unequal dimensions, tensor rotation, signed off-diagonal
  shifts and translation using the parallel-axis theorem. Verify the
  approximate qualifier survives the metadata/report path. Verify each
  shape formula against a cited primary reference before implementation;
  no unchecked formula is selected by this documentation task.
- **Mutations:** swap cylinder radius/length, use a shell formula for a solid,
  omit the parallel-axis shift or off-diagonal sign, or remove the approximate
  qualifier from a report result.
- **Risks / sequence:** ideal shapes may poorly represent actual internals;
  dimensions alone do not establish a mass distribution. Later than S3-03,
  using S3-09's reporting path; not authorized now and not required before
  S3-01 or the initial manually supplied-inertia acceptance case. (Owner
  choice 3; MIT L26, Parallel Axis Theorem/Rotation of Axes, below.)

### Geometry-change acceptance test: wingspan 11 m to 9 m

This is a **required Stage 3 acceptance test**, implemented in the later
S3-04/S3-08/S3-09 tasks, not in this documentation change. Use a neutrally
named, labeled synthetic straight planar wing. Hold chord distribution versus
normalized semispan, component masses/loading, atmosphere, supplied constant
whole-aircraft CLmax and other declared inputs fixed while scaling spanwise
stations from full span **11 m to 9 m**. The held-fixed CLmax assumption and
its declared coverage must include both synthetic cases; this does not assert
that a real aircraft's CLmax stays unchanged. No real-aircraft numbers or
identity are implied. Every future numeric fixture/setting gets a ledger
entry before implementation. (Owner acceptance requirement/choices 5-6.)

- **Independent expected geometry:** integrating the unchanged normalized
  chord distribution gives `S_new/S_old = 9/11`; with `AR = b²/S`,
  `AR_new/AR_old = 9/11`. These follow by algebra, not by asking the code
  under test for its expected answer. ([Geometry reference](../data/reference/estimator/geometry.md),
  Geometry contract and formulas; owner-supplied spans.)
- **Independent expected diagnostics:** weight remains fixed in this controlled
  test, so wing loading increases by `11/9`, and
  `Vs_new/Vs_old = sqrt(11/9)`. The report therefore shows higher conditional
  stall speed. In general, at fixed density and supplied CLmax, its geometry
  response is only `sqrt((W_new/W_old)*(S_old/S_new))`; no direct span/sweep/
  section correction is inserted. (S3-08's stated lift-balance formula;
  NASA Lift Equation below.)
- **Fresh wing analysis:** area, aspect ratio, lift slope, induced drag, wing
  neutral point and every other available wing quantity must be recomputed
  using the new geometry/condition/mesh, with `ESTIMATED_WING_ONLY` and the
  current geometry revision. Independent analytic/reference cases, with
  justified fixed comparison budgets, check numerical results in S3-04;
  freshness records also check that unchanged-valued outputs were reevaluated.
  Do not require neutral point or every other value to change solely because
  span changes. Include a separate hand reference-position/translation case
  that changes the neutral-point coordinate so a frozen-value mutation cannot
  hide behind an invariant pure-span result. (Owner choice 2; S3-04.)
- **Visible acceptance:** inputs and both geometry revisions are shown;
  provenance appears beside every number; each stall result carries
  **"conditional on supplied CLmax"** and **"flaps and high-lift devices are
  not modelled"**. State that the stall response to geometry is through area
  and weight only. Preserve the v0.1 validation disclaimer; no F-104/real
  aircraft template naming or experimental-validation implication.
- **Scratch mutations:** freeze wing geometry/aerodynamic outputs, load a
  static template wing slope, change CLmax silently, bypass a provenance flag
  or omit either stall notice; each must fail a corresponding acceptance
  check. Expected values/strings are separately authored, not generated by
  the report or estimator being tested. (Owner acceptance/flag requirements;
  AGENTS, Tests first.)

### Formula and evidence sources for the recorded plan

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

### Recorded owner answers and execution boundary

All six Stage 3 questions are **ANSWERED** by the owner's adoption with
conditions; no repeat approval is required for S3-01 after recording them.
(Owner Stage 3 adoption, choices 1-6.)

1. **Architecture:** pure `soas-designer`, CLI-owned I/O; S3-01 only authorized
   after this documentation commit. Later S3 tasks and parked T tasks do not
   start automatically.
2. **Aerodynamic origin:** complete assembled bundle for trim/static margin/
   export, incomplete drafts allowed. All computable wing quantities refresh
   from geometry; supplied/template aerodynamic values cover only the
   permitted non-estimated categories. No stored static wing outputs.
3. **Mass/inertia:** explicit component masses/CGs/intrinsic tensors; incomplete
   drafts may be saved, missing inertia blocks export. S3-10 is a later
   approximate simple-shape/parallel-axis helper, not an aggregate override.
4. **Conditions/CG:** explicit conditions, loading cases, CG bounds/samples and
   separate check results. No invented range, margin target or safety verdict.
5. **Stall:** supplied constant whole-aircraft CLmax within declared coverage;
   both required notices on every result, with geometry dependence only
   through area and weight. Required 11 m to 9 m acceptance test above.
6. **Evidence/provenance:** labeled synthetic examples, neutral templates,
   categorical flags beside every number, no confidence percentages. The one
   added provenance category `DERIVED_FROM_INPUTS` and later approximate
   qualifier are justified in the provenance contract above.

No unanswered choice blocked S3-01, now completed under its separate batch.
Detailed
coefficient/reference assembly and any required simulator terms outside the
allowed supplied categories are **later S3-05 specification gates**; they
are not silently settled here. The decisions-only commit was 81307f0;
S3-01's separate implementation does not start S3-02 or later tasks.
(Owner S3-01 batch; S3-05; AGENTS, task-specific stop condition.)
