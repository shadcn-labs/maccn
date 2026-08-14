"use client";

import { useTheme } from "next-themes";
import { useCallback, useEffect, useRef, useState } from "react";

import type { LivePreview } from "@/lib/live-preview";
import { loadPreviewEngine } from "@/lib/live-preview";

const max_live_instances = 12;
const idle_park_ms = 600;

const liveRegistry: {
  id: number;
  visible: () => boolean;
  release: () => void;
}[] = [];
let nextLiveId = 1;

const registerLive = (visible: () => boolean, release: () => void): number => {
  const id = nextLiveId;
  nextLiveId += 1;
  liveRegistry.push({ id, release, visible });
  while (liveRegistry.length > max_live_instances) {
    const index = liveRegistry.findIndex((entry) => !entry.visible());
    const [evicted] = liveRegistry.splice(Math.max(index, 0), 1);
    evicted?.release();
  }
  return id;
};

const unregisterLive = (id: number): void => {
  const index = liveRegistry.findIndex((entry) => entry.id === id);
  if (index !== -1) {
    liveRegistry.splice(index, 1);
  }
};

const handled_keys = new Set([
  "tab",
  "enter",
  "space",
  "backspace",
  "delete",
  "arrowleft",
  "arrowright",
  "arrowup",
  "arrowdown",
  "home",
  "end",
]);
const handled_shortcut_keys = new Set(["a", "c", "x", "v"]);

const engineKeyName = (key: string): string =>
  key === "space" ? "space" : key.toLowerCase();

const engineModifiers = (event: {
  metaKey: boolean;
  ctrlKey: boolean;
  altKey: boolean;
  shiftKey: boolean;
}): number => {
  let mask = 0;
  if (event.metaKey) {
    mask = mask + 1 + 2;
  }
  if (event.ctrlKey) {
    mask += 4;
  }
  if (event.altKey) {
    mask += 8;
  }
  if (event.shiftKey) {
    mask += 16;
  }
  return mask;
};

const engineConsumesKey = (event: {
  key: string;
  metaKey: boolean;
  ctrlKey: boolean;
  altKey: boolean;
}): boolean => {
  const key = engineKeyName(event.key);
  if ((event.metaKey || event.ctrlKey) && !event.altKey) {
    return handled_shortcut_keys.has(key);
  }
  return handled_keys.has(key) || event.key.length === 1;
};

