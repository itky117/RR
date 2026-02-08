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
    let sandbox_root = root.join("target").join("tests").join("multi_errors");
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
fn semantic_errors_are_reported_together() {
    let src = r#"
fn main() {
  return a + b;
}
main();
"#;
    let (ok, stdout, _stderr) = run_compile(src, "semantic_multi.rr");
    assert!(!ok, "compile must fail");
    assert!(stdout.contains("semantic validation failed"), "missing aggregate header:\n{}", stdout);
    assert!(stdout.contains("found "), "missing aggregate count:\n{}", stdout);
    assert!(stdout.contains("undefined variable 'a'"), "missing undefined variable a:\n{}", stdout);
    assert!(stdout.contains("undefined variable 'b'"), "missing undefined variable b:\n{}", stdout);
}
