use std::process::Command;

#[test]
pub fn test_search_and_output_to_console() {
    let output = Command::new("cargo")
        .args(&["run", "--", ".", "--in-file", "test", "--sort"])
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());
}

#[test]
pub fn test_search_and_output_to_file() {
    let output_file = "output.txt";

    let output = Command::new("cargo")
        .args(&["run", "--", ".", "--in-file", "test", "--sort", "-f", output_file])
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());
    assert!(std::path::Path::new(output_file).exists());

    std::fs::remove_file(output_file).expect("Failed to remove output file");
}
