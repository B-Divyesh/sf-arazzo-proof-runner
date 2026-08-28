# Copy audit

Audited 28 August 2026 after the round-one rewrite. Counts are whitespace-delimited. Interface labels, code, and data values are listed separately from sentences. No sentence exceeds 22 words, and no banned plain-words term remains.

## Landing-page sentences

| Words | Sentence |
| ---: | --- |
| 1 | Offline. |
| 7 | The guide and sample report remain available. |
| 7 | Demo — sample data, nothing is saved. |
| 11 | This page uses an in-memory sample and never reads browser storage. |
| 14 | For API owners testing multi-step workflows, it records redacted requests, responses, and assertion results. |
| 9 | Runs a bundled checkout workflow and opens its report. |
| 2 | Runs locally. |
| 4 | No account or telemetry. |
| 4 | Free and MIT licensed. |
| 8 | A report links each input, request, and assertion. |
| 4 | Install from the repository. |
| 11 | Then run the bundled demo without an account or API setup. |
| 4 | All sample assertions pass. |
| 5 | `proof.json` records machine-readable evidence. |
| 7 | The self-contained `report.html` is ready for review. |
| 13 | The CLI replaces authorization headers and configured secrets with `[REDACTED]` before writing either file. |
| 12 | Point the CLI to a local workflow and an explicit environment file. |
| 11 | The CLI calls only the base URL selected in that file. |
| 10 | Review each request, response, output, and assertion by workflow step. |
| 10 | The site does not receive workflow files or proof bundles. |
| 9 | Your selected API still receives the requests you run. |
| 8 | The CLI redacts configured secrets and sensitive headers. |
| 7 | Review response bodies before sharing a report. |
| 6 | Unsupported Arazzo features return an error. |
| 15 | The CLI names the step and feature, exits 2, and writes no partial proof bundle. |

Landing headings and actions are also within the limits: “Run Arazzo workflows and save proof” (6), “Try it with sample data” (5), “View install command” (3), “How it works” (3), “What the CLI does not do” (6), “Supported Arazzo features and explicit limits” (6), “Reset demo” (2), and “Start for real” (3).

## README sentences

| Words | Sentence |
| ---: | --- |
| 10 | `arazzo-proof` runs the documented Arazzo 1.0.x features on your machine. |
| 8 | Choose an environment and run a multi-step workflow. |
| 12 | The CLI writes redacted JSON and HTML reports for CI or review. |
| 12 | It is for API owners who need step-by-step evidence while adopting Arazzo. |
| 10 | The CLI has no account, telemetry, upload, or hosted service. |
| 13 | The demo starts its own loopback API and needs no setup or network service. |
| 13 | It creates a unique temporary workspace and prints the generated report path. |
| 8 | The same sample is visible at the demo URL. |
| 7 | See `.factory/demo.md` for its isolation contract. |
| 6 | The current package version is `0.1.0`. |
| 8 | Run `cargo package` to validate the release package. |
| 5 | Factory automation owns registry publishing. |
| 9 | The example above expects a compatible API at `127.0.0.1:4010`. |
| 10 | Use `arazzo-proof demo` for a first run without that API. |
| 9 | This writes `comparison.json` and a self-contained `comparison.html`. |
| 12 | The report marks each changed status, body, output, or assertion at its workflow step. |
| 7 | Every run command requires an environment file. |
| 8 | The CLI accepts YAML and JSON environment files. |
| 8 | The CLI rejects unsupported features with an error. |
| 2 | These include: |
| 8 | The error names the affected step or source. |
| 12 | Invalid or unsupported input exits 2 before writing a partial proof bundle. |
| 7 | `--json` keeps the same exit codes. |
| 7 | The CLI does not prompt for input. |
| 14 | The test suite covers the documented CLI, browser, accessibility, privacy, and offline behavior. |
| 8 | Claim commands are listed in `.factory/claims.json`. |
| 8 | The factory deploys `dist/site/` as a static site. |
| 6 | Build it with `npm run build:site`. |
| 8 | Do not deploy the CLI from this repository. |
| 1 | MIT. |
| 2 | See `LICENSE`. |

README lead-ins, bullet items, and commands are fragments rather than sentences. The automated `landing and README sentences use plain words` test checks every source line for the same 22-word and banned-word rules.

## Terminology table

| Concept | Required term |
| --- | --- |
| Complete run output | proof bundle |
| Human-readable output | HTML report; sample report only for the bundled example |
| Executable | CLI after the first `arazzo-proof` introduction |
| Feature boundary | documented Arazzo 1.0.x subset |
| Sample experience | demo |
| API destination | base URL |
