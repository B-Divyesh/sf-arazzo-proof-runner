# Perfection-loop polish 1

Candidate repaired: `5716673d97177d3fc0626d08136015a238fe2b96`  
Review consumed: `52a8fc65ea4d730830feceb09dd72b8fe9493d72` / `.factory/review-1.md`  
There were no earlier `.factory/review-*.md` or `.factory/polish-*.md` files in the repository.

Evidence screenshots are under `.factory/evidence/`. All live rows were rechecked at <https://arazzo-proof-runner.sociobot.in/> and its named routes after deployment.

## Severity findings

| ID | Change made | Evidence |
| --- | --- | --- |
| B1 | Replaced the first screen with the required six-word job headline, API-owner sentence, primary demo action, adjacent outcome, and three facts. | `landing and README sentences use plain words`; `home-mobile.png`; live `/` |
| B2 | Added real `arazzo-proof demo`, bundled checkout/OpenAPI fixtures, an ephemeral loopback server and temporary workspace, `?demo=1`, `/demo` redirect, persistent banner, reset, and exit. | `claim_bundled_demo_is_isolated_and_writes_real_proof`; `@claim:web-demo-isolation`; `demo-mobile.png`; live `/?demo=1` and `/demo` |
| B3 | Added `.factory/claims.json` and one uniquely tagged observable test for every retained claim family. Removed untestable promotional claims. | Every command in `.factory/claims.json` passed from `/tmp/arazzo-final-HRSCqu/repo` |
| B4 | Added the concrete-and-moss `404.html` and Azure 404 response override with status 404. | `@claim:routing-and-metadata`; `404-mobile.png`; live `/not-a-real-route` |
| M1 | Made the horizontally scrolling command block focusable and labelled; Axe now runs at 390 px. | `all routes pass desktop and mobile Axe with no console errors`; live `/?demo=1` Axe |
| M2 | Rebuilt Privacy and Terms with the shared header, navigation, skip link, footer, one-liner, build ID, 44 px targets, heading focus, announcements, and history restoration. | `hash and browser history navigation restore heading focus`; `privacy-mobile.png`; `terms-mobile.png`; live `/privacy/`, `/terms/` |
| M3 | Added route descriptions, canonicals, OG/Twitter metadata, a 1200×630 social image, SVG favicon, and 180×180 touch icon. | `@claim:routing-and-metadata`; asset dimensions; live head inspection |
| M4 | Added “How it works” with three ordered steps, then an explicit privacy/limits section before the feature ledger. | `home-desktop.png`; live `/#how` and `/#limits` |
| M5 | Replaced the misleading usage block with a valid install command and the setup-free `arazzo-proof demo`. Labelled the networked README example. | `claim_bundled_demo_is_isolated_and_writes_real_proof`; live `/#install` |
| M6 | Replaced the nested `aside` with a non-landmark `div`; browser tests fail on every Axe impact. | `all routes pass desktop and mobile Axe with no console errors` |
| M7 | Changed external labels to “Source on GitHub” / “GitHub” with accessible “(external)” text. | route DOM crawl in `@claim:routing-and-metadata`; live header/footer |

## Landing claim findings

