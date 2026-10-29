use crate::Compile;
use ir::commands::set::Set;

use crate::{
    byte_code::ByteCode, expression::ExpressionContext, macros::OpCodes, runtime::Decode,
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
    fn compile(&self, comp: &mut ByteCode, _: &()) {
        self.value.compile(comp, &ExpressionContext::Eval);
        self.variable.compile(comp, &VarContext::Build);
        comp.push(SetCodes::Var as u8);
    }
}
