# Adversarial first-read review 1 — Arazzo Proof Runner

Reviewed: 2026-08-28

Work order: `arazzo-proof-runner-review-1`

Candidate: `5716673d97177d3fc0626d08136015a238fe2b96`
Live site: <https://arazzo-proof-runner.sociobot.in/>

## Verdict: FAIL

The product has four blocking findings. A cold visitor cannot identify the intended user from the first screen, there is no one-click sandbox demo, the required claims manifest and tagged tests are absent, and unknown routes use the hosting provider's generic 404. The passing build and manually verified privacy behavior do not remove those blockers.

## Cold first read, before scrolling

Fresh Chromium contexts were opened at 390 × 844 and 1440 × 900. No prior storage or cookies were present.

| Viewport | What does this do? | For whom? | What should I click first? |
| --- | --- | --- | --- |
| 390 px | My inference: it runs some part of an Arazzo workflow and produces a redacted “proof.” The screen does not explain what Arazzo is or what the proof contains. | Cannot answer from the first screen. “API owners” appears only in the README. | The visual primary action says “Install the runner.” There is no try action. |
| Desktop | Same inference; the concrete workflow image adds sequence, not product meaning. | Cannot answer from the first screen. | “Install the runner,” with “Inspect a proof” as the secondary action. |

The blocking text is: “Execute a practical Arazzo subset, trace every substitution and assertion, then carry one redacted report into CI or review.” It names no user and depends on the unexplained terms “Arazzo subset,” “substitution,” “assertion,” “redacted report,” and “CI.”

Required first-screen replacement:

- Headline: “Run Arazzo workflows and save proof”
- Audience sentence: “For API owners testing multi-step workflows, it records redacted requests, responses, and assertion results.”
- Primary action: “Try it with sample data”
- Adjacent explanation: “Runs a bundled checkout workflow and opens its report.”
- Facts: “Runs locally.” “No account or telemetry.” “Free and MIT licensed.”

## Findings, ordered by severity

### BLOCKING B1 — the first screen does not identify its user

Quote: “Run the workflow. Keep the proof.” / “Execute a practical Arazzo subset…”

Why this loses a first-time visitor: “the workflow” and “the proof” require context the visitor has not received. Nothing above the fold says this is a CLI for API owners. A visitor can identify the requested click, but cannot answer who the product is for.

Concrete fix: use the replacement first-screen copy above. Keep “Arazzo” in the headline so qualified visitors can identify the format, and name “API owners” in the supporting sentence.

### BLOCKING B2 — there is no one-click demo or isolated demo command

Quotes and observations:

- No “Try it with sample data” action exists on the landing page.
- `/demo` returns HTTP 404 with the title “Azure Static Web Apps - 404: Not found.”
- `/?demo=1` renders the normal landing page.
- `arazzo-proof demo` exits 2 with `error: unrecognized subcommand 'demo'` when run in a fresh temp directory.
- “Inspect a proof” only scrolls to a fixed specimen. “Inject changed response” mutates that specimen in the DOM.
- There is no “Demo — sample data, nothing is saved” banner, “Reset demo,” or “Start for real.”
- `.factory/demo.md` is absent.

Why this loses a first-time visitor: the only primary path is installation. The specimen demonstrates one visual state but does not run the product, accept sample input, create output, or establish isolation. There is no way to evaluate the core job within 30 seconds.

Concrete fix: ship `arazzo-proof demo` with the example workflow and a local fixture server. It must run in a temp directory, write a real proof bundle, print its location, and require no external setup. Make the first-screen action run or visibly replay that exact command. Add the persistent demo banner and controls if the web specimen becomes interactive, and document the entry point and namespace in `.factory/demo.md`.

### BLOCKING B3 — claims are not registered or claim-tested

Quote: `.factory/claims.json` is missing, and `rg '@claim:'` finds no tagged tests.

Why this misleads a visitor: the landing page and README make claims about redaction, local-only behavior, supported inputs, output files, exit codes, offline use, and privacy. None has the required claim-to-test mapping, so a release verifier cannot run the advertised behavior from the prescribed sandbox.

