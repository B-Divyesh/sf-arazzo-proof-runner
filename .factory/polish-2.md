# Perfection-loop polish 2

Candidate repaired: `c039d8b669bda2e6af490862fb2f2d4857643125`

Reviews consumed: `.factory/review-1.md`, `.factory/polish-1.md`, and `.factory/review-2.md` at `e85f8ad9903b462df3ba8c1b6f72bd9b869a11d9`

Repair deployed: `6072f0f` at <https://arazzo-proof-runner.sociobot.in/>

## Evidence keys

- **E-HOME** — `landing and README sentences use plain words`; `catalog description is verb-first and no longer than 120 characters`; screenshots `.factory/evidence/live/root/screenshot-mobile.png` and `screenshot-desktop.png`; cold live `/` returned 200 with the new title and heading.
- **E-DEMO** — `claim_bundled_demo_is_isolated_and_writes_real_proof`; `@claim:web-demo-isolation`; screenshots `.factory/evidence/live/demo/screenshot-mobile.png` and `.factory/evidence/live/demo/screenshot-desktop.png`; cold live `/?demo=1` and `/demo` passed banner, reset, seeded-data isolation, and same-origin checks.
- **E-CLI** — named Rust claim test in the row; screenshot `.factory/evidence/live/demo-mobile.png` shows the recorded bundled command output; live `/?demo=1` matched the three-step CLI sample.
- **E-ROUTES** — `@claim:routing-and-metadata routes have titles, metadata, focus, skeleton, and designed 404`; `hash and browser history navigation restore heading focus`; screenshots `.factory/evidence/live/root/screenshot-mobile.png`, `.factory/evidence/live/privacy/screenshot-mobile.png`, `.factory/evidence/live/terms/screenshot-mobile.png`, and `.factory/evidence/live/404-mobile.png`; cold live `/`, `/demo`, `/privacy/`, `/terms/`, and `/not-a-real-route` passed status, title, metadata, skeleton, and focus checks.
- **E-A11Y** — `all routes pass desktop and mobile Axe with no console errors`; `reduced motion removes transitions and every mobile control has a 44px target`; the same live screenshots; 20 cold live route/viewport/theme combinations had zero Axe violations and no unexpected console errors.
- **E-PRIVACY** — `@claim:site-privacy every page uses only same-origin assets and no tracking state`; `@claim:offline-site cached public routes reload offline`; screenshots `.factory/evidence/live/demo/screenshot-mobile.png` and `.factory/evidence/live/privacy/screenshot-mobile.png`; cold live Demo and Privacy passed request allowlisting, storage inspection, and offline reload.
- **E-PERF** — `deploy artifact stays inside static performance budgets`; `.factory/evidence/lighthouse-local.json` scored 99/100/100/100; `.factory/evidence/live/lighthouse-live.json` scored 100/100/100/100 with 1.9 s LCP and zero CLS; live `/` returned 200.

## Review 2 findings