| ID | Change made | Evidence |
| --- | --- | --- |
| UC-L01 | Reworded the offline notice and registered the narrower guide/demo/privacy cache claim. | `@claim:offline-site` |
| UC-L02 | Removed the dense broad claim; the new copy names requests, responses, assertions, and the concrete outputs. | `@claim:bundled-demo`; `@claim:redaction-and-chaining` |
| UC-L03 | Retained “No account” and registered it with the complete web demo flow. | `@claim:web-demo-isolation`; `@claim:site-privacy` |
| UC-L04 | Retained “No telemetry” and intercept-tested every public page. | `@claim:site-privacy` |
| UC-L05 | Replaced “One binary” on the first screen with the required price fact; kept and tested single-binary packaging in docs. | `claim_package_declares_one_binary_and_mit_license`; `cargo package` |
| UC-L06 | Removed “stable Rust” from product copy. | `rg 'stable Rust'` returns no copy match |
| UC-L07 | Split the claim: the real demo uses only its generated loopback target; missing run arguments exit 2 without prompts. | `claim_bundled_demo_is_isolated_and_writes_real_proof`; `run_command_writes_bundle_and_returns_assertion_exit_code` |
| UC-L08 | Renamed the state to “All sample assertions pass” and tied it to the real three-assertion demo. | `@claim:web-demo-isolation`; `@claim:bundled-demo` |
| UC-L09 | Kept `proof.json` and parsed its three-step output. | `claim_bundled_demo_is_isolated_and_writes_real_proof` |
| UC-L10 | Kept `report.html`; inspected its workflow content and absence of external assets. | `claim_bundled_demo_is_isolated_and_writes_real_proof` |
| UC-L11 | Rewrote the sentence in plain words and scanned JSON/HTML for the seeded authorization secret. | `claim_bundled_demo_is_isolated_and_writes_real_proof`; `chained_workflow_produces_redacted_stable_evidence` |
| UC-L12 | Retained OpenAPI 3.0/3.1 and exercised 3.0 JSON plus 3.1 YAML. | `operation_path_and_parameter_substitution_work`; `chained_workflow_produces_redacted_stable_evidence` |
| UC-L13 | Retained both operation selectors and exercised both. | `operation_path_and_parameter_substitution_work`; chained workflow test |
| UC-L14 | Retained all four parameter locations and asserted their resolved evidence. | `operation_path_and_parameter_substitution_work` |
| UC-L15 | Retained JSON payloads and chaining; asserted a step-one value enters step two. | `chained_workflow_produces_redacted_stable_evidence` |
| UC-L16 | Retained six operators and exercised `==`, `!=`, `>`, `>=`, `<`, and `<=`. | `operation_path_and_parameter_substitution_work` |
| UC-L17 | Added pre-request rejection for remote sources and external references. | `claim_unsupported_features_name_the_problem_before_requests` |
| UC-L18 | Added pre-request rejection for OAuth and callbacks. | `claim_unsupported_features_name_the_problem_before_requests` |
| UC-L19 | Kept and tested JSONPath-context rejection. | `claim_unsupported_features_name_the_problem_before_requests` |
| UC-L20 | Kept and tested success/failure action rejection. | `claim_unsupported_features_name_the_problem_before_requests` |
| UC-L21 | Kept and tested non-JSON body rejection. | `claim_unsupported_features_name_the_problem_before_requests` |
| UC-L22 | Replaced the vague line with “Unsupported Arazzo features return an error.” | `claim_unsupported_features_name_the_problem_before_requests` |
| UC-L23 | Retained the exact exit/no-partial-output behavior and test matrix. | unsupported claim test; exit-code claim test |
| UC-L24 | Kept the USD/EUR mismatch and tested step-local generated comparison output. | `changed_response_assertion_is_visible_in_comparison_report`; web demo test |
| UC-L25 | Replaced the fragment with the required three facts and tested MIT, account, telemetry, and local execution. | single-binary/MIT, web-demo-isolation, site-privacy claim tests |
| UC-L26 | Replaced “A runner you can audit” with “Install the CLI.” | copy test; live `/#install` |
| UC-L27 | Removed both unmeasurable size/usefulness claims. | copy test; `rg` copy audit |
| UC-L28 | Removed the promotional “evolving” claim and named the documented 1.0.x subset. | copy test; supported-subset tests |

## README claim findings

