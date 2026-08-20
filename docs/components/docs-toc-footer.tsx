"use client";

import { SquarePenIcon } from "lucide-react";

import { DiscordIcon, XIcon } from "@/components/icons";
import { GITHUB, LINK } from "@/constants/links";
import { DOCS_DIR } from "@/lib/docs";
import { trackEvent } from "@/lib/events";
import { cn } from "@/lib/utils";

export const DocsTocFooter = ({
  docId,
  className,
}: {
  docId: string;
  className?: string;
}) => (
  <div className={cn("flex flex-col gap-2", className)}>
    {docId && (
      <a
        href={`${LINK.GITHUB}/edit/${GITHUB.branch}/docs/${DOCS_DIR}/${docId}`}
        target="_blank"
        rel="noopener noreferrer"
        className="text-muted-foreground flex items-center gap-1.5 text-[0.8rem] transition-colors hover:text-foreground [&_svg]:size-3"
        onClick={() =>
          trackEvent({
            name: "click_edit_page",
            properties: { doc: docId },
          })
        }
      >
        <SquarePenIcon />
        Edit this page
      </a>
    )}
    <a
      href={LINK.X_SHADCN_LABS}
      target="_blank"
      rel="noopener noreferrer"
      className="text-muted-foreground flex items-center gap-1.5 text-[0.8rem] transition-colors hover:text-foreground [&_svg]:size-3"
    >
      <XIcon />
      Follow @shadcnlabs
    </a>
    <a
      href={LINK.DISCORD}
      target="_blank"
      rel="noopener noreferrer"
      className="text-muted-foreground flex items-center gap-1.5 text-[0.8rem] transition-colors hover:text-foreground [&_svg]:size-3"
    >
      <DiscordIcon />
      Join community
    </a>
  </div>
);
