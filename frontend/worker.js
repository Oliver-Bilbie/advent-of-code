import { runSolution, loadSolution } from "./dispatcher.js";

self.onmessage = async (e) => {
  const { event, year, day, part, input } = e.data;

  if (event === "init") {
    return;
  }

  if (event === "load") {
    try {
      await loadSolution(year, day, part);
    } catch (error) {
      console.error(
        `Pre-load failed for Day ${day} Part ${part}: ${error.message}`,
      );
    }
    return;
  }

  if (event === "run") {
    try {
      const result = await runSolution(year, day, part, input);
      self.postMessage({ event: "result", part, result });
    } catch (error) {
      console.error("Worker Execution Error:", error);
      self.postMessage({
        event: "error",
        part,
        message:
          error.message || "Unknown error during dynamic WASM execution.",
      });
    }
  }
};
