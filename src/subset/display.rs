use crate::subset::ast::*;
use crate::util::pretty_print::PrettyPrint;
use std::fmt;

impl fmt::Display for Unop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_fmt(f)
    }
}

impl fmt::Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_fmt(f)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_fmt(f)
    }
}

impl fmt::Display for AttributeTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_fmt(f)
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_fmt(f)
    }
}

impl fmt::Display for EventTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_fmt(f)
    }
}
