const WASM_MODULE_CACHE = new Map();

export async function loadSolution(year, day, part) {
  const dayStr = String(day).padStart(2, "0");
  const name = `solution_${year}_${dayStr}_${part}`;

  if (WASM_MODULE_CACHE.has(name)) {
    return true; // Already loaded and cached
  }

  const jsGluePath = `./wasm/${name}.js`;

  try {
    const module = await import(jsGluePath);

    if (typeof module.default === "function") {
      await module.default();
      WASM_MODULE_CACHE.set(name, module);
      return true;
    } else {
      return false;
    }
  } catch (error) {
    return false;
  }
}

export async function runSolution(year, day, part, input) {
  const dayStr = String(day).padStart(2, "0");
  const name = `solution_${year}_${dayStr}_${part}`;

  const isLoaded = await loadSolution(year, day, part);

  if (!isLoaded) {
    return "Unable to load solution";
  }

  const moduleExports = WASM_MODULE_CACHE.get(name);

  if (typeof moduleExports.solve === "function") {
    try {
      return moduleExports.solve(input);
    } catch (runtimeError) {
      throw new Error(
        `Execution failed: ${runtimeError.message || runtimeError}`,
      );
    }
  } else {
    throw new Error(`Solver function 'solve' not found in module ${name}.`);
  }
}
