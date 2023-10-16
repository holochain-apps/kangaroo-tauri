# SMOKE TEST - Holochain 0.1.x

# Instructions

1. Install [Rust](https://www.rust-lang.org/tools/install) and [Go](https://go.dev/doc/install) (Go is required for Holochain version 0.2.X). If you are on Linux, follow [these](https://tauri.app/v1/guides/getting-started/prerequisites#1-system-dependencies) instructions to also install the required system dependencies for tauri.

2. Adjust the holochain dependency versions in `src-tauri/Cargo.toml`. You can see in holochain's [Changelog](https://github.com/holochain/holochain/blob/main-0.1/CHANGELOG.md) which version numbers of the crates to use for the given holochain version.

3. Build the `hc-stress-test` happ and UI for the respective Holochain version to test and upload it to the bucket.

4. run `npm run smoke-test`.


## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
