use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <path>");
        std::process::exit(1);
    }

    let path = &args[1];
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type().map(|f| f.is_file()).unwrap_or(false) {
                    let file_name = entry.file_name();
                    println!("{}", file_name.to_string_lossy());
                }
            }
        }
    } else {
        eprintln!("Error listing directory contents.");
    }
}
