import "./wasm_exec.js";

let go = new Go();
let instance;
let ready;

export default async function init() {
  if (!ready) {
    ready = (async () => {
      const response = await fetch("__WASM_PATH__");
      const bytes = await response.arrayBuffer();
      const result = await WebAssembly.instantiate(bytes, go.importObject);

      instance = result.instance;
      go.run(instance);

      return true;
    })();
  }

  return ready;
}

export function solve(input) {
  return globalThis["__SOLVER_NAME__"](input);
}

export function language() {
  return "Go 🐹";
}
