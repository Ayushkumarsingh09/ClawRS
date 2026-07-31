# clawrs-core

Foundation crate for **ClawRS**: strongly typed identifiers, domain errors, tenant/workspace primitives, and shared configuration shapes.

## Architecture

This crate sits at the center of the hexagonal layout. It has **no** I/O, async runtime, or database dependencies so every other crate can depend on it without cycles.

## Design decisions

- **Newtype IDs** (`AgentId`, `SessionId`, …) prevent accidental cross-wiring at compile time.
- **`ClawrsError`** is the single error enum for domain failures; infrastructure maps its own errors into this at boundaries.
- **Feature flags** on downstream crates gate optional subsystems; core stays minimal.

## Examples

See `src/ids.rs` tests for ID serialization round-trips.
