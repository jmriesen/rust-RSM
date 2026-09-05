use value::Value;

pub const STRING_OP: u8 = 60;
use crate::{
    Compile,
    bite_code::BiteCode,
    runtime::{Decode, program_counter::AssemblyDecoder},
};
impl Compile for Value {
    type Context = ();

    fn compile(&self, bite_code: &mut BiteCode, _: &Self::Context) {
        let len = self.content().len() as u16;

        bite_code.push(STRING_OP);
        bite_code.extend(len.to_le_bytes());
        bite_code.extend(self.content().iter().cloned());
        bite_code.push(0);
    }
}
impl Decode for Value {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        //TODO: refactor to remove the from_bytes call.
        //The decoder compile/decode should be the canonical way to convert between types.
        if let [STRING_OP] = decoder.consume_n() {
            let len = u16::from_le_bytes(decoder.consume_n());
            let value = Value::new(decoder.consume(len as usize).to_vec());
            decoder.consume_n::<1>();
            Some(value)
        } else {
            None
        }
    }
}