Concrete fix: add `.factory/claims.json`; give every entry in the claim audit below one unique ID and one test tagged `@claim:<id>`. Tests must invoke the bundled demo from a clean temp directory. Privacy tests must inspect serialized outputs and intercept all network traffic. Remove any claim that cannot be tested.

The manifest listed zero test commands, so there were no listed claim tests to run. As a diagnostic only, `npm test` was run from a clean clone and passed: 5 Rust unit tests, 2 CLI tests, 3 workflow tests, 1 doctest, and 4 site tests. The build produced 2.08 kB JS and 10.85 kB CSS. These untagged tests do not satisfy the claims contract.

### BLOCKING B4 — routing falls through to a generic provider 404

Quote: `/not-a-real-route` and `/demo` show “Azure Static Web Apps - 404: Not found,” with an Azure favicon, no product heading, and no way back.

Why this loses a first-time visitor: the visitor leaves the product's visual and navigation system, and the missing demo route looks like a broken deployment rather than an intentional boundary.

Concrete fix: add a product-designed `/404.html` using the concrete-and-moss identity, with one `h1`, the standard header/footer, and links to Home and Demo. Configure the static host to serve it for unknown paths while retaining a real 404 status.

### MAJOR M1 — the live 390 px page has a serious keyboard accessibility violation

Quote: Axe reports `scrollable-region-focusable` on `#install-command`: “Scrollable region must have keyboard access.”

Why this blocks use: the long command becomes horizontally scrollable on a phone but the `<pre>` cannot receive focus, so keyboard users cannot reach all of it. The existing site test uses a desktop viewport for Axe and misses this mobile-only failure.

Concrete fix: make the scroll region keyboard-focusable (for example, `tabindex="0"` with an accessible label and visible focus style), then run Axe at 390 px. Add that viewport to the existing accessibility test.

### MAJOR M2 — policy routes do not use the standard site skeleton or route-focus behavior

Quotes and observations:

- `/privacy/` and `/terms/` have no skip link, primary navigation, footer, product one-liner, or build/version ID.
- Their only link, “← Arazzo Proof Runner,” is 19 px high at 390 px instead of the required 44 px touch target.
- Following the Privacy link leaves focus on `<body>`; the route's `<h1>` is not focused or announced.
- Hash navigation and browser Back also leave focus on `<body>`.

Why this loses a first-time visitor: navigation and orientation change between routes, while keyboard and screen-reader users receive no route-change focus cue.

Concrete fix: render one shared header/footer/skip-link skeleton on every route. Give route headings programmatic focus (`tabindex="-1"`) after navigation and announce them. Preserve meaningful focus and scroll state on Back/Forward. Make every route link at least 44 × 44 CSS pixels.

### MAJOR M3 — required metadata is incomplete

Quote: the home page has no Open Graph title/description/image, Twitter card, or 180 px apple-touch icon. Privacy and Terms have no meta description, canonical URL, favicon, Open Graph data, Twitter card, or apple-touch icon.

Why this misleads a visitor: shared links have no product-authored preview and policy routes do not declare their canonical identity. The 1200 × 630 social image required by the site contract does not exist.

Concrete fix: add route-specific descriptions/canonicals and complete OG/Twitter tags. Export a 1200 × 630 image from the original concrete-and-moss art and a 180 px touch icon. Keep the existing route title pattern, which is correct.

### MAJOR M4 — the landing structure omits the required usage path

Quote: the page moves from “A runner you can audit” to a fixed “Proof specimen” and then “Deliberate subset.” There is no “How it works” sequence in three steps and no explicit plain-language “What it does not do / privacy” section.

Why this loses a first-time visitor: the page shows output before explaining the input → run → report workflow. Limitations are presented as a feature ledger, while the privacy consequences of calling a selected API are left on another route.

Concrete fix: after the live demo, add three concrete steps: “Choose an Arazzo file,” “Run against an environment,” and “Open the redacted report.” Follow with a short limits/privacy section. Keep the existing supported/unsupported detail below it.

### MAJOR M5 — the advertised install command is not a runnable first use

Quote: the copied block invokes `checkout.arazzo.yaml` and `environments/ci.yaml`, neither of which is supplied by the page or present under those names in the repository.

Why this loses a first-time visitor: installation may succeed, but the next copied command immediately depends on undisclosed files and a reachable API. It looks like a runnable example.

