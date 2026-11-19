# Independent verification: gap versus aspect ratio

This is an **informational record supplied by the owner**, not a new assertion
or real-aircraft validation. All wings/conditions are SYNTHETIC TEST VALUES.
Relative gaps are abs(VLM-LLT)/abs(LLT); percentages below are copied exactly
from the supplied independent refined sweep, not fitted expected outputs for
repository tests. Full span/chord panel counts are explicit.

| AR | Cosine-span / uniform-chord mesh | Relative CL gap | Relative CDi gap |
|---|---|---|---|
| 10 | 128x8 | 3.787766% | 11.840624% |
| 20 | 256x8 | 1.877427% | 8.701523% |
| 40 | 512x8 | 0.929194% | 6.969987% |
| 80 | 1024x8 | 0.461799% | 5.878795% |
| 160 | 2048x8 | 0.230329% | 5.115927% |

The supplied fits over **AR20..160** are CL p=1.00897, CDi p=0.25445 for
relative gap approximately C*AR^(-p), using least squares in log coordinates.
These describe a finite set of computed model differences on the stated mesh
sequence. They do not establish an infinite-AR law or identify a physical
cause or bug. The explanatory question remains unresolved in QUESTIONS.md.

## Independent script and reproduction scope

The verifier is preserved at independent/independent_vlm_llt.py with an origin
header. It imports no repository code and is NOT part of the build or CI.
Body SHA-256: `e1f336b81123625ce32a20d635be2f753395b227a3c50523a458da4d2b82a38d`. Byte comparison and Python AST comparison match the
original; its executable logic was not edited. Import inventory and every
numeric literal are recorded in independent/NUMERICAL_LEDGER.md. NumPy was
already present in the verifier environment; no dependency was added.

The original built-in `sweep` mode uses a COARSER mesh sequence than this
supplied refined table. That behavior is intentionally unchanged. Reproduce
this table's mesh configuration by calling its `vlm(ar, ny, nx=8)` function
with the five pairs shown above and converged `lifting_line` references;
do not claim the unmodified CLI `sweep` mode reproduces this exact table.
Part 3's driver/term-convergence record is in vlm_efficiency_gap.md.
Original evidence is in target/verifier/scratch/refined-sweep.log; its printed
fit values round to the supplied figures above. The supplied percentages
remain the authority for this record.

## Mesh-specific prediction-band qualification

The old zero-compatible extrapolation assertions and their control test were
deliberately replaced after leave-one-out fits showed that the verdict depends
on the AR subset. Those fits remain informational in vlm_subset_fits.md.

Richardson prediction bands from the prior acceptance review apply ONLY to
that geometry and Nx=1 mesh sequence. At AR6, the Nx1 prediction band is
[0.9987106156690151, 0.9988950560535235]. With Nx=8, the AR6 estimate from
Ny256/512/1024 is **0.997976872**, outside that band. It must not be judged with
an Nx1 band or used to enlarge that band. This is a mesh-specific numerical
comparison, not a failed physical efficiency bound; the Munk ceiling remains1.
The fresh production value and independent value agree at the quoted precision.

## Record-value ledger

Every table row contains supplied SYNTHETIC benchmark AR/mesh settings and
independently computed informational outputs. The fit range20..160 and
exponents1.00897/0.25445 are owner-supplied fit results. Conditions follow the
unchanged verifier: alpha=.05 rad, rectangular chord1 m/span=AR m, ideal section
slope2*pi, no twist/zero-lift angle, planar zero-offset geometry. The Nx1 band
and Nx8 estimate are explicitly labeled prior/current synthetic-grid numerical
records, not new tolerances. Mesh counts128/256/512/1024/2048, chord count8,
AR10/20/40/80/160, and the band example AR6/Nx1/Nx8 and triples256/512/1024
are all enumerated above; every script literal has a separate ledger.

Sources for equations: [MIT Lecture 9](https://web.mit.edu/16.unified/www/SPRING/fluids/Spring2008/LectureNotes/f09.pdf),
[Drela JVL](https://web.mit.edu/drela/Public/web/jvl/jvl.pdf), and the existing
estimator reference notes. Log-linear fit algebra follows
[NIST least squares](https://www.itl.nist.gov/div898/handbook/pmd/section4/pmd431.htm).
