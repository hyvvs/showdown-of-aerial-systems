# Model assumptions and limitations

Keep this honest and current. Every simplification the code makes belongs here. Reports and the README link to it.

## Environment (v0.1)
- Flat, non-rotating Earth; constant gravity.
- International Standard Atmosphere, no wind, no turbulence.

## Aircraft (v0.1)
- Rigid airframe (no flexing, no flutter).
- Mass properties constant during a run unless a task says otherwise.

## Aerodynamics (initial model)
- Quasi-steady aerodynamics (no unsteady effects).
- Linear lift and moment curves until a simple stall limit; post-stall behaviour is crude.
- Subsonic only until v0.2; compressibility handled by a correction below about Mach 0.7 to 0.8.
- No ground effect, no propwash on tail, no aeroelastic effects.

## Propulsion
- To be filled in when the engine model task starts. Note: thrust is not simply `power / speed` (that diverges at zero airspeed).

## Known weak regions
- Transonic (about Mach 0.8 to 1.2): low confidence.
- Large angles of attack, spins, and tumbling: not modelled.

## Validation status
| Case | Reference | Quantities compared | Error | Status |
|---|---|---|---|---|
| (none yet) | | | | |
