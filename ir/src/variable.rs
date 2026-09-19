use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlobleIdent {
    pub user_class: Option<Box<UserClassIdentifiers>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserClassIdentifiers {
    pub uci: Expression,
    pub env: Option<Env>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Env(pub Expression);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariableType {
    Named {
        name: String,
        globle_ident: Option<GlobleIdent>,
    },
    NakedVariable,
    IndirectVariable {
        expression: Box<Expression>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub var_type: VariableType,
    pub subscripts: Vec<Expression>,
}
