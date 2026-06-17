use crate::Variable;

pub struct KillInclusive(pub Vec<Variable>);
pub struct KillExclusive(pub Vec<Variable>);

pub enum Kill {
    Inclusive(KillInclusive),
    Exclusive(KillExclusive),
}
