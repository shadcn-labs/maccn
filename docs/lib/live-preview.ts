/**
 * Typed WASM wrapper for the maccn native-sdk preview engine.
 *
 * Architecture: a single shared WASM instance with multiple LivePreview
 * handles (one per preview tile). The page owns the clock and pixel buffer.
 */

const WASM_PATH = "/native-sdk/preview.wasm";

let engineReady: Promise<PreviewEngine> | undefined;

interface WasmExports {
  memory: WebAssembly.Memory;
  preview_create(name_ptr: number, name_len: number, dark: number): number;
  preview_render(preview_id: number, scale: number): number;
  preview_pointer(preview_id: number, kind: number, x: number, y: number): void;
  preview_scroll(preview_id: number, dx: number, dy: number): void;
  preview_key(
    preview_id: number,
    key_ptr: number,
    key_len: number,
    modifiers: number
  ): void;
  preview_text(preview_id: number, text_ptr: number, text_len: number): void;
  preview_destroy(preview_id: number): void;
  preview_alloc(len: number): number;
  preview_free(ptr: number): void;
  preview_set_now_ms(preview_id: number, ms: number): void;
  preview_set_theme(preview_id: number, dark: number): void;
  preview_logical_width(preview_id: number): number;
  preview_logical_height(preview_id: number): number;
  preview_cursor(preview_id: number): number;
  preview_pixel_byte_len(preview_id: number): number;
  preview_status(preview_id: number): number;
}

const cursorMap = ["default", "pointer", "text", "col-resize"] as const;

export class LivePreview {
  constructor(
    private engine: PreviewEngine,
    public readonly id: number,
    public readonly logicalWidth: number,
    public readonly logicalHeight: number,
    private pixelByteLen: number
  ) {}

  render(scale: number): ImageData | null {
    const status = this.engine.exports.preview_status(this.id);
    if (status === 0) {
      return null;
    }

    const w = Math.ceil(this.logicalWidth * scale);
    const h = Math.ceil(this.logicalHeight * scale);
    const byteLen = w * h * 4;

    const ptr = this.engine.exports.preview_alloc(byteLen);
    try {
      this.engine.exports.preview_render(this.id, scale);
      const heap = new Uint8Array(this.engine.exports.memory.buffer);
      const data = new Uint8Array(byteLen);
      data.set(heap.subarray(ptr, ptr + byteLen));
      return new ImageData(new Uint8ClampedArray(data.buffer), w, h);
    } finally {
      this.engine.exports.preview_free(ptr);
    }
  }

  pointer(kind: number, x: number, y: number): void {
    this.engine.exports.preview_pointer(this.id, kind, x, y);
  }

  scroll(dx: number, dy: number): void {
    this.engine.exports.preview_scroll(this.id, dx, dy);
  }

  key(key: string, modifiers: number): void {
    const encoder = new TextEncoder();
    const bytes = encoder.encode(key);
    const ptr = this.engine.exports.preview_alloc(bytes.length);
    new Uint8Array(this.engine.exports.memory.buffer, ptr, bytes.length).set(
      bytes
    );
    try {
      this.engine.exports.preview_key(this.id, ptr, bytes.length, modifiers);
    } finally {
      this.engine.exports.preview_free(ptr);
    }
  }

  text(content: string): void {
    const encoder = new TextEncoder();
    const bytes = encoder.encode(content);
    const ptr = this.engine.exports.preview_alloc(bytes.length);
    new Uint8Array(this.engine.exports.memory.buffer, ptr, bytes.length).set(
      bytes
    );
    try {
      this.engine.exports.preview_text(this.id, ptr, bytes.length);
    } finally {
      this.engine.exports.preview_free(ptr);
    }
  }

  setNow(ms: number): void {
    this.engine.exports.preview_set_now_ms(this.id, ms);
  }

  setTheme(dark: boolean): void {
    this.engine.exports.preview_set_theme(this.id, dark ? 1 : 0);
  }

  cursor(): string {
    const idx = this.engine.exports.preview_cursor(this.id);
    return cursorMap[idx] ?? "default";
  }

  destroy(): void {
    this.engine.exports.preview_destroy(this.id);
  }
}

export class PreviewEngine {
  constructor(public exports: WasmExports) {}

  static async load(): Promise<PreviewEngine> {
    const { instance } = await WebAssembly.instantiateStreaming(
      fetch(WASM_PATH)
    );

    const exports = instance.exports as unknown as WasmExports;
    return new PreviewEngine(exports);
  }

  create(name: string, dark: boolean): LivePreview | null {
    const encoder = new TextEncoder();
    const nameBytes = encoder.encode(name);
    const namePtr = this.exports.preview_alloc(nameBytes.length);
    const memLen = this.exports.memory.buffer.byteLength;

    if (namePtr + nameBytes.length > memLen) {
      console.error(
        `preview_alloc(${nameBytes.length}) returned ${namePtr} out of bounds (mem ${memLen})`
      );
      return null;
    }

    new Uint8Array(this.exports.memory.buffer, namePtr, nameBytes.length).set(
      nameBytes
    );

    try {
      const id = this.exports.preview_create(
        namePtr,
        nameBytes.length,
        dark ? 1 : 0
      );
      if (id === 0) {
        return null;
      }

      const w = this.exports.preview_logical_width(id);
      const h = this.exports.preview_logical_height(id);
      const pixelByteLen = this.exports.preview_pixel_byte_len(id);
      return new LivePreview(this, id, w, h, pixelByteLen);
    } finally {
      this.exports.preview_free(namePtr);
    }
  }
}

export async function loadPreviewEngine(): Promise<PreviewEngine | null> {
  try {
    engineReady ??= PreviewEngine.load();
    return await engineReady;
  } catch {
    engineReady = undefined;
    return null;
  }
}
