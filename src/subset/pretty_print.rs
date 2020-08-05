use crate::subset::ast::*;
use crate::util::pretty_print::{PrettyPrint, PRETTY_INDENT};
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
            Binop::Mul => RcDoc::text("*"),
            Binop::Lt => RcDoc::text("<"),
            Binop::Equal => RcDoc::text("=="),
            Binop::NotEqual => RcDoc::text("!="),
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
            Expr::RadixULit(width, radix, value) => RcDoc::as_string(width)
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

impl PrettyPrint for AssignTy {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            AssignTy::Blocking => RcDoc::text("="),
            AssignTy::NonBlocking => RcDoc::text("<="),
        }
    }
}

impl PrettyPrint for Map {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::intersperse(
            self.iter().map(|(id, expr)| {
                RcDoc::text(".")
                    .append(RcDoc::as_string(id))
                    .append(RcDoc::text("("))
                    .append(expr.to_doc())
                    .append(RcDoc::text(")"))
            }),
            RcDoc::text(",").append(RcDoc::hardline()),
        )
    }
}

impl PrettyPrint for Instance {
    fn to_doc(&self) -> RcDoc<()> {
        let params_doc = if self.param_map().is_empty() {
            RcDoc::space()
        } else {
            RcDoc::space()
                .append(RcDoc::text("#"))
                .append(RcDoc::space())
                .append(RcDoc::text("("))
                .append(RcDoc::hardline())
                .append(self.param_map().to_doc())
                .append(RcDoc::text(")"))
                .nest(PRETTY_INDENT)
                .append(RcDoc::hardline())
        };
        let ports_doc = if self.port_map().is_empty() {
            RcDoc::space()
                .append(RcDoc::text("("))
                .append(RcDoc::text(")"))
        } else {
            RcDoc::space()
                .append(RcDoc::text("("))
                .append(RcDoc::hardline())
                .append(self.port_map().to_doc())
                .append(RcDoc::text(")"))
                .nest(PRETTY_INDENT)
        };
        RcDoc::as_string(self.prim())
            .append(params_doc)
            .append(RcDoc::as_string(self.id()))
            .append(ports_doc)
    }
}