Concrete fix: separate the install command from usage placeholders, or replace the second line with the bundled `arazzo-proof demo` command. Label placeholders explicitly when a real target is required.

### MODERATE M6 — the root route has an additional landmark error

Quote: Axe reports `landmark-complementary-is-top-level` for `<aside class="artifact-note">` nested inside `<main>`.

Why this matters: the complementary landmark is not top-level, which can produce a confusing landmark list.

Concrete fix: use a non-landmark `<div>` for this note or move a genuinely complementary `<aside>` outside `<main>`. Add all Axe impacts to the browser gate, not only serious/critical ones.

### MODERATE M7 — external links are not identified as external

Quote: “Source” and “GitHub” open `github.com` but do not say that they leave the site.

Why this matters: the destination change is not apparent from the link name alone.

Concrete fix: label them “Source on GitHub (external)” and “GitHub (external),” visibly or with equivalent accessible text.

## Demo and sandbox evidence

| Check | Result |
| --- | --- |
| One-click sample action above the fold | **Fail** — absent |
| `/demo` | **Fail** — generic HTTP 404 |
| `?demo=1` | **Fail** — normal landing page |
| `arazzo-proof demo` in a fresh temp directory | **Fail** — unknown subcommand, exit 2 |
| Realistic data visible after “Inspect a proof” | Partial — a fixed three-step cart specimen is visible, but it is not a runner execution |
| Persistent demo banner | **Fail** — absent |
| Reset demo | **Fail** — absent |
| Start for real | **Fail** — absent |
| Separate demo storage namespace | **Fail / unverifiable** — there is no demo mode; local/session storage and IndexedDB remained empty before and after the specimen toggle |
| Real data untouched | **Unverifiable** — no real/demo data boundary exists |
| `.factory/demo.md` | **Fail** — absent |

## Claim audit — every item is an unlisted-claim finding

Because the manifest is absent, every row below is a separate finding. Repeated claims in different locations are listed separately because visitors can rely on each occurrence.

### Landing page claims

| ID | Exact claim | Concrete test to add |
| --- | --- | --- |
| UC-L01 | “The docs and proof specimen still work locally.” | Load Demo once, go offline, reload docs and specimen, and assert complete content. |
| UC-L02 | “Execute a practical Arazzo subset, trace every substitution and assertion, then carry one redacted report into CI or review.” | Run the bundled multi-step demo and assert substitutions, assertions, and both report formats. |
| UC-L03 | “No account” | Run the full demo while asserting no authentication UI/request occurs. |
| UC-L04 | “No telemetry” | Intercept the complete demo flow and assert only the explicit API fixture plus same-origin site requests. |
| UC-L05 | “One binary” | Package/install and assert the distribution exposes one executable. |
| UC-L06 | “Build with stable Rust.” | Build with the repository's declared stable toolchain in a clean container. |
| UC-L07 | “The CLI requires an environment file, never prompts in CI, and never uploads a run.” | Assert missing `--env` exits 2 without input reads; intercept demo traffic and confirm only the selected API target is called. |
| UC-L08 | “All specimen assertions pass.” | Assert the initial sample contains the stated passing assertions, not only the visible label. |
| UC-L09 | “`proof.json` for machines.” | Parse emitted `proof.json` from the demo and validate its schema and record counts. |
| UC-L10 | “`report.html` for review.” | Open emitted `report.html` offline and assert the sample steps and assertions are present. |
| UC-L11 | “Authorization and configured secrets are replaced before either is written.” | Seed unique header/body/URL secrets and assert they occur in neither output file. |
| UC-L12 | “Local OpenAPI 3.0 / 3.1 sources” | Run one local 3.0 and one local 3.1 fixture. |
| UC-L13 | “Operation IDs and local operation paths” | Run fixtures selecting each operation form. |
| UC-L14 | “Path, query, header, cookie parameters” | Assert all four substitutions reach the fixture server. |
| UC-L15 | “JSON payloads and chained outputs” | Assert a value from step 1 enters the JSON request for step 2. |
| UC-L16 | “Six comparison operators” | Exercise `==`, `!=`, `>`, `>=`, `<`, and `<=` with pass/fail cases. |
| UC-L17 | “Remote sources and external references” under “Fails clearly” | Assert each construct exits 2 with its feature named and no partial proof. |
| UC-L18 | “OAuth flows and callbacks” under “Fails clearly” | Assert each construct exits 2 with its feature named and no partial proof. |
| UC-L19 | “JSONPath criteria” under “Fails clearly” | Assert the construct exits 2 with its feature named and no partial proof. |
| UC-L20 | “Success and failure actions” under “Fails clearly” | Assert each construct exits 2 with its feature named and no partial proof. |
| UC-L21 | “Non-JSON request bodies” under “Fails clearly” | Assert the construct exits 2 with its feature named and no partial proof. |
| UC-L22 | “Unsupported is an error, not a guess.” | Run every advertised unsupported fixture and assert explicit failure. |
| UC-L23 | “The runner names the step and feature, exits 2, and writes no partial proof.” | Assert message, exit code, and absent output for each unsupported fixture. |
| UC-L24 | “The quoteCart assertion now fails: expected USD, received EUR.” | Change the fixture response and assert the generated proof and comparison expose that exact mismatch. |
| UC-L25 | “Free, local, MIT licensed.” | Assert the license file/package metadata and that the demo requires no account/payment or non-target service. |
| UC-L26 | “A runner you can audit” | Define the observable audit outcome and test it, or replace this with the concrete “Install the CLI.” |
| UC-L27 | “Small enough to understand. Useful enough to ship.” | Remove these unmeasurable claims and use “Supported Arazzo features and explicit limits.” |
| UC-L28 | “Built for an evolving open standard.” | Remove this untestable promotional claim and name the supported specification version. |

