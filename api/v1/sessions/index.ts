import type { VercelRequest, VercelResponse } from "@vercel/node";
import { id, store, type Session } from "../../_lib/store";

function cors(res: VercelResponse) {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type");
}

export default function handler(req: VercelRequest, res: VercelResponse) {
  cors(res);
  if (req.method === "OPTIONS") return res.status(204).end();

  const s = store();

  if (req.method === "GET") {
    const agentId = req.query.agent_id as string;
    const sessions = s.sessions
      .filter((x) => x.agent_id === agentId)
      .sort((a, b) => b.updated_at.localeCompare(a.updated_at));
    return res.status(200).json({ sessions });
  }

  if (req.method === "POST") {
    const body = req.body as { agent_id?: string; title?: string };
    if (!body.agent_id) return res.status(400).json({ error: "agent_id required" });
    const now = new Date().toISOString();
    const session: Session = {
      id: id(),
      agent_id: body.agent_id,
      title: body.title ?? "New chat",
      updated_at: now,
    };
    s.sessions.unshift(session);
    s.messages[session.id] = [];
    return res.status(200).json(session);
  }

  return res.status(405).json({ error: "method not allowed" });
}
