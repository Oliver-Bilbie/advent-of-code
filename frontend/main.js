import init, { run } from "./pkg/wasm_run.js";

async function start() {
  await init();

  document.getElementById("run-btn").addEventListener("click", () => {
    const year = parseInt(document.getElementById("year").value, 10);
    const day = parseInt(document.getElementById("day").value, 10);
    const input = document.getElementById("input").value;

    try {
      const result = run(year, day, 1, input);
      document.getElementById("output").textContent = result;
    } catch (e) {
      console.error(e);
      document.getElementById("output").textContent =
        `An error has occurred while running the solution; please check your input.\n${e}`;
    }
  });
}

start();