### README claims

| ID | Exact claim | Concrete test to add |
| --- | --- | --- |
| UC-R01 | “`arazzo-proof` is a small, local runner for an auditable subset of the OpenAPI Arazzo specification.” | Run the documented subset from a temp directory and validate the generated evidence. |
| UC-R02 | “It executes a multi-step workflow against an explicitly selected environment and writes a stable, redacted proof bundle for CI and integration review.” | Run twice against a deterministic fixture; compare output bytes and scan for seeded secrets. |
| UC-R03 | “Runs stay on your machine.” | Intercept all outbound connections and allow only the environment's configured target. |
| UC-R04 | “There is no telemetry, account, upload, or cloud dependency.” | Run the demo with network interception and assert no auth, telemetry, upload, or service traffic. |
| UC-R05 | “Build the single binary with stable Rust:” | Build on the declared stable toolchain and assert one executable artifact. |
| UC-R06 | “The package starts at `0.1.0`.” | Assert Cargo/package versions equal `0.1.0`. |
| UC-R07 | “Factory release automation owns registry publishing; maintainers can validate the release artifact with `cargo package`.” | Run `cargo package` in a clean clone and assert success. |
| UC-R08 | “Run one workflow and write `proof.json` plus a self-contained `report.html`:” | Run the documented command fixture and inspect both outputs without network assets. |
| UC-R09 | “Print a machine-readable summary to stdout (evidence still goes to `--out`):” | Parse stdout as JSON and assert both output files exist. |
| UC-R10 | “Compare two proof artifacts:” | Compare deterministic baseline/current demo outputs and assert the comparison files. |
| UC-R11 | “This writes `comparison.json` and a one-file `comparison.html`.” | Assert both files exist and the HTML has no external dependencies. |
| UC-R12 | “A changed status, body, output, or assertion is called out at the step where it changed.” | Mutate each field independently and assert the step-local diff. |
| UC-R13 | “An environment file is always required; there is no implicit production target.” | Omit `--env` and assert exit 2 before any request. |
| UC-R14 | “YAML and JSON are accepted:” | Run equivalent YAML and JSON environment fixtures. |
| UC-R15 | “`baseUrl` overrides the OpenAPI server for every operation.” | Use distinct schema and environment servers and assert every request reaches only the override. |
| UC-R16 | “`inputs` supplies workflow inputs referenced as `$inputs.petName`.” | Assert the fixture receives the configured value. |
| UC-R17 | “`values` supplies the runner extension `$env.tenant`.” | Assert the fixture receives the configured extension value. |
| UC-R18 | “`headers` applies request headers.” | Assert configured headers reach the fixture server. |
| UC-R19 | “`Authorization`, cookies, API-key-like headers, and every value listed in `secrets` are replaced with `[REDACTED]` before evidence is serialized.” | Seed every category and scan every JSON/HTML output for the raw values. |
| UC-R20 | “`redact` contains JSON Pointers removed from captured request and response bodies.” | Configure request/response pointers and assert replacement in all outputs. |
| UC-R21 | The ten bullets under “Supported Arazzo 1.0.x subset.” | Add one fixture/assertion for every advertised syntax family. |
| UC-R22 | “Unsupported features fail clearly instead of being guessed…” | Add one rejecting fixture per listed unsupported feature and assert exit 2/no partial proof. |
| UC-R23 | The three exit-code definitions (`0`, `1`, `2`). | Exercise success, assertion/change, and invalid/unsupported/transport paths. |
| UC-R24 | “`--json` never changes these exit-code semantics and the CLI never prompts, so it is safe in CI.” | Repeat the exit-code matrix with `--json`, closed stdin, and a timeout. |
| UC-R25 | “The integration suite runs three representative workflows against an in-process HTTP server and checks chaining, parameter substitution, redaction, and a visible changed assertion.” | Meta-test the named scenarios or replace this copy with a link to the test files. |
| UC-R26 | “The landing/docs site is plain HTML/CSS built by Vite, with no runtime dependencies or analytics.” | Inspect the production dependency graph and intercept all site requests. |
| UC-R27 | “MIT.” | Assert the shipped `LICENSE` is MIT and package metadata points to it. |
| UC-R28 | “It is for API owners adopting Arazzo who need reproducible evidence—not another general-purpose API client.” | Run the same bundled workflow twice and assert identical evidence, or remove “reproducible.” |

