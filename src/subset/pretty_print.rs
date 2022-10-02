use crate::subset::ast::*;
use crate::util::pretty_print::{block, intersperse, PrettyHelper, PrettyPrint};
use core::cmp::Ordering;
use itertools::Itertools;
use pretty::RcDoc;

/// Tracks the context in the guards to only generate parens when inside an
/// operator with stronger binding.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ParenCtx {
    Op,
    Not,
    And,
    Or,
    BAnd,
    BOr,
}

impl From<&Binop> for ParenCtx {
    fn from(s: &Binop) -> Self {
        match s {
            Binop::BitOr => ParenCtx::BOr,
            Binop::BitAnd => ParenCtx::BAnd,
            Binop::LogOr => ParenCtx::Or,
            Binop::LogAnd => ParenCtx::And,
            Binop::Add
            | Binop::Sub
            | Binop::Mul
            | Binop::Lt
            | Binop::Gt
            | Binop::Geq
            | Binop::Leq
            | Binop::Equal
            | Binop::NotEqual
            | Binop::IndexBit
            | Binop::ShiftLeft => ParenCtx::Op,
        }
    }
}

impl Ord for ParenCtx {
    fn cmp(&self, other: &Self) -> Ordering {
        use ParenCtx as P;
        if self == other {
            return Ordering::Equal;
        }
        match (self, other) {
            (P::Not, _) => Ordering::Greater,

            (P::Op, P::Not) => Ordering::Less,
            (P::Op, _) => Ordering::Greater,

            (P::BAnd, P::Not) | (P::BAnd, P::Op) => Ordering::Less,
            (P::BAnd, _) => Ordering::Greater,

            (P::BOr, P::Not) | (P::BOr, P::Op) | (P::BOr, P::BAnd) => Ordering::Less,
            (P::BOr, _) => Ordering::Greater,

            (P::And, P::Not) | (P::And, P::Op) | (P::And, P::BAnd) | (P::And, P::BOr) => {
                Ordering::Less
            }
            (P::And, _) => Ordering::Greater,

            (P::Or, _) => Ordering::Less,
        }
    }
}

impl PartialOrd for ParenCtx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn print_expr<'a>(e: &'a Expr, cur_ctx: ParenCtx) -> RcDoc<'a, ()> {
    match e {
        Expr::Binop(op, lhs, rhs) => {
            let ctx = ParenCtx::from(op);
            let doc = print_expr(lhs, ctx)
                .append(RcDoc::space())
                .append(op.to_doc())
                .append(RcDoc::space())
                .append(print_expr(rhs, ctx));
            if cur_ctx > ctx {
                doc.parens()
            } else {
                doc
            }
        }
        e => e.to_doc(),
    }
}

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
            Binop::Sub => RcDoc::text("-"),
            Binop::Mul => RcDoc::text("*"),
            Binop::Lt => RcDoc::text("<"),
            Binop::Gt => RcDoc::text(">"),
            Binop::Geq => RcDoc::text(">="),
            Binop::Leq => RcDoc::text("<="),
            Binop::Equal => RcDoc::text("=="),
            Binop::NotEqual => RcDoc::text("!="),
            Binop::ShiftLeft => RcDoc::text("<<"),
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
            Expr::X => RcDoc::text("'x"),
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
            Expr::Unop(op, input) => op.to_doc().append(print_expr(input, ParenCtx::Not)),
            Expr::Binop(Binop::IndexBit, lhs, rhs) => {
                print_expr(lhs, ParenCtx::Not).append(rhs.to_doc().brackets())
            }
            Expr::Binop(_, _, _) => print_expr(self, ParenCtx::Or),
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
                .append(if let Expr::Terop(Terop::Mux, _, _, _) = **fal {
                    RcDoc::hardline()
                } else {
                    RcDoc::nil()
                })
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
            Expr::Repeat(times, expr) => RcDoc::text(times.to_string())
                .append(expr.to_doc().braces())
                .braces(),
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
