#! /bin/bash

BYTES="$(ssh klqy@mosaic.local "cat ~/Library/kolloquy/secret.txt")"

echo -n "$BYTES" | base64 -d | nohup ./target/release/kolloquy-offload-server &
