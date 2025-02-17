use std::{env, num::NonZero};

use crate::{
    Compile,
    bite_code::{self, BiteCode},
};
use ir::{
    Variable,
    variable::{Env, GlobleIdent, UserClassIdentifiers, VariableType},
};

#[derive(Clone, Copy)]
pub enum VarContext {
    Eval = ffi::OPVAR as isize,
    Build = ffi::OPMVAR as isize,
    BuildNullable = ffi::OPMVARN as isize,
    For = ffi::CMFORSET as isize,
}

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
                comp.push(ffi::INDMVAR);
            }
        }

        self.subscripts.compile(comp, &ExpressionContext::Eval);

        comp.push(*context as u8);
        let op_code = match &self.var_type {
            E::Named {
                globle_ident: None, ..
            } => ffi::TYPVARNAM,
            E::Named {
                globle_ident: Some(GlobleIdent { user_class: None }),
                ..
            } => ffi::TYPVARGBL,
            E::Named {
                globle_ident:
                    Some(GlobleIdent {
                        user_class: Some(user_class),
                    }),
                ..
            } => {
                if user_class.env.is_none() {
                    ffi::TYPVARGBLUCI
                } else {
                    ffi::TYPVARGBLUCIENV
                }
            }
            E::NakedVariable => ffi::TYPVARNAKED,
            E::IndirectVariable { .. } => ffi::TYPVARIND,
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
