use super::Expression;

#[derive(Clone)]
pub struct GlobleIdent {
    pub user_class: Option<Box<UserClassIdentifiers>>,
}
#[derive(Clone)]
pub struct UserClassIdentifiers {
    pub uci: Expression,
    pub env: Option<Env>,
}
#[derive(Clone)]
pub struct Env(pub Expression);

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Variable {
    pub var_type: VariableType,
    pub subscripts: Vec<Expression>,
}
