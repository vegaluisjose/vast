use crate::subset::ast::*;

impl From<ExprConcat> for Expr {
    fn from(concat: ExprConcat) -> Self {
        Expr::Concat(concat)
    }
}

impl From<&str> for Expr {
    fn from(id: &str) -> Self {
        Expr::new_ref(id)
    }
}
