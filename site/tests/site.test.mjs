import { after, before, test } from "node:test";
import assert from "node:assert/strict";
import { createServer } from "node:http";
import { readFile, readdir, stat } from "node:fs/promises";
import { extname, join, normalize } from "node:path";
import { chromium } from "playwright";
import AxeBuilder from "@axe-core/playwright";

const root = join(process.cwd(), "dist", "site");
let server;
let browser;
let origin;

before(async () => {
  server = createServer(async (request, response) => {
    const pathname = new URL(request.url, "http://local").pathname;
    if (pathname === "/demo" || pathname === "/demo/") {
      response.writeHead(302, { location: "/?demo=1" });
      response.end();
      return;
    }
    const safe = normalize(pathname).replace(/^(\.\.[/\\])+/, "");
    let path = join(root, safe);
    try {
      if ((await stat(path)).isDirectory()) path = join(path, "index.html");
      const body = await readFile(path);
      const type = { ".html": "text/html", ".css": "text/css", ".js": "text/javascript", ".svg": "image/svg+xml", ".webp": "image/webp", ".jpg": "image/jpeg", ".png": "image/png", ".xml": "application/xml", ".json": "application/json" }[extname(path)] ?? "application/octet-stream";
      response.writeHead(200, { "content-type": type });
      response.end(body);
    } catch {
      response.writeHead(404, { "content-type": "text/html" });
      response.end(await readFile(join(root, "404.html")));
    }
  });
  await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
  origin = `http://127.0.0.1:${server.address().port}`;
  browser = await chromium.launch({ headless: true });
});

after(async () => {
  await browser?.close();
  await new Promise(resolve => server?.close(resolve));
});

test("all routes pass desktop and mobile Axe with no console errors", async () => {
  for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
    for (const path of ["/", "/?demo=1", "/privacy/", "/terms/", "/missing-route"]) {
      for (const colorScheme of ["light", "dark"]) {
      const context = await browser.newContext({ viewport, colorScheme });
      const page = await context.newPage();
      const errors = [];
      page.on("console", message => { if (message.type() === "error") errors.push(message.text()); });
      page.on("pageerror", error => errors.push(error.message));
      await page.goto(origin + path, { waitUntil: "networkidle" });
      assert.equal(await page.locator("h1").count(), 1, path);
      assert.equal(await page.locator("main").count(), 1, path);
      const results = await new AxeBuilder({ page }).analyze();
      assert.deepEqual(results.violations, [], `${path} at ${viewport.width}px: ${results.violations.map(v => v.id).join(", ")}`);
      if (path !== "/missing-route") assert.deepEqual(errors, [], path);
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth), false, path);
      await context.close();
      }
    }
  }
});

test("reduced motion removes transitions and every mobile control has a 44px target", async () => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 }, reducedMotion: "reduce" });
  const page = await context.newPage();
  await page.goto(`${origin}/?demo=1`, { waitUntil: "networkidle" });
  const reducedDuration = await page.locator(".button.primary").first().evaluate(element => parseFloat(getComputedStyle(element).transitionDuration));
  assert.ok(reducedDuration <= 0.001, `transition duration is ${reducedDuration}s`);
  const tooSmall = await page.locator("a, button, [tabindex='0']").evaluateAll(elements => elements.filter(element => {
    const box = element.getBoundingClientRect();
    return box.width > 0 && box.height > 0 && (box.width < 44 || box.height < 44);
  }).map(element => ({ text: element.textContent?.trim(), box: element.getBoundingClientRect().toJSON() })));
  assert.deepEqual(tooSmall, []);
  await page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth), false);
  await context.close();
});

