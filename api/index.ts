import type { VercelRequest, VercelResponse } from "@vercel/node";

export default function handler(_req: VercelRequest, res: VercelResponse) {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.status(200).json({
    name: "ClawRS API",
    version: "0.1.0-vercel",
    health: "/health",
    status: "/api/v1/status",
    repository: "https://github.com/Ayushkumarsingh09/ClawRS",
  });
}
