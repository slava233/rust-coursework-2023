use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Occurrence {
    File(PathBuf),
    Directory(PathBuf),
    TextFile(PathBuf),
}
