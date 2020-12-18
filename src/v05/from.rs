use crate::v05::ast::*;

impl From<Instance> for Parallel {
    fn from(inst: Instance) -> Self {
        Parallel::Inst(inst)
    }
}

impl From<Decl> for Stmt {
    fn from(decl: Decl) -> Self {
        Stmt::Decl(decl)
    }
}

impl From<SequentialIfElse> for Sequential {
    fn from(seq: SequentialIfElse) -> Self {
        Sequential::IfElse(seq)
    }
}

impl From<ParallelProcess> for Parallel {
    fn from(proc: ParallelProcess) -> Self {
        Parallel::Process(proc)
    }
}

impl From<ParallelProcess> for Stmt {
    fn from(proc: ParallelProcess) -> Self {
        Stmt::from(Parallel::from(proc))
    }
}

impl From<Parallel> for Stmt {
    fn from(par: Parallel) -> Self {
        Stmt::Parallel(par)
    }
}

impl From<Instance> for Stmt {
    fn from(inst: Instance) -> Self {
        Stmt::from(Parallel::from(inst))
    }
}
