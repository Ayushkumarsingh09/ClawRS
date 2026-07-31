const base = import.meta.env.VITE_API_BASE ?? "";

async function req<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${base}${path}`, {
    ...init,
    headers: {
      "Content-Type": "application/json",
      ...(init?.headers ?? {}),
    },
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error(body.error ?? res.statusText);
  }
  return res.json() as Promise<T>;
}

export type Agent = {
  id: string;
  name: string;
  kind: string;
  model: string;
  system_prompt: string;
  description: string;
};

export type Session = {
  id: string;
  agent_id: string;
  title: string;
  updated_at: string;
};

export type Message = {
  id: string;
  role: string;
  content: string;
  created_at: string;
};

export type Status = {
  version: string;
  provider: string;
  default_model: string;
  stats: { agents: number; sessions: number; messages: number };
};

export const api = {
  status: () => req<Status>("/api/v1/status"),
  agents: () => req<{ agents: Agent[] }>("/api/v1/agents").then((r) => r.agents),
  createAgent: (body: {
    name: string;
    model?: string;
    system_prompt?: string;
  }) =>
    req<Agent>("/api/v1/agents", {
      method: "POST",
      body: JSON.stringify(body),
    }),
  sessions: (agentId: string) =>
    req<{ sessions: Session[] }>(`/api/v1/sessions?agent_id=${agentId}`).then(
      (r) => r.sessions,
    ),
  createSession: (agentId: string, title?: string) =>
    req<Session>("/api/v1/sessions", {
      method: "POST",
      body: JSON.stringify({ agent_id: agentId, title: title ?? "New chat" }),
    }),
  messages: (sessionId: string) =>
    req<{ messages: Message[] }>(`/api/v1/sessions/${sessionId}/messages`).then(
      (r) => r.messages,
    ),
  chat: (sessionId: string, message: string) =>
    req<{ reply: string; tool_rounds: number }>(`/api/v1/sessions/${sessionId}/chat`, {
      method: "POST",
      body: JSON.stringify({ message }),
    }),
};
