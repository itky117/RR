# Compiler Pipeline

RR compile path in `src/compiler/pipeline.rs::compile()` uses a 6-step pipeline.

CLI entrypoints in `src/main.rs` call this pipeline API.

## High-Level Flow

`RR source`  
`-> Lexer/Parser (AST)`  
`-> HIR Lowering`  
`-> HIR Desugar`  
`-> MIR Lowering (SSA-like CFG)`  
`-> MIR Validation`  
`-> Tachyon Engine (opt or stabilization)`  
`-> MIR Structurizer + R emission`  
`-> Runtime injection`  
`-> self-contained .R output`

## Step-by-Step (matches CLI progress output)

1. `Source Analysis`
- Parse each module.
- Resolve imports.
- Lower AST to HIR.
- Collect global symbol table and arity info.

2. `Canonicalization`
- Run HIR desugaring (`src/hir/desugar.rs`).

3. `SSA Graph Synthesis`
- Lower each HIR function to MIR (`src/mir/lower_hir.rs`).
- Build blocks, SSA values, phi placeholders/backpatching.

4. `Tachyon Optimization` or `Tachyon Stabilization`
- `-O1/-O2`: full Tachyon optimization pipeline.
- `-O0`: stabilization-only path (still includes mandatory De-SSA before codegen).

5. `R Code Emission`
- Structurize CFG into high-level control shapes (`src/mir/structurizer.rs`).
- Emit R code from structured blocks (`src/codegen/mir_emit.rs`).
- Build RR-to-R source map entries.

6. `Runtime Injection`
- Prepend embedded runtime (`src/runtime/mod.rs`).
- Set source label (`rr_set_source(...)`).
- Append top-level synthetic invocations.

## Required Validation Stages

RR validates before and after critical MIR phases:

- Semantic validation (`validate_program`)
- Runtime-safety static validation (`validate_runtime_safety`)
- MIR structural verifier (`src/mir/verify.rs`)

Multiple diagnostics are aggregated and reported together when possible.

## Error Flow

The compiler pipeline returns `RR<T>` (`Result<T, RRException>`):

- pipeline layers return structured errors
- CLI decides final process exit code
- the compile core itself does not terminate the process directly

## Legacy IR Path

`src/legacy/ir/*` still exists as a legacy/experimental layer.
Main production pipeline uses HIR -> MIR -> codegen path.
