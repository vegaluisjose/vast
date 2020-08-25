use crate::v05::ast::*;

impl From<Instance> for Parallel {
    fn from(inst: Instance) -> Self {
        Parallel::Inst(inst)
    }
}
