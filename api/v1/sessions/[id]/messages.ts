import type { VercelRequest, VercelResponse } from "@vercel/node";
import { store } from "../../../_lib/store";

function cors(res: VercelResponse) {
  res.setHeader("Access-Control-Allow-Origin", "*");
}

export default function handler(req: VercelRequest, res: VercelResponse) {
  cors(res);
  const sessionId = req.query.id as string;
  const messages = store().messages[sessionId] ?? [];
  return res.status(200).json({ messages });
}
