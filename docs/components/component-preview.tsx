"use client";

import { useEffect, useRef } from "react";
import type { ReactNode } from "react";

/**
 * Embeds the interactive WASM demo for a maccn component.
 *
 * The demo is a GPUI app compiled to WebAssembly and built into
 * `public/examples` (see `crates/maccn/examples/wasm`). Each component page
 * passes its slug as `component`, and the demo renders it inside a window
 * frame that mirrors a native macOS window.
 *
 * A MutationObserver on <html> watches for dark/light class changes and
 * reloads the iframe so the WASM demo re-detects the system theme.
 */
export const ComponentPreview = ({
  component,
  children,
}: {
  /** The showcase slug, e.g. `"button"`. */
  component: string;
  children?: ReactNode;
}) => {
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useEffect(() => {
    const html = document.documentElement;
    let lastClass = html.className;

    const observer = new MutationObserver(() => {
      if (html.className !== lastClass) {
        lastClass = html.className;
        const iframe = iframeRef.current;
        if (iframe) {
          iframe.src = iframe.src;
        }
      }
    });

    observer.observe(html, { attributes: true, attributeFilter: ["class"] });
    return () => observer.disconnect();
  }, []);

  return (
    <div className="mv-preview">
      <iframe
        ref={iframeRef}
        src={`/examples?component=${encodeURIComponent(component)}`}
        title={`${component} interactive example`}
        allow="cross-origin-isolated"
      />
      {children && <div className="mv-preview-code">{children}</div>}
    </div>
  );
};
