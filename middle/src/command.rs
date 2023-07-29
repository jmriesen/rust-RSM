use super::*;
use pest::iterators::Pair;
use crate::localvar::{parse_local_var, VarTypes};
use crate::routine::extrinsic_function;
use crate::eval::{eval, indirect_atom};
use crate::function::write_jump;
use crate::{bindings::partab_struct, function::reserve_jump};

pub fn commands(line: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>) {
    for command in line.into_inner().into_iter(){
        let command_type = command.as_rule();
        let mut command = command.into_inner().peekable();

        let post_condition = command
            .next_if(|x| x.as_rule() == Rule::PostCondition)
            .map(|condition|{
                eval(condition.into_inner().next().unwrap(),partab,comp);
                comp.push(crate::bindings::JMP0 as u8);
                reserve_jump(comp)
            });

        let args = command;
        if command_type == Rule::For{
            let (args,mut tail) :(Vec<Pair<Rule>>,Vec<Pair<Rule>>)
                = args.partition(|x| x.as_rule() != Rule::Commands);
            let tail = tail.remove(0);

            let arg_less = args.len() == 0;

            if arg_less{
                comp.push(crate::bindings::CMFOR0 as u8);
                let exit = reserve_jump(comp);
                commands(tail, partab, comp);
                comp.push(crate::bindings::OPENDC as u8);
                //jump back to start of for loop.
                comp.push(crate::bindings::JMP as u8);
                let jump = reserve_jump(comp);
                write_jump(jump, exit, comp);
                //jump out of for loop
                write_jump(exit, comp.len(), comp);
                comp.push(crate::bindings::OPNOP as u8);
            }else{
                let mut args = args.into_iter();
                parse_local_var(args.next().unwrap(), partab, comp, VarTypes::For);
                let offset_for_code = reserve_jump(comp);
                let exit = reserve_jump(comp);
                for arg in args{
                    let arg = arg.into_inner();
                    let code = match arg
                        .map(|x| eval(x ,partab,comp))
                        .count(){
                            1 => crate::bindings::CMFOR1,
                            2 => crate::bindings::CMFOR2,
                            3 => crate::bindings::CMFOR3,
                            _ => unreachable!(),
                        };
                    comp.push(code as u8);
                }
                write_jump(offset_for_code, comp.len(), comp);
                commands(tail, partab, comp);
                comp.push(crate::bindings::OPENDC as u8);
                comp.push(crate::bindings::CMFOREND as u8);
                write_jump(exit, comp.len(), comp);
                comp.push(crate::bindings::OPNOP as u8);
            }

        }else{

            //These commands treat each argument independently.
            let arg_count =args
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
                write_jump(jump, comp.len(), comp)
            }
        }
        comp.push(crate::bindings::OPENDC as u8);
    }
}

fn line(line: Pair<Rule>, partab: &mut partab_struct, comp: &mut Vec<u8>){
    let command_sequence = line.into_inner().next().unwrap();
    commands(command_sequence,partab,comp);
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
            use eval::IndAtomContext::Close;
            match arg.as_rule(){
                Rule::Exp =>{
                    eval(arg,partab,comp);
                    comp.push(crate::bindings::CMCLOSE as u8);
                },
                Rule::AtomInd =>indirect_atom(arg, partab, comp, Close),
                _=> unreachable!(),
            }
        },
        Rule::Do =>{
            let mut args = arg.into_inner().peekable();
            let extrinsic = args.next().unwrap();
            let post_condition =args
                .next_if(|x| x.as_rule() == Rule::PostCondition)
                .map(|condition|{
                    eval(condition.into_inner().next().unwrap(),partab,comp);
                    comp.push(crate::bindings::JMP0 as u8);
                    reserve_jump(comp)
                });

            extrinsic_function(extrinsic, partab, comp,false);
            if let Some(jump) = post_condition{
                write_jump(jump, comp.len(), comp)
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
    #[case("b  b  ")]
    #[case("b:something  ")]
    #[case("b 1")]
    #[case("b 1,2")]
    #[case("c 1,2")]
    #[case("c @1")]
    #[case("d  ")]
    #[case("d tag")]
    #[case("d tag:12")]
    #[case("d tag(90):12,tag^rou:0")]
    #[case("e  ")]
    fn command_test(#[case] command: &str) {
        compare_to_c(command,Rule::Line,line, crate::bindings::parse);
    }

    #[rstest]
    #[case("f  ")]
    #[case("f  b  b  ")]
    #[case("f  f  b  ")]
    #[case("f  f  f  b  ")]
    #[case("f  f  f  b  ")]
    #[case("f x=1 ")]
    #[case("f x=1:2 ")]
    #[case("f x=1:2:3 ")]
    #[case("f x=1,2:3,4:5:6 ")]
    fn for_command(#[case] command: &str) {
        compare_to_c(command,Rule::Line, line, crate::bindings::parse);
    }
}
