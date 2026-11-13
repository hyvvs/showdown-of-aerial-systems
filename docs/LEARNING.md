# Learning plan and log

You are building this while learning. The aim: be able to review what the agent writes.

## Weeks 1 to 2: foundations
- Rust: *The Rust Programming Language* (the free "Rust Book"), chapters 1 to 10; then work through Rustlings exercises.
- Flight basics: NASA Glenn "Beginner's Guide to Aeronautics" (search for it by name): forces on an aircraft, lift, drag, stability, control surfaces.

## Weeks 3 to 4: flight dynamics
- Beard & McLain, *Small Unmanned Aircraft: Theory and Practice*: chapters on coordinate frames, kinematics and dynamics, and forces and moments. I believe it is freely available from the authors; verify. Suitable for UAV and odd-configuration thinking.
- As needed: a standard atmosphere explainer for T3.

## Self-check questions (answer in your own words, in the log below)
1. Why does lift depend on the square of airspeed?
2. What does a negative `Cm_alpha` mean physically, and why does it make an aircraft stable?
3. What is the neutral point, and what is static margin?
4. Why must the CG be ahead of the neutral point?
5. What does "trim" mean?
6. Why use a quaternion for attitude instead of Euler angles?
7. Why is thrust not just `power / speed`?
8. What does the stall speed formula assume?

## Log
After each task, the agent adds the date, task and **Concepts introduced** only, leaving **Understood** and **Still unclear** blank. The owner fills in those two fields in their own words, including any questions for the agent.

| Date | Task | Concepts introduced | Understood | Still unclear |
|---|---|---|---|---|
| 2026-11-12 | T0 | Cargo workspace: a group of Rust packages; library crate: reusable code without an executable entry point; package-identity test: checks the manifest's package name. |  |  |
| 2026-11-13 | Documentation corrections and owner decisions | Task dependencies: what must be ready before a task starts; hand-constructed force checks versus trim (a solved steady-flight balance); angle newtypes: distinct Rust types for radians and degrees with explicit conversions. |  |  |
| 2026-11-13 | Binding conventions and batch-mode documentation | Signed tensor entries versus products of inertia; low-speed fallback angles and explicit errors for non-finite inputs; batch stop conditions and local per-task commits. |  |  |
| 2026-11-13 | Hero-aircraft and release-scope documentation | Mach-indexed coefficient tables and interpolation; source-axis derivatives must be transformed before use in body axes; a future hero demo and source-data availability do not establish current model coverage. |  |  |
| 2026-11-13 | Batch-rule clarification and documentation baseline | A task's prerequisites determine which open decisions block it; stable, unique decision IDs keep references unambiguous; a local documentation commit records an agreed baseline. |  |  |
| 2026-11-13 | T1 | Continuous integration runs checks on pushes and pull requests; rustfmt checks formatting, Clippy checks code warnings, and tests check expected behaviour; required gates must fail the workflow when a check fails. |  |  |
