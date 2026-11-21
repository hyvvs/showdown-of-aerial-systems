# Decisions log

Agents: do not make an open decision yourself. Present options with trade-offs and wait for the owner.

| ID | Decision | Status | Notes / recommendation |
|---|---|---|---|
| D1 | Project name | DECIDED | "Showdown of Aerial Systems". "aero" was only a placeholder. Repo slug: `showdown-of-aerial-systems`; crate prefix: `soas`; first crate: `soas-core`. |
| D2 | License | OPEN | Remains OPEN by owner choice. The repo stays private; no LICENSE file. Must be decided before the repo is made public. |
| D3 | Aircraft file format | DECIDED | JSON, with a `format_version` field and unit suffixes in field names. Required before T5; `serde_json` is pre-approved in AGENTS.md. |
| D4 | Units strategy | DECIDED | Required before T2; owner chose option B: SI internally, unit suffixes where ambiguous, and angle newtypes only for now. Full policy below. |
| D5 | Web viewer stack | DEFERRED | Decide at Stage 4. Options: Bevy (Rust-native, heavy on web) or a JS viewer (e.g. Three.js) on a WASM core. Do not decide earlier. |
| D6 | Control-surface sign conventions | DECIDED | Positive elevator: trailing edge down, nose-down moment; positive aileron: left trailing edge down and right trailing edge up, positive rolling moment; positive rudder: trailing edge left, nose-left yaw moment. Each surface needs a sign test. Textbook source and verification limits are in CONVENTIONS.md. Required before T6. |
| D7 | SolidCraft integration | DEFERRED | Plan: drive its CLI/command scripts, not its Rust internals; v0.4. Pin a version. Its project moves fast. |
| D8 | First supersonic validation aircraft | OPEN | Candidate: F-104, using [NASA CR-96008](https://ntrs.nasa.gov/citations/19690022405) and related NTRS reports. Remains OPEN until the owner completes H4; data completeness and suitability are not yet established. |
| D9 | Validation tolerances | OPEN | Proposal only: about +/-10% for stall speed. Owner confirms after seeing the reference data (task H3). |
| D10 | Default time step | DECIDED | Owner-selected default `1/400 s` (`0.0025 s`), always a parameter. T4's step-size convergence test may justify revising it; record any change as a new decision. |
| D11 | Inertia off-diagonal signs | DECIDED | Option A: store signed tensor entries about the CG in body axes. Provide a constructor from products of inertia that negates off-diagonal terms correctly, with tests. |
| D12 | Wind and stability axes | DECIDED | Option A: instantaneous stability and wind axes as defined in CONVENTIONS.md; wind-axis force components `(-drag, +side, -lift)`. |
| D13 | Near-zero airspeed | DECIDED | Option A: one named constant `MIN_AIRSPEED_FOR_ANGLES_MPS`, initially `0.1 m/s`, an owner-selected placeholder to revisit. Below it, alpha and beta are zero; otherwise alpha is `atan2(w, u)` and beta is `asin(clamp(v/V, -1, 1))`. Non-finite inputs are an explicit error, never silently clamped. |
| D14 | Hero aircraft | DECIDED | Supersonic jet, F-104-class. The hero demo is targeted for v0.2; v0.1 stays subsonic. |
| D15 | Designer document family and version (S3-02 Q1) | DECIDED | JSON with `document_type` and an independent `format_version`, starting at `0`, explicitly provisional. Unsupported versions reject; no silent migration. |
| D16 | Designer variant tags and labels (S3-02 Q2) | DECIDED | Stable canonical string tags with named payload objects. Supplied parameters have an explicit closed `kind`; `name` is label-only and never determines meaning, scope, units or provenance. |
| D17 | Missing optional designer values (S3-02 Q3) | DECIDED | Explicit `null` in declared optional fields, with required `NOT_ESTIMATED` provenance and an explanation. Missing kind identity always rejects. Draft validity does not imply export readiness. |
| D18 | Designer numeric provenance (S3-02 Q4) | DECIDED | Parallel map with value-bound provenance: stable record ID plus field first; strictly validated order/index paths second; whole-array or whole-table-axis/sample-block records third, with value/null snapshots only where grouping is insufficient. Stable station IDs on every surface are owner-approved; labels/list positions cannot become record identity. Retain automatic edit/rename checks for missing paths, stale snapshots and ID-preserving reordering; D23 defines author-declared attribution and the accepted historical limits. Full array/object audit and binding trade-offs: S3_02_PLAN section 3. |
| D19 | Designer engine-table storage/staging (S3-02 Q5) | DECIDED | Inline engine tables first, with local ID references. S3-02e is DEFERRED, not dropped. The original external-reference scope is explicitly staged and is not complete until S3-02e is done or the owner explicitly drops it. |
| D20 | Designer save/replace policy (S3-02 Q6) | DECIDED | Refuse replacement unless explicitly requested; validate before writing. Stage into a temporary file, then rename/replace, so any failure leaves the original unchanged. No silent overwrite; CLI owns file I/O. |
| D21 | First designer templates/registry (S3-02 Q7) | DECIDED | The two registered neutral templates, Synthetic wing study and Synthetic aircraft study, explicitly synthetic and possibly incomplete drafts, with a headless registry. No real-aircraft template or rendered gallery. |
| D22 | Designer mass/loading staging (S3-02 Q8) | DECIDED | S3-03a first on current static components. Payload/fuel items and named loading cases enter only in separately approved S3-03b with a deliberate file-version extension. No implementation is authorized by these decisions. |
| D23 | Designer source attribution | DECIDED | Owner chose option (c): source attribution is declared by the author and not machine-verified. BIND-01/02/03 attribution-policy gates are CLOSED; structurally valid history/source swap cases are ACCEPTED LIMITS, not defects. Exact notice below is mandatory in file-format documentation, every report and the validation page. Loading never requires a hash. Detectable consistency checks remain S3-02b2 requirements. |
| D24 | Designer identifier length | DECIDED | Maximum 64 characters for station IDs and all other designer-document identifiers, counted after ASCII-form validation. S3-02b1 must reject longer IDs with a specific error and test 63/64/65 characters. Apply the same model limit in that reader task, not retroactively; combine the type/model change and affected fixture migration in one explicitly named commit. S3-02b1 remains PROPOSED until separately authorized. |

## D4: Units strategy — option B

- SI units internally: metres, kilograms, seconds, newtons, pascals, kelvin, radians.
- Unit suffixes in names wherever a unit is ambiguous, e.g. `span_m`, `mass_kg`, `alpha_rad`.
- Newtype wrappers for angles only for now: `Radians` and `Degrees`, with explicit, tested conversions.
- Non-SI units (degrees, knots, feet) appear only at the UI/report boundary, converted by tested functions.
- Adding any further newtype, or the `uom` crate, requires the owner's approval.
- Revisit `uom` only if unit bugs keep recurring.

## D15-D22: S3-02 owner answers and authorization boundary

Source: the owner's answers to S3_02_PLAN Q1-Q8, 2026-11-21. The approved
field/version/tag/error/fixture contract is in
[S3_02_PLAN.md, section 3](S3_02_PLAN.md#3-s3-02a-approved-document-contract).
These decisions approve **S3-02a documentation only**. The reader is split
into S3-02b1 (strict reader) and S3-02b2 (automatic structural/kind walkers
and provenance-edit checks); both and every later implementation task remain
PROPOSED until individually authorized. S3-02e alone is DEFERRED, retaining
its original scope. No dependency, code, test or file format implementation
is approved by this documentation update.

For D18 the owner rejected a universal per-number snapshot as the first
answer. Apply the ID/order/aggregate hierarchy, with no per-cell engine-table
metadata. The owner subsequently approved station IDs on all surfaces
because current vertical-tail ordering is not permutation-proof. Grouped
vectors/tensors/sample blocks need snapshots where grouping alone cannot
detect the required edit; hand-edited values then require matching snapshot
edits. Reordering stable-ID lists keeps the same bindings and is a positive
control, not a forced metadata rewrite. The approved wire grouping/IDs are
not a claim that today's per-element in-memory validator implements them.

## D23: Author-declared source attribution — option (c)

Source: owner's documentation decisions, 2026-11-21. Exact user-facing
notice, required in the designer file-format documentation, every report
and the validation page:

**"Source attribution is declared by the author and is not machine-verified."**

- Existing-ID reassignment and same-unit/source-only swaps that still pass
  all specified structural and value/null snapshot checks are **ACCEPTED
  LIMITS**, not defects. BIND-01/02/03 are CLOSED as attribution-policy gates;
  no historical-source verification is claimed implemented.
- S3-02b2 must still detect missing/orphan paths, stale value/null snapshots,
  invalid ordered-array permutations and improper positional rebinding.
  Whole-record reordering retains ID-based ownership as a positive control.
- Loading never requires a hash, digest or signature. An optional separate
  content-hash validation command may be proposed later; no command, hash
  algorithm, dependency or implementation is authorized by this decision.
  A consistency check would not replace the attribution notice or validate
  the physical truth of a source.

## D24: All designer identifiers — approved cap, deferred implementation

Source: owner's documentation decisions, 2026-11-21. The cap is 64
characters of the already-validated ASCII identifier form. It applies to
all designer IDs and ID references, not only station/component IDs.
Full case-sensitive identity equality and the existing uniqueness scopes
remain binding; never truncate, case-fold, hash or rename an ID to fit.

S3-02b1's error must identify the overlong field/token and give the specific
reason **"identifier must not exceed 64 characters"**. Its independent
boundary tests must accept otherwise-valid 63- and 64-character IDs and
reject 65-character IDs. The same bound is added to the in-memory model
as part of the reader task; today's model is unchanged. Any affected type/
model change and fixture/construction-site migration belong in one commit
whose message explicitly says they are together. No separate retroactive
model-only change or current test/fixture modification is authorized.

These decisions close the two owner questions; they do not start the reader.
S3-02b1 and the remaining child tasks stay PROPOSED, not authorized.
