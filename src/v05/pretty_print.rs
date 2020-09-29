// use crate::util::pretty_print::{PrettyHelper, PrettyPrint, PRETTY_INDENT};
use crate::util::pretty_print::{block, block_with_parens, intersperse, PrettyHelper, PrettyPrint};
use crate::v05::ast::*;
use pretty::RcDoc;

impl PrettyPrint for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Ty::Int => RcDoc::text("integer"),
            Ty::Width(w) => match w {
                0 => panic!("Error: width must be greater than zero"),
                1 => RcDoc::nil(),
                n => RcDoc::as_string(n - 1)
                    .append(RcDoc::text(":"))
                    .append(RcDoc::text("0"))
                    .brackets(),
            },
        }
    }
}

impl PrettyPrint for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Int(name, ty) => ty
                .to_doc()
                .append(RcDoc::space())
                .append(RcDoc::as_string(name)),
            Decl::Wire(name, ty) => {
                let extra_space = match ty.width() {
                    1 => RcDoc::nil(),
                    _ => RcDoc::space(),
                };
                RcDoc::text("wire")
                    .append(RcDoc::space())
                    .append(ty.to_doc())
                    .append(extra_space)
                    .append(RcDoc::as_string(name))
            }
            Decl::Reg(name, ty) => {
                let extra_space = match ty.width() {
                    1 => RcDoc::nil(),
                    _ => RcDoc::space(),
                };
                RcDoc::text("reg")
                    .append(RcDoc::space())
                    .append(ty.to_doc())
                    .append(extra_space)
                    .append(RcDoc::as_string(name))
            }
            Decl::Param(name, expr) => RcDoc::text("parameter")
                .append(RcDoc::space())
                .append(RcDoc::as_string(name))
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(expr.to_doc()),
        }
    }
}

impl PrettyPrint for SequentialIfElse {
    fn to_doc(&self) -> RcDoc<()> {
        let cond = RcDoc::text("if").append(self.cond().to_doc().parens());
        let true_body = if self.true_body().is_empty() {
            RcDoc::nil()
        } else {
            block(intersperse(
                self.true_body().iter().map(|x| x.to_doc()),
                RcDoc::hardline(),
            ))
            .begin_end()
        };
        let false_body = if self.false_body().is_empty() {
            RcDoc::nil()
        } else {
            block(intersperse(
                self.false_body().iter().map(|x| x.to_doc()),
                RcDoc::hardline(),
            ))
            .begin_end()
        };
        cond.append(RcDoc::space())
            .append(true_body)
            .append(false_body)
    }
}

impl PrettyPrint for Sequential {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            // wildcard for sensitivity list
            Sequential::Wildcard => RcDoc::text("*"),
            Sequential::Event(ty, expr) => ty.to_doc().append(RcDoc::space()).append(expr.to_doc()),
            Sequential::IfElse(ifelse) => ifelse.to_doc(),
            Sequential::Assign(lexpr, rexpr, ty) => lexpr
                .to_doc()
                .append(RcDoc::space())
                .append(ty.to_doc())
                .append(RcDoc::space())
                .append(rexpr.to_doc())
                .append(RcDoc::text(";")),
        }
    }
}

impl PrettyPrint for ParallelAlways {
    fn to_doc(&self) -> RcDoc<()> {
        let body = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            block(intersperse(
                self.body().iter().map(|x| x.to_doc()),
                RcDoc::hardline(),
            ))
            .begin_end()
        };
        RcDoc::text("always")
            .append(self.event().to_doc().parens())
            .append(RcDoc::space())
            .append(body)
    }
}

impl PrettyPrint for Parallel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Parallel::Inst(ty) => ty.to_doc(),
            Parallel::Assign(lexpr, rexpr) => RcDoc::text("assign")
                .append(RcDoc::space())
                .append(lexpr.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(rexpr.to_doc())
                .append(RcDoc::text(";")),
            Parallel::Always(always) => always.to_doc(),
        }
    }
}

impl PrettyPrint for Stmt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Stmt::Decl(decl) => decl.to_doc().append(RcDoc::text(";")),
            Stmt::Parallel(par) => par.to_doc(),
        }
    }
}

impl PrettyPrint for Port {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Port::Input(decl) => RcDoc::text("input")
                .append(RcDoc::space())
                .append(decl.to_doc()),
            Port::Output(decl) => RcDoc::text("output")
                .append(RcDoc::space())
                .append(decl.to_doc()),
        }
    }
}

impl PrettyPrint for Module {
    fn to_doc(&self) -> RcDoc<()> {
        let params = if self.params().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.params().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::hardline()),
            )
        };
        let ports = if self.ports().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.ports().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::hardline()),
            )
        };
        let name = if self.params.is_empty() && self.ports.is_empty() {
            RcDoc::as_string(&self.name)
                .append(RcDoc::space())
                .append(RcDoc::nil().parens())
        } else if self.params.is_empty() {
            block_with_parens(RcDoc::as_string(&self.name), ports)
        } else if self.ports.is_empty() {
            block_with_parens(
                RcDoc::as_string(&self.name)
                    .append(RcDoc::space())
                    .append(RcDoc::text("#")),
                params,
            )
        } else {
            block_with_parens(
                RcDoc::as_string(&self.name)
                    .append(RcDoc::space())
                    .append(RcDoc::text("#")),
                params,
            )
            .append(block_with_parens(RcDoc::nil(), ports))
        };
        let body = if self.body().is_empty() {
            RcDoc::hardline()
        } else {
            block(intersperse(
                self.body().iter().map(|x| x.to_doc()),
                RcDoc::hardline(),
            ))
        };
        let attr = if self.attr().attrs().is_empty() {
            RcDoc::nil()
        } else {
            self.attr().to_doc().append(RcDoc::hardline())
        };
        let module = RcDoc::space()
            .append(name)
            .append(RcDoc::text(";"))
            .append(body)
            .module_endmodule()
            .append(RcDoc::hardline());
        attr.append(module)
    }
}
