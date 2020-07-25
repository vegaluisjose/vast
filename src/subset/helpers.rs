use crate::subset::ast::*;

impl Expr {
    pub fn new_ref(name: &str) -> Expr {
        Expr::Ref(name.to_string())
    }

    pub fn new_const(value: i32) -> Expr {
        Expr::Const(value)
    }
}
