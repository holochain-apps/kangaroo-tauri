## Compile Holochain and lair-keystore

Check out `.github/workflows/release.yaml` to see what holochain and lair-keystore versions are needed for this version of the kangaroo.

The commands to install them should look like below, replacing the version numbers with the right ones and replacing `[_ARCHITECTURE_]` with the architecture of your computer. Run them in the root directory of this repository:

### Linux/macOS

```
mkdir src-tauri/bins

cargo install --version 0.4.2 lair_keystore
LAIR_PATH=$(which lair-keystore)
cp $LAIR_PATH src-tauri/bins/lair-keystore-v0.4.2-[_ARCHITECTURE_]

cargo install holochain --version holochain-v0.2.7-rc.1 --locked --features sqlite-encrypted
HOLOCHAIN_PATH=$(which holochain)
cp $HOLOCHAIN_PATH src-tauri/bins/holochain-v0.2.7-rc.1-[_ARCHITECTURE_]

```

`[_ARCHITECTURE_]` is `x86_64-apple-darwin` on an x86 macOS, `aarch64-apple-darwin` on an Arm/M1 macOS and `unknown-linux-gnu` on Linux.

### Windows

```
cargo install --version 0.4.2 lair_keystore
$LkPath = Get-Command lair-keystore | Select-Object -ExpandProperty Definition
Copy-Item $LkPath -Destination src-tauri/bins/lair-keystore-v0.4.2-x86_64-pc-windows-msvc.exe

cargo install holochain --version holochain-v0.2.7-rc.1 --locked --features sqlite-encrypted
$HcPath = Get-Command holochain | Select-Object -ExpandProperty Definition
Copy-Item $HcPath -Destination src-tauri/bins/holochain-v0.2.7-rc.1-x86_64-pc-windows-msvc.exe

```