# Demo contract

## Entry points

- Web: <https://arazzo-proof-runner.sociobot.in/?demo=1>
- CLI from this repository: `cargo run -- demo`
- Installed CLI: `arazzo-proof demo`

The first-screen “Try it with sample data” link opens the web entry point in one click. `/demo` redirects to the same mode.

## Sample data

The bundled checkout workflow creates cart `crt_17`, adds one `moss-biscuit`, and quotes the cart in USD. It has three steps and three passing assertions. The CLI uses `examples/checkout.arazzo.yaml` and `examples/checkout.openapi.yaml` through the production runner.

## Isolation and reset

The CLI binds an ephemeral loopback port and creates `arazzo-proof-demo-<pid>-<timestamp>` under the operating-system temporary directory. It never reads or writes the current project. It prints the workspace path so the user can inspect or remove it.

The web demo keeps state only in page memory. It does not use cookies, localStorage, sessionStorage, IndexedDB, or OPFS. “Reset demo” restores the sample response. “Start for real” drops `?demo=1` and moves to installation. Closing or reloading the page discards changes.
