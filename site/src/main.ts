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

const setChangedResponse = (changed: boolean) => {
  if (!toggle) return;
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
    : "All sample assertions pass.";
};

toggle?.addEventListener("click", () => setChangedResponse(toggle.getAttribute("aria-pressed") !== "true"));

const params = new URLSearchParams(location.search);
const demoMode = params.get("demo") === "1" || location.pathname === "/demo/" || location.pathname === "/demo";
const demoBanner = document.querySelector<HTMLElement>(".demo-banner");
const demoReplay = document.querySelector<HTMLElement>("[data-demo-replay]");
if (demoMode) {
  if (demoBanner) demoBanner.hidden = false;
  if (demoReplay) demoReplay.hidden = false;
  document.documentElement.dataset.mode = "demo";
  const title = "Demo — Arazzo Proof Runner";
  const description = "Try the bundled Arazzo checkout workflow and inspect its redacted proof bundle.";
  document.title = title;
  document.querySelector<HTMLLinkElement>('link[rel="canonical"]')?.setAttribute("href", "https://arazzo-proof-runner.sociobot.in/?demo=1");
  document.querySelector<HTMLMetaElement>('meta[name="description"]')?.setAttribute("content", description);
  document.querySelector<HTMLMetaElement>('meta[property="og:title"]')?.setAttribute("content", title);
  document.querySelector<HTMLMetaElement>('meta[property="og:description"]')?.setAttribute("content", description);
  document.querySelector<HTMLMetaElement>('meta[name="twitter:title"]')?.setAttribute("content", title);
  document.querySelector<HTMLMetaElement>('meta[name="twitter:description"]')?.setAttribute("content", description);
  document.querySelector<HTMLAnchorElement>("[data-demo-link]")?.setAttribute("aria-current", "page");
}

document.querySelector<HTMLButtonElement>("[data-reset-demo]")?.addEventListener("click", () => {
  setChangedResponse(false);
  demoStatus!.textContent = "Demo reset. All sample assertions pass.";
  document.querySelector("#sample-report")?.scrollIntoView();
  toggle?.focus();
});

const offline = document.querySelector<HTMLElement>(".offline");
const updateConnection = () => { if (offline) offline.hidden = navigator.onLine; };
window.addEventListener("online", updateConnection);
window.addEventListener("offline", updateConnection);
updateConnection();

if ("serviceWorker" in navigator) window.addEventListener("load", () => navigator.serviceWorker.register("/sw.js"));

const announceAndFocus = () => {
  const target = location.hash ? document.querySelector<HTMLElement>(location.hash) : document.querySelector<HTMLElement>("h1");
  const heading = target?.matches("h1,h2") ? target : target?.querySelector<HTMLElement>("h1,h2");
  if (!heading) return;
  heading.focus({ preventScroll: true });
  const status = document.querySelector<HTMLElement>(".route-status");
  if (status) status.textContent = heading.textContent ?? "Page changed";
};

window.addEventListener("hashchange", announceAndFocus);
window.addEventListener("pageshow", event => {
  const navigation = performance.getEntriesByType("navigation")[0] as PerformanceNavigationTiming | undefined;
  if (event.persisted || navigation?.type === "back_forward" || (location.hash && document.referrer.startsWith(location.origin))) {
    announceAndFocus();
  }
});

if (demoMode || (location.pathname !== "/" && location.pathname !== "/index.html")) {
  requestAnimationFrame(announceAndFocus);
}
