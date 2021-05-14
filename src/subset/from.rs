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

impl From<String> for Expr {
    fn from(id: String) -> Self {
        Expr::Ref(id)
    }
}

impl From<i32> for Expr {
    fn from(i: i32) -> Self {
        Expr::new_int(i)
    }
}
