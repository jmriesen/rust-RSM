use super::Expression;
use crate::{bite_code::BiteCode, localvar::VarContext};

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
    var_type: VariableType,
    subscripts: Vec<Expression>,
}

impl Variable {
    pub fn new(sitter: &lang_model::Variable<'_>, source_code: &str) -> Self {
        use lang_model::VariableHeading as E;
        use VariableType::*;
        let name = sitter.name().map(|x| {
            x.node()
                .utf8_text(source_code.as_bytes())
                .unwrap()
                .to_owned()
        });

        let var_type = match sitter.heading() {
            Some(E::IndirectVariable(x)) => IndirectVariable {
                expression: Box::new(Expression::new(&x.children(), source_code)),
            },
            Some(E::NakedVariable(_)) => NakedVariable,
            Some(E::GlobalVariable(_)) => GlobalVariable {
                name: name.unwrap(),
            },
            Some(E::GlobalUciVariable(x)) => GlobalUciVariable {
                name: name.unwrap(),
                uci: Box::new(Expression::new(&x.children(), source_code)),
            },
            Some(E::GlobalUciEnvVariable(x)) => {
                let args = x.children();
                assert_eq!(args.len(), 2);
                let mut args = args.into_iter().map(|x| Expression::new(&x, source_code));
                GlobalUciEnvVariable {
                    name: name.unwrap(),
                    uci: Box::new(args.next().expect("allready did bounds checking")),
                    env: Box::new(args.next().expect("allready did bounds checking")),
                }
            }
            None => Local {
                name: name.unwrap(),
            },
        };
        Self {
            var_type,
            subscripts: sitter
                .subs()
                .iter()
                .map(|x| Expression::new(&x, source_code))
                .collect(),
        }
    }
    pub fn compile(&self, comp: &mut BiteCode, context: VarContext) {
        use crate::expression::ExpressionContext;
        use VariableType as E;
        match &self.var_type {
            E::Local { .. } => {}
            E::NakedVariable => {}
            E::IndirectVariable { expression } => {
                expression.compile(comp, ExpressionContext::Eval);
                comp.push(ffi::INDMVAR);
            }
            E::GlobalVariable { .. } => {}
            E::GlobalUciVariable { uci, .. } => {
                uci.compile(comp, ExpressionContext::Eval);
            }
            E::GlobalUciEnvVariable { uci, env, .. } => {
                uci.compile(comp, ExpressionContext::Eval);
                env.compile(comp, ExpressionContext::Eval);
            }
        }

        for subscript in &self.subscripts {
            subscript.compile(comp, ExpressionContext::Eval);
        }

        comp.push(context as u8);
        let op_code = match &self.var_type {
            E::Local { .. } => ffi::TYPVARNAM,
            E::NakedVariable => ffi::TYPVARNAKED,
            E::IndirectVariable { .. } => ffi::TYPVARIND,
            E::GlobalVariable { .. } => ffi::TYPVARGBL,
            E::GlobalUciVariable { .. } => ffi::TYPVARGBLUCI,
            E::GlobalUciEnvVariable { .. } => ffi::TYPVARGBLUCIENV,
        } as u8;

        //Use a slightly more compact format if all we have to worry about is subscripts
        if matches!(self.var_type, E::GlobalVariable { .. } | E::Local { .. }) {
            comp.push(op_code + self.subscripts.len() as u8);
        } else {
            comp.push(op_code);
            comp.push(self.subscripts.len() as u8);
        }

        if let Some(name) = self.var_type.name() {
            let name: ffi::VAR_U = name.try_into().unwrap();
            comp.extend(name.as_array().iter().cloned())
        }
    }
}
