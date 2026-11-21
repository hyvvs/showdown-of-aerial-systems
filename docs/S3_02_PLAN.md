# S3-02: approved contract and proposed implementation tasks

**Status: S3-02a documentation APPROVED; implementation NOT AUTHORIZED.**
Originally proposed 2026-11-21 from main `72ac5bf`; updated with the owner's
Q1-Q8 answers and provenance-binding clarifications on the same date.
[DECISIONS D15-D22](DECISIONS.md) records those answers. Section 3 is the
S3-02a contract and inventory; sections 2 and 4 record staging and decisions.
S3-02b1/b2 and all subsequent implementation tasks remain PROPOSED until
individually authorized; S3-02e is DEFERRED, not dropped. No code, tests or
fixtures are created by this documentation task.

**Fix 11 clarification:** the later station-ID model checks path presence,
not attribution history. The OPEN binding gates in section 3.3.1 apply only
to the policy-dependent S3-02b2 binding tests. S3-02b1 can be separately
authorized with the limits recorded; no reader starts in this batch.

## 1. What S3-02 currently contains

The current queue calls S3-02 an independently versioned designer document
and loading task. Its written scope contains:

- Designer JSON distinct from the simulator aircraft and scenario formats.
- A version/schema agreement before tests; unsupported versions reject,
  without silent migration or changes to the simulator format.
- Pure conversion between JSON text and semantic data in soas-designer.
  File reading/writing and relative-path resolution belong to soas-cli.
- Local engine-table references, source metadata and reference validation.
  D19 explicitly stages this original scope: inline definitions/local IDs
  first; external files remain incomplete until S3-02e is done or the owner
  explicitly drops that scope.
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

## 2. Recorded split and proposed implementation tasks

**Recorded choice: split it.** Even without mass calculations or a gallery,
schema decisions, strict parsing, writing and filesystem behavior have
different failure modes. The automatic structural/kind rejection requirements
make the current "small" label optimistic for a single reviewable batch.

Only the S3-02a documentation contract is approved. All implementation child
IDs below are **PROPOSED, NOT AUTHORIZED**, except DEFERRED S3-02e.
The historical-attribution choice holds only the policy-dependent b2 binding
tests. References to b2 in other task dependencies retain the independently
specified structural/kind checks; they do not create a blanket hold on
unrelated reader/writer/template work. No task may claim the held tests pass.
Sizes describe relative implementation/review effort, not calendar promises.
Each task needs separate authorization, tests first, the normal gates and
recorded mutation evidence when implementation is requested. Proposed tests
are described here; none are written or run by this planning task. Future
test numbers/settings must be labelled synthetic and ledgered; numerical
tolerances and formula sources must be justified before implementation.

### S3-02a — Approved document contract — small, documentation only

- **Status:** approved documentation; the contract/inventories are section 3.
- **Purpose:** record the section 4 answers before serialization code.
- **Inputs → outputs:** owner answers, current semantic fields and recorded
  gates → an approved field/version/tag/error contract and fixture inventory.
- **Independent checks:** trace every current input/provenance field to its
  approved representation; distinguish optional values from mandatory kind
  identity; review a separately authored expected-record specification.
- **Mutation/review checks:** remove a provenance entry, allow an ambiguous
  kind or admit a computed-wing input in the field matrix; the
  requirements review must flag each omission. Executable checks are S3-02b1/b2/c.
- **Risks:** freezing an accidental Rust representation or adding fields for
  capabilities that do not exist. Does not approve later tasks automatically.

### S3-02b1 — Strict pure reader — PROPOSED, small/medium

- **Purpose:** turn text into a validated semantic draft without file I/O.
- **Inputs → outputs:** approved JSON text/contract → Design or a precise
  parse/shape/version/kind/semantic error for the self-contained first format.
  Depends on S3-02a. Preserve approved wire IDs/group provenance during pure
  conversion; never silently discard them to fit today's in-memory model.
- **Independent tests:** hand-authored documents versus separately authored
  records; every enum kind; missing draft data; all numeric provenance;
  representative named-object/closed-kind errors, unknown/duplicate fields
  (including duplicate provenance keys in raw text), version/discriminator
  errors, optional nulls and legitimate arrays. These direct tests do not
  replace the automatic walkers in S3-02b2.
- **Mutations:** accept struct sequences; case-fold/default a kind; ignore an
  unknown field/version; deserialize only a duplicate-collapsing map; infer
  meaning from a label; discard grouping/IDs while translating wire records.
- **Risks:** permissive struct decoding and diagnostics that report the wrong
  problem. Reuse existing core/CLI shape-test lessons, not their aircraft schema.
- **Acceptance limit:** parsing/current-model validation is not historical
  source verification. Preserve IDs, declarations and specified snapshots;
  report precisely which structural/consistency checks ran. OPEN section
  3.3.1 gates do not block b1. Do not invent digest fields, a length cap or
  source-authentication behavior; unknown fields still reject.

### S3-02b2 — Automatic walkers and provenance-edit checks — PROPOSED, small/medium

- **Purpose:** prove strict parsing and binding coverage without hand-listed
  object/kind/provenance paths. Split from b1 to make review manageable;
  neither task alone completes original S3-02b acceptance.
- **Inputs → outputs:** b1 reader, approved schema inventory and independent
  synthetic documents → automatic structural/kind/edit tests and scratch
  mutation evidence. Depends on S3-02b1; no new format or relaxed parser.
- **Independent tests:** discover every named object, including table cells
  and metadata; replace each with its values array. Discover every actual
  kind field; separately inject unknown/missing/wrong-case/numeric/duplicate
  declarations. Duplicates reach the parser as raw text. Keep legal arrays
  and all legitimate kinds as positive controls.
- **Binding tests (D18):** automatically add/remove/reorder array entries and
  rename paths/IDs. Reject stale, missing or orphan binding records; accept
  a semantically valid edit only with matching provenance. Distinguish
  ID-preserving record reorder, ordered-array validation and whole-array
  records as specified in section 3. No index-only reorder blind spot.
