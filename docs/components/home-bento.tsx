"use client";

import { useTheme } from "next-themes";
import { useCallback, useEffect, useRef, useState } from "react";

import { cn } from "@/lib/utils";

/**
 * Column spans per tile width, mirroring the reference grid: 12 columns on
 * desktop, 6 on tablet, 2 on phones.
 */
const SPAN = {
  2: "col-span-1 last:col-span-2 md:col-span-3 md:last:col-span-6 lg:col-span-2 lg:last:col-span-2",
  3: "col-span-2 md:col-span-3 lg:col-span-3",
  4: "col-span-2 md:col-span-6 lg:col-span-4",
  6: "col-span-2 md:col-span-6 lg:col-span-6",
} as const;

/** Converts a `#RRGGBB` CSS color into the `0xRRGGBBAA` GPUI expects. */
const toGpuiRgba = (css: string) => Number.parseInt(`${css.slice(1)}FF`, 16);

/**
 * `maccn::theme::accent`, in the order the macOS Appearance pane lists them.
 */
const ACCENTS = [
  { css: "#0088FF", name: "Blue" },
  { css: "#CB30E0", name: "Purple" },
  { css: "#FF2D55", name: "Pink" },
  { css: "#FF383C", name: "Red" },
  { css: "#FF8D28", name: "Orange" },
  { css: "#FFCC00", name: "Yellow" },
  { css: "#34C759", name: "Green" },
  { css: "#8E8E93", name: "Graphite" },
].map((accent) => ({ ...accent, hex: toGpuiRgba(accent.css) }));

const [DEFAULT_ACCENT] = ACCENTS;

const Tile = ({
  span,
  rows,
  fill,
  className,
  children,
  ...props
}: React.ComponentProps<"article"> & {
  /** Grid column span. Omit when `fill` sizes the tile instead. */
  span?: keyof typeof SPAN;
  rows?: 2;
  /**
   * Sizes the tile to fill its flex parent (`flex-1`) instead of placing it
   * on the grid directly — for tiles stacked inside a grid-positioned
   * wrapper rather than acting as a grid item themselves.
   */
  fill?: boolean;
}) => (
  <article
    className={cn(
      "flex min-w-0 flex-col justify-center gap-1.5 overflow-hidden rounded-[18px] border bg-card px-5.5 py-5 text-center transition-colors",
      fill ? "min-h-0 flex-1" : span && SPAN[span],
      rows === 2 && !fill && "row-span-2",
      className
    )}
    {...props}
  >
    {children}
  </article>
);

const TileTop = ({ children }: { children: React.ReactNode }) => (
  <p className="text-xs leading-[1.35] text-muted-foreground">{children}</p>
);

const TileLabel = ({
  className,
  children,
}: {
  className?: string;
  children: React.ReactNode;
}) => (
  <p className={cn("text-xs leading-[1.35] text-muted-foreground", className)}>
    {children}
  </p>
);

const TileFigure = ({
  small,
  accent,
  children,
}: {
  small?: boolean;
  accent?: string;
  children: React.ReactNode;
}) => (
  <strong
    className={cn(
      "block font-bold leading-none tracking-[-0.05em] transition-colors duration-200",
      small ? "text-[clamp(28px,3vw,42px)]" : "text-[clamp(38px,4.4vw,62px)]"
    )}
    style={accent ? { color: accent } : undefined}
  >
    {children}
  </strong>
);

const TileDemo = ({
  component,
  className,
  title,
}: {
  component: string;
  className?: string;
  title: string;
}) => (
  <iframe
    allow="cross-origin-isolated"
    className={cn("block h-full w-full border-0", className)}
    loading="lazy"
    src={`/examples?component=${encodeURIComponent(component)}&mode=card`}
    tabIndex={-1}
    title={title}
  />
);

const ControlTile = ({
  component,
  label,
  span,
  fill,
}: {
  component: string;
  label: string;
  span?: keyof typeof SPAN;
  fill?: boolean;
}) => (
  <Tile className="relative p-0" span={span} fill={fill}>
    <TileDemo component={component} title={`${label} demo`} />
    <TileLabel className="pointer-events-none absolute inset-x-0 bottom-3 text-foreground/60 dark:text-foreground/55">
      {label}
    </TileLabel>
  </Tile>
);

