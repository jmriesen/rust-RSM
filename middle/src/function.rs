use crate::{bindings::partab_struct, eval::eval, Rule, localvar::{parse_local_var,VarTypes}, SyntaxParser};
use pest::{iterators::Pair, Parser};

pub fn intrinsic_function(function: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let mut function = function.into_inner();

    let function = function.next().unwrap();
    let fn_type = function.as_rule();
    let mut args = function
        .into_inner();
    //select gets accepts a special syntax and gets converted into jump commands.
    if fn_type == Rule::Select{
        let jump_indexs:Vec<_> = args
            .map(|arg| {
                eval(arg, partab, comp);
                //reserving space for jump commands.
                comp.push(0);
                comp.push(0);
                comp.push(0);
                comp.len()
            }).collect();
        //select got to the end with out finding a value.
        comp.push(crate::bindings::OPERROR as u8);
        let errm4 = (-(crate::bindings::ERRM4 as i16)).to_le_bytes();
        comp.extend_from_slice(&errm4);

        //fill in jumps
        let mut jump_indexs = jump_indexs.iter();
        while let (Some(jmp0),Some(jmp)) =  (jump_indexs.next(),jump_indexs.next()){
            let len = comp.len();
            let mut write_jump = |location:usize,code:u8,jump_to:usize| {
                comp[location-3] = code;
                let offset = ((jump_to-location) as i16).to_le_bytes();
                comp[location-2..location]
                    .copy_from_slice(&offset);
            };
            write_jump(*jmp0,crate::bindings::JMP0 as u8,*jmp);
            write_jump(*jmp,crate::bindings::JMP as u8,len);
        }
    }else{
        //Some intrinsic act on variables them self rather then there value.
        match fn_type {
            Rule::Data
                | Rule::Get1
                | Rule::Get2
                | Rule::Increment1
                | Rule::Increment2 => {
                    parse_local_var(args.next().unwrap(), partab, comp, VarTypes::Build);
                },
            Rule::Name1
                | Rule::Name2
                | Rule::Order1
                | Rule::Order2
                | Rule::Query1
                | Rule::Query2 => {
                    parse_local_var(args.next().unwrap(), partab, comp, VarTypes::BuildNullable);
                },
            //Next is just Order but with a hard coded 2 arg.
            Rule::Next =>{
                parse_local_var(args.next().unwrap(), partab, comp, VarTypes::BuildNullable);
                eval(SyntaxParser::parse(Rule::Exp, "2").unwrap().next().unwrap(),partab,comp);
            }
            Rule::Text =>{
                todo!() //routine arg of -2.
            }
            _ => {},
        }

        let count =args.map(|arg| eval(arg, partab, comp))
            .count();
        comp.push(function_as_num(fn_type));

        if fn_type == Rule::Char {
            //Char is allowed a variable number of args.
            //Each arg is evaluated independently.
            comp.push(count as u8);
        }

    }
}

pub fn function_as_num(opcode: Rule) -> u8 {
    (match opcode {
        Rule::View2 => crate::bindings::FUNV2,
        Rule::View3 => crate::bindings::FUNV3,
        Rule::View4 => crate::bindings::FUNV4,
        Rule::Char => crate::bindings::FUNC,
        Rule::Text => crate::bindings::FUNT,
        Rule::Translate2 => crate::bindings::FUNTR2,
        Rule::Translate3 => crate::bindings::FUNTR3,
        Rule::Find2 => crate::bindings::FUNF2,
        Rule::Find3 => crate::bindings::FUNF3,
        Rule::FNumber2 => crate::bindings::FUNFN2,
        Rule::FNumber3 => crate::bindings::FUNFN3,
        Rule::Random => crate::bindings::FUNR,
        Rule::Reverse => crate::bindings::FUNRE,
        Rule::Piece2 => crate::bindings::FUNP2,
        Rule::Piece3 => crate::bindings::FUNP3,
        Rule::Piece4 => crate::bindings::FUNP4,
        Rule::Justify2 => crate::bindings::FUNJ2,
        Rule::Justify3 => crate::bindings::FUNJ3,
        Rule::Ascii1 => crate::bindings::FUNA1,
        Rule::Ascii2 => crate::bindings::FUNA2,
        Rule::Extract1 => crate::bindings::FUNE1,
        Rule::Extract2 => crate::bindings::FUNE2,
        Rule::Extract3 => crate::bindings::FUNE3,
        Rule::Data => crate::bindings::FUND,
        Rule::Get1 => crate::bindings::FUNG1,
        Rule::Get2 => crate::bindings::FUNG2,
        Rule::Increment1 => crate::bindings::FUNI1,
        Rule::Increment2 => crate::bindings::FUNI2,
        Rule::Length1 => crate::bindings::FUNL1,
        Rule::Length2 => crate::bindings::FUNL2,
        Rule::Name1 => crate::bindings::FUNNA1,
        Rule::Name2 => crate::bindings::FUNNA2,
        Rule::Order1 => crate::bindings::FUNO1,
        Rule::Order2 => crate::bindings::FUNO2,
        Rule::Next => crate::bindings::FUNO2,//Note Next uses Order2
        Rule::Query1 => crate::bindings::FUNQ1,
        Rule::Query2 => crate::bindings::FUNQ2,
        Rule::Stack1 => crate::bindings::FUNST1,
        Rule::Stack2 => crate::bindings::FUNST2,
        Rule::Qlength => crate::bindings::FUNQL,
        Rule::Qsubscript => crate::bindings::FUNQS,
        Rule::Select => 1,
        _ => unreachable!(),
    } as u8)
}

#[cfg(test)]
mod test {
    use core::ops::RangeInclusive;
    use std::iter::repeat;

    use crate::eval::test::test_eval;
    use rstest::rstest;

    #[rstest]
    #[case("View","V",2..=4)]
    //#[case("Text","T",1..=1)]
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
    #[case("length","c",1..=2)]
    #[case("Stack","st",1..=2)]
    fn x_call(#[case] full: &str, #[case] abbreviated: &str, #[case] range: RangeInclusive<usize>) {
        for val in range {
            let args = repeat("11011").take(val).collect::<Vec<_>>().join(",");

            test_eval(&format!("${}({})", full, args));
            test_eval(&format!("${}({})", abbreviated, args));
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
    fn x_call_on_variable(#[case] full: &str, #[case] abbreviated: &str, #[case] range: RangeInclusive<usize>) {
        for val in range {
            let args = repeat("variable").take(val).collect::<Vec<_>>().join(",");

            test_eval(&format!("${}({})", full, args));
            test_eval(&format!("${}({})", abbreviated, args));
        }
    }

    #[test]
    fn select_test(){
        test_eval("$s(1:2,3:4,5:6)");
    }
}
