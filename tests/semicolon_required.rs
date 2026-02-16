mod common;

use common::run_compile_case;

fn run_compile(source: &str, file_name: &str) -> (bool, String, String) {
    run_compile_case("semicolon_required", source, file_name, "-O1", &[])
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
    assert!(
        !ok,
        "compile must fail when same-line statement separator is missing"
    );
    assert!(
        stdout.contains("Missing ';'"),
        "missing semicolon diagnostic:\n{}",
        stdout
    );
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
