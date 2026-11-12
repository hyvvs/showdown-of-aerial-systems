# aero (placeholder name)

An aircraft design and flight simulation project written in Rust, intended to
share one aircraft description between engineering analysis and a flyable
simulation. The project name and license are still undecided.

The workspace currently contains one dependency-free library, `crates/aero-core`,
and a smoke test for its package identity. Physics and applications will be added
in later tasks.

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
