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

fn compile_to(rr_path: &PathBuf, out_path: &PathBuf, level: &str) {
    let rr_bin = PathBuf::from(env!("CARGO_BIN_EXE_RR"));
    let status = Command::new(rr_bin)
        .arg(rr_path)
        .arg("-o")
        .arg(out_path)
        .arg("--no-runtime")
        .arg(level)
        .status()
        .expect("failed to run RR");
    assert!(status.success(), "compile failed at {}", level);
}

#[test]
fn codegen_is_deterministic_for_identical_input() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sandbox_root = root
        .join("target")
        .join("tests")
        .join("commercial_determinism");
    fs::create_dir_all(&sandbox_root).expect("failed to create sandbox root");
    let proj_dir = unique_dir(&sandbox_root, "proj");
    fs::create_dir_all(&proj_dir).expect("failed to create project dir");

    let src = r#"
fn helper(x) {
  return (x * 2L) + 1L;
}

fn main() {
  let x = seq_along(20L);
  let y = seq_along(20L);
  for (i in 1L..length(x)) {
    y[i] = helper(x[i]);
  }
  print(sum(y));
}
main();
"#;
    let rr_path = proj_dir.join("determinism.rr");
    fs::write(&rr_path, src).expect("failed to write source");

    for level in ["-O0", "-O1", "-O2"] {
        let out_a = proj_dir.join(format!("det_a_{}.R", level.trim_start_matches('-')));
        let out_b = proj_dir.join(format!("det_b_{}.R", level.trim_start_matches('-')));
        compile_to(&rr_path, &out_a, level);
        compile_to(&rr_path, &out_b, level);

        let a = fs::read(&out_a).expect("failed to read output A");
        let b = fs::read(&out_b).expect("failed to read output B");
        assert_eq!(
            a, b,
            "non-deterministic codegen detected for {}",
            level
        );
    }
}

