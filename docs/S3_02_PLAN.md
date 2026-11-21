# S3-02 and adjacent work: proposed task split

**Status: PROPOSED — NOT AUTHORIZED.** Prepared 2026-11-21 from main
`72ac5bf`. This is a planning document, not an approved file specification,
implementation batch or change to recorded decisions. Recommendations below
need owner answers before dependent implementation. Existing conventions and
acceptance conditions remain binding. No code, tests or fixtures are created.

## 1. What S3-02 currently contains

The current queue calls S3-02 an independently versioned designer document
and loading task. Its written scope contains:

- Designer JSON distinct from the simulator aircraft and scenario formats.
- A version/schema agreement before tests; unsupported versions reject,
  without silent migration or changes to the simulator format.
- Pure conversion between JSON text and semantic data in soas-designer.
  File reading/writing and relative-path resolution belong to soas-cli.
- Local engine-table references, source metadata and reference validation.
- Hand-authored valid/invalid documents and round trips checked against
  separately written expected semantic records.
- An automatic walk discovering every nested named object, replacing each
  separately with an array of its values, and asserting rejection. Paths
  cannot be hand-listed. Legitimate lists, vectors, matrices and tables remain
  arrays; named records, including table cells, remain objects.
- Automatic discovery of schema-defined kind fields. Unknown, missing,
  wrong-case, numeric and duplicate kind fields must reject. Duplicate-key
  cases must reach the parser as raw text, not pass through a map that loses
  duplicates. Free labels do not identify a kind field or determine meaning.
- Unknown/duplicate fields, missing references and malformed data give clear
  field errors. Writer-side invalid or missing kind declarations reject too.
- Incomplete drafts preserve missing values/inertia and their provenance.
  Missing kind identity is invalid even when a numeric value is absent.
- Stored computed-wing overrides and lost numeric provenance are rejected.
- Any design-writing export enforces its declared required-kind contract
  before writing; that contract is different from ordinary draft validity.

