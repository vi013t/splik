# Splik

Splik (Simple Language Identifier Kit) is a small CLI tool for identifying the languages used in a project. It has similar capabilities to [onefetch](https://github.com/o2sh/onefetch), but it has more focused features and does not require the directory to be a git project.

Splik can:

- Calculate the total bytes, lines, and files for all recognized programming language files within a directory
- Display the list of languages and their information sorted from most used to least
- List all files in a directory that are of a given programming language

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
- `no-ignore-dotfiles [bool] (= false)`
  - Whether to not ignore files *and directories* that start with a dot (`.`). This is off by default to ignore things like `.vscode`, `.git`, etc.
- `find [string | null] (= null)`
  - Find all files of a given language instead of listing all languages. This will print absolute paths to all files of the given programming language, case-insensitive.

## Reference

Splik recognizes the following languages/extensions:

| Language         | Extensions                                                   |
|:----------------:|:------------------------------------------------------------:|
| Assembly         | `.asm`                                                       |
| Bash             | `.bash`                                                      |
| C                | `.c`, `.h`                                                   |
| C++              | `.cpp`, `.cxx`, `.cc`, `.c++`, `.hpp`, `.hxx`, `.hh`, `.h++` |
| C#               | `.cs`                                                        |
| Gleam            | `.gleam`                                                     |
| Go               | `.go`                                                        |
| Java             | `.java`                                                      |
| JavaScript       | `.js`, `.mjs`, `.cjs`                                        |
| JavaScript React | `.jsx`                                                       |
| Kotlin           | `.kt`                                                        |
| Lua              | `.lua`                                                       |
| PHP              | `.php`                                                       |
| Python           | `.py`                                                        |
| R                | `.r`                                                         |
| Ruby             | `.rb`                                                        |
| Rust             | `.rs`                                                        |
| SQL              | `.sql`                                                       |
| Svelte           | `.svelte`                                                    |
| TypeScript       | `.ts`                                                        |
| TypeScript React | `.tsx`                                                       |
| V                | `.v`                                                         |
| Vue              | `.vue`                                                       |
| Zig              | `.zig`                                                       |
