import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const banned = /\b(leverage|seamless|effortless|robust|powerful|intuitive|reimagine|supercharge|unlock|delightful|journey|ecosystem|AI-powered)\b/i;

test("landing and README sentences use plain words", async () => {
  const html = (await readFile("site/index.html", "utf8"))
    .replace(/<head>[\s\S]*?<\/head>/, " ")
    .replace(/<script[\s\S]*?<\/script>/g, " ")
    .replace(/<code>[\s\S]*?<\/code>/g, "\n")
    .replace(/<[^>]+>/g, "\n");
  const readme = (await readFile("README.md", "utf8"))
    .replace(/```[\s\S]*?```/g, " ")
    .replace(/^#+.*$/gm, " ")
    .replace(/^[-*] /gm, " ");
  for (const [name, copy] of [["landing", html], ["README", readme]]) {
    const sentences = copy.split(/\n+/).flatMap(line => line.trim().split(/(?<=[.!?])\s+/)).map(value => value.trim()).filter(Boolean);
    for (const sentence of sentences) {
      const words = sentence.split(/\s+/).filter(Boolean).length;
      assert.ok(words <= 22, `${name} has ${words} words: ${sentence}`);
      assert.equal(banned.test(sentence), false, `${name} uses banned wording: ${sentence}`);
    }
  }
});

test("catalog description is verb-first and no longer than 120 characters", async () => {
  const description = (await readFile(".factory/catalog-description.txt", "utf8")).trim();
  assert.ok(description.length <= 120, `${description.length} characters`);
  assert.match(description, /^Run\b/);
});

test("public copy contains no unregistered price claim", async () => {
  const files = ["site/index.html", "site/privacy/index.html", "site/terms/index.html", "site/404.html", "README.md"];
  for (const path of files) {
    const copy = await readFile(path, "utf8");
    assert.doesNotMatch(copy, /\bfree\b/i, `${path} contains a price claim`);
  }
});
