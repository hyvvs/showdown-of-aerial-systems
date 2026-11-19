# T15b: efficiency gap — informational experiment, no assertions

## Scope and implementation

All geometry, operating settings and mesh counts are labeled SYNTHETIC TEST
VALUES, not real aircraft data. This experiment uses the archived independent
NumPy VLM/LLT implementation for BOTH baseline and tip-refined cases; it does
not claim direct repository-solver runs at AR80/160. The archived script body
is unchanged. The driver's in-memory mesh provider supplies custom tip edges;
Biot-Savart, symmetry reduction, linear solve and Trefftz calculations remain
those of the independent verifier. No production mesh API or dependency is
added, and none of these results is an acceptance assertion.

Conditions: rectangular chord1 m, span=AR m, alpha=.05 rad, ideal a0=2*pi/rad,
zero twist/alpha_L0/offsets, cosine full-span edges, eight uniform chord panels.
Full-wing meshes are 128x8, 256x8, 512x8, 1024x8, 2048x8 at AR10/20/40/80/160.
The rigid semi-infinite wake is along -body x; Trefftz is the analytic
downstream-infinity limit. No finite wake length, mass, density or speed is
invented. NumPy2.4.6 was already available; no installation was performed.

## Sources and algebra

- [MIT Lecture 9](https://web.mit.edu/16.unified/www/SPRING/fluids/Spring2008/LectureNotes/f09.pdf):
  LLT circulation, induced drag and span efficiency.
- [Drela JVL, sections 3–6](https://web.mit.edu/drela/Public/web/jvl/jvl.pdf):
  quarter/three-quarter lattice, tangency, horseshoe influence and far-wake drag.
- The coefficient identity used throughout is CDi=CL^2/(pi*AR*e), also
  documented in the existing estimator references. The decomposition below is
  derived algebraically from this identity; its ordering is explicit.

The script's original numerical kernel choices (including its off-line
collinearity guard) remain documented in independent/NUMERICAL_LEDGER.md.
This run adds neither a validation threshold nor a claimed physical cause.

## Fourier-term convergence and reference choice

References were evaluated at 128,256,512,1024,2048,4096 odd Fourier terms;
4096 is used for all reported comparisons. The last refinement changes are
shown below; they are observations, not asserted tolerances or certified tail
bounds. They are much smaller than the reported efficiency/gap differences.

| AR | Final CL | Final CDi | Final e | Last relative CL change | Last relative CDi change | Last relative e change |
| --- | --- | --- | --- | --- | --- | --- |
| 10 | 0.252339542707 | 0.002200966199 | 0.920889195755 | 1.354e-14 | 8.038e-14 | 5.307e-14 |
| 20 | 0.277213683410 | 0.001420876473 | 0.860781917572 | 2.265e-14 | 2.405e-13 | 1.953e-13 |
| 40 | 0.292780728809 | 0.000862449949 | 0.790935757343 | 4.796e-14 | 7.998e-13 | 7.040e-13 |
| 80 | 0.302054727548 | 0.000504110118 | 0.720121870353 | 9.415e-14 | 2.738e-12 | 2.549e-12 |
| 160 | 0.307410886810 | 0.000287493858 | 0.653943249873 | 1.810e-13 | 9.584e-12 | 9.222e-12 |

The complete term-sweep values and every setting/result are recorded by JSON
path in independent/T15B_EFFICIENCY_LEDGER.md. No finite-data convergence test
can prove the remaining infinite-term error is zero.

## (a) Efficiency at the same CL

For each AR, the common CL is the LLT value at alpha=.05 rad. In this linear
model Gamma and CL scale with alpha, CDi with alpha^2, so e is invariant to
that scaling. Thus the VLM value can be compared at the same CL without a new
solve: alpha_VLM_match=.05*CL_LLT/CL_VLM and
CDi_VLM_match=CDi_VLM*(CL_LLT/CL_VLM)^2. This would hold at any common nonzero
CL within this linear model; it is not a real-aircraft extrapolation.
Relative efficiency difference is (e_VLM-e_LLT)/e_LLT.

| AR | Mesh | e_VLM | e_LLT | Relative e difference | Common CL | VLM alpha at common CL, rad | CDi gap at common CL |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 10 | 128x8 | 0.966939876471 | 0.920889195755 | 5.00067553% | 0.252339542707 | 0.051968442761 | 4.76251749% |
| 20 | 256x8 | 0.907752514413 | 0.860781917572 | 5.45673601% | 0.277213683410 | 0.050956674191 | 5.17438356% |
| 40 | 512x8 | 0.834467677557 | 0.790935757343 | 5.50385032% | 0.292780728809 | 0.050468954459 | 5.21672935% |
| 80 | 1024x8 | 0.758050420257 | 0.720121870353 | 5.26696264% | 0.302054727548 | 0.050231970885 | 5.00343366% |
| 160 | 2048x8 | 0.686031128806 | 0.653943249873 | 4.90682929% | 0.307410886810 | 0.050115430376 | 4.67732113% |

## (b) Fixed-alpha drag-gap decomposition

Let L=CL_LLT, V=CL_VLM, K=pi*AR, eL=e_LLT, eV=e_VLM. Define the
counterfactual Dcf=V^2/(K*eL): FIRST change CL while holding LLT efficiency,
THEN change efficiency at VLM CL. With DL=L^2/(K*eL), DV=V^2/(K*eV):

```text
DL-DV = (DL-Dcf) + (Dcf-DV)
CL^2 part = (L^2-V^2)/(K*eL)
efficiency part = V^2/K*(1/eL-1/eV)

normalized by DL, r=(V/L)^2:
CL^2 part = 1-r
efficiency part = r*(1-eL/eV)
total relative CDi gap = 1-r*eL/eV
```

This is an EXACT algebraic decomposition apart from roundoff, not causal
attribution. Reversing the order assigns the interaction term differently.
Percentages below all use DL as denominator and add to the total; they are
not shares of a separately defined physical drag source.

| AR | CL^2 part, % of DL | Efficiency part, % of DL | Total CDi gap, % of DL | Absolute CL^2 part | Absolute efficiency part |
| --- | --- | --- | --- | --- | --- |
| 10 | 7.43205938% | 4.40856436% | 11.84062374% | 1.635771149136e-04 | 9.703101143218e-05 |
| 20 | 3.71960603% | 4.98191687% | 8.70152290% | 5.285100698417e-05 | 7.078688478980e-05 |
| 40 | 1.84975384% | 5.12023269% | 6.96998653% | 1.595320103582e-05 | 4.415944428216e-05 |
| 80 | 0.92146599% | 4.95732872% | 5.87879472% | 4.645203313657e-06 | 2.499039567527e-05 |
| 160 | 0.46012751% | 4.65579949% | 5.11592700% | 1.322838330849e-06 | 1.338513756683e-05 |

Maximum observed identity residual is 2.151e-16 of DL. This is reported, not tested against a selected acceptance tolerance.

## (c) Outer-tip refinement

Keep every baseline cosine edge. Insert exact cuts at +/-0.95 semispan, then
subdivide each interval outside those cuts into two or four equal pieces.
Both halves stay mirrored; interior nodes, span/chord, alpha and section data
are unchanged. Outer5% is the owner-suggested numerical study region, NOT a
physical-validity threshold. Extra cut nodes and subdivisions give the actual
full-wing counts below. Eight chordwise panels are retained.

The independent mesh transcription was checked informationally before custom
edges: baseline coordinate and area differences are zero at both AR20 and
AR40. Only the mesh provider is replaced in memory for each solve and restored
immediately; the archived script file is never edited.

| AR | Tip mesh | Ny | e_VLM | e_LLT | Relative e difference | Change in e difference, percentage points | Relative CL gap | Relative CDi gap |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 20 | baseline | 256 | 0.907752514413 | 0.860781917572 | 5.45673601% | +0.00000000 | 1.87742667% | 8.70152290% |
| 20 | 2 pieces/tip interval | 310 | 0.907691552235 | 0.860781917572 | 5.44965382% | -0.00708219 | 1.89262333% | 8.72367041% |
| 20 | 4 pieces/tip interval | 414 | 0.907664177280 | 0.860781917572 | 5.44647358% | -0.01026243 | 1.89997510% | 8.73459720% |
| 40 | baseline | 512 | 0.834467677557 | 0.790935757343 | 5.50385032% | +0.00000000 | 0.92919393% | 6.96998653% |
| 40 | 2 pieces/tip interval | 618 | 0.834413817774 | 0.790935757343 | 5.49704069% | -0.00680963 | 0.93918384% | 6.98274345% |
| 40 | 4 pieces/tip interval | 826 | 0.834390041328 | 0.790935757343 | 5.49403458% | -0.00981574 | 0.94408119% | 6.98929006% |

Tip refinement slightly decreases the efficiency difference in these runs,
while the total fixed-alpha CDi gap slightly INCREASES because CL changes too.
This is consistent with the algebra above, not evidence of a physical cause.
Two local refinement levels are not a globally converged mesh study; the
unrefined interior and Nx remain fixed.

## (d) What the observations do and do not establish

- The CL^2 contribution decreases strongly over this finite AR range; the
  efficiency contribution accounts for most of the high-AR total gap under
  the stated decomposition. Same-CL comparison retains a nonzero drag gap.
- The efficiency differences change only slightly under these outer-tip
  refinements. This does not prove grid independence or exclude other
  discretization, quadrature, model-definition or implementation effects.
- LLT term refinement is small at the precision reported, but there is no
  physical dataset or fully independent continuum reference here. The run
  uses the archived independent equations, not additional production coverage
  at the largest AR. Neither a physical explanation nor a bug is claimed.
- The supplied p≈.25445 and p≈1.00897 fits describe a particular finite
  AR20..160/mesh sequence. No universal AR^-0.25 or AR^-1 asymptote is proved.
  The question why those rates differ remains UNRESOLVED in QUESTIONS.md.

## Reproduction and ledgers

All CONFIG values, derived actual mesh counts, every measured scalar and
every numerical literal in the driver are inventoried in
independent/T15B_EFFICIENCY_LEDGER.md. Their provenance is SYNTHETIC benchmark
input, mathematical definition, or independent computed observation; none is
invented real-wing data. No new dependency or acceptance assertion is added.

Save the appendix under target/t15b/efficiency_experiments.py and run from
project-kit. It imports only the archived verifier and preinstalled NumPy,
disables bytecode output, and writes logs/results under target/t15b.

```powershell
$env:TEMP=(Resolve-Path target/t15b/temp).Path
$env:TMP=$env:TEMP
python target/t15b/efficiency_experiments.py
```

## Appendix: exact experimental driver (no assertions)

```python
"""Informational T15b experiment only: no numerical acceptance assertions.
Every setting is a labeled SYNTHETIC TEST VALUE. This driver imports only
the archived independent verifier, never repository production code.
"""
import importlib.util
import json
import sys
from pathlib import Path
import numpy as np

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    'independent_vlm_llt', ROOT / 'data/reference/estimator/independent/independent_vlm_llt.py')
VERIFIER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFIER)

CONFIG = {
    'data_note': 'All settings SYNTHETIC TEST VALUES, not aircraft data or validity thresholds',
    'cases': [[10, 128], [20, 256], [40, 512], [80, 1024], [160, 2048]],
    'nx': 8,
    'alpha_rad': 0.05,
    'fourier_terms': [128, 256, 512, 1024, 2048, 4096],
    'tip_ar': [20, 40],
    'tip_semispan_fraction': 0.05,
    'tip_subdivisions': [2, 4],
}
RESULT = {'config': CONFIG, 'numpy_version': np.__version__,
          'llt_convergence': [], 'baseline': [], 'tip_refinement': []}


def comparison(vlm, llt, ar):
    """MIT/JVL coefficient law; change CL first at fixed LLT efficiency.
    The decomposition is algebraic and its ordering is explicit, not causal.
    """
    clv, cll = vlm['cl'], llt['cl']
    ev, el = vlm['e'], llt['e']
    cl_ratio_sq = (clv / cll)**2
    cl_part = 1 - cl_ratio_sq
    efficiency_part = cl_ratio_sq * (1 - el / ev)
    counterfactual_cd = clv**2 / (np.pi * ar * el)
    same_cl_cd = vlm['cdi'] * (cll / clv)**2
    return {
        'ar': ar, 'ny': vlm['ny'], 'nx': vlm['nx'],
        'cl_vlm': clv, 'cdi_vlm': vlm['cdi'], 'e_vlm': ev,
        'cl_llt': cll, 'cdi_llt': llt['cdi'], 'e_llt': el,
        'relative_cl_gap': abs(clv / cll - 1),
        'relative_cdi_gap': 1 - vlm['cdi'] / llt['cdi'],
        'relative_e_difference': ev / el - 1,
        'same_cl_target': cll,
        'vlm_alpha_at_same_cl_rad': CONFIG['alpha_rad'] * cll / clv,
        'vlm_cdi_at_same_cl': same_cl_cd,
        'relative_cdi_gap_at_same_cl': 1 - same_cl_cd / llt['cdi'],
        'cl_effect_absolute': llt['cdi'] - counterfactual_cd,
        'efficiency_effect_absolute': counterfactual_cd - vlm['cdi'],
        'cl_effect_fraction_of_llt_drag': cl_part,
        'efficiency_effect_fraction_of_llt_drag': efficiency_part,
        'decomposition_roundoff': cl_part + efficiency_part - (1 - vlm['cdi'] / llt['cdi']),
        'residual': vlm['residual'],
        'max_panel_width_m': vlm['max_panel_width'],
    }


def refined_edges(ar, baseline_ny, subdivisions):
    """Keep baseline cosine nodes; add exact 95%-semispan cuts, then split
    each outside interval. Both halves are symmetric; all inner nodes remain.
    """
    semispan = ar / 2
    cut = (1 - CONFIG['tip_semispan_fraction']) * semispan
    original = -semispan * np.cos(np.linspace(0, np.pi, baseline_ny + 1))
    original[baseline_ny // 2] = 0.0
    edges = np.unique(np.concatenate((original, [-cut, cut])))
    additions = []
    for a, b in zip(edges[:-1], edges[1:]):
        if a >= cut or b <= -cut:
            additions.extend(np.linspace(a, b, subdivisions + 1)[1:-1])
    return np.unique(np.concatenate((edges, additions)))


def rectangle_mesh_on_edges(edges, nx):
    """Same flat-plate quarter/three-quarter formulas as the verifier;
    only the requested span edges differ. No verifier file is modified.
    """
    ny = len(edges) - 1
    ya, yb = np.repeat(edges[:-1], nx), np.repeat(edges[1:], nx)
    fb = np.tile((np.arange(nx) + 0.25) / nx, ny)
    fc = np.tile((np.arange(nx) + 0.75) / nx, ny)
    xa, xb, xc = -fb, -fb, -fc
    yc = (ya + yb) / 2
    area = float(np.trapezoid(np.ones(len(edges)), edges))
    return xa, ya, xb, yb, xc, yc, edges, area


references = {}
for ar, ny in CONFIG['cases']:
    previous = None
    for terms in CONFIG['fourier_terms']:
        ll = VERIFIER.lifting_line(ar, terms, alpha=CONFIG['alpha_rad'])
        row = dict(ar=ar, terms=terms, **ll)
        if previous is not None:
            row['relative_cl_last_change'] = abs(ll['cl'] / previous['cl'] - 1)
            row['relative_cdi_last_change'] = abs(ll['cdi'] / previous['cdi'] - 1)
            row['relative_e_last_change'] = abs(ll['e'] / previous['e'] - 1)
        RESULT['llt_convergence'].append(row)
        previous = ll
        print('LLT', json.dumps(row), flush=True)
    references[ar] = ll
    v = VERIFIER.vlm(ar, ny, nx=CONFIG['nx'])
    row = comparison(v, ll, ar)
    RESULT['baseline'].append(row)
    print('BASELINE', json.dumps(row), flush=True)

original_mesh_function = VERIFIER.make_mesh
for ar, baseline_ny in CONFIG['cases']:
    if ar not in CONFIG['tip_ar']:
        continue
    # Print reconstruction differences, no assertions: verify unchanged formula
    # transcription on the baseline cosine grid before adding tip nodes.
    native = original_mesh_function(ar, baseline_ny, CONFIG['nx'], False)
    rebuilt = rectangle_mesh_on_edges(native[6], CONFIG['nx'])
    RESULT.setdefault('mesh_reconstruction', []).append({
        'ar': ar, 'max_coordinate_difference': float(max(
            np.max(np.abs(a - b)) for a, b in zip(native[:7], rebuilt[:7]))),
        'area_difference': float(rebuilt[7] - native[7]),
    })
    for subdivisions in CONFIG['tip_subdivisions']:
        edges = refined_edges(ar, baseline_ny, subdivisions)
        actual_ny = len(edges) - 1

        def tip_mesh(query_ar, query_ny, query_nx, ellipse, chosen=edges):
            return rectangle_mesh_on_edges(chosen, query_nx)

        # In-memory mesh provider only; archived verifier's bytes/logic stay
        # untouched. Restore it immediately after each experimental solve.
        VERIFIER.make_mesh = tip_mesh
        try:
            v = VERIFIER.vlm(ar, actual_ny, nx=CONFIG['nx'])
        finally:
            VERIFIER.make_mesh = original_mesh_function
        row = comparison(v, references[ar], ar)
        row.update(baseline_ny=baseline_ny, tip_subdivisions=subdivisions,
                   tip_semispan_fraction=CONFIG['tip_semispan_fraction'])
        RESULT['tip_refinement'].append(row)
        print('TIP', json.dumps(row), flush=True)

path = ROOT / 'target/t15b/efficiency-results.json'
path.write_text(json.dumps(RESULT, indent=2), encoding='utf-8')
print('SAVED', path, flush=True)
```