test("@claim:web-demo-isolation one click opens resettable sample without persistent data", async () => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  const requests = [];
  page.on("request", request => requests.push(new URL(request.url()).origin));
  await page.goto(origin);
  await page.evaluate(async () => {
    localStorage.setItem("real:project", "keep-local");
    sessionStorage.setItem("real:session", "keep-session");
    await new Promise((resolve, reject) => {
      const open = indexedDB.open("real-project", 1);
      open.onupgradeneeded = () => open.result.createObjectStore("records");
      open.onerror = () => reject(open.error);
      open.onsuccess = () => {
        const transaction = open.result.transaction("records", "readwrite");
        transaction.objectStore("records").put("keep-indexeddb", "sample");
        transaction.oncomplete = () => { open.result.close(); resolve(); };
        transaction.onerror = () => reject(transaction.error);
      };
    });
  });
  await page.getByRole("link", { name: "Try it with sample data" }).first().click();
  await page.waitForLoadState("networkidle");
  assert.equal(new URL(page.url()).search, "?demo=1");
  assert.equal(await page.title(), "Demo — Arazzo Proof Runner");
  await assert.doesNotReject(() => page.getByText("Demo — sample data, nothing is saved").waitFor());
  await assert.doesNotReject(() => page.getByText("checkout: 3 steps, 3 assertions passed").waitFor());
  await page.getByRole("button", { name: "Inject changed response" }).click();
  assert.match(await page.locator(".demo-status").textContent(), /expected USD, received EUR/);
  await page.getByRole("button", { name: "Reset demo" }).click();
  assert.equal(await page.getByRole("button", { name: "Inject changed response" }).getAttribute("aria-pressed"), "false");
  const clientState = await page.evaluate(async () => {
    const indexedValue = await new Promise((resolve, reject) => {
      const open = indexedDB.open("real-project", 1);
      open.onerror = () => reject(open.error);
      open.onsuccess = () => {
        const transaction = open.result.transaction("records", "readonly");
        const get = transaction.objectStore("records").get("sample");
        get.onsuccess = () => { open.result.close(); resolve(get.result); };
        get.onerror = () => reject(get.error);
      };
    });
    return {
      local: Object.fromEntries(Object.entries(localStorage)),
      session: Object.fromEntries(Object.entries(sessionStorage)),
      databases: indexedDB.databases ? (await indexedDB.databases()).map(database => database.name) : ["real-project"],
      indexedValue,
      cookies: document.cookie
    };
  });
  assert.deepEqual(clientState, {
    local: { "real:project": "keep-local" },
    session: { "real:session": "keep-session" },
    databases: ["real-project"],
    indexedValue: "keep-indexeddb",
    cookies: ""
  });
  assert.ok(requests.every(requestOrigin => requestOrigin === origin));
  await context.close();
});

test("@claim:offline-site cached public routes reload offline", async () => {
  const context = await browser.newContext({ serviceWorkers: "allow" });
  const page = await context.newPage();
  await page.goto(`${origin}/?demo=1`, { waitUntil: "networkidle" });
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload({ waitUntil: "networkidle" });
  await context.setOffline(true);
  await page.reload({ waitUntil: "domcontentloaded" });
  assert.equal(await page.title(), "Demo — Arazzo Proof Runner");
  assert.equal(await page.locator("h1").textContent(), "Run Arazzo workflows and save a proof bundle");
  await page.goto(`${origin}/privacy/`, { waitUntil: "domcontentloaded" });
  assert.equal(await page.locator("h1").textContent(), "Privacy");
  await context.close();
});

test("@claim:site-privacy every page uses only same-origin assets and no tracking state", async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const requests = [];
  page.on("request", request => requests.push(request.url()));
  for (const path of ["/", "/?demo=1", "/privacy/", "/terms/"]) await page.goto(origin + path, { waitUntil: "networkidle" });
  const deployedFiles = new Set();
  const collectFiles = async (directory, prefix = "") => {
    for (const entry of await readdir(directory, { withFileTypes: true })) {
      const relative = `${prefix}/${entry.name}`;
      if (entry.isDirectory()) await collectFiles(join(directory, entry.name), relative);
      else deployedFiles.add(relative);
    }
  };
  await collectFiles(root);
  const allowedPaths = new Set([...deployedFiles, "/", "/privacy/", "/terms/"]);
  for (const requestUrl of requests) {
    const url = new URL(requestUrl);
    assert.equal(url.origin, origin, `unexpected remote request: ${requestUrl}`);
    assert.ok(allowedPaths.has(url.pathname), `unexpected same-origin request: ${url.pathname}`);
  }
  assert.equal((await context.cookies()).length, 0);
  assert.deepEqual(await page.evaluate(() => ({ local: localStorage.length, session: sessionStorage.length })), { local: 0, session: 0 });
  assert.deepEqual(await page.evaluate(async () => indexedDB.databases ? (await indexedDB.databases()).map(database => database.name) : []), []);
  const cacheState = await page.evaluate(async () => {
    const names = await caches.keys();
    const entries = [];
    for (const name of names) {
      const cache = await caches.open(name);
      entries.push(...(await cache.keys()).map(request => new URL(request.url).pathname));
    }
    return { names, entries };
  });
  assert.ok(cacheState.names.every(name => name === "arazzo-proof-shell-v2"));
  assert.ok(cacheState.entries.every(path => allowedPaths.has(path)), `unexpected cache entry: ${cacheState.entries}`);
  assert.equal(await page.locator('script[src^="http"]').count(), 0);
  await context.close();
});

