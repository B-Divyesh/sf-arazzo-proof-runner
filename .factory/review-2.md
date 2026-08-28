# Adversarial first-read review 2 — Arazzo Proof Runner

Reviewed: 2026-08-28  
Work order: `arazzo-proof-runner-review-2`  
Candidate: `c039d8b669bda2e6af490862fb2f2d4857643125`  
Live site: <https://arazzo-proof-runner.sociobot.in/>

## Verdict: FAIL

The live product is clear on first read, has a credible one-click demo, preserves seeded real browser data, works offline, and passes the browser, accessibility, link, build, and CLI exercises. It still fails the zero-finding standard. The blocking defect is that several registered claims are broader than the assertions in their named tests. All 12 commands return green, but some promised behavior remains untested by the prescribed command. There are also four non-blocking consistency defects.

## Cold first read, before scrolling

Fresh Chromium contexts were opened at 390 × 844 and 1440 × 900 with no cookies or storage. Both were inspected at `scrollY = 0` before any interaction.

| Viewport | What does this do? | For whom? | What should I click first? | Result |
| --- | --- | --- | --- | --- |
| 390 px | It runs multi-step Arazzo API workflows and saves redacted request, response, and assertion evidence. | API owners testing multi-step workflows. | “Try it with sample data.” It says this opens a bundled checkout report. | Pass; all three answers and all three facts are visible before the fold. |
| 1440 px | The same, with the concrete workflow illustration reinforcing the three-stage run. | API owners testing multi-step workflows. | “Try it with sample data.” | Pass. |

The text that succeeds is: “Run Arazzo workflows and save proof” and “For API owners testing multi-step workflows, it records redacted requests, responses, and assertion results.” The primary action is “Try it with sample data,” followed by “Runs a bundled checkout workflow and opens its report.” No first-screen blocker remains.

## Findings, ordered by severity

### BLOCKING F-2-1 / prior B3 — green claim commands do not prove every registered promise

Locations: `.factory/claims.json`, `tests/claims.rs`, `tests/cli.rs`, `tests/workflows.rs`, and `site/tests/site.test.mjs`.

Every listed command passed, but these claims are still partly untested:

| Claim | Exact promise | What the named test actually omits | Concrete fix |
| --- | --- | --- | --- |
| `operation-selection-and-parameters` | “YAML and JSON files, OpenAPI 3.0 and 3.1, both operation selectors, all four parameter locations, and six comparison operators.” | The exact command filters to `operation_path_and_parameter_substitution_work`; its output says two tests were filtered out. That test exercises OpenAPI 3.0, `operationPath`, and query/header/cookie parameters. OpenAPI 3.1, `operationId`, and a path parameter only occur in the filtered chaining test. It also does not load an Arazzo JSON file. | Create one `@claim:operation-selection-and-parameters` test that runs 3.0 and 3.1, YAML and JSON inputs, both selectors, path/query/header/cookie parameters, and all six operators. Keep the manifest command scoped to that one test. |
| `exit-codes-and-json` | “Run and compare return exit 0 … 1 … and 2 …; `--json` emits parseable output without prompting.” | The test uses `demo` for exit 0, not a successful `run`. It does not test invalid `compare` exit 2, and it does not cover parseable JSON across the complete run/compare matrix. | Add a table-driven test for `run` and `compare` at 0, 1, and 2. Run applicable cases with `--json`, parse stdout, close stdin, and enforce a timeout. |
| `bundled-demo` | “unique temporary workspace” and “self-contained HTML.” | One invocation is checked only for a temp-directory prefix, so a fixed temp path would pass. HTML checks reject `https://` and external scripts but do not reject external styles, images, or other fetchable assets. | Run the demo twice and assert distinct workspaces. Parse the report and assert every `src`/`href` is absent, embedded, or an in-document anchor. |
| `web-demo-isolation` | Demo storage is isolated from real data. | The named test starts with empty storage. Code that overwrote or deleted an existing real key could still pass after a clean-context-only test. | Seed real localStorage, sessionStorage, and IndexedDB records before entering demo; mutate and reset the demo; assert every seeded value is unchanged and no `demo:` state persists. |
| `site-privacy` | “no analytics” and no persistent application data. | The test permits every same-origin request. A new same-origin `/analytics` request would pass. It also omits IndexedDB and Cache Storage inspection in this claim test. | Allowlist the expected document/assets/service worker paths and fail on any other request. Assert cookies, local/session storage, IndexedDB, and non-shell Cache Storage state. |
| `routing-and-metadata` | Home, demo, policy, and unknown routes all have metadata, navigation, focus behavior, and 404 treatment. | Full metadata/skeleton assertions run only for Home, Privacy, and Terms. Demo gets only a title assertion; the unknown route gets only status, `h1`, and Return Home. | Apply the complete title/description/canonical/OG/favicon/header/footer/focus assertion set to Demo and the designed 404 too, with an explicit 404 canonical policy. |

