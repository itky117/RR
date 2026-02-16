# Legacy Code

This folder contains compiler paths that are intentionally not part of the
active production pipeline.

- `ir/`: legacy IR experiment (`src/ir/*` moved here)
- `mir/build.rs` (`src/legacy/mir/build.rs`): legacy MIR builder based on legacy IR
- `codegen/emit.rs` (`src/legacy/codegen/emit.rs`): legacy IR emitter

Current production path is:

`syntax -> hir -> mir -> codegen/mir_emit`
