use crate::subset::ast::*;

impl Default for Instance {
    fn default() -> Instance {
        Instance {
            id: String::new(),
            prim: String::new(),
            params: Map::new(),
            ports: Map::new(),
            attr: Attribute::default(),
        }
    }
}

impl Default for InstancePath {
    fn default() -> InstancePath {
        InstancePath { path: Vec::new() }
    }
}