| ID | Change made | Evidence |
| --- | --- | --- |
| UC-R01 | Rewrote the opening around the documented 1.0.x features and real local demo. | bundled-demo and supported-operation claim tests |
| UC-R02 | Split the dense sentence; JSON/HTML creation, chaining, and redaction are independently asserted. | bundled-demo; redaction-and-chaining |
| UC-R03 | Removed “Runs stay on your machine”; documented the narrower loopback demo and selected-target behavior. | bundled-demo; operation-selection test |
| UC-R04 | Retained the account/telemetry/upload boundary and registered browser plus CLI privacy coverage. | site-privacy; bundled-demo |
| UC-R05 | Changed “stable Rust” to “Rust” and retained the tested one-binary statement. | single-binary-MIT test; `cargo package` |
| UC-R06 | Reworded the version sentence directly. | single-binary-MIT test |
| UC-R07 | Replaced internal release prose with the direct `cargo package` instruction. | clean-clone `cargo package --allow-dirty` passed |
| UC-R08 | Kept the two outputs and tested their content and self-containment. | bundled-demo test |
| UC-R09 | Reworded stdout output and parse-tested `--json`. | exit-codes-and-json test |
| UC-R10 | Standardized “proof artifact” to `proof.json`. | comparison-report test; copy audit terminology table |
| UC-R11 | Standardized “one-file” to “self-contained.” | comparison-report test |
| UC-R12 | Rewrote actively and tested the step-local assertion change. | comparison-report test |
| UC-R13 | Kept the explicit environment requirement and tested missing `--env` exit 2. | exit-codes-and-json test |
| UC-R14 | Reworded to name the CLI and exercised YAML plus JSON environment files. | operation-selection-and-parameters test |
| UC-R15 | Tested an environment base URL overriding a distinct unreachable OpenAPI server. | operation-selection-and-parameters test |
| UC-R16 | Exercised `$inputs.petName` in the chained request body. | redaction-and-chaining test |
| UC-R17 | Exercised `$env.tenant` in a request parameter. | operation-selection-and-parameters test |
| UC-R18 | Asserted configured request headers reach captured evidence. | operation-selection-and-parameters test |
| UC-R19 | Rewrote “serialized” in plain words and scanned all output for sensitive headers/listed secrets. | redaction-and-chaining; bundled-demo |
| UC-R20 | Corrected JSON Pointer wording and asserted configured body replacement. | redaction-and-chaining test |
| UC-R21 | Reduced the list to implemented syntax and covered every family across the supported-subset tests. | operation-selection-and-parameters; redaction-and-chaining |
| UC-R22 | Split the long sentence into a lead and list; added missing validation for refs, callbacks, webhooks, OAuth, and retry. | unsupported-errors test |
| UC-R23 | Expanded the CLI test across exit codes 0, 1, and 2 for runs and comparisons. | exit-codes-and-json test |
| UC-R24 | Split the sentence and tested parseable `--json`; all subprocesses run with closed stdin. | exit-codes-and-json test |
| UC-R25 | Removed the unneeded test-suite marketing claim; linked readers to executable claim commands. | README and copy test |
| UC-R26 | Removed implementation marketing and tested all public routes for same-origin requests and empty storage. | site-privacy test |
| UC-R27 | Retained MIT and asserted Cargo metadata plus shipped license text. | single-binary-MIT test |
| UC-R28 | Removed “reproducible” positioning; the opening now names API owners and step-by-step evidence. | copy test; bundled demo test |

## Policy claim findings

| ID | Change made | Evidence |
| --- | --- | --- |
| UC-P01 | Narrowed the wording to selected API requests and no project/Sociobot telemetry. | bundled-demo and site-privacy tests |
| UC-P02 | Retained the site privacy claim and tested every route’s requests, scripts, cookies, and storage. | `@claim:site-privacy` |
| UC-P03 | Retained offline caching and reloaded demo plus Privacy with the browser offline. | `@claim:offline-site` |
| UC-P04 | Reworded this as an instruction; the cache uses one named, versioned service-worker namespace. | `sw.js` inspection; offline claim test |
| UC-P05 | Reworded to “base URL in your environment file” and tested override of an unreachable schema server. | operation-selection-and-parameters test |
| UC-P06 | Split configured redaction from the warning about other response data. | redaction-and-chaining test; live `/privacy/` |
| UC-P07 | Retained free/MIT/warranty terms and asserted shipped license and metadata. | single-binary-MIT test |
| UC-P08 | Replaced “evolving” with a direct link to supported features and tested every listed rejection family. | unsupported-errors test; live `/terms/` |

## Copy findings

