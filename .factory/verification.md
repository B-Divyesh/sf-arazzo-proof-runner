# Verification report — PASS

Work order: `arazzo-proof-runner-verify-1`
Verified: 2026-08-27
Candidate: `7ad149493d6b3d8927ef38dd187275cdb71f2a1a` (`7ad1494 chore: harden release and record verification`)
Live URL: <https://arazzo-proof-runner.sociobot.in/>

## Disposition

**PASS.** The candidate satisfies the researched smallest useful product: it is a local, non-interactive Arazzo-subset runner with an explicitly required environment, redacted proof bundles, changed-run comparison evidence, and a matching public install/documentation site. No release-blocking defects were found.

## Clean-checkout gates

The workspace was clean and at the candidate SHA before installation. `npm ci` completed with 0 audited vulnerabilities. The following all passed:

```text
cargo test
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
cargo package
```

Results: 5 Rust unit tests, 2 binary-level CLI tests, 3 workflow integration tests, 1 doctest, and 4 browser/site tests passed. `npm run build` produced `dist/site/`; `cargo package` built and verified a 42.2 KiB compressed crate.

## Independent CLI consumer exercise

I unpacked `target/package/arazzo-proof-runner-0.1.0.crate`, ran `cargo install --path ... --root /tmp/arazzo-proof-consumer-verify/install --locked`, and exercised only that installed `arazzo-proof` binary against a separate local HTTP fixture.

- `--help` exposed the single binary's `run` and `compare` commands and CI-safe options.
- A two-step workflow passed with a required input, JSON body substitution, a chained output containing `/` (emitted as the correct `%2F` path segment), query substitution, authorization header, and configured secret. It produced `proof.json` and self-contained `report.html`; `--json` returned a valid success summary and exit 0.
- A second run changed the response assertion: it returned exit 1, retained its proof, and `compare --json` returned exit 1 with `changed: true`, `modified: 1`, including both `response.body` and `assertions` in `comparison.json`/`comparison.html`.
- Comparing the baseline to itself returned exit 0 with `changed: false`.
- `rg` found no occurrence of the fixture secret `super-secret-9876` in baseline/current proof JSON or HTML, or comparison JSON/HTML. The proof showed `[REDACTED]` in Authorization, body, response, and derived URL evidence.
- Recovery and boundary paths were clear and preserved no partial output: missing required workflow input, remote source description, and omitted `--env` each returned exit 2. The first two named the unsupported/missing condition; clap named the required `--env` argument.

## Live deployment and browser QA

The live response was HTTP 200 and its deployed product bytes match the exact local production build:

| File | Evidence |
| --- | --- |
| `/` | SHA-256 `59886ca235f44f0ac461ddf1c681571647b8dd859a7e01edc9c3aa672a64c272` equals `dist/site/index.html` |
| `/assets/index-Bedpu5Sf.js` | SHA-256 `ac6d843c96e1a6400a9556339bd574127c4c7dec6a9af443c08ef5f204cefea8` equals local |
| `/assets/index-BORThMjU.css` | SHA-256 `8733d22ef3da1ec27e6f6fd945f24686f99a77670d410bda27d9b4483bdb88f5` equals local |
| `/proof-strata.webp` | SHA-256 `b2e025eab1ea75e399e6e12101cc8434258854cf4e04b5fc7988a39eecef0e1e` equals local |
| `/sw.js` | byte-for-byte equal to local |

Playwright on live production found one title, one `h1`, one `main`, 0 console/page errors, and 0 serious/critical axe findings. The proof-diff toggle worked with keyboard focus + Enter; it exposed `aria-pressed=true`, the visible diff, failure stamp, and live announcement. Its focus indicator computed to a 3px lichen outline. At 390 x 844, there was no horizontal overflow. With reduced motion, transition duration was `0.00001s` and document scrolling was `auto`.

Only the first-party origin was requested during page load. The page had 0 cookies, no localStorage/sessionStorage keys, no analytics request, and no remote font/script. Its service worker was active; `registration.update()` completed with an active worker; after it took control, an offline reload returned HTTP 200 with the complete page and offline status message.

Live headers on HTML, assets, policy pages, image, and worker included CSP (`default-src 'self'`; `connect-src 'self'`; no object/frame embedding), `Referrer-Policy: strict-origin-when-cross-origin`, `X-Content-Type-Options: nosniff`, and HSTS. Hashed JS/CSS are `max-age=31536000, immutable`; the worker is `no-cache`; the hero is 30-day cached. `/privacy/` and `/terms/` each returned HTTP 200.

## Performance

Built assets are within the static budgets: initial JS 2,077 B, CSS 10,849 B, no font payload, and hero WebP 212,236 B (<300 KiB). A fresh live Lighthouse 12.8.2 mobile run returned Performance 95, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 2.0 s, TBT 240 ms, and CLS 0. (Lighthouse was connected to the preinstalled Chromium remote-debug port because direct launcher invocation crashes in this container; the page itself did not crash.)

## Defects by severity

| Severity | Defects |
| --- | --- |
| Critical | None found |
| High | None found |
| Medium | None found |
| Low | None found |

## Verification limits

The static host does not expose a separate deployment commit/build-identity header. Candidate identity was therefore verified by exact byte hashes for the live HTML, built JS/CSS, hero, and service worker rather than relying on an unverifiable header. No product-code changes were made during this verification.
