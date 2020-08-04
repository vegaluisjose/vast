use crate::v17::ast::*;
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

impl Port {
    pub fn new_input(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let logic = Decl::Logic(name.to_string(), ty);
        Port::Input(logic)
    }

    pub fn new_output(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let logic = Decl::Logic(name.to_string(), ty);
        Port::Output(logic)
    }
}

impl Sequential {
    pub fn new_error(msg: &str) -> Sequential {
        Sequential::Error(msg.to_string())
    }

    pub fn new_assert(expr: Expr) -> Sequential {
        Sequential::Assert(expr, None)
    }

    pub fn new_assert_with_else(expr: Expr, seq: Sequential) -> Sequential {
        Sequential::Assert(expr, Some(Rc::new(seq)))
    }
}

impl Parallel {
    pub fn new_inst(inst: Instance) -> Parallel {
        Parallel::Inst(inst)
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

impl Function {
    pub fn new(name: &str, ret: Ty) -> Function {
        Function {
            name: name.to_string(),
            inputs: Vec::new(),
            decls: Vec::new(),
            body: Vec::new(),
            ret,
        }
    }

    pub fn add_input(&mut self, name: &str, width: u64) {
        self.inputs.push(Port::new_input(name, width));
    }

    pub fn add_logic(&mut self, name: &str, width: u64) {
        self.decls.push(Decl::new_logic(name, width));
    }

    pub fn set_return_type(&mut self, ret: Ty) {
        self.ret = ret;
    }
}

impl Decl {
    pub fn new_param_uint(name: &str, value: u32) -> Decl {
        Decl::Param(
            name.to_string(),
            Ty::new_int(),
            Expr::new_ulit_dec(32, &value.to_string()),
        )
    }

    pub fn new_logic(name: &str, width: u64) -> Decl {
        Decl::Logic(name.to_string(), Ty::new_width(width))
    }

    pub fn new_func(func: Function) -> Decl {
        Decl::Func(func)
    }
}

impl Module {
    pub fn new_with_name(name: &str) -> Module {
        Module {
            name: name.to_string(),
            params: Vec::new(),
            ports: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_input(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_input(name, width));
    }

    pub fn add_output(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_output(name, width));
    }

    pub fn add_function(&mut self, func: Function) {
        self.body.push(Stmt::new_decl(Decl::new_func(func)));
    }

    pub fn add_instance(&mut self, inst: Instance) {
        self.body.push(Stmt::new_parallel(Parallel::new_inst(inst)));
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
}
