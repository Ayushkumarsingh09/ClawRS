import type { VercelRequest, VercelResponse } from "@vercel/node";
import { applyCors, handleOptions } from "../_lib/cors";
import { dispatchV1 } from "../_lib/v1-router";

export default async function handler(req: VercelRequest, res: VercelResponse) {
  applyCors(res);
  if (handleOptions(req, res)) return;

  const raw = req.query.path;
  const segments = Array.isArray(raw) ? raw : raw ? [raw] : [];
  return dispatchV1(req, res, segments);
}
