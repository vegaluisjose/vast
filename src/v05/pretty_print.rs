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

impl PrettyPrint for Parallel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Parallel::Assign => RcDoc::text("assign"),
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
        let mut body_doc = RcDoc::nil();
        for stmt in self.body.iter() {
            body_doc = body_doc
                .append(RcDoc::hardline())
                .append(stmt.to_doc())
                .append(RcDoc::text(";"));
        }
        body_doc = body_doc.nest(PRETTY_INDENT).group();
        let mut ports_doc = match self.ports.is_empty() {
            true => RcDoc::nil(),
            false => RcDoc::hardline()
                .append(RcDoc::intersperse(
                    self.ports.iter().map(|p| p.to_doc()),
                    RcDoc::text(",").append(RcDoc::hardline()),
                )),
        };
        ports_doc = ports_doc.nest(PRETTY_INDENT).group();
        RcDoc::text("module")
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name))
            .append(RcDoc::space())
            .append(RcDoc::text("("))
            .append(ports_doc)
            .append(RcDoc::text(")"))
            .append(RcDoc::text(";"))
            .append(body_doc)
            .append(RcDoc::hardline())
            .append(RcDoc::text("endmodule"))
    }
}
