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

# Current

Currently included directives:

* `insert`
* `include`
* recursive inclusion processing
* identification of files by canonical path

# Future

Several other features are intended for the 1.0.0 version::

* file blob inclusion, folder inclusion
* text/regex substitution with PCRE
* environment variable substitution

See [features.md](./features.md) for details.

