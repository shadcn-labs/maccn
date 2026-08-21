"use client";

import type { ReactNode } from "react";

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

  return (
    <div className="mv-preview mt-4 first:mt-0">
      <iframe
        src={`/examples?component=${encodeURIComponent(slug)}`}
        title={`${slug} interactive example`}
        allow="cross-origin-isolated"
        tabIndex={-1}
      />
      {children && <div className="mv-preview-code">{children}</div>}
    </div>
  );
};
