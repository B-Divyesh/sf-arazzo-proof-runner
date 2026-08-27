const copyButton = document.querySelector<HTMLButtonElement>("[data-copy]");
const copyStatus = document.querySelector<HTMLElement>(".copy-status");

copyButton?.addEventListener("click", async () => {
  const target = document.querySelector<HTMLElement>(copyButton.dataset.copy ?? "");
  if (!target || !copyStatus) return;
  try {
    await navigator.clipboard.writeText(target.innerText);
    copyStatus.textContent = "Command copied.";
  } catch {
    copyStatus.textContent = "Copy was blocked. Select the command manually.";
  }
});

const toggle = document.querySelector<HTMLButtonElement>("[data-demo-toggle]");
const changedStep = document.querySelector<HTMLElement>(".changed-step");
const stamp = document.querySelector<HTMLElement>(".demo-stamp");
const result = document.querySelector<HTMLElement>(".demo-result dd");
const diff = document.querySelector<HTMLElement>(".diff-note");
const demoStatus = document.querySelector<HTMLElement>(".demo-status");

toggle?.addEventListener("click", () => {
  const changed = toggle.getAttribute("aria-pressed") !== "true";
  toggle.setAttribute("aria-pressed", String(changed));
  toggle.textContent = changed ? "Restore baseline response" : "Inject changed response";
  changedStep?.classList.toggle("is-changed", changed);
  if (stamp) {
    stamp.className = `stamp demo-stamp ${changed ? "fail" : "pass"}`;
    stamp.textContent = changed ? "× Fail" : "✓ Pass";
  }
  if (result) result.textContent = changed ? '"EUR"' : '"USD"';
  if (diff) diff.hidden = !changed;
  if (demoStatus) demoStatus.textContent = changed
    ? "The quoteCart assertion now fails: expected USD, received EUR."
    : "All specimen assertions pass.";
});

const offline = document.querySelector<HTMLElement>(".offline");
const updateConnection = () => { if (offline) offline.hidden = navigator.onLine; };
window.addEventListener("online", updateConnection);
window.addEventListener("offline", updateConnection);
updateConnection();

if ("serviceWorker" in navigator) window.addEventListener("load", () => navigator.serviceWorker.register("/sw.js"));
