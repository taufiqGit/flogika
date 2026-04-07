// src/middleware.ts
import { defineMiddleware } from "astro:middleware";

export const onRequest = defineMiddleware(async (context, next) => {
  const { url, request, redirect } = context;

  // In static builds, request headers are unavailable on prerendered pages.
  // Production locale redirect is handled by the Rust server.
  if (!import.meta.env.DEV) return next();

  if (url.pathname !== "/") return next();

  // Common platform headers:
  const country =
    request.headers.get("x-vercel-ip-country") || // Vercel
    request.headers.get("cf-ipcountry") ||        // Cloudflare
    request.headers.get("x-country-code");        // custom proxy

  const acceptLanguage = request.headers.get("accept-language") || "";
  const isIndonesianLang = /\bid\b/i.test(acceptLanguage);

  if (country === "ID" || isIndonesianLang) {
    return redirect("/id", 307);
  }

  return next();
});