Why this is blocking: the claims contract requires the named test to assert the observable promise. A green command with relevant cases filtered out is not evidence for the advertised matrix. This is a half-fix of prior finding B3 and of prior claim findings `UC-L12`, `UC-L13`, `UC-L14`, `UC-R14`, `UC-R21`, `UC-R23`, `UC-R24`, and `UC-P02`.

### MAJOR F-2-2 — the shared header and footer do not keep the same navigation

Exact locations:

- Home header: “Demo / How it works / Limits / Source on GitHub.”
- Privacy and Terms headers: “Demo / How it works / Privacy.”
- 404 header: “Demo / Privacy / Terms.”
- The 404 footer drops the GitHub link present in the other footers.

Why this matters: moving between routes changes the available destinations and their positions. The required skeleton says the header is consistent on every route and includes Privacy; the home header does not.

Concrete fix: use one shared destination set and order on all route files, with only `aria-current` changing. Use the same footer links on the 404 page as on every 200 page.

### MINOR F-2-3 — the designed 404 has no canonical link

Location: live `/not-a-real-route` and `site/404.html`. Quote: the page has `meta name="robots" content="noindex"`, OG/Twitter tags, and favicons, but no `<link rel="canonical">`.

Why this matters: the route fails the requested canonical metadata checklist even though its 404 status and design are correct.

Concrete fix: document an explicit 404 canonical policy. If the factory contract requires a tag on every route, add a canonical for the designed `/404.html`; otherwise amend the check and claim so 404 pages are explicitly exempt.

### MINOR F-2-4 — the complete output has three competing names

Locations: “save proof” in the hero, “JSON and HTML reports” in the README opening, and “proof bundle” in the footer and later README sections.

Why this matters: a new visitor cannot tell whether “proof,” “reports,” and “proof bundle” are the same output or different artifacts. The repository’s terminology table itself says the complete run output is “proof bundle.”

Concrete fix: use “proof bundle” for the pair everywhere. Suggested headline: “Run Arazzo workflows and save a proof bundle.” Reserve `proof.json` for the machine-readable file and “HTML report” for the human-readable file.

### MINOR F-2-5 — one README process claim is not registered

Quote/location: README Install section, “Factory automation owns registry publishing.”

Why this matters: this is a claim a maintainer could rely on, but `.factory/claims.json` has no entry or test for the publishing ownership. It is also internal factory process rather than product guidance.

Concrete fix: remove the sentence from the public README, or add a repository-level test that verifies the actual release workflow and register the claim.

## Copy audit

Counts are whitespace-delimited after rendering. No sentence exceeds 22 words and no banned marketing word appears. The only copy flags are the terminology issue in F-2-4 and the unlisted process claim in F-2-5. Technical terms such as Arazzo, OpenAPI, JSON, and JSON Pointer are appropriate for the stated API-owner audience.

### Landing page sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 1 | “Offline.” | — |
| 7 | “The guide and sample report remain available.” | — |
| 7 | “Demo — sample data, nothing is saved.” | — |
| 11 | “This page uses an in-memory sample and never reads browser storage.” | — |
| 14 | “For API owners testing multi-step workflows, it records redacted requests, responses, and assertion results.” | — |
| 9 | “Runs a bundled checkout workflow and opens its report.” | — |
| 2 | “Runs locally.” | — |
| 4 | “No account or telemetry.” | — |
| 4 | “Free and MIT licensed.” | — |
| 8 | “A report links each input, request, and assertion.” | — |
| 4 | “Install from the repository.” | — |
| 11 | “Then run the bundled demo without an account or API setup.” | — |
| 4 | “All sample assertions pass.” | — |
| 5 | “`proof.json` records machine-readable evidence.” | — |
| 7 | “The self-contained `report.html` is ready for review.” | — |
| 13 | “The CLI replaces authorization headers and configured secrets with `[REDACTED]` before writing either file.” | — |
| 12 | “Point the CLI to a local workflow and an explicit environment file.” | — |
| 11 | “The CLI calls only the base URL selected in that file.” | — |
| 10 | “Review each request, response, output, and assertion by workflow step.” | — |
| 10 | “The site does not receive workflow files or proof bundles.” | — |
| 9 | “Your selected API still receives the requests you run.” | — |
| 8 | “The CLI redacts configured secrets and sensitive headers.” | — |
| 7 | “Review response bodies before sharing a report.” | — |
| 6 | “Unsupported Arazzo features return an error.” | — |
| 15 | “The CLI names the step and feature, exits 2, and writes no partial proof bundle.” | — |
| 9 | “Share one HTML report instead of pasted response snippets.” | — |
| 9 | “Run Arazzo workflows locally and save redacted proof bundles.” | — |
| 2 | “Command copied.” | — |
| 3 | “Copy was blocked.” | — |
| 4 | “Select the command manually.” | — |
| 9 | “The quoteCart assertion now fails: expected USD, received EUR.” | — |
| 2 | “Demo reset.” | — |

