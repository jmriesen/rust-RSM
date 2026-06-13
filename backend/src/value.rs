use value::Value;

pub const STRING_OP: u8 = 60;
use crate::{Compile, bite_code::BiteCode, runtime::Decode};
impl Compile for Value {
    type Context = ();

    fn compile(&self, bite_code: &mut BiteCode, _: &Self::Context) {
        bite_code.push(STRING_OP);
        bite_code.extend(self.as_bytes());
        bite_code.push(0);
    }
}
impl Decode for Value {
    fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
        if code == STRING_OP {
            let (value, new_tail) = Value::from_bytes(tail);
            Some((value, &new_tail[1..]))
        } else {
            None
        }
    }
}
