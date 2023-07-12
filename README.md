# Holochain Kangaroo

Put your Holochain App in this Kangaroo's tauri pouch and let it run around.

This repository let's you easily convert your Holochain app into a standalone tauri-based cross-platform Desktop app.

# Instructions

1. Install [Rust](https://www.rust-lang.org/tools/install) and [Go](https://go.dev/doc/install) (Go is required for Holochain version 0.2.X). If you are on Linux, follow [these](https://tauri.app/v1/guides/getting-started/prerequisites#1-system-dependencies) instructions to also install the required system dependencies for tauri.

2. Either use this repository as a template (by clicking on the green "Use this template" button) or fork it.<br>
(Using it as a template allows you to start with a clean git history and the contributors of this repository won't show up as contributors to your new repository. Forking has the advantage of being able to relatively easily pull in updates from this parent repository at a later stage.)

3. After cloning the newly created repository, run `npm install` to install the relevant tauri dependencies.

4. Add your `[your-project].happ` file to the `./pouch` folder

5. Add your unpacked `ui.zip` to `./pouch/ui`

6. Search the repository for `replace-me` and replace it with your project's name or follow the instructions in the comments if provided. **Note:** The `productName` in `src-tauri/tauri.conf.json` must not contain any dots or parentheses (and likely other special characters).

7. Add your app's icon: If you have an icon for your app, make sure to have it as a 1024x1024 pixel `.png` format and run `npm run tauri icon [path-to-your-1024x1024-png]` (https://tauri.app/v1/guides/features/icons). This will generate all the necessary icons and store it in `src-tauri/icons`

8. Set all the version numbers in `package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`. The verison number in `src-tauri/Cargo.toml` is part of the filesystem storage logic, **read the [note on versioning](#note-on-versioning)**

9. Build the app locally by running `npm run tauri build`

## Publish cross-platform Binaries

To publish cross-platform binaries (not code-signed), follow these steps:

1. Create a new branch `release`: `git checkout -b release`.

2. Push the new branch to github to trigger the release workflow.

For further releases:

1. Update the version number of your app in all relevant places.

2. Merge your changes from `main` into `release`

3. Push to github to trigger the release workflow.


## Code-signed cross-platform Binaries

The `.github/workflows/release-codesigned.yaml` contains a template workflow for binaries with macOS as well as Windows EV Certificate code signing. The workflow gets triggered when publishing on branch `release-codesigned`.

The macOS code signing is based on tauri's instructions [here](https://tauri.app/v1/guides/distribution/sign-macos).

The Windows code signing with EV cert is based on [these](https://melatonin.dev/blog/how-to-code-sign-windows-installers-with-an-ev-cert-on-github-actions/) instructions and uses a slightly modified [fork](https://github.com/matthme/tauri-action-ev-signing/) of tauri's github actions.

For the Windows part or if you want to only do macOS code signing, follow the instructions in the comments in `release-codesigned.yaml` (search for the keyword `HELP`).

If you want to sign your Windows binaries with an OV certificate instead of an EV certificate, follow [tauri's instructions](https://tauri.app/v1/guides/distribution/sign-windows).


## Auto-Updating of your App

To add automatic updates to your app, you may follow the instructions [here](https://tauri.app/v1/guides/distribution/updater).
An empty `updater.json` template file is part of this repository.

Some important notes:

* The Holochain Kangaroo stores data on the filesystem according to [semantic versioning](https://semver.org/). See [Note on versioning](#note-on-versioning)

* If you bump your Holochain (and/or) lair keystore version, you need to consider whether it remains compatible with the existing Holochain conductor / lair keystore.

As a consequence, **be careful not to trigger automatic updates on your end-users if your app
is a breaking change due to one of the above mentioned scenarios**.


## Note on Versioning

The Holochain Kangaroo stores data on the filesystem according to [semantic versioning](https://semver.org/). This has implications on your choice of package versions you give to your app in `src-tauri/Cargo.toml`.

<pre>Example:
Binaries built with Cargo.toml versions 0.0.2 and 0.0.3 will store their data in separate subfolder on the filesystem and will have independent Holochain conductors. From end-user perspective this is a breaking change and opening a 0.0.3 version of your app won't provide access to data stored with the 0.0.2 version of your app.

Binaries built with Cargo.toml versions 0.3.2 and 0.3.4 will share the same subfolder `0.3.x` on the filesystem and will share the same Holochain conductor.

Binaries built with Cargo.toml versions 2.0.5 and 2.3.4 will share the same subfolder `2.x.x` on the filesystem and will share the same Holochain conductor.</pre>

## Troubleshooting

* If you get the error `Error failed to bundle project: Failed to build data folders and files` when running `npm run tauri build`, a likely reason is that your `productName` in `src-tauri/tauri.conf.json` contains invalid characters, such as dots (`.`)

* If building the app fails with errors like `Error: No artifacts were found.` or `Error failed to bundle project: error running light.exe` the reason may again be that the `productName` in `src-tauri/tauri.conf.json` contains invalid characters such as parentheses (`(` or `)`) or possibly other unsupported special characters.



## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
