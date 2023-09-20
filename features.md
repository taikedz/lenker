# Features

(this is a list of intended features. see README.md for currently available features)

## Example content


Example of a Mardkown file generted with code snippets, and using some string substitution:


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

## Matching files

Every file encountered is "remembered" with its _canonical_ path.

In a folder called `this_dir` , the following two files are the same, and will be `include`-ed once.

```
#%include file.txt
#%include ../this_dir/file.txt
```

The canonical path is retained where the file is found. This also resolves symlinks.

Note that if the files are identical hardlinks, they will register as differnt files.

## Recursive inclusion

By default, a file that is included has its contents resolved as well. Use the `-R` option to prevent resolving that file's inclusions.

```
Resolved
#%insert content.txt

Non-resolved
#%insert -R content.txt
```

Paths that contain `./` or `../` will forcibly _only_ match on the path the currently-including file was found in, ignoring the `LENKER_PATH` contents.

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


