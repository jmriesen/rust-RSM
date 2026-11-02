use crate::{
    Compile, Compile2, StackAssembally,
    byte_code::ByteCode,
    runtime::{Decode, TEMP, program_counter::AssemblyDecoder},
    var_u,
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
                comp.push(INDIRECT_VAR);
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
            symbol_table::Ident::try_from(name.0.as_str())
                .unwrap()
                .compile(comp, &());
        }
    }
}
impl Compile2 for Variable {
    type Context = VarContext;
    fn compile_new(&self, comp: &mut Vec<StackAssembally>, context: &VarContext) {
        use crate::expression::ExpressionContext;
        use VariableType as E;
        //TODO: Deal with restructuring the variable encoding logic.
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
                user_class.uci.compile_new(comp, &ExpressionContext::Eval);
                if let Some(Env(env)) = &user_class.env {
                    env.compile_new(comp, &ExpressionContext::Eval);
                }
            }
            E::NakedVariable => {}
            E::IndirectVariable { expression } => {
                expression.compile_new(comp, &ExpressionContext::Eval);
                comp.push(StackAssembally::TEMP(TEMP(INDIRECT_VAR)));
            }
        }

        self.subscripts.compile_new(comp, &ExpressionContext::Eval);

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

        if op_code != VarCodes::Named as u8 {
            todo!("not yet implemented")
        }
        let name = if let E::Named { name, .. } = &self.var_type {
            Some(symbol_table::Ident::try_from(name.0.as_str()).unwrap())
        } else {
            None
        };
        let var = BuildVarInstructions {
            name: name.expect("Only supporting named variables right now."),
            subscripts: self.subscripts.len(),
        };
        let asm = match context {
            VarContext::Eval => StackAssembally::LoadVar(LoadVar { var }),
            VarContext::Build => StackAssembally::PushVar(PushVar { var }),
            VarContext::BuildNullable | VarContext::For => {
                let mut push_raw = |value| {
                    comp.push(StackAssembally::TEMP(TEMP(value)));
                };
                if matches!(
                    self.var_type,
                    E::Named {
                        globle_ident: None | Some(GlobleIdent { user_class: None }),
                        ..
                    }
                ) {
                    //This is the path I have always being going down.
                    push_raw(*context as u8);
                    push_raw(op_code + self.subscripts.len() as u8);
                } else {
                    //This is the path I have not being going down.
                    push_raw(*context as u8);
                    push_raw(op_code);
                    push_raw(self.subscripts.len() as u8);
                }
                let mut comp = ByteCode::new();

                if let E::Named { name, .. } = &self.var_type {
                    symbol_table::Ident::try_from(name.0.as_str())
                        .unwrap()
                        .compile(&mut comp, &());
                };
                let mut comp = comp.get_raw();
                let last = comp.pop().expect("variable name");

                for i in comp {
                    push_raw(i);
                }
                StackAssembally::TEMP(TEMP(last))
            }
        };
        comp.push(asm);
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
}
