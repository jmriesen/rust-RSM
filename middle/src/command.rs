use super::*;
use pest::iterators::Pair;
use crate::routine::extrinsic_function;
use crate::eval::eval;
use crate::function::write_jump;
use crate::{bindings::partab_struct, function::reserve_jump};

pub fn line(command: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    let command = command.into_inner().next().unwrap();
    let command_type = command.as_rule();
    let mut command = command.into_inner().peekable();

    let post_condition = command
        .next_if(|x| x.as_rule() == Rule::PostCondition)
        .map(|condition|{
            eval(condition.into_inner().next().unwrap(),partab,comp);
            reserve_jump(comp)
        });

        //These commands treat each argument independently.
        let arg_count = command
            .map(|arg| command_with_arg(command_type,arg,partab,comp))
            .count();

        if arg_count == 0 {
            comp.push(argumentless_command_op_code(command_type));
            if command_type == Rule::Do{
                //TODO for some reason the c source dose not consume the second' '
                //This leaves us with a second OPENDC;
                comp.push(crate::bindings::OPENDC as u8);
            }
        }

    if let Some(jump) = post_condition{
        write_jump(jump, crate::bindings::JMP0 as u8, comp.len(), comp)
    }

    comp.push(crate::bindings::OPENDC as u8);
    comp.push(crate::bindings::ENDLIN as u8);
}


fn argumentless_command_op_code(command:Rule)-> u8{
    (match command {
        Rule::Brake => crate::bindings::OPBRK0,
        Rule::Close => unreachable!(),
        Rule::Do => crate::bindings::CMDON,
        Rule::Else => crate::bindings::OPELSE,
        _=>todo!()
    }) as u8
}

fn command_with_arg(command:Rule,arg: Pair<Rule>,partab: &mut partab_struct,comp: &mut Vec<u8>){
    match command {
        Rule::Brake => {
            eval(arg,partab,comp);
            comp.push(crate::bindings::OPBRKN as u8);
        },
        Rule::Close =>{
            eval(arg,partab,comp);
            //TODO handle indirection
            comp.push(crate::bindings::CMCLOSE as u8);
        },
        Rule::Do =>{
            let mut args = arg.into_inner().peekable();
            let extrinsic = args.next().unwrap();
            let post_condition =args
                .next_if(|x| x.as_rule() == Rule::PostCondition)
                .map(|condition|{
                    eval(condition.into_inner().next().unwrap(),partab,comp);
                    reserve_jump(comp)
                });

            extrinsic_function(extrinsic, partab, comp,false);
            if let Some(jump) = post_condition{
                write_jump(jump, crate::bindings::JMP0 as u8, comp.len(), comp)
            }
        }
        Rule::Else => unreachable!(),
        _=>todo!()
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use rstest::rstest;
    use crate::ffi::test::*;

    #[rstest]
    #[case("b  ")]
    #[case("b:something  ")]
    #[case("b 1")]
    #[case("b 1,2")]
    #[case("c 1,2")]
    #[case("d  ")]
    #[case("d tag")]
    #[case("d tag:12")]
    #[case("d tag(90):12,tag^rou:0")]
    #[case("e  ")]
    fn command_test(#[case] command: &str) {
        compare_to_c(command,Rule::Line, line, crate::bindings::parse);
    }
}
