use super::*;
use crate::{
    bindings::partab_struct,
    eval::{eval, indirect_atom},
    function::{reserve_jump, write_jump},
    localvar::{parse_local_var, VarTypes},
    routine::extrinsic_function,
};
use pest::iterators::Pair;

pub fn commands(line: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    for command in line.into_inner() {
        let command_type = command.as_rule();
        let mut command = command.into_inner().peekable();

        let post_condition =
            command
                .next_if(|x| x.as_rule() == Rule::PostCondition)
                .map(|condition| {
                    eval(condition.into_inner().next().unwrap(), partab, comp);
                    comp.push(crate::bindings::JMP0);
                    reserve_jump(comp)
                });

        let args = command;
        if command_type == Rule::For {
            // For uses jump commands and requires special handling.
            for_command(args, partab, comp);
        } else {
            //These commands treat each argument independently.
            let arg_count = args
                .map(|arg| command_with_arg(command_type, arg, partab, comp))
                .count();

            if arg_count == 0 {
                comp.push(argumentless_command_op_code(command_type));
                if command_type == Rule::Do {
                    //TODO for some reason the c source dose not consume the second' '
                    //This leaves us with a second OPENDC;
                    comp.push(crate::bindings::OPENDC);
                }
            }
            if let Some(jump) = post_condition {
                write_jump(jump, comp.len(), comp)
            }
        }
        comp.push(crate::bindings::OPENDC);
    }
}
use core::iter::Peekable;
use pest::iterators::Pairs;
fn for_command(args: Peekable<Pairs<Rule>>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let (args, mut tail): (Vec<Pair<Rule>>, Vec<Pair<Rule>>) =
        args.partition(|x| x.as_rule() != Rule::Commands);
    let tail = tail.remove(0);

    let arg_less = args.is_empty();

    if arg_less {
        comp.push(crate::bindings::CMFOR0);
        let exit = reserve_jump(comp);
        commands(tail, partab, comp);
        comp.push(crate::bindings::OPENDC);
        //jump back to start of for loop.
        comp.push(crate::bindings::JMP);
        let jump = reserve_jump(comp);
        write_jump(jump, exit, comp);
        //jump out of for loop
        write_jump(exit, comp.len(), comp);
        comp.push(crate::bindings::OPNOP);
    } else {
        let mut args = args.into_iter();
        parse_local_var(args.next().unwrap(), partab, comp, VarTypes::For);
        let offset_for_code = reserve_jump(comp);
        let exit = reserve_jump(comp);
        for arg in args {
            let arg = arg.into_inner();
            let code = match arg.map(|x| eval(x, partab, comp)).count() {
                1 => crate::bindings::CMFOR1,
                2 => crate::bindings::CMFOR2,
                3 => crate::bindings::CMFOR3,
                _ => unreachable!(),
            };
            comp.push(code as u8);
        }
        write_jump(offset_for_code, comp.len(), comp);
        commands(tail, partab, comp);
        comp.push(crate::bindings::OPENDC);
        comp.push(crate::bindings::CMFOREND);
        write_jump(exit, comp.len(), comp);
        comp.push(crate::bindings::OPNOP);
    }
}

pub fn line(line: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let command_sequence = line.into_inner().next().unwrap();
    commands(command_sequence, partab, comp);
    comp.push(crate::bindings::ENDLIN);
}

fn argumentless_command_op_code(command: Rule) -> u8 {
    (match command {
        Rule::Brake => crate::bindings::OPBRK0,
        Rule::Close => unreachable!(),
        Rule::Do => crate::bindings::CMDON,
        Rule::Else => crate::bindings::OPELSE,
        _ => todo!(),
    }) as u8
}

fn command_with_arg(
    command: Rule,
    arg: Pair<Rule>,
    partab: &mut partab_struct,
    comp: &mut Vec<u8>,
) {
    match command {
        Rule::Brake => {
            eval(arg, partab, comp);
            comp.push(crate::bindings::OPBRKN);
        }
        Rule::Close => {
            use eval::IndAtomContext::Close;
            match arg.as_rule() {
                Rule::Exp => {
                    eval(arg, partab, comp);
                    comp.push(crate::bindings::CMCLOSE);
                }
                Rule::AtomInd => indirect_atom(arg, partab, comp, Close),
                _ => unreachable!(),
            }
        }
        Rule::Do => {
            let mut args = arg.into_inner().peekable();
            let extrinsic = args.next().unwrap();
            let post_condition =
                args.next_if(|x| x.as_rule() == Rule::PostCondition)
                    .map(|condition| {
                        eval(condition.into_inner().next().unwrap(), partab, comp);
                        comp.push(crate::bindings::JMP0);
                        reserve_jump(comp)
                    });

            extrinsic_function(extrinsic, partab, comp, false);
            if let Some(jump) = post_condition {
                write_jump(jump, comp.len(), comp)
            }
        }
        Rule::Else => unreachable!(),
        _ => todo!(),
    }
}

#[cfg(test)]
mod test {
    use crate::ffi::test::compile_c;
    use rstest::rstest;
    use crate::bindings;
    use crate::compile;

    #[test]
    fn multiple_commands() {
        let source_code = "w 9 w 8 w 7 w 6 w 5 w 4 w 3";
        let (orignal, _lock) = compile_c(source_code, bindings::parse);

        assert_eq!(orignal, compile(source_code));
    }

    #[rstest]
    #[case("b")]
    #[case("b  b  b")]
    #[case("b:something  ")]
    #[case("b 1")]
    #[case("b 1,2")]
    #[case("b 1,2 b 2")]
    #[case("c 1,2")]
    #[case("c @1")]
    #[case("d  ")]
    #[case("d tag")]
    #[case("d tag:12")]
    #[case("d tag(90):12,tag^rou:0")]
    #[case("e  ")]
    #[case("e  w 1")]
    #[case("f  ")]
    #[case("f  b  b  ")]
    #[case("f  f  b  ")]
    #[case("f  f  f  b  ")]
    #[case("f x=1 ")]
    #[case("f x=1:2 ")]
    #[case("f x=1:2:3 ")]
    #[case("f x=1,2:3,4:5:6 ")]
    fn command_test(#[case] source_code: &str) {
        let (orignal, _lock) = compile_c(source_code, bindings::parse);
        let temp = compile(source_code);

        assert_eq!(orignal, temp);
    }

    #[rstest]
    #[case("w 90")]
    #[case("w !")]
    #[case("w #")]
    #[case("w ?9")]
    #[case("w ?@temp")]
    #[case("w 1,#,!,?@temp")]
    fn write_command(#[case] source_code: &str) {
        let (orignal, _lock) = compile_c(source_code, bindings::parse);
        let temp = compile(source_code);

        assert_eq!(orignal, temp);
    }
}