export const ComponentPreviewLive = ({
  name,
  height = 200,
}: {
  name: string;
  height?: number;
}) => {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const previewRef = useRef<LivePreview | null>(null);
  const liveIdRef = useRef(0);
  const rafRef = useRef(0);
  const lastActivityRef = useRef(0);
  const visibleRef = useRef(false);
  const [live, setLive] = useState(false);
  const [painted, setPainted] = useState(false);
  const [interactive, setInteractive] = useState(false);

  useEffect(() => setInteractive(true), []);

  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme !== "light";
  const isDarkRef = useRef(isDark);
  isDarkRef.current = isDark;

  const syncCursor = useCallback(() => {
    const preview = previewRef.current;
    const canvas = canvasRef.current;
    if (!preview || !canvas) {
      return;
    }
    const cursor = preview.cursor();
    if (canvas.style.cursor !== cursor) {
      canvas.style.cursor = cursor;
    }
  }, []);

  const blit = useCallback(() => {
    const preview = previewRef.current;
    const canvas = canvasRef.current;
    if (!preview || !canvas) {
      return false;
    }
    const cssWidth = canvas.clientWidth || canvas.getBoundingClientRect().width;
    if (cssWidth <= 0) {
      return false;
    }
    const scale =
      (cssWidth * (window.devicePixelRatio || 1)) / preview.logicalWidth;
    const imageData = preview.render(scale);
    if (!imageData) {
      return false;
    }
    if (
      canvas.width !== imageData.width ||
      canvas.height !== imageData.height
    ) {
      canvas.width = imageData.width;
      canvas.height = imageData.height;
    }
    canvas.getContext("2d")?.putImageData(imageData, 0, 0);
    setPainted(true);
    return true;
  }, []);

  const stopLoop = useCallback(() => {
    if (rafRef.current) {
      cancelAnimationFrame(rafRef.current);
      rafRef.current = 0;
    }
  }, []);

  const wake = useCallback(() => {
    lastActivityRef.current = performance.now();
    if (rafRef.current || !previewRef.current) {
      return;
    }
    const tick = (time: number) => {
      rafRef.current = 0;
      const preview = previewRef.current;
      if (!preview || !visibleRef.current || document.hidden) {
        return;
      }
      preview.setNow(time);
      syncCursor();
      if (blit()) {
        lastActivityRef.current = time;
      }
      if (time - lastActivityRef.current < idle_park_ms) {
        rafRef.current = requestAnimationFrame(tick);
      }
    };
    rafRef.current = requestAnimationFrame(tick);
  }, [blit, syncCursor]);

  const deactivate = useCallback(() => {
    stopLoop();
    if (liveIdRef.current) {
      unregisterLive(liveIdRef.current);
      liveIdRef.current = 0;
    }
    previewRef.current?.destroy();
    previewRef.current = null;
    setLive(false);
    setPainted(false);
  }, [stopLoop]);

  const activate = useCallback(async () => {
    if (previewRef.current) {
      return;
    }
    const engine = await loadPreviewEngine();
    if (!engine || previewRef.current || !containerRef.current) {
      return;
    }
    if (
      !visibleRef.current &&
      document.activeElement !== containerRef.current
    ) {
      return;
    }

    const preview = engine.create(name, isDarkRef.current);
    if (!preview) {
      return;
    }

    previewRef.current = preview;
    liveIdRef.current = registerLive(() => visibleRef.current, deactivate);
    setLive(true);
    wake();
  }, [deactivate, name, wake]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) {
      return;
    }

    const warm = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            void loadPreviewEngine();
          }
        }
      },
      { rootMargin: "600px" }
    );

    const active = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          visibleRef.current = entry.isIntersecting;
          if (entry.isIntersecting) {
            if (previewRef.current) {
              wake();
            } else {
              void activate();
            }
          } else {
            stopLoop();
            const focused = document.activeElement;
            if (
              focused !== container &&
              (!canvasRef.current || focused !== canvasRef.current)
            ) {
              deactivate();
            }
          }
        }
      },
      { rootMargin: "64px" }
    );

    warm.observe(container);
    active.observe(container);

    const onVisibility = () => {
      if (document.hidden) {
        stopLoop();
      } else if (previewRef.current && visibleRef.current) {
        wake();
      }
    };
    document.addEventListener("visibilitychange", onVisibility);

    return () => {
      warm.disconnect();
      active.disconnect();
      document.removeEventListener("visibilitychange", onVisibility);
    };
  }, [activate, deactivate, stopLoop, wake]);

  useEffect(() => {
    if (live && document.activeElement === containerRef.current) {
      canvasRef.current?.focus();
    }
  }, [live]);

  useEffect(() => {
    const preview = previewRef.current;
    if (!preview || !live) {
      return;
    }
    preview.setTheme(isDark);
    wake();
  }, [isDark, live, wake]);

  useEffect(() => {
    if (!live) {
      return;
    }
    const canvas = canvasRef.current;
    if (!canvas) {
      return;
    }
    const observer = new ResizeObserver(() => wake());
    observer.observe(canvas);
    return () => observer.disconnect();
  }, [live, wake]);

  const onPointerDown = useCallback(
    (event: React.PointerEvent) => {
      if (!live) {
        return;
      }
      const canvas = canvasRef.current;
      if (!canvas) {
        return;
      }
      canvas.focus();
      const rect = canvas.getBoundingClientRect();
      previewRef.current?.pointer(
        0,
        event.clientX - rect.left,
        event.clientY - rect.top
      );
      wake();
    },
    [live, wake]
  );

  const onPointerMove = useCallback(
    (event: React.PointerEvent) => {
      if (!live) {
        return;
      }
      const canvas = canvasRef.current;
      if (!canvas) {
        return;
      }
      const rect = canvas.getBoundingClientRect();
      previewRef.current?.pointer(
        1,
        event.clientX - rect.left,
        event.clientY - rect.top
      );
      wake();
    },
    [live, wake]
  );

  const onPointerUp = useCallback(
    (event: React.PointerEvent) => {
      if (!live) {
        return;
      }
      const canvas = canvasRef.current;
      if (!canvas) {
        return;
      }
      const rect = canvas.getBoundingClientRect();
      previewRef.current?.pointer(
        2,
        event.clientX - rect.left,
        event.clientY - rect.top
      );
      wake();
    },
    [live, wake]
  );

  const onKeyDown = useCallback(
    (event: React.KeyboardEvent) => {
      if (!live) {
        return;
      }
      if (!engineConsumesKey(event)) {
        return;
      }
      event.preventDefault();
      event.stopPropagation();
      previewRef.current?.key(event.key, engineModifiers(event));
      wake();
    },
    [live, wake]
  );

  const onWheel = useCallback(
    (event: React.WheelEvent) => {
      if (!live) {
        return;
      }
      event.preventDefault();
      previewRef.current?.scroll(event.deltaX, event.deltaY);
      wake();
    },
    [live, wake]
  );

  return (
    <div
      ref={containerRef}
      className="mv-preview"
      tabIndex={interactive ? 0 : undefined}
      role="img"
      aria-label={`${name} live preview`}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onKeyDown={onKeyDown}
      onWheel={onWheel}
    >
      <div
        className="flex items-center justify-center"
        style={{ minHeight: height }}
      >
        {!live && !painted && (
          <div className="p-8 text-sm text-muted-foreground">Loading WASM…</div>
        )}
        <canvas
          ref={canvasRef}
          className={`block ${live ? "" : "hidden"}`}
          style={{ height, imageRendering: "pixelated", width: "100%" }}
        />
      </div>
    </div>
  );
};
