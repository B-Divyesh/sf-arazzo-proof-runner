# Arazzo Proof Runner — adversarial review handoff

Work order: `arazzo-proof-runner-review-1`

Reviewed: 2026-08-28

Candidate: `5716673d97177d3fc0626d08136015a238fe2b96`
Live URL: <https://arazzo-proof-runner.sociobot.in/>

## Outcome

**FAIL.** The full report is [review-1.md](review-1.md).

Blocking findings:

1. The first screen does not name the intended API-owner audience.
2. There is no one-click web demo or `arazzo-proof demo` sandbox.
3. `.factory/claims.json` and `@claim:` tests are absent despite many public claims.
4. `/demo` and unknown paths use the generic Azure Static Web Apps 404.

The review also records a serious mobile Axe issue, incomplete route metadata and shared structure, missing route-focus behavior, a non-runnable first-use command, two README sentences over 22 words, unclear terminology, and missing external-link disclosure.

## What was done

- Opened the live site cold in fresh Chromium contexts at 390 × 844 and 1440 × 900 and recorded the pre-scroll interpretation.
- Audited every landing and README sentence with whitespace-delimited word counts, plus headings and actions.
- Checked `/demo`, `?demo=1`, the specimen toggle, browser storage, banner/reset/start controls, `.factory/demo.md`, and `arazzo-proof demo` from a fresh temp directory.
- Checked the missing claims manifest and inventoried every claim-like statement on the landing page, README, Privacy, and Terms routes with a concrete test recommendation.
- Tested live metadata, titles, headings, landmarks, links, 404 behavior, route focus, touch targets, console errors, and Axe at mobile width.
- Exercised first-party-only network behavior and offline reload after service-worker activation.
- Crawled all advertised landing links and hash targets.
- Ran the repository gate from a clean local clone.
- Changed only `.factory/review-1.md` and this handoff; product code was not modified.

## Verification evidence

- `npm ci`: pass, 0 vulnerabilities.
- `npm test` from clean clone: pass — 5 Rust unit tests, 2 CLI tests, 3 workflow tests, 1 doctest, 4 site tests.
- Vite build within that gate: pass — 2.08 kB JS, 10.85 kB CSS.
- Live root, `/privacy/`, `/terms/`, and GitHub destination: HTTP 200.
- Live `/demo` and an arbitrary unknown route: HTTP 404 with generic Azure page.
- `arazzo-proof demo`: exit 2, unrecognized subcommand.
- Live Axe at 390 px: one serious `scrollable-region-focusable` issue on `#install-command`; one moderate nested-complementary-landmark issue.
- Live root: no console errors, one `h1`, one `main`, no page overflow at 390 px.
- Offline reload after first visit: pass for `/` and `/privacy/`.
- Network observation: same-origin requests only; no cookies or Web Storage/IndexedDB data observed. The service-worker cache was present.
- Link crawl: no dead advertised links; external GitHub links returned 200.

## What remains

Implement the six acceptance changes at the end of `review-1.md`, then rerun the review from fresh browser contexts and a clean clone. A subsequent reviewer should not accept the product until all blocking findings are gone and no more than three minor findings remain.

The earlier independent build verification remains in [verification.md](verification.md) as historical evidence. Its PASS predates this stricter first-read/demo/claims review and should not be treated as the current disposition.
