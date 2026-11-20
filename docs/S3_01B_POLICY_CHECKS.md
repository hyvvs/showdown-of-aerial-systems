# Fix 4: documentation acceptance checks

Source: owner's S3-01b Fix 4 policy-only instruction. This is a documentation
checklist and recorded read-only check, not an implementation of S3-02,
mass buildup, a gallery, or a report. No production or test code is changed
by either Fix 4 commit. The temporary checker is local ignored evidence at
`target/s3-01b/fix4-policy-check.py`.

## Required policy checks, first recorded failing

For **each** of ASSUMPTIONS.md and ROADMAP.md check that the text states:

- Neutral naming applies to templates and the gallery, with a registry plus review.
- User designs may name themselves freely.
- A denylist cannot be complete.
- The report disclaimer limits claims.

For **each** of ROADMAP.md and TASKS.md check these future-task conditions:

- Designer JSON/file loading requires an automatic object-shape regression,
  with no hand-listed paths: discover every nested object, replace each
  independently with an array of its values, and assert rejection.
- One missing component mass blocks total mass and CG instead of counting
  as zero; tests must verify the error/missing result rather than a subtotal.

Normalize whitespace and case when checking document phrases; do not infer
these policies from a generic missing-input statement. Confirm by reading
the affected task sections, not only by matching words elsewhere.

Before policy edits, the read-only check reported:

```text
FAIL docs/ASSUMPTIONS.md: template/gallery registry and review
FAIL docs/ASSUMPTIONS.md: free user naming
FAIL docs/ASSUMPTIONS.md: denylist incompleteness
FAIL docs/ASSUMPTIONS.md: disclaimer limits claims
FAIL docs/ROADMAP.md: template/gallery registry and review
FAIL docs/ROADMAP.md: free user naming
FAIL docs/ROADMAP.md: denylist incompleteness
FAIL docs/ROADMAP.md: disclaimer limits claims
FAIL docs/ROADMAP.md: automatic object shape rejection
FAIL docs/ROADMAP.md: missing mass blocks aggregates
FAIL docs/TASKS.md: automatic object shape rejection
FAIL docs/TASKS.md: missing mass blocks aggregates
Documentation checks: 0 passed; 12 failed
```

ROADMAP already described recursive object substitution for S3-02. This
check requires making automatic discovery, individual substitution and no
hand-listed path inventory explicit. It does not claim that requirement
was wholly absent. S3-03 already treated missing inputs as missing; its
acceptance tests need the explicit missing-single-component case.

No aircraft number, numerical tolerance or formula is introduced. The
counts above are observed documentation-check results, not fixture data.
The executable JSON and mass tests belong to their future tasks and are
not run or claimed implemented here.

## Result after policy edits

```text
Documentation checks: 12 passed; 0 failed
```

Local evidence: `target/s3-01b/fix4-red.txt` and `fix4-green.txt`. Manual
review confirms the clauses appear in the naming policy and the actual
S3-02/S3-03 task sections, not merely in a verification record. Both Fix 4
commits are docs-only. No existing assertion, expected value or tolerance
is changed, and future tasks remain unauthorized/unstarted.
