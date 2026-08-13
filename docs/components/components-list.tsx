import Link from "next/link";

import { ROUTES } from "@/constants/routes";
import { isComponentsFolder } from "@/lib/docs";
import type { PageTreeFolder, PageTreePage } from "@/lib/page-tree";
import { getAllPagesFromFolder, getPagesFromFolder } from "@/lib/page-tree";
import { source } from "@/lib/source";

const COMPONENT_DESCRIPTIONS: Record<string, string> = {
  badge: "A small status indicator or label.",
  box: "A grouped container with rounded corners and padding.",
  button: "A bordered, prominent, or destructive push button.",
  checkbox: "A tri-state checkbox with check and indeterminate marks.",
  "glass-panel": "A translucent panel with blur and vibrancy effects.",
  "help-button": "A circular help button with a question mark glyph.",
  label: "A text label with macOS typography styles.",
  "pop-up-button": "A dropdown button that opens a popover menu.",
  progress: "A horizontal progress bar with determinate and indeterminate modes.",
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

const getFolder = (name: string): PageTreeFolder | undefined => {
  for (const node of source.pageTree.children) {
    if (node.type === "folder" && node.name === name) {
      return node;
    }
  }
};

const ComponentGrid = ({ pages }: { pages: PageTreePage[] }) => (
  <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
    {pages.map((component) => (
      <Link
        className="group flex flex-col gap-1 rounded-xl border bg-card p-4 transition-colors hover:bg-accent/50"
        href={component.url}
        key={component.$id}
        transitionTypes={["nav-forward"]}
      >
        <span className="text-sm font-medium group-hover:text-accent-foreground">
          {component.name}
        </span>
        <span className="text-xs text-muted-foreground">
          {COMPONENT_DESCRIPTIONS[String(component.name)] ?? "A macOS-style control."}
        </span>
      </Link>
    ))}
  </div>
);

export const ComponentsList = ({
  folderName = "Components",
}: {
  folderName?: string;
}) => {
  const folder = getFolder(folderName);
  if (!folder) {
    return null;
  }

  if (!isComponentsFolder(folder)) {
    const pages = getPagesFromFolder(folder);
    if (pages.length === 0) {
      return null;
    }
    return <ComponentGrid pages={pages} />;
  }

  const pages = getAllPagesFromFolder(folder).filter(
    (page) => page.url !== ROUTES.DOCS_COMPONENTS
  );
  if (pages.length === 0) {
    return null;
  }

  return <ComponentGrid pages={pages} />;
};