| ID | Change made | Evidence |
| --- | --- | --- |
| CW-L01 | “The guide and sample report remain available.” | offline-site test; copy audit |
| CW-L02 | “Run Arazzo workflows and save proof.” | home screenshots; copy audit |
| CW-L03 | Added the API-owner audience sentence verbatim from the review. | home screenshots; copy audit |
| CW-L04 | “A report links each input, request, and assertion.” | home screenshots; copy audit |
| CW-L05 | Split install/demo behavior into short sentences and removed upload wording there. | copy audit |
| CW-L06 | Named `[REDACTED]` and what is replaced. | copy audit; redaction tests |
| CW-L07 | “Supported Arazzo features and explicit limits.” | home screenshot; copy test |
| CW-L08 | “Unsupported Arazzo features return an error.” | copy test |
| CW-L09 | “Share one HTML report instead of pasted response snippets.” | copy test |
| CW-L10 | “Documented Arazzo 1.0.x subset.” | README; copy test |
| CW-L11 | “The browser blocked clipboard access. Select the command manually.” | `main.ts`; copy test |
| CW-L12 | “CLI for API workflow reviews.” | home screenshots |
| CW-L13 | Primary action is “Try it with sample data”; install is secondary. | web-demo-isolation; home screenshots |
| CW-L14 | “Sample report” replaces unexplained specimen wording. | copy test |
| CW-L15 | First-screen facts now match the review exactly. | home-mobile screenshot |
| CW-L16 | “01 / Install the local CLI.” | copy test |
| CW-L17 | “Install the CLI.” | copy test |
| CW-L18 | “02 / Sample report.” | copy test |
| CW-L19 | “See exactly where a response changed.” | demo screenshots |
| CW-L20 | “Two output files.” | copy test |
| CW-L21 | “05 / Documented boundary.” | copy test |
| CW-L22 | “Supported features” and “Unsupported features.” | copy test |
| CW-L23 | “Review one complete run.” | copy test |
| CW-L24 | Final action now opens the real sample mode. | web-demo-isolation test |
| CW-R01 | Rewrote the opening without “small” or “auditable subset.” | README; copy test |
| CW-R02 | Split the 22-word sentence into three direct statements. | copy audit |
| CW-R03 | Removed negative positioning and untested reproducibility. | copy audit |
| CW-R04 | “The current package version is 0.1.0.” | copy audit; package test |
| CW-R05 | Replaced internal release language with a direct package command. | copy audit; package verification |
| CW-R06 | “Compare two `proof.json` files.” | copy audit |
| CW-R07 | Uses “self-contained” consistently. | copy audit |
| CW-R08 | Uses active “The report marks…” wording. | copy audit |
| CW-R09 | “The CLI accepts YAML and JSON environment files.” | copy audit; operation test |
| CW-R10 | Uses “before writing the proof bundle.” | copy audit |
| CW-R11 | Correctly says pointers are listed and values are replaced. | copy audit |
| CW-R12 | Split the 27-word unsupported sentence into a lead and list. | automated copy test |
| CW-R13 | Split JSON exit behavior from the no-prompt statement. | automated copy test |
| CW-R14 | Split the 23-word integration sentence, then removed it as unnecessary marketing. | automated copy test |
| CW-R15 | Removed site implementation detail from user-facing copy. | README; site-privacy claim |

## Structure checklist findings without report IDs

| Finding | Change made | Evidence |
| --- | --- | --- |
| Metadata gaps | Full route metadata and original-art derivatives added. | routing-and-metadata test; live source |
| Route focus/back | Hash, full-page, and back/forward focus now targets the route heading and announces it. | history focus test |
| Skeleton gaps | Header, ≤4-link nav, skip link, main, footer, legal links, one-liner, and build ID appear on every route. | routing-and-metadata test |
| Mobile touch/overflow | All controls measure at least 44×44; 390px and 200% text show no page overflow. | reduced-motion/mobile-control test |
| Dark/light contrast | Axe runs every public route across both color schemes and both required viewports. | all-route Axe test |
| Reduced motion | Reduced-motion media query removes transitions; browser assertion checks the computed duration. | reduced-motion test |
| Performance | Built JS 3.37 kB, CSS 14.00 kB, hero 208 kB; local Lighthouse 92/100/100/100 and live 91/100/100/100. | performance-budget test; `.factory/evidence/lighthouse-local.json`; `.factory/evidence/live/lighthouse-live.json` |
