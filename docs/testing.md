# Testing and Quality Gates

RR includes unit/integration/golden/perf/fuzz coverage.

## Run All Tests

```bash
cargo test -q
```

## Test Families

Representative integration suites under `tests/`:

- syntax and parser recovery:
  - `syntax_errors.rs`
  - `parse_multi_errors.rs`
  - `semicolon_required.rs`
- semantic/runtime static validation:
  - `semantic_errors.rs`
  - `runtime_static_errors.rs`
  - `multi_errors.rs`
  - `commercial_negative_corpus.rs`
- language support:
  - `support_expansion.rs`
  - `lambda_closure.rs`
  - `mir_lowering_loop_match.rs`
- optimization correctness:
  - `vectorization_extended.rs`
  - `benchmark_vectorization.rs`
  - `bce_shifted_index.rs`
  - `opt_level_equivalence.rs`
- CLI behavior:
  - `cli_commands.rs`
- stress and determinism:
  - `commercial_determinism.rs`
  - `commercial_stress_differential.rs`
- performance gate:
  - `perf_regression_gate.rs`

## Golden Semantics

`tests/golden.rs` compares RR-compiled output against R execution for `.rr` cases in `tests/golden/`.

Requirements:

- `Rscript` available in PATH

If unavailable, golden tests skip automatically.

## Fuzzing

Targets:

- `fuzz/fuzz_targets/parser.rs`
- `fuzz/fuzz_targets/pipeline.rs`

Run:

```bash
cargo install cargo-fuzz --locked
cargo fuzz run parser fuzz/corpus/parser -- -max_total_time=60
cargo fuzz run pipeline fuzz/corpus/pipeline -- -max_total_time=60
```

## CI

GitHub Actions workflow `/.github/workflows/ci.yml` runs:

- full test job with R installed
- parser/pipeline fuzz smoke job
