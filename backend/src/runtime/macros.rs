macro_rules! OpCode {
    ($name:ident=$code:expr) => {
        #[derive(Debug)]
        pub struct $name;
        impl Decode for $name {
            fn decode(
                decoder: &mut crate::runtime::byte_code::AssemballyDecoder<'_>,
            ) -> Option<Self> {
                if let [$code] = decoder.consume_n() {
                    Some(Self)
                } else {
                    None
                }
            }
        }
        impl $name {
            pub fn encode(self) -> u8 {
                $code
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
        #[derive(Debug)]
        pub enum $name {
            $($var = $code,)*
        }
        impl Decode for $name{
            fn decode(decoder: &mut crate::runtime::byte_code::AssemballyDecoder<'_>) -> Option<Self > {
                let [code] = decoder.consume_n();
                    match code {
                        $($code => Some(Self::$var),)*
                        _ => None,
                    }
            }
    }
}
}
pub(crate) use OpCodes;

macro_rules! OpCodesForeign {
    ($name:ident{
        $($var:ident=>$code:expr,)*
}) => {
        impl Decode for $name{
            fn decode(decoder: &mut crate::runtime::byte_code::AssemballyDecoder<'_>) -> Option<Self > {
                let [code] = decoder.consume_n();
                    match code {
                        $($code => Some(Self::$var),)*
                        _ => None,
                    }
            }
    }

        impl Encode for $name{
            fn encode(&self) -> u8 {
                match self{
                $(Self::$var => $code,)*
                }
            }
        }
    };
}
pub(crate) use OpCodesForeign;

macro_rules! StackAssembally{
    (
        $($instruction:ident,)*
) => {
#[derive(Debug)]
pub enum StackAssembally {
    $($instruction($instruction),)*
    }

$(impl StackAssemballyTrait for $instruction{})*

impl<'a> ByteCode<'a> {
    pub fn next(&mut self) -> StackAssembally {
        //Starting with none so that everything follows the same pattern.
        None
        $(.or_else(|| self.try_decode().map(StackAssembally::$instruction)))*
            .expect("Provided source was invalid/corruped")
    }
}
}
}
pub(crate) use StackAssembally;
