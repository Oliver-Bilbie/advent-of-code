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
      console.error(`Failed to load Day ${day} Part ${part}: ${error.message}`);
    }
    return;
  }

  if (event === "run") {
    try {
      const response = await runSolution(year, day, part, input);
      self.postMessage({
        event: "result",
        part,
        result: response.result,
        time: response.time,
        language: response.language,
      });
    } catch (error) {
      self.postMessage({
        event: "error",
        part,
        message: error.message,
      });
    }
    return;
  }
};
