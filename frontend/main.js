async function start() {
  const yearSelect = document.getElementById("year-select");
  const daySelect = document.getElementById("day-select");
  const inputField = document.getElementById("input");
  const runButton = document.getElementById("run-btn");
  const stopButton = document.getElementById("stop-btn");
  const resetButton = document.getElementById("reset-btn");
  const output1 = document.getElementById("output-part-1");
  const output2 = document.getElementById("output-part-2");
  const outputContainer = document.getElementById("output-container");

  let worker = newWorker();

  runButton.addEventListener("click", () => {
    const year = parseInt(yearSelect.value, 10);
    const day = parseInt(daySelect.value, 10);
    const input = inputField.value;

    output1.textContent = "Processing...";
    output2.textContent = "Queued";
    outputContainer.style.visibility = "visible";
    stopButton.disabled = false;
    resetButton.disabled = true;

    worker.postMessage({ event: "run", year, day, part: 1, input });
  });

  stopButton.onclick = () => {
    if (worker) {
      worker.terminate();
      if (output1.textContent === "Processing...") {
        output1.textContent = "Execution stopped.";
        output2.textContent = "Execution stopped.";
      } else if (output2.textContent === "Processing...") {
        output2.textContent = "Execution stopped.";
      }
      stopButton.disabled = true;
      resetButton.disabled = false;
      worker = newWorker();
    }
  };

  resetButton.onclick = () => {
    inputField.value = "";
    outputContainer.style.visibility = "hidden";
    output1.textContent = "";
    output2.textContent = "";
  };

  function newWorker() {
    const w = new Worker("worker.js", { type: "module" });
    w.postMessage({ event: "init" });

    w.onmessage = (e) => {
      const { event, part, result, message } = e.data;

      if (event === "result") {
        if (part === 1) {
          output1.textContent = result;
          output2.textContent = "Processing...";
          w.postMessage({
            event: "run",
            year: parseInt(yearSelect.value, 10),
            day: parseInt(daySelect.value, 10),
            part: 2,
            input: inputField.value,
          });
        } else if (part === 2) {
          output2.textContent = result;
          stopButton.disabled = true;
          resetButton.disabled = false;
        }
      }

      if (event === "error") {
        const msg = `An error has occurred while running the program.\nPlease check that the provided input is correct.\n${message}`;
        if (part === 1) {
          output1.textContent = msg;
          output2.textContent = "Cancelled";
        } else {
          output2.textContent = msg;
        }
        stopButton.disabled = true;
        resetButton.disabled = false;
      }
    };

    w.onerror = (e) => {
      console.error("Worker error:", e.message);
      output1.textContent = "Worker error occurred.";
      stopButton.disabled = true;
      resetButton.disabled = false;
    };

    return w;
  }
}

start();
