import "./wasm_exec.js";

let go = new Go();
let instance;

export default async function init() {
  if (instance) return;

  const response = await fetch("__WASM_PATH__");
  const bytes = await response.arrayBuffer();
  const result = await WebAssembly.instantiate(bytes, go.importObject);

  instance = result.instance;
  go.run(instance);
}

export function solve(input) {
  return globalThis.solve(input);
}