- **Mutations:** omit a discovered object/kind, collapse duplicate keys,
  bypass order validation, attach metadata by list position, ignore an
  orphan/missing binding, or detach a grouped value from its metadata.
- **Risks:** a generic JSON map erases duplicates; a path-only map cannot
  detect every equal-unit permutation. If the required automatic binding
  checks prove impractical, stop and report for owner reconsideration of
  value-plus-metadata layout; never claim completion by shrinking coverage.
- **Narrow hold:** only binding tests whose expected result needs the OPEN
  historical-attribution choice wait for that choice. Object/kind walkers
  and independent structural checks do not wait on it. Neither portion is
  authorized for implementation by this document.

### S3-02c — Pure writer and draft round trips — small

- **Purpose:** produce valid designer text without inventing absent data.
- **Inputs → outputs:** valid Design → specified JSON text, or an error before
  any output is handed to a file writer. Depends on S3-02a/b1/b2.
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
  policy → loaded/saved document or clear error; depends on S3-02b1/b2/c.
- **Independent tests:** invoke the CLI on hand-written files; check exact
  semantic contents and failures. Run from another working directory; use
  paths with spaces; verify rejection leaves an existing target unchanged
  and does not create a new target. Set TEMP/TMP inside the checkout.
- **Mutations:** write before validation; silently overwrite; resolve a local
  reference against the process directory; treat an unreadable file as empty.
- **Risks:** partial writes and platform-specific replacement behavior.
  D20 requires validation, explicit replacement permission, temporary file
  then rename/replace, and an unchanged original on failure. Do not emulate
  replacement by deleting the original first. Command spelling belongs to
  separately authorized CLI work; no new third-party CLI dependency.

### S3-02e — External engine-table files — DEFERRED, small/medium

- **Purpose:** fulfil the explicitly staged external/local-file scope after
  the initially self-contained format (D19). Deferred does not mean dropped.
- **Inputs → outputs:** an approved external-table contract, document path
  and explicit table references → resolved tables and validated ID links.
  Preserve original file bindings/source information for subsequent saves.
  CLI owns reads/resolution; pure parsing remains in libraries.
- **Independent tests:** separate hand-authored design/table files relocated
  together; document-relative resolution; missing/duplicate table IDs, wrong
  table kind/version, broken references and the approved path boundaries.
- **Mutations:** resolve from the working directory, select the wrong table,
  bypass path policy, or use a stale table when the referenced file changes.
- **Risks:** standalone table schema/version, path boundaries and unresolved
  states require a later contract. Original external-reference scope is not
  complete until this task is done or explicitly dropped by the owner.

### S3-02f — Synthetic templates and headless gallery registry — small companion

- **Purpose:** provide discoverable, honestly labelled examples after the
  codec works, without starting a browser gallery or reporting UI.
- **Inputs → outputs:** approved neutral names, synthetic drafts and ledgers
  → a registry of stable IDs, labels, document locations, sources and explicit
  completeness/capability information. Depends on S3-02b1/b2/c; D21 selects
  Synthetic wing study and Synthetic aircraft study, possibly incomplete.
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
Review sequence: S3-02a documentation is approved; authorize one codec/I/O child
at a time. Templates and S3-03a can follow independently once authorized;
external references remain deferred and loading extensions need their later
contract/approval. The existing
S3-04 estimator adapter, S3-05 assembly/export and S3-09 report stay separate.

## 3. S3-02a approved document contract

This is the approved documentation deliverable, not an implemented format.
The structural inventory comes first so no array is assumed safe merely
because its entries have equal units. Rules M1-M3 below are the owner's
binding clarification, applied in that order:

- **M1, stable identity:** address a record by its ID and field, never its
  list position. Reordering the record list cannot reattach its metadata.
- **M2, strict order:** index paths are permitted when a fixed ordering rule
  makes a nonidentity permutation invalid. Exercise that claim in tests.
- **M3, aggregate:** one provenance record covers a whole equal-unit array,
  or an engine table axis/sample block. No per-cell table provenance. If a
  group record alone cannot detect the required separation/edit, also bind
  its full value/null snapshot; this is consistency checking, not evidence
  of a source's truth.

### 3.1 Exhaustive array and object inventory

IDs in this table mean explicitly declared stable identities, not display
labels; today's mutable in-memory model cannot establish their history.
"Owned" means a singleton named object inherits its parent ID/field address;
it does not need a new ID. Structural metadata is validated but does not
recursively require provenance about its own provenance. The tables enumerate
all distinct schema kinds, not a hand-maintained list of fixture paths for
the future automatic tests.

