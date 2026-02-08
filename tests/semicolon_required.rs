use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_dir(root: &PathBuf, name: &str) -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    root.join(format!("{}_{}_{}", name, std::process::id(), ts))
}

fn run_compile(source: &str, file_name: &str) -> (bool, String, String) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sandbox_root = root.join("target").join("tests").join("semicolon_required");
    fs::create_dir_all(&sandbox_root).expect("failed to create sandbox root");
    let proj_dir = unique_dir(&sandbox_root, "proj");
    fs::create_dir_all(&proj_dir).expect("failed to create project dir");

    let rr_path = proj_dir.join(file_name);
    let out_path = proj_dir.join("out.R");
    fs::write(&rr_path, source).expect("failed to write source");

    let rr_bin = PathBuf::from(env!("CARGO_BIN_EXE_RR"));
    let output = Command::new(rr_bin)
        .arg(&rr_path)
        .arg("-o")
        .arg(&out_path)
        .arg("--no-runtime")
        .arg("-O1")
        .output()
        .expect("failed to run RR");

    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

#[test]
fn same_line_missing_semicolon_must_fail() {
    let src = r#"
fn main() {
  x <- 1L y <- 2L;
  return x + y;
}
main();
"#;
    let (ok, stdout, _stderr) = run_compile(src, "same_line_missing_semi.rr");
    assert!(!ok, "compile must fail when same-line statement separator is missing");
    assert!(stdout.contains("Missing ';'"), "missing semicolon diagnostic:\n{}", stdout);
}

#[test]
fn newline_separator_without_semicolon_is_allowed() {
    let src = r#"
fn main() {
  x <- 1L
  y <- 2L
  return x + y
}
main();
"#;
    let (ok, _stdout, _stderr) = run_compile(src, "newline_separated.rr");
    assert!(ok, "newline separated statements should compile");
}
