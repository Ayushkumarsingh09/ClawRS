# ClawRS roadmap

Incremental delivery with compile + test gates after every milestone.

## M0 — Foundation (shipped)

## M1 — Production gateway (current)

- [x] SQLite persistence (agents, sessions, messages, memory)
- [x] Config from environment (`clawrs-config`)
- [x] LLM factory (mock + OpenAI-compatible)
- [x] REST API v1 + optional API key auth
- [x] React web console (production build served by gateway)
- [x] Docker Compose + multi-stage Dockerfile
- [x] Architecture diagrams in README

## M2 — Security & multi-tenant gateway

- RBAC, API keys, JWT/OIDC
- Encrypted secrets store
- Rate limiting + audit log
- Real tenant routing (not ephemeral IDs per request)

## M3 — RAG & vector

- Qdrant adapter
- Hybrid BM25 + dense search
- Knowledge vault + filesystem sync

## M4 — MCP & plugins

- MCP host/client/proxy
- Dynamic plugin loading (WASM / dylib strategy TBD)
- Tool sandbox + approval flows

## M5 — Automation & teams

- Cron / webhooks / workflow DAG
- Hierarchical agents, delegation, task boards
- NATS event backbone

## M6 — Observability & enterprise

- OpenTelemetry export
- Cost & latency dashboards
- Helm / Terraform / Docker compose stacks

## M7 — Desktop & web UI

- Tauri desktop (offline SQLite)
- Web UI (Leptos or React) with realtime chat + kanban

## M8 — SDKs

- Rust (first-party), TypeScript, Python, Go, Java, C#
