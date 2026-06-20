use crate::Compile;
use ir::commands::set::Set;

use crate::{
    bite_code::BiteCode,
    expression::ExpressionContext,
    runtime::{Decode, OpCodes},
    variable::VarContext,
};

OpCodes! {
    SetCodes{
        Var = 41,
        //EXTRACT = 42,
        //PIECE = 43 ,
    }
}

impl Compile for Set {
    type Context = ();
    fn compile(&self, comp: &mut BiteCode, _: &()) {
        self.value.compile(comp, &ExpressionContext::Eval);
        self.variable.compile(comp, &VarContext::Build);
        comp.push(SetCodes::Var as u8);
    }
}
