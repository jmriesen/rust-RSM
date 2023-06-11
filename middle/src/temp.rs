use pest::iterators::Pair;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn opp()-> u32 {
    let source = unsafe { CStr::from_ptr(crate::source_ptr as *const i8) }
        .to_str()
        .unwrap();
    let (offset, value) = operator(source);
    unsafe { crate::source_ptr = crate::source_ptr.add(offset) };
    return value;
}

use pest::Parser;

#[derive(Parser)]
#[grammar = "opcode.pest"]
pub struct OpCodeParser;

pub fn operator(source: &str) -> (usize, u32) {
    if let Ok(mut code) = OpCodeParser::parse(Rule::PestOpcode, source) {
        let code = code.next().unwrap().into_inner().next().unwrap();
        (code.as_str().len(), opcode_as_num(code.as_rule()))
    }else{
        (0,0)
    }
}

fn opcode_as_num(opcode: Rule) -> u32 {
    use crate::bindings;
    use Rule::*;
    match opcode {
        PestOpcode => unreachable!("should not be hit"),
        OPADD => bindings::OPADD,
        OPSUB => bindings::OPSUB,
        OPMUL => bindings::OPMUL,
        OPDIV => bindings::OPDIV,
        OPINT => bindings::OPINT,
        OPMOD => bindings::OPMOD,
        OPPOW => bindings::OPPOW,
        OPCAT => bindings::OPCAT,
        OPGTR => bindings::OPGTR,
        OPAND => bindings::OPAND,
        OPCON => bindings::OPCON,
        OPFOL => bindings::OPFOL,
        OPPAT => bindings::OPPAT,
        OPEQL => bindings::OPEQL,
        OPLES => bindings::OPLES,
        OPNEQL => bindings::OPNEQL,
        OPNLES => bindings::OPNLES,
        OPNGTR => bindings::OPNGTR,
        OPNAND => bindings::OPNAND,
        OPNCON => bindings::OPNCON,
        OPNFOL => bindings::OPNFOL,
        OPNSAF => bindings::OPNSAF,
        OPSAF => bindings::OPSAF,
        OPNPAT => bindings::OPNPAT,
    }
}
