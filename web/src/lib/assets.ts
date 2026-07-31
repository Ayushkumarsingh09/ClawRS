/** Base URL for static assets (GitHub Pages subpath-safe). */
export const assetUrl = (path: string) =>
  `${import.meta.env.BASE_URL}${path.replace(/^\//, "")}`;

export const LOGO_URL = assetUrl("logo.png");
