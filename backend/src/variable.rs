use crate::{Compile, bite_code::BiteCode};
use ir::{Variable, variable::VariableType};

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
            E::Local { .. } => {}
            E::NakedVariable => {}
            E::IndirectVariable { expression } => {
                expression.compile(comp, &ExpressionContext::Eval);
                comp.push(ffi::INDMVAR);
            }
            E::GlobalVariable { .. } => {}
            E::GlobalUciVariable { uci, .. } => {
                uci.compile(comp, &ExpressionContext::Eval);
            }
            E::GlobalUciEnvVariable { uci, env, .. } => {
                uci.compile(comp, &ExpressionContext::Eval);
                env.compile(comp, &ExpressionContext::Eval);
            }
        }

        for subscript in &self.subscripts {
            subscript.compile(comp, &ExpressionContext::Eval);
        }

        comp.push(*context as u8);
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
