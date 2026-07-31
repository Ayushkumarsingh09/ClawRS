# clawrs-events

In-process and pluggable event bus for domain integration events. Designed for future NATS-backed distribution without changing producer APIs.

## Architecture

- **`DomainEvent`**: metadata + typed payload contract
- **`EventBus`**: publish / subscribe port (hexagonal)
- **`InMemoryEventBus`**: Tokio broadcast for single-node and tests
- **`EventStore`**: append-only port for event sourcing (in-memory impl included)
