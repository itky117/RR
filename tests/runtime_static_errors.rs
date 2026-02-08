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
    let sandbox_root = root.join("target").join("tests").join("runtime_static_errors");
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
fn static_if_na_condition_must_fail() {
    let src = r#"
fn main() {
  if (NA) { return 1L; } else { return 0L; }
}
main();
"#;
    let (ok, stdout, _stderr) = run_compile(src, "if_na.rr");
    assert!(!ok, "compile must fail for statically NA condition");
    assert!(stdout.contains("** (RR.RuntimeError)"), "missing runtime error header:\n{}", stdout);
    assert!(
        stdout.contains("condition is statically NA"),
        "missing NA condition detail:\n{}",
        stdout
    );
}

#[test]
fn static_divide_by_zero_must_fail() {
    let src = r#"
fn main() {
  return 1L / 0L;
}
main();
"#;
    let (ok, stdout, _stderr) = run_compile(src, "div_zero.rr");
    assert!(!ok, "compile must fail for guaranteed divide by zero");
    assert!(stdout.contains("** (RR.RuntimeError)"), "missing runtime error header:\n{}", stdout);
    assert!(
        stdout.contains("division by zero is guaranteed at compile-time"),
        "missing divide-by-zero detail:\n{}",
        stdout
    );
}

#[test]
fn static_invalid_write_index_must_fail() {
    let src = r#"
fn main() {
  x <- c(1L, 2L, 3L);
  x[0L] <- 10L;
  return x;
}
main();
"#;
    let (ok, stdout, _stderr) = run_compile(src, "bad_write_index.rr");
    assert!(!ok, "compile must fail for statically invalid write index");
    assert!(stdout.contains("** (RR.RuntimeError)"), "missing runtime error header:\n{}", stdout);
    assert!(
        stdout.contains("out of bounds"),
        "missing index-out-of-bounds detail:\n{}",
        stdout
    );
}

#[test]
fn multiple_static_runtime_errors_are_reported_together() {
    let src = r#"
fn main() {
  x <- c(1L, 2L);
  y <- x[0L];
  z <- 1L / 0L;
  if (NA) { return 1L; }
  return z + y;
}
main();
"#;
    let (ok, stdout, _stderr) = run_compile(src, "runtime_multi.rr");
    assert!(!ok, "compile must fail");
    assert!(
        stdout.contains("runtime safety validation failed"),
        "missing aggregate runtime header:\n{}",
        stdout
    );
    assert!(stdout.contains("found "), "missing aggregate count:\n{}", stdout);
    assert!(stdout.contains("condition is statically NA"), "missing NA condition error:\n{}", stdout);
    assert!(
        stdout.contains("division by zero is guaranteed at compile-time"),
        "missing division-by-zero error:\n{}",
        stdout
    );
    assert!(stdout.contains("out of bounds"), "missing index error:\n{}", stdout);
}
