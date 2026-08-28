import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

test("every registered claim has exactly one active tagged test", async () => {
  const claims = JSON.parse(await readFile(".factory/claims.json", "utf8"));
  const ids = claims.map(claim => claim.id);
  assert.equal(new Set(ids).size, ids.length, "claim IDs must be unique");
  const sources = await Promise.all([
    "tests/claims.rs",
    "tests/cli.rs",
    "tests/workflows.rs",
    "site/tests/site.test.mjs"
  ].map(path => readFile(path, "utf8")));
  const joined = sources.join("\n");
  for (const claim of claims) {
    assert.match(claim.id, /^[a-z0-9]+(?:-[a-z0-9]+)*$/);
    assert.ok(claim.claim && claim.where && claim.test && claim.sandbox, `${claim.id} is incomplete`);
    const tag = `@claim:${claim.id}`;
    assert.equal(joined.split(tag).length - 1, 1, `${claim.id} must have exactly one tagged test`);
  }
  const activeTags = [...joined.matchAll(/@claim:([a-z0-9-]+)/g)].map(match => match[1]);
  assert.deepEqual(new Set(activeTags), new Set(ids), "active claim tags must match the manifest");
});
