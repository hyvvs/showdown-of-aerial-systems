# T15b: independent taper sweep — informational, no assertions

## Solver identity and scope

**Every VLM number in this report comes from the archived independent NumPy
verifier**, not the production Rust solver. Production results and its AR160
resource limitation are separately documented in t15b_production_comparison.md.
No repository production code is imported. No dependency was added or installed.
NumPy version: 2.4.6 (already present).

The archived independent_vlm_llt.py is unchanged. The off-line driver supplies
an in-memory straight-taper mesh to its existing VLM function; horseshoe
influences, mirror reduction, dense solve and Trefftz drag are untouched.
The archive's LLT entry point only exposes rectangle/ellipse chords. The
driver reproduces **the same independent Fourier collocation matrix and
coefficient sums**, replacing only its chord distribution with the requested
straight taper. This geometry adapter is reproduced below and imports no Rust
or production algorithm. It is not a newly substituted production solver.

Archived whole-file SHA256 before/after:
`34840b6d04f42bfdf8a011e596fe86fe1eb654d8081b791bca9bfc76c80aa0ef` /
`34840b6d04f42bfdf8a011e596fe86fe1eb654d8081b791bca9bfc76c80aa0ef`.
The original body SHA256 remains the existing NUMERICAL_LEDGER.md value.

All geometry, operating values, mesh counts and Fourier settings are labelled
**SYNTHETIC TEST VALUES**, not real wings or validity thresholds. This report
adds no acceptance assertion, tolerance change, physical/bug verdict or T10/T11
work. Planar, incompressible, linear small-angle attached-flow assumptions
remain; no viscosity, compressibility, stall or wake relaxation is modeled.
Real-wing validity is unassessed.

## Geometry, formulas and sources

The owner-specified tapers are lambda=c_tip/c_root=1,.5,.25,.1. For every AR,
retain the existing mean-geometric-chord normalization S/b=1 m, hence b=AR m,
S=AR m^2, c_root=2/(1+lambda) m and c_tip=lambda*c_root. Each is a symmetric
straight taper with zero twist, zero section zero-lift angle and z=0. The
quarter chord is unswept at x=0: body x is forward, so x_LE(y)=c(y)/4.
Alpha=.05 rad; ideal section slope a0=2*pi per rad. No real section data is
invented. Absolute normalization only fixes coordinates for the scale-free
coefficients; the actual integrated area/AR is ledgered for every solve.

