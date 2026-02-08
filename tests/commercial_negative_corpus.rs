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
    let sandbox_root = root
        .join("target")
        .join("tests")
        .join("commercial_negative_corpus");
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
        .arg("-O2")
        .output()
        .expect("failed to run RR");

    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

#[test]
fn malformed_and_invalid_programs_fail_without_ice() {
    let corpus: [(&str, &str); 14] = [
        ("lex_bad_char", "fn main() { let x = 1$; return x; } main();"),
        ("lex_unterminated_string", "fn main() { let s = \"abc; return s; } main();"),
        ("parse_missing_expr", "fn main() { let x = ; return x; } main();"),
        ("parse_missing_semicolon", "fn main() { let x = 1L return x; } main();"),
        ("parse_unbalanced", "fn main() { if (1L < 2L) { return 1L; } main();"),
        ("semantic_undef_var", "fn main() { return nope + 1L; } main();"),
        ("semantic_undef_fn", "fn main() { return missing_fn(1L); } main();"),
        ("semantic_arity", "fn f(a,b){ return a+b; } fn main(){ return f(1L); } main();"),
        ("runtime_static_na_cond", "fn main(){ if (NA) { return 1L; } return 0L; } main();"),
        ("runtime_static_div0", "fn main(){ return 1L / 0L; } main();"),
        ("runtime_static_oob_write", "fn main(){ let x = c(1L,2L); x[0L] = 3L; return x; } main();"),
        ("multi_semantic", "fn main(){ return a + b + c; } main();"),
        ("multi_parse", "fn main(){ let x = ; let y = ; return ; } main();"),
        ("multi_runtime", "fn main(){ let x=c(1L,2L); y<-x[0L]; z<-1L/0L; if (NA) { return 1L; } return y+z; } main();"),
    ];

    for (name, src) in corpus {
        let (ok, stdout, _stderr) = run_compile(src, &format!("{name}.rr"));
        assert!(!ok, "case '{name}' must fail");
        assert!(
            stdout.contains("** (RR."),
            "case '{name}' must print formatted RR error:\n{}",
            stdout
        );
        assert!(
            stdout.contains("error["),
            "case '{name}' must include machine-readable error code:\n{}",
            stdout
        );
        assert!(
            !stdout.contains("ICE9001") && !stdout.contains("RR.InternalError"),
            "case '{name}' hit internal compiler error unexpectedly:\n{}",
            stdout
        );
    }
}

#[test]
fn aggregate_diagnostics_report_all_relevant_findings() {
    let src = r#"
fn main() {
  x <- c(1L, 2L);
  y <- x[0L];
  z <- 1L / 0L;
  if (NA) { return 1L; }
  return y + z;
}
main();
"#;
    let (ok, stdout, _stderr) = run_compile(src, "aggregate_all.rr");
    assert!(!ok, "compile must fail");
    assert!(
        stdout.contains("found "),
        "aggregate diagnostics must include count:\n{}",
        stdout
    );
    assert!(
        stdout.contains("condition is statically NA"),
        "missing NA static runtime diagnostic:\n{}",
        stdout
    );
    assert!(
        stdout.contains("division by zero is guaranteed at compile-time"),
        "missing divide-by-zero diagnostic:\n{}",
        stdout
    );
    assert!(
        stdout.contains("out of bounds"),
        "missing index bounds diagnostic:\n{}",
        stdout
    );
}
