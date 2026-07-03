use super::Expression;

#[derive(Clone, Debug)]
pub struct GlobleIdent {
    pub user_class: Option<Box<UserClassIdentifiers>>,
}
#[derive(Clone, Debug)]
pub struct UserClassIdentifiers {
    pub uci: Expression,
    pub env: Option<Env>,
}
#[derive(Debug, Clone)]
pub struct Env(pub Expression);

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Variable {
    pub var_type: VariableType,
    pub subscripts: Vec<Expression>,
}
