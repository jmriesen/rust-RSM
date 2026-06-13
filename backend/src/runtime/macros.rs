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
