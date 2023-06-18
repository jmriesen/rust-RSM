use libc::c_short;
use pest::Parser;
use crate::bindings::u_char;
use crate::bindings::partab_struct;
use core::ffi::CStr;
use std::ffi::CString;
use crate::ffi::sync_with_c;

#[derive(Parser)]
#[grammar = "localvar.pest"]
pub struct LocalVar;

#[no_mangle]
pub unsafe extern "C" fn parse_local_var_ffi(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    par_tab: *mut partab_struct,
)->c_short{
    let source = unsafe { CStr::from_ptr(*src as *const i8) }
    .to_str()
        .unwrap();
    let (offset,byte_code) = dbg!(parse_local_var(source,&mut *par_tab));
    let temp = byte_code.len() as c_short;
    unsafe{sync_with_c(src, comp, offset, byte_code);}
    temp
}

pub fn parse_local_var(src:&str, _partab: &mut partab_struct)->(usize,Vec<u8>){
    if let Ok(mut code) = LocalVar::parse(Rule::Variable, src) {
        let outer = code.next().unwrap();
        let offset = outer.as_str().len();

        let mut variable = outer.into_inner();

        let variable_descriptor = variable.next().unwrap();
        let variable_descriptor_type = variable_descriptor.as_rule();
        let mut variable_descriptor = variable_descriptor.into_inner();

        let variableType = match variable_descriptor_type {
            Rule::GlobalVariable =>crate::bindings::TYPVARGBL,
            Rule::LocalVariable =>crate::bindings::TYPVARNAM,
            Rule::NakedVariable =>crate::bindings::TYPVARNAKED,
            Rule::GlobalUciVariable => {
                todo!();//exp
                crate::bindings::TYPVARGBLUCI
            },
            Rule::GlobalUciEnvVariable => {
                todo!();//exp
                todo!();//exp
                crate::bindings::TYPVARGBLUCIENV
            },
            _=> unimplemented!()
        };

        let name = dbg!(variable_descriptor.next().unwrap().as_str());
        let name = CString::new(name).unwrap();
        let name = name.into_bytes_with_nul().into_iter().chain(std::iter::repeat(0)).take(32);


        //TODO add subscripts;
        let count = 0;
        let mut output:Vec<u8> = vec![crate::bindings::OPVAR as u8];
        match variableType {
            crate::bindings::TYPVARGBL| crate::bindings::TYPVARNAM |
            crate::bindings::TYPVARNAKED => {
                output.push((variableType|count) as u8);
            },
            _ =>{
                output.push(variableType as u8);
                output.push(count as u8);
            }
        }

        output.extend(dbg!(name));

        (offset,output)
    }else{
        //TODO I am ignoring errors for now.
        todo!()
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::eval::test::test_eval;
    #[test]
    fn parse_variable(){
        test_eval("^SomeString");
        test_eval("SomeString");
        //TODO there seems to be special handling for intrinsics
        //Figure out what is going on
        //test_eval("$J");
        //TODO these require that I can start calling the eval function from C
        //test_eval("^|atom|");
        //test_eval("^[atom1,atom2]");
    }
}
