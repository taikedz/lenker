#!/usr/bin/env bash

set -euo pipefail

main() {
    mkdir -p target
    cargo build --release > target/build-release.log 2>&1
    mkdir -p ~/.local/bin

    # Use lenker to produce lenkerx :-)
    target/release/lenker lenkerx/lenkerx.sh target/release/lenkerx >> target/build-lenkerx.log 2>&1

    cp target/release/lenker target/release/lenkerx ~/.local/bin/

    if ! which lenker >/dev/null; then
        echo "Add ~/.local/bin to your PATH"
    fi

    echo "Done."
}

main "$@"

