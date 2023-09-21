#!/usr/bin/env bash

set -euo pipefail

main() {
    mkdir -p target
    cargo build --release > target/build-release.log 2>&1
    mkdir -p ~/.local/bin
    cp target/release/lenker lenkerx/lenkerx ~/.local/bin/

    if ! which lenker >/dev/null; then
        echo "Add ~/.local/bin to your PATH"
    fi

    echo "Done."
}

main "$@"