### Policy-route claims

| ID | Exact claim | Concrete test to add |
| --- | --- | --- |
| UC-P01 | “The CLI runs locally and does not send telemetry, analytics, workflow files, environments, requests, responses, or proof artifacts to Arazzo Proof Runner or Sociobot.” | Intercept all network connections during the CLI demo and assert only the configured fixture target is contacted. |
| UC-P02 | “The documentation site has no analytics, advertising, accounts, cookies, or remote scripts.” | Load every route in a fresh context; inspect requests, cookies, storage, scripts, and account UI. |
| UC-P03 | “A service worker caches public site files on your device for offline reading.” | Load once, switch offline, and open every route named in the cache contract. |
| UC-P04 | “Remove the site data in your browser settings to clear that cache.” | Clear site data through browser context APIs and assert the named cache is absent. |
| UC-P05 | “Your API requests go only to the base URL you explicitly place in an environment file.” | Use a monitored fixture target and assert no request reaches the OpenAPI server or any other origin. |
| UC-P06 | “Configured secrets and sensitive headers are redacted, but your response bodies may contain other private data.” | Seed configured and unconfigured markers; assert only the configured/sensitive values are replaced in every output. |
| UC-P07 | “Arazzo Proof Runner is free, open-source software provided under the MIT License, without warranty.” | Assert the shipped license and package metadata; remove “free” unless release distribution is also checked. |
| UC-P08 | “The evolving Arazzo subset is documented in the project README; unsupported features fail explicitly.” | Compare the advertised unsupported list with rejecting fixtures that name each feature and produce no partial proof. |

## Copy audit

Counting method: whitespace-delimited tokens; code identifiers and arrow glyphs count as one word. Headings/actions are audited separately. No banned plain-words term appears verbatim. The hard cap is exceeded twice in the README.

### Landing-page sentences

