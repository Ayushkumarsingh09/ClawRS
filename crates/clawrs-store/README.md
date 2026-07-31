# ClawRS store

SQLx persistence: tenants, workspaces, agents, chat sessions, messages, and memory records.

Default feature: **SQLite** (`clawrs.db`). Enable `postgres` on this crate for future hosted deployments.

Migrations live in `migrations/` and run automatically on gateway startup.