| Array kind / location | Stable ID? | Order validated? | Binding mechanism and reason |
|---|---|---|---|
| design.components | Each component id | No list order requirement | M1; component ID plus field, including engine instances. |
| design.engine_tables | Each definition id | No list order requirement | M1 for table identity; M2/M3 inside each table as below. |
| design.controls | Each control id | No list order requirement | M1, so a list reorder preserves the same hinge/limit sources. |
| design.contributions | Each contribution id | No list order requirement | M1, not list index. |
| contribution.parameters | Each parameter id, mandatory in this format | No list order requirement | M1, parent contribution ID plus parameter ID; name remains label-only. |
| surface.stations, every surface kind | Each station id, mandatory in this format | Current surface constraints still apply, but are not permutation-proof for every vertical surface | M1, parent surface/component ID plus station ID. Owner approved station IDs for all surfaces. No false M2 claim. |
| fuselage.sections | No | Strictly increasing aft_m | M2; a nonidentity permutation cannot remain strictly increasing. |
| table.mach_nodes | No | Strictly increasing | M2 order validation, with M3 one whole-axis provenance record; no per-node metadata. |
| table.geometric_altitude_nodes_m | No | Strictly increasing | M2 plus M3 whole-axis record; geometric metres. |
| table.throttle_nodes | No | Strictly increasing | M2 plus M3 whole-axis record; retain core endpoint/dry-node rules. |
| table.samples, altitude slices | Owned by table id | Dimensions tied to axes; values not ordered | M3, the whole sample block with snapshot; shape alone cannot detect slice permutation. |
| table.samples altitude slice, Mach rows | Owned by table id | Dimensions only | Same whole-block M3 record, not one per row. |
| table.samples Mach row, throttle cells | Owned by table id | Dimensions only | Same whole-block M3 record, never one per cell. |
| station.leading_edge_body_m | Owned by station id | Fixed x/y/z slots, not sorted | M3 one vector record with snapshot; swapping equal-unit coordinates can remain valid. |
| fuselage.nose_body_m | Owned by component id | Fixed x/y/z slots | M3 one vector record with snapshot. |
| mass.cg_body_m, when present | Owned by component id | Fixed x/y/z slots | M3 one vector record with snapshot, or null record when absent. |
| engine.position_body_m | Owned by component id | Fixed x/y/z slots | M3 one vector record with snapshot. |
| engine.direction_body_unit | Owned by component id | Unit-vector validation, not permutation-proof order | M3 one vector record with snapshot; norm alone cannot detect a swap. |
| control.hinge_start_body_m | Owned by control id | Fixed x/y/z slots; hinge checks still apply | M3 one vector record with snapshot. |
| control.hinge_end_body_m | Owned by control id | Fixed x/y/z slots; hinge checks still apply | M3 one vector record with snapshot. |
| intrinsic_inertia.tensor_kg_m2, outer rows | Owned by component id | Shape/symmetry/positive definiteness, not permutation-proof identity | M3 one full tensor record with snapshot, not one per row/entry. |
| intrinsic_inertia.tensor_kg_m2, each row | Same owner | Fixed axis slots | Covered by that same full tensor M3 record. |
| intrinsic_inertia.axes_euler_rad | Owned by component id | Fixed roll/pitch/yaw slots | M3 one angle-vector record with snapshot. |
| metadata.provenance_flags | Belongs to its map key | Existing declared flag order and uniqueness | Metadata within the bound record; no recursive numeric record. |
| metadata.source_refs | Belongs to its map key | Every string nonblank; no physical order | Whole metadata field within its bound record; no numeric entries. |
| metadata.input_paths | Belongs to its map key | References checked; not a physical sequence | Whole metadata field within its bound record; no numeric entries. |
| metadata.value_snapshot arrays, tensor rows and table sample levels | Same bound aggregate key | Same shape as described value | M3 comparison payload, not new design inputs or recursively provenanced numbers. |

| Named object kind | Own ID / owner | Order relevant? | Binding/shape rule and reason |
|---|---|---|---|
| Envelope | Fixed root | No | Closed named record; document_type/version gate, no physical numeric provenance for format_version. |
| Design | Fixed envelope.design | No | Closed container; child numeric inputs covered below. |
| Component | id | No | M1 owner of geometry/mass. |
| Geometry wrapper | Component id | No | Closed kind plus exactly one named payload; inherited M1 owner. |
| Surface payload | Component id | No | M1 scalar incidence; ID-keyed stations. |
| Station | id within surface | No object-field order | M1; ordered surface validation is separate from provenance identity. |
| Airfoil input | Station id plus airfoil field | No | M1 for scalar section inputs; null object has missing-input explanation. |
| Fuselage payload | Component id | No | M1 scalars, M2 sections and M3 nose vector. |
| Fuselage section | Component id plus strictly ordered index | aft_m order | M2 scalar fields, no new independent ID. |
| Engine instance payload | Component id | No | M1 table_id reference; M3 position/direction arrays. |
| Engine-table definition | id | No | M1 scalar dry_throttle_max/source; owned table. |
| Engine table | Definition id plus table field | Axis order | Whole-axis and sample-block M3 records, with M2 axis validation. |
| Engine sample cell | Table/block owner, not a cell ID | Fixed axis coordinates | Named object thrust_n/fuel_flow_kgps; covered by whole sample block, not per-cell provenance. |
| Mass input | Component id plus mass field | No | M1 scalar mass; M3 CG; named inertia or explicit null. |
| Intrinsic inertia | Component id plus mass.intrinsic_inertia | No | M3 tensor and orientation arrays; absent object has one missing-input record. |
| Control | id | No | M1 scalar limits/fraction, M3 hinge arrays; parent ID reference. |
| Supplied contribution | id | No | M1 parameters, closed scope/source/coverage. |
| Supplied parameter | id within contribution | No | M1 value; explicit closed kind, label-only name. |
| provenance map | Fixed design.provenance | No | Keys must exactly match the independently traversed binding inventory; reject missing, orphan and duplicate keys. |
| Provenance record | Its binding key | No | Closed record; scalar/group unit or declared mixed-group units, sources/flags/dependencies and any required aggregate snapshot. |
| Sample-block units object | Its sample-block metadata key | No | Closed thrust_n/fuel_flow_kgps unit map; mixed-unit block stays one record. |
| Missing-airfoil units object | Its null airfoil metadata key | No | Closed section-slope/zero-lift-angle unit map; no invented dimensionless unit. |
| Missing-inertia units object | Its null intrinsic_inertia metadata key | No | Closed tensor/orientation unit map covering the absent aggregate. |
| Snapshot sample-cell object | Its sample-block metadata key | No | Same closed cell shape as the described block, not an independent input/source. |

All data arrays are covered by M1, M2 or M3. Root/singleton objects and
metadata-only arrays are explicitly distinguished; inventing recursive
provenance for metadata would never terminate. The template registry is
S3-02f's separate future contract, not an undeclared array in this format.

**Verified ordering limit:** today's vertical-tail validator derives its
span direction from first/last stations, so reversing the whole list can
also reverse the direction and still validate. It would be wrong to claim
M2 protects every surface. The owner approved stable station IDs for all
surfaces instead. No new fixed-axis rule is invented for ventral/canted
surfaces. Sources: current
[validate_surface/span_direction](../crates/soas-designer/src/validation.rs),
[numeric traversal](../crates/soas-designer/src/numbers.rs), and the owner's
2026-11-21 clarification. This was a source inspection, not a test run.

