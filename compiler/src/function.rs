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
            Directory(_) => ffi::XCDIR,
            Host(_) => ffi::XCHOST,
            File(_) => ffi::XCFILE,
            ErrMsg(_) => ffi::XCERR,
            OpCom(_) => ffi::XCOPC,
            Signal(_) => ffi::XCSIG,
            Spawn(_) => ffi::XCSPA,
            Version(_) => ffi::XCVER,
            Zwrite(_) => ffi::XCZWR,
            E(_) => ffi::XCE,
            Paschk(_) => ffi::XCPAS,
            V(_) => ffi::XCV,
            XCallX(_) => ffi::XCX,
            Xrsm(_) => ffi::XCXRSM,
            SetEnv(_) => ffi::XCSETENV,
            GetEnv(_) => ffi::XCGETENV,
            RouChk(_) => ffi::XCROUCHK,
            Fork(_) => ffi::XCFORK,
            IC(_) => ffi::XCIC,
            Wait(_) => ffi::XCWAIT,
            Debug(_) => ffi::XCDEBUG,
            Compress(_) => ffi::XCCOMP,
        }
    }
}

impl<'a> crate::OpCode for crate::models::IntrinsicVar<'a> {
    fn op_code(&self) -> u8 {
        use crate::models::IntrinsicVarChildren::*;
        match self.children() {
            Device(_) => ffi::VARD,
            Ecode(_) => ffi::VAREC,
            Estack(_) => ffi::VARES,
            Etrap(_) => ffi::VARET,
            Horolog(_) => ffi::VARH,
            Io(_) => ffi::VARI,
            Job(_) => ffi::VARJ,
            Key(_) => ffi::VARK,
            Principal(_) => ffi::VARP,
            Quit(_) => ffi::VARQ,
            Reference(_) => ffi::VARR,
            Storage(_) => ffi::VARS,
            StackVar(_) => ffi::VARST,
            System(_) => ffi::VARSY,
            Test(_) => ffi::VART,
            X(_) => ffi::VARX,
            Y(_) => ffi::VARY,
        }
    }
}

#[cfg(test)]
mod test {
    use core::ops::RangeInclusive;

    use crate::{test_compile_command, test_harness::test::compile_c};
    use rstest::rstest;

    #[test]
    fn select_test() {
        let source_code = "w $s(1:2,3:4,5:6)";
        let (orignal, _lock) = compile_c(source_code, ffi::parse);
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
                let (orignal, _lock) = compile_c(&source_code, ffi::parse);

                assert_eq!(orignal, test_compile_command(&source_code));
            }
            {
                let source_code = format!("w ${}({})", abbreviated, args);
                let (orignal, _lock) = compile_c(&source_code, ffi::parse);
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
                let (orignal, _lock) = compile_c(&source_code, ffi::parse);

                assert_eq!(orignal, test_compile_command(&source_code));
            }
            {
                let source_code = format!("w ${}({})", abbreviated, args);
                let (orignal, _lock) = compile_c(&source_code, ffi::parse);
                let temp = test_compile_command(&source_code);

                assert_eq!(orignal, temp);
            }
        }
    }
}