| ID | Change made | Evidence |
| --- | --- | --- |
| F-2-1 / prior B3 | Replaced all six incomplete assertions with full observable claim tests. Added a manifest-integrity test that requires one active tag per claim. | All 12 manifest commands passed independently from `/tmp/arazzo-polish2-clean-CQMpj6/repo`; `every registered claim has exactly one active tagged test`; E-DEMO, E-CLI, E-ROUTES, E-PRIVACY. |
| F-2-1 `operation-selection-and-parameters` | One tagged test now runs YAML and JSON Arazzo/OpenAPI/environment inputs, OpenAPI 3.0 and 3.1, `operationId`, `operationPath`, path/query/header/cookie parameters, and all six operators. | `claim_operation_selection_and_parameters_full_matrix`; E-CLI; live `/?demo=1`. |
| F-2-1 `exit-codes-and-json` | Added the complete run/compare 0/1/2 matrix. Completed cases parse JSON; every process has closed stdin and a ten-second timeout. | `claim_exit_codes_and_json_cover_run_and_compare_matrix`; E-CLI; live `/?demo=1`. |
| F-2-1 `bundled-demo` | Runs the demo twice, proves distinct OS-temp workspaces, validates both output files and redaction, and rejects every fetchable HTML reference. | `claim_bundled_demo_is_isolated_and_writes_real_proof`; E-DEMO. |
| F-2-1 `web-demo-isolation` | Seeds real localStorage, sessionStorage, and IndexedDB before Demo; mutates and resets Demo; then proves every seeded value remains and no demo state persists. | `@claim:web-demo-isolation`; E-DEMO. |
| F-2-1 `site-privacy` | Replaced same-origin-only acceptance with an exact built-file request allowlist. Added cookie, local/session, IndexedDB, and shell-only Cache Storage assertions. | `@claim:site-privacy`; E-PRIVACY. |
| F-2-1 `routing-and-metadata` | Applies the complete metadata, canonical, header/footer, focus, and status contract to Home, Demo, policies, and 404. | `@claim:routing-and-metadata`; E-ROUTES. |
| F-2-2 | All four route files now use the same ordered header destinations and the same footer destinations; only `aria-current` varies. | `@claim:routing-and-metadata` asserts labels and hrefs; E-ROUTES. |
| F-2-3 | Added an explicit canonical policy: the designed noindex 404 canonicals to `/404.html`. | `@claim:routing-and-metadata`; `.factory/evidence/live/404-mobile.png`; live `/not-a-real-route` returned 404 with the canonical tag. |
| F-2-4 | Standardized the complete output as “proof bundle”; reserved `proof.json` and “HTML report” for its files. Updated hero, metadata, footer, README, CLI help, package text, report copy, and terminology audit. | `landing and README sentences use plain words`; E-HOME; live `/` and `/?demo=1`. |
| F-2-5 | Removed “Factory automation owns registry publishing” from the public README. | `landing and README sentences use plain words`; E-HOME; live source/install section at `/#install`. |

## Review 1 severity findings, reverified

| ID | Change retained or strengthened | Evidence |
| --- | --- | --- |
| B1 | The first screen names the Arazzo job, API-owner audience, first action, outcome, and three facts. The complete output now uses “proof bundle.” | E-HOME. |
| B2 | Web `?demo=1`, `/demo`, and CLI `demo` remain one-click/setup-free, isolated, bannered, resettable, and seeded with the real checkout sample. | E-DEMO. |
| B3 | The claim manifest now has exactly one fully covering tagged test per claim. | `every registered claim has exactly one active tagged test`; 12/12 clean-clone claim commands passed. |
| B4 | Unknown paths retain the concrete-and-moss 404 and real HTTP 404 status. | E-ROUTES. |
| M1 | The mobile command scroller remains keyboard focusable with a visible focus treatment. | E-A11Y. |
| M2 | Policy routes retain skip links, shared navigation/footer, 44 px targets, route announcement, and heading focus. | E-ROUTES; E-A11Y. |
| M3 | Every route has its required title, description, canonical, OG/Twitter data, favicon, and touch icon. | E-ROUTES. |
| M4 | Demo is followed by the three-step usage path, privacy boundaries, and documented subset. | E-HOME; live `/#how` and `/#limits`. |
| M5 | The install block uses the real `arazzo-proof demo` first run. | `claim_bundled_demo_is_isolated_and_writes_real_proof`; E-HOME. |
| M6 | The nested complementary landmark remains removed. | E-A11Y. |
| M7 | GitHub links remain visibly or accessibly marked external. | `@claim:routing-and-metadata`; E-ROUTES. |

## Review 1 claim findings, reverified

