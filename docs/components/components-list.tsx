"use client";

import dynamic from "next/dynamic";
import Link from "next/link";

import { isComponentsFolder } from "@/lib/docs";
import type { PageTreeFolder, PageTreePage } from "@/lib/page-tree";
import {
  getComponentNameFromUrl,
  getFoldersFromFolder,
  getPagesFromFolderWithoutIndex,
} from "@/lib/page-tree";
import { source } from "@/lib/source";
import { cn } from "@/lib/utils";

const ComponentPreviewLive = dynamic(
  async () => {
    const mod = await import("@/components/component-preview-live");
    return mod.ComponentPreviewLive;
  },
  { ssr: false }
);

const componentsFolder = source.pageTree.children.find(
  (node): node is PageTreeFolder =>
    node.type === "folder" && isComponentsFolder(node)
);

const ComponentCard = ({
  component,
  base,
}: {
  component: PageTreePage;
  base: string;
}) => {
  const name = getComponentNameFromUrl(component.url);
  const title = String(component.name);

  return (
    <Link
      className="group rounded-lg bg-code p-1 transition-colors hover:bg-muted/80"
      href={component.url}
      key={component.$id ?? component.url}
    >
      <div className="flex h-[140px] items-center justify-center overflow-hidden rounded-md border bg-muted/50">
        {base === "gpui" ? (
          <iframe
            src={`/examples?component=${encodeURIComponent(name)}`}
            title={`${name} preview`}
            className="h-full w-full border-none"
            allow="cross-origin-isolated"
          />
        ) : (
          <ComponentPreviewLive name={name} height={120} />
        )}
      </div>
      <div className="p-2 pb-1 text-base font-medium underline-offset-4 group-hover:underline">
        {title}
      </div>
    </Link>
  );
};

const ComponentCardGrid = ({
  components,
  base,
  className,
}: {
  components: PageTreePage[];
  base: string;
  className?: string;
}) => (
  <div className={cn("grid gap-4 sm:grid-cols-2", className)}>
    {components.map((component) => (
      <ComponentCard component={component} base={base} key={component.$id} />
    ))}
  </div>
);

export const ComponentsList = ({
  category,
  className,
}: {
  category?: string;
  className?: string;
}) => {
  if (!componentsFolder) {
    return null;
  }

  const baseFolders = getFoldersFromFolder(componentsFolder);

  if (category) {
    const categoryFolder = baseFolders.find(
      (folder) => folder.$id?.split("/").at(-1) === category
    );

    if (!categoryFolder) {
      return null;
    }

    const components = getPagesFromFolderWithoutIndex(categoryFolder);
    return (
      <ComponentCardGrid
        components={components}
        base={category}
        className={className}
      />
    );
  }

  // Show GPUI components by default
  const gpuiFolder = baseFolders.find(
    (folder) => folder.$id?.split("/").at(-1) === "gpui"
  );

  if (gpuiFolder) {
    const components = getPagesFromFolderWithoutIndex(gpuiFolder);
    return (
      <ComponentCardGrid
        components={components}
        base="gpui"
        className={className}
      />
    );
  }

  return null;
};
