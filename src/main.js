const { invoke } = window.__TAURI__.tauri;

let literaturesInput;
let literaturesOutput;

async function verify() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  literaturesOutput.textContent = await invoke("verify", { lines: literaturesInput.value });
}

window.addEventListener("DOMContentLoaded", () => {
  literaturesInput = document.querySelector("#literatures-input");
  literaturesOutput = document.querySelector("#literatures-output");
  document
    .querySelector("#send-button")
    .addEventListener("click", () => verify());
});
