# Other NASA sources: coverage and sign evidence

## Scope, identity checks and access record

The owner supplied the three exact NTRS identifiers below because
`docs/PROJECT_BRIEF.md` is absent from this checkout. No substitute report has
been selected. CR-96008 remains the primary inventory in
[cr96008.md](cr96008.md). This file records available quantities and limitations;
it does not digitize curves or transcribe coefficient, geometry, mass or inertia
values. Coverage ranges, report dates and page/condition locators are metadata.

All three exact PDFs were opened on 2026-11-19. Relevant source pages were read
as text and the title, axes, control definitions and selected tables/figures
were checked against rendered page images. **Could not open: none of the three
requested PDFs.** The web preview rejected the large SL56H06 file and one
catalog request timed out, but the exact NASA PDF downloads succeeded.

| Exact source | Retrieved title, authors and date | Aircraft identity actually established |
|---|---|---|
| [NTRS 20090026345](https://ntrs.nasa.gov/citations/20090026345), [PDF](https://ntrs.nasa.gov/api/citations/20090026345/downloads/20090026345.pdf), NACA RM SL56H06 | **Longitudinal and Lateral Stability Characteristics of a 0.04-Scale Model of the Lockheed F-104A Airplane at Mach Numbers of 1.82 and 2.01. Coord. No. AF-245**, M. Leroy Spearman and Cornelius Driver (cover / PDF 1; title and authors also p. 1 / PDF 2). NTRS publication date: **August 6, 1956**. The scan has an **August 30, 1956** cover stamp and an author/laboratory date of **July 20, 1956** on p. 13 / PDF 14; these are recorded separately, not silently reconciled into one issue date. | Explicitly a Lockheed F-104A **scale model**, with modified tails, ventrals and external-store variants (pp. 1-4). Matches the owner's model-study description; it is not full-scale flight evidence. |
| [NTRS 19630004010](https://ntrs.nasa.gov/citations/19630004010), [PDF](https://ntrs.nasa.gov/api/citations/19630004010/downloads/19630004010.pdf), NASA MEMO 2-2-59H | **Flight-Determined Stability and Control Derivatives of a Supersonic Airplane With a Low-Aspect-Ratio Unswept Wing and a Tee-Tail**, William H. Andrews and Herman A. Rediess, **April 1959** (cover / PDF 1; title and memorandum number p. 1 / PDF 3). The laboratory signature date is **October 29, 1958**, p. 15 / PDF 17, not the publication date. | An unnamed supersonic fighter with that wing/tail arrangement, turbojet/afterburner, all-moving horizontal tail and optional ventral fin (pp. 2, 5-6; Fig. 1, p. 21 / PDF 23). It matches the supplied **F-104-type** description, but the report text does not identify an F-104 production variant or serial number. That identity is not supplied from memory. |
| [NTRS 19660022438](https://ntrs.nasa.gov/citations/19660022438), [PDF](https://ntrs.nasa.gov/api/citations/19660022438/downloads/19660022438.pdf), NASA MEMO 2-3-59L | **Comparison of Control-Fixed Stability Derivatives for Two Supersonic Fighter Airplanes as Determined From Flight and Wind-Tunnel Tests**, Harold L. Crane, Milton D. McLaughlin and Jack A. White, **April 1959** (cover / PDF 1; title/authors p. 1 / PDF 2). Laboratory signature date: **October 27, 1958**, p. 16 / PDF 17. | **Scope mismatch for F-104/Navion data:** the subjects are airplane A, a swept high-wing/low-tail fighter, and airplane B, a swept midwing fighter with its horizontal tail below the wing (pp. 6-7; drawings pp. 22-23 / PDF 23-24). Neither is identified as F-104 or Navion. The title matches the supplied comparison description; treating its values as F-104 data would be unsupported. No names are guessed from photographs. |

Printed-page citations below are local to each report. For SL56H06, printed
p. 1 is PDF 2; figures with faint/missing page numerals are also given explicit
PDF locators. For MEMO 2-2-59H, printed p. 1 is PDF 3. For MEMO 2-3-59L,
printed p. 1 is PDF 2. These offsets were checked on the page images.

## NACA RM SL56H06: F-104A model study

### What it adds to CR-96008

| Addition | Exact place to look | Qualification against the primary inventory |
|---|---|---|
| Detailed wing, horizontal/vertical-tail, ventral-fin, fuselage and store geometry; airfoil descriptions; dimensioned component drawings. | Table I, pp. 14-15 / PDF 15-16; Figs. 2-3 / PDF 21-26; model description pp. 4-5. | More geometric definition than CR-96008's reference S/b/c and three-view (CR pp. 74, 76). These are **model** dimensions and model configurations; scale conversion and agreement with a selected full-scale variant have not been established. |
| Longitudinal coefficient curves across alpha/lift and component-on/off combinations, including nonlinear moment behavior. | Table II figure index, pp. 16-17; Figs. 5-7 / PDF 32-37; discussion pp. 6-7. | Adds curve shape and component effects absent from CR VII-B's isolated operating points (CR p. 77). Do not replace the report's plotted CX with a guessed CD convention. |
| Lateral/directional force and moment curves, derivative summaries, tail/ventral changes and store effects. | Figs. 8-16 / PDF 38-61; store Figs. 17-19 / PDF 62-66; summary Figs. 20-24 / PDF 67-71; indexed in Table II, pp. 16-17. | Adds lateral model evidence where CR explicitly says lateral F-104 data are unavailable (CR p. 74). Configuration effects are not interchangeable with the CR loading/tank cases. |
| Test setup, tunnel conditions, force-balance measurement, corrections and an error discussion. | pp. 4-6, especially p. 5. | Adds experimental provenance absent from CR's "Unpublished Data" reference (CR p. 74). The report's measurement-error estimates are not project acceptance tolerances. |

### Coverage, units and limitations

- **Coverage:** two tunnel Mach numbers, **1.82 and 2.01**, not a Mach sweep or
  flight-altitude grid. Table II lists configuration-specific alpha/beta ranges;
  basic longitudinal runs extend approximately **-4 to 21 degrees alpha**,
  with lateral sweeps approximately **-4 to 14 degrees beta** at selected
  alphas. These are table-index ranges, not a digitized envelope or evidence
  for every alpha/beta combination (pp. 5-6, Table II pp. 16-17).
- **Units:** geometry in inches/feet and square inches/feet as individually
  labeled; angles in degrees; tunnel stagnation temperature/dewpoint in
  degrees F and pressure in absolute lb/square inch. Force/moment coefficients
  use qS, qSb or qS times mean geometric chord; retain the source's **mean
  geometric chord** designation (pp. 2-5, Table I pp. 14-15; Fig. 2 caption).
- **Important discrepancy:** the association between Mach and the two
  Reynolds numbers is reversed between the introduction on **p. 2** and the
  test-conditions table on **p. 5**. Both readings are visible in the scan.
  The NTRS abstract alone cannot resolve the primary-source disagreement.
  Leave that association unresolved before later transcription (pp. 2, 5).
- **Configuration/propulsion limitations:** inlets were faired and internal
  ducting was not modeled; the horizontal tail was fixed at its stated
  incidence. Drag was adjusted to a specified base-pressure condition.
  This is not a documented full-scale engine-off/windmilling drag condition
  (pp. 4-5).
- **Still missing for H3/H4:** full-scale loading/inertia, a matched altitude
  history, dynamic derivatives, conventional-rudder/aileron control sweeps,
  engine-off stall speed, glide/sink measurements and a matched trim-control
  schedule. The contents are static model tests, with the fixed-tail and
  configuration scope identified on pp. 4-6 and Table II pp. 16-17; they do
  not establish these missing full-aircraft measurements.

### Axes and control signs

The source explicitly assigns longitudinal lift/drag/pitch coefficients to
**stability axes**, and sideforce/roll/yaw coefficients to **body axes**, with
the moment reference at **25 percent wing mean geometric chord**. Fig. 1(a/b)
draws the axes, relative wind and positive moment arrows. Carry both the frame
and moment-reference location into any later transformation; this is not one
uniform body-axis table (p. 2; Fig. 1 / PDF 19-20).

**Axial-force sign caution:** p. 2 defines CX using X/qS, p. 3 describes X as
opposite to drag at zero sideslip, while the longitudinal figures use CX as
their plotted ordinate. Preserve the symbols and inspect the sign mapping
explicitly before equating that ordinate to either project body-X coefficient
or positive drag; no sign conversion is selected by this inventory (pp. 2-3;
Figs. 5-7 / PDF 32-37).

**Control signs not established:** horizontal-tail incidence is defined as a
degree-valued symbol, but the tested tail is fixed and no positive
elevator/aileron/rudder travel convention is supplied by the symbol list or
test matrix. Do not treat tail-presence comparisons as an elevator derivative,
or borrow D6 signs to fill this source's omissions (pp. 3-4, Table II pp. 16-17).

## NASA MEMO 2-2-59H: flight-determined derivatives

### What it adds to CR-96008

| Addition | Printed page / PDF locator | Qualification |
|---|---|---|
| Physical geometry, airfoil descriptions, tail/control dimensions, deflection limits, empty/takeoff loading labels and CG information. | Table I, pp. 18-19 / PDF 20-21; physical description pp. 5-6; Fig. 1 p. 21 / PDF 23. | Extends the primary F-104 section's reference geometry, but the unnamed test-airplane identity and configuration must be matched before combining values with CR VII-A (CR pp. 74, 76). |
| Roll, pitch and yaw inertias and xz product plotted versus weight, with CG information. | p. 6; Fig. 2 p. 22 / PDF 24. | Helps locate the roll/yaw inertia information missing in CR VII-A. The source explicitly says the main inertias are **manufacturer estimates** and the product is calculated from those estimates/principal-axis inclination. These are not measured-inertia validation data. |
| Trim alpha and stabilizer angle versus Mach; transient time histories and longitudinal derivative curves. | Fig. 3 p. 23 / PDF 25; Figs. 4-7 pp. 24-27 / PDF 26-29; analysis pp. 7-10. | Adds a trim-control schedule and measured-response provenance absent from CR's isolated alpha table and transfer-function factors (CR pp. 76-79). Longitudinal flight/tunnel comparisons in Fig. 7 are corrected to a stated common CG, not automatically the changing CGs in CR VII-A (p. 8; Fig. 7 p. 27). |
| Static lateral/directional derivatives, dynamic combinations, aileron and separate yaw-damper effectiveness; wind-tunnel comparisons. | Figs. 13-18 pp. 34-39 / PDF 36-41; methods pp. 8-10; discussion pp. 11-14. | Adds data families absent from the primary F-104 section. Static plots include ventral-on/off distinctions; preserve each legend rather than merging them into a single aircraft curve (Fig. 13 p. 34; CR p. 74). |
| Instrumentation, phase/position corrections, identification assumptions, analog-response checks and uncertainty discussion. | pp. 6-10, 12-14. | Helps distinguish measured transients, estimated inputs and adjusted simulation values, which the primary F-104 tables do not separate (CR pp. 74-79). |

### Coverage and interpretation

Flight data cover **Mach 0.88-2.08**, near **40,000 ft pressure altitude**, with
the stated altitude excursion **38,000-42,000 ft**, in the **1 g trim**
angle-of-attack range. Controls are held fixed during the transient except
for the initiating pulse; installed automatic dampers were not activated,
although the separate yaw-damper surface was manually pulsed for the
investigation (pp. 2, 6-7; pressure-altitude definition p. 3).

This supplies Mach-dependent data at a narrow altitude band, not independent
altitude derivatives or a rectangular altitude/Mach grid. It does not identify
engine-off test conditions, full-aircraft drag polars, stall/max-lift runs or
glide/sink records; its documented tests are trim disturbances and transient
response measurements (pp. 2, 5-10; Figs. 3-18, pp. 23-39).

The longitudinal damping result is a **sum** involving pitch-rate and
alpha-dot derivatives. The lateral yaw-damping result is a **combination**
involving yaw-rate and beta-dot derivatives. Some cross-rate terms are
manufacturer estimates or assumed from unpublished tunnel data. None may be
silently promoted to an independently flight-measured individual derivative
(pp. 8-10; Figs. 6, 14, pp. 26, 35).

The roll-derivative discussion explores sensitivity to estimated roll inertia;
the paper does not establish that the alternative inertia is the corrected
truth. Likewise, solid analog-simulation symbols are adjusted values used to
match time histories, not an additional independent flight measurement set
(pp. 10, 12-14; Figs. 6, 13-15).

### Units, axes and exact control definitions

Results are explicitly referred to the **body system of axes**, aligned with
the instrumentation, including the lateral analysis. They must not be merged
with CR's fixed-stability-axis derivatives without preserving nominal alpha,
reference chord, CG and any required rotation (pp. 2, 6, 8; CR A-1, B-1).
The source identifies the frame but does not supply a complete standalone
positive-axis/control-arrow diagram equivalent to CR Appendix A in its symbol
list and aircraft drawing; retain that limitation rather than infer every
direction from the phrase "body axes" (pp. 2-5; Fig. 1 p. 21).

Lengths/speeds, pressure altitude, mass/density and inertia use ft, ft/s, ft,
slugs, slug/ft cubed and slug-ft squared. Dynamic pressure is lb/square ft;
control angles are degrees, angular rates radians/s. **Angular coefficient
slopes in the plots are often per degree, while damping/rate derivatives are
per radian with nondimensional rate factors.** Verify each ordinate rather
than apply one angular conversion to an entire figure (symbols pp. 2-5;
Figs. 6, 13-15, pp. 26, 34-36).

| Surface/symbol | Exact positive direction stated | Sign-check limitation |
|---|---|---|
| Stabilizer, it | Trailing edge down (p. 3 / PDF 5). | It is the **all-moving horizontal tail**, not an independently modeled elevator hinged to a fixed tail (p. 5). The plotted stabilizer pitch-effectiveness sign is available in Fig. 6, p. 26. |
| Aileron, delta-a | Left aileron deflected down (p. 4 / PDF 6). | Total aileron deflection has a separate symbol. Preserve single-surface versus total deflection normalization; right-side travel is not separately defined by that sentence (p. 4; Fig. 15 p. 36). |
| Yaw-damper surface, delta-yd | Deflected to the left (p. 4 / PDF 6). | The wording does not explicitly name a leading/trailing edge. The conventional rudder is a **separate surface**; neither its positive travel nor its effectiveness is established by this yaw-damper definition (pp. 4-5; Table I p. 19). |
| Conventional rudder | No explicit positive-travel definition located in the symbol list or geometry table (pp. 2-5, 19). | Do not relabel Cn-delta-yd as Cn-delta-r or claim it independently proves the project's rudder sign. |

In particular, Fig. 15 plots the yaw-damper coefficient with its own source
sign, while CR Appendix A defines positive rudder by a negative yaw moment.
The different surface names and incomplete travel-direction description must
be resolved before using the former as a D6 rudder test; no equality of those
conventions is asserted here (MEMO pp. 4-5, 9, 36; CR A-4).

## NASA MEMO 2-3-59L: two-fighter comparison

### Aircraft-scope finding

This exact identifier matches the requested **two-fighter comparison title**,
but it does **not** provide an identified F-104/Navion data set. A has a swept
high wing and low tail; B has a swept midwing and a horizontal tail below it.
The paper labels them A/B. Their parameters, photos and drawings must retain
those labels; no aircraft names or crosswalk to CR's F-104 are inferred
(pp. 6-7, Tables I/II pp. 18-19, Figs. 1-4 pp. 20-24).

### What it adds, and what it cannot fill

| Evidence added | Source pages | Relevance to CR-96008 gaps |
|---|---|---|
| Matched flight/tunnel plots of lift slope, pitch slope, static-margin-related moment/lift slope and longitudinal damping; static and dynamic lateral derivatives. | Figs. 8-13 pp. 28-33 / PDF 29-34; definitions pp. 2-6; discussion pp. 12-16. | Comparison-method evidence beyond CR's unexplained F-104 tables. These values belong to A/B, so they fill **no direct F-104 or Navion numerical gap** (CR pp. 74, 100; this memo pp. 6-7). |
| Configuration matching, aeroelastic corrections, inlet/engine mass-flow corrections, and a tunnel-interference exclusion. | pp. 7-8, 12-13. | Documents reasons to qualify any later model/flight comparison. It does not supply an F-104 correction or a universal correction/tolerance transferable to CR (this memo pp. 6-7, 12-13). |
| Flight instruments, phase/amplitude corrections, assumed identification terms and control-motion effects. | pp. 8-12, 14-16. | Shows limitations of treating every flight-derived derivative as direct measurement of a rigid, control-fixed airframe; not substitute aircraft input data. |
| Geometry, control descriptions, test weight/CG and inertia ranges for A/B. | pp. 6-8; Tables I/II pp. 18-19; drawings pp. 22-23. | Useful inventory categories, but not F-104/Navion geometry/loading/inertia and not evidence to close H3/H4 for those aircraft. |

Flight coverage is approximately **Mach 0.7-1.44 for A** and **0.7-1.23 for B**,
near **35,000 ft**, identified as pressure altitude in the discussion. These
are trim-level-flight disturbances with augmentation/yaw damping disabled as
described for each aircraft. B is explicitly described as using **power for
level flight**. No power-off comparison is established (pp. 1, 6-8, 12, 15-16).

The tunnel comparison has a stated **Mach 1.03-1.12** interval where measured
tunnel data were excluded because wall-reflected disturbances were large.
Interpolated curves are distinguished in the figure legends. This is not
continuous measured tunnel coverage across that interval (p. 7; Figs. 8-9,
pp. 28-29).

The title's control-fixed qualification also needs the paper's own caveat:
some B pitch-damping results changed when the pilot relaxed grip and the
response-feel system moved the control. Preserve those subsets/legends in any
later use (pp. 14, 16; Fig. 9 p. 29).

### Axes, units and control signs

The reported derivatives use **stability axes**, explicitly a right-handed
system through CG with X along the projection of the relative airstream on
the symmetry plane. Instrument measurements used body axes and were
transferred to stability axes during analysis. These two provenance layers
must not be confused (pp. 2, 9, 11).

The symbol definitions give positive bank as right wing down and positive
yaw as nose right. Length/speed, mass/density, dynamic pressure and inertia
are in ft/ft/s, slugs/slug per cubic ft, lb per square ft and slug-ft squared;
tables label the inertias as stability-axis quantities. Coefficient angular
slopes are per radian, although some angles are displayed in degrees
(pp. 2-6; Tables I/II pp. 18-19).

**No complete positive elevator/aileron/rudder convention is established by
this source.** The geometry tables provide travel limits, including some
physical up/down labels, and the text identifies different mechanisms for A
and B. Those are not definitions of a common signed deflection variable.
Do not use them to fill D6's missing rudder evidence or assign F-104 control
signs (pp. 6-7; Tables I/II pp. 18-19).

## Remaining source work before transcription

- Resolve the primary F-104 condition-Mach discrepancy and Navion reference
  area discrepancy; these reports do not establish a crosswalk that repairs
  either entry (CR pp. 74, 76-79, 100-101; source identities above).
- Resolve SL56H06's Reynolds/Mach pairing and distinguish its catalog date
  from the scanned date stamps before freezing a provenance record
  (SL56H06 cover, pp. 2, 5, 13).
- Establish the unnamed flight airplane's exact variant/configuration match,
  preserve body versus fixed-stability axes and CG corrections, and obtain
  explicit conventional-rudder sign evidence. Stabilizer/aileron/yaw-damper
  definitions alone do not close every D6 sign check (MEMO 2-2-59H pp. 2-8,
  19; CR A-1, A-4, B-1).
- Obtain actual engine-off stall/glide/trim comparison records and matching
  configurations/loading. None of these inspected reports establishes such
  an F-104/Navion validation set (CR pp. 74-79, 100-103; SL56H06 pp. 4-6;
  MEMO 2-2-59H pp. 2, 5-10; MEMO 2-3-59L pp. 6-8).

Plots have not been digitized, no coefficients have been converted, and no
source-axis transformation or control-sign test has been implemented.
