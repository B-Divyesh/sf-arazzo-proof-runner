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
    const safe = normalize(pathname).replace(/^(\.\.[/\\])+/, "");
    let path = join(root, safe);
    try {
      if ((await stat(path)).isDirectory()) path = join(path, "index.html");
      const body = await readFile(path);
      const type = { ".html": "text/html", ".css": "text/css", ".js": "text/javascript", ".svg": "image/svg+xml", ".webp": "image/webp" }[extname(path)] ?? "application/octet-stream";
      response.writeHead(200, { "content-type": type });
      response.end(body);
    } catch {
      response.writeHead(404);
      response.end("Not found");
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

test("landing page has no serious accessibility violations or console errors", async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const errors = [];
  page.on("console", message => { if (message.type() === "error") errors.push(message.text()); });
  page.on("pageerror", error => errors.push(error.message));
  await page.goto(origin, { waitUntil: "networkidle" });
  assert.equal(await page.locator("h1").count(), 1);
  assert.equal(await page.locator("main").count(), 1);
  assert.match(await page.title(), /Arazzo Proof Runner/);
  const results = await new AxeBuilder({ page }).analyze();
  const serious = results.violations.filter(item => ["serious", "critical"].includes(item.impact));
  assert.deepEqual(serious, []);
  assert.deepEqual(errors, []);
  await context.close();
});

test("proof specimen exposes a changed assertion to keyboard users", async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const response = await page.goto(origin);
  assert.equal(response.status(), 200);
  const toggle = page.locator("[data-demo-toggle]");
  assert.equal((await toggle.textContent()).trim(), "Inject changed response");
  assert.equal(await page.locator(".diff-note").isVisible(), false);
  await toggle.focus();
  await page.keyboard.press("Enter");
  assert.equal(await toggle.getAttribute("aria-pressed"), "true");
  await assert.doesNotReject(() => page.getByText("× Fail").waitFor());
  assert.equal(await page.locator(".diff-note").isVisible(), true);
  assert.match(await page.locator(".demo-status").textContent(), /expected USD, received EUR/);
  await context.close();
});

test("390px layout does not overflow and policy pages resolve", async () => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  await page.goto(origin);
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth);
  assert.equal(overflow, false);
  await page.goto(`${origin}/privacy/`);
  assert.equal(await page.locator("h1").textContent(), "Privacy");
  await page.goto(`${origin}/terms/`);
  assert.equal(await page.locator("h1").textContent(), "Terms");
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
