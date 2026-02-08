# Tachyon Engine

`TachyonEngine` is the MIR optimizer (`src/mir/opt.rs`).

## Optimization Levels

- `-O0`: no aggressive optimization, but still runs mandatory codegen stabilization (including De-SSA)
- `-O1`: optimized pipeline
- `-O2`: same pipeline with stronger opportunities from analysis/rewrites

## Program-Level Strategy

1. Optimize each function independently.
2. Run inter-procedural inlining rounds (bounded).
3. Run De-SSA globally before emission.
4. Cleanup after De-SSA.

## Function-Level Iterative Passes

Core loop (bounded by `RR_OPT_MAX_ITERS`):

1. Structural transforms
- vectorization (`v_opt`)
- tail-call optimization (`tco`)
- immediate cleanup (`simplify_cfg` + `dce`) if changed

2. Canonical optimization passes
- `simplify_cfg`
- SCCP (`sccp`)
- intrinsics rewrite (`intrinsics`)
- GVN/CSE (`gvn`)
- simplify (`simplify`)
- DCE (`dce`)
- loop optimizer (`loop_opt`)
- LICM (`licm`)
- fresh allocation tuning (`fresh_alloc`)
- bounds-check elimination (`bce`)

3. Always verify MIR invariants at key boundaries.

## Inlining Controls

Inlining is cost-model driven and constrained by environment policy.

Defaults from `src/mir/opt/inline.rs`:

- `RR_INLINE_MAX_BLOCKS=24`
- `RR_INLINE_MAX_INSTRS=160`
- `RR_INLINE_MAX_COST=220`
- `RR_INLINE_MAX_CALLER_INSTRS=480`
- `RR_INLINE_MAX_TOTAL_INSTRS=900`
- `RR_INLINE_ALLOW_LOOPS=false`
- `RR_DISABLE_INLINE=false` (set true-like value to disable)
- `RR_INLINE_MAX_ROUNDS=3` (from `src/mir/opt.rs`)

## Vectorization Coverage (current implementation)

Implemented pattern families include:

- elementwise map
- conditional map
- shifted map
- recurrence add-constant
- reduction (sum/prod/min/max)
- call-map with builtin/user whitelist
- selected 2D row/column map and reduction patterns

Vectorization remains pattern-based, not arbitrary polyhedral scheduling.

## De-SSA and Parallel Copy

De-SSA is mandatory before codegen:

- phi elimination via parallel copy
- critical-edge handling
- sequentialization with temporaries for cycles

Codegen assumes phi-free MIR and will error on remaining phi nodes.
