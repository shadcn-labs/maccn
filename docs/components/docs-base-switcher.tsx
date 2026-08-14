"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

import { ROUTES } from "@/constants/routes";
import { cn } from "@/lib/utils";

const bases = [
  { label: "GPUI", name: "gpui" as const, prefix: ROUTES.DOCS_COMPONENTS_GPUI },
  {
    label: "Native SDK",
    name: "native-sdk" as const,
    prefix: ROUTES.DOCS_COMPONENTS_NATIVE_SDK,
  },
];

function detectBase(pathname: string): "gpui" | "native-sdk" | null {
  if (pathname.startsWith(ROUTES.DOCS_COMPONENTS_GPUI)) {
    return "gpui";
  }
  if (pathname.startsWith(ROUTES.DOCS_COMPONENTS_NATIVE_SDK)) {
    return "native-sdk";
  }
  return null;
}

function getComponentSlug(pathname: string, basePrefix: string): string | null {
  const rest = pathname.slice(basePrefix.length);
  const segments = rest.split("/").filter(Boolean);
  return segments[0] ?? null;
}

export const DocsBaseSwitcher = ({ className }: { className?: string }) => {
  const pathname = usePathname();
  const activeBase = detectBase(pathname);

  if (!activeBase) {
    return null;
  }

  const componentSlug = getComponentSlug(
    pathname,
    bases.find((b) => b.name === activeBase)!.prefix
  );

  return (
    <div
      className={cn(
        "inline-flex w-full items-center gap-6 border-b pb-3",
        className
      )}
    >
      {bases.map((base) => {
        const isActive = base.name === activeBase;
        const href = componentSlug
          ? `${base.prefix}/${componentSlug}`
          : base.prefix;

        return (
          <Link
            key={base.name}
            href={href}
            data-active={isActive}
            className="relative inline-flex items-center justify-center gap-1 pt-1 pb-0.5 text-base font-medium text-muted-foreground transition-colors after:absolute after:inset-x-0 after:bottom-[-4px] after:h-0.5 after:bg-foreground after:opacity-0 after:transition-opacity hover:text-foreground data-[active=true]:text-foreground data-[active=true]:after:opacity-100"
          >
            {base.label}
          </Link>
        );
      })}
    </div>
  );
};
