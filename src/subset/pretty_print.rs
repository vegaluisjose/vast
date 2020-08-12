use crate::subset::ast::*;
use crate::util::pretty_print::{PrettyPrint, PRETTY_INDENT};
use pretty::RcDoc;

impl PrettyPrint for Unop {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Unop::LogNot => RcDoc::text("!"),
            Unop::Not => RcDoc::text("~"),
            Unop::And => RcDoc::text("&"),
            Unop::Nand => RcDoc::text("~&"),
            Unop::Or => RcDoc::text("|"),
            Unop::Nor => RcDoc::text("~|"),
            Unop::Xor => RcDoc::text("^"),
            Unop::Xnor => RcDoc::text("~^"),
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

impl PrettyPrint for InstancePath {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::intersperse(self.path().iter().map(RcDoc::as_string), RcDoc::text("."))
    }
}

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(name) => RcDoc::as_string(name),
            Expr::Int(num) => RcDoc::as_string(num),
            Expr::ULit(width, radix, value) => RcDoc::as_string(width)
                .append(RcDoc::text("'"))
                .append(radix.to_doc())
                .append(RcDoc::as_string(value)),
            Expr::Str(value) => RcDoc::text(r#"""#)
                .append(RcDoc::as_string(value))
                .append(RcDoc::text(r#"""#)),
            Expr::IPath(path) => path.to_doc(),
            Expr::Unop(op, input) => op.to_doc().append(input.to_doc()),
            Expr::Bit(var, index) => var
                .to_doc()
                .append(RcDoc::text("["))
                .append(index.to_doc())
                .append(RcDoc::text("]")),
            Expr::Binop(op, lhs, rhs) => lhs
                .to_doc()
                .append(RcDoc::space())
                .append(op.to_doc())
                .append(RcDoc::space())
                .append(rhs.to_doc()),
            Expr::Terop(Terop::Mux, cond, tru, fal) => cond
                .to_doc()
                .append(RcDoc::space())
                .append(RcDoc::text("?"))
                .append(RcDoc::space())
                .append(tru.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(fal.to_doc()),
            Expr::Terop(Terop::Slice, var, hi, lo) => var
                .to_doc()
                .append(RcDoc::text("["))
                .append(hi.to_doc())
                .append(RcDoc::text(":"))
                .append(lo.to_doc())
                .append(RcDoc::text("]")),
            Expr::Terop(Terop::IndexSlice, var, lo, width) => var
                .to_doc()
                .append(RcDoc::text("["))
                .append(lo.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("+"))
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(width.to_doc())
                .append(RcDoc::text("]")),
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
            .append(RcDoc::text(";"))
    }
}
