use std::{io::BufRead, os::unix::fs::MetadataExt, path::PathBuf};

use clap::Parser as _;

fn main() {
    let arguments = Arguments::parse();

    let source = if std::fs::read_dir("src").is_ok() { "./src" } else { "." };
    let mut languages = LanguageList::default();

    analyze_directory(source, &arguments, &mut languages);

    languages.sort();

    if let Some(language) = arguments.find {
        languages.find(&language);
        return;
    }

    match arguments.output {
        OutputFormat::HumanReadable => languages.display(),
        OutputFormat::Json => println!("{}", serde_json::to_string(&languages).unwrap()),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&languages).unwrap()),
    }
}

#[derive(clap::Parser)]
struct Arguments {
    /// Include files and folders that begin with a dot (.). By default, this is false, so
    /// directories such as `.git`, `.vscode`, and `.cargo` are not included, as well as
    /// files such as `.gitignore`. Setting this to true will include these files in the counts.
    #[arg(long, short = 'd')]
    include_dotfiles: bool,

    /// The format of the output. The default is human-readable, which outputs in a pretty
    /// format; But other formats such as JSON and YAML are available for tasks such as
    /// script parsing.
    #[arg(value_enum, long, short, default_value_t = OutputFormat::HumanReadable)]
    output: OutputFormat,

    /// List all files of the specified language. This will only list files which match
    /// the given language, and each file will be listed with its absolute path.
    #[arg(long, short)]
    find: Option<String>,

    /// Languages to exclude (case-insensitive). Language names specified here will not be
    /// counted or displayed.
    #[arg(long, short)]
    exclude: Vec<String>,

    /// Files and directories to include, which are excluded by default. For example, dotfiles,
    /// such as `.git` and `.vscode` are ignored, but you can exclusively include one of them
    /// with something like `splik --include .git`, while still ignoring all other dotfiles.
    /// To include all dotfiles, use `--include-dotfiles`. Additionally, this can be used to
    /// include non-dotfiles that are ignored by default, such as `node_modules`, `target`, etc.
    #[arg(long, short)]
    include: Vec<String>,
}

const IGNORED_DIRECTORIES: &'static [&'static str] = &["node_modules", "target", "dist", "build", "public", "out"];

#[derive(serde::Serialize, PartialEq, Eq)]
struct LanguageInfo {
    name: &'static str,
    files: Vec<String>,
    lines: u32,
    bytes: u64,
}

impl PartialOrd for LanguageInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.bytes.partial_cmp(&self.bytes)
    }
}

impl Ord for LanguageInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.bytes.cmp(&self.bytes)
    }
}

impl LanguageInfo {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            files: Vec::new(),
            lines: 0,
            bytes: 0,
        }
    }
}

#[derive(Default, serde::Serialize)]
struct LanguageList {
    languages: Vec<LanguageInfo>,
}

