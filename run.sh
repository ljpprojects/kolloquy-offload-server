#! /bin/bash

# set BYTES in the program's env before running

echo -n "$BYTES" | base64 -d | nohup ./target/release/kolloquy-offload-server &
