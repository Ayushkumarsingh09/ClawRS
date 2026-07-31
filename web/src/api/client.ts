import { demoApi } from "./demo";
import type { Agent, Message, Session, Status } from "./types";

export type { Agent, Message, Session, Status } from "./types";
export type ConnectionMode = "live" | "demo";

/** Production API used when GitHub Pages build omits `VITE_API_BASE`. */
const PRODUCTION_API = "https://clawrs-api.vercel.app";

function resolveApiBase(): string {
  const fromEnv = (import.meta.env.VITE_API_BASE ?? "").replace(/\/$/, "");
  if (fromEnv) return fromEnv;
  if (typeof window !== "undefined" && window.location.hostname.endsWith("github.io")) {
    return PRODUCTION_API;
  }
  return "";
}

const apiBase = resolveApiBase();
const forceDemo = import.meta.env.VITE_FORCE_DEMO === "true";

let resolvedMode: ConnectionMode | null = forceDemo ? "demo" : null;

async function fetchWithTimeout(url: string, ms: number): Promise<Response> {
  const controller = new AbortController();
  const timer = window.setTimeout(() => controller.abort(), ms);
  try {
    return await fetch(url, { method: "GET", signal: controller.signal, mode: "cors" });
  } finally {
    window.clearTimeout(timer);
  }
}

async function probeLiveApi(): Promise<boolean> {
  if (!apiBase) return false;
  const healthUrl = `${apiBase}/health`;
  try {
    const res = await fetchWithTimeout(healthUrl, 8000);
    return res.ok;
  } catch {
    return false;
  }
}

export async function getConnectionMode(): Promise<ConnectionMode> {
  if (resolvedMode) return resolvedMode;
  if (forceDemo) {
    resolvedMode = "demo";
    return resolvedMode;
  }
  const live = await probeLiveApi();
  resolvedMode = live ? "live" : "demo";
  return resolvedMode;
}

async function req<T>(path: string, init?: RequestInit): Promise<T> {
  const mode = await getConnectionMode();
  if (mode === "demo") {
    throw new Error("demo-routing");
  }
  const hasBody = init?.body != null && init.body !== "";
  const res = await fetch(`${apiBase}${path}`, {
    ...init,
    headers: {
      ...(hasBody ? { "Content-Type": "application/json" } : {}),
      ...(init?.headers ?? {}),
    },
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error((body as { error?: string }).error ?? res.statusText);
  }
  return res.json() as Promise<T>;
}

export const api = {
  connectionMode: getConnectionMode,

  status: async (): Promise<Status> => {
    if ((await getConnectionMode()) === "demo") return demoApi.status();
    return req<Status>("/api/v1/status");
  },

  agents: async (): Promise<Agent[]> => {
    if ((await getConnectionMode()) === "demo") return demoApi.agents();
    return req<{ agents: Agent[] }>("/api/v1/agents").then((r) => r.agents);
  },

  createAgent: async (body: {
    name: string;
    model?: string;
    system_prompt?: string;
  }): Promise<Agent> => {
    if ((await getConnectionMode()) === "demo") return demoApi.createAgent(body);
    return req<Agent>("/api/v1/agents", {
      method: "POST",
      body: JSON.stringify(body),
    });
  },

  sessions: async (agentId: string): Promise<Session[]> => {
    if ((await getConnectionMode()) === "demo") return demoApi.sessions(agentId);
    return req<{ sessions: Session[] }>(`/api/v1/sessions?agent_id=${agentId}`).then(
      (r) => r.sessions,
    );
  },

  createSession: async (agentId: string, title?: string): Promise<Session> => {
    if ((await getConnectionMode()) === "demo") return demoApi.createSession(agentId, title);
    return req<Session>("/api/v1/sessions", {
      method: "POST",
      body: JSON.stringify({ agent_id: agentId, title: title ?? "New chat" }),
    });
  },

  messages: async (sessionId: string): Promise<Message[]> => {
    if ((await getConnectionMode()) === "demo") return demoApi.messages(sessionId);
    return req<{ messages: Message[] }>(`/api/v1/sessions/${sessionId}/messages`).then(
      (r) => r.messages,
    );
  },

  chat: async (
    sessionId: string,
    message: string,
  ): Promise<{ reply: string; tool_rounds: number }> => {
    if ((await getConnectionMode()) === "demo") return demoApi.chat(sessionId, message);
    return req<{ reply: string; tool_rounds: number }>(`/api/v1/sessions/${sessionId}/chat`, {
      method: "POST",
      body: JSON.stringify({ message }),
    });
  },
};