- [Caughey, Aerodynamic Background, eqs.2.1,2.3,2.7–2.10,2.15](https://courses.cit.cornell.edu/mae5070/AeroBackground.pdf):
  area/aspect ratio, straight-taper chord, MAC and ideal section slope.
  c(y)=c_root*[1-(1-lambda)*2*abs(y)/b].
- [MIT Unified Fluids Lecture 9](https://web.mit.edu/16.unified/www/SPRING/fluids/Spring2008/LectureNotes/f09.pdf):
  Gamma=2*b*V*sum(A_n*sin(n*theta)); collocation matrix
  sin(n*theta)*[4*b/(a0*c(theta))+n/sin(theta)] with RHS alpha-alpha_L0.
  CL=pi*AR*A1, CDi=pi*AR*sum(n*A_n^2), e=CL^2/(pi*AR*CDi).
  Odd modes and half-span symmetry are the archived independent formulation.
- [Drela, JVL formulation, sections 3–6](https://web.mit.edu/drela/Public/web/jvl/jvl.pdf):
  quarter-chord bound vortices, three-quarter-chord control points, tangency,
  induced velocity and far-wake forces. The archive's wake is semi-infinite
  along -body x, with Trefftz drag at downstream infinity, not a finite wake.
- [NIST e-Handbook, Least Squares](https://www.itl.nist.gov/div898/handbook/pmd/section4/pmd431.htm):
  minimize sums of squared residuals; transformations/objectives for the two
  owner-requested empirical fit forms are stated below. No causal fit is implied.

The meshes are full-wing Ny x Nx=128x8,256x8,512x8,1024x8,2048x8 at
AR10,20,40,80,160. Span edges y_j=-(b/2)*cos(pi*j/Ny); chord divisions are
uniform fractions. These are the existing cosine mesh settings, not a new
production default. Max panel width/(S/b) across all cases is
0.122718414903, below the owner-requested
one-mean-chord width constraint. S/b is distinguished from MAC; all computed
MACs are at least S/b, so the width constraint also holds using MAC.
The MAC table uses Caughey's trapezoid formula
MAC=2*c_root*(1+lambda+lambda^2)/(3*(1+lambda)); S/b is the simple area/span
mean and should not be confused with this aerodynamic reference chord.

| lambda | Root chord m | Tip chord m | S/b m | MAC m | Max panel width / MAC |
| --- | --- | --- | --- | --- | --- |
| 1.0 | 1.000000000000 | 1.000000000000 | 1.0 | 1.000000000000 | 0.122718415 |
| 0.5 | 1.333333333333 | 0.666666666667 | 1.0 | 1.037037037037 | 0.118335614 |
| 0.25 | 1.600000000000 | 0.400000000000 | 1.0 | 1.120000000000 | 0.109570013 |
| 0.1 | 1.818181818182 | 0.181818181818 | 1.0 | 1.223140495868 | 0.100330596 |

## Efficiency and CL/CDi differences

Signed relative efficiency difference is e_VLM/e_LLT-1. CL and CDi gaps are
absolute relative differences abs(VLM/LLT-1), with LLT in the denominator.
All LLT references below use 4096 odd terms. Linear circulation scales with
alpha; CL scales with alpha and CDi with alpha^2. Therefore e is unchanged
when comparing at the same nonzero CL. The common CL is the LLT value;
alpha_VLM_match=.05*CL_LLT/CL_VLM is ledgered. This is within-model algebra,
not a real-wing extrapolation.

| lambda | AR | Ny x Nx | e VLM | e LLT (4096) | Signed relative e difference | Absolute relative CL gap | Absolute relative CDi gap |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.0 | 10 | 128x8 | 0.966939876471 | 0.920889195755 | 5.00067553% | 3.78776553% | 11.84062374% |
| 1.0 | 20 | 256x8 | 0.907752514413 | 0.860781917572 | 5.45673601% | 1.87742667% | 8.70152290% |
| 1.0 | 40 | 512x8 | 0.834467677557 | 0.790935757343 | 5.50385032% | 0.92919393% | 6.96998653% |
| 1.0 | 80 | 1024x8 | 0.758050420257 | 0.720121870353 | 5.26696264% | 0.46179929% | 5.87879472% |
| 1.0 | 160 | 2048x8 | 0.686031128806 | 0.653943249873 | 4.90682929% | 0.23032901% | 5.11592700% |
| 0.5 | 10 | 128x8 | 1.001331201893 | 0.977798407220 | 2.40671231% | 3.29574064% | 8.68065611% |
| 0.5 | 20 | 256x8 | 0.979028638264 | 0.954256804054 | 2.59592953% | 1.38702702% | 5.21535811% |
| 0.5 | 40 | 512x8 | 0.946188324740 | 0.920817860557 | 2.75520983% | 0.58645740% | 3.81945141% |
| 0.5 | 80 | 1024x8 | 0.905229332056 | 0.880683085061 | 2.78718275% | 0.25697048% | 3.21096783% |
| 0.5 | 160 | 2048x8 | 0.860483365081 | 0.837600949799 | 2.73189940% | 0.11728012% | 2.88744012% |
| 0.25 | 10 | 128x8 | 0.992995096481 | 0.977977978477 | 1.53552721% | 3.29208715% | 7.89016746% |
| 0.25 | 20 | 256x8 | 0.973692008156 | 0.960875846990 | 1.33379991% | 1.27051206% | 3.80789235% |
| 0.25 | 40 | 512x8 | 0.952264136209 | 0.940796066600 | 1.21897508% | 0.46991216% | 2.13061951% |
| 0.25 | 80 | 1024x8 | 0.930389713318 | 0.919948614512 | 1.13496544% | 0.17409105% | 1.46620356% |
| 0.25 | 160 | 2048x8 | 0.909088144717 | 0.899356346575 | 1.08208478% | 0.06677706% | 1.20258136% |
| 0.1 | 10 | 128x8 | 0.952521691256 | 0.936219977112 | 1.74122691% | 3.61493005% | 8.68911264% |
| 0.1 | 20 | 256x8 | 0.916155058420 | 0.905481445214 | 1.17877768% | 1.39596195% | 3.90518108% |
| 0.1 | 40 | 512x8 | 0.885602557325 | 0.879094466039 | 0.74031763% | 0.49493032% | 1.71503202% |
| 0.1 | 80 | 1024x8 | 0.863495086991 | 0.859544987492 | 0.45955704% | 0.16568534% | 0.78703633% |
| 0.1 | 160 | 2048x8 | 0.848712905413 | 0.846064806101 | 0.31299013% | 0.05380521% | 0.41925937% |

At lambda .5 / AR10 the raw finite-mesh value e=1.001331201893 is above 1.
It is retained without adjustment. This is not one of the elliptic Richardson
estimates screened by the existing Munk tests. It does not demonstrate a
continuum efficiency above the planar bound; its cause is unestablished here.
This observational part adds no assertion or threshold-based verdict for it.

## Fourier refinement, shown rather than assumed

Every case was run at 128,256,512,1024,2048,4096 odd Fourier terms. The table
shows efficiency at four of those levels and last-doubling relative changes
for CL/CDi/e. The full coefficient/term sweep is in the numerical ledger.
4096 is the finest computed reference, not a certified infinite-term result.
No threshold-based convergence verdict is selected.

| lambda | AR | e N128 | e N1024 | e N2048 | e N4096 | Last relative CL change | Last relative CDi change | Last relative e change |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1.0 | 10 | 0.920889199157 | 0.920889195756 | 0.920889195755 | 0.920889195755 | 1.354472e-14 | 8.038015e-14 | 5.306866e-14 |
| 1.0 | 20 | 0.860781929300 | 0.860781917575 | 0.860781917572 | 0.860781917572 | 2.264855e-14 | 2.404743e-13 | 1.952882e-13 |
| 1.0 | 40 | 0.790935796577 | 0.790935757353 | 0.790935757344 | 0.790935757343 | 4.796163e-14 | 7.998047e-13 | 7.039924e-13 |
| 1.0 | 80 | 0.720121999945 | 0.720121870384 | 0.720121870355 | 0.720121870353 | 9.414691e-14 | 2.737588e-12 | 2.549183e-12 |
| 1.0 | 160 | 0.653943678268 | 0.653943249975 | 0.653943249879 | 0.653943249873 | 1.809664e-13 | 9.583889e-12 | 9.222068e-12 |
| 0.5 | 10 | 0.977797691143 | 0.977798396727 | 0.977798405122 | 0.977798407220 | 1.876966e-08 | 3.968556e-08 | 2.146243e-09 |
| 0.5 | 20 | 0.954255134756 | 0.954256779553 | 0.954256799154 | 0.954256804054 | 2.082459e-08 | 4.678401e-08 | 5.134827e-09 |
| 0.5 | 40 | 0.920814906241 | 0.920817816953 | 0.920817851837 | 0.920817860557 | 2.222186e-08 | 5.391362e-08 | 9.469899e-09 |
| 0.5 | 80 | 0.880678849159 | 0.880683021220 | 0.880683072294 | 0.880683085061 | 2.313236e-08 | 6.076194e-08 | 1.449722e-08 |
| 0.5 | 160 | 0.837595924566 | 0.837600867736 | 0.837600933382 | 0.837600949799 | 2.370238e-08 | 6.700450e-08 | 1.959975e-08 |
| 0.25 | 10 | 0.977972417783 | 0.977977897024 | 0.977977962187 | 0.977977978477 | 2.855639e-08 | 7.376971e-08 | 1.665694e-08 |
| 0.25 | 20 | 0.960865289703 | 0.960875692417 | 0.960875816077 | 0.960875846990 | 3.335602e-08 | 9.888385e-08 | 3.217182e-08 |
| 0.25 | 40 | 0.940778961961 | 0.940795816244 | 0.940796016533 | 0.940796066600 | 3.706390e-08 | 1.273453e-07 | 5.321755e-08 |
| 0.25 | 80 | 0.919924117702 | 0.919948255333 | 0.919948542689 | 0.919948614512 | 3.970292e-08 | 1.574796e-07 | 7.807381e-08 |
| 0.25 | 160 | 0.899324580988 | 0.899355875014 | 0.899356252291 | 0.899356346575 | 4.145626e-08 | 1.877469e-07 | 1.048344e-07 |
| 0.1 | 10 | 0.936210562617 | 0.936219839204 | 0.936219949531 | 0.936219977112 | 3.471621e-08 | 9.889242e-08 | 2.946001e-08 |
| 0.1 | 20 | 0.905464300724 | 0.905481194216 | 0.905481395016 | 0.905481445214 | 4.200338e-08 | 1.394441e-07 | 5.543734e-08 |
| 0.1 | 40 | 0.879067039333 | 0.879094064843 | 0.879094385806 | 0.879094466039 | 4.796006e-08 | 1.871880e-07 | 9.126788e-08 |
| 0.1 | 80 | 0.859505224095 | 0.859544405987 | 0.859544871210 | 0.859544987492 | 5.237644e-08 | 2.400363e-07 | 1.352835e-07 |
| 0.1 | 160 | 0.846011536376 | 0.846064023257 | 0.846064649583 | 0.846064806101 | 5.539879e-08 | 2.957923e-07 | 1.849948e-07 |

Maximum last-doubling relative changes across the sweep: CL
5.539878589e-08, CDi 2.957923069e-07, e 1.849947928e-07.
These measure observed Fourier sensitivity only; they do not bound VLM mesh
error, establish a universal error tail or remove the stated model limits.

## Two empirical fits and every residual

Fit only AR20,40,80,160 for each taper. Define g=abs(e_VLM/e_LLT-1), a
dimensionless relative gap. Both fits use these same four values:

- Power law g=C*AR^(-p): ordinary least squares of ln(g)=ln(C)-p*ln(AR).
  Its optimization objective is the sum of squared **log-gap** residuals.
- Inverse log g=a+b/ln(AR): ordinary least squares in **original gap** units.
  It has a different objective; residual sizes are shown in both original
  units and, for the power fit, log units. A fitted intercept a is not a
  demonstrated asymptotic floor.

Residual=observed-predicted. RMSE=sqrt(mean(residual^2)); it is descriptive,
not a pass/fail limit or uncertainty band. Coefficients, original-scale fits
and all residuals are preserved at full float precision in the ledger.

| lambda | C | p | Power original RMSE | Power log RMSE | a | b | Inverse-log original RMSE |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.0 | 0.06519089259 | 0.052321151 | 1.068639e-03 | 2.006352e-02 | 0.04317440136 | 0.03751561672 | 1.347580e-03 |
| 0.5 | 0.02468183288 | -0.023760481 | 5.420378e-04 | 1.998050e-02 | 0.03003363631 | -0.0110978873 | 4.556849e-04 |
| 0.25 | 0.01785574994 | 0.100820643 | 1.260564e-04 | 1.059247e-02 | 0.007150668188 | 0.01853695465 | 1.926897e-05 |
| 0.1 | 0.07963361929 | 0.642720824 | 1.290489e-04 | 2.338599e-02 | -0.009800177539 | 0.06418293536 | 2.282628e-04 |

| lambda | AR | Observed g | Power prediction | Power original residual | Power log residual | Inverse-log prediction | Inverse-log residual |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.0 | 20 | 0.0545673601 | 0.0557334003 | -1.166040e-03 | -2.114371e-02 | 0.0556974219 | -1.130062e-03 |
| 1.0 | 40 | 0.0550385032 | 0.0537483708 | +1.290132e-03 | +2.371964e-02 | 0.0533443235 | +1.694180e-03 |
| 1.0 | 80 | 0.0526696264 | 0.0518340411 | +8.355854e-04 | +1.599184e-02 | 0.0517356494 | +9.339771e-04 |
| 1.0 | 160 | 0.0490682929 | 0.0499878931 | -9.196002e-04 | -1.856778e-02 | 0.0505663879 | -1.498095e-03 |
| 0.5 | 20 | 0.0259592953 | 0.0265027235 | -5.434281e-04 | -2.071775e-02 | 0.0263290705 | -3.697752e-04 |
| 0.5 | 40 | 0.0275520983 | 0.0269428245 | +6.092737e-04 | +2.236168e-02 | 0.0270251652 | +5.269331e-04 |
| 0.5 | 80 | 0.0278718275 | 0.0273902338 | +4.815936e-04 | +1.742989e-02 | 0.0275010439 | +3.707835e-04 |
| 0.5 | 160 | 0.0273189940 | 0.0278450728 | -5.260788e-04 | -1.907382e-02 | 0.0278469354 | -5.279414e-04 |
| 0.25 | 20 | 0.0133379991 | 0.0132010178 | +1.369813e-04 | +1.032311e-02 | 0.0133384557 | -4.565822e-07 |
| 0.25 | 40 | 0.0121897508 | 0.0123099809 | -1.202301e-04 | -9.814884e-03 | 0.0121757591 | +1.399170e-05 |
| 0.25 | 80 | 0.0113496544 | 0.0114790868 | -1.294324e-04 | -1.133955e-02 | 0.0113808923 | -3.123792e-05 |
| 0.25 | 160 | 0.0108208478 | 0.0107042761 | +1.165717e-04 | +1.083133e-02 | 0.0108031450 | +1.770280e-05 |
| 0.1 | 20 | 0.0117877768 | 0.0116117422 | +1.760347e-04 | +1.504629e-02 | 0.0116246126 | +1.631642e-04 |
| 0.1 | 40 | 0.0074031763 | 0.0074373657 | -3.418945e-05 | -4.607583e-03 | 0.0075988555 | -1.956792e-04 |
| 0.1 | 80 | 0.0045955704 | 0.0047636615 | -1.680911e-04 | -3.592371e-02 | 0.0048466835 | -2.511131e-04 |
| 0.1 | 160 | 0.0031299013 | 0.0030511436 | +7.875770e-05 | +2.548500e-02 | 0.0028462731 | +2.836281e-04 |

Four aspect ratios and two fitted parameters provide limited finite-range
descriptions. Different objectives and mesh changes with AR prevent a residual
ranking from establishing a cause, physical mechanism or infinite-AR law.
No fitted exponent/intercept is made an acceptance criterion.
The rectangular and lambda .5 efficiency differences are not monotonic over
this AR range; lambda .5 has a negative fitted p. The p values here describe
the **efficiency** gap and are not the earlier CDi-gap fitted exponents.

## Tip-panel resolution for lambda .25 and .1

The rightmost cosine-panel width is Delta_y=(b/2)*[1-cos(pi/Ny)], derived
directly from the edge formula. Compare with the **local tip chord**, not
only the mean chord. The final column is the maximum panel width divided by
its local midpoint chord anywhere on the wing; it exposes broader interior
panels even when the very last tip panel is narrow.

| lambda | AR | Ny | Tip panel width m | Local tip chord m | Tip panel width / tip chord | Max width / local midpoint chord anywhere |
| --- | --- | --- | --- | --- | --- | --- |
| 0.25 | 10 | 128 | 0.001505906519 | 0.400000000000 | 0.37647663% | 0.115943717 |
| 0.25 | 20 | 256 | 0.000752981609 | 0.400000000000 | 0.18824540% | 0.115951376 |
| 0.25 | 40 | 512 | 0.000376494348 | 0.400000000000 | 0.09412359% | 0.115956749 |
| 0.25 | 80 | 1024 | 0.000188247617 | 0.400000000000 | 0.04706190% | 0.115957820 |
| 0.25 | 160 | 2048 | 0.000094123864 | 0.400000000000 | 0.02353097% | 0.115957955 |
| 0.1 | 10 | 128 | 0.001505906519 | 0.181818181818 | 0.82824859% | 0.154787591 |
| 0.1 | 20 | 256 | 0.000752981609 | 0.181818181818 | 0.41413988% | 0.154827056 |
| 0.1 | 40 | 512 | 0.000376494348 | 0.181818181818 | 0.20707189% | 0.154841154 |
| 0.1 | 80 | 1024 | 0.000188247617 | 0.181818181818 | 0.10353619% | 0.154842759 |
| 0.1 | 160 | 2048 | 0.000094123864 | 0.181818181818 | 0.05176813% | 0.154844078 |

The last-panel width ranges from
0.02353097% to
0.82824859% of local tip chord for
these two tapers. Thus the final panels are geometrically narrow compared
with the local chord. This supports the geometric meaning of the sampled
tip shape; sufficiency for circulation and induced-drag convergence remains
unassessed. This length ratio alone does **not** prove aerodynamic resolution.
Interior width/chord ratios are separately shown, and this sweep is not a
joint spanwise/chordwise refinement study. No width-based accuracy threshold
or pass/fail verdict is imposed.

## What these data do and do not show

The table reports how the two independent model calculations differ for each
specified taper, AR and growing cosine mesh. It separates their CL, CDi and
same-CL efficiency differences and exposes remaining Fourier sensitivity.
The empirical fits summarize this finite sample with visible residuals.
They do not distinguish physical effects, model-definition differences,
wake/force quadrature, unresolved VLM discretization or implementation issues.
No gap is labelled physical or a bug.

For lambda1 the geometry adapter is a pure quarter-chord translation of the
previous rectangle. Recorded coordinate/area reconstruction differences are
zero; the e differences from the previously archived baseline at the directly
cross-referenced aspect ratios are [(10, 0.0), (20, 0.0), (40, 0.0), (80, 0.0), (160, 0.0)]. These are measured
roundoff changes, not a newly asserted tolerance. Source hashes are unchanged.
Run wall time for this complete off-line experiment: 150.0803612 s;
that is not a production-solver performance claim.

The unresolved question remains: **why does the CDi gap decay as AR^-0.25 while
the CL gap decays as AR^-1?** The earlier exponents describe a particular
finite-range rectangular sequence; no cause or universal exponent follows
from this taper sweep.

## Numerical/settings ledger and reproduction

Every configuration, mesh/term count, full coefficient, relative change, fit
coefficient, residual, timing and derived geometry result is enumerated in
independent/T15B_TAPER_LEDGER.md. The driver numerical-literal inventory is
included there; archived kernel settings retain their prior NUMERICAL_LEDGER.md
provenance. The complete no-assertion driver below is a documentation appendix,
outside build/CI. Run it under target/t15b-profile in the existing NumPy
environment; it writes only ignored results/logs. Do not invoke the archive's
CLI modes, which write next to its source. No package installation is required.
The report tables are direct arithmetic/formatting of its JSON output; all
coefficients and residuals use the definitions above.

```python
"""No-assertion informational sweep. All settings are SYNTHETIC TEST VALUES.
No repository production imports. Archived verifier bytes/logic stay unchanged.
Geometry adapters supply straight taper to its VLM and its LLT matrix formula.
Sources: MIT Lecture 9 (LLT), Drela JVL (VLM), Caughey (wing geometry),
NIST e-Handbook 4.4.3.1 (least-squares fitting); links in the generated report.
"""
import importlib.util
import hashlib
import json
import sys
import time
from pathlib import Path
import numpy as np

sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[2]
PATH=ROOT/'data/reference/estimator/independent/independent_vlm_llt.py'
SPEC=importlib.util.spec_from_file_location('independent_taper_reference',PATH)
VERIFIER=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFIER)
CONFIG=dict(data_note='Every setting SYNTHETIC TEST VALUE; no real aircraft, no assertion threshold',
    cases=[[10,128],[20,256],[40,512],[80,1024],[160,2048]],
    taper_ratios=[1.0,0.5,0.25,0.1],nx=8,alpha_rad=.05,
    fourier_terms=[128,256,512,1024,2048,4096],
    mean_geometric_chord_m=1.0,quarter_chord_x_m=0.0,
    twist_rad=0.0,zero_lift_angle_rad=0.0,z_m=0.0,
    section_slope_per_rad=float(2*np.pi),spanwise_spacing='full-span cosine',
    fit_ar=[20,40,80,160],fit_residual_definition='observed minus fitted',
    relative_gap_definition='abs(VLM/LLT-1)',
    relative_efficiency_definition='(e_VLM/e_LLT)-1',
    fit_efficiency_definition='absolute relative efficiency difference',
    coefficient_print_decimal_places=12,percentage_print_decimal_places=8)
RESULT=dict(config=CONFIG,numpy_version=np.__version__,
    archived_sha256_before=hashlib.sha256(PATH.read_bytes()).hexdigest(),
    llt_convergence=[],comparison=[],fits=[],mesh_reconstruction=[])
SAVE=ROOT/'target/t15b-profile/taper-results.json'
def emit(kind,row): print(kind,json.dumps(row),flush=True)
def save(): SAVE.write_text(json.dumps(RESULT,indent=2),encoding='utf-8')

def chord_at_y(y,ar,lam):
    # Caughey S=b*(c_root+c_tip)/2; normalize S/b=1 m, b=AR m.
    root=2*CONFIG['mean_geometric_chord_m']/(1+lam)
    return root*(1-(1-lam)*np.abs(2*y/ar))

def taper_mesh(ar,ny,nx,ellipse,lam):
    # Only geometry is adapted; archive keeps all influences/solves/drag logic.
    edges=-ar/2*np.cos(np.linspace(0,np.pi,ny+1))
    edges[ny//2]=0.0
    chords=chord_at_y(edges,ar,lam)
    leading=chords/4 # body x forward: x_quarter=x_LE-c/4=0.
    ya=np.repeat(edges[:-1],nx); yb=np.repeat(edges[1:],nx)
    ca=np.repeat(chords[:-1],nx); cb=np.repeat(chords[1:],nx)
    lea=np.repeat(leading[:-1],nx); leb=np.repeat(leading[1:],nx)
    bound=np.tile((np.arange(nx)+.25)/nx,ny)
    control=np.tile((np.arange(nx)+.75)/nx,ny)
    xa=lea-bound*ca; xb=leb-bound*cb
    xc=(lea+leb-control*(ca+cb))/2
    yc=(ya+yb)/2
    area=float(np.trapezoid(chords,edges))
    return xa,ya,xb,yb,xc,yc,edges,area

def tapered_lifting_line(ar,terms,lam):
    # The archived MIT collocation formula, with its rectangle chord replaced
    # by the requested exact straight-taper chord. No numerical kernel change.
    theta=np.arange(1,terms+1)*np.pi/(2*terms)
    n=np.arange(1,2*terms,2)
    chord=chord_at_y(ar/2*np.cos(theta),ar,lam)
    sines=np.sin(theta[:,None]*n)
    matrix=sines*(4*ar/(CONFIG['section_slope_per_rad']*chord[:,None])
                  +n/np.sin(theta[:,None]))
    a=np.linalg.solve(matrix,np.full(terms,CONFIG['alpha_rad']))
    cl=np.pi*ar*a[0]
    cd=np.pi*ar*np.dot(n,a*a)
    return dict(cl=float(cl),cdi=float(cd),e=float(cl*cl/(np.pi*ar*cd)))

native_mesh=VERIFIER.make_mesh
start=time.perf_counter()
for lam in CONFIG['taper_ratios']:
    for ar,ny in CONFIG['cases']:
        previous=None
        for terms in CONFIG['fourier_terms']:
            ll=tapered_lifting_line(ar,terms,lam)
            row=dict(taper_ratio=lam,ar=ar,terms=terms,**ll)
            if previous is not None:
                for k in ('cl','cdi','e'):
                    row['relative_'+k+'_last_change']=abs(ll[k]/previous[k]-1)
            RESULT['llt_convergence'].append(row)
            previous=ll
            emit('LLT',row)
        mesh=taper_mesh(ar,ny,CONFIG['nx'],False,lam)
        edges=mesh[6]; widths=np.diff(edges)
        local_chord=chord_at_y((edges[:-1]+edges[1:])/2,ar,lam)
        root_chord=2*CONFIG['mean_geometric_chord_m']/(1+lam)
        tip_chord=lam*root_chord
        if lam==1.0:
            original=native_mesh(ar,ny,CONFIG['nx'],False)
            # Uniform quarter-chord translation of the old rectangle; record,
            # do not assert, reconstruction and previously reported outputs.
            differences=[]
            for i in range(7):
                shift=.25 if i in (0,2,4) else 0.0
                differences.append(float(np.max(np.abs(mesh[i]-original[i]-shift))))
            RESULT['mesh_reconstruction'].append(dict(ar=ar,
                max_coordinate_difference_after_translation=max(differences),
                area_difference=mesh[7]-original[7]))
        VERIFIER.make_mesh=lambda a,n,c,e,chosen=lam:taper_mesh(a,n,c,e,chosen)
        try: v=VERIFIER.vlm(ar,ny,nx=CONFIG['nx'])
        finally: VERIFIER.make_mesh=native_mesh
        row=dict(taper_ratio=lam,ar=ar,ny=ny,nx=CONFIG['nx'],
            root_chord_m=root_chord,tip_chord_m=tip_chord,
            span_m=ar,area_m2=mesh[7],actual_ar=ar*ar/mesh[7],
            cl_vlm=v['cl'],cdi_vlm=v['cdi'],e_vlm=v['e'],
            cl_llt=ll['cl'],cdi_llt=ll['cdi'],e_llt=ll['e'],
            relative_cl_gap=abs(v['cl']/ll['cl']-1),
            relative_cdi_gap=abs(v['cdi']/ll['cdi']-1),
            relative_e_difference=v['e']/ll['e']-1,
            same_cl_target=ll['cl'],
            alpha_vlm_at_same_cl_rad=CONFIG['alpha_rad']*ll['cl']/v['cl'],
            max_panel_width_m=float(widths.max()),
            max_panel_width_over_mean_chord=float(widths.max()/CONFIG['mean_geometric_chord_m']),
            tip_panel_width_m=float(widths[-1]),
            tip_panel_width_over_tip_chord=float(widths[-1]/tip_chord),
            max_width_over_local_midpoint_chord=float(np.max(widths/local_chord)),
            vlm_max_absolute_collocation_residual=v['residual'],
            vlm_wall_s=v['seconds'])
        RESULT['comparison'].append(row)
        emit('COMPARISON',row)
        save()

for lam in CONFIG['taper_ratios']:
    selected=[r for r in RESULT['comparison'] if r['taper_ratio']==lam and r['ar'] in CONFIG['fit_ar']]
    ar=np.array([r['ar'] for r in selected],dtype=float)
    gap=np.abs([r['relative_e_difference'] for r in selected])
    # NIST least squares. Power fit minimizes LOG residuals; inverse-log fit
    # minimizes ORIGINAL gap residuals. They have different objectives.
    slope,log_c=np.linalg.lstsq(np.column_stack((np.log(ar),np.ones_like(ar))),np.log(gap),rcond=None)[0]
    power=np.exp(log_c)*ar**slope
    a,b=np.linalg.lstsq(np.column_stack((np.ones_like(ar),1/np.log(ar))),gap,rcond=None)[0]
    inverse_log=a+b/np.log(ar)
    fit=dict(taper_ratio=lam,power_C=float(np.exp(log_c)),power_p=float(-slope),
        inverse_log_a=float(a),inverse_log_b=float(b),
        power_original_scale_rmse=float(np.sqrt(np.mean((gap-power)**2))),
        power_log_scale_rmse=float(np.sqrt(np.mean((np.log(gap)-np.log(power))**2))),
        inverse_log_original_scale_rmse=float(np.sqrt(np.mean((gap-inverse_log)**2))),
        residuals=[dict(ar=int(x),observed_gap=float(g),power_prediction=float(p),
            power_original_residual=float(g-p),power_log_residual=float(np.log(g)-np.log(p)),
            inverse_log_prediction=float(z),inverse_log_residual=float(g-z))
            for x,g,p,z in zip(ar,gap,power,inverse_log)])
    RESULT['fits'].append(fit)
    emit('FIT',fit)
RESULT['archived_sha256_after']=hashlib.sha256(PATH.read_bytes()).hexdigest()
RESULT['total_wall_s']=time.perf_counter()-start
save()
print('SAVED',SAVE,flush=True)
```
