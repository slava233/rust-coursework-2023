use std::fs;
use std::path::Path;

pub fn search_in_text_file(file_path: &Path, search_string: &str) -> bool {
    if let Ok(contents) = fs::read_to_string(file_path) {
        return contents.contains(search_string);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_search_in_text_file() {
        let file_path = "Hello.txt";
        let content = "Hello, this is a test file!";
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(content.as_bytes()).expect("Failed to write to file");

        let path = Path::new(file_path);
        assert!(search_in_text_file(&path, "Hello"));
        assert!(!search_in_text_file(&path, "World"));

        fs::remove_file(file_path).expect("Failed to remove file");
    }
}
