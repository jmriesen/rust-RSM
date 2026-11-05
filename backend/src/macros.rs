macro_rules! OpCode {
    ($name:ident=$code:expr) => {
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct $name;
        impl LegacyEncoding for $name {
            fn decode(
                decoder: &mut crate::runtime::program_counter::AssemblyDecoder<'_>,
            ) -> Option<Self> {
                if let [$code] = decoder.consume_n() {
                    Some(Self)
                } else {
                    None
                }
            }
            fn encode(&self, byte_code: &mut crate::ByteCode) {
                use crate::byte_code::EncoderExt;
                byte_code.push_raw($code);
            }
        }
    };
}
pub(crate) use OpCode;
macro_rules! OpCodes {
    (
$name:ident{
    $($var:ident=$code:expr,)*
}
) => {
        #[derive(Debug,serde::Deserialize, serde::Serialize,Copy,Clone)]
        pub enum $name {
            $($var = $code,)*
        }
        impl LegacyEncoding for $name{
            fn decode(decoder: &mut crate::runtime::program_counter::AssemblyDecoder<'_>) -> Option<Self > {
                let [code] = decoder.consume_n();
                    match code {
                        $($code => Some(Self::$var),)*
                        _ => None,
                    }
            }
        fn encode(&self, byte_code: &mut crate::ByteCode) {
            use crate::byte_code::EncoderExt;
            byte_code.push_raw(self.clone() as u8);
    }
    }
}
}
pub(crate) use OpCodes;

macro_rules! OpCodesForeign {
    ($name:ident{
        $($var:ident=>$code:expr,)*
}) => {
        impl LegacyEncoding for $name{
            fn decode(decoder: &mut crate::runtime::program_counter::AssemblyDecoder<'_>) -> Option<Self > {
                let [code] = decoder.consume_n();
                    match code {
                        $($code => Some(Self::$var),)*
                        _ => None,
                    }
            }
    fn encode(&self, byte_code: &mut crate::ByteCode) {
        use crate::byte_code::EncoderExt;
        byte_code.push_raw(
            match self{
                $(Self::$var => $code,)*
            }
        )
        }

    }
    };
}
pub(crate) use OpCodesForeign;

macro_rules! StackAssembally{
    (
        $($instruction:ident,)*
) => {
#[derive(Debug,serde::Deserialize, serde::Serialize)]
pub enum StackAssembally {
    $($instruction($instruction),)*
    }
    impl StackAssembally {
    pub fn encode(&self, decoder: &mut crate::ByteCode) {
        match self {
        $(StackAssembally::$instruction(x) => LegacyEncoding::encode(x,decoder),)*

        }
    }
    }

$(impl StackAssemblyTrait for $instruction{})*

    impl<'a> Iterator for ProgramCounter<'a> {
            type Item = StackAssembally;
            fn next(&mut self) -> Option<StackAssembally> {
                if self.has_next(){
                    None
                }else{
                    Some(
                        //Starting with none so that everything follows the same pattern.
                        None
                        $(.or_else(|| self.try_decode().map(StackAssembally::$instruction)))*
                            .expect("Provided source was invalid/corrupted")
                    )
                }
            }
        }
    }
}
pub(crate) use StackAssembally;
