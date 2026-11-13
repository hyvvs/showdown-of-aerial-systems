# Showdown of Aerial Systems

An aircraft design and flight simulation project written in Rust, intended to
share one aircraft description between engineering analysis and a flyable
simulation. The license is still undecided.

The crate prefix `soas` stands for **Showdown of Aerial Systems**.
The repository slug is `showdown-of-aerial-systems`.

The workspace contains one simulation library, `crates/soas-core`, with tests for
the implemented helpers. Headless and browser applications will be added in
later tasks.

## Development

Install Rust through rustup, then run these commands from this directory:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

`rust-toolchain.toml` selects stable Rust and requests rustfmt (formatting) and
Clippy (code checks). The library uses Rust edition 2024.

Start with [AGENTS.md](AGENTS.md) and the [task queue](docs/TASKS.md). See the
[vision](docs/VISION.md), [roadmap](docs/ROADMAP.md), and
[model limitations](docs/ASSUMPTIONS.md) for the project scope.