### 3.2 Envelope, fields and canonical tags

JSON only; document_type is the exact string `soas-designer`;
format_version is the integer `0`, independently versioned and provisional.
Missing/wrong-type/wrong-discriminator/unsupported-version envelopes reject;
there is no implicit version, migration or simulator-format change.
Objects are named objects only; legitimate arrays remain arrays. Every
listed field is required to be present. Only fields explicitly marked
nullable below may be null. Empty collections are legal only where current
semantic validation permits them. Unknown/duplicate fields always reject.
Body-coordinate/direction vectors have three entries in x/y/z order;
axes_euler_rad has three entries in roll/pitch/yaw order; tensor_kg_m2 is
three rows of three signed entries. These are existing semantic dimensions,
not new test numbers. All inputs retain SI/radians and existing finite,
positive, direction-normalization, inertia and control checks unchanged;
the reader does not silently normalize, convert or repair a declaration.

| Object | Exact fields / representation |
|---|---|
| Envelope | document_type, format_version, design. |
| Design | name, identity, real_aircraft_claim (nullable, currently only null accepted), datum, components, engine_tables, controls, contributions, provenance. |
| Component | id, name, geometry, mass. |
| Geometry wrapper | kind and exactly the matching named payload: surface, fuselage or engine. No generic untyped data field or extra payload. |
| Surface | kind, mirrored, incidence_rad, stations. |
| Station | id, leading_edge_body_m, chord_m, twist_rad, airfoil (nullable). |
| Airfoil input | name, source, lift_curve_slope_per_rad (nullable), zero_lift_angle_rad (nullable). Defining inputs, not saved wing results. |
| Fuselage | nose_body_m, length_m, sections. |
| Fuselage section | aft_m, offset_y_m, offset_z_m, width_m, height_m. |
| Engine instance | table_id, position_body_m, direction_body_unit. Direction is force on the aircraft, not exhaust. |
| Engine-table definition | id, source, dry_throttle_max, table. |
| Engine table | mach_nodes, geometric_altitude_nodes_m, throttle_nodes, samples. samples nesting is altitude, then Mach, then throttle; dimensions match node counts. |
| Engine sample | thrust_n, fuel_flow_kgps, each a finite nonnegative scalar. |
| Mass input | mass_kg (nullable), cg_body_m (nullable), intrinsic_inertia (nullable). |
| Intrinsic inertia | tensor_kg_m2, axes_euler_rad. Signed tensor about component CG; orientation uses existing roll/pitch/yaw order. |
| Control | id, parent_surface_id, kind, hinge_start_body_m, hinge_end_body_m, chord_fraction, min_rad, max_rad. |
| Supplied contribution | id, scope, source, coverage, parameters. |
| Supplied parameter | id, kind, name, value (nullable), unit. name never selects kind/scope/unit/source. |
| provenance map | Binding path to closed provenance record; path grammar and aggregation below. |
| Provenance record | provenance_flags, source_refs, input_paths, geometry_revision (nullable), approximate; plus unit for scalars/homogeneous groups, or units for the declared mixed groups below. value_snapshot is required for the M3 vectors/tensor/sample block described below and absent elsewhere. |
| Sample-block units | thrust_n is N; fuel_flow_kgps is kg/s. No invented mixed-unit string. |
| Missing-airfoil units | lift_curve_slope_per_rad is 1/rad; zero_lift_angle_rad is rad. Only for the null aggregate record. |
| Missing-inertia units | tensor_kg_m2 is kg*m^2; axes_euler_rad is rad. Only for the null aggregate record. |

| Tag field | Exact allowed strings |
|---|---|
| design.identity | synthetic_template, user_design |
| geometry.kind | surface, fuselage, engine; the same-named payload is required |
| surface.kind | wing, horizontal_tail, vertical_tail |
| control.kind | elevator, aileron, rudder |
| contribution.scope | tail, fuselage, interference, cl_max; wing/whole_aircraft inputs reject |
| parameter.kind | Six supplied kinds in the table below; never a computed-wing kind |
| provenance_flags entries | Existing exact uppercase public labels; validity/order rules below |

| parameter.kind | Scope | Unit |
|---|---|---|
| tail_lift_curve_slope | tail | 1/rad |
| tail_neutral_point_contribution | tail | m |
| tail_pitching_moment_slope | tail | 1/rad |
| fuselage_pitching_moment_contribution | fuselage | 1 |
| interference_factor | interference | 1 |
| whole_aircraft_cl_max | cl_max | 1 |

All tags are exact and case-sensitive. No unknown/missing/numeric kind,
fallback, default, alias or label inference. Known computed-wing kinds also
reject as forbidden inputs. A label containing a wing quantity name is
legal when its declared kind is a permitted non-wing kind. IDs use the
existing nonempty ASCII letter/digit/hyphen/underscore rule. Existing global
component/table/control/contribution uniqueness is preserved; station and
parameter IDs are unique within their respective parent surface/contribution.
Names retain existing nonempty/uniqueness checks independently of IDs.

### 3.3 Provenance addresses, nulls and binding trade-offs

The map uses dotted field names and bracketed identities, relative to design
(without a design prefix). Stable IDs appear in brackets for components,
controls, engine_tables, contributions, stations and supplied parameters.
A field name is a schema field, never a free label. Only strictly ordered
fuselage sections retain zero-based integer index addresses. All other
bracket contents are the declared ID, with no label normalization. A
station/parameter ID is a required new defining identity; today's index/
parameter-label paths are not the approved file binding. The explicit
geometry wrapper/payload is reflected in the addresses:

