# Features

## Resolution path

Specified files will be resolved along paths in `INCLUSION_PATH` . If unspecified, `INCLUSION_PATH=.` is default.

This will match along each path search location until a match is found. Once a match is found, path processing ends.

## Matching files

Every file encountered is "remembered" with its _absolute_ path.

In a folder called `this_dir` , the following two files are the same, and will be `include`-ed once.

```
#%include file.txt
#%include ../this_dir/file.txt
```

The absolute path is retained where the file is found. To use the absolute paths of resolved symlinks, use

```
#%include -S file.txt
```

Note that if one inclusion resolves symlinks, and the next inclusion doesn't (and vice-versa), the absolute paths may be different and register as different files.

## Recursive inclusion

By default, a file that is included has its content added verbatim. Use the `-R` option to resolve that file's inclusions.

```
Non-resolved
#%insert content.txt

Resolved
#%insert -R content.txt
```

Paths that start with `./` will forcibly _only_ match on the path the currently-including file was found in.

```
# top file
#%include subdir/subfile.txt
```

```
# subdir/subfile.txt
#%include ./other.txt

# This will search relative to subdir/subfile.txt _only_ , and ignore INCLUDE_PATH
```

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