“All sample assertions pass.” occurs both as initial live-region copy and after reset; it is counted once above and checked in both states.

Landing headings and actions are all at or below nine words. The actionable controls use result-naming verbs: “Try it with sample data,” “View install command,” “Copy command,” “Inject changed response,” “Restore baseline response,” “Reset demo,” and “Start for real.” Navigation labels are destinations rather than action buttons. Headings make sense out of context.

### README sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 10 | “`arazzo-proof` runs the documented Arazzo 1.0.x features on your machine.” | — |
| 8 | “Choose an environment and run a multi-step workflow.” | — |
| 12 | “The CLI writes redacted JSON and HTML reports for CI or review.” | Terminology; see F-2-4. |
| 12 | “It is for API owners who need step-by-step evidence while adopting Arazzo.” | — |
| 10 | “The CLI has no account, telemetry, upload, or hosted service.” | Claim coverage; see F-2-1. |
| 13 | “The demo starts its own loopback API and needs no setup or network service.” | — |
| 13 | “It creates a unique temporary workspace and prints the generated report path.” | Claim coverage; see F-2-1. |
| 7 | “The same sample is visible at https://arazzo-proof-runner.sociobot.in/?demo=1.” | — |
| 6 | “See `.factory/demo.md` for its isolation contract.” | — |
| 7 | “Build the single `arazzo-proof` binary with Rust:” | — |
| 6 | “The current package version is `0.1.0`.” | — |
| 8 | “Run `cargo package` to validate the release package.” | — |
| 5 | “Factory automation owns registry publishing.” | Unlisted claim; F-2-5. |
| 10 | “Run one workflow and write `proof.json` plus a self-contained `report.html`:” | — |
| 9 | “The example above expects a compatible API at `127.0.0.1:4010`.” | — |
| 10 | “Use `arazzo-proof demo` for a first run without that API.” | — |
| 11 | “Print a JSON summary to stdout while writing the same proof bundle:” | — |
| 4 | “Compare two `proof.json` files:” | — |
| 9 | “This writes `comparison.json` and a self-contained `comparison.html`.” | — |
| 12 | “The report marks each changed status, body, output, or assertion at its workflow step.” | — |
| 7 | “Every run command requires an environment file.” | — |
| 8 | “The CLI accepts YAML and JSON environment files.” | — |
| 8 | “`baseUrl` replaces every OpenAPI server for the run.” | — |
| 7 | “`inputs` supplies workflow inputs such as `$inputs.petName`.” | — |
| 7 | “`values` supplies the runner extension `$env.tenant`.” | — |
| 4 | “`headers` applies request headers.” | — |
| 11 | “Sensitive headers and listed secrets become `[REDACTED]` in the proof bundle.” | — |
| 10 | “JSON Pointers under `redact` replace matching request and response values.” | — |
| 8 | “The CLI rejects unsupported features with an error.” | — |
| 2 | “These include:” | — |
| 8 | “The error names the affected step or source.” | — |
| 12 | “Invalid or unsupported input exits 2 before writing a partial proof bundle.” | — |
| 6 | “`--json` keeps the same exit codes.” | Claim coverage; see F-2-1. |
| 7 | “The CLI does not prompt for input.” | Claim coverage; see F-2-1. |
| 8 | “Claim commands are listed in `.factory/claims.json`.” | — |
| 8 | “The factory deploys `dist/site/` as a static site.” | — |
| 6 | “Build it with `npm run build:site`.” | — |
| 8 | “Do not deploy the CLI from this repository.” | — |
| 1 | “MIT.” | — |
| 2 | “See `LICENSE`.” | — |

