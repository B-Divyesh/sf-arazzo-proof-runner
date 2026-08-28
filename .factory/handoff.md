# Arazzo Proof Runner — polish 3 handoff

## Outcome

Perfection-loop round 3 is complete. The released candidate now has no unresolved finding from reviews 1–3. The static site retains the concrete-and-moss visual system and the Rust single-binary CLI remains unchanged.

Implementation commit: `6b158da6e16f46ce591aaddbe9d01db75107b406`

Deployment ID: `1a893757-69ae-4b9d-8ebb-07238106575b`

Live site: <https://arazzo-proof-runner.sociobot.in/>

Demo: <https://arazzo-proof-runner.sociobot.in/?demo=1>

## Changes

- Removed the unregistered “Free” price claim from the first screen and Terms while retaining tested MIT-license wording.
- Added a visible “Built by Param Factory” item and `v0.1.0 · polish-3` marker to every footer, including Demo and the designed 404.
- Extended the routing claim test across Home, Demo, Privacy, Terms, and 404 to assert factory attribution and build identity.
- Added a regression test that rejects price claims in all public site templates and README.
- Updated the verb-first catalog description and round-three copy audit.
- Recorded the complete cumulative finding map in `.factory/polish-3.md`.

## Clean-clone verification

Fresh clone: `/tmp/arazzo-polish3-clean-YT5XXC/repo` at `6b158da6e16f46ce591aaddbe9d01db75107b406`.

- All 12 commands in `.factory/claims.json` passed independently.
- `npm test` passed: 5 Rust unit tests, 3 Rust claim tests, 2 CLI tests, 3 workflow tests, 1 doctest, and 12 site/copy/browser tests.
- Browser gates include 20 route/viewport/theme Axe combinations, demo isolation/reset, exact-request privacy, offline reload, metadata/routing/focus, 44 px targets, 200% text, reduced motion, and asset budgets.
- `cargo fmt --all -- --check` passed.
- `cargo clippy --all-targets --all-features -- -D warnings` passed.
- `cargo package` passed: 181.3 KiB package, 47.0 KiB compressed.
- `npm run build:site` produced `dist/site/`: 3,959 B JS, 14,076 B CSS, and 212,236 B hero WebP.

## Deployed verification

- `/opt/fleet/lib/verify-url.sh` passed cold on Home, Demo, Privacy, and Terms: correct title, `lang=en`, one `h1`, `<main>`, alt/labels, and no console errors.
- Fresh live contexts passed 20/20 Axe checks across five route states, 390/1440 px, and light/dark modes.
- Home, Demo, Privacy, Terms, and unknown routes showed “Built by Param Factory,” `polish-3`, correct route metadata, and no “Free” wording. Unknown routes returned the product 404 with status 404.
- The one-click Demo showed the persistent banner, realistic three-step checkout, mutation/reset, and preserved seeded localStorage, sessionStorage, and IndexedDB data.
- Exact-request allowlisting, zero cookies/application storage, shell-only Cache Storage, offline Demo/Privacy reload, focus/history restoration, and all 14 public links passed.
- Live Home, Privacy, Terms, 404, service worker, CSS, and JS bytes matched the local production build exactly.
- Security headers passed: CSP, HSTS, Referrer-Policy, and X-Content-Type-Options. Hashed assets use immutable caching.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100, SEO 100; FCP 0.8 s, LCP 2.0 s, TBT 70 ms, CLS 0.

Evidence is under `.factory/evidence/polish-3/`; the complete per-finding map is `.factory/polish-3.md`.

## Run and verify

```sh
npm ci
npm test
cargo run -- demo
cargo clippy --all-targets --all-features -- -D warnings
cargo package
```

## Known gaps and next steps

None.
