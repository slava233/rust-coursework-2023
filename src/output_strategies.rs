use crate::occurrence::Occurrence;

use std::fs;
use std::io::Write;

pub trait OutputStrategy {
    fn output(&self, occurrences: &[Occurrence]);
}

pub struct ConsoleOutput;

impl OutputStrategy for ConsoleOutput {
    fn output(&self, occurrences: &[Occurrence]) {
        for occurrence in occurrences {
            println!("{:?}", occurrence);
        }
    }
}

pub struct FileOutput<'a> {
    file_name: &'a str,
}

impl<'a> FileOutput<'a> {
    pub fn new(file_name: &'a str) -> Self {
        FileOutput { file_name }
    }
}

impl OutputStrategy for FileOutput<'_> {
    fn output(&self, occurrences: &[Occurrence]) {
        let mut file = match fs::File::create(self.file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error creating file {}: {}", self.file_name, e);
                std::process::exit(1);
            }
        };

        for occurrence in occurrences {
            if let Err(e) = writeln!(file, "{:?}", occurrence) {
                eprintln!("Error writing to file: {}", e);
                std::process::exit(1);
            }
        }
    }
}
