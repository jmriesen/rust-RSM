use crate::Variable;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KillType {
    Inclusive,
    Exclusive,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Kill {
    pub r#type: KillType,
    pub variables: Vec<Variable>,
}