test("@claim:routing-and-metadata routes have titles, metadata, focus, skeleton, and designed 404", async () => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  const expectedHeader = ["Demo", "How it works", "Privacy", "Source on GitHub (external)"];
  const expectedHeaderHrefs = ["/?demo=1#sample-report", "/#how", "/privacy/", "https://github.com/B-Divyesh/sf-arazzo-proof-runner"];
  const expectedFooter = ["Privacy", "Terms", "GitHub (external)"];
  const expectedFooterHrefs = ["/privacy/", "/terms/", "https://github.com/B-Divyesh/sf-arazzo-proof-runner"];
  const routes = [
    ["/", 200, "Arazzo Proof Runner — save a workflow proof bundle", "Run Arazzo workflows and save a proof bundle", "https://arazzo-proof-runner.sociobot.in/", false],
    ["/demo", 200, "Demo — Arazzo Proof Runner", "Run Arazzo workflows and save a proof bundle", "https://arazzo-proof-runner.sociobot.in/?demo=1", true],
    ["/privacy/", 200, "Privacy — Arazzo Proof Runner", "Privacy", "https://arazzo-proof-runner.sociobot.in/privacy/", true],
    ["/terms/", 200, "Terms — Arazzo Proof Runner", "Terms", "https://arazzo-proof-runner.sociobot.in/terms/", true],
    ["/not-a-real-route", 404, "Page not found — Arazzo Proof Runner", "This path has no step", "https://arazzo-proof-runner.sociobot.in/404.html", true]
  ];
  for (const [path, status, title, heading, canonical, shouldFocus] of routes) {
    const response = await page.goto(origin + path, { waitUntil: "networkidle" });
    assert.equal(response.status(), status);
    assert.equal(await page.title(), title);
    assert.equal(await page.locator("h1").textContent(), heading);
    assert.equal(await page.locator('meta[name="description"]').getAttribute("content").then(value => Boolean(value)), true);
    assert.equal(await page.locator('link[rel="canonical"]').getAttribute("href"), canonical);
    for (const selector of ['meta[property="og:title"]', 'meta[property="og:description"]', 'meta[property="og:image"]', 'meta[name="twitter:card"]', 'meta[name="twitter:title"]', 'meta[name="twitter:description"]', 'meta[name="twitter:image"]', 'link[rel="icon"]', 'link[rel="apple-touch-icon"]']) {
      assert.equal(await page.locator(selector).count(), 1, `${path}: ${selector}`);
    }
    assert.deepEqual(await page.locator("header nav a").allTextContents().then(values => values.map(value => value.trim().replace(/\s+/g, " "))), expectedHeader);
    assert.deepEqual(await page.locator("header nav a").evaluateAll(links => links.map(link => link.getAttribute("href"))), expectedHeaderHrefs);
    assert.deepEqual(await page.locator("footer nav a").allTextContents().then(values => values.map(value => value.trim().replace(/\s+/g, " "))), expectedFooter);
    assert.deepEqual(await page.locator("footer nav a").evaluateAll(links => links.map(link => link.getAttribute("href"))), expectedFooterHrefs);
    assert.equal(await page.locator("footer .factory-credit").textContent(), "Built by Param Factory");
    assert.equal(await page.locator("footer .build-id").textContent(), "v0.1.0 · polish-3");
    assert.equal(await page.locator('a.skip-link[href="#main"]').count(), 1);
    assert.equal(await page.locator("header nav").count(), 1);
    assert.equal(await page.locator("footer").count(), 1);
    if (shouldFocus) assert.equal(await page.evaluate(() => document.activeElement?.tagName), "H1");
    if (status === 404) assert.equal(await page.getByRole("link", { name: "Return home" }).count(), 1);
  }
  await context.close();
});

test("hash and browser history navigation restore heading focus", async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(origin);
  await page.getByRole("link", { name: "How it works" }).first().click();
  await page.waitForFunction(() => document.activeElement?.id === "how-title");
  assert.equal(await page.evaluate(() => document.activeElement?.id), "how-title");
  await page.getByRole("navigation", { name: "Primary navigation" }).getByRole("link", { name: "Privacy" }).click();
  await page.waitForFunction(() => document.activeElement?.tagName === "H1");
  assert.equal(await page.evaluate(() => document.activeElement?.tagName), "H1");
  await page.goBack();
  await page.waitForFunction(() => document.activeElement?.id === "how-title");
  assert.equal(await page.evaluate(() => document.activeElement?.id), "how-title");
  await context.close();
});

test("deploy artifact stays inside static performance budgets", async () => {
  const hero = await stat(join(root, "proof-strata.webp"));
  assert.ok(hero.size <= 300 * 1024, `hero is ${hero.size} bytes`);
  const assetNames = await readdir(join(root, "assets"));
  const sizes = await Promise.all(assetNames.map(async name => ({ name, size: (await stat(join(root, "assets", name))).size })));
  const js = sizes.filter(asset => asset.name.endsWith(".js")).reduce((sum, asset) => sum + asset.size, 0);
  const css = sizes.filter(asset => asset.name.endsWith(".css")).reduce((sum, asset) => sum + asset.size, 0);
  assert.ok(js <= 200 * 1024, `initial JS is ${js} bytes`);
  assert.ok(css <= 50 * 1024, `CSS is ${css} bytes`);
});
