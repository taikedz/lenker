#%include out.sh

help:print() {
cmd="$(basename "$0")"

cat <<EOHELP

Usage:

    $cmd INTERPERETER SCRIPT [ ARGS ... ]

Specify the interpreter first, followed by the script to link, followed by any arguments to pass into the script.

e.g.

    $cmd bash myscript.sh arg1 arg2

    $cmd "bash -x" myscript.sh arg1 arg2

EOHELP
}
