# Adversarial first-read review 4 — Arazzo Proof Runner

Reviewed: 2026-08-28  
Work order: `arazzo-proof-runner-review-4`  
Candidate and clean-clone SHA: `15f018621d8f77a09ab15e74318a259c6640ee4b`  
Live site: <https://arazzo-proof-runner.sociobot.in/>

## Verdict: PASS

No blocking, major, minor, or unlisted-claim findings remain. The full first-read, demo, sandbox, claims, route, accessibility, copy, and historical-finding checks were run again; this is not a diff-only review.

## Cold first read, before scrolling

Fresh Chromium contexts at 390 × 844 and 1440 × 900 had no cookies or prior storage. Both screens gave the same answers within the first viewport.

| Question | Cold-reader answer | First-screen evidence |
| --- | --- | --- |
| What does it do? | It runs Arazzo API workflows and saves a redacted proof bundle. | “Run Arazzo workflows and save a proof bundle” |
| Who is it for? | API owners testing multi-step workflows. | “For API owners testing multi-step workflows…” |
| What should I click first? | “Try it with sample data”; it runs a checkout workflow and opens its report. | Primary action plus “Runs a bundled checkout workflow and opens its report.” |

This passed on mobile and desktop. The 390 px first screen has one clear green primary action, its immediate outcome, and three short facts. It is not a generic SaaS hero: the concrete-and-moss evidence bench, square slabs, ledger navigation, and workflow seam match the documented visual thesis.

## Copy audit

Counts use whitespace-delimited words; code identifiers count as one word. The audit includes dynamic status text because a visitor can encounter it. All complete sentences are at or below 22 words. No banned marketing term, unexplained promotional adjective, terminology conflict, or non-result action was found. Necessary technical terms are named for the intended API-owner audience and are explained by the surrounding sample or documentation.

### Landing-page sentences

| Words | Sentence |
| ---: | --- |
| 1 | Offline. |
| 7 | The guide and sample report remain available. |
| 7 | Demo — sample data, nothing is saved. |
| 11 | This page uses an in-memory sample and never reads browser storage. |
| 14 | For API owners testing multi-step workflows, it records redacted requests, responses, and assertion results. |
| 9 | Runs a bundled checkout workflow and opens its report. |
| 2 | Runs locally. |
| 4 | No account or telemetry. |
| 2 | MIT licensed. |
| 8 | A report links each input, request, and assertion. |
| 4 | Install from the repository. |
| 11 | Then run the bundled demo without an account or API setup. |
| 4 | All sample assertions pass. |
| 5 | `proof.json` records machine-readable evidence. |
| 7 | The self-contained `report.html` is ready for review. |
| 13 | The CLI replaces authorization headers and configured secrets with `[REDACTED]` before writing either file. |
| 12 | Point the CLI to a local workflow and an explicit environment file. |
| 11 | The CLI calls only the base URL selected in that file. |
| 10 | Review each request, response, output, and assertion by workflow step. |
| 10 | The site does not receive workflow files or proof bundles. |
| 9 | Your selected API still receives the requests you run. |
| 8 | The CLI redacts configured secrets and sensitive headers. |
| 7 | Review response bodies before sharing a report. |
| 6 | Unsupported Arazzo features return an error. |
| 15 | The CLI names the step and feature, exits 2, and writes no partial proof bundle. |
| 9 | Share one HTML report instead of pasted response snippets. |
| 9 | Run Arazzo workflows locally and save a redacted proof bundle. |
| 2 | Command copied. |
| 3 | Copy was blocked. |
| 4 | Select the command manually. |
| 9 | The quoteCart assertion now fails: expected USD, received EUR. |
| 2 | Demo reset. |

### README sentences

