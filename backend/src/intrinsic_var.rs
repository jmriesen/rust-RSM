use ir::IntrinsicVar;

use crate::{macros::OpCodesForeign, runtime::LegacyEncoding};
OpCodesForeign!(
    IntrinsicVar {
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
}
);
