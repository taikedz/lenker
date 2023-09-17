# Lenker

A general-purpose file content inclusion tool. Can be used as a pre-processing step.

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

Several other features are available:

* file blob inclusion, folder inclusion
* recursive inclusion-processing
* text/regex substitution with PCRE
* environment variable substitution

Example of a Mardown file generted with code snippets, and using some string substitution:


    # My Little Utility

    #%include common/std-header.md

    The main file is so simple:

    ```python
    #%insert src/main.py
    ```

    We can try to change to python3 like so:

    ```python3
    #%insert -S "r/^/    /,r/print(.+)/print(\1)/g"
    ```

    #%insert shoutouts/*.md

    #%insert common/footer.md


See [features.md](./features.md) for details.

