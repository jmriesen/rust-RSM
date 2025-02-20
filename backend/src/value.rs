use value::Value;

const STRING_OP: u8 = 60;
use crate::{Compile, bite_code::BiteCode};
impl Compile for Value {
    type Context = ();

    fn compile(&self, bite_code: &mut BiteCode, _: &Self::Context) {
        bite_code.push(STRING_OP);
        bite_code.extend(self.as_bytes());
        bite_code.push(0);
    }
}
