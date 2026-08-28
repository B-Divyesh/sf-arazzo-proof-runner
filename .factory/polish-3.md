# Perfection-loop polish 3

Candidate repaired: `99abc7f42e2b47396b17e069fe782e76434ced92`

Review consumed: `6252deb1072d7b16c71a2131c316c9a927d777b5` and every earlier `.factory/review-*.md` / `.factory/polish-*.md`

Implementation commit: `6b158da6e16f46ce591aaddbe9d01db75107b406`

Deployment: `1a893757-69ae-4b9d-8ebb-07238106575b` at <https://arazzo-proof-runner.sociobot.in/>

## Evidence keys

- **E-HOME** — `landing and README sentences use plain words`; `public copy contains no unregistered price claim`; `.factory/evidence/polish-3/root/cold-mobile.png`; cold live `/` returned 200 with the required first screen and no “Free” copy.
- **E-DEMO** — `claim_bundled_demo_is_isolated_and_writes_real_proof`; `@claim:web-demo-isolation`; `.factory/evidence/polish-3/demo/cold-mobile.png`; cold live `/?demo=1` showed the banner, real checkout sample, mutation, reset, and preserved seeded browser data.
- **E-CLI** — the named Rust test in each row, run independently from clean clone `/tmp/arazzo-polish3-clean-YT5XXC/repo`; `.factory/evidence/polish-3/demo/cold-desktop.png`; live `/?demo=1` matched the bundled three-step CLI sample.
- **E-ROUTES** — `@claim:routing-and-metadata routes have titles, metadata, focus, skeleton, and designed 404`; `.factory/evidence/polish-3/not-found/cold-mobile.png`; cold live `/`, `/demo`, `/privacy/`, `/terms/`, and `/not-a-real-route` passed status, title, metadata, navigation, attribution, focus, and 404 checks.
- **E-A11Y** — `all routes pass desktop and mobile Axe with no console errors`; `reduced motion removes transitions and every mobile control has a 44px target`; round-three route screenshots; 20/20 cold live route/viewport/theme Axe combinations passed.
- **E-PRIVACY** — `@claim:site-privacy every page uses only same-origin assets and no tracking state`; `@claim:offline-site cached public routes reload offline`; `.factory/evidence/polish-3/privacy/cold-mobile.png`; live exact-request allowlist, empty app storage, shell-only cache, and offline Demo/Privacy checks passed.
- **E-COPY** — `landing and README sentences use plain words`; `public copy contains no unregistered price claim`; `.factory/evidence/polish-3/terms/cold-mobile.png`; live Home and Terms contain no price claim.
- **E-PACKAGE** — `claim_package_declares_one_binary_and_mit_license`; clean-clone `cargo package`; `.factory/evidence/polish-3/terms/cold-desktop.png`; live `/terms/` names the MIT License without a price claim.
- **E-PERF** — `deploy artifact stays inside static performance budgets`; `.factory/evidence/polish-3/lighthouse-live.json`; `.factory/evidence/polish-3/root/cold-desktop.png`; live Lighthouse scored 99/100/100/100 with 2.0 s LCP, 70 ms TBT, and 0 CLS.

## Review 3 findings

| ID | Change made | Evidence |
| --- | --- | --- |
| F-3-1 | Removed “Free” from the first-screen fact and Terms. Kept only the package-tested MIT statement and added a public-copy regression test. | E-HOME; E-COPY; E-PACKAGE. |
| F-3-2 | Added visible “Built by Param Factory” attribution and the `polish-3` build marker to Home, Demo, Privacy, Terms, and 404. Extended the route claim to assert both on every route. | E-ROUTES; E-A11Y. |

## Review 2 findings

| ID | Change retained or strengthened | Evidence |
| --- | --- | --- |
| F-2-1 | Every manifest claim still has one active tag and a fully observable named test. All 12 commands passed independently in the clean clone. | E-CLI; E-DEMO; E-PRIVACY; E-ROUTES. |
| F-2-2 | Every route keeps the same ordered header and footer destinations; attribution is now shared too. | E-ROUTES. |
| F-2-3 | The noindex product 404 canonicals to `/404.html` and returns HTTP 404. | E-ROUTES. |
| F-2-4 | “Proof bundle” remains the sole name for the complete output. | E-HOME; E-COPY. |
| F-2-5 | The untestable registry-publishing sentence remains absent from README. | E-COPY. |

The six F-2-1 subfindings remain fully covered: `operation-selection-and-parameters` by `claim_operation_selection_and_parameters_full_matrix`; `exit-codes-and-json` by `claim_exit_codes_and_json_cover_run_and_compare_matrix`; `bundled-demo` by `claim_bundled_demo_is_isolated_and_writes_real_proof`; `web-demo-isolation`, `site-privacy`, and `routing-and-metadata` by their exact Playwright claim tests. E-CLI, E-DEMO, E-PRIVACY, and E-ROUTES include their live counterparts.

## Review 1 severity findings

