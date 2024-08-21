# Splik

Splik (Simple Language Identifier Kit) is a small CLI tool for identifying the languages used in a project. It has similar capabilities to [onefetch](https://github.com/o2sh/onefetch), but it has more focused features and does not require the directory to be a git project.

## Installation

Splik is available on all platforms through Cargo:

```bash
cargo install splik
```

## Usage

To run splik on the current directory, simply run `splik`:

```bash
splik
```

The list of available options is as follows:

- `output [human-readable | json | yaml] (= human-readable)`
    - The output format. The default is human readable, but other formats can be specified for scripts to easily parse.
- `ignore-dotfiles [bool] (= true)`
    - Whether to ignore files *and directories* that start with a dot (`.`). This is on by default to ignore things like `.vscode`, `.git`, etc.
