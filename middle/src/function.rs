use pest::iterators::Pair;
use crate::Rule;
use crate::bindings::partab_struct;
use crate::eval::eval;

pub fn intrinsic_function(function: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let mut function = function.into_inner();

    let function = function.next().unwrap();
    let fn_type = function.as_rule();

    let count = function.into_inner()
        .map(|arg| eval(arg, partab, comp))
        .count();
    comp.push(function_as_num(fn_type));

    if fn_type == Rule::Char{
        //Char is allowed a variable number of args.
        //Each arg is evaluated independently.
        comp.push(count as u8);
    }
}

pub fn function_as_num(opcode: Rule)->u8{
    (match opcode{
        Rule::View2 => crate::bindings::FUNV2,
        Rule::View3 => crate::bindings::FUNV3,
        Rule::View4 => crate::bindings::FUNV4,
        Rule::Char => crate::bindings::FUNC,
        Rule::Text => crate::bindings::FUNT,
        Rule::Translate2 => crate::bindings::FUNTR2,
        Rule::Translate3 => crate::bindings::FUNTR3,
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
        _ =>unreachable!()
    } as u8 )
}

#[cfg(test)]
mod test{
    use std::iter::repeat;
    use core::ops::RangeInclusive;

    use crate::eval::test::test_eval;
    use rstest::rstest;

    #[rstest]
    #[case("View","V",2..=4)]
    #[case("Text","T",1..=1)]
    #[case("Translate","TR",2..=3)]
    #[case("Random","R",1..=1)]
    #[case("Reverse","Re",1..=1)]
    #[case("Piece","P",2..=4)]
    #[case("Justify","J",2..=3)]
    #[case("extract","E",1..=3)]
    #[case("ascii","a",1..=2)]
    #[case("char","c",1..=8)]
    fn x_call(#[case] full: &str,#[case] abbreviated: &str,#[case] range:RangeInclusive<usize>) {
        for val in range{
            let args =
                repeat("11011")
                .take(val)
                .collect::<Vec<_>>()
                .join(",");

            test_eval(&format!("${}({})",full,args));
            test_eval(&format!("${}({})",abbreviated,args));
        }
    }
}
