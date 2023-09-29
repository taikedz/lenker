#!/usr/bin/env bash

set -euo pipefail

#%include out.sh
#%include help.sh

fail() {
    help:print
    out:error "$@"
}

INTERPRETER="${1:-}"; shift || fail 100 "No interpreter specified"
INPUT="${1:-}"; shift || fail 101 "Input file not specified"

OUTPUT="$(mktemp ".$(basename "$INPUT")-XXXX")"
cleanup() { rm "$OUTPUT"; }

lenker "$INPUT" "$OUTPUT" | (grep -v "Wrote: $OUTPUT" || :)
trap cleanup EXIT

$INTERPRETER "$OUTPUT" "$@"

