export const BASES = [
  {
    description:
      "GPUI components compiled to WebAssembly, rendered live in an iframe.",
    name: "wasm",
    title: "WASM",
  },
  {
    description: "macOS native components rendered via the Zig Native SDK.",
    name: "native-sdk",
    title: "Native SDK",
  },
] as const;

export type Base = (typeof BASES)[number];
export type BaseName = Base["name"];

export const DEFAULT_BASE = BASES[0].name;

export const BASE_NAMES = BASES.map((b) => b.name) as [BaseName, ...BaseName[]];

export const getBase = (name: BaseName) => BASES.find((b) => b.name === name);
