# Holochain Kangaroo

Put your Holochain App in this Kangaroo's tauri pouch and let it run.


# Instructions

0. Install [Rust](https://www.rust-lang.org/tools/install) and [Go](https://go.dev/doc/install) (Go is required for Holochain version 0.2.X).

1. Click on the green "Use this template" button in this repository to create your own repository based off the Holochain Kangaroo and clone your newly created repository.

2. Run `npm install` to install the relevant tauri dependencies.

3. If you are on Linux, follow [these](https://tauri.app/v1/guides/getting-started/prerequisites#1-system-dependencies) instructions to also install the required system dependencies for tauri.

4. Add your `[your-project].happ` file to the `./pouch` folder

5. Add your unpacked `ui.zip` to `./pouch/ui`

6. Search the repository for `replace-me` and replace it with your project's name or follow the instructions in the comments if provided. **Note:** The `productName` in `src-tauri/tauri.conf.json` must not contain any dots.

7. Add your app's icon: If you have an icon for your app, make sure to have it as a 1024x1024 pixel `.png` format and run `npm run tauri icon [path-to-your-1024x1024-png]` (https://tauri.app/v1/guides/features/icons). This will generate all the necessary icons and store it in `src-tauri/icons`

8. Set all the version numbers in `package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`. The verison number in `src-tauri/Cargo.toml` is part of the filesystem storage logic. Whenever you change that version number, the data will be stored in a new location, meaning that a new, independent conductor will be set upfor this version.

9. Build the app locally by running `npm run tauri build`

## Publish cross-platform binaries

To publish cross-platform binaries (not code-signed), follow these steps:

1. Create a new branch `release`: `git checkout -b release`.

2. Push the new branch to github to trigger the release workflow.

For further releases:

1. Update the version number of your app in all relevant places.

2. Merge your changes from `main` into `release`

3. Push to github to trigger the release workflow.


## Troubleshooting

* If you get the error `Error failed to bundle project: Failed to build data folders and files` when running `npm run tauri build`, a likely reason is that your `productName` in `src-tauri/tauri.conf.json` contains invalid characters, such as dots (`.`)

*

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
