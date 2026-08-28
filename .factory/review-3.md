# Adversarial first-read review 3 — Arazzo Proof Runner

Reviewed: 2026-08-28  
Work order: `arazzo-proof-runner-review-3`  
Candidate and clean clone: `99abc7f42e2b47396b17e069fe782e76434ced92`  
Live site: <https://arazzo-proof-runner.sociobot.in/>

## Verdict: FAIL

Two findings remain. The product is clear and tryable, and every registered claim command passed. The footer is still missing the required Param Factory attribution, and the public copy makes a price claim that is not represented by a claim entry or an observable test.

## Cold first read

Fresh Chromium contexts at 390 × 844 and 1440 × 900 were loaded with no prior cookies or storage. The page was inspected at `scrollY = 0` before interaction.

| Viewport | What it does | Who it is for | First click | Result |
| --- | --- | --- | --- | --- |
| 390 px | Runs multi-step Arazzo API workflows and saves a redacted proof bundle. | API owners testing multi-step workflows. | “Try it with sample data.” | Pass |
| 1440 px | The same; the workflow illustration reinforces the sequence. | API owners testing multi-step workflows. | “Try it with sample data.” | Pass |

The complete first-screen wording is clear: “Run Arazzo workflows and save a proof bundle”; “For API owners testing multi-step workflows, it records redacted requests, responses, and assertion results”; and “Runs a bundled checkout workflow and opens its report.” All three facts and the action were visible before the fold at both widths. There is no cold-read blocking finding.

## Findings, ordered by severity

### MAJOR F-3-1 — the price claim is not registered or testable

Location: landing first-screen fact and `/terms/`.

Exact quotes:

- “Free and MIT licensed.”
- “Arazzo Proof Runner is free software under the MIT License.”

Why this fails the claims check: `.factory/claims.json` has `single-binary-mit`, whose promise and test establish one binary, version `0.1.0`, and the MIT license. It does not contain or test a price/distribution promise. “Free” is therefore an unlisted claim a visitor could rely on.

Concrete fix: remove “Free” from both public statements and retain the tested wording “MIT licensed,” or add a specific `free-to-use` claim with a deterministic, observable release/distribution test. Removing the untestable price assertion is the smaller and clearer fix.

### MINOR F-3-2 — every footer omits the required factory attribution

Location: live `/`, `/?demo=1`, `/privacy/`, `/terms/`, and the designed 404; source: each `site-footer` in `site/index.html`, `site/privacy/index.html`, `site/terms/index.html`, and `site/404.html`.

Exact footer content is “Arazzo Proof Runner”; “Run Arazzo workflows locally and save a redacted proof bundle”; `v0.1.0 · polish-2`; and Privacy, Terms, GitHub links. It does not include “Built by Param Factory.”

Why this fails the structure check: the footer is otherwise consistent, but the required standard skeleton includes the product one-liner, Privacy, Terms, **Built by Param Factory**, and a build/version identifier on every route.

Concrete fix: add a visible “Built by Param Factory” footer item to all four templates, keep its styling and link treatment consistent, and extend `@claim:routing-and-metadata` to assert it on Home, Demo, Privacy, Terms, and 404.

## Copy audit

Counts are whitespace-delimited after rendering. Commands and list-item fragments are not sentences; headings, controls, terminology, and those fragments were separately checked below. No audited sentence exceeds 22 words. No banned marketing adjective appears. The only copy finding is F-3-1.

### Landing-page sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 1 | Offline. | — |
| 7 | The guide and sample report remain available. | — |
| 7 | Demo — sample data, nothing is saved. | — |
| 11 | This page uses an in-memory sample and never reads browser storage. | — |
| 14 | For API owners testing multi-step workflows, it records redacted requests, responses, and assertion results. | — |
| 9 | Runs a bundled checkout workflow and opens its report. | — |
| 2 | Runs locally. | — |
| 4 | No account or telemetry. | — |
| 4 | Free and MIT licensed. | F-3-1: remove “Free”; use “MIT licensed.” |
| 8 | A report links each input, request, and assertion. | — |
| 4 | Install from the repository. | — |
| 11 | Then run the bundled demo without an account or API setup. | — |
| 4 | All sample assertions pass. | — |
| 5 | `proof.json` records machine-readable evidence. | — |
| 7 | The self-contained `report.html` is ready for review. | — |
| 13 | The CLI replaces authorization headers and configured secrets with `[REDACTED]` before writing either file. | — |
| 12 | Point the CLI to a local workflow and an explicit environment file. | — |
| 11 | The CLI calls only the base URL selected in that file. | — |
| 10 | Review each request, response, output, and assertion by workflow step. | — |
| 10 | The site does not receive workflow files or proof bundles. | — |
| 9 | Your selected API still receives the requests you run. | — |
| 8 | The CLI redacts configured secrets and sensitive headers. | — |
| 7 | Review response bodies before sharing a report. | — |
| 6 | Unsupported Arazzo features return an error. | — |
| 15 | The CLI names the step and feature, exits 2, and writes no partial proof bundle. | — |
| 9 | Share one HTML report instead of pasted response snippets. | — |
| 10 | Run Arazzo workflows locally and save a redacted proof bundle. | — |
| 2 | Command copied. | — |
| 3 | Copy was blocked. | — |
| 4 | Select the command manually. | — |
| 9 | The quoteCart assertion now fails: expected USD, received EUR. | — |
| 2 | Demo reset. | — |

