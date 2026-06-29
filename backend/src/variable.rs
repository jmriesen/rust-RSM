use crate::{
    Compile,
    bite_code::BiteCode,
    commands::set::SetCodes,
    runtime::{Decode, OpCode, program_counter::AssemballyDecoder},
};
use ir::{
    Variable,
    variable::{Env, GlobleIdent, VariableType},
};
use symbol_table::VariableName;

#[derive(Clone, Copy)]
pub enum VarContext {
    Eval = 61_isize,
    Build = 62_isize,
    BuildNullable = 63_isize,
    For = 177_isize,
}

pub enum VarCodes {
    Named = 0,
    Globle = 128,
    Naked = 252,
    GlobleUci = 253,
    GlobleUciEnv = 254,
    IndirectVariable = 255,
}

const INDERECT_VAR: u8 = 66;
impl Compile for Variable {
    type Context = VarContext;
    fn compile(&self, comp: &mut BiteCode, context: &VarContext) {
        use crate::expression::ExpressionContext;
        use VariableType as E;
        match &self.var_type {
            E::Named {
                globle_ident: None | Some(GlobleIdent { user_class: None }),
                ..
            } => {}
            E::Named {
                globle_ident:
                    Some(GlobleIdent {
                        user_class: Some(user_class),
                    }),
                ..
            } => {
                user_class.uci.compile(comp, &ExpressionContext::Eval);
                if let Some(Env(env)) = &user_class.env {
                    env.compile(comp, &ExpressionContext::Eval);
                }
            }
            E::NakedVariable => {}
            E::IndirectVariable { expression } => {
                expression.compile(comp, &ExpressionContext::Eval);
                comp.push(INDERECT_VAR);
            }
        }

        self.subscripts.compile(comp, &ExpressionContext::Eval);

        comp.push(*context as u8);
        let op_code = match &self.var_type {
            E::Named {
                globle_ident: None, ..
            } => VarCodes::Named,
            E::Named {
                globle_ident: Some(GlobleIdent { user_class: None }),
                ..
            } => VarCodes::Globle,
            E::Named {
                globle_ident:
                    Some(GlobleIdent {
                        user_class: Some(user_class),
                    }),
                ..
            } => {
                if user_class.env.is_none() {
                    VarCodes::GlobleUci
                } else {
                    VarCodes::GlobleUciEnv
                }
            }
            E::NakedVariable => VarCodes::Naked,
            E::IndirectVariable { .. } => VarCodes::IndirectVariable,
        } as u8;

        //Use a slightly more compact format if all we have to worry about is subscripts
        if matches!(
            self.var_type,
            E::Named {
                globle_ident: None | Some(GlobleIdent { user_class: None }),
                ..
            }
        ) {
            comp.push(op_code + self.subscripts.len() as u8);
        } else {
            comp.push(op_code);
            comp.push(self.subscripts.len() as u8);
        }

        if let E::Named { name, .. } = &self.var_type {
            name.as_str().compile(comp, &());
        }
    }
}

#[derive(Debug)]
pub struct BuildVarInstructions {
    pub name: VariableName,
    pub subscripts: usize,
}
impl Decode for BuildVarInstructions {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        //TODO: handle other types
        let [code_num_subscriptions] = decoder.consume_n();
        let variable_string = decoder.consume_n::<32>();
        let variable_string: Vec<_> = variable_string
            .iter()
            .take_while(|x| **x != 0)
            .cloned()
            .collect();
        Some(Self {
            name: VariableName::new(&variable_string).unwrap(),
            //TODO: handle parsing different types of variables.
            subscripts: code_num_subscriptions as usize,
        })
    }
}

#[derive(Debug)]
pub struct LoadVar {
    pub var: BuildVarInstructions,
}
impl Decode for LoadVar {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        const CODE: u8 = VarContext::Eval as u8;
        if let [CODE] = decoder.consume_n() {
            Some(Self {
                var: Decode::decode(decoder).unwrap(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct PushVar {
    pub var: BuildVarInstructions,
}
impl Decode for PushVar {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        const BUILD: u8 = VarContext::Build as u8;
        if let [BUILD] = decoder.consume_n() {
            let name = Decode::decode(decoder).unwrap();
            Some(Self { var: name })
        } else {
            None
        }
    }
}
