const yearSelect = document.getElementById("year-select");
const daySelect = document.getElementById("day-select");
const inputField = document.getElementById("input");
const runButton = document.getElementById("run-btn");
const stopButton = document.getElementById("stop-btn");
const resetButton = document.getElementById("reset-btn");
const output1 = document.getElementById("output-part-1");
const output2 = document.getElementById("output-part-2");
const outputContainer = document.getElementById("output-container");

export function getYear() {
  return parseInt(yearSelect.value, 10);
}

export function getDay() {
  return parseInt(daySelect.value, 10);
}

export function getInput() {
  return inputField.value;
}

export function setRunning(part) {
  if (part === 1) {
    setOutput(1, "Processing...", "#ffffff");
    setOutput(2, "Queued", "#ffffff");
  } else if (part === 2) {
    setOutput(2, "Processing...", "#ffffff");
  } else {
    console.error(`Invalid part: ${part}`);
  }

  outputContainer.style.visibility = "visible";
  runButton.disabled = true;
  stopButton.disabled = false;
  resetButton.disabled = true;
}

export function setOutput(part, message, color) {
  if (part === 1) {
    output1.textContent = message;
    output1.style.borderLeftColor = color;
  } else if (part === 2) {
    output2.textContent = message;
    output2.style.borderLeftColor = color;
  } else {
    console.error(`Invalid part: ${part}`);
  }
}

export function setDone() {
  runButton.disabled = false;
  stopButton.disabled = true;
  resetButton.disabled = false;
}

export function setStopped() {
  if (
    output1.textContent !== "Processing..." &&
    output2.textContent !== "Processing..."
  ) {
    return; // Nothing is running
  }

  if (output1.textContent === "Processing...") {
    setOutput(1, "Execution stopped", "#cc9900");
  }
  setOutput(2, "Execution stopped", "#cc9900");
  setDone();
}

export function setReset() {
  inputField.value = "";
  outputContainer.style.visibility = "hidden";
  setOutput(1, "", "#ffffff");
  setOutput(2, "", "#ffffff");
}
