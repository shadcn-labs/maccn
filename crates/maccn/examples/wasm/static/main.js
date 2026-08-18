async function init() {
  const loading = document.getElementById('loading');
  try {
    const wasm = await import('/examples/wasm/maccn_wasm.js');
    await wasm.default();
    const params = new URLSearchParams(window.location.search);
    const component = params.get('component');
    const cardMode = params.get('mode') === 'card';
    await wasm.run(component || undefined, cardMode);
    loading?.remove();
  } catch (error) {
    console.error('Failed to initialize maccn example:', error);
    if (loading) loading.textContent = `Failed to load example: ${error?.message || error}`;
  }
}
init();
