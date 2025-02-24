use ir::ExternalCalls;

use crate::{Compile, bite_code::BiteCode};

impl Compile for ExternalCalls {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        use ExternalCalls::*;
        bite_code.push(match self {
            Directory => 240,
            Host => 237,
            File => 238,
            ErrMsg => 241,
            OpCom => 242,
            Signal => 236,
            Spawn => 243,
            Version => 244,
            Zwrite => 245,
            E => 246,
            Paschk => 247,
            V => 248,
            XCallX => 249,
            Xrsm => 250,
            SetEnv => 251,
            GetEnv => 252,
            RouChk => 253,
            Fork => 254,
            IC => 255,
            Wait => 234,
            Debug => 239,
            Compress => 235,
        });
    }
}
