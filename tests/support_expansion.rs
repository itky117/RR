use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct RunResult {
    status: i32,
    stdout: String,
    stderr: String,
}

fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n")
}

fn rscript_path() -> Option<String> {
    if let Ok(path) = std::env::var("RRSCRIPT") {
        if !path.trim().is_empty() {
            return Some(path);
        }
    }
    Some("Rscript".to_string())
}

fn rscript_available(path: &str) -> bool {
    Command::new(path)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_rscript(path: &str, script: &Path) -> RunResult {
    let output = Command::new(path)
        .arg("--vanilla")
        .arg(script)
        .output()
        .expect("failed to execute Rscript");
    RunResult {
        status: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    }
}

fn compile_rr(rr_bin: &Path, rr_path: &Path, out_path: &Path, level: &str) {
    let status = Command::new(rr_bin)
        .arg(rr_path)
        .arg("-o")
        .arg(out_path)
        .arg("--no-runtime")
        .arg(level)
        .status()
        .expect("failed to run RR compiler");
    assert!(
        status.success(),
        "RR compile failed for {} ({})",
        rr_path.display(),
        level
    );
}

#[test]
fn unary_field_and_seq_len_for_are_supported() {
    let rscript = match rscript_path() {
        Some(p) if rscript_available(&p) => p,
        _ => {
            eprintln!("Skipping support expansion test: Rscript not available.");
            return;
        }
    };

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_dir = root.join("target").join("tests").join("support_expansion");
    fs::create_dir_all(&out_dir).expect("failed to create target/tests/support_expansion");
    let rr_bin = PathBuf::from(env!("CARGO_BIN_EXE_RR"));

    let rr_src = r#"
fn t_unary_field() {
    let a = -5;
    let b = !FALSE;
    let rec = {x: 1, y: 2};
    rec.x = 10;
    print(a);
    print(b);
    print(rec.x);
    print(rec.y);
    return rec.x + rec.y;
}

fn t_seq_len_sum(n) {
    let s = 0;
    for (i in seq_len(n)) {
        s = s + i;
    }
    return s;
}

print(t_unary_field());
print(t_seq_len_sum(5));
"#;

    let rr_path = out_dir.join("support_expansion.rr");
    fs::write(&rr_path, rr_src).expect("failed to write source");

    let o0 = out_dir.join("support_expansion_o0.R");
    let o1 = out_dir.join("support_expansion_o1.R");
    let o2 = out_dir.join("support_expansion_o2.R");

    compile_rr(&rr_bin, &rr_path, &o0, "-O0");
    compile_rr(&rr_bin, &rr_path, &o1, "-O1");
    compile_rr(&rr_bin, &rr_path, &o2, "-O2");

    let base = run_rscript(&rscript, &o0);
    let run_o1 = run_rscript(&rscript, &o1);
    let run_o2 = run_rscript(&rscript, &o2);

    assert_eq!(base.status, run_o1.status, "status mismatch O0 vs O1");
    assert_eq!(base.status, run_o2.status, "status mismatch O0 vs O2");
    assert_eq!(
        normalize(&base.stdout),
        normalize(&run_o1.stdout),
        "stdout mismatch O0 vs O1"
    );
    assert_eq!(
        normalize(&base.stdout),
        normalize(&run_o2.stdout),
        "stdout mismatch O0 vs O2"
    );
    assert_eq!(
        normalize(&base.stderr),
        normalize(&run_o1.stderr),
        "stderr mismatch O0 vs O1"
    );
    assert_eq!(
        normalize(&base.stderr),
        normalize(&run_o2.stderr),
        "stderr mismatch O0 vs O2"
    );

    let expected = "[1] -5\n[1] TRUE\n[1] 10\n[1] 2\n[1] 12\n[1] 15\n";
    assert_eq!(
        normalize(&base.stdout),
        expected,
        "unexpected baseline semantics"
    );
}
