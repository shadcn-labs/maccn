"use client";

import { cn } from "@/lib/utils";

const BentoCard = ({
  className,
  children,
  ...props
}: React.ComponentProps<"div">) => (
  <div
    className={cn(
      "group relative flex flex-col justify-end overflow-hidden rounded-2xl border bg-card text-card-foreground shadow-sm transition-colors",
      className
    )}
    {...props}
  >
    {children}
  </div>
);

const BentoIframe = ({
  slug,
  className,
}: {
  slug: string;
  className?: string;
}) => (
  <iframe
    src={`/examples?component=${encodeURIComponent(slug)}`}
    title={`${slug} demo`}
    allow="cross-origin-isolated"
    className={cn("pointer-events-none h-full w-full border-0", className)}
  />
);

const StatCard = ({
  label,
  value,
  description,
  className,
}: {
  label: string;
  value: string;
  description: string;
  className?: string;
}) => (
  <BentoCard
    className={cn("items-center justify-center p-6 text-center", className)}
  >
    <div className="text-sm text-muted-foreground">{label}</div>
    <div className="mt-1 text-5xl font-bold tracking-tight">{value}</div>
    <div className="mt-1 text-sm text-muted-foreground">{description}</div>
  </BentoCard>
);

export const HomeBento = () => (
  <div className="grid grid-cols-2 gap-4 md:grid-cols-4 lg:grid-cols-5">
    {/* Row 1 */}
    <BentoCard className="col-span-1 items-center justify-center overflow-hidden p-0">
      <BentoIframe slug="switch/Basic" />
    </BentoCard>

    <StatCard label="Ships with" value="19" description="Components" />

    <StatCard
      label="Every control in"
      value="5"
      description="Sizes, mini to extraLarge"
    />

    <BentoCard className="col-span-1 items-center justify-center text-center">
      <div className="flex gap-1 rounded-lg border bg-muted p-1">
        <span className="rounded-md px-3 py-1 text-sm">Light</span>
        <span className="rounded-md bg-primary px-3 py-1 text-sm text-primary-foreground">
          Dark
        </span>
      </div>
      <div className="mt-3 text-sm text-muted-foreground">
        Both appearances, one token set
      </div>
    </BentoCard>

    <BentoCard className="col-span-1 row-span-2 flex flex-col items-center justify-center overflow-hidden p-6 text-center md:col-span-2 lg:col-span-1">
      <div className="text-xl font-bold">Liquid Glass with real refraction</div>
      <div className="mt-4 w-full overflow-hidden">
        <BentoIframe slug="slider/Basic" className="h-12" />
      </div>
      <div className="mt-3 text-sm text-muted-foreground">
        Refraction in Chromium, material fallback elsewhere
      </div>
    </BentoCard>

    {/* Row 2 */}
    <BentoCard className="col-span-1 items-center justify-center text-center">
      <div className="text-sm text-muted-foreground">System accents</div>
      <div className="mt-3 flex gap-1.5">
        {[
          "bg-blue-500",
          "bg-purple-500",
          "bg-pink-500",
          "bg-red-500",
          "bg-orange-500",
          "bg-yellow-500",
          "bg-green-500",
          "bg-gray-400",
        ].map((color) => (
          <div key={color} className={cn("h-6 w-6 rounded-full", color)} />
        ))}
      </div>
    </BentoCard>

    {/* Center: Settings panel with real demos */}
    <BentoCard className="col-span-2 row-span-2 overflow-hidden p-0">
      <div className="flex items-center gap-2 border-b px-4 py-3">
        <div className="flex gap-1.5">
          <div className="h-3 w-3 rounded-full bg-red-500" />
          <div className="h-3 w-3 rounded-full bg-yellow-500" />
          <div className="h-3 w-3 rounded-full bg-green-500" />
        </div>
        <div className="flex-1 text-center text-sm font-medium">Settings</div>
      </div>
      <div className="flex-1">
        <BentoIframe slug="box/Basic" className="h-full min-h-[280px]" />
      </div>
    </BentoCard>

    <BentoCard className="col-span-1 overflow-hidden p-0">
      <BentoIframe slug="progress/Basic" />
    </BentoCard>

    {/* Row 3 */}
    <BentoCard className="col-span-1 overflow-hidden p-0">
      <BentoIframe slug="search-field/Basic" />
    </BentoCard>

    <BentoCard className="col-span-1 overflow-hidden p-0">
      <BentoIframe slug="stepper/Basic" />
    </BentoCard>

    {/* Measured against AppKit */}
    <BentoCard className="col-span-2 items-center justify-center p-6 text-center md:col-span-1">
      <div className="text-lg font-bold text-primary">
        Measured against AppKit, not guessed
      </div>
    </BentoCard>

    {/* Bottom row */}
    <StatCard
      label="Built for"
      value="GPUI"
      description="Rust-native rendering"
    />

    <StatCard
      label="Whole library"
      value="0 KB"
      description="Zero JS runtime required"
    />

    <BentoCard className="col-span-1 items-center justify-center p-6 text-center">
      <div className="text-sm text-muted-foreground">Released under</div>
      <div className="mt-1 text-5xl font-bold tracking-tight">MIT</div>
      <div className="mt-1 text-sm text-muted-foreground">Use it anywhere</div>
    </BentoCard>
  </div>
);
