use value::Value;

pub const STRING_OP: u8 = 60;
use crate::{
    Compile,
    bite_code::BiteCode,
    runtime::{Decode, program_counter::AssemballyDecoder},
};
impl Compile for Value {
    type Context = ();

    fn compile(&self, bite_code: &mut BiteCode, _: &Self::Context) {
        bite_code.push(STRING_OP);
        bite_code.extend(self.as_bytes());
        bite_code.push(0);
    }
}
impl Decode for Value {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        if let [STRING_OP] = decoder.consume_n() {
            let (value, amount_to_consume) = {
                let original_len = decoder.tail().len();
                let (value, new_tail) = Value::from_bytes(decoder.tail());
                let after_parsing_len = new_tail.len();
                (value, original_len - after_parsing_len)
            };
            decoder.consume(amount_to_consume + 1);
            Some(value)
        } else {
            None
        }
    }
}
