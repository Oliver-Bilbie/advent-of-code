const WASM_MODULE_CACHE = new Map();

export async function loadSolution(year, day, part) {
  const dayStr = String(day).padStart(2, "0");
  const crateName = `solution_${year}_${dayStr}_${part}`;

  if (WASM_MODULE_CACHE.has(crateName)) {
    return; // Already loaded and cached
  }

  const jsGluePath = `./wasm/${crateName}.js`;

  try {
    const module = await import(jsGluePath);

    if (typeof module.default === "function") {
      await module.default();
    } else {
      console.warn(
        `WASM module ${crateName} is missing the default initialization function.`,
      );
    }

    WASM_MODULE_CACHE.set(crateName, module);
  } catch (error) {
    throw new Error(`Loading failed: ${error.message}`);
  }
}

export async function runSolution(year, day, part, input) {
  const dayStr = String(day).padStart(2, "0");
  const crateName = `solution_${year}_${dayStr}_${part}`;

  await loadSolution(year, day, part);

  const moduleExports = WASM_MODULE_CACHE.get(crateName);

  if (typeof moduleExports.solve === "function") {
    try {
      return moduleExports.solve(input);
    } catch (runtimeError) {
      throw new Error(
        `Execution aborted by Rust panic: ${runtimeError.message || runtimeError}`,
      );
    }
  } else {
    throw new Error(
      `Solver function 'solve' not found in module ${crateName}.`,
    );
  }
}