| # | Words | Exact sentence | Flag and proposed rewrite |
| ---: | ---: | --- | --- |
| 1 | 1 | “Offline.” | — |
| 2 | 8 | “The docs and proof specimen still work locally.” | CW-L01: “proof specimen” and “locally” are unclear here. Use “The guide and sample report still work offline.” |
| 3 | 3 | “Run the workflow.” | CW-L02: headline assumes a known workflow. Combine as “Run Arazzo workflows and save proof.” |
| 4 | 3 | “Keep the proof.” | CW-L02: “proof” is undefined. Use the combined headline above. |
| 5 | 19 | “Execute a practical Arazzo subset, trace every substitution and assertion, then carry one redacted report into CI or review.” | CW-L03: audience missing; “practical” is unmeasured; five technical terms arrive at once. Use the audience sentence in B1. |
| 6 | 5 | “Inputs → requests → assertions.” | — for the target technical audience once the audience is named. |
| 7 | 4 | “Evidence set in concrete.” | CW-L04: metaphor does not describe an outcome. Use “A report links each input, request, and assertion.” |
| 8 | 4 | “Build with stable Rust.” | — |
| 9 | 15 | “The CLI requires an environment file, never prompts in CI, and never uploads a run.” | CW-L05: three ideas in one sentence. Use “Every run requires an environment file. The command never prompts or uploads a run.” |
| 10 | 4 | “All specimen assertions pass.” | — as a status, but it needs a registered claim test. |
| 11 | 3 | “`proof.json` for machines.” | — |
| 12 | 3 | “`report.html` for review.” | — |
| 13 | 10 | “Authorization and configured secrets are replaced before either is written.” | CW-L06: say how. Use “The runner replaces authorization headers and configured secrets with `[REDACTED]` before writing either file.” |
| 14 | 4 | “Small enough to understand.” | CW-L07: unmeasured marketing claim. Replace the paired heading with “Supported Arazzo features and explicit limits.” |
| 15 | 4 | “Useful enough to ship.” | CW-L07: unmeasured marketing claim. Use the replacement above. |
| 16 | 7 | “Unsupported is an error, not a guess.” | CW-L08: “unsupported” lacks a noun. Use “Unsupported Arazzo features return an error.” |
| 17 | 14 | “The runner names the step and feature, exits 2, and writes no partial proof.” | — |
| 18 | 7 | “Replace pasted snippets with one stable artifact.” | CW-L09: “stable artifact” is vague and untested. Use “Share one HTML report instead of pasted response snippets.” |
| 19 | 4 | “Free, local, MIT licensed.” | — after claim registration. |
| 20 | 6 | “Built for an evolving open standard.” | CW-L10: promotional and nonspecific. Use “Supports the documented Arazzo 1.0.x subset.” |
| 21 | 2 | “Command copied.” | — |
| 22 | 3 | “Copy was blocked.” | CW-L11: cause is missing. Use “The browser blocked clipboard access.” |
| 23 | 4 | “Select the command manually.” | — |
| 24 | 9 | “The quoteCart assertion now fails: expected USD, received EUR.” | — |

### Landing headings, actions, and fragments

| Copy | Words | Finding and proposed rewrite |
| --- | ---: | --- |
| “Local Arazzo evidence / v0.1” | 5 | CW-L12: version-like eyebrow does not identify the user. Use “CLI for API workflow reviews.” |
| “Install the runner” | 3 | CW-L13: valid verb, wrong primary task for evaluation. Make “Try it with sample data” primary and “View install command” secondary. |
| “Inspect a proof” | 3 | CW-L14: “proof” and whether it is a sample are unclear. Use “View the sample report.” |
| “No account” / “No telemetry” / “One binary” | 2 / 2 / 2 | CW-L15: these omit offline and price, which the required three facts call for. Use the three facts in B1. |
| “01 / Start local” | 4 | CW-L16: “local” can mean machine or network. Use “01 / Install the local CLI.” |
| “A runner you can audit” | 5 | CW-L17: “audit” does not state the job. Use “Install the CLI.” |
| “Copy command” | 2 | Result-naming verb; no flag. |
| “02 / Proof specimen” | 4 | CW-L18: “specimen” is jargon. Use “02 / Sample report.” |
| “A changed response has a place to land” | 8 | CW-L19: metaphor obscures the result. Use “See exactly where a response changed.” |
| “Inject changed response” / “Restore baseline response” | 3 / 3 | Result-naming verbs; no flag. |
| “Two durable files” | 3 | CW-L20: “durable” is unmeasured. Use “Two output files.” |
| “03 / Deliberate subset” | 4 | CW-L21: adjective is promotional. Use “03 / Supported features and limits.” |
| “Runs today” / “Fails clearly” | 2 / 2 | CW-L22: headings make little sense out of context. Use “Supported features” / “Unsupported features.” |
| “Make the run reviewable” | 4 | CW-L23: abstract. Use “Share the report.” |
| “Run your first proof” | 4 | CW-L24: the link only scrolls to installation and does not run anything. Use “View install command,” or link “Try it with sample data” to a real demo. |

