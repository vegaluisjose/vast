use crate::util::pretty_print::{PrettyPrint, PRETTY_INDENT};
use crate::v05::ast::*;
use pretty::RcDoc;

impl PrettyPrint for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
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

impl PrettyPrint for Sequential {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            // wildcard for sensitivity list
            Sequential::Wildcard => RcDoc::text("*"),
            Sequential::Event(ty, expr) => ty.to_doc().append(RcDoc::space()).append(expr.to_doc()),
            Sequential::If(expr, _, _) => RcDoc::text("if")
                .append(RcDoc::space())
                .append(RcDoc::text("("))
                .append(expr.to_doc())
                .append(RcDoc::text(")")),
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

impl PrettyPrint for Parallel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Parallel::Instance(ty) => ty.to_doc(),
            Parallel::Assign(lexpr, rexpr) => RcDoc::text("assign")
                .append(RcDoc::space())
                .append(lexpr.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(rexpr.to_doc()),
            Parallel::Always => RcDoc::text("always"),
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
        let body_doc = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            let mut doc = RcDoc::nil();
            for stmt in self.body().iter() {
                doc = doc
                    .append(RcDoc::hardline())
                    .append(RcDoc::hardline())
                    .append(stmt.to_doc())
                    .append(RcDoc::text(";"));
            }
            doc = doc.append(RcDoc::hardline()).nest(PRETTY_INDENT);
            doc
        };
        let mut ports_doc = if self.ports().is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::hardline().append(RcDoc::intersperse(
                self.ports().iter().map(|p| p.to_doc()),
                RcDoc::text(",").append(RcDoc::hardline()),
            ))
        };
        ports_doc = ports_doc.nest(PRETTY_INDENT);
        let mut params_doc = if self.params().is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::text("#")
                .append(RcDoc::space())
                .append(RcDoc::text("("))
                .append(RcDoc::hardline())
                .append(RcDoc::intersperse(
                    self.params().iter().map(|p| p.to_doc()),
                    RcDoc::text(",").append(RcDoc::hardline()),
                ))
                .append(RcDoc::text(")"))
        };
        params_doc = params_doc.nest(PRETTY_INDENT);
        RcDoc::text("module")
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name()))
            .append(RcDoc::space())
            .append(params_doc)
            .append(RcDoc::text("("))
            .append(ports_doc)
            .append(RcDoc::text(")"))
            .append(RcDoc::text(";"))
            .append(body_doc)
            .append(RcDoc::hardline())
            .append(RcDoc::text("endmodule"))
            .append(RcDoc::hardline())
    }
}
