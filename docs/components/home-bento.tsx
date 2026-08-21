"use client";

import { useRef } from "react";
import { useTheme } from "next-themes";
import Link from "next/link";

import { ROUTES } from "@/constants/routes";
import { useFeedback } from "@/hooks/use-feedback";
import { useMounted } from "@/hooks/use-mounted";
import { useThemeIframeSync } from "@/hooks/use-theme-iframe-sync";
import { cn } from "@/lib/utils";

/**
 * Column spans per tile width, mirroring the reference grid: 12 columns on
 * desktop, 6 on tablet, 2 on phones.
 */
const SPAN = {
  // `last:` mirrors the reference's `.mv-tile--w2:last-child`: with an odd
  // count of half-width tiles the narrow layouts would otherwise end on a hole.
  2: "col-span-1 last:col-span-2 md:col-span-3 md:last:col-span-6 lg:col-span-2 lg:last:col-span-2",
  3: "col-span-2 md:col-span-3 lg:col-span-3",
  4: "col-span-2 md:col-span-6 lg:col-span-4",
  6: "col-span-2 md:col-span-6 lg:col-span-6",
} as const;

/**
 * The WASM canvases paint macOS window chrome from the OS colour scheme, not
 * from the docs theme, so any chrome drawn in the DOM next to one has to key
 * off `prefers-color-scheme` too or the seam shows. Values are
 * `MaccnTheme::{light,dark}` in `crates/maccn/src/theme.rs`.
 */
const OS_SURFACE =
  "bg-[#f6f6f6] [@media(prefers-color-scheme:dark)]:bg-[#1e1e1e]";
const OS_SEPARATOR =
  "border-black/10 [@media(prefers-color-scheme:dark)]:border-white/10";
const OS_LABEL_SECONDARY =
  "text-black/50 [@media(prefers-color-scheme:dark)]:text-white/55";

/** `MaccnTheme::light().accent` / `MaccnTheme::dark().accent`. */
const ACCENT_TEXT = "text-[#0088ff] dark:text-[#0091ff]";

/** `maccn::theme::accent`, in the order the macOS Appearance pane lists them. */
const ACCENTS = [
  { color: "#0088FF", name: "Blue" },
  { color: "#CB30E0", name: "Purple" },
  { color: "#FF2D55", name: "Pink" },
  { color: "#FF383C", name: "Red" },
  { color: "#FF8D28", name: "Orange" },
  { color: "#FFCC00", name: "Yellow" },
  { color: "#34C759", name: "Green" },
  { color: "#8E8E93", name: "Graphite" },
];

/** The keynote-stage backdrop shared by the two showcase tiles. */
const STAGE_BACKGROUND = {
  background: [
    "radial-gradient(120% 90% at 20% 0%, rgb(94 63 255 / 0.34), transparent 62%)",
    "radial-gradient(110% 80% at 88% 100%, rgb(10 132 255 / 0.30), transparent 58%)",
    "#0a0a0d",
  ].join(","),
};