| Binding family | Canonical address pattern (placeholders, not fixture values) |
|---|---|
| Surface defining fields | components[component-id].geometry.surface.incidence_rad; stations below that surface use stations[station-id].field, including airfoil.field or the absent airfoil aggregate |
| Fuselage defining fields | components[component-id].geometry.fuselage.field; sampled sections use sections[index].field under that payload |
| Engine instance defining fields | components[component-id].geometry.engine.position_body_m or direction_body_unit |
| Mass defining fields | components[component-id].mass.mass_kg, cg_body_m, or intrinsic_inertia.tensor_kg_m2 / axes_euler_rad; absent intrinsic_inertia uses its aggregate path |
| Control defining fields | controls[control-id].field, using whole hinge vectors and scalar fraction/limits |
| Supplied value | contributions[contribution-id].parameters[parameter-id].value |
| Engine definition scalar | engine_tables[table-id].dry_throttle_max |
| Engine table groups | engine_tables[table-id].table.mach_nodes, geometric_altitude_nodes_m, throttle_nodes or samples (each with the full table prefix) |

There is one canonical spelling per address, no alternative legacy spelling
or aliases. input_paths use exactly these binding keys. Source/label/string
fields do not acquire numeric records; renaming an ID updates all owning
keys and references, while changing label-only name does not rename a key.
Format/header metadata is not a physical input needing its own provenance.

Scalar fields have one record. Each homogeneous vector/tensor has one
record at the whole field, with its unit. Engine table axes each have one
record at the axis field, and samples has one record at the entire sample
block, with the closed units object. No per-element vector/tensor or per-node/
cell engine-table map entries. Every numeric member is still covered by
its owning record and displayed with that provenance in later reports.

An array-level source alone cannot detect a changed coordinate permutation
or rearranged sample block. Consequently M3 vectors, tensors, angle vectors
and the sample block also require value_snapshot: the complete same-shaped
value (or null for a nullable vector), compared by finite decoded f64 bits,
including signed zero, with no tolerance. It duplicates data for consistency,
not authority; the defining field is the input. Table axes use strict ordering plus exact
axis shape/coverage, without snapshots or per-node records. Renames/additions/
removals must update keys and dependency paths, leaving no orphan entries.

Declared optional scalars use null plus NOT_ESTIMATED and a nonblank
explanation in source_refs. A null aggregate (airfoil, CG or intrinsic inertia)
has one aggregate missing-input record, not fictional child numbers. Present
airfoil/inertia objects use their declared scalar/group records instead.
Missing mixed aggregates use only the exact units objects specified above;
unit and units are mutually exclusive. Scalar and homogeneous aggregate
units retain existing SI strings, including kg*m^2 for tensor entries.
real_aircraft_claim and nullable geometry_revision are nonnumeric policy/
metadata fields, not absent physical estimates needing recursive records.
Missing fields and missing kind identity are never treated as optional null.

Present defining inputs require USER_SUPPLIED or TEMPLATE_DEFAULT, optionally
OUTSIDE_COVERAGE. Absent inputs require NOT_ESTIMATED; they do not become zero.
ESTIMATED_WING_ONLY and DERIVED_FROM_INPUTS remain reserved for future result
records, not static defining-input overrides. Flags are unique and in the
existing declared order: ESTIMATED_WING_ONLY, USER_SUPPLIED, TEMPLATE_DEFAULT,
NOT_ESTIMATED, OUTSIDE_COVERAGE, DERIVED_FROM_INPUTS. Every source_refs list
is nonempty and every member is nonblank, including template values. input_paths reference
existing binding keys, never the same record. Preserve geometry_revision and
approximate, without implying any calculation has occurred.

Trade-offs are explicit: ID paths survive record reordering but require
unique/stable IDs and coordinated ID/reference renames; order validation
rejects accidental section/node permutations rather than guessing intent;
group records reduce map size but provide one source declaration for the
whole group. Distinct per-cell source histories must be preserved in the
table/block source references or the task must stop for a revised contract,
not silently invent per-cell metadata. Snapshots increase file size and
require hand-edited array values to have matching snapshot edits. They cannot
detect a deliberately coordinated false source or prove physical truth.

**Required automatic edit acceptance:** b2 discovers the array/binding
classes from the agreed schema. For each data array, test insertion,
deletion and permutation, plus field-path and ID renames where applicable.
An edit that separates values from their provenance must reject until the
records/keys/references/snapshots match; semantic order/dimension failures
still reject even after metadata changes. Pure ID-preserving list reorder is
a positive control: unchanged ID-keyed records already match, so no pointless
map edit is required and no source can move to another ID. A source is bound
to a named vector/tensor as a whole, never to a moving list index. Metadata-
only arrays receive their own flag/source/reference validity probes rather
than recursive provenance. If any required case proves impractical, stop
and report to the owner; do not weaken the check or choose a new layout.

### 3.3.1 Fix 11: path presence, consistency and OPEN historical-binding gates

