use crate::Variable;

#[derive(Clone, Copy, Debug)]
pub enum KillType {
    Inclusive,
    Exclusive,
}

pub struct Kill {
    pub r#type: KillType,
    pub variables: Vec<Variable>,
}
