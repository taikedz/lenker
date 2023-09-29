out:error() {
    local code="$1"; shift
    echo "ERROR: $*" >&2
    exit $code
}