Sources: [TASKS, Stage 3 and additional acceptance conditions](TASKS.md#stage-3-recorded-designerreport-queue-s3-01-completed),
[ROADMAP, S3-02](ROADMAP.md#s3-02--versioned-designer-document-and-loading--small-after-s3-01),
and [S3-01d, future acceptance conditions](S3_01D_VERIFICATION.md#future-acceptance-conditions-recorded-not-implemented).

Mass/CG/inertia calculations are currently **S3-03**, not S3-02. Neutral
template/gallery naming policy is already binding, but a template catalogue
and a rendered gallery are not an implemented S3-02 feature. Gallery/report
presentation belongs with later S3-09 work. This proposal does not quietly
move those features into the file parser or renumber the parent tasks.

## 2. Is it one batch? Proposed small tasks

**Recommendation: split it.** Even without mass calculations or a gallery,
schema decisions, strict parsing, writing and filesystem behavior have
different failure modes. The automatic structural/kind rejection requirements
make the current "small" label optimistic for a single reviewable batch.

All child IDs below are **proposed labels only**, not active queue entries.
Sizes describe relative implementation/review effort, not calendar promises.
Each task needs separate authorization, tests first, the normal gates and
recorded mutation evidence when implementation is requested. Proposed tests
are described here; none are written or run by this planning task. Future
test numbers/settings must be labelled synthetic and ledgered; numerical
tolerances and formula sources must be justified before implementation.

### S3-02a — Approve the document contract — small, documentation only

- **Purpose:** settle the questions in section 4 before serialization code.
- **Inputs → outputs:** owner answers, current semantic fields and recorded
  gates → an approved field/version/tag/error contract and fixture inventory.
- **Independent checks:** trace every current input/provenance field to its
  proposed representation; distinguish optional values from mandatory kind
  identity; review a separately authored expected-record specification.
- **Mutation/review checks:** remove a provenance entry, allow an ambiguous
  kind or admit a computed-wing input in the proposed field matrix; the
  requirements review must flag each omission. Executable checks are S3-02b/c.
- **Risks:** freezing an accidental Rust representation or adding fields for
  capabilities that do not exist. Does not approve later tasks automatically.

### S3-02b — Pure reader and automatic rejection tests — small/medium

- **Purpose:** turn text into a validated semantic draft without file I/O.
- **Inputs → outputs:** approved JSON text/contract → Design or a precise
  parse/shape/version/kind/semantic error for the recommended self-contained
  first format. If Q5 selects external files, define a separate unresolved
  document stage; do not claim a valid Design until references are resolved
  and semantic validation passes. Depends on S3-02a.
- **Independent tests:** hand-authored documents versus separately authored
  records; every enum kind; missing draft data; all numeric provenance;
  automatic every-object-to-array substitutions and every-kind-field invalid
  substitutions from section 1. Discover true fields through the schema, not
  labels. Preserve duplicate keys in raw text and retain legal array controls.
- **Mutations:** accept struct sequences; case-fold/default a kind; ignore an
  unknown field/version; deserialize only a duplicate-collapsing map; infer
  meaning from a label; skip a newly nested object in the automatic walk.
- **Risks:** permissive struct decoding and diagnostics that report the wrong
  problem. Reuse existing core/CLI shape-test lessons, not their aircraft schema.

### S3-02c — Pure writer and draft round trips — small

- **Purpose:** produce valid designer text without inventing absent data.
- **Inputs → outputs:** valid Design → specified JSON text, or an error before
  any output is handed to a file writer. Depends on S3-02a/b.
- **Independent tests:** compare with independently written expected fields
  and records, not just this writer read by this reader. Preserve every numeric
  field, source, flag, missing value, reference and datum. Reject invalid
  in-memory kinds and non-finite inputs; do not silently turn them into null.
- **Mutations:** drop source metadata, store a missing value as zero, omit a
  kind, serialize a computed-wing override, or discard tensor/angle entries.
- **Risks:** a round trip can hide a shared reader/writer mistake. Saving a
  valid draft is not proof that an aerodynamic or simulator export is ready.

### S3-02d — CLI document I/O — small

- **Purpose:** own disk access while the library stays pure.
- **Inputs → outputs:** explicit input/output paths and agreed overwrite
  policy → loaded/saved document or clear error; depends on S3-02b/c.
- **Independent tests:** invoke the CLI on hand-written files; check exact
  semantic contents and failures. Run from another working directory; use
  paths with spaces; verify rejection leaves an existing target unchanged
  and does not create a new target. Set TEMP/TMP inside the checkout.
- **Mutations:** write before validation; silently overwrite; resolve a local
  reference against the process directory; treat an unreadable file as empty.
- **Risks:** partial writes and relocation failures. Command spelling and
  write/replace behavior need approval; no new third-party CLI dependency.

### S3-02e — External engine-table files — conditional, small/medium

- **Purpose:** fulfil external/local-file references if selected in Q5;
  separate that work from an initially self-contained document.
- **Inputs → outputs:** an approved external-table contract, document path
  and explicit table references → resolved tables and validated ID links.
  Preserve original file bindings/source information for subsequent saves.
  CLI owns reads/resolution; pure parsing remains in libraries.
- **Independent tests:** separate hand-authored design/table files relocated
  together; document-relative resolution; missing/duplicate table IDs, wrong
  table kind/version, broken references and the approved path boundaries.
- **Mutations:** resolve from the working directory, select the wrong table,
  bypass path policy, or use a stale table when the referenced file changes.
- **Risks:** standalone table schema/version and unresolved-document states
  are not specified today. If Q5 defers this child, the owner must explicitly
  approve that staging of the current S3-02 reference scope; it is not dropped.

### S3-02f — Synthetic templates and headless gallery registry — small companion

- **Purpose:** provide discoverable, honestly labelled examples after the
  codec works, without starting a browser gallery or reporting UI.
- **Inputs → outputs:** approved neutral names, synthetic drafts and ledgers
  → a registry of stable IDs, labels, document locations, sources and explicit
  completeness/capability information. Depends on S3-02b/c and Q7.
- **Independent tests:** a reviewed expected catalogue, every registered file
  loading through the codec, missing/orphan/duplicate registrations, source
  and provenance checks, and every-object/kind rejection coverage on examples.
- **Mutations:** drop registry membership, relabel template values as user
  supplied, admit a static wing result, or imply a real-aircraft identity.
- **Risks:** synthetic examples looking validated. Use registry plus human
  review, not a supposedly complete denylist. User design names remain free.
  Actual gallery presentation and report disclaimers stay with S3-09.

### S3-03a — Component total mass and CG — small

- **Purpose:** calculate totals from explicit component inputs, not geometry
  guesses. This remains a child of S3-03, not a file-format feature.
- **Inputs → outputs:** existing component masses/CGs → total mass, common-
  datum CG, contributing-component ledger and explicit missing/error results.
  Depends on S3-01; it does not require a gallery or disk codec.
- **Independent tests:** hand-sum unequal, non-collinear component masses and
  positions. From the same complete case, remove exactly one mass: **both
  total mass and CG must become unavailable**, not a subtotal. Missing CG
  blocks CG while a total with all masses known may remain available. Check
  input reordering and translation against independent expectations; cover
  empty/zero-total cases and invalid component inputs explicitly.
- **Mutations:** replace missing mass with zero, omit an engine, average
  positions without mass weighting, or publish a partial sum as total mass.
- **Risks:** component/fuel double counting and misleading partial results.
  Missing inertia need not hide mass/CG, but still blocks simulator export.

### S3-03b — Named loading cases and inclusion ledger — small/medium

- **Purpose:** represent explicit payload/fuel changes with separate results.
- **Inputs → outputs:** approved mass-item/loading representation (Q8), named
  fixed cases and source data → selected component inputs and a ledger showing
  exactly what was included once. Depends on S3-03a and a schema agreement.
- **Independent tests:** hand-written complete/missing cases, payload/fuel
  removal and independent case totals; engine/fuel inclusion counted once.
- **Mutations:** carry values across cases, count fuel twice, silently omit a
  missing item, or default a missing case to a complete-looking result.
- **Risks:** the current component geometry enum has no payload/fuel-only
  variant. Specify a real representation and version change rather than fake
  fuselage geometry. Fuel stays constant during a run; no burn model is added.

### S3-03c — Intrinsic tensor axes — small

- **Purpose:** express supplied component inertia in the common body axes.
- **Inputs → outputs:** signed tensors and stated component-axis orientations
  → body-axis tensors or missing/error records. Uses the existing angle and
  D11 sign conventions; depends on an approved S3-03 contract.
- **Independent tests:** hand-computed axis rotations with unequal diagonal
  and off-diagonal entries, symmetry and invalid-tensor cases; no expected
  matrix obtained from the rotation helper under test.
- **Mutations:** transpose the rotation, swap axes, or lose an off-diagonal
  sign. Formula sources and a roundoff budget must be checked before coding.
- **Risks:** confusing tensor orientation with coordinate/datum shifts.

### S3-03d — Combined CG inertia and completeness — small/medium

- **Purpose:** complete S3-03 after totals, loading and axis rotation work.
- **Inputs → outputs:** eligible body-axis tensors, component masses/CGs and
  case CG → combined inertia about that CG, inclusion ledger and explicit
  missing/error status. Depends on S3-03a/c and loading rules where applicable.
- **Independent tests:** manually integrated/simple distributed-mass cases,
  signed parallel-axis shifts, reordered inputs, rigid translation and a
  single missing tensor. Known mass/CG must not turn missing inertia into zero.
  Check invalid/singular aggregate inertia is ineligible for simulator export;
  the actual export/write rejection is exercised in S3-05.
- **Mutations:** shift about the file origin, apply a shift twice, reverse a
  product sign, or treat absent intrinsic inertia as a zero tensor.
- **Risks:** an assumed point mass or ideal shape being mistaken for supplied
  distribution data. No silent point-mass fallback; approximate shape helpers
  remain S3-10. Actual required-kind/export writes stay S3-05 acceptance work.

The mass/rotation tests above inherit [ROADMAP S3-03](ROADMAP.md#s3-03--component-mass-cg-and-inertia-buildup--medium-after-s3-01)
and its cited mechanics sources; this plan introduces no formula or test
number. Completing S3-03a alone does not complete the original S3-03 task.
Suggested review sequence: approve S3-02a, then authorize one codec/I/O child
at a time. Templates and S3-03a can follow independently; optional external
references and loading extensions wait for their own answers. The existing
S3-04 estimator adapter, S3-05 assembly/export and S3-09 report stay separate.

## 3. Proposed format and field inventory

**Recommendation, not a decision:** retain the currently planned JSON family,
with an independent designer format_version and a document_type identifying
a designer document. It is not the simulator aircraft format_version. The
wire contract should be explicit rather than whatever representation a Rust
serialization derive happens to produce. Use the existing pre-approved serde /
serde_json stack; runtime dependency declarations belong to the authorized
codec task. No additional third-party library is needed by this proposal.
See Q1–Q5 before coding.

The proposed first format maps the current [semantic types](../crates/soas-designer/src/lib.rs),
not future solver results. These are proposed field names/groups, not JSON
code or a completed schema:

| Object/group | Proposed fields and meaning |
|---|---|
| Envelope | document_type, format_version, design. Version/discriminator agreement rejects wrong-format files and unsupported versions. |
| Design | name, identity, real_aircraft_claim, datum, components, engine_tables, controls, contributions, provenance. Current real-aircraft claims remain unsupported. |
| Component | id, name, geometry, mass. IDs remain stable references; display names do not classify a value. |
| Geometry | Explicit kind and named payload for surface, fuselage or engine; no positional struct encoding. Exact tags/layout await Q2. |
| Surface/stations | surface kind, mirrored, incidence_rad, stations; each station has leading_edge_body_m, chord_m, twist_rad and optional airfoil. Preserve declared order and common datum. |
| Airfoil input | name, source, optional lift_curve_slope_per_rad and zero_lift_angle_rad. These are defining section inputs, not stored computed wing results. |
| Fuselage | nose_body_m, length_m, sections; section aft_m, offset_y_m, offset_z_m, width_m, height_m. No inferred aerodynamic loft. |
| Engine instance | table_id, position_body_m, direction_body_unit: direction of force on the aircraft. |
| Engine table definition | id, source, dry_throttle_max, table. Table fields are mach_nodes, geometric_altitude_nodes_m, throttle_nodes, samples; cells retain thrust_n and fuel_flow_kgps and validated ordering. Inline/file storage and preserved external bindings await Q5. |
| Mass input | optional mass_kg, cg_body_m, intrinsic_inertia; present inertia includes signed tensor_kg_m2 and axes_euler_rad about the component CG. These are inputs, not calculated totals. |
| Control | id, parent_surface_id, kind, hinge_start_body_m, hinge_end_body_m, chord_fraction, min_rad, max_rad; preserve physical signs, order and span rules. |
| Supplied contribution | id, scope, source, coverage, parameters. Each parameter has mandatory closed kind, name (label only), value and unit. Map explicitly to the in-memory quantity identity. |
| Numeric provenance | Proposed parallel map keyed by numeric path; provenance_flags (mapped from the current Rust flags field), unit, source_refs, input_paths, geometry_revision, approximate. Preserve every vector/tensor/table entry and missing-input explanation. Q4 compares an alternative layout. |

Only the six approved supplied kinds are valid: tail_lift_curve_slope,
tail_neutral_point_contribution, tail_pitching_moment_slope,
fuselage_pitching_moment_contribution, interference_factor and
whole_aircraft_cl_max. Keep exact canonical case, permitted scope/unit and
source rules. Missing/unknown/computed kind declarations cannot be written
as valid inputs. A label containing a wing quantity remains just a label.
Source: [semantic model, closed kinds and completeness](S3_01_SEMANTIC_MODEL.md#closed-supplied-quantity-kinds-s3-01c2-fix-5).

Keep SI and unit-bearing field names from D4: angles in this proposed file
are radians; user-facing degrees stay at a tested boundary. Missing optional
values are proposed as explicit null with NOT_ESTIMATED explanations, not
zero/defaults. Kind identity is still mandatory. Lists/vectors/matrices stay
arrays; all named records stay objects. Unknown fields reject, including
invented wing_area, aggregate wing coefficients or saved authoritative solver
caches. Results belong in later reports, not defining design inputs.

Named loading cases, mass-only payload/fuel items, calculated totals, report
snapshots, gallery rendering, aerodynamic assembly and readiness policies
are not silently added to this first field inventory. Their tasks must
specify and version any needed extension; an arbitrary extensions object is
not a substitute for that agreement. Existing body axes/datum, D4/D6/D11,
all provenance flags and the incomplete-draft policy are already binding,
not new questions. Sources: [CONVENTIONS](CONVENTIONS.md),
[ROADMAP, estimated-versus-supplied contract](ROADMAP.md#stage-3-task-plan--recorded-owner-decisions),
and [S3-01d future gates](S3_01D_VERIFICATION.md#future-acceptance-conditions-recorded-not-implemented).

## 4. Owner questions before dependent tasks

These are unanswered proposals; nothing below is recorded as DECIDED.
Q1–Q5 are for the document contract before codec tests/implementation.
Q6 is needed before CLI writing, Q7 before templates, Q8 before loading-case
extensions. Later questions do not block unrelated pure mass/CG work once
that task has its own explicit authorization.

### Q1 — Initial independent designer version

- **Recommend:** JSON with document_type and format_version initially 0;
  explicitly provisional, with strict rejection of unsupported versions.
- **Alternative:** the same JSON contract starting at 1; a first supported
  release convention, but the owner must still agree compatibility rules.
- JSON is already the planned family. Switching to TOML/YAML would reopen
  that plan and dependency approval; it is not needed for the proposed split.
  Neither version number is selected by this document; no silent migrations.

### Q2 — How are variants and labels represented?

- **Recommend:** stable canonical string tags with named payload objects for
  geometry and explicit closed kind on supplied parameters; retain name as
  a label-only field. Define every tag/field spelling in S3-02a.
- **Alternative:** externally tagged Rust-style variants; less mapping work,
  but saved spelling/layout becomes more closely tied to Rust type names.
- Either choice must reject unknown/missing/wrong-case/numeric/duplicate
  kind fields and named-object arrays. No untagged or label-inferred fallback.

### Q3 — How are optional missing values stored?

- **Recommend:** explicit null in declared optional fields, with required
  NOT_ESTIMATED provenance/explanation; straightforward missing-data review.
- **Alternative:** omit optional fields and reconstruct absence explicitly;
  shorter files, but more care is needed to distinguish allowed absence from
  missing required fields. Missing kinds always reject under either option.

### Q4 — Where is per-number provenance stored?

- **Recommend:** the current parallel numeric-path map; smallest change to
  the in-memory contract and keeps defining SI fields readable. Renames,
  array edits and missing aggregate paths must be validated together.
- **Alternative:** a named value/metadata object around each number; more
  local metadata when hand editing, but a larger/different wire mapping and
  more nested records. Neither option adds unit-wrapper types or loses flags.

### Q5 — Inline or external engine tables first?

- **Recommend:** inline table definitions first, engine instances referring
  to local IDs; a self-contained movable document matching the current model.
  Explicitly defer external-file support to S3-02e with owner approval.
- **Alternative:** external references in the first format; matches the full
  current reference/path scope sooner but requires a standalone table
  contract, resolution stage and clear missing-file errors now.
- If external files are selected, also approve document-relative paths and
  their allowed boundaries before S3-02e; recommend portable relative paths
  with explicit rejection outside the approved root over machine-specific
  absolute paths. No process-working-directory resolution. Do not silently
  call the full original reference scope complete after implementing inline only.

### Q6 — What happens when a save target already exists?

- **Recommend:** refuse replacement unless explicitly requested, validate
  before writing and use a documented staged replacement so failure leaves
  the original intact; no silent destructive overwrite.
- **Alternative:** always create a separately named revision/backup; preserves
  history but adds naming, cleanup and failure-policy decisions. Any partial
  write or required-kind failure must leave an absent/unchanged final target.

### Q7 — What is the first template pack?

- **Recommend:** the currently registered neutral Synthetic wing study and
  Synthetic aircraft study as explicitly synthetic, possibly incomplete
  drafts, with reviewed fields/ledgers and a headless registry.
- **Alternative:** minimal blank skeletons only; fewer example values, but
  less useful demonstration of sources and component relationships.
- Neither option is a real-aircraft template, complete flight model, rendered
  gallery or permission to invent missing coefficients. Naming policy and
  free user-design names are already settled; they are not reopened here.

### Q8 — When should loading/mass-only items enter the schema?

- **Recommend:** first implement S3-03a on current static components, then
  separately approve explicit payload/fuel mass-item and named-case fields
  in S3-03b with a deliberate file-version extension.
- **Alternative:** specify those additions before the first designer schema;
  reduces an early revision but expands the first review and delays the codec.
- Current surface/fuselage/engine geometry must not be abused to represent
  payload. No missing component mass may count as zero under either choice;
  intrinsic inertia remains optional for drafts and required for export.

## 5. What can be claimed about a whole aircraft today?

Today the designer describes parts and checks declared inputs: references,
coordinates, units, kinds, sources, tensors and hinge rules. It can retain
an incomplete draft **in memory**. It does not yet save/load a designer
file, calculate aircraft mass/CG totals, connect the design to the wing
estimator, assemble a whole-aircraft model or export/report the design.
Passing these checks does not mean an aircraft can fly.

The separate estimator can analyse an isolated symmetric planar wing under
its stated incompressible, attached-flow and section assumptions. It cannot
supply coupled tail/downwash, fuselage/interference effects, whole-aircraft
control effectiveness or CLmax. A wing neutral point is not an aircraft
neutral point. Describing a tail does not estimate what that tail does.

The separate simulator has table-driven dynamics, engines and trim/static-
margin routines. Those routines consume a complete eligible aircraft dataset;
their existence does not make an incomplete designer draft flyable. Later
assembly must combine fresh geometry-derived wing outputs with only the
approved supplied non-wing contributions, preserving sources and references.

S3-03 totals will be conditional on supplied component data, not structural
weight predictions. Later stall results remain conditional on supplied
whole-aircraft CLmax, with flaps/high-lift devices unmodelled; a geometry edit
affects that stall calculation through area and weight, not estimated CLmax.
The required wingspan-change demonstration and Mach warning stay in their
existing later tasks. No physical-validity threshold is invented here.

The recorded v0.1 status remains **"code-to-code cross-checked and analytic-case verified; not experimentally validated"**. It must not imply that the new
designer or a complete aircraft has been experimentally validated. The
conceptual-estimate disclaimer, result-specific limitations and per-number
flags remain mandatory when reports arrive. No aggregate safe-to-fly claim,
confidence percentage or real-aircraft implication follows from these tasks.
The status phrase does not imply a particular AVL/OpenVSP run; actual
comparison evidence and its limits must be identified.
Sources: [ROADMAP, current capability and evidence policy](ROADMAP.md#stage-3-task-plan--recorded-owner-decisions),
[semantic-model scope](S3_01_SEMANTIC_MODEL.md#scope-and-evidence),
and [ROADMAP, S3-09](ROADMAP.md#s3-09--headless-html-report-and-validation-page--medium-after-s3-0204060708).

This plan leaves the parent queue and roadmap requirements intact. Review
the proposed split and questions first; authorize only a named next task
after its prerequisites are settled. No implementation is authorized here.
