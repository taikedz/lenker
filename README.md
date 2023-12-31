# Lenker

A general-purpose text-file linker, that can link from shared resource directories. Can be used as a pre-processing step.

Write files, use `#%include FILEPATH` (resolve and add contents once) or `#%insert PATH` (add contents every time) to add the file's content into that location.

Resolve files from shared folders specified in `LENKER_PATH` - re-use snippets stored in a common library location, and make content re-use standard practice!

Useful for combining multiple files into a single file - some use-case examples:

* shell scripts (pre-link them, rather than sourcing at runtime)
* Markdown files (for re-usable content)
* YAML files (for combining definitions)

Anywhere you want to combine files pre-runtime which don't have the in-language feature to do so, this tool is your friend.

Anywhere you want to combine files from a shared source, this tool will help you.

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

# `lenkerx` : Interpreter/shebang usage

You can use lenkerx as a shebang target, or as a command line interpretrer:

Link files and execute the result:

```sh
# ARGS are passed directly to the compiled script
# lenkerx SHELL INPUT [ARGS ...]
lenkerx bash my-file.sh --some --args
```

`lenkerx` in shebang lines:

```sh
#!/usr/bin/env lenkerx bash

#%include common-files/*.sh

my-function "Hello"

```

See the source for [lenkerx](lenkerx/) for an example of a shell script written with lenker macros.

# Versions

## Current

Currently included directives:

* `insert`
* `include`
* recursive inclusion processing
* identification of files by canonical path
* resolve file paths from `LENKER_PATH`

## Future

Several other features are intended for the 1.0.0 version::

* file blob inclusion, folder inclusion
* text/regex substitution with PCRE
* in-line variable substitution

See [features.md](docs/features.md) for details.

# Build

See [build.md](docs/build.md) for details on building and installing from this source repo.)

