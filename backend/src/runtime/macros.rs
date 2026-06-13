macro_rules! OpCode {
    ($name:ident=$code:expr) => {
        #[derive(Debug)]
        pub struct $name;
        impl Decode for $name {
            fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
                if code == $code {
                    Some((Self, tail))
                } else {
                    None
                }
            }
        }
        impl $name {
            fn encode(self) -> u8 {
                $code
            }
        }
    };
}
pub(crate) use OpCode;
macro_rules! OpCodes {
    ($name:ident{
        $($var:ident=$code:expr,)*
}) => {
        #[derive(Debug)]
        pub enum $name {
            $($var = $code,)*
        }
        impl Decode for $name{
            fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
                match code {
                $($code => Some(Self::$var),)*
                    _ => None,
                }
                .map(|x| (x, tail))
            }
        }
    };
}
pub(crate) use OpCodes;

macro_rules! OpCodesForeign {
    ($name:ident{
        $($var:ident=>$code:expr,)*
}) => {
        impl Decode for $name{
            fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
                match code {
                $($code => Some(Self::$var),)*
                    _ => None,
                }
                .map(|x| (x, tail))
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
