# T14 directed horseshoe influence coefficients

## Formula sources (checked online 2026-11-14)

- [Sheffield and Deffenbaugh, NASA CR-3208, Appendix A, printed p. 23 / PDF p. 29](https://ntrs.nasa.gov/api/citations/19800005792/downloads/19800005792.pdf#page=29):
  Biot-Savart integral A-1, straight finite segment A-2, semi-infinite-ray limit
  A-4. Only the inviscid off-filament relations are used. Its optional physical
  cutoff/core model is not adopted; this task instead specifies an error on a
  filament. Some extracted equation characters are garbled; the vector form
  below is the unit-direction evaluation of the integral, cross-checked by
  independent scalar angle-integral test cases.
- [Drela, JVL 2.20 Formulation, sections 3.1 and 4.2](https://web.mit.edu/drela/Public/web/jvl/jvl.pdf):
  horseshoe lattice with bound vortex at panel quarter chord and control at
  three-quarter chord; velocity contributions add by Biot-Savart integration.
  Our coordinate sign conversion and singularity policy are explicit below.

No aircraft measurements/constants were imported. These are mathematical
influence coefficients, not validation of the hero aircraft or a vortex-core fit.

## Orientation, units and singularities

Body axes remain x forward, y right, z down. On a wing a positive horseshoe has
its bound segment directed from left to right (+y), an incoming ray from
downstream infinity to the start, and an outgoing ray from the end to downstream
infinity. Downstream is -body x for the linearized wing problem. A positive
unit circulation produces downwash (+z) aft inside the horseshoe.

The functions return velocity per circulation in 1/m; multiply by circulation
in m^2/s to obtain m/s. At unit circulation 1 m^2/s, the numerical values are
the requested unit-strength velocities. No flight speed/density is selected.

For segment a->b, r1=p-a, r2=p-b, u=(b-a)/|b-a|, h=|u cross r1|:

```text
v/Gamma = ((u cross r1)/h^2)
          * [u dot (r1/|r1|) - u dot (r2/|r2|)] / (4*pi)
```

For a ray a->infinity along unit u, the finite-segment limit gives:

```text
v/Gamma = ((u cross r)/h^2) * [1+u dot (r/|r|)] / (4*pi)
horseshoe = finite(a,b) - ray(a,u) + ray(b,u)
```

Normalizing the cross product before dividing by h avoids squaring a very small
distance. For upstream ray points the identity 1+cos=sin^2/(1-cos) avoids
cancellation; this is an algebraic rewrite, not a physical cutoff.

Computed exact collinearity inside/on either endpoint of a finite segment,
or at/downstream of a ray start, returns an explicit VortexError::Singular.
Collinear points outside a finite segment or behind a ray return zero velocity
for that filament. No NaN, self-induced placeholder velocity, guessed core
radius or near-filament smoothing is returned. Very near off-filament velocities
can be large; only non-finite arithmetic is rejected. Degenerate segments,
non-finite input and nonunit ray directions error. The unit-direction tolerance
is floating-point roundoff (64 machine epsilons), not aerodynamic validity.

## Independent scalar hand checks

Every coordinate and expected result in horseshoe.json is SYNTHETIC TEST DATA,
with every numeric path ledgered in horseshoe_values.md. New test settings and
inputs are in vortex_test_values.md. Expected values were generated without
calling the Rust implementation. Each case stores its scalar hand expression.

For a=(0,-1,0), b=(0,1,0), wake=(-1,0,0), k=1/(4*pi):

- p=(-1,0,0): bound contributes sqrt(2)*k in z; rays contribute
  2*(1+1/sqrt(2))*k, total (1+sqrt(2))/(2*pi).
- p=(-1,0,1): bound=(1,0,1)*k/sqrt(3); ray sum=(0,0,1+1/sqrt(3))*k.
  Thus vx=k/sqrt(3), vy=0, vz=k*(1+2/sqrt(3)).
- p=(-1,2,2): R1=sqrt(14), R2=sqrt(6), u=3/R1-1/R2.
  Bound=k*(2u/5,0,u/5); outgoing right=k*(0,2,-1)*(1+1/R2)/5;
  incoming left=-k*(0,2,-3)*(1+1/R1)/13. All components are positive.
- p=(-1,2,0): the scalar z sum is
  k*(3/sqrt(10)-1/sqrt(2)+(1+1/sqrt(10))/3-(1+1/sqrt(2))) < 0,
  checking outside upwash independently of the interior downwash cases.

The directed long segment (0,-L,0)->(0,L,0) at p=(r,0,0) has
vz=-L/(2*pi*r*sqrt(L^2+r^2)), from the endpoint-angle integral. As L grows it
approaches the 2D result -1/(2*pi*r); at r=2 the test reaches <1e-10 relative
difference with the ledgered sequence. Mirroring a panel AND reversing endpoint
order keeps positive circulation and preserves vx/vz while reversing vy.

Seven tests check these values/signs, reversed circulation orientation, finite
and ray long-distance limits, on-filament errors/collinear extensions, mirrored
panels and invalid inputs. Absolute hand-value tolerance is 2e-14 in 1/m;
all settings are numerical synthetic test choices, not physical validation
thresholds. Before implementation the test failed on missing estimator::vortex.
No dependency, physical convention, T10 or T11 work is part of this task.

Final T14 gates: formatting and Clippy with warnings denied pass; all 159
workspace tests pass, 0 failed/ignored. Existing filesystem setting
CARGO_INCREMENTAL=0 is retained. Files: estimator src/lib.rs, src/vortex.rs,
tests/vortex.rs and three fixture/ledger files; docs/TASKS.md,
docs/ASSUMPTIONS.md, docs/LEARNING.md and this reference note.
