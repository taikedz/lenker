# Lenker

A general-purpose text-file linker. Can be used as a pre-processing step.

Write files, use `#%include PATH` (resolve once) or `#%insert PATH` (resolve every time) to add the file's content into that location.

Useful for combining multiple files into a single file - some use-case examples:

* shell scripts (pre-link them, rather than sourcing at runtime)
* Markdown files (for re-usable content)
* YAML files (for combining definitions)

Anywhere you want to combine files pre-runtime which don't have the in-language feature to do so, this tool is your friend.

The options system allows doing much more than basic concatenation. Make content re-use standard!


## include vs insert

```
# greetings.txt

Hello !
```

```
# farewells.txt

Bye !
```

```
# inclusions.txt

#%insert  greetings.txt
#%insert  greetings.txt
#%include farewells.txt
#%include farewells.txt
```

results in

```
Hello !
Hello !
Bye !
```

# `lenkerx` : nterpreter/shebang usage

You can use lenkerx as a shebang target, or as a command line interpretrer:

Link files and execute the result:

```sh
# ARGS are passed directly to the compiled script
lenkerx INPUT [ARGS ...]
```

`lenkerx` in shebang lines:

```sh
#!/usr/bin/env lenkerx

#%include common-files/*.sh

my-function "Hello"

```

# Versions

## Current

Currently included directives:

* `insert`
* `include`
* recursive inclusion processing
* identification of files by canonical path

## Future

Several other features are intended for the 1.0.0 version::

* file blob inclusion, folder inclusion
* text/regex substitution with PCRE
* environment variable substitution

See [features.md](./features.md) for details.

# Build

See [build.md](build.md) for details on building and installing from this source repo.)

