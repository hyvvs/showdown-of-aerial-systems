# Conventions

These are binding. Mismatched frames, units, and signs are the most common cause of flight-sim bugs.
Items marked **DECISION NEEDED** must be settled by the owner (see docs/DECISIONS.md) before code depends on them.

## Units
- **SI everywhere internally**: metres, kilograms, seconds, newtons, pascals, kelvin, radians.
- Degrees, knots, feet, etc. appear **only** at the UI/report boundary, never in core logic.
- Names carry units where ambiguity is possible, e.g. `span_m`, `mass_kg`, `alpha_rad`.
- Unit-typing strategy (newtype wrappers vs. a units crate): DECISION NEEDED (D4).

## Coordinate frames (aerospace standard)
- **Body frame**: x forward (out of the nose), y right (starboard wing), z down. Origin at the aircraft reference point; the dynamics origin is the centre of gravity. Document which point a file refers to.
- **World frame**: NED (x North, y East, z Down). Flat, non-rotating Earth for v0.1. Altitude `h = -z`.
- Rendering uses a different convention in many engines (often y up). **Convert only at the render boundary**, in one place.

## Attitude
- Orientation is a unit quaternion (Hamilton convention) that rotates **body-frame vectors into the NED frame**.
- Euler angles (yaw psi, pitch theta, roll phi; 3-2-1 sequence) are for input/output and display only, never for integration.
- Renormalize the quaternion every step; test that drift stays tiny.

## Rates and velocities
- Angular rates p, q, r: about body x, y, z (roll, pitch, yaw rates).
- Body velocity components u, v, w along x, y, z.
- Airspeed `V = sqrt(u^2 + v^2 + w^2)`; angle of attack `alpha = atan2(w, u)`; sideslip `beta = asin(v / V)`.
- Dynamic pressure `q = 0.5 * rho * V^2`.

## Forces and moments
- Aerodynamic forces are computed in wind/stability axes (lift perpendicular to airspeed, drag opposite to it) and **explicitly transformed** to body axes. The transformation has its own tests.
- Moments: roll L about x, pitch M about y, yaw N about z. **Positive pitching moment = nose up.**
- Control deflection signs: **positive elevator = trailing edge down**, which produces a nose-down (negative) pitching moment.
- Aileron and rudder sign conventions: DECISION NEEDED (D6). Pick one cited textbook convention, write it here, and add a test per surface.

## Mass properties
- Inertia stored as a full 3x3 tensor about the CG in body axes. Document explicitly how off-diagonal terms are signed (tensor entries vs. products of inertia) and test it.

## Atmosphere and gravity (v0.1)
- International Standard Atmosphere (ISA), no wind, supported from sea level to 20 km initially (error outside that range).
- Constant gravity `g0 = 9.80665 m/s^2`, flat Earth.

## Numerics
- Fixed time step, parameterised (default proposal: 1/400 s). RK4 integrator initially.
- Simulation core must be deterministic on one machine and binary: same inputs, same outputs. No unordered-map iteration in simulation logic.
- Cross-platform bit-identical results are **not** promised.
