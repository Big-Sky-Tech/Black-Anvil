import { invoke } from "@tauri-apps/api/core";

const runBtn = document.getElementById("run");
const output = document.getElementById("output");
const textarea = document.getElementById("script");

runBtn.addEventListener("click", async () => {
  output.textContent = "Running...";
  try {
    const res = await invoke("run_starlark", { script: textarea.value });
    output.textContent = String(res);
  } catch (e) {
    output.textContent = `Error: ${e}`;
  }
});
