# Owner-run AVL comparison — no AVL run performed by the agent

These are newly authored **synthetic test input files**, not AVL source, a
binary, or output from AVL. Check AVL's license before committing or
redistributing any of its files or generated output. No AVL source or binary
may be committed here. Do not treat the presence of these inputs as validation.

Input syntax/commands were checked against the
[AVL 3.40 user primer](https://web.mit.edu/drela/Public/web/avl/avl_doc.txt).
The scripts have **not been executed**; compatibility with your chosen AVL
build and its output precision remains to be checked by you.

## Matching the repository

- `rectangle_ar40_64x8.avl`: original expected-difference case, AR=40,
  full-wing 64 spanwise x 8 chordwise panels, cosine span, uniform chord,
  span=40 m, chord=1 m, area=40 m2, no twist/camber, reference at origin.
- `rectangle_ar40_128x8.avl`: same wing and settings, matching the new trend
  experiment's finer span mesh. Counts always mean FULL wing, with no image
  symmetry. One surface extends from y=-20 to +20, so native Sspace=1 applies
  cosine over the full span rather than clustering at the root as well.
- `ellipse_ar40_128x8.avl`: practical match to the diagnostic elliptic polygon,
  with the same sampled chord/leading-edge boundaries. One strip per explicit
  section interval pins all 128 cosine strip boundaries; eight uniform chord
  panels per strip. Exact analytic ellipse area/MAC normalize both runs, while
  the represented geometry is piecewise linear. Zero tip chord is explicit;
  if your AVL build rejects it, stop and save the diagnostic rather than
  introducing a nonzero tip chord silently. This ellipse is not a production
  T12 input (T12 requires positive station chords).

AVL geometric X is aft, Z up: X=-body_x, Y=body_y, Z=-body_z. Pitch sign is
unchanged. Convert its neutral-point X back with body_x=-Xnp. The reference
point is the origin in both systems. Flat-plate sections imply ideal 2*pi
section slope and zero intrinsic section pitching moment; no NACA shape,
profile drag, mass, density, speed or real aircraft/airfoil data are supplied.
`INPUT_VALUES.md` ledgers every numeric input and command setting.

Trailing legs are body-axis-aligned in both methods (AVL +X, repository -x);
Trefftz is the analytic far-wake limit, no finite downstream distance. Remaining
differences: repository linearizes alpha while AVL uses trigonometric geometry
and its force formulation; near-field integration and spanwise wake quadrature
can differ. The native spacing formulas/actual panel locations should be
confirmed from AVL's element/geometry display before asserting an exact mesh
match. No claim of bit-identical panel locations or results is made without
running AVL. Mesh density, coordinate/reference definitions and flat-plate
assumptions are deliberately matched, not tuned after seeing output.

## Exact owner commands (PowerShell)

Put an independently obtained, license-checked AVL executable outside this
repository. Replace only its path below. Run from a fresh AVL process with no
existing `.run` files and no earlier output files of the same names, because
overwrite prompts would consume command-script lines.

```powershell
Push-Location 'A:\Showdown of Aerial Systems\project-kit\data\reference\avl'
Get-Content -LiteralPath '.\rectangle_ar40_64x8.commands' |
    & 'C:\path\to\avl.exe' *> '.\rectangle_ar40_64x8.console.txt'
Get-Content -LiteralPath '.\rectangle_ar40_128x8.commands' |
    & 'C:\path\to\avl.exe' *> '.\rectangle_ar40_128x8.console.txt'
Get-Content -LiteralPath '.\ellipse_ar40_128x8.commands' |
    & 'C:\path\to\avl.exe' *> '.\ellipse_ar40_128x8.console.txt'
Pop-Location
```

Each script loads geometry, sets alpha=0.05 rad expressed in degrees, beta and
all rates to zero, executes, and saves total forces (`.ft.txt`), stability
derivatives (`.st.txt`) and strip forces (`.fs.txt`). Keep the complete console
transcript too, including version banner, warnings, settings and convergence.
If commands/prompts differ in your build, use OPER interactively following the
script and record exactly what changed. Do not fabricate missing output fields.

Read and preserve:

| Quantity | Where/interpretation |
|---|---|
| CL | `CLtot` total lift coefficient in FT; also record Trefftz lift if printed |
| Induced CD | `CDind` / Trefftz-plane `CDi`, according to the output labels; preserve both if present and identify which is far field; do not substitute `CDtot` or profile drag |
| Cm | `Cmtot` about the stated reference, pitch sign unchanged |
| CL slope, Cm slope | `CLa`, `Cma` in ST, normally per radian; retain labels/units |
| Neutral point | `Xnp` in ST if available; otherwise calculate Xnp=Xref-Cref*Cma/CLa and retain the derivative values; repository body x=-Xnp |
| Efficiency | Far-field span efficiency `e`, beta=0 |
| Mesh | Element/strip counts and strip positions, confirming full-wing cosine distribution and uniform chord |

The neutral-point relation follows a moment-reference shift and the condition
dCm/dalpha=0 there; see the estimator VLM reference note. Keep coordinate signs
and dimensional Xnp separate from chord-fraction static margin.

Before submitting output for comparison, record executable version/provenance,
license review, input filenames and SHA-256 hashes, operating settings, whether
any mesh discrepancy was observed, and any manual script changes. No AVL output
was already present when this continuation began, so no AVL comparison test
can yet be added honestly.