### README sentences

| # | Words | Exact sentence | Flag and proposed rewrite |
| ---: | ---: | --- | --- |
| 1 | 15 | “`arazzo-proof` is a small, local runner for an auditable subset of the OpenAPI Arazzo specification.” | CW-R01: “small” is unmeasured and “auditable subset” is abstract. Use “`arazzo-proof` runs the documented Arazzo 1.0.x features on your machine.” |
| 2 | 22 | “It executes a multi-step workflow against an explicitly selected environment and writes a stable, redacted proof bundle for CI and integration review.” | CW-R02: at the cap and dense; “stable” is an unlisted claim. Split: “Choose an environment and run a multi-step workflow. The CLI writes redacted JSON and HTML reports for CI or review.” |
| 3 | 15 | “It is for API owners adopting Arazzo who need reproducible evidence—not another general-purpose API client.” | CW-R03: negative positioning adds work. Use “It is for API owners who need reproducible evidence while adopting Arazzo.” |
| 4 | 5 | “Runs stay on your machine.” | — after claim registration. |
| 5 | 9 | “There is no telemetry, account, upload, or cloud dependency.” | — after claim registration. |
| 6 | 7 | “Build the single binary with stable Rust:” | — |
| 7 | 5 | “The package starts at `0.1.0`.” | CW-R04: “starts at” is indirect. Use “The current package version is `0.1.0`.” |
| 8 | 15 | “Factory release automation owns registry publishing; maintainers can validate the release artifact with `cargo package`.” | CW-R05: internal factory terminology is not user documentation. Use “Run `cargo package` to validate the release package.” |
| 9 | 10 | “Run one workflow and write `proof.json` plus a self-contained `report.html`:” | — after claim registration. |
| 10 | 11 | “Print a machine-readable summary to stdout (evidence still goes to `--out`):” | — for the technical audience. |
| 11 | 4 | “Compare two proof artifacts:” | CW-R06: output is elsewhere called a bundle and report. Use “Compare two `proof.json` files:” |
| 12 | 7 | “This writes `comparison.json` and a one-file `comparison.html`.” | CW-R07: “one-file” conflicts with “self-contained.” Use “This writes `comparison.json` and a self-contained `comparison.html`.” |
| 13 | 16 | “A changed status, body, output, or assertion is called out at the step where it changed.” | CW-R08: passive. Use “The report marks each changed status, body, output, or assertion at its workflow step.” |
| 14 | 12 | “An environment file is always required; there is no implicit production target.” | — |
| 15 | 5 | “YAML and JSON are accepted:” | CW-R09: accepted by what is implicit. Use “The CLI accepts YAML and JSON environment files:” |
| 16 | 8 | “`baseUrl` overrides the OpenAPI server for every operation.” | — |
| 17 | 7 | “`inputs` supplies workflow inputs referenced as `$inputs.petName`.” | — |
| 18 | 6 | “`values` supplies the runner extension `$env.tenant`.” | — |
| 19 | 4 | “`headers` applies request headers.” | — |
| 20 | 18 | “`Authorization`, cookies, API-key-like headers, and every value listed in `secrets` are replaced with `[REDACTED]` before evidence is serialized.” | CW-R10: “serialized” is avoidable jargon. Use “…before the CLI writes the proof files.” |
| 21 | 11 | “`redact` contains JSON Pointers removed from captured request and response bodies.” | CW-R11: the property contains pointers, not removed data. Use “List JSON Pointers under `redact`; the CLI replaces those body values in saved evidence.” |
| 22 | 27 | “Unsupported features fail clearly instead of being guessed: remote source URLs, callbacks/webhooks, external `$ref`, OAuth flows, non-JSON request bodies, JSONPath criteria, retry policies, and success actions/failure actions.” | CW-R12 **>22 words**. Use “The CLI rejects unsupported features with an error. These include:” followed by the existing list. |
| 23 | 17 | “`--json` never changes these exit-code semantics and the CLI never prompts, so it is safe in CI.” | CW-R13: three ideas and “safe” is broad. Use “`--json` keeps the same exit codes. The CLI does not prompt for input.” |
| 24 | 23 | “The integration suite runs three representative workflows against an in-process HTTP server and checks chaining, parameter substitution, redaction, and a visible changed assertion.” | CW-R14 **>22 words**. Use “The integration suite runs three workflows against a local test server. It checks chaining, parameter substitution, redaction, and changed assertions.” |
| 25 | 15 | “The landing/docs site is plain HTML/CSS built by Vite, with no runtime dependencies or analytics.” | CW-R15: internal implementation detail is misplaced in user docs. Move it to contributor documentation, or use “The documentation site loads no analytics.” |
| 26 | 1 | “MIT.” | — |
| 27 | 2 | “See [LICENSE](LICENSE).” | — |

