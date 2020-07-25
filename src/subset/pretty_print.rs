use crate::subset::ast::*;
use crate::util::pretty_print::PrettyPrint;
use pretty::RcDoc;

impl PrettyPrint for Rop {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Rop::LogNot => RcDoc::text("!"),
            Rop::Not => RcDoc::text("~"),
            Rop::And => RcDoc::text("&"),
            Rop::Nand => RcDoc::text("~&"),
            Rop::Or => RcDoc::text("|"),
            Rop::Nor => RcDoc::text("~|"),
            Rop::Xor => RcDoc::text("^"),
            Rop::Xnor => RcDoc::text("~^"),
        }
    }
}

impl PrettyPrint for Binop {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Binop::Add => RcDoc::text("+"),
        }
    }
}

impl PrettyPrint for Radix {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Radix::Dec => RcDoc::text("d"),
            Radix::Bin => RcDoc::text("b"),
            Radix::Hex => RcDoc::text("h"),
        }
    }
}

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(name) => RcDoc::as_string(name),
            Expr::ULit(width, radix, value) => RcDoc::as_string(width)
                .append(RcDoc::text("'"))
                .append(radix.to_doc())
                .append(RcDoc::as_string(value)),
            Expr::Str(value) => RcDoc::text(r#"""#)
                .append(RcDoc::as_string(value))
                .append(RcDoc::text(r#"""#)),
            Expr::Unop(op, input) => op.to_doc().append(input.to_doc()),
            Expr::Binop(op, lhs, rhs) => lhs
                .to_doc()
                .append(RcDoc::space())
                .append(op.to_doc())
                .append(RcDoc::space())
                .append(rhs.to_doc()),
        }
    }
}

impl PrettyPrint for EventTy {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            EventTy::Posedge => RcDoc::text("posedge"),
            EventTy::Negedge => RcDoc::text("negedge"),
        }
    }
}
