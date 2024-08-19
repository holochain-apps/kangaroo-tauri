#!/bin/bash

PLATFORM=$(rustc -vV | sed -n 's/^.*host: \(.*\)*$/\1/p')
REQUIRED_HOLOCHAIN_VERSION="0.3.2"
REQUIRED_LAIR_VERSION="0.4.5"

if [[ "$PLATFORM" == *windows* ]]; then
  HOLOCHAIN_BINARY_FILENAME="holochain-v$REQUIRED_HOLOCHAIN_VERSION-$PLATFORM.exe"
  LAIR_BINARY_FILENAME="lair-keystore-v$REQUIRED_LAIR_VERSION-$PLATFORM.exe"
else
  HOLOCHAIN_BINARY_FILENAME="holochain-v$REQUIRED_HOLOCHAIN_VERSION-$PLATFORM"
  LAIR_BINARY_FILENAME="lair-keystore-v$REQUIRED_LAIR_VERSION-$PLATFORM"
fi

DESTINATION_DIR="src-tauri/bins"

# Check that this script is being run from the right location
if [ ! -f "package.json" ] || [ ! -f "src-tauri/tauri.conf.json" ];
then
    echo "Error: You must run this script in the root directory of the kangaroo repository."
    exit 1
fi
# Check wheter cargo is available
if [ ! command -v cargo &> /dev/null ] || [ ! command -v rustc &> /dev/null ];
then
    echo "Error: You need to install Rust first."
    exit 1
fi

# create src-tauri/bins if id doesn't exist
if [ ! -d $DESTINATION_DIR ];
    then mkdir $DESTINATION_DIR
fi

# check whether correct holochain binary is already in the src-tauri/bins folder
if [ -f "$DESTINATION_DIR/$HOLOCHAIN_BINARY_FILENAME" ];
    then
        echo "Required holochain binary already installed."
    else
        echo "Installing required holochain binary from matthme/holochain-binaries."
        HOLOCHAIN_BINARIES_URL="https://github.com/matthme/holochain-binaries/releases/download/holochain-binaries-$REQUIRED_HOLOCHAIN_VERSION/$HOLOCHAIN_BINARY_FILENAME"
        curl -L $HOLOCHAIN_BINARIES_URL -o $DESTINATION_DIR/$HOLOCHAIN_BINARY_FILENAME
        chmod +x $DESTINATION_DIR/$HOLOCHAIN_BINARY_FILENAME
        echo "holochain binary downloaded and save to $DESTINATION_DIR/$HOLOCHAIN_BINARY_FILENAME"
fi

# check whether correct lair binary is already in the src-tauri/bins folder
if [ -f "$DESTINATION_DIR/$LAIR_BINARY_FILENAME" ];

    then
        echo "Required lair-keystore binary already installed."
    else
    	echo "Installing required lair-keystore binary from crates.io"
        LAIR_BINARIES_URL="https://github.com/matthme/holochain-binaries/releases/download/lair-binaries-$REQUIRED_LAIR_VERSION/$LAIR_BINARY_FILENAME"
        curl -L $LAIR_BINARIES_URL -o $DESTINATION_DIR/$LAIR_BINARY_FILENAME
        chmod +x $DESTINATION_DIR/$LAIR_BINARY_FILENAME
        echo "lair binary downloaded and saved to $DESTINATION_DIR/$LAIR_BINARY_FILENAME"
fi

echo "done."