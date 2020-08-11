use crate::util::pretty_print::{block, block_with_parens, intersperse, PrettyHelper, PrettyPrint};
use crate::v17::ast::*;
use pretty::RcDoc;

impl PrettyPrint for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Ty::Void => RcDoc::text("void"),
            Ty::Int => RcDoc::text("int"),
            Ty::Width(w) => match w {
                0 => panic!("Error: width must be greater than zero"),
                1 => RcDoc::nil(),
                n => RcDoc::text("[")
                    .append(RcDoc::as_string(n - 1))
                    .append(RcDoc::text(":"))
                    .append(RcDoc::text("0"))
                    .append(RcDoc::text("]")),
            },
        }
    }
}

impl PrettyPrint for CaseBranch {
    fn to_doc(&self) -> RcDoc<()> {
        let cond = self
            .cond
            .to_doc()
            .append(RcDoc::space())
            .append(RcDoc::text(":"))
            .append(RcDoc::space());
        let body = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.body()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
                RcDoc::hardline(),
            )
        };
        let body = if self.body().len() > 1 {
            block(body).begin_end()
        } else {
            body
        };
        cond.append(body)
    }
}

impl PrettyPrint for CaseDefault {
    fn to_doc(&self) -> RcDoc<()> {
        let default = RcDoc::text("default")
            .append(RcDoc::space())
            .append(RcDoc::text(":"))
            .append(RcDoc::space());
        let body = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.body()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
                RcDoc::hardline(),
            )
        };
        let body = if self.body().len() > 1 {
            block(body).begin_end()
        } else {
            body
        };
        default.append(body)
    }
}

impl PrettyPrint for Case {
    fn to_doc(&self) -> RcDoc<()> {
        let branches = if self.branches().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.branches().iter().map(|x| x.to_doc()),
                RcDoc::hardline(),
            )
        };
        let branches = if let Some(default) = &self.default {
            branches.append(RcDoc::hardline()).append(default.to_doc())
        } else {
            branches
        };
        self.cond
            .to_doc()
            .parens()
            .append(block(branches))
            .case_endcase()
    }
}

impl PrettyPrint for Function {
    fn to_doc(&self) -> RcDoc<()> {
        let inputs = if self.inputs().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.inputs()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
                RcDoc::hardline(),
            )
        };
        let decls = if self.decls().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.decls()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
                RcDoc::hardline(),
            )
        };
        let preamble = if self.inputs().is_empty() && self.decls().is_empty() {
            RcDoc::nil()
        } else if self.inputs().is_empty() {
            decls.append(RcDoc::hardline())
        } else if self.decls().is_empty() {
            inputs.append(RcDoc::hardline())
        } else {
            inputs
                .append(RcDoc::hardline())
                .append(decls)
                .append(RcDoc::hardline())
        };
        let body = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.body()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
                RcDoc::hardline(),
            )
        };
        RcDoc::space()
            .append(self.ret.to_doc())
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name))
            .append(RcDoc::text(";"))
            .append(block(preamble.append(block(body).begin_end())))
            .func_endfunc()
    }
}

impl PrettyPrint for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Int(name, ty) => ty
                .to_doc()
                .append(RcDoc::space())
                .append(RcDoc::as_string(name)),
            Decl::Logic(name, ty) => {
                let extra_space = match ty.width() {
                    1 => RcDoc::nil(),
                    _ => RcDoc::space(),
                };
                RcDoc::text("logic")
                    .append(RcDoc::space())
                    .append(ty.to_doc())
                    .append(extra_space)
                    .append(RcDoc::as_string(name))
            }
            Decl::Func(function) => function.to_doc(),
            Decl::Param(name, ty, expr) => RcDoc::text("parameter")
                .append(RcDoc::space())
                .append(ty.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::as_string(name))
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(expr.to_doc()),
        }
    }
}

impl PrettyPrint for Sequential {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Sequential::Error(msg) => RcDoc::text("$")
                .append(RcDoc::text("error"))
                .append(RcDoc::as_string(msg).quotes().parens()),
            Sequential::Display(msg) => RcDoc::text("$")
                .append(RcDoc::text("display"))
                .append(RcDoc::as_string(msg).quotes().parens()),
            Sequential::Return(expr) => RcDoc::text("return")
                .append(RcDoc::space())
                .append(expr.to_doc()),
            Sequential::SeqAssign(lexpr, rexpr, ty) => lexpr
                .to_doc()
                .append(RcDoc::space())
                .append(ty.to_doc())
                .append(RcDoc::space())
                .append(rexpr.to_doc()),
            Sequential::Event(ty, expr) => ty.to_doc().append(RcDoc::space()).append(expr.to_doc()),
            Sequential::Assert(expr, branch) => {
                let cond = RcDoc::text("assert").append(expr.to_doc().parens());
                if let Some(block) = branch {
                    cond.append(RcDoc::space())
                        .append(RcDoc::text("else"))
                        .append(RcDoc::space())
                        .append(block.to_doc())
                } else {
                    cond
                }
            }
            Sequential::If(_, _, _) => unimplemented!(),
        }
    }
}

impl PrettyPrint for AlwaysComb {
    fn to_doc(&self) -> RcDoc<()> {
        let body = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            block(intersperse(
                self.body()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
                RcDoc::hardline(),
            ))
            .begin_end()
        };
        RcDoc::text("always_comb")
            .append(RcDoc::space())
            .append(body)
    }
}

impl PrettyPrint for Parallel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Parallel::Inst(ty) => ty.to_doc(),
            Parallel::ParAssign(_, _) => unimplemented!(),
            Parallel::ParAlwaysComb(always) => always.to_doc(),
            Parallel::AlwaysFF(_, _) => unimplemented!(),
        }
    }
}

impl PrettyPrint for Stmt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Stmt::Decl(decl) => decl.to_doc(),
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
        let ports = if self.ports().is_empty() {
            RcDoc::nil()
        } else {
            intersperse(
                self.ports().iter().map(|x| x.to_doc()),
                RcDoc::text(",").append(RcDoc::hardline()),
            )
        };
        let name = if self.ports().is_empty() {
            RcDoc::as_string(&self.name)
                .append(RcDoc::space())
                .append(RcDoc::nil().parens())
        } else {
            block_with_parens(RcDoc::as_string(&self.name), ports)
        };
        let body = if self.body().is_empty() {
            RcDoc::hardline()
        } else {
            block(intersperse(
                self.body().iter().map(|x| x.to_doc()),
                RcDoc::hardline(),
            ))
        };
        RcDoc::space()
            .append(name)
            .append(RcDoc::text(";"))
            .append(body)
            .module_endmodule()
            .append(RcDoc::hardline())
    }
}
