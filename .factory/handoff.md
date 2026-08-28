# Arazzo Proof Runner — polish round 2 handoff

## Outcome

Perfection-loop round 2 is complete. Every finding in `.factory/review-1.md` and `.factory/review-2.md` is resolved and reverified. The deployed product remains a Rust CLI with a Vite-built static landing/docs site and keeps the concrete-and-moss visual system.

Live: <https://arazzo-proof-runner.sociobot.in/>

Demo: <https://arazzo-proof-runner.sociobot.in/?demo=1>

Deployed repair: `6072f0f`

Azure deployment: `fd512f5e-e772-4f14-bae1-546d5ef1a60d`

## What changed

- Completed all six under-tested claim contracts: operation/file/version/parameter/operator coverage; run/compare exit matrix; unique/self-contained CLI demo; seeded web isolation; exact privacy allowlist; and complete route metadata/focus/404 assertions.
- Added a manifest-integrity test that enforces one active `@claim:<id>` test for every registered claim.
- Standardized the complete output as “proof bundle” across the hero, metadata, footer, README, CLI help, package description, and generated reports.
- Applied one ordered header/footer destination set to Home, Demo, Privacy, Terms, and 404.
- Added the explicit noindex 404 canonical URL and expanded route tests to cover it.
- Removed the unregistered internal publishing sentence from README.
- Kept the one-click in-memory `?demo=1` sample with banner, Reset demo, Start for real, and real-browser-data isolation.
- Updated `.factory/catalog-description.txt`, `.factory/copy-audit.md`, `.factory/claims.json`, screenshots, Lighthouse evidence, and `.factory/polish-2.md`.

## Exact verification

Clean clone: `/tmp/arazzo-polish2-clean-CQMpj6/repo`.

- Every command in `.factory/claims.json`: **12/12 passed independently**.
- `npm test`: passed 5 Rust unit tests, 3 claim tests, 2 CLI tests, 3 workflow tests, 1 doctest, and 11 site/browser tests; produced `dist/site/`.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo package`: passed; 181.3 KiB unpacked and 47.0 KiB compressed.
- Browser/Axe: zero violations on Home, Demo, Privacy, Terms, and 404 at 1440×900 and 390×844 in light and dark themes.
- Mobile/reduced motion: no horizontal overflow, all controls at least 44×44 CSS px, 200% text retained layout, reduced transitions computed at zero.
- Privacy: exact deployed-file network allowlist passed; no cookies, local/session app state, or IndexedDB; only the named shell cache was allowed.
- Demo isolation: seeded localStorage, sessionStorage, and IndexedDB survived enter/change/reset; no demo state persisted.
- Offline: Demo and Privacy reloaded after the browser context went offline.
- Local `verify-url.sh`: Home, Demo, Privacy, and Terms passed title/lang/one-h1/main/alt/labels/console checks.
- Local Lighthouse mobile: performance 99, accessibility 100, best practices 100, SEO 100; LCP 2.1 s, CLS 0.
- Built assets: 3.96 kB JS, 14.00 kB CSS, 208 kB hero image.

After deployment, fresh browser contexts repeated the checks against the public domain:

- Home, Demo, Privacy, and Terms returned 200; the product-authored unknown route returned 404.
- `verify-url.sh` passed every 200 route with zero console errors.
- All 20 route/viewport/theme Axe combinations passed.
- The five-route title, description, canonical, social metadata, skeleton, status, and heading-focus matrix passed.
- Demo preserved seeded real data, reset to USD, made only expected same-origin requests, and reloaded offline with Privacy.
- Live Lighthouse mobile: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.9 s, CLS 0.

Evidence is under `.factory/evidence/`; the finding-by-finding map is `.factory/polish-2.md`.

## Run and verify

```sh
npm ci
npm test
cargo clippy --all-targets --all-features -- -D warnings
cargo package
npm run build:site
```

CLI demo:

```sh
cargo run -- demo
```

Deployable static output: `dist/site/`.

## Known gaps and next steps

None for the brief or cumulative review findings. Registry publishing remains factory-owned and was not performed from this work order.
