use crate::v05::ast::*;

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
            Ty::Width(w) => w.clone(),
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
}

impl Module {
    pub fn new_with_name(name: &str) -> Module {
        Module {
            name: name.to_string(),
            ports: Vec::new(),
            body: Vec::new(),
        }
    }
}
