# ClawRS agent instructions

ClawRS is a Rust-native AI agent platform. When changing code:

1. Keep **domain logic** out of Axum handlers—use `clawrs-agent` and ports.
2. Prefer **trait-based ports** (`MemoryStore`, `LlmProvider`, `EventBus`, `Tool`).
3. No `unsafe` unless justified in crate README.
4. Every crate must **compile** and include a **README** with architecture notes.
5. Run `cargo test --workspace` and `cargo clippy --workspace -- -D warnings` before PRs.
6. Match naming: `clawrs-*` crates, `ClawrsError`, `clawrs` binary.

Target architecture lives in `docs/architecture/overview.md` and `docs/ROADMAP.md`.
