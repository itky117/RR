# IR Model (HIR and MIR)

## HIR (`src/hir/def.rs`)

HIR represents typed/semi-typed language structure before CFG lowering.

Key entities:

- `HirProgram` -> `HirModule` -> `HirItem`
- `HirFn` with params, attrs, local map, body
- `HirStmt`:
  - `Let`, `Assign`, `If`, `While`, `For`, `Return`, `Break`, `Next`, `Expr`
- `HirExpr`:
  - locals/globals/literals/calls/index/field
  - control expressions (`IfExpr`, `Match`, `Try`)
  - composite literals (`VectorLit`, `ListLit`)
  - lambda-related forms after lowering

Specialized forms:

- `HirTidyCall`/`TidyExpr` for tidy-style operations
- pattern nodes (`HirPat`) for match lowering

## MIR (`src/mir/def.rs`)

MIR is SSA-like, CFG-based, optimization-facing IR.

Core:

- `FnIR`:
  - `blocks: Vec<Block>`
  - `values: Vec<Value>`
  - `entry`, `body_head`
  - `unsupported_dynamic` + `fallback_reasons`
- `Block`:
  - instruction list + terminator
- `ValueKind`:
  - SSA primitives: `Const`, `Phi`, `Param`, `Load`
  - structural primitives: `Len`, `Indices`, `Range`
  - compute: `Binary`, `Unary`, `Call`
  - memory-like access: `Index1D`, `Index2D`
- `Instr`:
  - `Assign`, `Eval`
  - `StoreIndex1D`, `StoreIndex2D`
- `Terminator`:
  - `Goto`, `If`, `Return`, `Unreachable`

## Key Invariants

- Phi nodes must be eliminated before codegen.
- Value/Block IDs must remain valid under transforms.
- Runtime safety validator checks static error cases (e.g., guaranteed division by zero).

## Why MIR Is Central

Most advanced behavior is implemented on MIR:

- SCCP
- GVN/CSE
- LICM
- TCO
- BCE
- Vectorization (`v_opt`)
- De-SSA with parallel copy
