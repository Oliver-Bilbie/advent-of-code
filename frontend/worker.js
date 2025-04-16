let wasmModule = null;

self.onmessage = async (e) => {
  const { event, year, day, part, input } = e.data;

  if (event === "init" && !wasmModule) {
    try {
      const module = await import("./pkg/wasm_run.js");
      await module.default(); // this is `init()`
      wasmModule = module;
    } catch (err) {
      self.postMessage({ event: "error", message: "WASM init failed: " + err });
    }
    return;
  }

  if (event === "run") {
    try {
      const result = wasmModule.run(year, day, part, input);
      self.postMessage({ event: "result", part, result });
    } catch (err) {
      self.postMessage({ event: "error", part, message: err.toString() });
    }
  }
};
