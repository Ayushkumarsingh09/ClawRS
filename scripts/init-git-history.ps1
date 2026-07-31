# Builds a structured commit history (50+ commits) for ClawRS.
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot/..

if (Test-Path .git) { Remove-Item -Recurse -Force .git }

git init -b main

function Commit-Files($Message, [string[]]$Files) {
    foreach ($f in $Files) {
        if (Test-Path $f) { git add $f }
    }
    $pending = git diff --cached --name-only
    if (-not $pending) { Write-Host "skip empty: $Message"; return }
    git commit -m $Message
}

Commit-Files "chore: add Apache-2.0 license" @("LICENSE")
Commit-Files "docs: add initial project readme" @("README.md")
Commit-Files "chore: add gitignore and formatter config" @(".gitignore", "rustfmt.toml")
Commit-Files "chore: pin stable Rust toolchain" @("rust-toolchain.toml")
Commit-Files "build: scaffold cargo workspace" @("Cargo.toml", "Cargo.lock")
Commit-Files "docs: add agent contributor guide" @("AGENTS.md")

Commit-Files "feat(core): add clawrs-core crate manifest" @("crates/clawrs-core/Cargo.toml")
Commit-Files "feat(core): add library root and error types" @("crates/clawrs-core/src/lib.rs", "crates/clawrs-core/src/error.rs")
Commit-Files "feat(core): add strongly typed domain identifiers" @("crates/clawrs-core/src/ids.rs")
Commit-Files "feat(core): add tenant and workspace context" @("crates/clawrs-core/src/tenant.rs")
Commit-Files "feat(core): add build version metadata" @("crates/clawrs-core/src/version.rs")
Commit-Files "docs(core): document core crate responsibilities" @("crates/clawrs-core/README.md")

Commit-Files "feat(events): add events crate manifest" @("crates/clawrs-events/Cargo.toml")
Commit-Files "feat(events): add event envelope model" @("crates/clawrs-events/src/lib.rs", "crates/clawrs-events/src/envelope.rs")
Commit-Files "feat(events): add in-memory event bus" @("crates/clawrs-events/src/bus.rs")
Commit-Files "feat(events): add event store projections" @("crates/clawrs-events/src/store.rs")
Commit-Files "docs(events): describe event-driven integration" @("crates/clawrs-events/README.md")

Commit-Files "feat(memory): add memory crate manifest" @("crates/clawrs-memory/Cargo.toml")
Commit-Files "feat(memory): add tier and record models" @("crates/clawrs-memory/src/lib.rs", "crates/clawrs-memory/src/tier.rs", "crates/clawrs-memory/src/record.rs")
Commit-Files "feat(memory): add memory store port and in-memory backend" @("crates/clawrs-memory/src/store.rs")

Commit-Files "feat(llm): add llm crate manifest" @("crates/clawrs-llm/Cargo.toml")
Commit-Files "feat(llm): add chat message and tool definitions" @("crates/clawrs-llm/src/message.rs")
Commit-Files "feat(llm): add completion request types" @("crates/clawrs-llm/src/request.rs")
Commit-Files "feat(llm): add completion response and streaming chunks" @("crates/clawrs-llm/src/response.rs")
Commit-Files "feat(llm): add provider trait and exports" @("crates/clawrs-llm/src/provider.rs", "crates/clawrs-llm/src/lib.rs")
Commit-Files "feat(llm): add mock provider for offline development" @("crates/clawrs-llm/src/mock.rs")
Commit-Files "feat(llm): add OpenAI-compatible HTTP adapter" @("crates/clawrs-llm/src/openai_compatible.rs")
Commit-Files "feat(llm): add provider factory and dynamic wrapper" @("crates/clawrs-llm/src/factory.rs")

Commit-Files "feat(tools): add tools crate manifest" @("crates/clawrs-tools/Cargo.toml")
Commit-Files "feat(tools): add tool trait and execution context" @("crates/clawrs-tools/src/lib.rs", "crates/clawrs-tools/src/tool.rs", "crates/clawrs-tools/src/context.rs")
Commit-Files "feat(tools): add tool registry" @("crates/clawrs-tools/src/registry.rs")
Commit-Files "feat(tools): add echo builtin tool" @("crates/clawrs-tools/src/builtin.rs")

