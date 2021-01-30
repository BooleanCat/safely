use std::path::Path;
use std::process::Command;

#[test]
fn it_displays_help() {
    let output = Command::new(
        Path::new("..")
            .join("target")
            .join("release")
            .join("safely"),
    )
    .arg("--help")
    .output()
    .expect("failed to execute process");

    assert!(String::from_utf8(output.stdout).unwrap().contains("USAGE:"));
}
