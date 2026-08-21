import { useEffect } from "react";
import type { RefObject } from "react";

/**
 * Watches the `<html>` class for theme changes and reloads the iframe so the
 * WASM demo picks up the new colour scheme instantly.
 */
export const useThemeIframeSync = (
  ref: RefObject<HTMLIFrameElement | null>
) => {
  useEffect(() => {
    const html = document.documentElement;
    let lastClass = html.className;

    const observer = new MutationObserver(() => {
      if (html.className !== lastClass) {
        lastClass = html.className;
        const iframe = ref.current;
        if (iframe) {
          const currentSrc = iframe.src;
          iframe.src = currentSrc;
        }
      }
    });

    observer.observe(html, { attributeFilter: ["class"], attributes: true });
    return () => observer.disconnect();
  }, [ref]);
};
