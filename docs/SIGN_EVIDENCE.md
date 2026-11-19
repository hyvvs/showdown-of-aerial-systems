# Control-sign evidence against D6

Documentation audit, 2026-11-19. The comparison target is [CONVENTIONS.md](CONVENTIONS.md), §“Forces and moments”, D6: positive elevator trailing edge down gives negative pitch; positive paired aileron means left trailing edge down/right up and positive roll; positive rudder trailing edge left gives negative yaw. This audit does not change that decision (CONVENTIONS D6).

**How to read the verdict.** “Agrees” identifies the supported physical direction or moment sign; a qualified agreement does not prove an aircraft's command mapping. “Unclear” means the cited passage does not establish the complete convention. Printed report pages are used, with explicit PDF positions for unnumbered figures. A web manual has sections rather than invented page numbers. Source-axis definitions alone do not prove a control-deflection sign (CR A-1-A-4; H pp. 3-5; JSBSim §“Conventions”).

## Sources, editions and access

| Key / exact source opened | Retrieved identity/date and access evidence |
|---|---|
| [CR, NTRS 19690022405](https://ntrs.nasa.gov/api/citations/19690022405/downloads/19690022405.pdf) | Gary L. Teper, *Aircraft Stability and Control Data*, NASA CR-96008, April 1969 (cover/PDF 1). Relevant scans opened: pp. 74-79, 100-103, A-1-A-7, B-1; no requested CR page remained inaccessible. |
| [S, NTRS 20090026345](https://ntrs.nasa.gov/api/citations/20090026345/downloads/20090026345.pdf) | Spearman/Driver, *Longitudinal and Lateral Stability Characteristics of a 0.04-Scale Model of the Lockheed F-104A Airplane at Mach Numbers of 1.82 and 2.01*, RM SL56H06 (cover/p. 1). Catalog date August 6, 1956; cover stamp August 30, author date July 20 on p. 13: distinct date evidence, not silently reconciled ([catalog](https://ntrs.nasa.gov/citations/20090026345); cover, p. 13). PDF opened; axes/model/test pages inspected. |
| [H, NTRS 19630004010](https://ntrs.nasa.gov/api/citations/19630004010/downloads/19630004010.pdf) | Andrews/Rediess, *Flight-Determined Stability and Control Derivatives of a Supersonic Airplane With a Low-Aspect-Ratio Unswept Wing and a Tee-Tail*, MEMO 2-2-59H, April 1959 (cover); author date October 29, 1958 (p. 15). PDF opened. The airplane is unnamed; F-104 production-variant identity is not established (pp. 2, 5-6). |
| [L, NTRS 19660022438](https://ntrs.nasa.gov/api/citations/19660022438/downloads/19660022438.pdf) | Crane/McLaughlin/White, *Comparison of Control-Fixed Stability Derivatives for Two Supersonic Fighter Airplanes as Determined From Flight and Wind-Tunnel Tests*, MEMO 2-3-59L, April 1959 (cover); author date October 27, 1958 (p. 16). PDF opened. Subjects are swept-wing airplanes A/B, not identified F-104/Navion data (pp. 6-7). |
| [JSBSim manual](https://jsbsim-team.github.io/jsbsim-reference-manual/user/concepts/flight-control-and-systems-modeling/#conventions) | *Flight Control and System Modelling*, live undated web chapter retrieved 2026-11-19, introduction and §“Conventions”; its [aerosurface diagram](https://jsbsim-team.github.io/jsbsim-reference-manual/assets/img/ac_aerosurface_deflections.svg) was opened and rendered for visual inspection. No dated edition is asserted. |
| [Caughey notes](https://courses.cit.cornell.edu/mae5070/Caughey_2011_04.pdf#page=11) | David A. Caughey, *Introduction to Aircraft Stability and Control*, Cornell course notes, 2011 (cover/PDF 1); §1.2.2, printed p. 5/PDF 11 opened and visually checked. Already cited in CONVENTIONS D6. |
| [Beard/McLain author-linked chapter 4 slides](https://drive.google.com/file/d/1BjJuj8QLWV9E1FX6sHVHXIGaIizUaAJ5/view) | *Small Unmanned Aircraft: Theory and Practice*, chapter 4 slide 8/PDF 8, “Control Surfaces – Conventional”; footer identifies Beard/McLain and Princeton 2012. Author [resource repository](https://github.com/byu-magicc/mavsim_public), README “Lecture slides”, links this file. Browser preview stalled, but the public PDF download opened and slide 8 was visually checked. The slide footer is not proof of the slide file's issue date or of the published book's exact passage. |
| Beard/McLain **published textbook** cited in CONVENTIONS D6 | The 2012 published chapter 4 passage was **not obtained/opened**; no page-level verdict is claimed. The author-linked work-in-progress second-edition draft was not used as a substitute (author resource README “Textbook”; CONVENTIONS D6's existing citation). The slides above are separate, accessible evidence. |

## Elevator / longitudinal control

| Source | Page / section and paraphrased evidence | Verdict against D6 |
|---|---|---|
| CR | A-4 defines positive elevator from trim by a nose-down moment for an aft surface; A-2 defines positive pitch as nose up. Physical trailing-edge direction is not specified there. | **Agrees on moment sign; physical travel unclear.** Applies to the appendix convention, not independent proof of a particular F-104/Navion linkage (pp. 74-79, 100-103). |
| S | Tail incidence is fixed in these tests; the axis/moment diagram does not define a signed elevator variable (pp. 2-5; Fig. 1, PDF 19-20; Table II pp. 16-17). | **Unclear.** Tail-on/off or incidence evidence is not an elevator sign test. |
| H | Stabilizer angle is positive trailing edge down (p. 3); the aircraft has an all-moving horizontal tail (p. 5), and Fig. 6 shows negative pitching effectiveness (p. 26). | **Agrees on travel and pitch direction for the stabilizer.** It is not a verified hinged-elevator/variant mapping. |
| L | Control mechanisms and up/down travel limits are described without a complete positive elevator-variable definition (pp. 6-7; Tables I/II pp. 18-19). | **Unclear.** Physical limits do not establish the signed variable. |
| JSBSim | §“Conventions”, diagram: positive elevator is drawn trailing edge down with negative pitching-moment change. | **Agrees.** Aircraft-specific cockpit-to-surface mapping still needs checking (chapter introduction). |
| Caughey | §1.2.2, p. 5/PDF 11: positive elevator is trailing edge down; aft-tail lift then gives negative pitch effectiveness. | **Agrees**, explicit physical direction and moment sign. |
| Beard/McLain slides | Chapter 4 slide 8: positive elevator is drawn down and stated to produce negative pitch rate. | **Agrees on travel and rotation direction**; this is slide evidence, not verification of the published book passage. |
| Beard/McLain published book | Chapter 4 citation only; passage/page unavailable (access record above). | **Unclear / could not open.** |

## Aileron

| Source | Page / section and paraphrased evidence | Verdict against D6 |
|---|---|---|
| CR | A-4 defines positive aileron by positive roll, including spoiler implementations; A-2 defines positive roll as right wing down. No paired trailing-edge direction or differential/total normalization is supplied. | **Agrees on moment sign; physical travel/normalization unclear.** |
| S | Body-axis roll directions are drawn, but the configuration matrix does not establish a signed aileron sweep (p. 2; Fig. 1, PDF 19-20; pp. 4-5, 16-17). | **Unclear.** Positive roll is not proof of positive aileron travel. |
| H | Positive aileron means left aileron down; total aileron deflection has a separate symbol (p. 4). Effectiveness is plotted separately (Fig. 15, p. 36). | **Agrees on the stated left-surface direction; incomplete paired/scale evidence.** Do not equate single-surface and total deflections without resolving their definitions. |
| L | Mechanisms/travel limits are described, but no complete signed paired-aileron definition is established (pp. 6-7, 18-19). | **Unclear.** |
| JSBSim | Chapter introduction defines its aileron variable using the **right** aileron; §“Conventions” diagram labels positive right deflection down and negative roll change. | **Disagrees for the scalar convention:** D6's positive pair has right aileron up and positive roll. An explicit sign/scale mapping is needed before comparisons; do not change D6 or assume all JSBSim aircraft control laws use one mapping (chapter introduction). |
| Caughey | §1.2.2, p. 5/PDF 11: positive aileron means right up/left down and positive rolling effectiveness. | **Agrees**, explicit paired travel and moment sign. |
| Beard/McLain slides | Chapter 4 slide 8 draws left down/right up, defines the pair as half the left-minus-right deflection, and states positive roll rate. | **Agrees on pair and rotation direction.** Its pair normalization is evidence to preserve, not a reason to assume another source uses it. |
| Beard/McLain published book | Chapter 4 citation only; passage/page unavailable (access record above). | **Unclear / could not open.** |

## Rudder

| Source | Page / section and paraphrased evidence | Verdict against D6 |
|---|---|---|
| CR | A-4 defines positive rudder by a nose-left moment; A-2 defines positive yaw as nose right. No physical trailing-edge direction is specified. | **Agrees on moment sign; physical travel unclear.** |
| S | Body-axis yaw signs are shown, but no signed rudder-control test is defined (p. 2; Fig. 1, PDF 19-20; pp. 4-5, 16-17). | **Unclear.** |
| H | The positive-left definition on p. 4 belongs to the **yaw-damper surface**, not automatically the separate conventional rudder described on p. 5. “Left” does not explicitly identify the trailing edge; Fig. 15's yaw-damper effectiveness is positive (p. 36). | **Unclear for D6 rudder.** Surface identity, geometry and sign mapping remain unresolved; do not relabel yaw-damper effectiveness as conventional-rudder effectiveness. |
| L | Positive yaw is nose right, but that defines rotation, not positive rudder deflection; control/travel descriptions supply no complete signed rudder convention (pp. 6-7, 18-19). | **Unclear.** |
| JSBSim | §“Conventions”, diagram: positive rudder is drawn trailing edge left with negative yawing-moment change. | **Agrees** for the diagram's physical deflection. |
| Caughey | §1.2.2, p. 5/PDF 11: positive rudder is trailing edge left and gives negative yaw effectiveness. | **Agrees**, explicit travel and moment sign. |
| Beard/McLain slides | Chapter 4 slide 8 draws rudder left and states negative yaw rate for positive rudder. | **Agrees on travel and rotation direction**, with the slide/book distinction above. |
| Beard/McLain published book | Chapter 4 citation only; passage/page unavailable (access record above). | **Unclear / could not open.** |

Before later sign tests or data conversion, preserve whether each variable is physical deflection, paired/total deflection, stabilizer angle, yaw-damper angle or cockpit command. Preserve body versus nominal stability axes and moment origin too: CR fixes its stability axes at the nominal condition, H uses body axes, S splits longitudinal/lateral frames, and L transforms instrument body-axis measurements to stability axes (CR A-1, A-4, B-1; H pp. 2-5; S p. 2, Fig. 1/PDF 19-20; L pp. 2, 9, 11; JSBSim chapter introduction). No transformation, aircraft parameter file or convention edit is supplied by this audit.