| Words | Sentence |
| ---: | --- |
| 10 | `arazzo-proof` runs the documented Arazzo 1.0.x features on your machine. |
| 8 | Choose an environment and run a multi-step workflow. |
| 12 | The CLI writes a redacted proof bundle with JSON and HTML reports. |
| 12 | It is for API owners who need step-by-step evidence while adopting Arazzo. |
| 10 | The CLI has no account, telemetry, upload, or hosted service. |
| 13 | The demo starts its own loopback API and needs no setup or network service. |
| 13 | It creates a unique temporary workspace and prints the generated report path. |
| 7 | The same sample is visible at the demo URL. |
| 6 | See `.factory/demo.md` for its isolation contract. |
| 7 | Build the single `arazzo-proof` binary with Rust. |
| 8 | The current package version is `0.1.0`. |
| 8 | Run `cargo package` to validate the release package. |
| 10 | Run one workflow and write `proof.json` plus a self-contained `report.html`. |
| 9 | The example above expects a compatible API at `127.0.0.1:4010`. |
| 10 | Use `arazzo-proof demo` for a first run without that API. |
| 12 | Print a JSON summary to stdout while writing the same proof bundle. |
| 5 | Compare two `proof.json` files. |
| 9 | This writes `comparison.json` and a self-contained `comparison.html`. |
| 14 | The report marks each changed status, body, output, or assertion at its workflow step. |
| 7 | Every run command requires an environment file. |
| 8 | The CLI accepts YAML and JSON environment files. |
| 8 | The CLI rejects unsupported features with an error. |
| 2 | These include. |
| 8 | The error names the affected step or source. |
| 12 | Invalid or unsupported input exits 2 before writing a partial proof bundle. |
| 7 | `--json` keeps the same exit codes. |
| 7 | The CLI does not prompt for input. |
| 8 | Claim commands are listed in `.factory/claims.json`. |
| 10 | The factory deploys `dist/site/` as a static site. |
| 6 | Build it with `npm run build:site`. |
| 8 | Do not deploy the CLI from this repository. |
| 1 | MIT. |
| 2 | See `LICENSE`. |

The source’s “These include:” is a two-word lead-in; its punctuation was normalised to a sentence for the count. README list items are declarative fragments under precise headings, not hidden marketing claims.

### Headings, actions, and terminology

Headings are meaningful in the page outline: “Install the CLI,” “See exactly where a response changed,” “What the CLI does not do,” and “Supported Arazzo features and explicit limits.” The required “How it works” section is backed by three specific step headings. Actions name their outcome: “Try it with sample data,” “View install command,” “Copy command,” “Inject changed response,” “Reset demo,” and the required demo exit action “Start for real.”

The complete artifact is consistently a **proof bundle**; its files are consistently `proof.json` and an HTML report / `report.html`. “Demo,” “CLI,” “environment file,” and “base URL” also keep one meaning across the landing page and README.

## Demo and sandbox exercise

The first screen's one-click action opened `/?demo=1#sample-report`. The first post-click phone viewport showed the persistent demo banner, the real-command terminal recording (`checkout: 3 steps, 3 assertions passed` and a report path), and the sample report rather than a setup form or blank state.

- “Demo — sample data, nothing is saved,” “Reset demo,” and “Start for real” were present.
- “Inject changed response” changed `quoteCart` from USD/pass to EUR/fail; Reset restored the pass state.
- Pre-seeded `localStorage`, `sessionStorage`, and IndexedDB real-data sentinels were unchanged. No demo key, cookie, or database was added.
- A clean browser visit cached the shell; with the browser offline, Demo reloaded with its title, heading, banner, and sample. Privacy also reloaded.
- Network capture during the live demo requested only this origin's document, JavaScript, CSS, and owned image. There were no remote scripts, analytics, account, or API requests.
- From a separately created temporary directory, the clean-clone binary ran `arazzo-proof demo`. It created a uniquely named operating-system temporary workspace, printed the report and proof paths, wrote `proof.json` and self-contained `report.html`, and serialized `[REDACTED]` rather than the demo Authorization value.

## Claims execution

All twelve `.factory/claims.json` commands passed independently from clean clone `/tmp/arazzo-review4-clean-f6Lzmr/repo` at the candidate SHA.

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