README headings are understandable for a technical document except “Usage,” which is too broad to help out of context; use “Run and compare workflows.” There are no README buttons to audit.

### Terminology consistency

| Concept | Terms currently used | Required single usage |
| --- | --- | --- |
| Complete run output | proof, proof bundle, proof artifact, stable artifact, evidence | “proof bundle” |
| Human-readable file | proof, report, specimen, artifact | “HTML report”; “sample report” only for the landing example |
| Executable | runner, CLI, binary | Introduce “CLI (`arazzo-proof`)”; use “CLI” afterward |
| Feature boundary | practical subset, deliberate subset, auditable subset, supported subset | “documented Arazzo 1.0.x subset” |

## Structure and behavior checklist

| Check | Result |
| --- | --- |
| Root title pattern and length | Pass — “Arazzo Proof Runner — run the workflow, keep the proof,” under 60 characters |
| Route-specific Privacy/Terms titles | Pass |
| One `h1`, `lang`, and `main` on valid routes | Pass |
| Meta description and canonical | Partial — root only |
| OG/Twitter/1200 × 630 image/apple-touch icon | Fail |
| SVG favicon | Root pass; policy routes fail |
| Designed 404 | **Blocking fail** |
| Deep links | Privacy and Terms load; hash sections exist |
| Back/forward focus restoration | Fail — focus remains on `<body>` |
| Consistent header/footer/skip link | Fail on Privacy and Terms |
| Privacy/Terms links | Pass on root; both return 200 |
| Dead-link crawl | Pass for all advertised root links and hash targets; GitHub returned 200 |
| External-link disclosure | Fail |
| Standard landing order | Partial — no real demo, three-step explanation, or explicit limits/privacy section |
| Distinct identity | Pass — concrete/moss palette, original slab art, near-square controls, and workflow seam do not resemble a generic gradient/card template |
| 390 px horizontal page overflow | Pass |
| Touch targets | Root mostly passes; policy home links fail at 19 px high |
| Serious/critical accessibility | Fail — one serious mobile Axe violation |
| Console/page errors on root load | Pass |
| First-load JS budget | Pass — 2.08 kB built JS |
| Reduced-motion rule present | Pass by source inspection |

## Privacy and offline exercise

- A fresh browser observed only same-origin requests for the page, JS, CSS, hero image, service worker shell, and policy page. No cookies, local storage, session storage, IndexedDB, remote scripts, fonts, or analytics were observed.
- After first load and service-worker activation, the home page reloaded offline with HTTP 200, the expected `h1`, and the offline banner. `/privacy/` also opened offline.
- These observations support the current browser-shell privacy/offline behavior, but the claims remain unlisted and there is no demo sandbox in which to verify the whole product flow.
- CLI redaction behavior passed the untagged clean-clone tests. Target-only networking was not accepted as claim evidence because no prescribed demo command or claim test exists.

## Required acceptance changes

1. Supply the real one-command CLI demo and connect the first-screen sample action to it.
2. Add the claims manifest and tagged sandbox tests for every retained claim.
3. Rewrite the first screen to name API owners and the concrete output.
4. Ship a designed 404 and consistent route skeleton/metadata/focus behavior.
5. Fix the 390 px keyboard-scroll defect and extend Axe coverage to mobile.
6. Apply the copy rewrites, especially the two README sentences over 22 words and the inconsistent output terminology.
