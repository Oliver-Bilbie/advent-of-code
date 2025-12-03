import {
  getYear,
  getDay,
  getInput,
  setRunning,
  setStopped,
  setOutput,
  setDone,
  setReset,
} from "./form.js";

let worker = newWorker();

async function start() {
  const yearSelect = document.getElementById("year-select");
  const daySelect = document.getElementById("day-select");
  const runButton = document.getElementById("run-btn");
  const stopButton = document.getElementById("stop-btn");
  const resetButton = document.getElementById("reset-btn");

  runButton.addEventListener("click", () => {
    setRunning(1);
    worker.postMessage({
      event: "run",
      year: getYear(),
      day: getDay(),
      part: 1,
      input: getInput(),
    });
  });

  stopButton.onclick = () => {
    if (worker) {
      setStopped();
      worker.terminate();
    }
    worker = newWorker();
  };

  resetButton.onclick = setReset;

  // pre-load WASM when a year/day is selected
  yearSelect.addEventListener("change", triggerWasmLoad);
  daySelect.addEventListener("change", triggerWasmLoad);
  // pre-load WASM on page load for the default year/day
  document.addEventListener("DOMContentLoaded", () => {
    triggerWasmLoad();
  });
}

function triggerWasmLoad() {
  for (const part of [1, 2]) {
    worker.postMessage({
      event: "load",
      year: getYear(),
      day: getDay(),
      part: part,
    });
  }
}

function newWorker() {
  const w = new Worker("worker.js", { type: "module" });
  w.postMessage({ event: "init" });

  w.onmessage = (e) => {
    const { event, part, result, message } = e.data;

    if (event === "result") {
      if (part === 1) {
        setOutput(1, result, "#009900");
        setRunning(2);
        w.postMessage({
          event: "run",
          year: getYear(),
          day: getDay(),
          part: 2,
          input: getInput(),
        });
      } else if (part === 2) {
        setOutput(2, result, "#009900");
        setDone();
      } else {
        console.error(`Invalid part: ${part}`);
      }
    }

    if (event === "error") {
      const msg = `An error has occurred while running the program.\nPlease check that the provided input is correct.\n\n${message}`;
      if (part === 1) {
        setOutput(1, msg, "#ff0000");
        setOutput(2, "Cancelled", "#ffffff");
      } else if (part === 2) {
        setOutput(2, msg, "#ff0000");
      } else {
        console.error(`Invalid part: ${part}`);
      }

      setDone();
    }
  };

  w.onerror = (e) => {
    console.error("Worker error:", e.message);
    setOutput(1, "Worker error occurred", "#993399");
    setDone();
  };

  return w;
}

start();
