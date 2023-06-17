use std::ffi::CStr;
use crate::bindings::u_char;

#[no_mangle]
pub unsafe extern "C" fn opp(
    src: *mut *mut u_char
)-> u32 {
    let source = unsafe { CStr::from_ptr(*src as *const i8) }
        .to_str()
        .unwrap();
    let (offset, value) = operator(source);
    unsafe { (*src) = (*src).add(offset) };
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
        _ => unreachable!("should not be hit"),
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use std::ffi::CString;
    use crate::bindings::*;
    #[test]
    pub fn parse_op_code_test() {
        parse_op_code("+", OPADD);
        parse_op_code("-", OPSUB);
        parse_op_code("*", OPMUL);
        parse_op_code("/", OPDIV);
        parse_op_code("\\",OPINT);
        parse_op_code("#", OPMOD);
        parse_op_code("**",OPPOW);
        parse_op_code("_", OPCAT);
        parse_not_able_op_code("=", OPEQL, OPNEQL);
        parse_not_able_op_code("<", OPLES, OPNLES);
        parse_not_able_op_code(">", OPGTR, OPNGTR);
        parse_not_able_op_code("&", OPAND, OPNAND);
        parse_not_able_op_code("[", OPCON, OPNCON);
        parse_not_able_op_code("]", OPFOL, OPNFOL);
        parse_not_able_op_code("]]", OPSAF, OPNSAF);
        parse_not_able_op_code("?", OPPAT, OPNPAT);
    }

    pub fn parse_not_able_op_code(src: &str, opcode: u32, not_opcode: u32) {
        parse_op_code(src, opcode);
        parse_op_code(&format!("'{}", src), not_opcode);
    }

    pub fn parse_op_code(src: &str, opcode: u32) {
        let source = CString::new(format!("{}extra", dbg!(src))).unwrap().into_raw();
        //TODO this is being leaked.
        let mut source = source as *mut u8;
        {
            let source_temp = &mut source as *mut *mut u_char;
            assert_eq!(opcode, unsafe { opp(source_temp) } as u32);
            assert_eq!(
                Ok("extra"),
                unsafe { CStr::from_ptr(source as *const i8) }.to_str()
            );
        }
    }
}
