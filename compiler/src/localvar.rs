use crate::models::{Expression, VariableHeading};

use crate::Compileable;
use lang_model as models;

pub enum VarTypes {
    Eval = ffi::OPVAR as isize,
    Build = ffi::OPMVAR as isize,
    BuildNullable = ffi::OPMVARN as isize,
    For = ffi::CMFORSET as isize,
}

trait HeadingExt {
    fn op_code(&self) -> u8;
    fn union_length(&self) -> bool;
    fn args(&self) -> Vec<Expression>;
    fn is_indirect(&self) -> bool;
}

impl<'a> HeadingExt for Option<VariableHeading<'a>> {
    fn op_code(&self) -> u8 {
        use models::VariableHeading::*;
        (if let Some(heading) = self {
            match heading {
                NakedVariable(_) => ffi::TYPVARNAKED,
                IndirectVariable(_) => ffi::TYPVARIND,
                GlobalVariable(_) => ffi::TYPVARGBL,
                GlobalUciVariable(_) => ffi::TYPVARGBLUCI,
                GlobalUciEnvVariable(_) => ffi::TYPVARGBLUCIENV,
            }
        } else {
            ffi::TYPVARNAM
        }) as u8
    }
    fn union_length(&self) -> bool {
        use models::VariableHeading::*;
        matches!(self, Some(GlobalVariable(_)) | None)
    }
    fn args(&self) -> Vec<Expression> {
        use models::VariableHeading::*;
        //TODO in theory I should be able to remove all of these alocations.
        if let Some(heading) = self {
            match heading {
                NakedVariable(_) => vec![],
                GlobalVariable(_) => vec![],
                GlobalUciVariable(exp) => vec![exp.children()],
                GlobalUciEnvVariable(exps) => exps.children(),
                IndirectVariable(exp) => vec![exp.children()],
            }
        } else {
            vec![]
        }
    }
    fn is_indirect(&self) -> bool {
        use models::VariableHeading::*;
        matches!(self, Some(IndirectVariable(_)))
    }
}
impl<'a> Compileable for crate::models::Variable<'a> {
    type Context = VarTypes;
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: VarTypes) {
        use crate::expression::ExpressionContext;
        let heading = self.heading();

        for arg in heading.args() {
            arg.compile(source_code, comp, ExpressionContext::Eval);
        }

        if heading.is_indirect() {
            comp.push(ffi::INDMVAR);
        }

        //NOTE c docs says subscripts heading,
        //but that is not what the code outputs
        for subscript in self.subs() {
            subscript.compile(source_code, comp, ExpressionContext::Eval);
        }

        comp.push(context as u8);
        comp.push(heading.op_code());
        //Consider requiting this so we only push opcode once.
        if heading.union_length() {
            *comp.last_mut().unwrap() |= self.subs().len() as u8;
        } else {
            comp.push(self.subs().len() as u8);
        }

        if let Some(name) = self.name() {
            //TODO abstract away.
            let name: ffi::VAR_U = name
                .node()
                .utf8_text(source_code.as_bytes())
                .unwrap()
                .try_into()
                .unwrap();
            comp.extend(name.as_array())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{test_compile_command, test_harness::test::compile_c};
    use rstest::rstest;
    #[rstest]
    #[case("SomeString")]
    #[case("^SomeString")]
    #[case("^[atom]varName")]
    #[case("^|atom|varName")]
    #[case("^[atom1,atom2]varName")]
    #[case("SomeString(sub1)")]
    #[case("^SomeString(sub1)")]
    #[case("^(sub1)")]
    #[case("^|atom|varName(sub1)")]
    #[case("^[atom1,atom2]varName(sub1)")]
    #[case("SomeString(sub1,sub2)")]
    #[case("^SomeString(sub1,sub2)")]
    #[case("^|atom|varName(sub1,sub2)")]
    #[case("^[atom1,atom2]varName(sub1,sub2)")]
    #[case("@atom@(sub1,sub2)")]
    #[case("@varName")]
    //TODO index
    fn parse_var(#[case] num: &str) {
        let source_code = format!("w {}", num);
        let (orignal, _lock) = compile_c(&source_code, ffi::parse);

        assert_eq!(orignal, test_compile_command(&source_code));
    }
}
