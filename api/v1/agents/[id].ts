import type { VercelRequest, VercelResponse } from "@vercel/node";
import { store } from "../../_lib/store";

function cors(res: VercelResponse) {
  res.setHeader("Access-Control-Allow-Origin", "*");
}

export default function handler(req: VercelRequest, res: VercelResponse) {
  cors(res);
  const agentId = req.query.id as string;
  const agent = store().agents.find((a) => a.id === agentId);
  if (!agent) return res.status(404).json({ error: "agent not found" });
  return res.status(200).json(agent);
}
