use super::ast::*;

impl From<SequentialIfElse> for Sequential {
    fn from(seq: SequentialIfElse) -> Self {
        Sequential::If(seq)
    }
}
