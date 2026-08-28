# Arazzo Proof Runner

`arazzo-proof` runs the documented Arazzo 1.0.x features on your machine. Choose an environment and run a multi-step workflow. The CLI writes redacted JSON and HTML reports for CI or review.

It is for API owners who need step-by-step evidence while adopting Arazzo. The CLI has no account, telemetry, upload, or hosted service.

## Try the bundled demo

The demo starts its own loopback API and needs no setup or network service. It creates a unique temporary workspace and prints the generated report path.

```sh
cargo run -- demo
```

The same sample is visible at <https://arazzo-proof-runner.sociobot.in/?demo=1>. See [.factory/demo.md](.factory/demo.md) for its isolation contract.

## Install

Build the single `arazzo-proof` binary with Rust:

```sh
cargo install --path .
arazzo-proof --help
```

The current package version is `0.1.0`. Run `cargo package` to validate the release package. Factory automation owns registry publishing.

## Run and compare workflows

Run one workflow and write `proof.json` plus a self-contained `report.html`:

```sh
arazzo-proof run examples/pet-order.arazzo.yaml \
  --env examples/local.env.yaml \
  --workflow orderPet \
  --out proof/current
```

The example above expects a compatible API at `127.0.0.1:4010`. Use `arazzo-proof demo` for a first run without that API.

Print a JSON summary to stdout while writing the same proof bundle:

```sh
arazzo-proof run workflow.yaml --env ci.env.yaml --out proof/current --json
```

Compare two `proof.json` files:

```sh
arazzo-proof compare proof/baseline/proof.json proof/current/proof.json \
  --out proof/diff
```

This writes `comparison.json` and a self-contained `comparison.html`. The report marks each changed status, body, output, or assertion at its workflow step.

### Environment file

Every run command requires an environment file. The CLI accepts YAML and JSON files:

```yaml
name: local
baseUrl: http://127.0.0.1:4010
inputs:
  petName: Ada
values:
  tenant: sandbox
headers:
  Authorization: Bearer local-secret
secrets:
  - local-secret
redact:
  - /owner/email
```

- `baseUrl` replaces every OpenAPI server for the run.
- `inputs` supplies workflow inputs such as `$inputs.petName`.
- `values` supplies the runner extension `$env.tenant`.
- `headers` applies request headers.
- Sensitive headers and listed secrets become `[REDACTED]` in the proof bundle.
- JSON Pointers under `redact` replace matching request and response values.

### Supported Arazzo 1.0.x subset

- Local OpenAPI 3.0 or 3.1 YAML and JSON sources.
- Workflow inputs, ordered steps, and step outputs.
- Operations selected by `operationId` or a local `operationPath`.
- Path, query, header, and cookie parameters.
- JSON request bodies and chained output expressions.
- Response body, response header, and status expressions.
- The operators `==`, `!=`, `>`, `>=`, `<`, and `<=`.
- Redacted JSON and self-contained HTML proof bundles.

The CLI rejects unsupported features with an error. These include:

- remote sources, external references, callbacks, and webhooks;
- OAuth flows and retry policies;
- non-JSON request bodies and JSONPath criteria;
- success and failure actions.

The error names the affected step or source. Invalid or unsupported input exits 2 before writing a partial proof bundle.

## Exit codes

- `0`: the run passed, or a comparison found no changes.
- `1`: an assertion failed, or a comparison found changes.
- `2`: input, syntax, I/O, or request execution failed.

`--json` keeps the same exit codes. The CLI does not prompt for input.

## Develop and verify

```sh
npm ci
npm test
npm run build:site       # static site -> dist/site/
cargo package
```

Claim commands are listed in [.factory/claims.json](.factory/claims.json).

## Deploy

The factory deploys `dist/site/` as a static site. Build it with `npm run build:site`. Do not deploy the CLI from this repository.

## Repository layout

- `src/` — runner, redaction, comparison, and report library.
- `tests/` — CLI and workflow integration tests.
- `examples/` — bundled Arazzo and environment examples.
- `site/` — static guide and interactive sample report.
- `.factory/design.md` — visual system and asset provenance.

## License

MIT. See [LICENSE](LICENSE).
