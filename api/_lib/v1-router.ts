import type { VercelRequest, VercelResponse } from "@vercel/node";
import { id, llmReply, store, type Session } from "./store";

export async function dispatchV1(
  req: VercelRequest,
  res: VercelResponse,
  segments: string[],
): Promise<void> {
  const method = req.method ?? "GET";
  const s = store();

  if (segments.length === 1 && segments[0] === "status" && method === "GET") {
    const messages = Object.values(s.messages).reduce((n, m) => n + m.length, 0);
    res.status(200).json({
      version: "0.1.0-vercel",
      provider: process.env.OPENAI_API_KEY ? "openai-compatible" : "vercel-mock",
      default_model: process.env.CLAWRS_DEFAULT_MODEL ?? "gpt-4o-mini",
      stats: {
        agents: s.agents.length,
        sessions: s.sessions.length,
        messages,
      },
    });
    return;
  }

  if (segments.length === 1 && segments[0] === "agents" && method === "GET") {
    res.status(200).json({ agents: s.agents });
    return;
  }

  if (segments.length === 1 && segments[0] === "agents" && method === "POST") {
    const body = req.body as { name?: string; model?: string; system_prompt?: string };
    const agent = {
      id: id(),
      name: body.name ?? "Agent",
      kind: "general",
      model: body.model ?? process.env.CLAWRS_DEFAULT_MODEL ?? "gpt-4o-mini",
      system_prompt: body.system_prompt ?? "You are a helpful agent.",
      description: "",
    };
    s.agents.push(agent);
    res.status(200).json(agent);
    return;
  }

  if (segments.length === 2 && segments[0] === "agents" && method === "GET") {
    const agent = s.agents.find((a) => a.id === segments[1]);
    if (!agent) {
      res.status(404).json({ error: "agent not found" });
      return;
    }
    res.status(200).json(agent);
    return;
  }

  if (segments.length === 1 && segments[0] === "sessions" && method === "GET") {
    const agentId = req.query.agent_id as string;
    const sessions = s.sessions
      .filter((x) => x.agent_id === agentId)
      .sort((a, b) => b.updated_at.localeCompare(a.updated_at));
    res.status(200).json({ sessions });
    return;
  }

  if (segments.length === 1 && segments[0] === "sessions" && method === "POST") {
    const body = req.body as { agent_id?: string; title?: string };
    if (!body.agent_id) {
      res.status(400).json({ error: "agent_id required" });
      return;
    }
    const now = new Date().toISOString();
    const session: Session = {
      id: id(),
      agent_id: body.agent_id,
      title: body.title ?? "New chat",
      updated_at: now,
    };
    s.sessions.unshift(session);
    s.messages[session.id] = [];
    res.status(200).json(session);
    return;
  }

  if (
    segments.length === 3 &&
    segments[0] === "sessions" &&
    segments[2] === "messages" &&
    method === "GET"
  ) {
    const sessionId = segments[1];
    res.status(200).json({ messages: s.messages[sessionId] ?? [] });
    return;
  }

  if (
    segments.length === 3 &&
    segments[0] === "sessions" &&
    segments[2] === "chat" &&
    method === "POST"
  ) {
    const sessionId = segments[1];
    const body = req.body as { message?: string };
    const text = body.message?.trim();
    if (!text) {
      res.status(400).json({ error: "message required" });
      return;
    }

    const session = s.sessions.find((x) => x.id === sessionId);
    if (!session) {
      res.status(404).json({ error: "session not found" });
      return;
    }

    const agent = s.agents.find((a) => a.id === session.agent_id);
    if (!agent) {
      res.status(404).json({ error: "agent not found" });
      return;
    }

    const list = s.messages[sessionId] ?? [];
    const now = new Date().toISOString();
    list.push({ id: id(), role: "user", content: text, created_at: now });

    const reply = await llmReply(agent.system_prompt, text, list.slice(0, -1));
    list.push({
      id: id(),
      role: "assistant",
      content: reply,
      created_at: new Date().toISOString(),
    });
    s.messages[sessionId] = list;
    session.updated_at = new Date().toISOString();
    if (session.title === "New chat") session.title = text.slice(0, 48);

    res.status(200).json({
      reply,
      tool_rounds: 0,
      prompt_tokens: 0,
      completion_tokens: 0,
    });
    return;
  }

  res.status(404).json({ error: "not found" });
}
