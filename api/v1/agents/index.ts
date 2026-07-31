import type { VercelRequest, VercelResponse } from "@vercel/node";
import { id, store, type Agent } from "../../_lib/store";

function cors(res: VercelResponse) {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type, Authorization, X-ClawRS-Key");
}

export default async function handler(req: VercelRequest, res: VercelResponse) {
  cors(res);
  if (req.method === "OPTIONS") return res.status(204).end();

  const s = store();

  if (req.method === "GET") {
    return res.status(200).json({ agents: s.agents });
  }

  if (req.method === "POST") {
    const body = req.body as {
      name?: string;
      model?: string;
      system_prompt?: string;
    };
    const now = new Date().toISOString();
    const agent: Agent = {
      id: id(),
      name: body.name ?? "Agent",
      kind: "general",
      model: body.model ?? process.env.CLAWRS_DEFAULT_MODEL ?? "gpt-4o-mini",
      system_prompt: body.system_prompt ?? "You are a helpful agent.",
      description: "",
    };
    s.agents.push(agent);
    return res.status(200).json(agent);
  }

  return res.status(405).json({ error: "method not allowed" });
}
