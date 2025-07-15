#!/bin/bash

VERSION="v0.1.1-alpha"
BINARY="swissarch"
REPO="JustBeingJeeta/Swissarch"
URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY"

echo "Downloading $BINARY from $URL"
curl -L -o "$BINARY" "$URL"

if [ -f "/usr/local/bin/$BINARY" ]; then
  echo "$BINARY Is already installed, upgrading to next version"
  sudo rm "/usr/local/bin/$BINARY"
fi

chmod +x "$BINARY"
sudo mv "$BINARY" /usr/local/bin/

echo "$BINARY was successfully installed! start it using the command 'swissarch'"
