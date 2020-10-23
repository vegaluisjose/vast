use crate::subset::ast::*;
use crate::util::pretty_print::{block, intersperse, PrettyHelper, PrettyPrint};
use itertools::Itertools;
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
            Binop::BitOr => RcDoc::text("|"),
            Binop::BitAnd => RcDoc::text("&"),
            Binop::LogOr => RcDoc::text("||"),
            Binop::LogAnd => RcDoc::text("&&"),
            Binop::Add => RcDoc::text("+"),
            Binop::Mul => RcDoc::text("*"),
            Binop::Lt => RcDoc::text("<"),
            Binop::Gt => RcDoc::text(">"),
            Binop::Geq => RcDoc::text(">="),
            Binop::Leq => RcDoc::text("<="),
            Binop::Equal => RcDoc::text("=="),
            Binop::NotEqual => RcDoc::text("!="),
            Binop::IndexBit => RcDoc::nil(),
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
        intersperse(self.path().iter().map(RcDoc::as_string), RcDoc::text("."))
    }
}

impl PrettyPrint for ExprConcat {
    fn to_doc(&self) -> RcDoc<()> {
        intersperse(
            self.exprs().iter().rev().map(|x| x.to_doc()),
            RcDoc::text(",").append(RcDoc::space()),
        )
        .braces()
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
            Expr::Str(value) => RcDoc::as_string(value).quotes(),
            Expr::Signed(expr) => RcDoc::text("$")
                .append(RcDoc::text("signed"))
                .append(expr.to_doc().parens()),
            Expr::IPath(path, index) => {
                if let Some(expr) = index.as_ref() {
                    path.to_doc().append(expr.to_doc().brackets())
                } else {
                    path.to_doc()
                }
            }
            Expr::Unop(op, input) => op.to_doc().append(input.to_doc()),
            Expr::Binop(Binop::IndexBit, lhs, rhs) => lhs.to_doc().append(rhs.to_doc().brackets()),
            Expr::Binop(op, lhs, rhs) => lhs
                .to_doc()
                .append(RcDoc::space())
                .append(op.to_doc())
                .append(RcDoc::space())
                .append(rhs.to_doc())
                .parens(),
            Expr::Call(name, params) => RcDoc::as_string(name).append(
                intersperse(
                    params.iter().map(RcDoc::as_string),
                    RcDoc::text(",").append(RcDoc::space()),
                )
                .parens(),
            ),
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
            Expr::Terop(Terop::Slice, var, hi, lo) => var.to_doc().append(
                hi.to_doc()
                    .append(RcDoc::text(":"))
                    .append(lo.to_doc())
                    .brackets(),
            ),
            Expr::Terop(Terop::IndexSlice, var, lo, width) => var.to_doc().append(
                lo.to_doc()
                    .append(RcDoc::space())
                    .append(RcDoc::text("+"))
                    .append(RcDoc::text(":"))
                    .append(RcDoc::space())
                    .append(width.to_doc())
                    .brackets(),
            ),
            Expr::Concat(concat) => concat.to_doc(),
        }
    }
}

impl PrettyPrint for AttributeTy {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            AttributeTy::Val(name) => RcDoc::as_string(name),
            AttributeTy::Stmt(id, value) => RcDoc::as_string(id)
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(RcDoc::as_string(value).quotes()),
        }
    }
}

impl PrettyPrint for Attribute {
    fn to_doc(&self) -> RcDoc<()> {
        intersperse(
            self.attrs().iter().rev().map(|x| x.to_doc()),
            RcDoc::text(",").append(RcDoc::space()),
        )
        .stars()
        .parens()
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
        intersperse(
            self.iter()
                .sorted_by_key(|(id, _)| (*id).clone())
                .map(|(id, expr)| {
                    RcDoc::text(".")
                        .append(RcDoc::as_string(id))
                        .append(expr.to_doc().parens())
                }),
            RcDoc::text(",").append(RcDoc::hardline()),
        )
    }
}

impl PrettyPrint for Instance {
    fn to_doc(&self) -> RcDoc<()> {
        let params = if self.param_map().is_empty() {
            RcDoc::space()
        } else {
            RcDoc::space()
                .append(RcDoc::text("#"))
                .append(RcDoc::space())
                .append(block(self.param_map().to_doc()).parens())
                .append(RcDoc::space())
        };
        let ports = if self.port_map().is_empty() {
            RcDoc::space().parens()
        } else {
            RcDoc::space().append(block(self.port_map().to_doc()).parens())
        };
        let attr = if self.attr().attrs().is_empty() {
            RcDoc::nil()
        } else {
            self.attr().to_doc().append(RcDoc::hardline())
        };
        attr.append(RcDoc::as_string(self.prim()))
            .append(params)
            .append(RcDoc::as_string(self.id()))
            .append(ports)
            .append(RcDoc::text(";"))
    }
}
