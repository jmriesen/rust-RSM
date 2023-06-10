#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::all)]
#[allow(dead_code)]
mod bindings;
use crate::bindings::*;

pub mod temp;

use std::ffi::CStr;
use std::ffi::CString;


#[test]
pub fn parse_op_code_test() {
    parse_op_code("+", bindings::OPADD);
    todo!();
    parse_op_code("-", bindings::OPSUB);
    parse_op_code("*", bindings::OPMUL);
    parse_op_code("/", bindings::OPDIV);
    parse_op_code("\\", bindings::OPINT);
    parse_op_code("#", bindings::OPMOD);
    parse_op_code("**", bindings::OPPOW);
    parse_op_code("_", bindings::OPCAT);
    parse_not_able_op_code("=", bindings::OPEQL, bindings::OPNEQL);
    parse_not_able_op_code("<", bindings::OPLES, bindings::OPNLES);
    parse_not_able_op_code(">", bindings::OPGTR, bindings::OPNGTR);
    parse_not_able_op_code("&", bindings::OPAND, bindings::OPNAND);
    parse_not_able_op_code("[", bindings::OPCON, bindings::OPNCON);
    parse_not_able_op_code("]", bindings::OPFOL, bindings::OPNFOL);
    parse_not_able_op_code("]]", bindings::OPSAF, bindings::OPNSAF);
    parse_not_able_op_code("?", bindings::OPPAT, bindings::OPNPAT);
}

pub fn parse_not_able_op_code(src: &str, opcode: u32, not_opcode: u32) {
    parse_op_code(src, opcode);
    parse_op_code(&format!("'{}", src), not_opcode);
}

pub fn parse_op_code(src: &str, opcode: u32) {
    let mut source: Vec<u8> = CString::new(format!("{}extra", dbg!(src))).unwrap().into();
    {
        unsafe { source_ptr = source.as_mut_ptr() };
        assert_eq!(opcode, unsafe { bindings::operator() } as u32);
        assert_eq!(
            Ok("extra"),
            unsafe { CStr::from_ptr(source_ptr as *const i8) }.to_str()
        );
    }
}
