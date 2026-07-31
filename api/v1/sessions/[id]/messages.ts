import type { VercelRequest, VercelResponse } from "@vercel/node";
import { applyCors, handleOptions } from "../../../_lib/cors";
import { store } from "../../../_lib/store";

export default function handler(req: VercelRequest, res: VercelResponse) {
  applyCors(res);
  if (handleOptions(req, res)) return;

  const sessionId = req.query.id as string;
  const messages = store().messages[sessionId] ?? [];
  return res.status(200).json({ messages });
}