| IDs | Change retained or strengthened | Evidence |
| --- | --- | --- |
| UC-L01 | Offline wording remains narrow and demonstrable. | `@claim:offline-site`; E-PRIVACY. |
| UC-L02, UC-L08, UC-L09, UC-L10 | The real three-step sample emits parsed `proof.json`, self-contained `report.html`, and three passing assertions. | `claim_bundled_demo_is_isolated_and_writes_real_proof`; E-DEMO. |
| UC-L03, UC-L04 | No-account and no-telemetry facts remain; the privacy test now rejects unlisted same-origin requests too. | `@claim:web-demo-isolation`; `@claim:site-privacy`; E-DEMO; E-PRIVACY. |
| UC-L05, UC-L06 | The untested stable-Rust wording stays removed; one binary and MIT remain tested. | `claim_package_declares_one_binary_and_mit_license`; E-HOME. |
| UC-L07 | Explicit environments, selected targets, and no prompting remain executable behavior. | `claim_exit_codes_and_json_cover_run_and_compare_matrix`; `claim_operation_selection_and_parameters_full_matrix`; E-CLI. |
| UC-L11 | Authorization, listed secrets, and configured pointers are absent from both serialized outputs. | `chained_workflow_produces_redacted_stable_evidence`; `claim_bundled_demo_is_isolated_and_writes_real_proof`; E-CLI. |
| UC-L12, UC-L13, UC-L14, UC-L16 | The formerly split 3.0/3.1, selector, parameter, and operator matrix is now in one tagged test. | `claim_operation_selection_and_parameters_full_matrix`; E-CLI. |
| UC-L15 | A step output still feeds a later request. | `chained_workflow_produces_redacted_stable_evidence`; E-CLI. |
| UC-L17, UC-L18, UC-L19, UC-L20, UC-L21, UC-L22, UC-L23 | Every advertised unsupported family is rejected by name before requests or partial output. | `claim_unsupported_features_name_the_problem_before_requests`; E-CLI; live `/#subset`. |
| UC-L24 | The USD/EUR change is visible and Reset restores the baseline. | `changed_response_assertion_is_visible_in_comparison_report`; `@claim:web-demo-isolation`; E-DEMO. |
| UC-L25, UC-L26, UC-L27, UC-L28 | Required facts and versioned scope remain; vague promotional and unmeasurable language stays removed. | `landing and README sentences use plain words`; E-HOME. |
| UC-R01, UC-R02, UC-R08, UC-R09, UC-R10, UC-R11, UC-R12 | README output language now consistently names the proof bundle and its two concrete files; generation and comparison are fully inspected. | bundled-demo, redaction, comparison, and exit-code claim tests; E-CLI. |
| UC-R03, UC-R04, UC-R26 | Overbroad local-only wording stays removed; no-account/telemetry/upload and exact site-network boundaries are tested. | `@claim:site-privacy`; `claim_bundled_demo_is_isolated_and_writes_real_proof`; E-PRIVACY. |
| UC-R05, UC-R06, UC-R07, UC-R27 | One binary, version, package validation, and MIT remain concrete; the new unlisted publishing sentence is removed. | `claim_package_declares_one_binary_and_mit_license`; clean-clone `cargo package`; E-HOME. |
| UC-R13, UC-R23, UC-R24 | Run and compare now cover 0/1/2 with JSON parsing, closed stdin, and timeout. | `claim_exit_codes_and_json_cover_run_and_compare_matrix`; E-CLI. |
| UC-R14, UC-R15, UC-R17, UC-R18, UC-R21 | YAML/JSON, base URL override, environment values, headers, and the full supported matrix run in the named test. | `claim_operation_selection_and_parameters_full_matrix`; E-CLI. |
| UC-R16, UC-R19, UC-R20 | Inputs, secret/header redaction, and JSON Pointer redaction remain serialized-output assertions. | `chained_workflow_produces_redacted_stable_evidence`; E-CLI. |
| UC-R22 | The complete unsupported syntax list remains pre-request validated. | `claim_unsupported_features_name_the_problem_before_requests`; E-CLI. |
| UC-R25, UC-R28 | Test-suite marketing and reproducibility positioning remain removed. Generated report copy now says only what was captured. | `landing and README sentences use plain words`; E-HOME. |
| UC-P01, UC-P02 | Selected-target and no-telemetry behavior remain; site privacy now uses an exact path allowlist and all storage APIs. | `claim_operation_selection_and_parameters_full_matrix`; `@claim:site-privacy`; E-PRIVACY. |
| UC-P03, UC-P04 | Demo and Privacy remain readable offline; only the named shell cache is allowed. | `@claim:offline-site`; `@claim:site-privacy`; E-PRIVACY. |
| UC-P05, UC-P06 | Environment base URL behavior and configured redaction remain tested; the residual response-data warning remains visible. | operation and redaction claim tests; E-PRIVACY; live `/privacy/`. |
| UC-P07, UC-P08 | MIT/warranty and versioned feature boundaries remain concrete and linked. | package and unsupported-feature claim tests; E-ROUTES; live `/terms/`. |

