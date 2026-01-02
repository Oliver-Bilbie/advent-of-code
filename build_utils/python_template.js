import "https://cdn.jsdelivr.net/pyodide/v0.29.0/full/pyodide.js";

let ready;
let pyodide;
let script;

export default async function init() {
  if (!ready) {
    ready = (async () => {
      pyodide = await loadPyodide();
      const response = await fetch("__SCRIPT_PATH__");
      script = await response.text();
      return true;
    })();
  }

  return ready;
}

export function solve(input) {
  const python = `${script}\nsolve(${JSON.stringify(input)})`;
  return pyodide.runPython(python);
}

export function language() {
  return "Python 🐍";
}
