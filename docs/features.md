# Features

(this is a list of intended features. see README.md for currently available features)

## Example content


Example of a Mardkown file generated with code snippets, and using some string substitution:


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


## Resolution path

Specified files will be resolved along paths in `LENKER_PATH`, as well as the locations of the files. If unspecified, `LENKER_PATH=.` is default.

This will match along each path search location until a match is found. Once a match is found, path processing ends.

Status: implemented.

## Matching files

Every file encountered is "remembered" with its _canonical_ path.

In a folder called `this_dir` , the following two files are the same, and will be `include`-ed once.

```
#%include file.txt
#%include ../this_dir/file.txt
```

The canonical path is retained where the file is found. This also resolves symlinks.

Note that if the files are identical hardlinks, they will register as differnt files.

Status: implemented.


## Specific inclusion

Paths that contain `./` or `../` will forcibly _only_ match on the path the currently-including file was found in, ignoring the `LENKER_PATH` contents.

Status: implemented

## Folder separator conversion

Paths are resolved using the platforms's folder separator: `/` on nix systems, `\\` on Windows systems.

The following would both work in the same file:

```
#%include folder\file1.txt
#%include folder/file2.txt
```

## Recursive inclusion

By default, a file that is included has its contents resolved as well. Use the `-R` option to prevent resolving that file's inclusions.

```
Resolved
#%insert content.txt

Non-resolved
#%insert -R content.txt
```

Status: todo

## Blobs and regexes

```
#%include *.txt
```

This will look for all files matching the blob, and include them in _string character order_. So `10.txt` is included before `2.txt` To sort with human numbering:

```
#%include -H *.txt
```

To use regexes, add the `-r` option:

```
#%include -r .*\.txt
#%include -r -H .*\.txt
```

To include only files that do not match, use `-x` option - the following includes all files that do not match the `*.txt` blob

```
#%include -x *.txt
```

Status: todo

## Folder inclusions

If a folder is specified instead of a file, the direct files of the folder are included, following same ordering rules as above. Child folders and hidden files (starting with `.`) are skipped (no folder recursion).

## Substitutions

Declare a substitution operation to include `file.txt` contents, after replacing `foo` with `bar`:

```
#%include -S s/foo/bar/g file.txt
```

Use a regex subtitution - here, add a tab at the start of a new line

```
#%include -S r/^/\t/ file.txt
```

Use alternative separator - set the single character separator freely:

```
#%include -S s|http://|https://|g file.txt
```

Status: todo

## Environment Variables

Use environment variables anywhere in the inclusion string by using `%NAME%`:

```
#%include content-%VARIANT%.txt
```

You can set a variable with `#%setname` , to preserve literal percentage signs and items that are not to be options:

```
#%setname PERCENT_NAME 100%_completion.txt
#%setname MINUS_NAME -hyphenated.txt
#%include %MINUS_NAME% %PERCENT_NAME%
```

Status: todo

# User extenstions

An extended capability that might appear: custom user extensions written in an embdded scripting language. Maybe [RustPython](https://github.com/RustPython/RustPython), but the language selection is still open. I'm pretty keen on combining python and rust though, so...

The processor would be able to call user-defined scriptlets:

```python

def handle_line(line:str) -> list[str]:
    # receive and check/process a line,
    # return an array (rust vector) of lines to the engine


def handle_file(file_name:str, lines:list[str]) -> list[str]:
    # inspect lines, remove/insert/transform,
    # and return the adjusted lines as a Vec<String>


# The lenker object is provided into rust
lenker.register_line_handler(handle_line)
lenker.register_file_handler(handle_file)
```

This allows custom defined behaviours to be called either on a per-line basis or as a custom line pre-processor.

The source document then declares the modules to activate in Lenker, for example for improved script pre-processing:

```sh
# Enable the per-line handler in a module `bash-function-sigs`
#%lenker-ext bash-function-sigs : line

# Or, activate both
#%lenker-ext bash-function-sigs : line,file
```

