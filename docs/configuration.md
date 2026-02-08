# Configuration

This page lists environment variables recognized by RR codebase.

## CLI and Output

- `RR_FORCE_COLOR`
  - Force ANSI colors in CLI/errors when `NO_COLOR` is not set.
- `NO_COLOR`
  - Disable ANSI colors.
- `RR_VERBOSE_LOG`
  - Force detailed compile progress traces in CLI logger.

## Language Strictness

- `RR_STRICT_LET`
  - Forbid implicit declaration through assignment (`<-` / `=` to undeclared name).
- `RR_STRICT_ASSIGN`
  - Alias trigger for strict-let behavior.

## Optimizer Control

- `RR_VERIFY_EACH_PASS` (default `false`)
  - Run MIR verifier after each pass.
- `RR_OPT_MAX_ITERS` (default `24`)
  - Max per-function optimization iterations.
- `RR_INLINE_MAX_ROUNDS` (default `3`)
  - Max inter-procedural inline rounds.

## Inlining Policy

- `RR_DISABLE_INLINE` (default `false`)
- `RR_INLINE_MAX_BLOCKS` (default `24`)
- `RR_INLINE_MAX_INSTRS` (default `160`)
- `RR_INLINE_MAX_COST` (default `220`)
- `RR_INLINE_MAX_CALLER_INSTRS` (default `480`)
- `RR_INLINE_MAX_TOTAL_INSTRS` (default `900`)
- `RR_INLINE_ALLOW_LOOPS` (default `false`)

## Test and CI Performance Gates

- `RR_PERF_GATE_MS` (default `12000`)
  - O2 compile-time budget for perf gate test.
- `RR_PERF_O2_O1_RATIO` (default `12`)
  - Allowed O2/O1 slowdown ratio in perf gate test.

## Rscript Override Notes

Integration tests use `RRSCRIPT` in test harnesses to override the executable used for direct R calls.
Main RR CLI runtime path currently invokes `Rscript` directly.
