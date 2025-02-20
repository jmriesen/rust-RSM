use crate::{Compile, bite_code::BiteCode};
use ir::{
    Variable,
    variable::{Env, GlobleIdent, VariableType},
};

#[derive(Clone, Copy)]
pub enum VarContext {
    Eval = 61 as isize,
    Build = 62 as isize,
    BuildNullable = 63 as isize,
    For = 177 as isize,
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
        if matches!(self.var_type, E::Named {
            globle_ident: None | Some(GlobleIdent { user_class: None }),
            ..
        }) {
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
