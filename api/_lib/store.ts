type Agent = {
  id: string;
  name: string;
  kind: string;
  model: string;
  system_prompt: string;
  description: string;
};

type Session = {
  id: string;
  agent_id: string;
  title: string;
  updated_at: string;
};

type Message = {
  id: string;
  role: string;
  content: string;
  created_at: string;
};

type Store = {
  agents: Agent[];
  sessions: Session[];
  messages: Record<string, Message[]>;
};

declare global {
  // eslint-disable-next-line no-var
  var __clawrsStore: Store | undefined;
}

const DEFAULT_AGENT: Agent = {
  id: "00000000-0000-4000-8000-000000000001",
  name: "Claw",
  kind: "general",
  model: process.env.CLAWRS_DEFAULT_MODEL ?? "gpt-4o-mini",
  system_prompt:
    "You are Claw, a precise agent on the ClawRS platform hosted on Vercel.",
  description: "Default ClawRS agent",
};

export function store(): Store {
  if (!global.__clawrsStore) {
    global.__clawrsStore = {
      agents: [DEFAULT_AGENT],
      sessions: [],
      messages: {},
    };
  }
  return global.__clawrsStore;
}

export function id() {
  return crypto.randomUUID();
}

export async function llmReply(
  system: string,
  user: string,
  history: Message[],
): Promise<string> {
  const key = process.env.OPENAI_API_KEY ?? process.env.CLAWRS_OPENAI_API_KEY;
  const base =
    process.env.CLAWRS_OPENAI_BASE_URL ?? "https://api.openai.com";
  const model = process.env.CLAWRS_DEFAULT_MODEL ?? "gpt-4o-mini";

  if (!key) {
    return `[Claw · demo] Connect OPENAI_API_KEY on Vercel for live LLM output. You said: ${user}`;
  }

  const messages = [
    { role: "system", content: system },
    ...history.map((m) => ({ role: m.role, content: m.content })),
    { role: "user", content: user },
  ];

  const res = await fetch(`${base.replace(/\/$/, "")}/v1/chat/completions`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${key}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ model, messages, temperature: 0.4 }),
  });

  if (!res.ok) {
    const text = await res.text();
    return `LLM error (${res.status}): ${text.slice(0, 400)}`;
  }

  const data = (await res.json()) as {
    choices?: { message?: { content?: string } }[];
  };
  return data.choices?.[0]?.message?.content ?? "(empty response)";
}

export type { Agent, Session, Message, Store };
