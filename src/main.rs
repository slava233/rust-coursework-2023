use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Occurrence {
    File(PathBuf),
    Directory(PathBuf),
    TextFile(PathBuf),
}

fn list_files(path: &Path, find_file: Option<&str>, search_string: Option<&str>, sort: bool) -> Vec<Occurrence> {
    let mut occurrences: Vec<Occurrence> = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir.flatten() {
            let entry_path = entry.path();
            let is_directory = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

            if is_directory {
                occurrences.push(Occurrence::Directory(entry_path.clone()));
                let sub_occurrences = list_files(&entry_path, find_file, search_string, sort);
                occurrences.extend(sub_occurrences);
            } else {
                let file_name = entry_path.file_name().and_then(|n| n.to_str());

                let occurrence = if let Some(name) = file_name {
                    if name.ends_with(".txt") || name.ends_with(".rs") {
                        Occurrence::TextFile(entry_path.clone())
                    } else {
                        Occurrence::File(entry_path.clone())
                    }
                } else {
                    Occurrence::File(entry_path.clone())
                };

                occurrences.push(occurrence);
            }
        }
    }

    if let Some(find_file_name) = find_file {
        occurrences = occurrences
            .into_iter()
            .filter(|occurrence| {
                match occurrence {
                    Occurrence::File(path) | Occurrence::TextFile(path) => {
                        path.to_string_lossy().contains(find_file_name)
                    }
                    _ => false,
                }
            })
            .collect();
    }

    if let Some(search_string) = search_string {
        occurrences = occurrences
            .into_iter()
            .filter(|occurrence| {
                match occurrence {
                    Occurrence::TextFile(path) => {
                        search_in_text_file(&path, search_string)
                    }
                    _ => false,
                }
            })
            .collect();
    }

    if sort {
        occurrences.sort();
    }

    occurrences
}

fn search_in_text_file(file_path: &Path, search_string: &str) -> bool {
    if let Ok(contents) = fs::read_to_string(file_path) {
        return contents.contains(search_string);
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path> [--find <file_name>] [--in-file <search_string>] [--sort] [-f <output_file>]");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);

    let mut find_file: Option<&str> = None;
    let mut search_string: Option<&str> = None;
    let mut sort: bool = false;
    let mut output_file: Option<&str> = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--find" => {
                if i + 1 < args.len() {
                    find_file = Some(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("Error: --find option requires a file name");
                    std::process::exit(1);
                }
            }
            "--in-file" => {
                if i + 1 < args.len() {
                    search_string = Some(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("Error: --in-file option requires a search string");
                    std::process::exit(1);
                }
            }
            "--sort" => {
                sort = true;
                i += 1;
            }
            "-f" => {
                if i + 1 < args.len() {
                    output_file = Some(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("Error: -f option requires a file name");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: Unknown option {}", args[i]);
                std::process::exit(1);
            }
        }
    }

    let occurrences = list_files(path, find_file, search_string, sort);

    if let Some(output_file_name) = output_file {
        let mut file = match fs::File::create(output_file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error creating file {}: {}", output_file_name, e);
                std::process::exit(1);
            }
        };

        for occurrence in occurrences {
            if let Err(e) = writeln!(file, "{:?}", occurrence) {
                eprintln!("Error writing to file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        for occurrence in occurrences {
            println!("{:?}", occurrence);
        }
    }
}