| ID | Change retained or strengthened | Evidence |
| --- | --- | --- |
| B1 | The first screen names the Arazzo job, API-owner audience, proof bundle, first action, and outcome. | E-HOME. |
| B2 | The web and CLI demos remain setup-free, realistic, isolated, bannered, resettable, and disposable. | E-DEMO; E-CLI. |
| B3 | `.factory/claims.json` has 12 unique claims and exactly one active tagged test per claim. | E-CLI; E-DEMO; E-PRIVACY; E-ROUTES. |
| B4 | Unknown paths use the concrete-and-moss product 404 with a real 404 response. | E-ROUTES. |
| M1 | The mobile command scroller remains labelled, focusable, and visibly focused. | E-A11Y. |
| M2 | Policy routes retain the shared skeleton, 44 px controls, heading focus, announcements, and history restoration. | E-ROUTES; E-A11Y. |
| M3 | Every route retains its title, description, canonical, OG/Twitter metadata, favicon, and touch icon. | E-ROUTES. |
| M4 | Demo is followed by three usage steps, privacy boundaries, and supported/unsupported features. | E-HOME. |
| M5 | The install block pairs installation with the real `arazzo-proof demo` first run. | E-DEMO; E-CLI. |
| M6 | The invalid nested complementary landmark remains removed. | E-A11Y. |
| M7 | GitHub destinations remain visibly or accessibly marked external. | E-ROUTES. |

## Review 1 landing claim findings

| ID | Change retained or strengthened | Evidence |
| --- | --- | --- |
| UC-L01 | Offline wording is narrow and Demo plus Privacy reload offline. | E-PRIVACY. |
| UC-L02 | The real demo runs three steps, substitutions, assertions, and both output files. | E-DEMO; E-CLI. |
| UC-L03, UC-L04 | The retained no-account/no-telemetry facts are covered by the isolated demo and exact request allowlist. | E-DEMO; E-PRIVACY. |
| UC-L05, UC-L06 | One binary and MIT are package-tested; “stable Rust” remains absent. | E-PACKAGE; E-COPY. |
| UC-L07 | Explicit environment, selected target, no-prompt, and no-upload boundaries remain exercised. | E-CLI; E-PRIVACY. |
| UC-L08, UC-L09, UC-L10 | The demo proves three assertions, parsed `proof.json`, and self-contained `report.html`. | E-DEMO; E-CLI. |
| UC-L11 | Sensitive headers and configured secrets are absent from both outputs. | `chained_workflow_produces_redacted_stable_evidence`; E-CLI. |
| UC-L12, UC-L13, UC-L14 | OpenAPI 3.0/3.1, both selectors, and all parameter positions run in one claim test. | `claim_operation_selection_and_parameters_full_matrix`; E-CLI. |
| UC-L15, UC-L16 | Chained JSON output and all six comparison operators remain exercised. | `chained_workflow_produces_redacted_stable_evidence`; operation matrix; E-CLI. |
| UC-L17, UC-L18, UC-L19, UC-L20, UC-L21, UC-L22, UC-L23 | Every advertised unsupported family is rejected by name before a request or partial bundle. | `claim_unsupported_features_name_the_problem_before_requests`; E-CLI. |
| UC-L24 | The USD/EUR assertion change remains visible and Reset restores the sample. | `changed_response_assertion_is_visible_in_comparison_report`; E-DEMO. |
| UC-L25 | The price assertion is now removed; local/no-account behavior and MIT licensing remain separately tested. | E-HOME; E-DEMO; E-PACKAGE. |
| UC-L26, UC-L27, UC-L28 | Vague audit, size/usefulness, and evolving-standard promotion remains replaced by concrete actions and versioned scope. | E-COPY; E-HOME. |

## Review 1 README claim findings

| ID | Change retained or strengthened | Evidence |
| --- | --- | --- |
| UC-R01, UC-R02 | README keeps the versioned local runner, selected environment, and redacted two-file proof bundle wording. | operation, bundled-demo, and redaction claim tests; E-CLI. |
| UC-R03, UC-R04 | Overbroad machine-only wording remains absent; no account, telemetry, upload, or hosted service is tested at the actual boundaries. | E-DEMO; E-PRIVACY. |
| UC-R05, UC-R06, UC-R07 | One binary, version 0.1.0, and release-package validation pass from the clean clone. | E-PACKAGE. |
| UC-R08, UC-R09 | Both files and JSON stdout remain generated and parsed. | bundled-demo and exit-code tests; E-CLI. |
| UC-R10, UC-R11, UC-R12 | Comparison emits JSON/self-contained HTML and locates the changed assertion at its workflow step. | `changed_response_assertion_is_visible_in_comparison_report`; E-CLI. |
| UC-R13 | Missing explicit environment remains exit 2 before a request. | `claim_exit_codes_and_json_cover_run_and_compare_matrix`; E-CLI. |
| UC-R14, UC-R15, UC-R16, UC-R17, UC-R18 | YAML/JSON, target override, inputs, environment values, and headers remain exercised. | operation and redaction claim tests; E-CLI. |
| UC-R19, UC-R20 | Sensitive headers, listed secrets, and configured JSON Pointers remain redacted from saved evidence. | `chained_workflow_produces_redacted_stable_evidence`; E-CLI. |
| UC-R21, UC-R22 | Every supported matrix item and every advertised unsupported family remains executable evidence. | operation and unsupported claim tests; E-CLI. |
| UC-R23, UC-R24 | Run and compare cover exit 0/1/2 with JSON parsing, closed stdin, and deadlines. | `claim_exit_codes_and_json_cover_run_and_compare_matrix`; E-CLI. |
| UC-R25, UC-R26 | Test-suite marketing remains absent; exact live site requests and storage are checked instead. | E-COPY; E-PRIVACY. |
| UC-R27 | MIT remains package-tested and linked. | E-PACKAGE. |
| UC-R28 | Reproducibility promotion remains absent; the audience and evidence job are stated directly. | E-COPY; E-HOME. |

