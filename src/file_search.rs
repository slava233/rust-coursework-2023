use crate::file_operations::search_in_text_file;
use crate::occurrence::Occurrence;

use std::fs;
use std::path::Path;

pub fn list_files(path: &Path, find_file: Option<&str>, search_string: Option<&str>, sort: bool) -> Vec<Occurrence> {
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
        occurrences.retain(|occurrence| {
            match occurrence {
                Occurrence::File(path) | Occurrence::TextFile(path) => {
                    path.to_string_lossy().contains(find_file_name)
                }
                _ => false,
            }
        });
    }

    if let Some(search_string) = search_string {
        occurrences.retain(|occurrence| {
            match occurrence {
                Occurrence::TextFile(path) => search_in_text_file(&path, search_string),
                _ => false,
            }
        });
    }

    if sort {
        occurrences.sort();
    }

    occurrences
}
