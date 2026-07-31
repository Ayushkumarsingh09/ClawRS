CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS workspaces (
    id TEXT PRIMARY KEY NOT NULL,
    tenant_id TEXT NOT NULL REFERENCES tenants(id),
    name TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id),
    name TEXT NOT NULL,
    kind TEXT NOT NULL DEFAULT 'general',
    model TEXT NOT NULL,
    system_prompt TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY NOT NULL,
    agent_id TEXT NOT NULL REFERENCES agents(id),
    title TEXT NOT NULL DEFAULT 'New chat',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT NOT NULL REFERENCES sessions(id),
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS memory_records (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    tier TEXT NOT NULL,
    content TEXT NOT NULL,
    importance REAL NOT NULL DEFAULT 0.5,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_agent ON sessions(agent_id);
CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id, created_at);
CREATE INDEX IF NOT EXISTS idx_memory_session ON memory_records(session_id, created_at);