The post-test live-copy cross-check found each claim-like sentence covered by one or more registered entries: bundled run/output, redaction/chaining, documented feature matrix, comparison, exit codes, unsupported boundaries, single-binary/MIT, demo isolation, offline site, privacy, or routing/metadata. No unlisted claim remains.

The clean clone built `dist/site/` (3,959 B JavaScript and 14,076 B CSS before gzip), passed formatting and strict Clippy, and passed `cargo package` (181.3 KiB, 47.0 KiB compressed). The local production Home, Privacy, Terms, and 404 HTML SHA-256 values exactly matched the corresponding live responses.

## Route, structure, and accessibility checks

Home, Demo, Privacy, Terms, and the unknown-route 404 were loaded directly. They have one `h1`, one `main`, route-appropriate titles/descriptions/canonicals, OG/Twitter metadata, SVG favicon, Apple touch icon, skip link, shared header/footer, Privacy and Terms links, factory attribution, and build marker. `/demo` redirects to the direct demo URL; unknown routes return the product-designed 404 with a return-home action and HTTP 404.

The live header/footer crawl found all navigable targets healthy: Home, Demo, Privacy, Terms, and GitHub returned 200. The only 404 response encountered was the current unknown 404 page's own skip-link target, an intentional same-document anchor rather than a dead destination. Browser Back from Privacy restored focus to the `How it works` heading; direct Demo/Privacy/Terms/404 route loads focused the route heading and announced it. The checked mobile and desktop route matrix had no console errors or Axe violations; mobile controls remained at least 44 px and reduced motion removed transitions.

## Earlier-finding recheck

Every prior review and all three polish records were read. The following checks confirmed the fixes in both live behavior and current source/tests.

| Earlier IDs | Confirmed current state |
| --- | --- |
| Review 1 `B1`–`B4` | The audience/job/action is first-screen clear; a real CLI and web demo exist; the claims manifest has one active test per claim; `/demo` and the designed 404 route work. |
| Review 1 `M1`–`M7` | Scrollable command is keyboard-focusable; all routes use the shared skeleton and focus behavior; metadata/assets are complete; the three-step path and privacy limits are present; the install path uses the bundled demo; Axe has no landmark or mobile violations; external destinations are labelled. |
| Review 1 `UC-L01`–`UC-L28` | Offline, demo/output, no-account/telemetry, redaction, supported matrix, unsupported boundaries, diff/reset, pricing removal, and concrete copy reproduce through the registered claim matrix and live checks. |
| Review 1 `UC-R01`–`UC-R28` | README behavior, selected-target boundary, output/comparison, input/exit-code matrix, redaction, package/license, and removal of untestable promotion reproduce through the same clean-clone tests and source audit. |
| Review 1 `UC-P01`–`UC-P08` | The selected-base-URL, site request/storage boundary, offline shell, cache-removal instruction, redaction caveat, MIT/warranty, and documented scope remain accurate. |
| Review 1 `CW-L01`–`CW-L24`, `CW-R01`–`CW-R15` | The complete audit above confirms short, direct, consistent landing and README copy; first-screen facts, actions, errors, and terms remain clear. |
| Review 2 `F-2-1`–`F-2-5` | Each manifest entry ran its whole observable test; shared navigation matches on all routes; the 404 canonical is present; “proof bundle” is consistent; the untestable publication sentence remains absent. |
| Review 3 `F-3-1`–`F-3-2` | No price claim remains in public copy; all route footers visibly include “Built by Param Factory” and `v0.1.0 · polish-3`. |

## Missed leverage

No missing AI feature is indicated. The brief is a local deterministic proof runner; adding a model call would weaken its local, reproducible, no-upload boundary. Import/export is already the product's Arazzo/OpenAPI/environment input and JSON/HTML proof output. No provider key or decorative AI feature is present.

## What would make this perfect

Keep this acceptance level on later releases: rerun every registered claim from a fresh clone, exercise the temporary-directory CLI demo and browser storage/network/offline checks, and reject any new public sentence until it has a matching claim test or is reduced to non-claim guidance. No additional product change is required for this reviewed candidate.
