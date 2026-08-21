"use client";

import { useEffect, useRef } from "react";
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
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useEffect(() => {
    const html = document.documentElement;
    let lastClass = html.className;

    const observer = new MutationObserver(() => {
      if (html.className !== lastClass) {
        lastClass = html.className;
        const iframe = iframeRef.current;
        if (iframe) {
          iframe.src = `${iframe.src}`;
        }
      }
    });

    observer.observe(html, { attributeFilter: ["class"], attributes: true });
    return () => observer.disconnect();
  }, []);

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
