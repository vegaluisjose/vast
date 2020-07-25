use crate::subset::ast::*;

impl Expr {
    pub fn id(&self) -> String {
        match self {
            Expr::Ref(id) => id.to_string(),
            _ => panic!("Error: do not support id"),
        }
    }

    pub fn new_ref(name: &str) -> Expr {
        Expr::Ref(name.to_string())
    }

    pub fn new_str(value: &str) -> Expr {
        Expr::Str(value.to_string())
    }

    pub fn new_dec_ulit(width: u32, value: &str) -> Expr {
        assert!(width > 0, "Error: width must be greater than zero");
        Expr::ULit(width, Radix::Dec, value.to_string())
    }

    pub fn new_hex_ulit(width: u32, value: &str) -> Expr {
        assert!(width > 0, "Error: width must be greater than zero");
        Expr::ULit(width, Radix::Hex, value.to_string())
    }

    pub fn new_bin_ulit(width: u32, value: &str) -> Expr {
        assert!(width > 0, "Error: width must be greater than zero");
        Expr::ULit(width, Radix::Bin, value.to_string())
    }
}