README headings are meaningful out of context. It has no buttons. List items under Supported Arazzo, unsupported features, exit codes, and repository layout are noun fragments rather than sentences; each is under 14 words and uses consistent technical terms.

## Demo and sandbox exercise

| Check | Result |
| --- | --- |
| One-click first-screen action | Pass — “Try it with sample data.” |
| First demo screen | Pass — visible title, recorded real CLI output, `createCart`, realistic `crt_17`, and passing evidence are already in view. |
| Persistent banner | Pass — “Demo — sample data, nothing is saved,” Reset demo, and Start for real. |
| Reset | Pass — after injecting EUR, Reset restores USD, Pass, and `aria-pressed=false`. |
| Start for real | Pass — removes `?demo=1`, hides the banner, and opens `#install`. |
| Fresh storage | Pass — no cookies, localStorage, sessionStorage, or IndexedDB. |
| Existing real data | Pass in independent exercise — seeded `real:project`, `real:session`, and `real-project` IndexedDB data remained present after enter/change/reset. |
| Network | Pass — every web-demo request stayed on `arazzo-proof-runner.sociobot.in`; no analytics or remote assets were observed. |
| Offline | Pass — after service-worker activation, Demo and Privacy reloaded with HTTP 200 while the context was offline. |
| CLI temp-dir behavior | Pass — `cargo run -- demo` exited 0 from an empty temporary directory, left that directory empty, and wrote a three-step redacted JSON/HTML bundle under `/tmp/arazzo-proof-demo-…`. |

## Claims execution from a clean clone

Clean clone: `/tmp/arazzo-review2-clean-UkPhJt/repo`. Dependencies were installed with `npm ci`. Each manifest command was run independently and exactly as written.

| ID | Result |
| --- | --- |
| `bundled-demo` | Command pass; coverage gap in F-2-1. |
| `redaction-and-chaining` | Pass. |
| `operation-selection-and-parameters` | Command pass; coverage gap in F-2-1. |
| `comparison-report` | Pass. |
| `exit-codes-and-json` | Command pass; coverage gap in F-2-1. |
| `cli-help` | Pass. |
| `unsupported-errors` | Pass. |
| `single-binary-mit` | Pass. |
| `web-demo-isolation` | Command pass; coverage gap in F-2-1. |
| `offline-site` | Pass. |
| `site-privacy` | Command pass; coverage gap in F-2-1. |
| `routing-and-metadata` | Command pass; coverage gap in F-2-1. |

Result: 12 commands passed, 0 commands failed, but six claims are not fully asserted. `npm test` also passed: 5 Rust unit tests, 3 claim tests, 2 CLI tests, 3 workflow tests, 1 doctest, and 10 site tests. The build produced `dist/site/` with 3.37 kB JS and 14.00 kB CSS.

## Structure, accessibility, and live crawl

| Check | Result |
| --- | --- |
| Titles | Pass — root, Demo, Privacy, Terms, and 404 use product-authored route titles. |
| One `h1`, `lang`, `main` | Pass on all five routes. |
| Description, OG/Twitter, favicon, touch icon | Pass on all routes. |
| Canonical | Root, Demo, Privacy, Terms pass; 404 fails F-2-3. |
| Designed 404 | Pass — HTTP 404, concrete-and-moss treatment, Return home and Demo links. |
| Deep links, hash focus, Back | Pass — `#how`, Privacy, and browser Back restore the intended heading focus. |
| Link crawl | Pass — every root link returned 200 after redirects; GitHub returned 200. |
| Header/footer consistency | Fail F-2-2. |
| Privacy/Terms | Pass — present and linked from every footer. |
| Axe | Pass — zero violations across 20 live route/viewport/theme combinations. |
| Touch/overflow | Pass — no control below 44 × 44 px and no horizontal overflow at 390 px. |
| Console | Pass on all 200 routes; the expected failed-document message occurs only for the true 404 response. |
| `verify-url.sh` | Pass on root and Demo: title, `lang=en`, one `h1`, main, alt text, labels, and zero console errors. |
| Reduced motion | Pass by computed test and source inspection. |
| Identity | Pass — brutalist concrete, moss seams, near-square slabs, original project art, and restrained motion are distinct from a generic SaaS template. |

