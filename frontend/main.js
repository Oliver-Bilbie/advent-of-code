async function start() {
  const yearSelect = document.getElementById("year-select");
  const daySelect = document.getElementById("day-select");
  const inputField = document.getElementById("input");
  const runButton = document.getElementById("run-btn");
  const stopButton = document.getElementById("stop-btn");
  const output1 = document.getElementById("output-part-1");
  const output2 = document.getElementById("output-part-2");
  const outputContainer = document.getElementById("output-container");

  let worker = newWorker();

  runButton.addEventListener("click", () => {
    const year = parseInt(yearSelect.textContent, 10);
    const day = parseInt(daySelect.textContent, 10);
    const input = inputField.value;

    output1.textContent = "Processing...";
    output2.textContent = "Queued";
    outputContainer.style.visibility = "visible";
    stopButton.style.visibility = "visible";

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
      stopButton.style.visibility = "hidden";
      worker = newWorker();
    }
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
            year: parseInt(yearSelect.textContent, 10),
            day: parseInt(daySelect.textContent, 10),
            part: 2,
            input: inputField.value,
          });
        } else if (part === 2) {
          output2.textContent = result;
          stopButton.style.visibility = "hidden";
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
        stopButton.style.visibility = "hidden";
      }
    };

    w.onerror = (e) => {
      console.error("Worker error:", e.message);
      output1.textContent = "Worker error occurred.";
      stopButton.style.visibility = "hidden";
    };

    return w;
  }
}

function setupDropdowns() {
  document.querySelectorAll(".dropdown").forEach((dropdown) => {
    const toggle = dropdown.querySelector(".dropdown-toggle");
    const items = dropdown.querySelectorAll(".dropdown-item");

    toggle.addEventListener("click", (e) => {
      e.stopPropagation();
      document.querySelectorAll(".dropdown").forEach((d) => {
        if (d !== dropdown) d.classList.remove("open");
      });
      dropdown.classList.toggle("open");
    });

    items.forEach((item) => {
      item.addEventListener("click", () => {
        toggle.textContent = item.textContent;
        dropdown.classList.remove("open");
      });
    });
  });

  // Close dropdowns if clicking outside
  document.addEventListener("click", () => {
    document.querySelectorAll(".dropdown").forEach((dropdown) => {
      dropdown.classList.remove("open");
    });
  });
}

setupDropdowns();
start();
