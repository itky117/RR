use crate::mir::*;

/// Checks if a statement has side effects.
pub fn stmt_is_pure(stmt: &Instr, fn_ir: &FnIR) -> bool {
    match stmt {
        Instr::Assign { src, .. } => rvalue_is_pure(*src, fn_ir),
        Instr::Eval { val, .. } => rvalue_is_pure(*val, fn_ir),
        Instr::StoreIndex1D { .. } => false, // Memory write is a side effect
        Instr::StoreIndex2D { .. } => false, // Memory write is a side effect
    }
}

/// Checks if an Rvalue (ValueKind) is pure.
pub fn rvalue_is_pure(vid: ValueId, fn_ir: &FnIR) -> bool {
    let val = &fn_ir.values[vid];
    match &val.kind {
        ValueKind::Const(_) => true,
        ValueKind::Binary { lhs, rhs, .. } => {
            rvalue_is_pure(*lhs, fn_ir) && rvalue_is_pure(*rhs, fn_ir)
        }
        ValueKind::Unary { rhs, .. } => rvalue_is_pure(*rhs, fn_ir),
        ValueKind::Phi { args } => args.iter().all(|(v, _)| rvalue_is_pure(*v, fn_ir)),
        ValueKind::Call { callee, args, .. } => {
            if call_is_pure(callee) {
                args.iter().all(|a| rvalue_is_pure(*a, fn_ir))
            } else {
                false
            }
        }
        ValueKind::Len { base } => rvalue_is_pure(*base, fn_ir),
        ValueKind::Indices { base } => rvalue_is_pure(*base, fn_ir),
        ValueKind::Range { start, end } => {
            rvalue_is_pure(*start, fn_ir) && rvalue_is_pure(*end, fn_ir)
        }
        ValueKind::Index1D { base, idx, .. } => {
            rvalue_is_pure(*base, fn_ir) && rvalue_is_pure(*idx, fn_ir)
        }
        ValueKind::Index2D { base, r, c } => {
            rvalue_is_pure(*base, fn_ir) && rvalue_is_pure(*r, fn_ir) && rvalue_is_pure(*c, fn_ir)
        }
        _ => false, // Conservative default
    }
}

/// Checks if a function call (by name) is pure based on a whitelist.
pub fn call_is_pure(callee: &str) -> bool {
    match callee {
        // Built-in pure functions
        "length" | "seq_len" | "seq_along" | "abs" | "sqrt" | "sin" | "cos" | "tan" | "log"
        | "exp" | "c" | "sum" | "mean" | "var" | "sd" | "min" | "max" | "prod" | "colSums"
        | "rowSums" | "%*%" | "crossprod" | "tcrossprod" | "is.na" | "is.finite"
        | "rr_field_get" | "rr_field_exists" | "rr_list_rest" | "rr_named_list"
        | "rr_row_sum_range" | "rr_col_sum_range" => true,
        // Comparison/Logical helpers that might be lowered as calls
        "rr_bool" => true,
        _ => false,
    }
}

/// Checks if an entire basic block is effect-free (excluding terminator).
pub fn block_is_pure(bid: BlockId, fn_ir: &FnIR) -> bool {
    let block = &fn_ir.blocks[bid];
    block.instrs.iter().all(|i| stmt_is_pure(i, fn_ir))
}

/// Checks if a loop is pure (no side effects in any of its body blocks).
pub fn loop_is_pure(fn_ir: &FnIR, body: &std::collections::HashSet<BlockId>) -> bool {
    for &bid in body {
        if !block_is_pure(bid, fn_ir) {
            return false;
        }
        // Also check if terminator has side effects (usually not, but If/Goto are pure)
        let block = &fn_ir.blocks[bid];
        match &block.term {
            Terminator::Return(Some(v)) => {
                if !rvalue_is_pure(*v, fn_ir) {
                    return false;
                }
            }
            Terminator::If { cond, .. } => {
                if !rvalue_is_pure(*cond, fn_ir) {
                    return false;
                }
            }
            Terminator::Goto(_) | Terminator::Return(None) | Terminator::Unreachable => {}
        }
    }
    true
}