const AppearanceTile = ({
  onThemeChange,
}: {
  onThemeChange: (theme: string) => void;
}) => {
  const onRef = useRef(onThemeChange);
  onRef.current = onThemeChange;

  useEffect(() => {
    const handler = (event: MessageEvent) => {
      if (
        event.data?.type === "theme-change" &&
        (event.data.theme === "light" || event.data.theme === "dark")
      ) {
        onRef.current(event.data.theme);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  return (
    <Tile className="relative p-0 overflow-hidden" span={3}>
      <iframe
        allow="cross-origin-isolated"
        className="block h-full w-full border-0"
        loading="lazy"
        src="/examples?component=appearance-picker&mode=card"
        tabIndex={-1}
        title="Appearance picker"
      />
      <TileLabel className="pointer-events-none absolute inset-x-0 bottom-3 text-foreground/60 dark:text-foreground/55">
        Segmented Control
      </TileLabel>
    </Tile>
  );
};

/** Posts a message to every maccn WASM iframe on the page. */
const postToWasmIframes = (data: Record<string, unknown>) => {
  const iframes = document.querySelectorAll<HTMLIFrameElement>(
    "iframe[src*='/examples']"
  );
  for (const el of iframes) {
    el.contentWindow?.postMessage(data, "*");
  }
};

export const HomeBento = () => {
  const { setTheme } = useTheme();
  const [accent, setAccent] = useState(DEFAULT_ACCENT);

  const handleAccentChange = useCallback((a: (typeof ACCENTS)[number]) => {
    setAccent(a);
    postToWasmIframes({ hex: a.hex, type: "accent-change" });
  }, []);

  return (
    <div className="grid auto-rows-[140px] grid-flow-dense grid-cols-2 gap-2.5 md:grid-cols-6 lg:grid-cols-12">
      <ControlTile component="badge" label="Badge" span={2} />

      <Tile span={2}>
        <TileTop>Ships with</TileTop>
        <TileFigure accent={accent.css}>18</TileFigure>
        <TileLabel>Components</TileLabel>
      </Tile>

      <Tile span={2}>
        <TileTop>Every control in</TileTop>
        <TileFigure accent={accent.css}>5</TileFigure>
        <TileLabel>Sizes, mini to extraLarge</TileLabel>
      </Tile>

      <AppearanceTile onThemeChange={setTheme} />

      <div className={cn(SPAN[3], "row-span-2 flex flex-col gap-2.5")}>
        <ControlTile component="checkbox" label="Checkbox" fill />
        <ControlTile component="radio-group" label="Radio Group" fill />
      </div>

      <Tile className="gap-3.5" span={3}>
        <TileTop>System accents</TileTop>
        <div className="flex justify-center gap-2.5">
          {ACCENTS.map((a) => (
            <button
              aria-label={a.name}
              className={cn(
                "size-6 rounded-full border-[0.5px] border-border transition-all duration-150 hover:scale-110",
                a.hex === accent.hex && "outline outline-2 outline-offset-2"
              )}
              key={a.name}
              onClick={() => handleAccentChange(a)}
              style={{
                backgroundColor: a.css,
                ...(a.hex === accent.hex ? { outlineColor: a.css } : {}),
              }}
              title={a.name}
              type="button"
            />
          ))}
        </div>
      </Tile>

      <Tile
        className="justify-start border-transparent p-0"
        rows={2}
        span={6}
        style={{ background: "var(--background)" }}
      >
        <div className="flex h-full w-full flex-col rounded-[10px] border border-border overflow-hidden">
          <div className="relative flex h-10 shrink-0 items-center border-b border-border bg-card px-4">
            <div className="flex gap-2">
              <span className="size-3 rounded-full border-[0.5px] border-border bg-[#ff5f57]" />
              <span className="size-3 rounded-full border-[0.5px] border-border bg-[#febc2e]" />
              <span className="size-3 rounded-full border-[0.5px] border-border bg-[#28c840]" />
            </div>
            <span className="-translate-x-1/2 absolute left-1/2 text-[13px] font-semibold text-muted-foreground">
              Settings
            </span>
          </div>
          <div className="min-h-0 flex-1">
            <TileDemo component="box" title="Settings pane demo" />
          </div>
        </div>
      </Tile>

      <ControlTile component="search-field" label="Search Field" span={3} />

      <ControlTile component="button" label="Button" span={3} />

      <Tile className="justify-center" span={4}>
        <strong
          className="text-[clamp(19px,1.9vw,27px)] font-bold leading-[1.15] tracking-[-0.03em] transition-colors duration-200"
          style={{ color: accent.css }}
        >
          Measured against AppKit,
          <br />
          not guessed
        </strong>
      </Tile>

      <Tile span={4}>
        <TileTop>Built for</TileTop>
        <TileFigure small accent={accent.css}>
          GPUI
        </TileFigure>
        <TileLabel>Rust-native rendering</TileLabel>
      </Tile>

      <Tile span={2}>
        <TileTop>Written in</TileTop>
        <TileFigure small accent={accent.css}>
          Rust
        </TileFigure>
        <TileLabel>No JS runtime</TileLabel>
      </Tile>

      <Tile span={2}>
        <TileTop>Released under</TileTop>
        <TileFigure small accent={accent.css}>
          MIT
        </TileFigure>
        <TileLabel>Use it anywhere</TileLabel>
      </Tile>
    </div>
  );
};
