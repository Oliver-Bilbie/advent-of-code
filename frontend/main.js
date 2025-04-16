import init, { run } from "./pkg/wasm_run.js";

async function start() {
  await init();

  document.getElementById("run-btn").addEventListener("click", () => {
    const year = parseInt(document.getElementById("year").value, 10);
    const day = parseInt(document.getElementById("day").value, 10);
    const input = document.getElementById("input").value;

    document.getElementById("output-part-1").textContent = "Processing...";
    document.getElementById("output-part-2").textContent = "Queued";
    document.getElementById("output-container").style.visibility = "visible";

    setTimeout(() => {
      try {
        const result = run(year, day, 1, input);
        document.getElementById("output-part-1").textContent = result;
      } catch (e) {
        console.error(e);
        document.getElementById("output-part-1").textContent =
          `An error has occurred while running the solution; please check your input.\n${e}`;
      }

      document.getElementById("output-part-2").textContent = "Processing...";

      setTimeout(() => {
        try {
          const result = run(year, day, 2, input);
          document.getElementById("output-part-2").textContent = result;
        } catch (e) {
          console.error(e);
          document.getElementById("output-part-2").textContent =
            `An error has occurred while running the solution; please check your input.\n${e}`;
        }
      }, 0);
    }, 0);
  });
}

start();
