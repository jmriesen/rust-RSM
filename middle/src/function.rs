

pub fn reserve_jump(comp: &mut Vec<u8>) -> usize {
    comp.push(0);
    comp.push(0);
    comp.len()
}

pub fn write_jump(location: usize, jump_to: usize, comp: &mut [u8]) {
    let offset = (jump_to as i16 - location as i16).to_le_bytes();
    comp[location - 2..location].copy_from_slice(&offset);
}

impl<'a> crate::OpCode for crate::models::XCallCode<'a> {
    fn op_code(&self) -> u8 {
        use crate::models::XCallCode::*;
        match self {
            Directory(_) => crate::bindings::XCDIR,
            Host(_) => crate::bindings::XCHOST,
            File(_) => crate::bindings::XCFILE,
            ErrMsg(_) => crate::bindings::XCERR,
            OpCom(_) => crate::bindings::XCOPC,
            Signal(_) => crate::bindings::XCSIG,
            Spawn(_) => crate::bindings::XCSPA,
            Version(_) => crate::bindings::XCVER,
            Zwrite(_) => crate::bindings::XCZWR,
            E(_) => crate::bindings::XCE,
            Paschk(_) => crate::bindings::XCPAS,
            V(_) => crate::bindings::XCV,
            XCallX(_) => crate::bindings::XCX,
            Xrsm(_) => crate::bindings::XCXRSM,
            SetEnv(_) => crate::bindings::XCSETENV,
            GetEnv(_) => crate::bindings::XCGETENV,
            RouChk(_) => crate::bindings::XCROUCHK,
            Fork(_) => crate::bindings::XCFORK,
            IC(_) => crate::bindings::XCIC,
            Wait(_) => crate::bindings::XCWAIT,
            Debug(_) => crate::bindings::XCDEBUG,
            Compress(_) => crate::bindings::XCCOMP,
        }
    }
}

impl<'a> crate::OpCode for crate::models::IntrinsicVar<'a> {
    fn op_code(&self) -> u8 {
        use crate::models::IntrinsicVarChildren::*;
        match self.children() {
            Device(_) => crate::bindings::VARD,
            Ecode(_) => crate::bindings::VAREC,
            Estack(_) => crate::bindings::VARES,
            Etrap(_) => crate::bindings::VARET,
            Horolog(_) => crate::bindings::VARH,
            Io(_) => crate::bindings::VARI,
            Job(_) => crate::bindings::VARJ,
            Key(_) => crate::bindings::VARK,
            Principal(_) => crate::bindings::VARP,
            Quit(_) => crate::bindings::VARQ,
            Reference(_) => crate::bindings::VARR,
            Storage(_) => crate::bindings::VARS,
            StackVar(_) => crate::bindings::VARST,
            System(_) => crate::bindings::VARSY,
            Test(_) => crate::bindings::VART,
            X(_) => crate::bindings::VARX,
            Y(_) => crate::bindings::VARY,
        }
    }
}

#[cfg(test)]
mod test {
    use core::ops::RangeInclusive;

    use crate::{bindings, test_compile_command, ffi::test::compile_c};
    use rstest::rstest;

    #[test]
    fn select_test() {
        let source_code = "w $s(1:2,3:4,5:6)";
        let (orignal, _lock) = compile_c(source_code, bindings::parse);
        let temp = test_compile_command(source_code);

        assert_eq!(orignal, temp);
    }
    #[rstest]
    #[case("View","V",2..=4)]
    #[case("Text","T",1..=1)]
    #[case("Translate","TR",2..=3)]
    #[case("Find","F",2..=3)]
    #[case("fnumber","Fn",2..=3)]
    #[case("Random","R",1..=1)]
    #[case("Reverse","Re",1..=1)]
    #[case("Piece","P",2..=4)]
    #[case("Justify","J",2..=3)]
    #[case("extract","E",1..=3)]
    #[case("ascii","a",1..=2)]
    #[case("char","c",1..=8)]
    //TODO test upper bounds of Char
    //currenrly getting segfale problby would need to increase the buffer.
    #[case("char","c",50..=50)]
    #[case("length","l",1..=2)]
    #[case("Stack","st",1..=2)]
    fn intrinsic_fun(
        #[case] full: &str,
        #[case] abbreviated: &str,
        #[case] range: RangeInclusive<usize>,
    ) {
        use core::iter::repeat;
        for val in range {
            let args = repeat("11011").take(val).collect::<Vec<_>>().join(",");

            {
                let source_code = format!("w ${}({})", full, args);
                let (orignal, _lock) = compile_c(&source_code, bindings::parse);

                assert_eq!(orignal, test_compile_command(&source_code));
            }
            {
                let source_code = format!("w ${}({})", abbreviated, args);
                let (orignal, _lock) = compile_c(&source_code, bindings::parse);
                let temp = test_compile_command(&source_code);

                assert_eq!(orignal, temp);
            }
        }
    }

    #[rstest]
    #[case("Data","D",1..=1)]
    #[case("Get","G",1..=2)]
    #[case("increment","i",1..=2)]
    #[case("name","na",1..=2)]
    #[case("order","o",1..=2)]
    #[case("query","q",1..=2)]
    #[case("Next","n",1..=1)]
    #[case("Qlength","QL",1..=1)]
    #[case("QSUBSCRIPT","Qs",2..=2)]
    fn intrinsic_variable_fn(
        #[case] full: &str,
        #[case] abbreviated: &str,
        #[case] range: RangeInclusive<usize>,
    ) {
        use core::iter::repeat;
        for val in range {
            let args = repeat("variable").take(val).collect::<Vec<_>>().join(",");
            {
                let source_code = format!("w ${}({})", full, args);
                let (orignal, _lock) = compile_c(&source_code, bindings::parse);

                assert_eq!(orignal, test_compile_command(&source_code));
            }
            {
                let source_code = format!("w ${}({})", abbreviated, args);
                let (orignal, _lock) = compile_c(&source_code, bindings::parse);
                let temp = test_compile_command(&source_code);

                assert_eq!(orignal, temp);
            }
        }
    }
}
