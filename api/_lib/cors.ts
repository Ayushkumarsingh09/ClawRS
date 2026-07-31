import type { VercelResponse } from "@vercel/node";

export function applyCors(
  res: VercelResponse,
  methods = "GET, POST, OPTIONS",
): void {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", methods);
  res.setHeader("Access-Control-Allow-Headers", "Content-Type, Authorization, X-ClawRS-Key");
}

export function handleOptions(req: { method?: string }, res: VercelResponse): boolean {
  if (req.method === "OPTIONS") {
    res.status(204).end();
    return true;
  }
  return false;
}
