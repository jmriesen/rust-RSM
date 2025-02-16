use super::Expression;

#[derive(Clone)]
pub enum VariableType {
    Local {
        name: String,
    },
    NakedVariable,
    IndirectVariable {
        expression: Box<Expression>,
    },
    GlobalVariable {
        name: String,
    },
    GlobalUciVariable {
        name: String,
        uci: Box<Expression>,
    },
    GlobalUciEnvVariable {
        name: String,
        uci: Box<Expression>,
        env: Box<Expression>,
    },
}

impl VariableType {
    pub fn name(&self) -> Option<&str> {
        match self {
            VariableType::Local { name } => Some(name),
            VariableType::NakedVariable => None,
            VariableType::IndirectVariable { .. } => None,
            VariableType::GlobalVariable { name } => Some(name),
            VariableType::GlobalUciVariable { name, .. } => Some(name),
            VariableType::GlobalUciEnvVariable { name, .. } => Some(name),
        }
    }
}

#[derive(Clone)]
pub struct Variable {
    pub var_type: VariableType,
    pub subscripts: Vec<Expression>,
}
