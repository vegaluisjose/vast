use crate::subset::ast::*;

impl From<ExprConcat> for Expr {
    fn from(concat: ExprConcat) -> Self {
        Expr::Concat(concat)
    }
}
