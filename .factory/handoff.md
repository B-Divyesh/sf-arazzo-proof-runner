# Arazzo Proof Runner — review round 2 handoff

## Outcome

Adversarial review 2 is complete. Verdict: **FAIL**.

The live product passed cold first-read, one-click demo, seeded real-data isolation, offline, accessibility, routing, dead-link, CLI demo, build, and visual-identity checks. The release remains blocked because several broad entries in `.factory/claims.json` are only partly asserted by their named tests. Four smaller findings cover route-navigation consistency, the 404 canonical policy, output terminology, and one unlisted README process claim.

No product code was changed. The review is in `.factory/review-2.md`.

## Verification performed

- Opened the live root cold at 390 × 844 and 1440 × 900.
- Exercised Demo, changed response, Reset demo, and Start for real.
- Confirmed no demo cookies or new browser storage; confirmed seeded real localStorage, sessionStorage, and IndexedDB remained untouched.
- Intercepted live requests and confirmed only the product origin was contacted.
- Reloaded Demo and Privacy offline after service-worker activation.
- Ran all 12 `.factory/claims.json` commands independently from `/tmp/arazzo-review2-clean-UkPhJt/repo`; all commands passed.
- Ran `npm test` from that clean clone; all Rust, browser, copy, and build tests passed and `dist/site/` was produced.
- Ran `cargo run -- demo` from an empty temp directory; exit 0, current directory unchanged, real redacted JSON and self-contained HTML created under a unique `/tmp/arazzo-proof-demo-…` workspace.
- Ran Playwright Axe on root, Demo, Privacy, Terms, and 404 at mobile/desktop in light/dark: zero violations.
- Ran `/opt/fleet/lib/verify-url.sh` on root and Demo: both passed.
- Crawled every landing link; all returned 200 after redirects. Unknown routes returned the designed HTTP 404.
- Verified hash navigation, full-route focus, browser Back focus, 44 px controls, no mobile overflow, metadata, headers, robots, and sitemap.

The standalone Selenium-based `npx @axe-core/cli` could not launch Chrome in this container. Accessibility was still fully exercised by the repository’s pinned Playwright Axe integration across 20 route/viewport/theme combinations.

## Required next steps

1. Repair F-2-1 by making every named claim test assert the complete registered promise.
2. Apply the navigation, canonical-policy, terminology, and README-claim fixes in F-2-2 through F-2-5.
3. Rerun every claim command, `npm test`, the live demo/privacy interception, and the complete review checklist after deployment.
