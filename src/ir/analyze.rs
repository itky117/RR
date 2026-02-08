use crate::syntax::ast::*;
use crate::ir::*;
use std::collections::HashMap;
use crate::syntax::ast::Lit;

pub struct Analyzer {
    // Variable map: name -> (type, shape, facts).
    var_types: HashMap<String, (Ty, Shape, Facts)>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self { var_types: HashMap::new() }
    }

    pub fn analyze_program(&mut self, prog: &mut IRProgram) {
        for stmt in &mut prog.stmts {
            self.analyze_stmt(stmt);
        }
    }

    fn analyze_stmt(&mut self, stmt: &mut IRStmt) {
        match &mut stmt.kind {
            IRStmtKind::Assign { target, value } => {
                self.analyze_expr(value);
                self.analyze_lvalue(target);
                // Propagate to LValue name (only if simple assignment)
                if let IRLValue::Name(n) = target {
                    self.var_types.insert(n.clone(), (value.ty.clone(), value.shape.clone(), value.facts.clone()));
                }
            },
            IRStmtKind::ExprStmt { expr } => { self.analyze_expr(expr); },
            IRStmtKind::If { cond, then_blk, else_blk } => {
                self.analyze_expr(cond);
                // Flow-insensitive for branch updates.
                for s in then_blk { self.analyze_stmt(s); }
                if let Some(blk) = else_blk {
                    for s in blk { self.analyze_stmt(s); }
                }
            },
            IRStmtKind::While { cond, body } => {
                self.analyze_expr(cond);
                for s in body { self.analyze_stmt(s); }
            },
            IRStmtKind::For { var, seq, body } => {
                self.analyze_expr(seq);
                
                // --- Loop Variable Facts ---
                // If seq is `rr_indices(x)`, then `var` is Int, Scalar, NonNeg.
                if let IRExprKind::RrIndices { .. } = &seq.kind {
                    let mut facts = Facts::empty();
                    facts.add(Facts::NON_NEG | Facts::INT_SCALAR);
                    self.var_types.insert(var.clone(), (Ty::Int, Shape::Scalar, facts));
                } else if let IRExprKind::RrRange { .. } = &seq.kind {
                     // rr_range produces Int vector. elements are Int Scalar.
                     // Range can be negative, so only Ty::Int
                     let mut facts = Facts::empty();
                     facts.add(Facts::INT_SCALAR);
                     self.var_types.insert(var.clone(), (Ty::Int, Shape::Scalar, facts));
                }
                
                for s in body { self.analyze_stmt(s); }
            },
            IRStmtKind::FnDecl { body, .. } => {
                // New scope ideal, but just recurse
                for s in body { self.analyze_stmt(s); }
            },
            IRStmtKind::Return { value } => {
                if let Some(e) = value { self.analyze_expr(e); }
            }
        }
    }

    fn analyze_lvalue(&mut self, lv: &mut IRLValue) {
        match lv {
            IRLValue::Name(_) => {}, // Name looked up on use, here we set it (handled in stmt)
            IRLValue::Index1D { base, idx } => {
                self.analyze_expr(base);
                self.analyze_expr(idx);
            },
            IRLValue::Index2D { base, r, c } => {
                self.analyze_expr(base);
                self.analyze_expr(r);
                self.analyze_expr(c);
            }
        }
    }

    fn analyze_expr(&mut self, expr: &mut IRExpr) {
        match &mut expr.kind {
            IRExprKind::Lit(l) => {
                match l {
                    Lit::Int(i) => {
                        expr.ty = Ty::Int; 
                        expr.shape = Shape::Scalar;
                        expr.facts.add(Facts::INT_SCALAR);
                        if *i >= 0 { expr.facts.add(Facts::NON_NEG); }
                    },
                    Lit::Float(f) => {
                        expr.ty = Ty::Float;
                        expr.shape = Shape::Scalar;
                        if *f >= 0.0 { expr.facts.add(Facts::NON_NEG); }
                    },
                    Lit::Bool(_) => {
                        expr.ty = Ty::Bool;
                        expr.shape = Shape::Scalar;
                        expr.facts.add(Facts::BOOL_SCALAR);
                    },
                    Lit::Str(_) => { expr.ty = Ty::Str; expr.shape = Shape::Scalar; },
                    _ => {}
                }
                // Literals are definitely Checked (conceptually)
            },
            IRExprKind::Name(n) => {
                if let Some((t, s, f)) = self.var_types.get(n) {
                    expr.ty = t.clone();
                    expr.shape = s.clone();
                    expr.facts = *f;
                }
            },
            IRExprKind::Unary { op: _, rhs } => { // op is _ because unused
                self.analyze_expr(rhs);
                // Propagate? !bool -> bool. -int -> int.
                expr.ty = rhs.ty.clone();
                expr.shape = rhs.shape.clone();
                // Facts might be lost (negation loses NON_NEG)
            },
            IRExprKind::Binary { op, lhs, rhs } => {
                self.analyze_expr(lhs);
                self.analyze_expr(rhs);
                
                match op {
                    crate::syntax::ast::BinOp::Eq | crate::syntax::ast::BinOp::Ne | 
                    crate::syntax::ast::BinOp::Lt | crate::syntax::ast::BinOp::Le | 
                    crate::syntax::ast::BinOp::Gt | crate::syntax::ast::BinOp::Ge |
                    crate::syntax::ast::BinOp::And | crate::syntax::ast::BinOp::Or => {
                        expr.ty = Ty::Bool;
                        if lhs.shape == Shape::Scalar && rhs.shape == Shape::Scalar {
                            expr.shape = Shape::Scalar;
                            expr.facts.add(Facts::BOOL_SCALAR);
                        } else {
                            expr.shape = Shape::Vec;
                        }
                    },
                    _ => {
                        // Arithmetic
                        expr.ty = lhs.ty.clone(); 
                        expr.shape = if lhs.shape == Shape::Vec || rhs.shape == Shape::Vec { Shape::Vec } else { Shape::Scalar };
                        if expr.ty == Ty::Int && expr.shape == Shape::Scalar {
                             expr.facts.add(Facts::INT_SCALAR);
                        }
                        // NonNeg propagation?
                    }
                }
            },
// ... rest matches original ...

             IRExprKind::RrRange { a, b } => {
                self.analyze_expr(a);
                self.analyze_expr(b);
                expr.ty = Ty::Int;
                expr.shape = Shape::Vec;
                // Range isn't necessarily non-neg
            },
            // Recurse for others
             IRExprKind::Call { callee, args } => {
                 self.analyze_expr(callee);
                 for a in args { self.analyze_expr(a); }
             },
             IRExprKind::Index1D { base, idx } => {
                 self.analyze_expr(base);
                 self.analyze_expr(idx);
                 // Ty propagation?
                 expr.ty = base.ty.clone(); // Rough approx
                 expr.shape = Shape::Scalar; // x[i]
             },
             IRExprKind::Slice1D { base, a, b } => {
                 self.analyze_expr(base);
                 self.analyze_expr(a);
                 self.analyze_expr(b);
                 expr.ty = base.ty.clone();
                 expr.shape = Shape::Vec;
             },
             IRExprKind::VectorLit(v) => {
                 for e in v { self.analyze_expr(e); }
                 expr.ty = Ty::Any; // Could infer from elem 0
                 expr.shape = Shape::Vec;
             },
             _ => {}
        }
    }
}
