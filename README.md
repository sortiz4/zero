# Zero
Zero is a fast, simple, single-pass command line utility that securely erases
files by zero-filling them. It's non-recursive and non-verbose by default, and
always prompts the user before descending into absolute paths.

#### Usage

```
Securely erase files (single-pass).

Usage:
    zero [OPTIONS] [DIRS|FILES]

Options:
    -d, --dry-run       Do not overwrite any files (verbose)
    -h, --help          Output this message
    -i, --interactive   Prompt before overwriting each file
    -r, --recursive     Recursively descend into directories
    -s, --suppress      Suppress all interaction
    -v, --verbose       Explain what's being done
    -V, --version       Output version information
```