Commit-Files "feat(agent): add agent crate manifest" @("crates/clawrs-agent/Cargo.toml")
Commit-Files "feat(agent): add agent kinds and prompt modes" @("crates/clawrs-agent/src/kind.rs", "crates/clawrs-agent/src/prompt.rs")
Commit-Files "feat(agent): export agent module surface" @("crates/clawrs-agent/src/lib.rs")
Commit-Files "feat(agent): add eight-stage pipeline stages" @("crates/clawrs-agent/src/stages.rs")
Commit-Files "feat(agent): add pipeline orchestrator" @("crates/clawrs-agent/src/pipeline.rs")
Commit-Files "feat(agent): add agent runner with tool loop" @("crates/clawrs-agent/src/run.rs")

Commit-Files "feat(config): add environment-driven app configuration" @("crates/clawrs-config/Cargo.toml", "crates/clawrs-config/src/lib.rs")

Commit-Files "feat(store): add persistence crate manifest" @("crates/clawrs-store/Cargo.toml")
Commit-Files "feat(store): add initial SQL schema migration" @("crates/clawrs-store/migrations/001_init.sql")
Commit-Files "feat(store): add connection pool helper" @("crates/clawrs-store/src/pool.rs")
Commit-Files "feat(store): add row models for agents and sessions" @("crates/clawrs-store/src/models.rs")
Commit-Files "feat(store): add repository for CRUD and chat history" @("crates/clawrs-store/src/repository.rs")
Commit-Files "feat(store): add sqlite memory store adapter" @("crates/clawrs-store/src/memory.rs")
Commit-Files "feat(store): wire store crate exports" @("crates/clawrs-store/src/lib.rs")
Commit-Files "docs(store): document persistence layer" @("crates/clawrs-store/README.md")

Commit-Files "feat(gateway): add gateway crate manifest" @("crates/clawrs-gateway/Cargo.toml")
Commit-Files "feat(gateway): add shared API error handling" @("crates/clawrs-gateway/src/error.rs")
Commit-Files "feat(gateway): bootstrap application state from config" @("crates/clawrs-gateway/src/state.rs")
Commit-Files "feat(gateway): add health and status endpoints" @("crates/clawrs-gateway/src/api/health.rs")
Commit-Files "feat(gateway): add agents REST handlers" @("crates/clawrs-gateway/src/api/agents.rs")
Commit-Files "feat(gateway): add sessions REST handlers" @("crates/clawrs-gateway/src/api/sessions.rs")
Commit-Files "feat(gateway): add chat completion handler" @("crates/clawrs-gateway/src/api/chat.rs")
Commit-Files "feat(gateway): compose API router and library exports" @("crates/clawrs-gateway/src/api/mod.rs", "crates/clawrs-gateway/src/lib.rs")

Commit-Files "feat(cli): add clawrs binary crate" @("crates/clawrs-cli/Cargo.toml", "crates/clawrs-cli/src/main.rs")

Commit-Files "docs: add architecture overview" @("docs/architecture/overview.md")
Commit-Files "docs: add product roadmap" @("docs/ROADMAP.md")
Commit-Files "docs: add Render deployment guide" @("docs/deploy-render.md")

Commit-Files "ci: add Rust and web build pipeline" @(".github/workflows/ci.yml")
Commit-Files "ci: add GitHub Pages deployment workflow" @(".github/workflows/pages.yml")

Commit-Files "build: add multi-stage Dockerfile" @("Dockerfile")
Commit-Files "build: add local docker compose stack" @("docker-compose.yml")
Commit-Files "build: add Fly.io deployment manifest" @("fly.toml")
Commit-Files "build: add Render blueprint" @("render.yaml")
Commit-Files "chore: add example environment file" @(".env.example")

Commit-Files "assets: add ClawRS brand logo" @("assets/logo.png")

Commit-Files "feat(web): scaffold Vite React application" @("web/package.json", "web/package-lock.json", "web/vite.config.ts", "web/tsconfig.json", "web/index.html")
Commit-Files "feat(web): add application entry and types" @("web/src/main.tsx", "web/src/vite-env.d.ts")
Commit-Files "feat(web): add global design tokens and layout styles" @("web/src/styles/global.css")
Commit-Files "feat(web): add typed HTTP client for gateway API" @("web/src/api/client.ts")
Commit-Files "feat(web): add console UI and component styles" @("web/src/App.tsx", "web/src/App.module.css")
Commit-Files "feat(web): add public logo asset" @("web/public/logo.png")

Commit-Files "docs: document live deployment and architecture diagrams" @("README.md")
Commit-Files "chore: add git history bootstrap script" @("scripts/init-git-history.ps1")

Write-Host "Commit count: $(git rev-list --count HEAD)"
