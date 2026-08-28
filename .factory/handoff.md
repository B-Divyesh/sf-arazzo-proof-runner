# Arazzo Proof Runner — polish round 1 handoff

## Outcome

All findings in `.factory/review-1.md` are resolved. No earlier review or polish report existed. The repaired static site and CLI were pushed to `main` and deployed to <https://arazzo-proof-runner.sociobot.in/>.

Implementation commits:

- `efe104a` — isolated CLI/web demo, claims, routing, copy, metadata, policies, tests, and evidence
- `d6981a2` — dark-mode contrast and mobile interaction gates
- `f843470` — finding-by-finding polish record and final claim coverage
- `02f50a3` — 404 dark-mode contrast found during the live cold check

## Product changes

- Added `arazzo-proof demo`. It starts a loopback fixture, runs the real bundled three-step checkout, and writes a proof bundle under a unique system temporary directory.
- Added one-click `?demo=1` with a persistent “sample data, nothing is saved” banner, reset, and “Start for real.” Web state is memory-only.
- Rewrote the first screen to identify API owners, the job, the report contents, and the first action.
- Added `.factory/claims.json` with 12 independently runnable claim commands and unique `@claim:` tags.
- Added a shared route skeleton, route focus announcements, back/hash focus restoration, complete metadata, original-art social/touch images, `/demo`, and a styled 404 with a real 404 status.
- Added the three-step usage path, privacy limits, runnable install/demo commands, and all review-requested copy changes.
- Added validation for external references, callbacks, webhooks, OAuth flows, and retry policies before requests run.
- Preserved the concrete-and-moss visual thesis and recorded derivative asset provenance in `.factory/design.md`.

The exhaustive finding map is in `.factory/polish-1.md`. Demo isolation is documented in `.factory/demo.md`; sentence counts and terminology are in `.factory/copy-audit.md`.

## Verification evidence

Final fresh clone: `/tmp/arazzo-release-3Rm23C/repo` from commit `02f50a3`.

- Every command in `.factory/claims.json`: 12/12 passed independently.
- `npm test`: 5 Rust unit tests, 1 doctest, 8 CLI/workflow/claim integration tests, and 10 browser/copy tests passed.
- Browser coverage: every route at 390×844 and 1440×900 in light and dark themes; zero Axe violations and zero valid-page console errors.
- Interaction coverage: keyboard scroll region, route/hash/back focus, live announcements, 44×44 targets, reduced motion, 200% text, and no horizontal page overflow passed.
- Privacy: all public-route requests remained same-origin; cookies, localStorage, sessionStorage, and IndexedDB remained empty through the demo flow.
- Offline: demo and Privacy reloaded after the browser context was switched offline.
- `cargo package --allow-dirty`: passed; 27 files, 175.7 KiB unpacked and 45.8 KiB compressed in the clean-clone check.
- Static budgets: 3.37 kB JavaScript, 14.00 kB CSS, 208 kB hero image.
- Local Lighthouse: performance 92, accessibility 100, best practices 100, SEO 100 (`.factory/evidence/lighthouse-local.json`).
- Live Lighthouse: performance 91, accessibility 100, best practices 100, SEO 100 (`.factory/evidence/live/lighthouse-live.json`).

Screenshots:

- `.factory/evidence/home-mobile.png`
- `.factory/evidence/demo-mobile.png`
- `.factory/evidence/home-desktop.png`
- `.factory/evidence/demo-desktop.png`
- `.factory/evidence/privacy-mobile.png`
- `.factory/evidence/terms-mobile.png`
- `.factory/evidence/404-mobile.png`
- `.factory/evidence/live/root/screenshot-desktop.png`
- `.factory/evidence/live/root/screenshot-mobile.png`
- `.factory/evidence/live/demo/screenshot-desktop.png`
- `.factory/evidence/live/demo/screenshot-mobile.png`

## Deployment and cold live check

Deployment command:

```sh
npm ci
npm run build:site
/opt/fleet/lib/deploy-static.sh arazzo-proof-runner dist/site
```

Final deployment ID: `2b3830ff-948e-4691-9103-1f4a31466eda`.

Cold public checks after the final deployment:

- `/`, `/?demo=1`, `/privacy/`, `/terms/`, `robots.txt`, `sitemap.xml`, social image, and touch icon: HTTP 200.
- `/demo`: redirects to `/?demo=1`, then HTTP 200.
- `/not-a-real-route`: product-authored page with HTTP 404.
- Factory `verify-url.sh`: root and demo both passed with one `h1`, `lang=en`, a main landmark, image alt text, and zero console errors.
- Playwright + Axe: 20 public route/theme/viewport combinations passed with zero violations and no valid-page console errors.
- Live demo reset and storage isolation passed in a fresh 390px context.
- Live service-worker demo and Privacy reload passed offline.

## Run and verify

```sh
cargo run -- demo
npm ci
npm test
npm run build:site
cargo package
```

## Known gaps and next steps

No acceptance gap remains. Registry publication is intentionally left to factory release automation, as required by the CLI publishing contract.
