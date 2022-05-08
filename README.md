# Lines counter

## Description
The program is designed to count the number of lines of any text file. All you need to do is specify the desired directory, and everything else the program will do for you.

# Usage
1. For developers
    - [Install](https://www.rust-lang.org/tools/install) Rust
    - Specify work `directory` in [settings.json](/settings.json#L11)
    - Specify correct file extensions in [settings.json](/settings.json#L12)
    - Run `cargo run` command to get start
2. For users
    - Go to [res](/res/) folder in repository
    - Specify work `directory` in [settings.json](/res/settings.json#L11)
    - Specify correct file extensions in [settings.json](/res/settings.json#L12)
    - Run exec file `./lines_counter`. In [res](/res/) folder you get the output file

# Settings
- `exclude_comments` - available values:
    - `All` - all comments
    - `Inline` - only inline comments
    - `Multiline` - only multiline comments
    - `NotExclude` - comments will be included in the result
- `exclude_empty_line` - you can set `true` or `false`
- `comment_format` - specify inline/multiline comments
- `directory` - specify work directory from `home directory` (e.g. `/Documents`)
- `extensions` - specify file extestions e.g. `[".rs", ".toml"]`. If is empty included all files
