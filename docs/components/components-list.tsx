"use client";

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

const componentsFolder = source.pageTree.children.find(
  (node): node is PageTreeFolder =>
    node.type === "folder" && isComponentsFolder(node)
);

const ComponentCard = ({ component }: { component: PageTreePage }) => {
  const name = getComponentNameFromUrl(component.url);
  const title = String(component.name);

  return (
    <div className="rounded-lg bg-code p-1 transition-colors hover:bg-muted/80">
      <div className="flex h-35 items-center justify-center overflow-hidden rounded-md border bg-muted/50">
        <iframe
          src={`/examples?component=${encodeURIComponent(name)}&mode=card`}
          title={`${name} preview`}
          className="h-full w-full border-none"
          allow="cross-origin-isolated"
          tabIndex={-1}
        />
      </div>
      <Link
        href={component.url}
        className="p-2 pb-1 text-base font-medium underline-offset-4 block hover:underline"
      >
        {title}
      </Link>
    </div>
  );
};

const ComponentCardGrid = ({
  components,
  className,
}: {
  components: PageTreePage[];
  className?: string;
}) => (
  <div className={cn("grid gap-4 sm:grid-cols-2", className)}>
    {components.map((component) => (
      <ComponentCard component={component} key={component.$id} />
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
    return <ComponentCardGrid components={components} className={className} />;
  }

  // Show GPUI components by default
  const gpuiFolder = baseFolders.find(
    (folder) => folder.$id?.split("/").at(-1) === "gpui"
  );

  if (gpuiFolder) {
    const components = getPagesFromFolderWithoutIndex(gpuiFolder);
    return <ComponentCardGrid components={components} className={className} />;
  }

  return null;
};