The standalone `@axe-core/cli` launcher could not start its Selenium Chrome in this container. This did not leave accessibility untested: the pinned Playwright Axe integration ran successfully across both required widths, both color schemes, and every route, and `npm test` repeated the same gate.

## Earlier-finding recheck

### Severity findings from review 1

| Earlier ID | Status now | Evidence |
| --- | --- | --- |
| B1 | Fixed | First-screen cold read passes at both widths. |
| B2 | Fixed | Web and CLI demo exercises pass. |
| B3 | **Half-fixed; blocking again via F-2-1** | Manifest and commands exist, but six broad claims have incomplete named assertions. |
| B4 | Fixed | Product-authored 404 returns HTTP 404. |
| M1 | Fixed | Mobile command scroll region is focusable; Axe passes. |
| M2 | Fixed for its original missing-skeleton/focus/touch defect | Policy routes have the skeleton, heading focus, and 44 px targets. The varying destination sets are a new finding, F-2-2. |
| M3 | Fixed for the named root/policy assets | Metadata and required assets are present. The 404 canonical omission is new F-2-3. |
| M4 | Fixed | Demo → How it works → boundaries → feature ledger order is live. |
| M5 | Fixed | Install block uses the real setup-free demo command. |
| M6 | Fixed | Nested complementary landmark is gone; Axe passes. |
| M7 | Fixed | External GitHub links are visibly or accessibly identified. |

### Earlier landing-claim findings

| ID | Status | Verification |
| --- | --- | --- |
| UC-L01 | Fixed | Offline Demo/Privacy reload passed. |
| UC-L02 | Fixed | Real three-step demo and output files verified. |
| UC-L03 | Fixed | No account UI/request observed. |
| UC-L04 | Fixed in live behavior | No telemetry request observed; strengthen the automated allowlist per F-2-1. |
| UC-L05 | Fixed | Single binary and MIT metadata tested. |
| UC-L06 | Fixed | “stable Rust” removed. |
| UC-L07 | Fixed | Explicit environment and no-prompt behavior exercised. |
| UC-L08 | Fixed | Three sample assertions visible and tested. |
| UC-L09 | Fixed | Demo `proof.json` parsed and inspected. |
| UC-L10 | Fixed | Demo HTML opened and content inspected. |
| UC-L11 | Fixed | Seeded authorization secret absent from both outputs. |
| UC-L12 | **Half-fixed; F-2-1** | 3.1 case is filtered out by the named operation test. |
| UC-L13 | **Half-fixed; F-2-1** | `operationId` case is filtered out by the named operation test. |
| UC-L14 | **Half-fixed; F-2-1** | Path case is filtered out by the named operation test. |
| UC-L15 | Fixed | Chained output enters the later request. |
| UC-L16 | Fixed | Six operators asserted in the named test. |
| UC-L17 | Fixed | Remote sources and refs rejected. |
| UC-L18 | Fixed | OAuth and callbacks rejected. |
| UC-L19 | Fixed | JSONPath criteria rejected. |
| UC-L20 | Fixed | Success/failure actions rejected. |
| UC-L21 | Fixed | Non-JSON body rejected. |
| UC-L22 | Fixed | Unsupported-feature errors are explicit. |
| UC-L23 | Fixed | Step/feature, exit 2, and no partial output are tested. |
| UC-L24 | Fixed | USD/EUR assertion change is visible and resettable. |
| UC-L25 | Fixed | Facts are present; behavior and license verified. |
| UC-L26 | Fixed | Promotional heading removed. |
| UC-L27 | Fixed | Unmeasurable size/usefulness claim removed. |
| UC-L28 | Fixed | Evolving-standard claim replaced with versioned scope. |

### Earlier README-claim findings

