#!/bin/bash

if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: $0 <remmina-dir> <tabby-dir>"
    echo "Example: $0 /path/to/remmina /path/to/tabby"
    exit 1
fi

REM_DIR="$1"
TAB_DIR="$2"

# cargo run -- --remmina-dir "$REM_DIR" --tabby-dir "$TAB_DIR" --protocol rdp,vnc,ssh --remmina-check --execute
# cargo run -- --remmina-dir "$REM_DIR" --tabby-dir "$TAB_DIR"


# cargo run -- --remmina-dir "$REM_DIR" --tabby-dir "$TAB_DIR" --protocol ssh --remmina-check --execute --yes
cargo run -- --remmina-dir "$REM_DIR" --tabby-dir "$TAB_DIR" --protocol ssh,rdp --remmina-check --execute
#cargo run -- --remmina-dir "$REM_DIR" --tabby-dir "$TAB_DIR" --protocol ssh --remmina-check