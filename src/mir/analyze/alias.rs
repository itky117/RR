
use crate::mir::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AliasClass {
    Var(VarId),
    Fresh(ValueId),
    Unknown,
}

pub fn alias_class_for_base(fn_ir: &FnIR, base: ValueId) -> AliasClass {
    match &fn_ir.values[base].kind {
        ValueKind::Load { var } => AliasClass::Var(var.clone()),
        ValueKind::Param { index } => {
            if let Some(name) = fn_ir.params.get(*index) {
                AliasClass::Var(name.clone())
            } else {
                AliasClass::Unknown
            }
        }
        ValueKind::Call { callee, .. } => {
            if call_returns_fresh(callee) {
                AliasClass::Fresh(base)
            } else {
                AliasClass::Unknown
            }
        }
        ValueKind::Binary { .. }
        | ValueKind::Unary { .. }
        | ValueKind::Range { .. }
        | ValueKind::Indices { .. }
        | ValueKind::Len { .. } => AliasClass::Fresh(base),
        _ => AliasClass::Unknown,
    }
}

pub fn aliases(a: &AliasClass, b: &AliasClass) -> bool {
    match (a, b) {
        (AliasClass::Unknown, _) | (_, AliasClass::Unknown) => true,
        _ => a == b,
    }
}

fn call_returns_fresh(callee: &str) -> bool {
    matches!(
        callee,
        "c"
            | "seq_len"
            | "seq_along"
            | "rr_indices"
            | "rr_range"
            | "matrix"
            | "colSums"
            | "rowSums"
            | "%*%"
            | "crossprod"
            | "tcrossprod"
    )
}
