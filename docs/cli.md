# CLI Reference

## Commands

RR supports three entry styles:

```bash
RR <input.rr> [options]
RR run [main.rr|dir|.] [options]
RR build [dir|file.rr] [options]
```

`cargo run -- ...` can be used in place of direct `RR ...`.

## Options

- `-O0`, `-O1`, `-O2`
- `-o0`, `-o1`, `-o2` (accepted aliases)
- `-o <file>`:
  - legacy mode: output file path
  - `build` mode: alias for `--out-dir`
- `--out-dir <dir>`: build output directory
- `--keep-r`: keep generated `.gen.R` when using `run`
- `--no-runtime`: compile only (legacy mode)

## `run` Resolution Rules

Input can be:

- `.`
- a directory
- a `.rr` file

If directory or `.`, RR looks for `main.rr` in that directory.

## `build` Resolution Rules

Input can be:

- directory: recursively compile all `.rr` files
- single `.rr` file: compile one file into output directory

## Exit Behavior

- `0`: success
- non-zero: parse/semantic/runtime/compiler failure

Errors are formatted via RR diagnostics (colored when terminal supports ANSI or `RR_FORCE_COLOR` is set).
Compiler execution is delegated to `src/compiler/pipeline.rs`; CLI handles the final exit code.
