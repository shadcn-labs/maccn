export const FALLBACK_SITE_ORIGIN = "https://maccn.vercel.app" as const;

const getBaseUrl = () => {
  if (process.env.NODE_ENV !== "production") {
    return "http://localhost:3000";
  }

  if (process.env.VERCEL_PROJECT_PRODUCTION_URL) {
    return `https://${process.env.VERCEL_PROJECT_PRODUCTION_URL}`;
  }

  return process.env.SITE_URL ?? FALLBACK_SITE_ORIGIN;
};

const baseUrl = getBaseUrl();

export const SITE = {
  AUTHOR: {
    NAME: "Aniket Pawar",
    TWITTER: "@alaymanguy",
  },
  DESCRIPTION: {
    LONG: "macOS-inspired controls, built for GPUI on top of gpui-base.",
    SHORT: "macOS-inspired GPUI components",
  },
  KEYWORDS: [
    "gpui",
    "gpui-base",
    "rust",
    "ui",
    "components",
    "macos",
    "appkit",
  ] as const,
  NAME: "maccn",
  OG_IMAGE: `${baseUrl}/og`,
  URL: baseUrl,
};

export const META_THEME_COLORS = {
  dark: "#09090b",
  light: "#ffffff",
};

export const UTM_PARAMS = {
  utm_source: new URL(baseUrl).hostname,
};
