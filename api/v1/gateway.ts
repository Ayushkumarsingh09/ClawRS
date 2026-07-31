import type { VercelRequest, VercelResponse } from "@vercel/node";
import { applyCors, handleOptions } from "../_lib/cors";
import { dispatchV1 } from "../_lib/v1-router";

function routeSegments(req: VercelRequest): string[] {
  const route = req.query.route;
  if (typeof route === "string" && route.length > 0) {
    return route.split("/").filter(Boolean);
  }
  if (Array.isArray(route)) {
    return route.flatMap((r) => String(r).split("/")).filter(Boolean);
  }

  const rawUrl = req.url ?? "";
  const path = rawUrl.split("?")[0];
  const prefix = "/api/v1/";
  if (path.startsWith(prefix)) {
    return path.slice(prefix.length).split("/").filter(Boolean);
  }
  return [];
}

/** Single entry for all /api/v1/* routes (shared in-memory store on warm instances). */
export default async function handler(req: VercelRequest, res: VercelResponse) {
  applyCors(res);
  if (handleOptions(req, res)) return;
  return dispatchV1(req, res, routeSegments(req));
}
