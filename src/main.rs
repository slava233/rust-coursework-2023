mod file_operations;
mod file_search;
mod occurrence;
mod output_strategies;

use std::env;
use output_strategies::{ConsoleOutput, FileOutput, OutputStrategy};
use std::path::Path;

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
            "-f" => {
                if i + 1 < args.len() {
                    output_file = Some(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("Error: -f option requires a file name");
                    std::process::exit(1);
                }
            }
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
            _ => {
                eprintln!("Error: Unknown option {}", args[i]);
                std::process::exit(1);
            }
        }
    }

    let occurrences = file_search::list_files(path, find_file, search_string, sort);

    let output_strategy: Box<dyn OutputStrategy> = if let Some(output_file_name) = output_file {
        Box::new(FileOutput::new(output_file_name))
    } else {
        Box::new(ConsoleOutput)
    };

    output_strategy.output(&occurrences);
}
