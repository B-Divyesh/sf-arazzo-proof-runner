# Arazzo Proof Runner — build handoff

Work order: `arazzo-proof-runner-build-1`

Version: `0.1.0`

Completed: 2026-08-27

## What shipped

- A single Rust binary, `arazzo-proof`, with non-interactive `run` and `compare` commands, useful `--help`, documented exit codes, and `--json` summaries.
- A practical, explicitly documented Arazzo 1.0.x subset: local OpenAPI 3.0/3.1 sources, operation IDs and local operation paths, four parameter locations, JSON request bodies, workflow/environment/step-output substitutions, step outputs, and six success-criterion comparison operators.
- Required environment files with base URL overrides, workflow inputs, extension values, headers, secrets, and JSON Pointer body redaction.
- Stable `proof.json` plus self-contained `report.html` bundles. No timestamp, request duration, or volatile response headers enter the artifact.
- Baseline/current comparison through `comparison.json` and one-file `comparison.html`; response and assertion changes are shown at their workflow step.
- Defense-in-depth redaction for authorization, cookies, API keys, token/secret header names, configured secret values, assertion actual/expected values, URLs, bodies, outputs, and hand-authored input proof files during comparison.
- Three representative in-process HTTP workflow fixtures plus a true binary-level CLI test.
- A responsive Vite documentation site in `dist/site/` with a keyboard-operable proof-diff specimen, offline shell cache, dark treatment, privacy/terms pages, and no analytics, remote scripts, or third-party fonts.
- Original `proof-strata.webp` hero generated with the required factory image deployment and compressed to 208 KB. Full provenance and prompt are in `.factory/design.md`.

## Run and verify

```sh
npm ci
npm test
npm run build
cargo clippy --all-targets --all-features -- -D warnings
cargo package
```

The static deployment command is `npm run build` and its root is `dist/site/` (`dist/site/index.html` is present). The CLI release artifact is ready for factory publishing with `cargo package`; no publish was attempted.

Verified locally on 2026-08-27:

- `npm test`: passed (5 Rust unit tests, 2 CLI tests, 3 live workflow tests, 1 doctest, and 4 browser/site tests).
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo package`: passed; 42.2 KB compressed crate.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- Factory `verify-url.sh`: HTTP 200, title and `lang` present, exactly one h1, main landmark present, 0 missing image alts, 0 unlabeled buttons, 0 console/page errors.
- Playwright + axe: 0 serious or critical violations; keyboard proof toggle passes; 390×844 layout has no horizontal overflow.
- Lighthouse 12.8.2 mobile/local: Performance 99, Accessibility 100, Best Practices 100, SEO 100; FCP 0.94 s, LCP 2.14 s, CLS 0, total blocking time 0 ms. Lab Lighthouse does not report field INP; 0 ms TBT is the interaction proxy.
- Initial built assets: 2.08 KB JavaScript, 10.85 KB CSS, no font payload, 208 KB hero WebP.

## Known limits

- This intentionally remains an Arazzo subset. Remote source descriptions, callbacks/webhooks, OAuth acquisition, external reference resolution, JSONPath criteria, success/failure actions, retries, and non-JSON request bodies are not implemented. The README labels the boundary and the runner errors on unsupported execution constructs it encounters.
- The runner executes steps in declared order and stops on configuration or transport errors. Assertion failures still complete the workflow and produce evidence, then return exit code 1.
- OpenAPI schemas are used to locate operations and servers, not for request/response schema validation.
- The generated crate is ready to publish, but factory registry credentials and release automation are intentionally untouched.

## Suggested next steps

Track Arazzo specification revisions, add external reference resolution only with strict local/remote trust controls, and publish signed binaries for the main desktop CI platforms through factory release automation.
