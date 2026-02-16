pub mod lower;
pub mod analyze;
pub mod optimize;

use crate::syntax::ast::{Lit, UnaryOp, BinOp};
use crate::utils::Span;
pub use crate::mir::flow::Facts;

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub stmts: Vec<IRStmt>,
}

#[derive(Debug, Clone)]
pub struct IRStmt {
    pub kind: IRStmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum IRStmtKind {
    Assign { target: IRLValue, value: IRExpr },
    ExprStmt { expr: IRExpr },
    FnDecl { name: String, params: Vec<String>, body: Vec<IRStmt> },
    If { cond: IRExpr, then_blk: Vec<IRStmt>, else_blk: Option<Vec<IRStmt>> },
    While { cond: IRExpr, body: Vec<IRStmt> },
    For { var: String, seq: IRExpr, body: Vec<IRStmt> }, 
    Return { value: Option<IRExpr> },
}

#[derive(Debug, Clone)]
pub enum IRLValue {
    Name(String),
    Index1D { base: Box<IRExpr>, idx: Box<IRExpr> },
    Index2D { base: Box<IRExpr>, r: Box<IRExpr>, c: Box<IRExpr> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ty { Int, Float, Bool, Str, List, Any }

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Scalar,
    Vec,          // length unknown
    VecN(usize),  // length known
    Mat,          // 2D unknown
    Unknown,
}



#[derive(Debug, Clone)]
pub struct IRExpr {
    pub kind: IRExprKind,
    pub ty: Ty,
    pub shape: Shape,
    pub facts: Facts,
    pub span: Span,
}

impl IRExpr {
    pub fn new(kind: IRExprKind, span: Span) -> Self {
        Self { kind, ty: Ty::Any, shape: Shape::Unknown, facts: Facts::empty(), span }
    }
}

#[derive(Debug, Clone)]
pub enum IRExprKind {
    Lit(Lit),
    Name(String),
    Unary { op: UnaryOp, rhs: Box<IRExpr> },
    Binary { op: BinOp, lhs: Box<IRExpr>, rhs: Box<IRExpr> },
    Call { callee: Box<IRExpr>, args: Vec<IRExpr> },
    
    // Explicit runtime calls (lowered from AST)
    RrRange { a: Box<IRExpr>, b: Box<IRExpr> }, // rr_range(a,b)
    RrIndices { x: Box<IRExpr> },               // rr_indices(x)
    
    // Optimized Indexing
    Index1D { base: Box<IRExpr>, idx: Box<IRExpr> },
    Index2D { base: Box<IRExpr>, r: Box<IRExpr>, c: Box<IRExpr> },

    // NEW: Slice for x[a..b]
    Slice1D { base: Box<IRExpr>, a: Box<IRExpr>, b: Box<IRExpr> },
    
    VectorLit(Vec<IRExpr>), // c(...)
    ListLit(Vec<(String, IRExpr)>), // list(...)
}