## Review 1 policy claim findings

| ID | Change retained or strengthened | Evidence |
| --- | --- | --- |
| UC-P01, UC-P02 | CLI target boundaries and the site’s exact request/storage boundaries remain tested. | operation claim test; E-PRIVACY. |
| UC-P03, UC-P04 | The versioned shell cache keeps Demo and Privacy readable offline and remains removable browser data. | E-PRIVACY. |
| UC-P05 | Environment `baseUrl` still overrides an unreachable OpenAPI server. | operation matrix; E-CLI. |
| UC-P06 | Configured redaction and the warning about other response data remain separate and visible. | redaction test; E-PRIVACY. |
| UC-P07 | The unregistered price wording is removed; MIT and warranty wording remain. | E-COPY; E-PACKAGE. |
| UC-P08 | Versioned support boundaries and explicit rejections remain linked and tested. | unsupported claim test; E-ROUTES; E-CLI. |

## Review 1 copy findings

| ID | Change retained or strengthened | Evidence |
| --- | --- | --- |
| CW-L01, CW-L02, CW-L03, CW-L04 | Offline, job, audience, and report-link wording remains direct. | E-HOME; E-COPY. |
| CW-L05, CW-L06 | Installation/privacy ideas remain split; `[REDACTED]` behavior is named concretely. | E-COPY; redaction test. |
| CW-L07, CW-L08, CW-L09, CW-L10 | Scope, error, sharing, and version wording remains concrete and testable. | E-COPY; E-CLI. |
| CW-L11, CW-L12, CW-L13, CW-L14, CW-L15 | Clipboard recovery, audience label, sample action, sample report, and first-screen facts remain clear; the fact list now omits price. | E-HOME; E-DEMO. |
| CW-L16, CW-L17, CW-L18, CW-L19, CW-L20 | Install/sample sequencing, exact response-change wording, and two-file explanation remain. | E-HOME; E-DEMO. |
| CW-L21, CW-L22, CW-L23, CW-L24 | Boundary headings and both real demo actions remain direct. | E-COPY; E-DEMO. |
| CW-R01, CW-R02, CW-R03, CW-R04, CW-R05 | README opening, audience, version, and packaging prose remains short and public-facing. | E-COPY; E-PACKAGE. |
| CW-R06, CW-R07, CW-R08, CW-R09, CW-R10, CW-R11 | Comparison, self-containment, active report language, formats, proof-bundle term, and pointer wording remain consistent. | E-COPY; E-CLI. |
| CW-R12, CW-R13, CW-R14, CW-R15 | Unsupported, JSON/no-prompt, test-suite, and site implementation prose remains short or removed. | E-COPY; E-CLI; E-PRIVACY. |

## Final verification

- Clean clone SHA: `6b158da6e16f46ce591aaddbe9d01db75107b406`; 12/12 claim commands passed independently.
- `npm test`: 5 unit tests, 3 claim tests, 2 CLI tests, 3 workflow tests, 1 doctest, and 12 site/copy/browser tests passed.
- `cargo fmt --all -- --check`, strict Clippy, and `cargo package` passed; package size was 181.3 KiB (47.0 KiB compressed).
- Production build: 3,959 B JS, 14,076 B CSS, and 212,236 B hero image; `dist/site/` produced.
- Worker `verify-url.sh` passed Home, Demo, Privacy, and Terms. The designed unknown route returned HTTP 404.
- Live verification: 20/20 Axe combinations, 14/14 links, route focus/history, demo isolation/reset, exact request allowlist, empty app storage, offline Demo/Privacy, CSP/security headers, and immutable hashed-asset caching passed.
- Live bytes for Home, Privacy, Terms, 404, service worker, CSS, and JS exactly matched `dist/site/`.
- Lighthouse: Performance 99, Accessibility 100, Best Practices 100, SEO 100; FCP 0.8 s, LCP 2.0 s, TBT 70 ms, CLS 0.
- Remaining findings: none.
