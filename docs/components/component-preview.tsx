"use client";

import { useRef } from "react";
import type { ReactNode } from "react";

import { useThemeIframeSync } from "@/hooks/use-theme-iframe-sync";

/**
 * Embeds the interactive WASM demo for a maccn component.
 */
export const ComponentPreview = ({
  component,
  name,
  children,
}: {
  /** The showcase slug, e.g. `"button"`. */
  component?: string;
  /** Alias for `component`, matching the macvue preview naming. */
  name?: string;
  children?: ReactNode;
}) => {
  const slug = name ?? component ?? "";
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useThemeIframeSync(iframeRef);

  return (
    <div className="mv-preview mt-4 first:mt-0">
      <iframe
        ref={iframeRef}
        src={`/examples?component=${encodeURIComponent(slug)}`}
        title={`${slug} interactive example`}
        allow="cross-origin-isolated"
        tabIndex={-1}
      />
      {children && <div className="mv-preview-code">{children}</div>}
    </div>
  );
};
