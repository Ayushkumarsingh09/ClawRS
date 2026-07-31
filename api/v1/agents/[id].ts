import type { VercelRequest, VercelResponse } from "@vercel/node";
import { applyCors, handleOptions } from "../../_lib/cors";
import { store } from "../../_lib/store";

export default function handler(req: VercelRequest, res: VercelResponse) {
  applyCors(res);
  if (handleOptions(req, res)) return;

  const agentId = req.query.id as string;
  const agent = store().agents.find((a) => a.id === agentId);
  if (!agent) return res.status(404).json({ error: "agent not found" });
  return res.status(200).json(agent);
}
