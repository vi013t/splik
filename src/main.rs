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
    fn add_file(&mut self, path: &PathBuf) {
        if let Some(Ok(extension)) = path.extension().map(|os_str| {
            Ok::<String, std::io::Error>(
                os_str
                    .to_str()
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "OsStr isn't a valid str"))?
                    .to_string(),
            )
        }) {
            if let Some(language) = LANGUAGES.get(&extension) {
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
        let mut total_files = 0;
        let mut total_bytes = 0;
        let mut total_lines = 0;
        for language_info in &self.languages {
            total_files += language_info.files.len();
            total_lines += language_info.lines;
            total_bytes += language_info.bytes;
        }

        for language_info in &self.languages {
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

    #[arg(long)]
    find: Option<String>,
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
            languages.add_file(&path);
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
