use std::fs;
use std::path::Path;

pub fn search_in_text_file(file_path: &Path, search_string: &str) -> bool {
    if let Ok(contents) = fs::read_to_string(file_path) {
        return contents.contains(search_string);
    }
    false
}
