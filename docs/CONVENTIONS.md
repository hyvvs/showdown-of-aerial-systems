# Conventions

These are binding. Mismatched frames, units, and signs are the most common cause of flight-sim bugs.
Items marked **DECISION NEEDED** must be settled by the owner (see docs/DECISIONS.md) before code depends on them.

## Units
- **SI everywhere internally**: metres, kilograms, seconds, newtons, pascals, kelvin, radians.
- Degrees, knots, feet, etc. appear **only** at the UI/report boundary, converted by tested functions, never in core logic.
- Names carry units where ambiguity is possible, e.g. `span_m`, `mass_kg`, `alpha_rad`.
- Use newtype wrappers for angles only for now: `Radians` and `Degrees`, with explicit, tested conversions (D4, option B).
- Adding any further newtype, or the `uom` crate, requires the owner's approval.
- Revisit `uom` only if unit bugs keep recurring.

## Coordinate frames (aerospace standard)
- **Body frame**: x forward (out of the nose), y right (starboard wing), z down. Origin at the aircraft reference point; the dynamics origin is the centre of gravity. Document which point a file refers to.
- **World frame**: NED (x North, y East, z Down). Flat, non-rotating Earth for v0.1. Simulator altitude is **geometric**, `h = -z`.
- Rendering uses a different convention in many engines (often y up). **Convert only at the render boundary**, in one place.
- **Instantaneous stability frame (D12, option A):** x follows the air-relative velocity projected onto the body x-z plane, y equals body y, and z completes the right-handed frame. This is the instantaneous frame, not a body-fixed frame aligned with an earlier reference condition.
- **Wind frame (D12):** x follows the full air-relative velocity, z equals stability z, and y completes the right-handed frame. The frames coincide at zero sideslip.
- **Source/verification:** [JSBSim Manual, Frames of reference](https://jsbsim-team.github.io/jsbsim-reference-manual/user/concepts/frames-of-reference/) describes both meanings of stability axes and the wind frame. Verified online on 2026-11-13; the owner selected the instantaneous meaning.

## Attitude
- Orientation is a unit quaternion (Hamilton convention) that rotates **body-frame vectors into the NED frame**.
- Euler angles (yaw psi, pitch theta, roll phi; 3-2-1 sequence) are for input/output and display only, never for integration.
- Renormalize the quaternion every step; test that drift stays tiny.

## Rates and velocities
- Angular rates p, q, r: about body x, y, z (roll, pitch, yaw rates).
- Body velocity components u, v, w along x, y, z.
- Airspeed `V = sqrt(u^2 + v^2 + w^2)`.
- **Angle policy (D13, option A):** use one named constant `MIN_AIRSPEED_FOR_ANGLES_MPS`, initially `0.1 m/s`. This is an owner-selected placeholder to revisit, not a validated physical threshold. For finite inputs below that airspeed, set `alpha = 0` and `beta = 0`. At or above it, use `alpha = atan2(w, u)` and `beta = asin(clamp(v / V, -1, 1))`.
- Non-finite inputs are an explicit error, checked before any fallback or clamping; never silently clamp them. Test the threshold boundary, fallback angles, asin clamping and non-finite input errors.
- **Sources/verification:** [JSBSim `FGAuxiliary.cpp`, `Run`](https://raw.githubusercontent.com/JSBSim-Team/jsbsim/master/src/models/FGAuxiliary.cpp) guards small airspeed and the u-w projection, with zero fallback angles; it uses atan2 for beta, not asin. [Rust `f64::asin`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.asin) documents a NaN result outside `[-1, 1]`. Both verified online on 2026-11-13. The chosen asin/clamping policy and `0.1 m/s` threshold are owner decisions, not requirements verified from JSBSim.
- Dynamic pressure `q = 0.5 * rho * V^2`.

## Forces and moments
- Aerodynamic forces use wind-axis components `(-drag, +side, -lift)` (D12): lift is perpendicular to airspeed, drag opposes it, and positive side force follows wind +y (body-right at zero sideslip). **Explicitly transform** to body axes; test the transformations separately. Source/verification: the JSBSim frames reference cited above, verified online on 2026-11-13.
- Moments: roll L about x, pitch M about y, yaw N about z. **Positive pitching moment = nose up.**
- **Control deflection signs (D6):** positive elevator = trailing edge down, producing a nose-down (negative) pitching moment; positive aileron = left trailing edge down and right trailing edge up, producing a positive rolling moment; positive rudder = trailing edge left, producing a nose-left (negative) yawing moment. Each surface needs its own sign test.
- **Textbook source/verification:** Beard & McLain, *Small Unmanned Aircraft: Theory and Practice* (Princeton University Press, 2012), chapter 4, control deflections; [authors' textbook resources](https://github.com/byu-magicc/mavsim_public). I could not directly verify the published textbook passage or read the author-linked draft/slides here; the owner will check it. As a primary-source cross-check, [Caughey, *Introduction to Aircraft Stability and Control* (Cornell, 2011), section 1.2.2, printed page 5](https://courses.cit.cornell.edu/mae5070/Caughey_2011_04.pdf#page=11) explicitly gives all three deflection directions and the negative pitch, positive roll and negative yaw control derivatives; verified online on 2026-11-13.

## Mass properties
- **Inertia (D11, option A):** store a full symmetric 3x3 tensor of signed entries about the CG in body axes: `Ixy = -integral(x*y dm)`, `Ixz = -integral(x*z dm)`, `Iyz = -integral(y*z dm)`.
- Provide a constructor from products of inertia defined without those leading minus signs: `Pxy = integral(x*y dm)`, `Pxz = integral(x*z dm)`, `Pyz = integral(y*z dm)`. Negate each when assembling the tensor, copy each to its symmetric entry, and preserve the diagonal moments. Test all off-diagonal signs, symmetry and unchanged diagonals with hand-constructed mass cases.
- **Source/verification:** [JSBSim `FGMassBalance.cpp`, `ReadInertiaMatrix`](https://raw.githubusercontent.com/JSBSim-Team/jsbsim/master/src/models/FGMassBalance.cpp) supports alternative input sign conventions and converts structural axes to body axes. Source verified online on 2026-11-13; its structural-frame signs must not be copied unchanged into this project's body frame.

## Atmosphere and gravity (v0.1)
- International Standard Atmosphere (ISA), no wind, supported from sea level to 20 km initially (error outside that range).
- The simulator uses `isa_at_geometric_altitude` with geometric `h = -z` in metres. Its supported altitude range is [0, 20000] geometric metres inclusive; the geopotential API is for explicitly identified reference calculations.
- Constant gravity `g0 = 9.80665 m/s^2`, flat Earth.

## Numerics
- **Time step (D10):** fixed during a run, always a parameter, with default `1/400 s` (`0.0025 s`). RK4 integrator initially. T4's step-size convergence test may justify revising the default; record any change as a new decision.
- **Sources/verification:** the owner selected `1/400 s` from this file's earlier proposal; no supporting external source or project convergence evidence was verified for it. [JSBSim `FGFDMExec.cpp`, constructor](https://raw.githubusercontent.com/JSBSim-Team/jsbsim/master/src/FGFDMExec.cpp) uses `1/120 s` as a standalone fallback, verified online on 2026-11-13; that comparison does not validate this project's chosen default.
- Simulation core must be deterministic on one machine and binary: same inputs, same outputs. No unordered-map iteration in simulation logic.
- Cross-platform bit-identical results are **not** promised.
