# Decisions log

Agents: do not make an open decision yourself. Present options with trade-offs and wait for the owner.

| ID | Decision | Status | Notes / recommendation |
|---|---|---|---|
| D1 | Project name | OPEN | "aero" is a placeholder (probably already taken). Renaming later = search/replace of crate names. |
| D2 | License | OPEN | Recommendation: `MIT OR Apache-2.0` (standard in Rust; same as SolidCraft). Owner confirms before any LICENSE file is added. |
| D3 | Aircraft file format | OPEN | Candidates: JSON (best web/JS interop), TOML (comments, human-friendly), RON (Rust-specific). Decide before Task T7. |
| D4 | Units strategy | OPEN | Newtype wrappers vs. `uom` crate vs. suffix naming only. Agent writes a short comparison; owner decides. |
| D5 | Web viewer stack | DEFERRED | Decide at Stage 4. Options: Bevy (Rust-native, heavy on web) or a JS viewer (e.g. Three.js) on a WASM core. Do not decide earlier. |
| D6 | Aileron / rudder sign conventions | OPEN | Choose one cited textbook convention; record in CONVENTIONS.md. Needed before T6. |
| D7 | SolidCraft integration | DEFERRED | Plan: drive its CLI/command scripts, not its Rust internals; v0.4. Pin a version. Its project moves fast. |
| D8 | First supersonic validation aircraft | DEFERRED | Before v0.2, confirm that public data (e.g. NASA NTRS reports) actually exists for the chosen aircraft. |
| D9 | Validation tolerances | OPEN | Proposal only: about +/-10% for stall speed. Owner confirms after seeing the reference data (task H3). |
