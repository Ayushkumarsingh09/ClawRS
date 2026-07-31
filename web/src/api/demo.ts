import type { Agent, Message, Session, Status } from "./types";

const STORAGE_KEY = "clawrs_demo_v1";

type Store = {
  agents: Agent[];
  sessions: Session[];
  messages: Record<string, Message[]>;
};

const DEFAULT_AGENT: Agent = {
  id: "00000000-0000-4000-8000-000000000001",
  name: "Claw",
  kind: "general",
  model: "demo-local",
  system_prompt:
    "You are Claw, a capable agent on the ClawRS platform. In demo mode responses are generated locally in your browser.",
  description: "Built-in demo agent (offline)",
};

function loadStore(): Store {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (raw) {
    try {
      return JSON.parse(raw) as Store;
    } catch {
      /* fall through */
    }
  }
  return {
    agents: [DEFAULT_AGENT],
    sessions: [],
    messages: {},
  };
}

function saveStore(store: Store) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(store));
}

function id() {
  return crypto.randomUUID();
}

export const demoApi = {
  mode: "demo" as const,

  status(): Status {
    const store = loadStore();
    const messages = Object.values(store.messages).reduce((n, m) => n + m.length, 0);
    return {
      version: "0.1.0-demo",
      provider: "browser-demo",
      default_model: "demo-local",
      stats: {
        agents: store.agents.length,
        sessions: store.sessions.length,
        messages,
      },
    };
  },

  agents(): Agent[] {
    return loadStore().agents;
  },

  createAgent(body: {
    name: string;
    model?: string;
    system_prompt?: string;
  }): Agent {
    const store = loadStore();
    const agent: Agent = {
      id: id(),
      name: body.name,
      kind: "general",
      model: body.model ?? "demo-local",
      system_prompt: body.system_prompt ?? "You are a helpful agent.",
      description: "",
    };
    store.agents.push(agent);
    saveStore(store);
    return agent;
  },

  sessions(agentId: string): Session[] {
    return loadStore()
      .sessions.filter((s) => s.agent_id === agentId)
      .sort((a, b) => b.updated_at.localeCompare(a.updated_at));
  },

  createSession(agentId: string, title = "New chat"): Session {
    const store = loadStore();
    const now = new Date().toISOString();
    const session: Session = {
      id: id(),
      agent_id: agentId,
      title,
      updated_at: now,
    };
    store.sessions.unshift(session);
    store.messages[session.id] = [];
    saveStore(store);
    return session;
  },

  messages(sessionId: string): Message[] {
    return loadStore().messages[sessionId] ?? [];
  },

  chat(
    sessionId: string,
    text: string,
  ): { reply: string; tool_rounds: number } {
    const store = loadStore();
    const now = new Date().toISOString();
    const list = store.messages[sessionId] ?? [];
    list.push({
      id: id(),
      role: "user",
      content: text,
      created_at: now,
    });

    const agent = store.agents.find(
      (a) => a.id === store.sessions.find((s) => s.id === sessionId)?.agent_id,
    );
    const name = agent?.name ?? "Claw";
    const reply = `[${name} · demo] I received your message. Connect a ClawRS gateway (Render/Docker/local) for real LLM replies. You said: ${text}`;

    list.push({
      id: id(),
      role: "assistant",
      content: reply,
      created_at: new Date().toISOString(),
    });
    store.messages[sessionId] = list;

    const session = store.sessions.find((s) => s.id === sessionId);
    if (session) {
      session.updated_at = new Date().toISOString();
      if (session.title === "New chat") {
        session.title = text.slice(0, 48);
      }
    }
    saveStore(store);
    return { reply, tool_rounds: 0 };
  },
};
