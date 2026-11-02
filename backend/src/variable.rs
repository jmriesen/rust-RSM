use crate::{
    Compile, StackAssembally,
    byte_code::ByteCode,
    runtime::{Decode, TEMP, program_counter::AssemblyDecoder},
};
use ir::{
    Variable,
    variable::{Env, GlobleIdent, VariableType},
};
use serde::{Deserialize, Serialize};
use symbol_table::Ident;

#[derive(Clone, Copy)]
pub enum VarContext {
    Eval = 61_isize,
    Build = 62_isize,
    BuildNullable = 63_isize,
    For = 177_isize,
}

pub enum VarCodes {
    Named = 0,
    //Global
    Global = 128,
    Naked = 252,
    GlobalUci = 253,
    GlobalUciEnv = 254,
    IndirectVariable = 255,
}

const INDIRECT_VAR: u8 = 66;
impl Compile for Variable {
    type Context = VarContext;
    fn compile(&self, comp: &mut ByteCode, context: &VarContext) {
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
                comp.push_asm(StackAssembally::TEMP(TEMP(INDIRECT_VAR)));
            }
        }

        self.subscripts.compile(comp, &ExpressionContext::Eval);

        let op_code = match &self.var_type {
            E::Named {
                globle_ident: None, ..
            } => VarCodes::Named,
            E::Named {
                globle_ident: Some(GlobleIdent { user_class: None }),
                ..
            } => VarCodes::Global,
            E::Named {
                globle_ident:
                    Some(GlobleIdent {
                        user_class: Some(user_class),
                    }),
                ..
            } => {
                if user_class.env.is_none() {
                    VarCodes::GlobalUci
                } else {
                    VarCodes::GlobalUciEnv
                }
            }
            E::NakedVariable => VarCodes::Naked,
            E::IndirectVariable { .. } => VarCodes::IndirectVariable,
        } as u8;

        let mut old_encoding = || {
            let mut push_old = |value| {
                comp.push_asm(StackAssembally::TEMP(TEMP(value)));
            };
            //TOOD: remove branch eventually
            push_old(*context as u8);
            //Use a slightly more compact format if all we have to worry about is subscripts
            if matches!(
                self.var_type,
                E::Named {
                    globle_ident: None | Some(GlobleIdent { user_class: None }),
                    ..
                }
            ) {
                push_old(op_code + self.subscripts.len() as u8);
            } else {
                push_old(op_code);
                push_old(self.subscripts.len() as u8);
            }
            if let E::Named { name, .. } = &self.var_type {
                symbol_table::Ident::try_from(name.0.as_str())
                    .unwrap()
                    .compile(comp, &());
            };
        };
        if let VariableType::Named {
            name,
            globle_ident: None,
        } = &self.var_type
        {
            let name = symbol_table::Ident::try_from(name.0.as_str()).unwrap();

            let var = BuildVarInstructions {
                name,
                //"Only supporting named variables right now."),
                subscripts: self.subscripts.len(),
            };
            match context {
                VarContext::Eval => comp.push_asm(StackAssembally::LoadVar(LoadVar { var })),
                VarContext::Build => comp.push_asm(StackAssembally::PushVar(PushVar { var })),
                _ => old_encoding(),
            }
        } else {
            old_encoding()
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BuildVarInstructions {
    pub name: Ident,
    pub subscripts: usize,
}
impl Decode for BuildVarInstructions {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        //TODO: handle other types
        let [code_num_subscriptions] = decoder.consume_n();
        let variable_string = decoder.consume_n::<32>();
        let variable_string: Vec<_> = variable_string
            .iter()
            .take_while(|x| **x != 0)
            .cloned()
            .collect();
        Some(Self {
            name: Ident::new(&variable_string).unwrap(),
            //TODO: handle parsing different types of variables.
            subscripts: code_num_subscriptions as usize,
        })
    }
    fn encode(&self, byte_code: &mut crate::ByteCode) {
        use crate::byte_code::EncoderExt;
        byte_code.push_raw(
            self.subscripts
                .try_into()
                .expect("TODO: to many variable detection"),
        );
        self.name.encode(byte_code);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoadVar {
    pub var: BuildVarInstructions,
}
impl Decode for LoadVar {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        const CODE: u8 = VarContext::Eval as u8;
        if let [CODE] = decoder.consume_n() {
            Some(Self {
                var: Decode::decode(decoder).unwrap(),
            })
        } else {
            None
        }
    }
    fn encode(&self, byte_code: &mut crate::ByteCode) {
        use crate::byte_code::EncoderExt;
        byte_code.push_raw(VarContext::Eval as u8);
        self.var.encode(byte_code);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushVar {
    pub var: BuildVarInstructions,
}
impl Decode for PushVar {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        const BUILD: u8 = VarContext::Build as u8;
        if let [BUILD] = decoder.consume_n() {
            let name = Decode::decode(decoder).unwrap();
            Some(Self { var: name })
        } else {
            None
        }
    }
    fn encode(&self, byte_code: &mut crate::ByteCode) {
        use crate::byte_code::EncoderExt;
        byte_code.push_raw(VarContext::Build as u8);
        self.var.encode(byte_code);
    }
}
