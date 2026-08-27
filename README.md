# Arazzo Proof Runner

`arazzo-proof` is a small, local runner for an auditable subset of the OpenAPI Arazzo specification. It executes a multi-step workflow against an explicitly selected environment and writes a stable, redacted proof bundle for CI and integration review.

It is for API owners adopting Arazzo who need reproducible evidence—not another general-purpose API client. Runs stay on your machine. There is no telemetry, account, upload, or cloud dependency.

## Install

Build the single binary with stable Rust:

```sh
cargo install --path .
arazzo-proof --help
```

The package starts at `0.1.0`. Factory release automation owns registry publishing; maintainers can validate the release artifact with `cargo package`.

## Usage

Run one workflow and write `proof.json` plus a self-contained `report.html`:

```sh
arazzo-proof run examples/pet-order.arazzo.yaml \
  --env examples/local.env.yaml \
  --workflow orderPet \
  --out proof/current
```

Print a machine-readable summary to stdout (evidence still goes to `--out`):

```sh
arazzo-proof run workflow.yaml --env ci.env.yaml --out proof/current --json
```

Compare two proof artifacts:

```sh
arazzo-proof compare proof/baseline/proof.json proof/current/proof.json \
  --out proof/diff
```

This writes `comparison.json` and a one-file `comparison.html`. A changed status, body, output, or assertion is called out at the step where it changed.

### Environment file

An environment file is always required; there is no implicit production target. YAML and JSON are accepted:

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

- `baseUrl` overrides the OpenAPI server for every operation.
- `inputs` supplies workflow inputs referenced as `$inputs.petName`.
- `values` supplies the runner extension `$env.tenant`.
- `headers` applies request headers. `Authorization`, cookies, API-key-like headers, and every value listed in `secrets` are replaced with `[REDACTED]` before evidence is serialized.
- `redact` contains JSON Pointers removed from captured request and response bodies.

### Supported Arazzo 1.0.x subset

- Local `sourceDescriptions` pointing to OpenAPI 3.0/3.1 YAML or JSON files.
- `workflowId`, workflow `inputs`, ordered `steps`, and step `outputs`.
- Operations selected by `operationId` or local `operationPath` JSON Pointer.
- Path, query, header, and cookie parameters; JSON `requestBody.payload`.
- Runtime expressions: `$inputs.name`, `$env.name`, `$steps.stepId.outputs.name`, `$response.body#/pointer`, `$response.header.Name`, and `$statusCode` in criteria.
- Success criteria comparisons: `==`, `!=`, `>`, `>=`, `<`, `<=` against strings, numbers, booleans, and `null`.
- Stable JSON/HTML evidence with request/response metadata, redaction, outputs, and assertion results.

Unsupported features fail clearly instead of being guessed: remote source URLs, callbacks/webhooks, external `$ref`, OAuth flows, non-JSON request bodies, JSONPath criteria, retry policies, and success actions/failure actions.

## Exit codes

- `0`: command completed and all assertions passed; comparisons are unchanged.
- `1`: the workflow ran but an assertion failed, or a comparison found changes.
- `2`: invalid input, unsupported syntax, I/O, or request execution failure.

`--json` never changes these exit-code semantics and the CLI never prompts, so it is safe in CI.

## Develop and verify

```sh
cargo test
npm install
npm test
npm run build:site       # static site -> dist/site/
npm run build            # same deploy build
cargo package
```

The integration suite runs three representative workflows against an in-process HTTP server and checks chaining, parameter substitution, redaction, and a visible changed assertion. The landing/docs site is plain HTML/CSS built by Vite, with no runtime dependencies or analytics.

## Repository layout

- `src/` — typed runner, redaction, comparison, and HTML rendering library.
- `tests/` — CLI and deterministic workflow integration tests.
- `examples/` — runnable Arazzo and environment examples.
- `site/` — static install guide and proof-report specimen.
- `.factory/design.md` — product-specific visual system and asset provenance.

## License

MIT. See [LICENSE](LICENSE).
