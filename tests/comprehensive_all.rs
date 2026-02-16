mod common;

use common::{compile_rr, normalize, rscript_available, rscript_path, run_rscript};
use std::fs;
use std::path::PathBuf;

#[test]
fn comprehensive_script_is_opt_level_equivalent() {
    let rscript = match rscript_path() {
        Some(p) if rscript_available(&p) => p,
        _ => {
            eprintln!("Skipping comprehensive_all test: Rscript not available.");
            return;
        }
    };

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rr_path = root
        .join("tests")
        .join("golden")
        .join("records_lambda_pipe_try.rr");
    assert!(
        rr_path.exists(),
        "missing test script: {}",
        rr_path.display()
    );

    let out_dir = root.join("target").join("tests").join("comprehensive_all");
    fs::create_dir_all(&out_dir).expect("failed to create target/tests/comprehensive_all");

    let rr_bin = PathBuf::from(env!("CARGO_BIN_EXE_RR"));
    let o0 = out_dir.join("comprehensive_o0.R");
    let o1 = out_dir.join("comprehensive_o1.R");
    let o2 = out_dir.join("comprehensive_o2.R");

    compile_rr(&rr_bin, &rr_path, &o0, "-O0");
    compile_rr(&rr_bin, &rr_path, &o1, "-O1");
    compile_rr(&rr_bin, &rr_path, &o2, "-O2");

    let base = run_rscript(&rscript, &o0);
    let run_o1 = run_rscript(&rscript, &o1);
    let run_o2 = run_rscript(&rscript, &o2);

    assert_eq!(
        base.status, run_o1.status,
        "status mismatch between O0 and O1"
    );
    assert_eq!(
        base.status, run_o2.status,
        "status mismatch between O0 and O2"
    );
    assert_eq!(
        normalize(&base.stdout),
        normalize(&run_o1.stdout),
        "stdout mismatch between O0 and O1"
    );
    assert_eq!(
        normalize(&base.stdout),
        normalize(&run_o2.stdout),
        "stdout mismatch between O0 and O2"
    );
    assert_eq!(
        normalize(&base.stderr),
        normalize(&run_o1.stderr),
        "stderr mismatch between O0 and O1"
    );
    assert_eq!(
        normalize(&base.stderr),
        normalize(&run_o2.stderr),
        "stderr mismatch between O0 and O2"
    );

    assert!(
        !base.stdout.is_empty(),
        "baseline output should not be empty"
    );
    assert!(
        base.stdout.contains("[1] 52"),
        "expected deterministic final checksum marker in output"
    );
}
