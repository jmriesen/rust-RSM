use crate::Variable;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KillType {
    Inclusive,
    //TODO: Note elusive kills cant actually be used on all variables, only local without subscript.
    //This should be reflected in the type system.
    Exclusive,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Kill {
    pub r#type: KillType,
    pub variables: Vec<Variable>,
}
