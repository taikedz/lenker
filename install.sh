#!/usr/bin/env bash

set -euo pipefail

main() {
    cargo build --release > build-release.log 2>&1
    mkdir -p ~/.local/bin
    cp target/release/lenker ~/.local/bin/lenker

    if ! which lenker >/dev/null; then
        echo "Add ~/.local/bin to your PATH"
    fi

    echo "Done."
}

main "$@"