| ID | Status | Verification |
| --- | --- | --- |
| UC-R01 | Fixed | Opening is concrete and versioned. |
| UC-R02 | Fixed | Real bundle, chaining, and redaction verified. |
| UC-R03 | Fixed | Overbroad “stays on your machine” wording removed. |
| UC-R04 | Fixed in behavior | No account/telemetry/upload observed; automate the exact network allowlist per F-2-1. |
| UC-R05 | Fixed | One Rust binary asserted. |
| UC-R06 | Fixed | Version 0.1.0 asserted. |
| UC-R07 | Fixed | `cargo package` guidance is direct. |
| UC-R08 | Fixed | JSON and HTML outputs inspected. |
| UC-R09 | Fixed | JSON stdout parses. |
| UC-R10 | Fixed | Comparison uses two `proof.json` files. |
| UC-R11 | Fixed | Comparison HTML exists and is self-contained in source behavior. |
| UC-R12 | Fixed | Step-local assertion difference tested. |
| UC-R13 | Fixed | Missing environment exits 2. |
| UC-R14 | **Half-fixed; F-2-1** | The broad YAML/JSON support matrix is not in one named claim test. |
| UC-R15 | Fixed | Environment base URL overrides the unreachable schema server. |
| UC-R16 | Fixed | `$inputs.petName` exercised. |
| UC-R17 | Fixed | `$env.tenant` exercised. |
| UC-R18 | Fixed | Headers exercised. |
| UC-R19 | Fixed | Sensitive headers and secrets are redacted. |
| UC-R20 | Fixed | JSON Pointer body redaction exercised. |
| UC-R21 | **Half-fixed; F-2-1** | Feature coverage is split across a filtered-out test rather than the named claim test. |
| UC-R22 | Fixed | Every listed unsupported family rejected. |
| UC-R23 | **Half-fixed; F-2-1** | Run/compare 0/1/2 matrix is incomplete. |
| UC-R24 | **Half-fixed; F-2-1** | `--json` is not exercised across the promised matrix. |
| UC-R25 | Fixed | Test-suite marketing removed. |
| UC-R26 | Fixed in live behavior | No tracking/storage observed; strengthen allowlist per F-2-1. |
| UC-R27 | Fixed | MIT license asserted. |
| UC-R28 | Fixed | Reproducibility marketing removed from README opening. |

### Earlier policy-claim findings

| ID | Status | Verification |
| --- | --- | --- |
| UC-P01 | Fixed in live/source behavior | Requests use the selected loopback/base URL; no telemetry observed. |
| UC-P02 | **Half-fixed; F-2-1** | Same-origin analytics would not fail the named test. |
| UC-P03 | Fixed | Offline cache exercised live. |
| UC-P04 | Fixed | Browser removal instruction and named cache are present. |
| UC-P05 | Fixed | Base URL override tested. |
| UC-P06 | Fixed | Redaction and residual response-data warning are separate. |
| UC-P07 | Fixed | MIT/warranty wording and license verified. |
| UC-P08 | Fixed | Supported-feature link works; rejection matrix runs. |

### Earlier copy and unnumbered structure findings

Each earlier copy ID was rechecked in the live DOM and source. These are fixed: `CW-L01`, `CW-L02`, `CW-L03`, `CW-L04`, `CW-L05`, `CW-L06`, `CW-L07`, `CW-L08`, `CW-L09`, `CW-L10`, `CW-L11`, `CW-L12`, `CW-L13`, `CW-L14`, `CW-L15`, `CW-L16`, `CW-L17`, `CW-L18`, `CW-L19`, `CW-L20`, `CW-L21`, `CW-L22`, `CW-L23`, `CW-L24`, `CW-R01`, `CW-R02`, `CW-R03`, `CW-R04`, `CW-R05`, `CW-R06`, `CW-R07`, `CW-R08`, `CW-R09`, `CW-R10`, `CW-R11`, `CW-R12`, `CW-R13`, `CW-R14`, and `CW-R15`. The new output-term inconsistency is F-2-4 rather than a regression of those sentence-specific rewrites.

The earlier unnumbered metadata, route-focus, policy-skeleton, mobile, dark/light contrast, reduced-motion, and performance findings are fixed by live inspection and the clean `npm test`. The destination-set inconsistency and 404 canonical omission are newly isolated as F-2-2 and F-2-3.

## Missed leverage

No missed-leverage finding. The brief calls for import of Arazzo/OpenAPI/environment files, redacted JSON/HTML export, and comparison of two runs; all are present. Sync would conflict with the local-first evidence model. An AI step would make deterministic workflow proof less trustworthy and is not implied by the job, so no Sociobot gateway feature should be added.

## What would make this perfect

1. Make each broad claim command prove its full matrix, especially operation formats/selectors/parameters and run/compare exit codes.
2. Seed real browser data and allowlist exact requests in the automated isolation/privacy claims.
3. Use one header/footer destination set on every route.
4. Resolve and document the 404 canonical policy.
5. Standardize “proof bundle” and remove or register the publishing-ownership sentence.

When those items are complete and the whole checklist is rerun, there should be nothing left to report.
