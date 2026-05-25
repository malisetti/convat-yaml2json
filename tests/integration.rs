use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

fn run_binary(stdin: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_convat-yaml2json"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn convat-yaml2json");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "binary failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn json_lines(stdout: &str) -> Vec<Value> {
    stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).expect("valid JSON line"))
        .collect()
}

#[test]
fn single_document_yaml_to_json_line() {
    let input = "name: alice\nage: 30\n";
    let stdout = run_binary(input);
    let lines = json_lines(&stdout);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0]["name"], "alice");
    assert_eq!(lines[0]["age"], 30);
}

#[test]
fn multi_document_yaml_to_json_lines() {
    let input = "---\nname: alice\n---\nname: bob\n";
    let stdout = run_binary(input);
    let lines = json_lines(&stdout);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0]["name"], "alice");
    assert_eq!(lines[1]["name"], "bob");
}

#[test]
fn convert_unit_single_and_multi() {
    let single = convat_yaml2json::convert("greeting: hello\n").unwrap();
    assert_eq!(json_lines(&single).len(), 1);
    assert_eq!(json_lines(&single)[0]["greeting"], "hello");

    let multi = convat_yaml2json::convert("---\nx: 1\n---\nx: 2\n").unwrap();
    let multi_lines = json_lines(&multi);
    assert_eq!(multi_lines.len(), 2);
    assert_eq!(multi_lines[0]["x"], 1);
    assert_eq!(multi_lines[1]["x"], 2);
}