**Current model behavior, demonstrated by tests:** each numeric path needs
valid metadata, with correct units/flags, nonblank source text and valid
references. There is no trusted previous ID-to-value/source assignment in
the model. Exchanging the existing first/last station IDs without moving
geometry can leave the same complete path set and still validate. Swapping
the two existing chord source records, both with unit m, can also validate.
Unique IDs/source strings and a complete map do not establish historical
binding. These are limits, not successful S3-02b2 binding checks. See
[Fix 11 evidence](S3_02_IDS2_VERIFICATION.md#fix-11--path-presence-and-historical-attribution-limits).

The following table accounts for every array category in section 3.1. File
mechanisms remain future requirements, not claims about the current model.
They detect specific internal inconsistencies; none alone establishes who
originally attributed a source to a value.

| Contract array category | Exact specified detection mechanism | What can still pass / historical limit |
|---|---|---|
| components, controls, engine_tables, contributions and contribution.parameters | M1 full ID plus field; uniqueness, reference resolution and exact key coverage. Whole-record reorder preserves the same mapping. M3 child snapshots may detect different numeric arrays under a swapped ID. | Exchanging already existing IDs can preserve all keys/references. Scalar data have no value snapshot; identical numeric payloads also give no snapshot discrepancy. No general historical detector is specified. |
| surface.stations, all kinds | M1 station ID under surface ID. Future M3 leading-edge-vector snapshot detects an ID-only swap with different coordinates and stale metadata, by comparing each ID's actual vector with its snapshot. | Today's model has no snapshots. Key coverage alone permits ID swaps. Updating snapshots together with a declaration, or moving source-only content, leaves no evidence of the previous assignment. ID spelling cannot choose a physical position. |
| fuselage.sections | M2 strictly increasing aft_m rejects a nonidentity permutation of section records; index-path coverage detects missing/orphan records. | Data can remain ordered while same-unit source records exchange. Order validation compares geometry, not source history; scalar records have no snapshot. |
| table.mach_nodes, geometric_altitude_nodes_m, throttle_nodes | M2 strict increase rejects node permutations; table dimensions/core bounds reject incompatible shapes. M3 supplies one record per axis, not per node. | There is no axis value snapshot in this contract. An ordered, dimension-consistent edit or same-unit source-only change is not historical verification. |
| table.samples and both nested slice/row arrays | M3 one whole sample-block snapshot compares every named cell/value, with axis dimensions and closed mixed units. Unequal-cell/slice permutation with a stale snapshot rejects. | Matching snapshots or a source-only edit still validate structurally. No per-cell source assignment exists; never invent per-cell provenance to claim history. |
| leading_edge_body_m, nose_body_m, cg_body_m, engine.position_body_m/direction_body_unit, both hinge vectors | M3 one complete vector/null record with exact value/null snapshot; fixed axis slots, shape and current geometric checks. Different-value permutations with stale snapshots reject. | Equal-value permutations change no value; source-only changes leave the snapshot valid. Whole-vector attribution does not encode separate coordinate histories. |
| tensor_kg_m2 outer array and each row | M3 one complete tensor snapshot plus shape/symmetry/positive-definiteness checks. A changed tensor with a stale snapshot rejects. | An unchanged tensor with different declared sources still matches; tensor validity cannot verify attribution. |
| axes_euler_rad | M3 complete angle-vector snapshot in declared roll/pitch/yaw slots. | Matching values/null do not identify which source should have been declared. |
| metadata.provenance_flags | Declared flag domain/order/uniqueness and input-presence rules. | A permitted origin flag is an author declaration, not evidence of historical ownership. |
| metadata.source_refs | Nonempty list, every source/explanation nonblank; part of its owner record. | Swapping valid source text only is invisible to value snapshots, order and key coverage. No source lookup/authentication is specified. |
| metadata.input_paths | Existing binding-key references and no self-reference. | Exchanging two valid dependency references can still resolve; coverage does not prove the author intended those dependencies. |
| metadata.value_snapshot arrays, tensor rows and sample levels | Compare the snapshot against the bound defining value, using the same shapes/closed cell objects. No recursive provenance about snapshots. | Coordinated edits still match; an unchanged value/null with a different source still matches. |

**OPEN gates for S3-02b2 (no mechanism chosen):**

| Gate | Plain-language synthetic example | Current/file checks that help | Uncovered case / decision needed |
|---|---|---|---|
| BIND-01: existing-ID reassignment beyond snapshot mismatch | Exchange two already existing IDs while leaving the records in place; all referenced IDs still exist. | Full-ID paths reject missing/new/orphan keys. A station's different coordinate vector will fail its future stale M3 snapshot. | M1 alone cannot detect reassignment. Scalar-only or identical-payload cases, and coordinated snapshot edits, need an explicit policy about history. Never infer it from names. |
| BIND-02: same-unit record reassignment | Exchange two existing chord metadata records; both still declare metres and valid source text. | Missing paths/wrong units reject. Where a whole record contains a snapshot, moving it to a different value rejects if that snapshot no longer matches. | Scalars currently have no snapshots; equal-value/null records still match. Mere source-record uniqueness is insufficient. |
| BIND-03: source-only/equal-value reassignment | Keep every value/null and snapshot, but exchange the two source declarations. | Empty text, invalid flags and malformed records reject. | No structural check can recover the original attribution from the final valid document. A matching value is not proof of its stated source. Owner must choose the policy/mechanism below. |

The last case is not a missing string comparison: the identical final
document could have been authored intentionally or produced by a source
swap. A validator sees the same bytes in both cases. Without trusted prior
evidence it cannot distinguish those histories. Inline fingerprints can
check internal consistency, but cannot independently certify attribution.

**Candidate mechanisms — options only, not decisions.** A hash is a content
fingerprint; to prove historical change it needs a trusted comparison value.
None of the candidates establishes that a cited experiment actually measured
the declared quantity. Full String identity remains required; a fingerprint
must never replace collision-safe identity equality.

| Candidate | Detects | Misses / dependencies | Hand-editing cost |
|---|---|---|---|
| (a) Hash of a provenance record's source text and bound value/null, stored with the record | Stale accidental edits if the stored fingerprint is not updated; complete record moves to unequal values can fail recomputation. | Same-value moves can still match if owner ID/path is not included. Including the owner would need approval. Anyone able to edit both content and an untrusted inline hash can recompute it. Algorithm, canonical representation, versioning and any dependency need approval; no authenticity claim. | Every covered value/source edit needs recomputation; an editing tool or an exact manual procedure is needed. |
| (b) Hashed or signed whole-document digest | A mismatch with a trusted external digest detects an edit. A signature requires a trusted signing key. Coverage could include all IDs, values and sources. | An editable digest stored only in the same document is recomputable. A whole-document check does not tell an intended correction from an error or prove source truth. Text versus canonical-content hashing, anchors/keys and dependencies are unsettled. | Recompute/re-sign after every covered edit; raw-text hashing also reacts to formatting. Key/trust management adds work. |
| (c) Accept and disclose author-declared attribution | Makes the claim honest while retaining all structural/unit/order/snapshot checks and human source review. | Deliberately accepts the historical-attribution limit; would require the owner's explicit adjustment of policy-dependent binding acceptance. It does not detect the source-only swap. | Lowest: normal edits plus visible source/assumption declarations; every report and format document must carry the limit. |
| (d) Unique source records per value/approved aggregate | Duplicate/reused source-record identities can be rejected; distinct visible content can help human review notice a swap. | Swapping two distinct valid records still preserves uniqueness. Need a trusted owner link to prove assignment. Numeric per-cell records would conflict with D18 table grouping and require another decision; never add them silently. | More records/labels and reference maintenance, even when many values legitimately cite one source. |

**Recommendation, not selection:** (c) for the first provisional format,
with explicit author-declared attribution, rather than implying historical
verification. If accidental-edit detection is also desired, consider (a)
as an additional limited check after its representation is approved. No
candidate is selected or implemented here. The owner's choice must close
the applicable BIND gates and specify the corresponding b2 assertions;
existing gates are not silently removed or weakened.

**Dependency boundary:** only policy-dependent S3-02b2 binding tests wait
for this choice. S3-02b1 strict reading and other unrelated work can be
separately authorized; object/kind walkers need not wait either. Reader
acceptance must state that source attribution is declared and not
machine-verified, retain structural checks and avoid unsupported history
claims. A later selected mechanism that changes fields needs explicit
version handling, not a permissive extension bucket. No reader starts now.

Until a mechanism is chosen, every report and the validation page must say
**"Source attribution is declared by the author and is not machine-verified."**
If a mechanism is later chosen/implemented, describe its exact scope and
remaining limits; a checksum/signature does not validate aircraft physics.
ROADMAP S3-09 carries this output and mutation-test requirement.

### 3.3.2 Proposed ID length cap — owner decision needed

**PROPOSED, NOT APPROVED:** at most 64 characters for station and component
IDs, retaining exact case-sensitive full-string comparison. Current IDs
are ASCII, so character and byte counts coincide. The purpose is to bound
path/error/report length and resource use for accidental extreme identifiers,
not to claim a physically meaningful or externally standardized threshold.
The value comes from the owner's Fix 11 proposal request.

Options: approve this cap; retain current uncapped length; or ask for a
different justified limit. Recommend the proposed cap for readability and
bounded paths, pending owner approval and a compatibility decision. There
is **no length validation code or boundary test added now**. Do not truncate,
hash, case-fold or rename existing IDs automatically. Other ID kinds and
free display-name/source text are not implicitly capped. This proposal
does not block unrelated work or b1 using today's declared ID rules.

### 3.3.3 Future type/fixture changes must compile together

For a future model change that alters a type used by fixtures, put the type
change and **all required fixture/construction-site migration in one commit**.
The commit message must explicitly say that it contains the type change and
fixture migration together. Every commit must build. Tests-first commits
must compile against their current tree; an intended failing assertion is
different from an uncompilable tree. If a buildable tests-first design is
not specified, stop and agree the workflow before committing a broken API.
Keep fixture numbers/assertions/signs/tolerances unchanged unless separately
authorized, and run the gates on the combined change. This is prospective;
the old station-ID commit history is retained. No shared type changes here.

This approved wire binding differs from today's in-memory per-element
traversal. Future b1 must preserve the station/parameter IDs and grouped
provenance through load/save, adapting semantic storage/validation explicitly
where necessary; rebuilding per-index metadata and discarding original
bindings is forbidden. Recording this contract alone does not authorize code.

**Later station-only authorization (2026-11-21):** S3-02-ids implements
required in-memory station IDs and ID-based station numeric paths, with an
identity-only fixture migration. This is a prerequisite model change, not
S3-02b1/b2 or codec work. Other wire changes above remain unimplemented;
see [station-ID evidence](S3_02_IDS_VERIFICATION.md).

### 3.4 Error contract and write boundary

| Error class | Required diagnostic / behavior |
|---|---|
| JSON syntax/type/shape | Identify offending field/path (root named explicitly); malformed text includes parser line/column where available. Named objects reject arrays, including engine cells, metadata and snapshot cell objects. |
| Envelope/version | Identify document_type or format_version and the unsupported/missing/wrong-type value; never migrate or assume a version. |
| Closed fields/tags | Identify unknown, missing, wrong-case or numeric tag/field. Duplicate keys identify the duplicated field, including dynamic provenance keys; raw text must reach the duplicate detector. |
| IDs/references | Identify missing/duplicate identity, dangling parent/table/input reference or renamed path with stale metadata. No label-based recovery. |
| Provenance/binding | Identify missing/orphan/duplicate key, wrong unit/flag/source, order violation or snapshot mismatch at the described field. No inferred USER_SUPPLIED/default or silent snapshot update while reading. |
| Semantic inputs | Retain specific field/reason for finite/positive/order/shape, scope/unit, tensor, control and source failures. Existing sign, hinge-span and unchanged numerical checks still apply. |
| Writer validation | Invalid/unknown/missing kinds, non-finite values, missing bindings or snapshots reject before text is offered to a file writer; never convert invalid numbers to null or create guessed metadata. |
| CLI I/O/replacement | Include the failing path/operation. Validate first; refuse an existing target without explicit replacement. Write a temporary file then rename/replace; any failure leaves the original unchanged, or the target absent if it did not exist. Never delete the original first. |

Error categories/path/reason are the contract; a particular Rust enum or
exact punctuation is not mandated. Pure text conversion stays in
soas-designer and filesystem access stays in soas-cli. Runtime declarations
for pre-approved serde/serde_json are later authorized work, not new
dependencies added by this document. No command spelling or external-table
path policy is approved prematurely.

A valid draft may omit supplied entries/values not required for draft saving.
This does not waive the S3-01d required-kind gate for any operation that writes
an assembled/exported design: define its required supplied-kind set before
implementation; remove each whole required entry from an independent complete
design and reject before write, leaving an absent/unchanged target. Bypassing
that check must fail its future test. Actual assembly/export remains S3-05.

### 3.5 Approved fixture and check inventory (future work only)

No numeric fixture or test is created here. Every future fixture value,
mutation setting and numerical test setting needs a labelled synthetic ledger
entry; expected records are independently authored, not generated by the
reader/writer under test. No real-aircraft data or threshold is invented.

| Fixture/check family | Required content / independently specified expectation | Task |
|---|---|---|
| Full semantic draft | Every object/array kind above, all surface/control kinds, inline engine tables, all six supplied kinds, distinct IDs, units, sources and group bindings; separately written expected semantic/metadata record. | b1 |
| Incomplete draft | Each declared nullable physical input absent in turn, retained NOT_ESTIMATED explanation; missing kind/ID remains an error; no fabricated zero or export-ready claim. | b1/c |
| Direct rejection set | Syntax, named-object arrays, all envelope/tag/unknown/duplicate/type failures, missing references/provenance, nonfinite/invalid semantic inputs and forbidden wing results. Duplicate cases are raw text. | b1 |
| Automatic object walker | Discover every object in the full document, including map records, units and snapshot objects, without hand-listed paths; replace each separately by its values array and reject; legal array controls remain accepted. | b2 |
| Automatic kind walker | Discover actual schema kind fields; unknown/missing/wrong-case/numeric/duplicate mutation individually for each; all legitimate kinds pass controls. Labels resembling kinds do not become discovered fields. | b2 |
| Automatic provenance edits | Discover all data-array classes and path/ID bindings; add/remove/reorder, rename paths/IDs, stale versus matching maps/snapshots, orphan/missing keys and ID-reorder positive controls. Strictly ordered section/node permutations reject by validation. | b2 |
| Binding edge cases | Whole vertical-tail station reversal with IDs cannot reattach metadata; grouped vector/tensor/sample-block permutations reject stale snapshots; engine axis/sample dimensions and named cell shapes remain checked. | b1/b2 |
| Writer and round trip | Separately specified fields/records plus read/write/read; every ID, source, null, unit, datum, grouped snapshot and reference retained. Invalid in-memory declarations cannot serialize as valid documents. | c |
| CLI transaction | Explicit temporary locations, invalid/new/existing targets, explicit replace versus refusal, staged-write and rename failure; original bytes unchanged on every failure. TEMP/TMP inside checkout. | d |
| Neutral template registry | Exactly the two registered synthetic names with reviewed drafts/ledgers, stable catalogue IDs and completeness flags; no real-aircraft claim or rendered gallery. | f |
| Deferred external references | Standalone version/path/resolution/binding contract and relocated multi-file cases, to be specified before that separately authorized task. | e, DEFERRED |

Formula/sign/unit and existing semantic-validation sources remain
[CONVENTIONS](CONVENTIONS.md),
[semantic model](S3_01_SEMANTIC_MODEL.md),
[current semantic types](../crates/soas-designer/src/lib.rs), and
[core engine table contract](AIRCRAFT_FORMAT.md).
The strict future gates are
[S3-01d, future acceptance conditions](S3_01D_VERIFICATION.md#future-acceptance-conditions-recorded-not-implemented).
The ID/aggregate/snapshot binding choices above come from the owner's
2026-11-21 answers/clarifications, not an implemented or tested schema.

## 4. Owner answers — ANSWERED, documentation only

| Question | Recorded owner answer | Decision / effect |
|---|---|---|
| Q1: first version | JSON, document_type, format_version starting at 0, explicitly provisional; reject unsupported versions; no silent migration. | D15; section 3 envelope. |
| Q2: variants/labels | Stable canonical strings, named payload objects, explicit kind on supplied parameters; name is label-only. | D16; section 3 tags/fields. |
| Q3: missing values | Explicit null in declared optional fields with required NOT_ESTIMATED provenance/explanation; missing kind identity always rejects. | D17; no guessed defaults. |
| Q4: provenance layout | Parallel numeric-path map with automatic add/remove/reorder and path/ID-rename rejection unless binding matches. Apply stable IDs first, strict order second, whole-array/table groups third, snapshot only where a group alone is insufficient. | D18; complete audit and tests in section 3. If impractical, stop/report for owner layout reconsideration. |
| Q5: table storage | Inline first; external-file S3-02e DEFERRED, not dropped. Original external-reference scope remains incomplete until e is done or explicitly dropped. | D19; original scope is explicitly staged. |
| Q6: saves | Refuse replacement unless explicit; validate first; temporary file then rename/replace; any failure leaves the original unchanged. | D20; no silent overwrite. |
| Q7: templates | The two registered neutral synthetic templates, possibly incomplete, with a headless registry. No real-aircraft template or rendered gallery. | D21; contents still require reviewed synthetic ledgers. |
| Q8: loading | S3-03a first on current static components; payload/fuel/named cases only in separately approved S3-03b with deliberate file-version extension. | D22; no fake fuselage geometry for payload. |

**Additional answered clarification:** the owner approved stable station IDs
for all surfaces after inspection showed whole vertical-tail list reversal
can still validate. Supplied parameters also have explicit stable IDs; free
labels do not act as identity. The snapshot-only recommendation was not
adopted globally: the owner's M1/M2/M3 hierarchy controls where snapshots
are necessary. The value-plus-metadata alternative is not selected.

**Reader split adopted:** S3-02b1 implements the strict reader with direct
rejection tests; S3-02b2 adds the automatic every-object/every-kind walkers
and binding-edit checks. This separates a parser from the tests that prove
schema-wide coverage without deleting any original acceptance condition.
Both remain PROPOSED, NOT AUTHORIZED, as do c/d/f and all S3-03 children;
e is DEFERRED. The owner approved only the S3-02a documentation contract.

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

The owner answers settle this documentation contract without completing
the parent queue. S3-02e retains the staged external-reference scope. Only
an individually named future task can authorize implementation; none starts
in this batch.
