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
- `include-dotfiles [bool] (= false)`
  - Whether to not ignore files *and directories* that start with a dot (`.`). This is off by default to ignore things like `.vscode`, `.git`, etc.
- `find [string | null] (= null)`
  - Find all files of a given language instead of listing all languages. This will print absolute paths to all files of the given programming language, case-insensitive.
- `exclude [string[]] (= [])`
    - A list of languages to exclude from both the count and display.
- `include [string[]] (= [])`
    - A list of file / directory names that are ignored by default (`node_modules`, `target`, etc.) to include in the count and display.

## Reference

Splik recognizes the following languages/extensions:

| Language         | Extensions                                                   |
|------------------|--------------------------------------------------------------|
| Assembly         | `.asm`                                                       |
| Bash             | `.bash`                                                      |
| C                | `.c`, `.h`                                                   |
| C++              | `.cpp`, `.cxx`, `.cc`, `.c++`, `.hpp`, `.hxx`, `.hh`, `.h++` |
| C#               | `.cs`                                                        |
| Fortran          | `.f`, `.for`, `.f90`, `.f95`                                 |
| Gleam            | `.gleam`                                                     |
| Go               | `.go`                                                        |
| Haskell          | `.hs`, `.lhs`                                                |
| Java             | `.java`                                                      |
| JavaScript       | `.js`, `.mjs`, `.cjs`                                        |
| JavaScript React | `.jsx`                                                       |
| Kotlin           | `.kt`                                                        |
| Lua              | `.lua`                                                       |
| MATLAB           | `.m`                                                         |
| PHP              | `.php`                                                       |
| Python           | `.py`                                                        |
| R                | `.r`                                                         |
| Ruby             | `.rb`                                                        |
| Rust             | `.rs`                                                        |
| SQL              | `.sql`                                                       |
| Svelte           | `.svelte`                                                    |
| Swift            | `.swift`                                                     |
| TypeScript       | `.ts`                                                        |
| TypeScript React | `.tsx`                                                       |
| V                | `.v`                                                         |
| Vue              | `.vue`                                                       |
| Zig              | `.zig`                                                       |

## Limitations

Splik is limited in a few ways:

- The language of a file is determined purely by its extension/name. The actual contents of the file are not analyzed. This can lead to inaccuracies - i.e., theres nothing stopping you from renaming `main.c` to `main.py`, and splik will think it's a Python file.
- Splik operates off of a known list of languages, meaning any new languages need to be manually contributed to splik itself before it can be recognized. Once a new language is added, all users of the tool will need to update splik to be able to recognize that language.
