use std::{io::BufRead, os::unix::fs::MetadataExt};

use clap::Parser as _;

fn main() {
    let arguments = Arguments::parse();

    let source = if std::fs::read_dir("src").is_ok() { "./src" } else { "." };
    let mut languages = LanguageList::default();

    analyze_directory(source, &arguments, &mut languages);

    languages.sort();

    match arguments.output {
        OutputFormat::HumanReadable => languages.display(),
        OutputFormat::Json => println!("{}", serde_json::to_string(&languages).unwrap()),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&languages).unwrap()),
    }
}

const IGNORED_DIRECTORIES: &'static [&'static str] = &["node_modules", "target", "dist", "build", "public", "out"];

#[derive(serde::Serialize, PartialEq, Eq)]
struct LanguageInfo {
    name: &'static str,
    files: u32,
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
            files: 0,
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
    fn add_file(&mut self, name: &'static str, lines: u32, bytes: u64) {
        let info = self.languages.iter_mut().find(|language| language.name == name);
        let info = if let Some(info) = info {
            info
        } else {
            let value = LanguageInfo::new(name);
            self.languages.push(value);
            self.languages.last_mut().unwrap()
        };
        info.lines += lines;
        info.files += 1;
        info.bytes += bytes;
    }

    fn sort(&mut self) {
        self.languages.sort();
    }

    fn display(&self) {
        let mut total_files = 0;
        let mut total_bytes = 0;
        let mut total_lines = 0;
        for language_info in &self.languages {
            total_files += language_info.files;
            total_lines += language_info.lines;
            total_bytes += language_info.bytes;
        }

        for language_info in &self.languages {
            println!(
                "{}: {} bytes ({}%), {} lines ({}%), {} files ({}%)",
                language_info.name,
                language_info.bytes,
                language_info.bytes / total_bytes * 100,
                language_info.lines,
                language_info.lines / total_lines * 100,
                language_info.files,
                language_info.files / total_files * 100,
            );
        }
    }
}

#[derive(Clone, clap::ValueEnum, Debug)]
enum OutputFormat {
    HumanReadable,
    Json,
    Yaml,
}

#[derive(clap::Parser)]
struct Arguments {
    #[arg(long)]
    ignore_dotfiles: bool,

    #[arg(value_enum, long, default_value_t = OutputFormat::HumanReadable)]
    output: OutputFormat,
}

fn analyze_directory(directory_name: &str, arguments: &Arguments, languages: &mut LanguageList) {
    let Ok(entries) = std::fs::read_dir(directory_name) else { return };
    for entry in entries {
        let Ok(path) = entry.map(|entry| entry.path()) else { return };

        // Dotifiles
        if arguments.ignore_dotfiles && path.starts_with(".") {
            continue;
        }

        // Directories
        if path.is_dir() {
            if IGNORED_DIRECTORIES.contains(&path.to_str().unwrap()) {
                continue;
            }
            analyze_directory(path.to_str().unwrap(), arguments, languages);
        }

        // Files
        if path.is_file() {
            if let Some(Ok(extension)) = path.extension().map(|os_str| {
                Ok::<String, std::io::Error>(
                    os_str
                        .to_str()
                        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "OsStr isn't a valid str"))?
                        .to_string(),
                )
            }) {
                if let Some(language) = LANGUAGES.get(&extension) {
                    languages.add_file(
                        language,
                        std::fs::read(&path).unwrap().lines().count() as u32,
                        std::fs::metadata(&path).unwrap().size(),
                    )
                }
            }
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
    "gleam" => "Gleam",
    "go" => "Go",
    "java" => "Java",
    "js" => "JavaScript",
    "mjs" => "JavaScript",
    "cjs" => "JavaScript",
    "jsx" => "JavaScript React",
    "kt" => "Kotlin",
    "lua" => "Lua",
    "php" => "PHP",
    "py" => "Python",
    "r" => "R",
    "rb" => "Ruby",
    "rs" => "Rust",
    "sql" => "SQL",
    "svelte" => "Svelte",
    "ts" => "TypeScript",
    "tsx" => "TypeScript React",
    "v" => "V",
    "vue" => "Vue",
    "zig" => "Zig",
};
