use ir::{
    commands::kill::{Kill, KillType},
    variable::VariableType,
};

use crate::{
    Compile,
    runtime::{Decode, Encode, OpCodesForeign},
    variable::VarContext,
};

OpCodesForeign! {
    KillType{
        Inclusive => 0,
        Exclusive => 0,
    }
}

impl Compile for Kill {
    type Context = ();

    fn compile(&self, bite_code: &mut crate::BiteCode, context: &Self::Context) {
        self.variables.compile(bite_code, &VarContext::Build);
        bite_code.push(self.r#type.encode());
        bite_code.push(self.variables.len() as u8);
    }
}
