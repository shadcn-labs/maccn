"use client";

import dynamic from "next/dynamic";
import Link from "next/link";
import { usePathname } from "next/navigation";

import { ROUTES } from "@/constants/routes";
import type { PageTreePage } from "@/lib/page-tree";
import { source } from "@/lib/source";

const ComponentPreviewLive = dynamic(
  () =>
    import("@/components/component-preview-live").then(
      (mod) => mod.ComponentPreviewLive
    ),
  { ssr: false }
);

const COMPONENT_DESCRIPTIONS: Record<string, string> = {
  badge: "A small status indicator or label.",
  box: "A grouped container with rounded corners and padding.",
  button: "A bordered, prominent, or destructive push button.",
  checkbox: "A tri-state checkbox with check and indeterminate marks.",
  "glass-panel": "A translucent panel with blur and vibrancy effects.",
  "help-button": "A circular help button with a question mark glyph.",
  label: "A text label with macOS typography styles.",
  "pop-up-button": "A dropdown button that opens a popover menu.",
  progress:
    "A horizontal progress bar with determinate and indeterminate modes.",
  "radio-group": "A group of mutually exclusive radio buttons.",
  "search-field": "A search input with magnifier icon and clear button.",
  "secure-field": "A masked password input field.",
  "segmented-control": "A segmented control for switching between views.",
  separator: "A horizontal or vertical dividing line.",
  slider: "A draggable slider for selecting a value in a range.",
  spinner: "An animated loading indicator with spinning blades.",
  stepper: "An increment/decrement control with up and down chevrons.",
  switch: "A toggle switch with smooth animation.",
  "text-field": "A text input with placeholder, prefix, and suffix support.",
};

function getFolderPages(folderName: string): PageTreePage[] {
  for (const node of source.pageTree.children) {
    if (node.type === "folder" && node.name === folderName) {
      return node.children.filter(
        (child): child is PageTreePage =>
          child.type === "page" && child.name !== "index"
      );
    }
  }
  return [];
}

function detectBase(pathname: string): "gpui" | "native-sdk" | null {
  if (pathname.startsWith(ROUTES.DOCS_COMPONENTS_GPUI)) {
    return "gpui";
  }
  if (pathname.startsWith(ROUTES.DOCS_COMPONENTS_NATIVE_SDK)) {
    return "native-sdk";
  }
  return null;
}

const ComponentCard = ({
  component,
  base,
}: {
  component: PageTreePage;
  base: "gpui" | "native-sdk";
}) => (
  <Link
    className="group flex flex-col gap-3 rounded-xl border bg-card p-4 transition-colors hover:bg-accent/50"
    href={component.url}
    key={component.$id}
  >
    <div className="flex h-[140px] items-center justify-center overflow-hidden rounded-lg bg-muted/50">
      {base === "native-sdk" ? (
        <ComponentPreviewLive name={String(component.name)} height={120} />
      ) : (
        <div className="flex items-center justify-center text-xs text-muted-foreground">
          Preview
        </div>
      )}
    </div>
    <div className="flex flex-col gap-1">
      <span className="text-sm font-medium group-hover:text-accent-foreground">
        {component.name}
      </span>
      <span className="text-xs text-muted-foreground">
        {COMPONENT_DESCRIPTIONS[String(component.name)] ??
          "A macOS-style control."}
      </span>
    </div>
  </Link>
);

export const ComponentsList = () => {
  const pathname = usePathname();
  const base = detectBase(pathname);

  if (!base) {
    return null;
  }

  const pages = getFolderPages(base === "gpui" ? "gpui" : "native-sdk");

  if (pages.length === 0) {
    return null;
  }

  return (
    <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
      {pages.map((component) => (
        <ComponentCard component={component} base={base} key={component.$id} />
      ))}
    </div>
  );
};
