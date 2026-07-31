import type { VercelRequest, VercelResponse } from "@vercel/node";
import { applyCors, handleOptions } from "../_lib/cors";
import { store } from "../_lib/store";

export default function handler(req: VercelRequest, res: VercelResponse) {
  applyCors(res);
  if (handleOptions(req, res)) return;

  const s = store();
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
}
