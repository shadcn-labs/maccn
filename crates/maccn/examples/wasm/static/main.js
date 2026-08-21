const embedded = window.parent !== window;

function hostPrefersDark() {
  if (!embedded) return undefined;
  try {
    return window.parent.document.documentElement.classList.contains("dark");
  } catch {
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
  }
}

function watchHostTheme(wasm) {
  if (!embedded) return;
  let root;
  try {
    root = window.parent.document.documentElement;
  } catch {
    return;
  }

  let current = root.classList.contains("dark");
  new MutationObserver(() => {
    const next = root.classList.contains("dark");
    if (next !== current) {
      current = next;
      document.documentElement.classList.toggle("dark", next);
      wasm.set_theme(next);
    }
  }).observe(root, { attributes: true, attributeFilter: ["class"] });

  window.addEventListener("message", (event) => {
    if (
      event.data?.type === "theme-change" &&
      (event.data.theme === "light" || event.data.theme === "dark")
    ) {
      const next = event.data.theme === "dark";
      if (next !== current) {
        current = next;
        document.documentElement.classList.toggle("dark", next);
        wasm.set_theme(next);
        window.parent.document.documentElement.classList.toggle("dark", next);
      }
    }

    if (event.data?.type === "accent-change" && typeof event.data.hex === "number") {
      console.log("[maccn iframe] received hex:", event.data.hex, "=0x" + event.data.hex.toString(16).padStart(8, "0"));
      try {
        wasm.set_accent(event.data.hex);
      } catch (e) {
        console.error("[maccn iframe] set_accent FAILED:", e);
      }
    }
  });
}

async function init() {
  const loading = document.getElementById("loading");
  try {
    const wasm = await import("/examples/wasm/maccn_wasm.js");
    await wasm.default();
    const params = new URLSearchParams(window.location.search);
    const component = params.get("component");
    const cardMode = params.get("mode") === "card";
    await wasm.run(component || undefined, cardMode, hostPrefersDark());
    watchHostTheme(wasm);
    loading?.remove();
  } catch (error) {
    console.error("Failed to initialize maccn example:", error);
    if (loading) loading.textContent = `Failed to load example: ${error?.message || error}`;
  }
}
init();
