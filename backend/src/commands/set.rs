use crate::{Compile, runtime::StackAssembally};
use ir::commands::set::Set;

use crate::{
    byte_code::ByteCode, expression::ExpressionContext, macros::OpCodes, runtime::LegacyEncoding,
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
        comp.push_asm(StackAssembally::SetCodes(SetCodes::Var));
    }
}
