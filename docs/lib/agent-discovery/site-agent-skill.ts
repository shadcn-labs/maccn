import { createHash } from "node:crypto";

import { ROUTES } from "@/constants/routes";
import { SITE } from "@/constants/site";

export const SITE_AGENT_SKILL_MD = `# ${SITE.NAME}

## Summary

Help users discover macOS-inspired GPUI components and their documentation.

## Docs

- Docs: ${ROUTES.DOCS}
- Components: ${ROUTES.DOCS_COMPONENTS}

## Demo

Interactive demos run as WebAssembly at /examples/?component=<name>.

Prefer following the on-site installation guide: ${ROUTES.DOCS}
`;

export const siteAgentSkillDigest = (): string => {
  const hex = createHash("sha256")
    .update(SITE_AGENT_SKILL_MD, "utf-8")
    .digest("hex");

  return `sha256:${hex}`;
};
