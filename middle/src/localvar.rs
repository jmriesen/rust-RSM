use super::*;
use crate::{
    bindings::{partab_struct, var_u},
};

pub enum VarTypes {
    Eval,
    Build,
    BuildNullable,
    For,
}

impl VarTypes {
    pub fn code(self) -> u8 {
        use VarTypes::*;
        match self {
            Eval => crate::bindings::OPVAR,
            Build => crate::bindings::OPMVAR,
            BuildNullable => crate::bindings::OPMVARN,
            For => crate::bindings::CMFORSET,
        }
    }
}
impl<'a> crate::models::Variable<'a> {
    pub fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: VarTypes) {
        let subscripts = self.subs();

        use models::VariableHeading::*;
        let var_type = self
            .heading()
            .map(|heading| match &heading {
                NakedVariable(_) => bindings::TYPVARNAKED,
                IndirectVariable(exp) => {
                    exp.children()
                        .compile(source_code, comp, ExpressionContext::Eval);
                    comp.push(bindings::INDMVAR);
                    bindings::TYPVARIND
                }
                GlobalVariable(_) => bindings::TYPVARGBL,
                GlobalUciVariable(exp) => {
                    exp.children()
                        .compile(source_code, comp, ExpressionContext::Eval);
                    bindings::TYPVARGBLUCI
                }
                GlobalUciEnvVariable(exps) => {
                    exps.children()
                        .iter()
                        .for_each(|x| x.compile(source_code, comp, ExpressionContext::Eval));
                    bindings::TYPVARGBLUCIENV
                }
            })
            .unwrap_or(bindings::TYPVARNAM);

        //NOTE c docs says subscripts heading,
        //but that is not what the code outputs
        subscripts
            .iter()
            .for_each(|x| x.compile(source_code, comp, ExpressionContext::Eval));

        comp.push(context.code());
        match var_type {
            crate::bindings::TYPVARGBL | crate::bindings::TYPVARNAM => {
                comp.push((var_type | subscripts.len() as u32) as u8);
            }
            _ => {
                comp.push(var_type as u8);
                comp.push(subscripts.len() as u8);
            }
        }
        if let Some(name) = self.name() {
            let name = name.node().utf8_text(source_code.as_bytes()).unwrap();
            let name = bindings::VAR_U::from(name);
            comp.extend(name.as_array())
        }
    }
}



#[cfg(test)]
mod test {
    use rstest::rstest;
    use crate::ffi::test::compile_c;
    use crate::compile;
    use crate::bindings;
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
        let (orignal, _lock) = compile_c(&source_code, bindings::parse);

        assert_eq!(orignal, compile(&source_code));
    }
}
