# Stable station IDs — SYNTHETIC TEST VALUES ONLY

This ledger covers every value/setting in tests/station_ids.rs. No physical
value is measured data, an aircraft default, or a real-aircraft claim. All
comparisons are exact; no tolerance or finite-difference setting is introduced.

| Value / setting | Synthetic purpose and independent expectation |
|---|---|
| ROOT [1,0,2] m | Geometric first station, ID tip deliberately means nothing. |
| WING_TIP [-2,3,4] m | Last station for wing/horizontal tail, ID root; distinct coordinates and positive spanwise progression. |
| DORSAL_TIP [-2,0,-4] m | Last station of dorsal fin, upward in body axes. |
| VENTRAL_TIP [-2,0,8] m | Last station of ventral fin, downward in body axes. |
| Mirrored tip [-2,-3,4] m | Hand reflection of the declared wing tip about body y=0; ROOT is on the plane and is not duplicated. Formula: CONVENTIONS, Coordinate frames; existing surface_points_body_m contract. |
| incidence 0.05 rad | Defining synthetic angle; no aerodynamic calculation. |
| First station chord/twist/section slope/zero-lift angle | 2.5 m, 0.125 rad, 6 per rad, -0.1 rad, all synthetic defining values. |
| Last station chord/twist/section slope/zero-lift angle | 1.25 m, -0.25 rad, 5 per rad, -0.2 rad; distinct from first station so positional rebinding is observable. |
| Absent mass, CG, inertia | None, not numerical zero; NOT_ESTIMATED with explanation. |
| Hinge endpoints | Exactly ROOT and the selected TIP, inclusive span boundaries. Direction is by list position, never ID spelling; existing owner-defined hinge rule in S3_01_SEMANTIC_MODEL, Span containment. |
| Control fraction/limits | 0.2, -0.3 rad, 0.4 rad, synthetic values reused from semantic_values.md. |
| Structural settings | First/last station slots 0/1; x/y/z slots 0/1/2, vector length 3; four cases: mirrored wing, horizontal tail, dorsal fin, ventral fin; two surfaces share the same station IDs. These are fixture addressing/count settings. |
| IDs and invalid probes | surface-a/surface-b, tip/root, control, other-id; duplicate tip; empty string, space, tab/newline and bad.id. No ID conveys geometric role. All are synthetic labels, not measured numbers. |
| Compile-fail example | Zero coordinates/angles, unit chord and three vector entries only to make every pre-existing Station field valid; omitting the required ID must fail compilation. The example is not an aircraft fixture or a numerical prediction. |

The expected path/value inventory is manually authored in expected(), never
obtained from Design::numbers. That inventory supplies test metadata only;
production traversal and validation remain the behavior under test. References
are the owner's stable-station-ID instructions, DECISIONS D18 and the approved
S3_02_PLAN section 3. No new physical formula is introduced.

## S3-02-ids2 Fix 10 — exact identity, synthetic labels only

All six pairs below are SYNTHETIC TEST LABELS, not real identifiers or
physical data. Every pair is accepted on each of the four existing surface
cases, retaining distinct full-ID numeric paths and source records. Expected
paths are remapped from the independently authored inventory, never obtained
from production traversal. Geometry, physical values, structural indices and
exact comparisons reuse the preceding ledger without change.

| Synthetic ID pair | Byte lengths | Unsigned ASCII byte sums | First characters | Deliberately weak comparisons that collide |
|---|---|---|---|---|
| A / a | 1 / 1 | 65 / 97 | A / a | ASCII case-folding; length |
| ab / ba | 2 / 2 | 195 / 195 | a / b | Byte sum; length |
| ac / ca | 2 / 2 | 196 / 196 | a / c | Byte sum; length |
| abc / cba | 3 / 3 | 294 / 294 | a / c | Byte sum; length |
| aa / az | 2 / 2 | 194 / 219 | a / a | First character; length |
| pa / pb | 2 / 2 | 209 / 210 | p / p | First character; length |

The sums/lengths are hand-checkable properties of these literal ASCII labels,
not identity algorithms to implement. Scratch mutations b and h substitute
ASCII-lowercased identity or byte-sum-only identity; additional h variants
use length-only and first-character-only identity. These settings introduce
no numerical tolerance or physical formula. Full string comparison remains
the production behavior. Source: owner's Fix 10 request and D18 opaque IDs.

## S3-02-ids2 Fix 11 — current-limit characterization

The three added tests reuse every existing synthetic geometry, value,
ID label, index and metadata field above. The swapped-ID chord expectations
are the same 2.5 m and 1.25 m values under the exchanged keys; no geometry
changes. Source-record swaps exchange the existing two distinct nonempty
synthetic references with the same m unit. The missing-record control removes
one existing station chord metadata entry and expects the existing exact
numeric-provenance-missing error. No new physical value, tolerance, hash,
source claim or production rule is introduced.

These tests describe today's model-layer checks, not future S3-02b2 binding
acceptance. The scratch mutation bypasses only missing-path rejection, so
the control must fail while both documented permissive cases still pass.
Source: owner's Fix 11 request and follow-up binding-gate clarification.
