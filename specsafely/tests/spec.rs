use std::path::PathBuf;
use std::process::Command;

const SAFELY_PATH: &'static str = "../target/release/safely";

#[test]
fn it_displays_help() {
    let output = Command::new(PathBuf::from(SAFELY_PATH))
        .arg("--help")
        .output()
        .expect("failed to execute process");

    assert!(String::from_utf8(output.stdout).unwrap().contains("USAGE:"));
}

#[test]
fn it_fails_getting_state_for_container_that_does_not_exist() {
    let output = Command::new(PathBuf::from(SAFELY_PATH))
        .args(&["state", "foo"])
        .output()
        .expect("failed to execute command");

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("no such container"));
}

#[test]
fn it_fails_killing_a_container_that_does_not_exist() {
    let output = Command::new(PathBuf::from(SAFELY_PATH))
        .args(&["kill", "foo"])
        .output()
        .expect("failed to execute command");

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("no such container"));
}

#[test]
fn it_fails_deleting_a_container_that_does_not_exist() {
    let output = Command::new(PathBuf::from(SAFELY_PATH))
        .args(&["delete", "foo"])
        .output()
        .expect("failed to execute command");

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("no such container"));
}

#[test]
fn it_fails_starting_a_container_that_does_not_exist() {
    let output = Command::new(PathBuf::from(SAFELY_PATH))
        .args(&["start", "foo"])
        .output()
        .expect("failed to execute command");

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("no such container"));
}
