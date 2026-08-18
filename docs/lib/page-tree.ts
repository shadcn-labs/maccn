import type {
  Node as PageTreeNode,
  Root as PageTreeRoot,
} from "fumadocs-core/page-tree";

import { ROUTES } from "@/constants/routes";
import { EXCLUDED_SECTIONS, isComponentsFolder } from "@/lib/docs";

export type PageTreeFolder = Extract<PageTreeNode, { type: "folder" }>;
export type PageTreePage = Extract<PageTreeNode, { type: "page" }>;

export interface TreeGroup {
  label: string;
  pages: PageTreePage[];
}

export const getAllPagesFromFolder = (
  folder: PageTreeFolder
): PageTreePage[] => {
  const pages: PageTreePage[] = [];

  for (const child of folder.children) {
    if (child.type === "page") {
      pages.push(child);
    } else if (child.type === "folder") {
      pages.push(...getAllPagesFromFolder(child));
    }
  }

  return pages;
};

export const getPagesFromFolder = (folder: PageTreeFolder): PageTreePage[] =>
  folder.children.filter(
    (child): child is PageTreePage => child.type === "page"
  );

export const getPagesFromFolderWithoutIndex = (
  folder: PageTreeFolder
): PageTreePage[] =>
  folder.children.filter(
    (child): child is PageTreePage =>
      child.type === "page" && child.name !== "index"
  );

export const getFoldersFromFolder = (
  folder: PageTreeFolder
): PageTreeFolder[] =>
  folder.children.filter(
    (child): child is PageTreeFolder => child.type === "folder"
  );

export const getComponentNameFromUrl = (url: string): string => {
  const segments = url.split("/").filter(Boolean);
  return segments.at(-1) ?? "";
};

const matchesBase = (folder: PageTreeFolder, base: string): boolean =>
  folder.$id === base ||
  String(folder.$id ?? "").endsWith(`/${base}`) ||
  (typeof folder.name === "string" &&
    folder.name.toLowerCase() === base.toLowerCase());

export const findBaseFolder = (
  folder: PageTreeFolder,
  base: string
): PageTreeFolder | undefined => {
  for (const child of folder.children) {
    if (child.type !== "folder") {
      continue;
    }
    if (matchesBase(child, base)) {
      return child;
    }
  }
};

export const getCurrentBase = (pathname: string): string => {
  const baseScopedMatch = pathname.match(
    /\/docs\/(?:components|gpui)\/([^/]+)(?:\/|$)/
  );
  if (baseScopedMatch) {
    return baseScopedMatch[1];
  }

  return "gpui";
};

export const getFolderPages = (
  folder: PageTreeFolder,
  base?: string
): PageTreePage[] => {
  if (base) {
    const baseFolder = findBaseFolder(folder, base);
    if (!baseFolder) {
      return [];
    }

    return getAllPagesFromFolder(baseFolder);
  }

  return getAllPagesFromFolder(folder);
};

export const getTreeGroups = (
  tree: PageTreeRoot,
  currentBase: string
): TreeGroup[] => {
  const groups: TreeGroup[] = [];

  for (const item of tree.children) {
    if (item.type !== "folder") {
      continue;
    }
    if (EXCLUDED_SECTIONS.has(item.$id ?? "")) {
      continue;
    }

    if (isComponentsFolder(item)) {
      const baseFolder = findBaseFolder(item, currentBase);
      if (baseFolder) {
        const pages = getAllPagesFromFolder(baseFolder).filter(
          (page) =>
            page.url !== ROUTES.DOCS_COMPONENTS &&
            page.url !== `${ROUTES.DOCS_COMPONENTS}/${currentBase}`
        );
        if (pages.length > 0) {
          groups.push({
            label: "Components",
            pages,
          });
        }
      } else {
        const pages = getAllPagesFromFolder(item).filter(
          (page) => page.url !== ROUTES.DOCS_COMPONENTS
        );
        if (pages.length > 0) {
          groups.push({
            label: "Components",
            pages,
          });
        }
      }
    } else {
      const pages = getFolderPages(item);
      if (pages.length > 0) {
        groups.push({
          label: typeof item.name === "string" ? item.name : String(item.name),
          pages,
        });
      }
    }
  }

  return groups;
};
