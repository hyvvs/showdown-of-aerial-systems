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
Add an entry after each task: date, task, what I understood, what I did not, and questions for the agent.

| Date | Task | Understood | Still unclear |
|---|---|---|---|
| 2026-11-12 | T0 | Agent notes for owner review: a Cargo workspace groups Rust packages; a library has no executable entry point; the package-name test failed with an intentionally wrong manifest name and passed after correction. All three quality gates passed. | Owner understanding has not been checked. Project name (D1) and license (D2) remain open. |
