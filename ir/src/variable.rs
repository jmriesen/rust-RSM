use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident(pub String);

impl derive_visitor::Drive for Ident {
    fn drive<V: derive_visitor::Visitor>(&self, _: &mut V) {}
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct GlobleIdent {
    pub user_class: Option<Box<UserClassIdentifiers>>,
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct UserClassIdentifiers {
    pub uci: Expression,
    pub env: Option<Env>,
}
#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Env(pub Expression);

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum VariableType {
    Named {
        name: Ident,
        globle_ident: Option<GlobleIdent>,
    },
    NakedVariable,
    IndirectVariable {
        expression: Box<Expression>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Variable {
    pub var_type: VariableType,
    pub subscripts: Vec<Expression>,
}
