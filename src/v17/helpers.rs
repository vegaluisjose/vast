use crate::v17::ast::*;

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

impl Decl {
    pub fn new_param_uint(name: &str, value: u32) -> Decl {
        Decl::Param(
            name.to_string(),
            Ty::new_int(),
            Expr::new_ulit_dec(32, &value.to_string()),
        )
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
