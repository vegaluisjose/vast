use crate::pretty::PrettyPrinter;
use pretty::RcDoc;
use std::fmt;
use std::rc::Rc;

pub type Id = String;

#[derive(Clone, Debug)]
pub enum Unop {
    LogicalNegation,
    BitwiseNegation,
    BitwiseAnd,
    BitwiseNand,
    BitwiseOr,
    BitwiseNor,
    BitwiseXor,
    BitwiseXnor,
}

impl PrettyPrinter for Unop {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Unop::LogicalNegation => RcDoc::text("!"),
            Unop::BitwiseNegation => RcDoc::text("~"),
            Unop::BitwiseAnd => RcDoc::text("&"),
            Unop::BitwiseNand => RcDoc::text("~&"),
            Unop::BitwiseOr => RcDoc::text("|"),
            Unop::BitwiseNor => RcDoc::text("~|"),
            Unop::BitwiseXor => RcDoc::text("^"),
            Unop::BitwiseXnor => RcDoc::text("~^"),
        }
    }
}

impl fmt::Display for Unop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(Id),
    Unop(Unop, Rc<Expr>),
}

impl PrettyPrinter for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(name) => RcDoc::as_string(name),
            Expr::Unop(op, name) => op.to_doc().append(name.to_doc()),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_negation() {
        assert_eq!("!".to_string(), Unop::LogicalNegation.to_string());
    }
}

