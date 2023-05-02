const { invoke } = window.__TAURI__.tauri;

let literaturesInput;
let literaturesOutput;
let sendBotton;

async function verify() {
  sendBotton.disabled = true;
  literaturesOutput.textContent = "検証中...";
  literaturesOutput.textContent = await invoke("verify", { lines: literaturesInput.value });
  sendBotton.disabled = false;
}

window.addEventListener("DOMContentLoaded", () => {
  literaturesInput = document.querySelector("#literatures-input");
  literaturesOutput = document.querySelector("#literatures-output");
  sendBotton = document.querySelector("#send-button");
  sendBotton.addEventListener("click", () => verify());
});
