import type { VercelRequest, VercelResponse } from "@vercel/node";
import { id, llmReply, store } from "../../../_lib/store";

function cors(res: VercelResponse) {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", "POST, OPTIONS");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type");
}

export default async function handler(req: VercelRequest, res: VercelResponse) {
  cors(res);
  if (req.method === "OPTIONS") return res.status(204).end();
  if (req.method !== "POST") return res.status(405).json({ error: "method not allowed" });

  const sessionId = req.query.id as string;
  const body = req.body as { message?: string };
  const text = body.message?.trim();
  if (!text) return res.status(400).json({ error: "message required" });

  const s = store();
  const session = s.sessions.find((x) => x.id === sessionId);
  if (!session) return res.status(404).json({ error: "session not found" });

  const agent = s.agents.find((a) => a.id === session.agent_id);
  if (!agent) return res.status(404).json({ error: "agent not found" });

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

  return res.status(200).json({
    reply,
    tool_rounds: 0,
    prompt_tokens: 0,
    completion_tokens: 0,
  });
}