const Tile = ({
  span,
  rows,
  className,
  children,
  ...props
}: React.ComponentProps<"article"> & {
  span: keyof typeof SPAN;
  rows?: 2;
}) => (
  <article
    className={cn(
      "flex min-w-0 flex-col justify-center gap-1.5 overflow-hidden rounded-[18px] border bg-card px-5.5 py-5 text-center transition-colors",
      SPAN[span],
      rows === 2 && "row-span-2",
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
  children,
}: {
  small?: boolean;
  children: React.ReactNode;
}) => (
  <strong
    className={cn(
      "block font-bold leading-none tracking-[-0.05em]",
      ACCENT_TEXT,
      small ? "text-[clamp(28px,3vw,42px)]" : "text-[clamp(38px,4.4vw,62px)]"
    )}
  >
    {children}
  </strong>
);

/**
 * A live WASM control, in the showcase's `card` mode: one control, centred, no
 * chrome. Lazy so the 18 MB module is only instantiated once a tile is near
 * the viewport.
 */
const TileDemo = ({
  component,
  className,
  title,
}: {
  component: string;
  className?: string;
  title: string;
}) => {
  const iframeRef = useRef<HTMLIFrameElement>(null);
  useThemeIframeSync(iframeRef);

  return (
    <iframe
      ref={iframeRef}
      allow="cross-origin-isolated"
      className={cn("block h-full w-full border-0", className)}
      loading="lazy"
      src={`/examples?component=${encodeURIComponent(component)}&mode=card`}
      tabIndex={-1}
      title={title}
    />
  );
};

/**
 * A control tile: the canvas fills the tile edge-to-edge — the demo paints its
 * own window background, so no card padding may show around it — and the
 * caption is overlaid on top of it.
 */
const ControlTile = ({
  component,
  label,
  span,
}: {
  component: string;
  label: string;
  span: keyof typeof SPAN;
}) => (
  <Tile className="relative p-0" span={span}>
    <TileDemo component={component} title={`${label} demo`} />
    <TileLabel
      className={cn(
        "pointer-events-none absolute inset-x-0 bottom-3",
        OS_LABEL_SECONDARY
      )}
    >
      {label}
    </TileLabel>
  </Tile>
);

/** Light/Dark, wired to the real site theme rather than faked. */
const AppearanceTile = () => {
  const { resolvedTheme, setTheme } = useTheme();
  const isMounted = useMounted();
  const feedbackOn = useFeedback({ sound: "toggleOn" });
  const feedbackOff = useFeedback({ sound: "toggleOff" });

  return (
    <Tile className="items-center gap-4" span={3}>
      <div
        aria-label="Appearance"
        className="inline-flex items-center gap-0.5 rounded-lg bg-muted p-0.5"
        role="radiogroup"
      >
        {(["light", "dark"] as const).map((appearance) => {
          const isActive = isMounted && resolvedTheme === appearance;

          return (
            <button
              aria-checked={isActive}
              className={cn(
                "rounded-md px-4 py-1 text-sm capitalize transition-colors",
                isActive
                  ? "bg-background text-foreground shadow-sm"
                  : "text-muted-foreground hover:text-foreground"
              )}
              key={appearance}
              onClick={() => {
                if (appearance === "dark") {
                  feedbackOff();
                } else {
                  feedbackOn();
                }
                setTheme(appearance);
              }}
              role="radio"
              type="button"
            >
              {appearance}
            </button>
          );
        })}
      </div>
      <TileLabel>Both appearances, one token set</TileLabel>
    </Tile>
  );
};

export const HomeBento = () => (
  <section aria-labelledby="home-bento-title" className="relative py-16">
    {/* Colour, then grain over it: the flat gradient alone reads as a CSS
        wash, and the tiles need something to sit on. */}
    <div
      aria-hidden="true"
      className="pointer-events-none absolute inset-0 opacity-50 dark:opacity-100"
      style={{
        background: [
          "radial-gradient(46% 40% at 22% 28%, rgb(94 63 255 / 0.18), transparent 72%)",
          "radial-gradient(44% 42% at 78% 60%, rgb(10 132 255 / 0.16), transparent 72%)",
          "radial-gradient(38% 34% at 50% 100%, rgb(191 90 242 / 0.12), transparent 74%)",
        ].join(","),
      }}
    />
    <div
      aria-hidden="true"
      className="pointer-events-none absolute inset-0 opacity-[0.035]"
      style={{
        backgroundImage:
          "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='180' height='180'%3E%3Cfilter id='g'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.75' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='180' height='180' filter='url(%23g)'/%3E%3C/svg%3E\")",
      }}
    />

    <div className="relative mb-8 flex flex-col items-start justify-between gap-4 sm:flex-row sm:items-end sm:gap-8">
      <div>
        <p className="mb-3 text-xs font-semibold uppercase tracking-[0.08em] text-muted-foreground">
          What you get
        </p>
        <h2
          className="max-w-160 text-[clamp(38px,5vw,64px)] font-bold leading-[0.96] tracking-[-0.045em]"
          id="home-bento-title"
        >
          The whole control set.
        </h2>
      </div>
      <Link
        className="shrink-0 pb-1 text-sm font-medium text-muted-foreground underline-offset-4 hover:text-foreground hover:underline"
        href={ROUTES.DOCS_COMPONENTS}
        transitionTypes={["nav-forward"]}
      >
        Explore all components →
      </Link>
    </div>

    <div className="relative grid auto-rows-[140px] grid-flow-dense grid-cols-2 gap-2.5 md:grid-cols-6 lg:grid-cols-12">
      <ControlTile component="switch" label="Switch" span={2} />

      <Tile span={2}>
        <TileTop>Ships with</TileTop>
        <TileFigure>19</TileFigure>
        <TileLabel>Components</TileLabel>
      </Tile>

      <Tile span={2}>
        <TileTop>Every control in</TileTop>
        <TileFigure>5</TileFigure>
        <TileLabel>Sizes, mini to extraLarge</TileLabel>
      </Tile>

      <AppearanceTile />

      <Tile className="gap-0 p-0" rows={2} span={3}>
        <div className="px-5.5 pt-6 pb-4">
          <strong className="block text-[clamp(17px,1.5vw,21px)] font-semibold leading-[1.15] tracking-[-0.03em]">
            Liquid Glass panels, tuned for GPUI
          </strong>
          <TileLabel className="mt-2">Regular and Clear materials</TileLabel>
        </div>
        <div className={cn("min-h-0 flex-1 border-t", OS_SEPARATOR)}>
          <TileDemo component="glass-panel" title="Glass panel demo" />
        </div>
      </Tile>

      <Tile className="gap-3.5" span={3}>
        <TileTop>System accents</TileTop>
        <div className="flex justify-center gap-2.5">
          {ACCENTS.map((accent) => (
            <span
              aria-label={accent.name}
              className="size-6 rounded-full border-[0.5px] border-black/15"
              key={accent.name}
              style={{ backgroundColor: accent.color }}
              title={accent.name}
            />
          ))}
        </div>
      </Tile>

      {/* Centrepiece: a macOS window cropped by the tile, the way a keynote
          slide crops the thing it is showing off. */}
      <Tile
        className="justify-start border-transparent p-0"
        rows={2}
        span={6}
        style={STAGE_BACKGROUND}
      >
        {/* Pushed down so the tile crops the pane mid-group rather than ending
            flush with the tile edge, which reads as an accident. */}
        <div className="mx-auto mt-14 w-[min(94%,520px)] overflow-hidden rounded-t-[10px] border border-b-0 border-black/20 shadow-[0_2px_4px_rgb(0_0_0/0.28),0_26px_52px_rgb(0_0_0/0.44)]">
          <div
            className={cn(
              "relative flex h-10 items-center border-b px-4",
              OS_SURFACE,
              OS_SEPARATOR
            )}
          >
            <div className="flex gap-2">
              <span className="size-3 rounded-full border-[0.5px] border-black/15 bg-[#ff5f57]" />
              <span className="size-3 rounded-full border-[0.5px] border-black/15 bg-[#febc2e]" />
              <span className="size-3 rounded-full border-[0.5px] border-black/15 bg-[#28c840]" />
            </div>
            <span
              className={cn(
                "-translate-x-1/2 absolute left-1/2 text-[13px] font-semibold",
                OS_LABEL_SECONDARY
              )}
            >
              Settings
            </span>
          </div>
          <TileDemo
            className="h-55"
            component="box"
            title="Settings pane demo"
          />
        </div>
      </Tile>

      <ControlTile component="search-field" label="Search Field" span={3} />

      <ControlTile component="stepper" label="Stepper" span={3} />

      <Tile className="justify-center" span={4}>
        <strong
          className={cn(
            "text-[clamp(19px,1.9vw,27px)] font-bold leading-[1.15] tracking-[-0.03em]",
            ACCENT_TEXT
          )}
        >
          Measured against AppKit,
          <br />
          not guessed
        </strong>
      </Tile>

      <Tile span={4}>
        <TileTop>Built for</TileTop>
        <TileFigure small>GPUI</TileFigure>
        <TileLabel>Rust-native rendering</TileLabel>
      </Tile>

      <Tile span={2}>
        <TileTop>Written in</TileTop>
        <TileFigure small>Rust</TileFigure>
        <TileLabel>No JS runtime</TileLabel>
      </Tile>

      <Tile span={2}>
        <TileTop>Released under</TileTop>
        <TileFigure small>MIT</TileFigure>
        <TileLabel>Use it anywhere</TileLabel>
      </Tile>
    </div>
  </section>
);
