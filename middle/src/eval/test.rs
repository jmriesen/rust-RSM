use std::sync::Mutex;
static guard: Mutex<()> = Mutex::new(());
use super::*;
use crate::{bindings::PARTAB, ffi::test::*};

pub fn test_eval(src: &str) {
    use crate::bindings::{comp_ptr, source_ptr, systab, MVAR, SYSTAB, TRANTAB, VAR_U};
    use std::io::Write;

    //TODO this is being leaked.
    let source = CString::new(dbg!(src)).unwrap();
    let source = source.into_raw() as *mut u8;

    let mut par_tab = PARTAB {
        jobtab: null_mut(),
        vol_fds: [0; 1],
        jnl_fds: [0; 1],
        debug: 0,
        strstk_start: null_mut(),
        strstk_last: null_mut(),
        varlst: null_mut(),
        checkonly: 0,
        errors: 0,
        sp: null_mut(),
        lp: null_mut(),
        ln: null_mut(),
        src_var: MVAR {
            name: VAR_U { var_q: 0 },
            volset: 0,
            uci: 0,
            slen: 0,
            key: [0; 256],
        },
    };
    //TODO Figure out how these different fields actually work.
    //Zeroing out for now so that I can avoid seg faults
    let mut sys_tab = SYSTAB {
        address: null_mut(),
        jobtab: null_mut(),
        maxjob: 0,
        sem_id: 0,
        historic: crate::bindings::HISTORIC_DNOK as i32,
        precision: 0,
        max_tt: 0,
        tt: [TRANTAB {
            from_global: VAR_U { var_cu: [0; 32] },
            from_vol: 0,
            from_uci: 0,
            to_global: VAR_U { var_cu: [0; 32] },
            to_vol: 0,
            to_uci: 0,
        }; 8],
        start_user: 0,
        lockstart: null_mut(),
        locksize: 0,
        lockhead: null_mut(),
        lockfree: null_mut(),
        addoff: 0,
        addsize: 0,
        vol: [null_mut(); 1],
        last_blk_used: [0; 1],
    };

    //TODO something about the value of 100 can cause the len to be calculated incorrectly.
    //attributing to some of the unsafe code in this test harness.
    //I think it should be fine for now, after all my end goal is to remove the unsafe C all together.
    const buffer_len: usize = 600;
    let mut compiled_original = [0u8; buffer_len];
    let _ = std::io::stdout().flush();
    let compile_stack_len = {
        let _lock = guard.lock();
        unsafe { source_ptr = source };
        unsafe { comp_ptr = compiled_original.as_mut_ptr() };
        unsafe { partab = par_tab.clone() };
        unsafe { systab = &mut sys_tab as *mut SYSTAB };
        unsafe { crate::bindings::eval() }
        unsafe { comp_ptr.offset_from(compiled_original.as_ptr()) }
    };

    use core::ptr::null_mut;

    let mut byte_code = vec![];
    use crate::pest::Parser;
    eval(
        dbg!(SyntaxParser::parse(Rule::Exp, src))
            .unwrap()
            .next()
            .unwrap(),
        &mut par_tab,
        &mut byte_code,
    );

    assert_eq!(
        compiled_original[..compile_stack_len as usize],
        byte_code[..]
    );
    use crate::bindings::partab;
    unsafe { assert_eq!(any_as_u8_slice(&partab), any_as_u8_slice(&par_tab)) };
}

use rstest::rstest;

#[test]
fn pattern_match() {
    test_eval("SomeString?.A");
    test_eval("SomeString?1.3A");
    test_eval("SomeString?.(8A,1(1N))");
    test_eval("SomeString?.2A");
    test_eval("SomeString?1.A");
}

#[rstest]
#[case("9")]
#[case("+-+-+-+-+-234")]
#[case("10000000")]
#[case("-10000")]
#[case("--45")]
#[case("00000001")]
#[case("0.1")]
#[case("0.00001")]
#[case("0.0")]
#[case(".0000000")]
#[case("0.0000000")]
#[case("0.000010000")]
#[case("00000000.00000000")]
//TODO implement HISTORIC_EOK
//#[case("1E100")]
//#[case("1E-100")]
//#[case("1.90E-100")]
fn parse_number(#[case] num: &str) {
    test_eval(num);
}

#[rstest]
#[case("\"Some string\"")]
#[case("\"Some numbers89097\"")]
#[case("\" string with quote\"\"quote\"\"\" some text\"")]
fn parse_string(#[case] num: &str) {
    test_eval(num);
}

#[rstest]
#[case("-98")]
#[case("'98")]
#[case("+98")]
fn parse_unary_exp(#[case] num: &str) {
    test_eval(num);
}

#[rstest]
#[case("98+9")]
#[case("-98\\var(7,9)")]
#[case("98+(something+9)")]
fn parse_exp(#[case] num: &str) {
    test_eval(num);
}
