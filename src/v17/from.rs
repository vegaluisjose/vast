use super::ast::*;

impl From<SequentialIfElse> for Sequential {
    fn from(seq: SequentialIfElse) -> Self {
        Sequential::If(seq)
    }
}

impl From<ParallelProcess> for Parallel {
    fn from(process: ParallelProcess) -> Self {
        Parallel::Process(process)
    }
}

impl From<Parallel> for Stmt {
    fn from(parallel: Parallel) -> Self {
        Stmt::Parallel(parallel)
    }
}

impl From<ParallelProcess> for Stmt {
    fn from(process: ParallelProcess) -> Self {
        Stmt::from(Parallel::from(process))
    }
}

impl From<Function> for Decl {
    fn from(function: Function) -> Self {
        Decl::Func(function)
    }
}
