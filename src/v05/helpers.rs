use crate::v05::ast::*;
use std::rc::Rc;

impl Ty {
    pub fn new_int() -> Ty {
        Ty::Int
    }

    pub fn new_width(width: u64) -> Ty {
        assert!(width > 0, "Error: width must be greater than zero");
        Ty::Width(width)
    }

    pub fn width(&self) -> u64 {
        match self {
            Ty::Width(w) => *w,
            _ => panic!("Error: type does not support width"),
        }
    }
}

impl Decl {
    pub fn new_int(name: &str) -> Decl {
        Decl::Int(name.to_string(), Ty::new_int())
    }

    pub fn new_wire(name: &str, width: u64) -> Decl {
        Decl::Wire(name.to_string(), Ty::new_width(width))
    }

    pub fn new_reg(name: &str, width: u64) -> Decl {
        Decl::Reg(name.to_string(), Ty::new_width(width))
    }

    pub fn new_array(name: &str, width: u64, depth: u64) -> Decl {
        Decl::Array(name.to_string(), Ty::new_width(width), Ty::new_width(depth))
    }

    pub fn new_param_uint(name: &str, value: u32) -> Decl {
        Decl::Param(name.to_string(), Expr::new_ulit_dec(32, &value.to_string()))
    }

    pub fn new_param_str(name: &str, value: &str) -> Decl {
        Decl::Param(name.to_string(), Expr::new_str(value))
    }
}

impl Port {
    pub fn new_input(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let wire = Decl::Wire(name.to_string(), ty);
        Port::Input(wire)
    }

    pub fn new_output(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let wire = Decl::Wire(name.to_string(), ty);
        Port::Output(wire)
    }

    pub fn new_output_reg(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let reg = Decl::Reg(name.to_string(), ty);
        Port::Output(reg)
    }
}

impl SequentialIfElse {
    pub fn new(cond: Expr) -> Self {
        SequentialIfElse {
            cond: Some(cond),
            body: Vec::new(),
            elsebr: None,
        }
    }

    pub fn cond(&self) -> Option<&Expr> {
        self.cond.as_ref()
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }

    pub fn else_branch(&self) -> Option<&Sequential> {
        self.elsebr.as_deref()
    }

    pub fn add_seq(&mut self, seq: Sequential) {
        self.body.push(seq);
    }

    pub fn set_else(&mut self, seq: Sequential) {
        self.elsebr = Some(Rc::new(seq));
    }
}

impl Sequential {
    pub fn new_posedge(name: &str) -> Self {
        let expr = Expr::new_ref(name);
        Sequential::Event(EventTy::Posedge, expr)
    }

    pub fn new_blk_assign(lexpr: Expr, rexpr: Expr) -> Sequential {
        Sequential::Assign(lexpr, rexpr, AssignTy::Blocking)
    }

    pub fn new_nonblk_assign(lexpr: Expr, rexpr: Expr) -> Sequential {
        Sequential::Assign(lexpr, rexpr, AssignTy::NonBlocking)
    }

    pub fn new_case(case: Case) -> Sequential {
        Sequential::SeqCase(case)
    }
}

impl ParallelProcess {
    pub fn new_always() -> Self {
        ParallelProcess {
            ty: ProcessTy::Always,
            event: None,
            body: Vec::new(),
        }
    }

    pub fn ty(&self) -> &ProcessTy {
        &self.ty
    }

    pub fn event(&self) -> Option<&Sequential> {
        self.event.as_ref()
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }

    pub fn add_seq(&mut self, seq: Sequential) -> &mut Self {
        self.body.push(seq);
        self
    }

    pub fn set_event(&mut self, seq: Sequential) {
        self.event = Some(seq)
    }
}

impl Parallel {
    pub fn new_inst(inst: Instance) -> Parallel {
        Parallel::from(inst)
    }

    pub fn id(&self) -> String {
        match self {
            Parallel::Inst(inst) => inst.id(),
            Parallel::Assign(lexpr, _) => lexpr.id(),
            _ => panic!("Error: always do not support id"),
        }
    }
}

impl Stmt {
    pub fn new_parallel(par: Parallel) -> Stmt {
        Stmt::Parallel(par)
    }

    pub fn new_decl(decl: Decl) -> Stmt {
        Stmt::Decl(decl)
    }
}

impl Module {
    pub fn new(name: &str) -> Module {
        Module {
            name: name.to_string(),
            params: Vec::new(),
            ports: Vec::new(),
            body: Vec::new(),
            attr: Attribute::default(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn body(&self) -> &Vec<Stmt> {
        &self.body
    }

    pub fn ports(&self) -> &Vec<Port> {
        &self.ports
    }

    pub fn params(&self) -> &Vec<Decl> {
        &self.params
    }

    pub fn attr(&self) -> &Attribute {
        &self.attr
    }

    pub fn add_param_uint(&mut self, name: &str, value: u32) {
        self.params.push(Decl::new_param_uint(name, value));
    }

    pub fn add_param_str(&mut self, name: &str, value: &str) {
        self.params.push(Decl::new_param_str(name, value));
    }

    pub fn add_port(&mut self, port: Port) {
        self.ports.push(port);
    }

    pub fn add_input(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_input(name, width));
    }

    pub fn add_output(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_output(name, width));
    }

    pub fn add_output_reg(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_output_reg(name, width));
    }

    pub fn add_instance(&mut self, inst: Instance) {
        self.body.push(Stmt::new_parallel(Parallel::new_inst(inst)));
    }

    pub fn add_decl(&mut self, decl: Decl) {
        self.body.push(Stmt::from(decl));
    }

    pub fn add_stmt(&mut self, stmt: Stmt) {
        self.body.push(stmt.into());
    }

    pub fn set_attr(&mut self, attr: Attribute) {
        self.attr = attr;
    }
}

impl CaseBranch {
    pub fn new(cond: Expr) -> CaseBranch {
        CaseBranch {
            cond,
            body: Vec::new(),
        }
    }

    pub fn add_seq(&mut self, seq: Sequential) -> &mut Self {
        self.body.push(seq);
        self
    }

    pub fn add_case(&mut self, case: Case) -> &mut Self {
        self.body.push(Sequential::new_case(case));
        self
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }
}

impl CaseDefault {
    pub fn add_seq(&mut self, seq: Sequential) -> &mut Self {
        self.body.push(seq);
        self
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }
}

impl Default for CaseDefault {
    fn default() -> CaseDefault {
        CaseDefault { body: Vec::new() }
    }
}

impl Case {
    pub fn new(cond: Expr) -> Case {
        Case {
            cond,
            branches: Vec::new(),
            default: None,
        }
    }

    pub fn add_branch(&mut self, branch: CaseBranch) -> &mut Self {
        self.branches.push(branch);
        self
    }

    pub fn set_default(&mut self, branch: CaseDefault) {
        self.default = Some(branch);
    }

    pub fn branches(&self) -> &Vec<CaseBranch> {
        &self.branches
    }

    pub fn default(&self) -> &CaseDefault {
        if let Some(default) = &self.default {
            &default
        } else {
            panic!("Default branch has not been set");
        }
    }
}
