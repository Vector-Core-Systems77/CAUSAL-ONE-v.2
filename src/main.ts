import { invoke } from "@tauri-apps/api/core";

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div style="padding: 20px; font-family: sans-serif;">
    <h1>Causal One</h1>
    <p>التطبيق شغال ✅</p>
    <button id="generateBtn" style="padding: 12px 30px; font-size: 18px; margin-top: 20px;">
      ولد الصفر التالي
    </button>
    <p id="result" style="margin-top: 20px; font-size: 20px; color: #00ff41;"></p>
  </div>
`;

document.getElementById("generateBtn")!.addEventListener("click", async () => {
  try {
    const zero = await invoke<number>("generate_next_zero");
    document.getElementById("result")!.innerText = `Zero: ${zero.toFixed(8)}`;
  } catch (e) {
    document.getElementById("result")!.innerText = `خطأ: ${e}`;
  }
});