## Review 1 copy and structural findings, reverified

| IDs | Change retained or strengthened | Evidence |
| --- | --- | --- |
| CW-L01, CW-L02, CW-L03, CW-L04, CW-L05, CW-L06 | First-screen, offline, install, and redaction wording remains plain; CW-L02 now consistently says “proof bundle.” | `landing and README sentences use plain words`; `.factory/copy-audit.md`; E-HOME. |
| CW-L07, CW-L08, CW-L09, CW-L10 | Versioned scope, errors, sharing, and subset wording remains direct. | copy test; unsupported claim test; E-HOME. |
| CW-L11, CW-L12, CW-L13, CW-L14, CW-L15 | Clipboard error, audience label, primary demo action, sample label, and three first-screen facts remain. | copy test; E-HOME; E-DEMO. |
| CW-L16, CW-L17, CW-L18, CW-L19, CW-L20 | Install/sample sequence, change wording, and two-file explanation remain. | copy test; E-HOME; E-DEMO. |
| CW-L21, CW-L22, CW-L23, CW-L24 | Boundary headings and the final real-demo action remain. | copy test; E-HOME; live `/#subset`. |
| CW-R01, CW-R02, CW-R03, CW-R04, CW-R05 | README opening, audience, version, and packaging text remain direct; internal publishing prose is now removed. | copy test; E-HOME. |
| CW-R06, CW-R07, CW-R08, CW-R09, CW-R10, CW-R11 | Comparison, self-containment, active report wording, file formats, proof-bundle term, and pointer wording remain. | copy test; comparison/redaction/operation claim tests; E-CLI. |
| CW-R12, CW-R13, CW-R14, CW-R15 | Unsupported, JSON/no-prompt, integration, and implementation prose remains short and testable. | copy test; exit/unsupported/site-privacy claim tests; E-PRIVACY. |
| Metadata gaps, route-focus/back, skeleton gaps | Full route metadata, canonical 404 policy, shared skeleton, and focus restoration are now asserted on every route. | E-ROUTES. |
| Mobile touch/overflow, dark/light contrast, reduced motion | All routes pass at 390 px and desktop in light/dark; controls remain at least 44 px; reduced motion removes transitions. | E-A11Y. |
| Performance | Built JS is 3.96 kB and CSS is 14.00 kB; local and live Lighthouse exceed every required threshold. | E-PERF. |

## Final verification

- Clean clone: `/tmp/arazzo-polish2-clean-CQMpj6/repo` at `160dad7`.
- Claims: 12/12 manifest commands passed independently.
- Full suite: `npm test` passed 5 unit tests, 3 claim tests, 2 CLI tests, 3 workflow tests, 1 doctest, and 11 site/browser tests.
- Release: `cargo clippy --all-targets --all-features -- -D warnings` and `cargo package` passed; package was 181.3 KiB, 47.0 KiB compressed.
- Build: `dist/site/` produced 3.96 kB JS and 14.00 kB CSS; hero remains 208 kB.
- Local verification: all four 200 routes passed `verify-url.sh` with one `h1`, `lang=en`, `main`, alt text, labelled controls, and zero console errors.
- Deployment: Azure Static Web Apps deployment `fd512f5e-e772-4f14-bae1-546d5ef1a60d` succeeded.
- Live verification: root, Demo, Privacy, and Terms passed `verify-url.sh`; the designed unknown route returned 404; 20/20 Axe combinations passed; cold Demo isolation/privacy/offline and the five-route metadata/focus matrix passed.
- Remaining findings: none.
