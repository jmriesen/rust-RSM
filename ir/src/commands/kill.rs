use crate::Variable;

#[derive(Clone, Copy, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum KillType {
    Inclusive,
    //TODO: Note elusive kills cant actually be used on all variables, only local without subscript.
    //This should be reflected in the type system.
    Exclusive,
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Kill {
    pub r#type: KillType,
    pub variables: Vec<Variable>,
}
