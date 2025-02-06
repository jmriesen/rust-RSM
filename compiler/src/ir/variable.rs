use lang_model::Expression;

use crate::{localvar::VarContext, Compileable};

#[derive(Clone)]
pub enum VariableType<'a> {
    Local {
        name: String,
    },
    NakedVariable,
    IndirectVariable {
        expression: Expression<'a>,
    },
    GlobalVariable {
        name: String,
    },
    GlobalUciVariable {
        name: String,
        uci: Expression<'a>,
    },
    GlobalUciEnvVariable {
        name: String,
        uci: Expression<'a>,
        env: Expression<'a>,
    },
}

impl<'a> VariableType<'a> {
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
pub struct Variable<'a> {
    var_type: VariableType<'a>,
    subscripts: Vec<Expression<'a>>,
}

impl<'a> Variable<'a> {
    pub fn new(sitter: &lang_model::Variable<'a>, source_code: &str) -> Self {
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
                expression: x.children(),
            },
            Some(E::NakedVariable(_)) => NakedVariable,
            Some(E::GlobalVariable(_)) => GlobalVariable {
                name: name.unwrap(),
            },
            Some(E::GlobalUciVariable(x)) => GlobalUciVariable {
                name: name.unwrap(),
                uci: x.children(),
            },
            Some(E::GlobalUciEnvVariable(x)) => {
                let args = x.children();
                assert_eq!(args.len(), 2);
                let mut args = args.into_iter();
                GlobalUciEnvVariable {
                    name: name.unwrap(),
                    uci: args.next().expect("allready did bounds checking"),
                    env: args.next().expect("allready did bounds checking"),
                }
            }
            None => Local {
                name: name.unwrap(),
            },
        };
        Self {
            var_type,
            subscripts: sitter.subs(),
        }
    }
}
pub fn compile(variable: &Variable, source_code: &str, comp: &mut Vec<u8>, context: VarContext) {
    use crate::expression::ExpressionContext;
    use VariableType as E;
    match &variable.var_type {
        E::Local { .. } => {}
        E::NakedVariable => {}
        E::IndirectVariable { expression } => {
            expression.compile(source_code, comp, ExpressionContext::Eval);
            comp.push(ffi::INDMVAR);
        }
        E::GlobalVariable { .. } => {}
        E::GlobalUciVariable { uci, .. } => {
            uci.compile(source_code, comp, ExpressionContext::Eval);
        }
        E::GlobalUciEnvVariable { uci, env, .. } => {
            uci.compile(source_code, comp, ExpressionContext::Eval);
            env.compile(source_code, comp, ExpressionContext::Eval);
        }
    }

    for subscript in &variable.subscripts {
        subscript.compile(source_code, comp, ExpressionContext::Eval);
    }

    comp.push(context as u8);
    let op_code = match &variable.var_type {
        E::Local { .. } => ffi::TYPVARNAM,
        E::NakedVariable => ffi::TYPVARNAKED,
        E::IndirectVariable { .. } => ffi::TYPVARIND,
        E::GlobalVariable { .. } => ffi::TYPVARGBL,
        E::GlobalUciVariable { .. } => ffi::TYPVARGBLUCI,
        E::GlobalUciEnvVariable { .. } => ffi::TYPVARGBLUCIENV,
    } as u8;

    //Use a slightly more compact format if all we have to worry about is subscripts
    if matches!(
        variable.var_type,
        E::GlobalVariable { .. } | E::Local { .. }
    ) {
        comp.push(op_code | variable.subscripts.len() as u8);
    } else {
        comp.push(op_code);
        comp.push(variable.subscripts.len() as u8);
    }

    if let Some(name) = variable.var_type.name() {
        let name: ffi::VAR_U = name.try_into().unwrap();
        comp.extend(name.as_array())
    }
}
