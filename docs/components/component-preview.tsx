"use client";

import dynamic from "next/dynamic";
import { usePathname } from "next/navigation";
import { useEffect, useRef } from "react";
import type { ReactNode } from "react";

import { ROUTES } from "@/constants/routes";

const ComponentPreviewLive = dynamic(
  () =>
    import("@/components/component-preview-live").then(
      (mod) => mod.ComponentPreviewLive
    ),
  { ssr: false }
);

function detectBase(pathname: string): "gpui" | "native-sdk" {
  if (pathname.startsWith(ROUTES.DOCS_COMPONENTS_NATIVE_SDK)) {
    return "native-sdk";
  }
  return "gpui";
}

/**
 * Embeds the interactive WASM demo for a maccn component.
 *
 * Both bases render inside an identical .mv-preview container:
 * - "gpui": GPUI app in an iframe
 * - "native-sdk": lightweight WASM engine on a <canvas>
 */
export const ComponentPreview = ({
  component,
  children,
}: {
  /** The showcase slug, e.g. `"button"`. */
  component: string;
  children?: ReactNode;
}) => {
  const pathname = usePathname();
  const base = detectBase(pathname);
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useEffect(() => {
    if (base !== "gpui") {
      return;
    }
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

    observer.observe(html, { attributeFilter: ["class"], attributes: true });
    return () => observer.disconnect();
  }, [base]);

  return (
    <div className="mv-preview">
      {base === "gpui" ? (
        <iframe
          ref={iframeRef}
          src={`/examples?component=${encodeURIComponent(component)}`}
          title={`${component} interactive example`}
          allow="cross-origin-isolated"
        />
      ) : (
        <ComponentPreviewLive name={component} />
      )}
      {children && <div className="mv-preview-code">{children}</div>}
    </div>
  );
};
