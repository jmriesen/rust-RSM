use crate::Variable;

pub enum KillType {
    Inclusive,
    Exclusive,
}

pub struct Kill {
    pub r#type: KillType,
    pub variables: Vec<Variable>,
}