impl LanguageList {
    fn add_file(&mut self, path: &PathBuf, arguments: &Arguments) {
        if let Some(Ok(extension)) = path.extension().map(|os_str| {
            Ok::<String, std::io::Error>(
                os_str
                    .to_str()
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "OsStr isn't a valid str"))?
                    .to_string(),
            )
        }) {
            if let Some(language) = LANGUAGES.get(&extension) {
                if arguments.exclude.contains(&(*language).to_owned()) {
                    return;
                };

                let info = self.languages.iter_mut().find(|other_language| &other_language.name == language);
                let info = if let Some(info) = info {
                    info
                } else {
                    let value = LanguageInfo::new(language);
                    self.languages.push(value);
                    self.languages.last_mut().unwrap()
                };
                info.lines += std::fs::read(&path).unwrap().lines().count() as u32;
                info.bytes += std::fs::metadata(&path).unwrap().size();
                info.files.push(path.canonicalize().unwrap().to_str().unwrap().to_owned());
            }
        }
    }

    fn sort(&mut self) {
        self.languages.sort();
    }

    fn find(&self, language_name: &str) {
        let language_name = language_name.to_lowercase();
        for file in self
            .languages
            .iter()
            .find(|language| language.name.to_lowercase() == language_name)
            .map(|language| language.files.iter())
            .unwrap_or_else(|| [].iter())
        {
            println!("{file}");
        }
    }

    fn display(&self) {
        // Calculate the total lines/files/bytes
        let mut total_files = 0;
        let mut total_bytes = 0;
        let mut total_lines = 0;
        for language_info in &self.languages {
            total_files += language_info.files.len();
            total_lines += language_info.lines;
            total_bytes += language_info.bytes;
        }

        let mut other_bytes = 0;
        let mut other_files = 0;
        let mut other_lines = 0;

        for language_info in &self.languages {
            let byte_percent = 100.0 * (language_info.bytes as f64) / (total_bytes as f64);

            if byte_percent >= 1. {
                println!(
                    "{}: {} bytes ({}%), {} lines ({}%), {} files ({}%)",
                    language_info.name,
                    language_info.bytes,
                    100 * language_info.bytes / total_bytes,
                    language_info.lines,
                    100 * language_info.lines / total_lines,
                    language_info.files.len(),
                    100 * language_info.files.len() / total_files
                );
            } else {
                other_bytes += language_info.bytes;
                other_files += language_info.files.len();
                other_lines += language_info.lines;
            }
        }

        // Print "other" languages
        if other_bytes != 0 {
            println!(
                "Other: {} bytes ({}%), {} lines ({}%), {} files ({}%)",
                other_bytes,
                format_number(100.0 * (other_bytes as f64) / (total_bytes as f64)),
                other_lines,
                format_number(100.0 * (other_lines as f64) / (total_lines as f64)),
                other_files,
                format_number(100.0 * (other_files as f64) / (total_files as f64)),
            );
        }
    }
}

fn format_number(number: f64) -> String {
    if number >= 1.0 {
        return format!("{}", number as i32);
    }

    format!("{:.2}", number)
}

#[derive(Clone, clap::ValueEnum, Debug)]
enum OutputFormat {
    HumanReadable,
    Json,
    Yaml,
}

fn analyze_directory(directory_name: &str, arguments: &Arguments, languages: &mut LanguageList) {
    let Ok(entries) = std::fs::read_dir(directory_name) else { return };
    for entry in entries {
        // Get the path and skip it if it errors
        let Ok(path) = entry.map(|entry| entry.path()) else { return };
        let pathname = path.canonicalize().unwrap().to_str().unwrap().to_owned();

        // Dotifiles
        if !arguments.include_dotfiles && path.file_name().unwrap().to_str().unwrap().starts_with(".") {
            continue;
        }

        // Directories
        if path.is_dir() {
            if IGNORED_DIRECTORIES.contains(&(&pathname as &str)) && !arguments.include.contains(&pathname) {
                continue;
            }
            analyze_directory(path.to_str().unwrap(), arguments, languages);
        }

        // Files
        if path.is_file() {
            languages.add_file(&path, arguments);
        }
    }
}

const LANGUAGES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "asm" => "Assembly",
    "bash" => "Bash",
    "c" => "C",
    "h" => "C",
    "cpp" => "C++",
    "c++" => "C++",
    "cxx" => "C++",
    "cc" => "C++",
    "hpp" => "C++",
    "hh" => "C++",
    "h++" => "C++",
    "hxx" => "C++",
    "cs" => "C#",
    "f" => "Fortran",
    "for" => "Fortran",
    "f90" => "Fortran",
    "f95" => "Fortran",
    "gleam" => "Gleam",
    "go" => "Go",
    "lhs" => "Haskell",
    "hs" => "Haskell",
    "java" => "Java",
    "js" => "JavaScript",
    "mjs" => "JavaScript",
    "cjs" => "JavaScript",
    "jsx" => "JavaScript React",
    "kt" => "Kotlin",
    "lua" => "Lua",
    "m" => "MATLAB",
    "php" => "PHP",
    "py" => "Python",
    "r" => "R",
    "rb" => "Ruby",
    "rs" => "Rust",
    "sql" => "SQL",
    "svelte" => "Svelte",
    "swift" => "Swift",
    "ts" => "TypeScript",
    "tsx" => "TypeScript React",
    "v" => "V",
    "vue" => "Vue",
    "zig" => "Zig",
};
