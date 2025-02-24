use ir::IntrinsicVar;

use crate::{Compile, bite_code::BiteCode};

impl Compile for IntrinsicVar {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        use IntrinsicVar::*;
        bite_code.push(match self {
            Device => 80,
            Ecode => 81,
            Estack => 82,
            Etrap => 83,
            Horolog => 84,
            Io => 85,
            Job => 86,
            Key => 87,
            Principal => 88,
            Quit => 89,
            Reference => 90,
            Storage => 91,
            StackVar => 92,
            System => 93,
            Test => 94,
            X => 95,
            Y => 96,
        });
    }
}
