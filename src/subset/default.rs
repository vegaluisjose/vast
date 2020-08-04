use crate::subset::ast::*;

impl Default for Instance {
    fn default() -> Instance {
        Instance {
            id: String::new(),
            prim: String::new(),
            params: Map::new(),
            ports: Map::new(),
        }
    }
}