### README sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 10 | `arazzo-proof` runs the documented Arazzo 1.0.x features on your machine. | — |
| 8 | Choose an environment and run a multi-step workflow. | — |
| 12 | The CLI writes a redacted proof bundle with JSON and HTML reports. | — |
| 12 | It is for API owners who need step-by-step evidence while adopting Arazzo. | — |
| 10 | The CLI has no account, telemetry, upload, or hosted service. | — |
| 13 | The demo starts its own loopback API and needs no setup or network service. | — |
| 13 | It creates a unique temporary workspace and prints the generated report path. | — |
| 7 | The same sample is visible at the demo URL. | — |
| 6 | See `.factory/demo.md` for its isolation contract. | — |
| 6 | The current package version is `0.1.0`. | — |
| 8 | Run `cargo package` to validate the release package. | — |
| 9 | The example above expects a compatible API at `127.0.0.1:4010`. | — |
| 10 | Use `arazzo-proof demo` for a first run without that API. | — |
| 9 | This writes `comparison.json` and a self-contained `comparison.html`. | — |
| 12 | The report marks each changed status, body, output, or assertion at its workflow step. | — |
| 7 | Every run command requires an environment file. | — |
| 8 | The CLI accepts YAML and JSON environment files. | — |
| 8 | The CLI rejects unsupported features with an error. | — |
| 2 | These include: | — |
| 8 | The error names the affected step or source. | — |
| 12 | Invalid or unsupported input exits 2 before writing a partial proof bundle. | — |
| 7 | `--json` keeps the same exit codes. | — |
| 7 | The CLI does not prompt for input. | — |
| 8 | Claim commands are listed in `.factory/claims.json`. | — |
| 8 | The factory deploys `dist/site/` as a static site. | — |
| 6 | Build it with `npm run build:site`. | — |
| 8 | Do not deploy the CLI from this repository. | — |
| 1 | MIT. | — |
| 2 | See `LICENSE`. | — |

Headings are understandable in isolation, including “Run Arazzo workflows and save a proof bundle,” “See exactly where a response changed,” “How it works,” “What the CLI does not do,” and “Supported Arazzo features and explicit limits.” Result-naming buttons are “Try it with sample data,” “View install command,” “Copy command,” “Inject changed response,” “Restore baseline response,” “Reset demo,” and “Start for real.” The terminology remains consistent: the complete output is a **proof bundle**; `proof.json` is the machine-readable file; `report.html` is the HTML report; and the executable is the CLI.

## Demo, sandbox, and privacy exercises

- The visible first-screen “Try it with sample data” action reached `/?demo=1#sample-report` in one click. At 390 px it scrolled to the live three-step `createCart` / `addItem` / `quoteCart` checkout evidence, with the heading focused and the persistent banner at the top.
- The banner says “Demo — sample data, nothing is saved” and exposes Reset demo and Start for real. Reset restored USD, the passing stamp, and the original action label after injecting the EUR response.
- A fresh browser context was seeded with `real:` localStorage, sessionStorage, and IndexedDB data before entering and mutating demo. The seeded data survived unchanged; demo created no storage keys, cookies, or IndexedDB database.
- Network interception in the registered privacy exercise accepted only exact first-party deployed files. The offline exercise loaded Demo online, activated the service worker, went offline, and reloaded Demo and Privacy successfully.
- From a separate temporary caller directory, `cargo run --manifest-path /tmp/arazzo-review3-tFXb6e/repo/Cargo.toml -- demo` printed a unique `/tmp/arazzo-proof-demo-…` workspace with `proof.json` and `report.html`; no output appeared in the caller directory.

## Claim commands from a clean clone

Clean clone: `/tmp/arazzo-review3-tFXb6e/repo` at `99abc7f42e2b47396b17e069fe782e76434ced92`. `npm ci --ignore-scripts` completed before the checks. Every manifest command passed independently:

| Claim ID | Result |
| --- | --- |
| `bundled-demo` | Pass |
| `redaction-and-chaining` | Pass |
| `operation-selection-and-parameters` | Pass |
| `comparison-report` | Pass |
| `exit-codes-and-json` | Pass |
| `cli-help` | Pass |
| `unsupported-errors` | Pass |
| `single-binary-mit` | Pass |
| `web-demo-isolation` | Pass |
| `offline-site` | Pass |
| `site-privacy` | Pass |
| `routing-and-metadata` | Pass |

The repaired operation test now uses YAML and JSON Arazzo/OpenAPI/environment inputs, both OpenAPI versions and operation selectors, all parameter positions, and all six operators. The exit-code test covers `run` and `compare` at 0/1/2, parses JSON where output is promised, closes stdin, and uses a ten-second deadline. Thus no previous claim test remains partially covered. The clean-clone full suite also passed: 5 unit tests, 3 claims tests, 2 CLI tests, 3 workflow tests, 1 doctest, 3 copy/manifest tests, and 8 site/browser checks; `npm run build:site` produced `dist/site/`.

## Route, accessibility, and link checks

Home, Demo, Privacy, Terms, and the unknown route were directly loaded at 390 px. All have one `h1`, a description, canonical, Open Graph/Twitter image metadata, favicon and Apple touch icon; the unknown route returns a designed product 404 with a 404 status. Titles are product-specific and plain: Home is “Arazzo Proof Runner — save a workflow proof bundle”; Demo, Privacy, Terms, and 404 use their route-specific product titles. Deep links, Demo redirect, hash navigation, and browser Back restored heading focus; live `/demo` resolves to the demo.

All collected public links returned 200 after redirects; the current-page 404 skip-link correctly remains an in-document hash on its already-404 document. The header and legal links are otherwise consistent. The original concrete-and-moss illustration, near-square evidence slabs, visible seams, and restrained motion match `.factory/design.md` and are not a generic SaaS template. The pinned Playwright Axe suite passed all public routes at 390 px and desktop in light and dark modes, with no console errors on 200 routes.

## Earlier finding recheck

Every earlier finding was checked against the live deployment and source, not accepted from its marked status alone.

| Earlier IDs | Status in this review | Evidence |
| --- | --- | --- |
| `B1`, `B2`, `B4` | Fixed | Cold-read content, real web/CLI demo, and live product 404 checks above. |
| `B3`; `F-2-1`; `UC-L12`–`UC-L14`; `UC-R14`, `UC-R21`, `UC-R23`, `UC-R24`; `UC-P02` | Fixed | All 12 clean-clone manifest commands passed; current test source contains the complete operation, exit, browser-isolation, privacy allowlist, and route matrices. |
| `M1`–`M7`; `F-2-2`; `F-2-3` | Fixed | Mobile Axe passes, focusable command scroller, shared nav set, canonical 404, all route metadata, headings, and links were verified live. |
| `M4`, `M5`, `M6`; `F-2-4`, `F-2-5` | Fixed | Landing order is Demo → usage → boundaries; install points to the real demo; no nested complementary landmark; “proof bundle” is consistent; removed publishing process claim stays absent. |
| `UC-L01`–`UC-L28` except the price wording now raised as `F-3-1` | Fixed | Offline, demo, output, redaction, support matrix, unsupported-error, and local/privacy assertions were rerun. |
| `UC-R01`–`UC-R28` | Fixed | README matches the tested command behavior, terminology, local demo, exit codes, and supported boundary. |
| `UC-P01`–`UC-P08` except the price wording now raised as `F-3-1` | Fixed | Privacy/terms statements align with selected-target, redaction, offline-cache, license, and rejection tests. |
| `CW-L01`–`CW-L24`; `CW-R01`–`CW-R15` | Fixed | The sentence-by-sentence audit above found no old long, vague, or inconsistent wording. |

`F-3-1` and `F-3-2` are newly identified checklist defects, not regressions of an earlier finding ID.

## Missed leverage

No missed-leverage finding. The brief implies local Arazzo/OpenAPI/environment input, redacted JSON/HTML proof output, and comparison; all are present. Sync would weaken the local-first privacy boundary. An AI action would not make deterministic evidence more trustworthy and is not implied by the job, so there is no need for a Sociobot gateway feature.

## What would make this perfect

1. Remove or register and test the “Free” price claim.
2. Add “Built by Param Factory” consistently to every footer and assert it in the routing metadata test.
3. Rerun this complete first-read, claims, sandbox, history, route, link, and accessibility checklist after deployment. Only then should the verdict become PASS.
