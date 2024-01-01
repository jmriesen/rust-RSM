use crate::models::{VariableHeading, Expression};

use super::*;


pub enum VarTypes {
    Eval = crate::bindings::OPVAR as isize,
    Build = crate::bindings::OPMVAR as isize,
    BuildNullable = crate::bindings::OPMVARN as isize,
    For = crate::bindings::CMFORSET as isize,
}

impl<'a> VariableHeading<'a>{
    pub fn op_code(heading:&Option<Self>)->u8{

        use models::VariableHeading::*;
        (if let Some(heading) = heading{
            match heading {
                NakedVariable(_) => bindings::TYPVARNAKED,
                IndirectVariable(_) => bindings::TYPVARIND,
                GlobalVariable(_) => bindings::TYPVARGBL,
                GlobalUciVariable(_) => bindings::TYPVARGBLUCI,
                GlobalUciEnvVariable(_) => bindings::TYPVARGBLUCIENV
            }
        }else{
            bindings::TYPVARNAM
        })as u8
    }
    fn union_length(heading:&Option<Self>)->bool{

        use models::VariableHeading::*;
        match heading {
            Some(GlobalVariable(_)) |None => true,
            _ =>false,
        }
    }
    fn args(heading:&Option<Self>)->Vec<Expression>{

        use models::VariableHeading::*;
        //TODO in theory I should be able to remove all of these alocations.
        if let Some(heading) = heading{
            match heading {
                NakedVariable(_) =>vec![],
                GlobalVariable(_) => vec![],
                GlobalUciVariable(exp) => vec![exp.children()],
                GlobalUciEnvVariable(exps) => exps.children(),
                IndirectVariable(exp) => vec![exp.children()],
            }
        }else{
            vec![]
        }
    }
}

impl<'a> crate::models::Variable<'a> {
    pub fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: VarTypes) {
        let heading = self.heading();
        VariableHeading::args(&heading).iter()
            .for_each(|x| x.compile(source_code, comp, ExpressionContext::Eval));
        if let Some(VariableHeading::IndirectVariable(_)) = heading{
            comp.push(bindings::INDMVAR);
        }

        //NOTE c docs says subscripts heading,
        //but that is not what the code outputs
        let subscripts = self.subs();
        subscripts
            .iter()
            .for_each(|x| x.compile(source_code, comp, ExpressionContext::Eval));

        comp.push(context as u8);
        let op_code = VariableHeading::op_code(&self.heading());
        if VariableHeading::union_length(&heading){
            comp.push(op_code | (subscripts.len() as u8) );
        }else{
            comp.push(op_code);
            comp.push(subscripts.len() as u8);

        }

        if let Some(name) = self.name() {
            //TODO abstract away.
            let name = name.node().utf8_text(source_code.as_bytes()).unwrap();
            let name = bindings::VAR_U::from(name);
            comp.extend(name.as_array())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{bindings, test_compile_command, ffi::test::compile_c};
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
        let (orignal, _lock) = compile_c(&source_code, bindings::parse);

        assert_eq!(orignal, test_compile_command(&source_code));
    }
}
