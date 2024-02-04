#![feature(array_chunks)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::all)]
#[allow(dead_code)]
pub mod bindings;

mod command;
mod dollar;
mod expression;
mod ffi;
mod function;
mod localvar;
mod op_code;
mod routine;
mod var;

use crate::function::{reserve_jump, write_jump};

mod models{
    //TODO clean up and remove glob inport.
    pub use lang_model::*;
}

use crate::localvar::VarTypes;


trait Compileable{
    type Context;
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>,contex:Self::Context);
}

trait OpCode{
   fn op_code(&self) -> u8 ;
}

///Test harness that for commands
///
///Wraps the provided command in addtional formatting before calling compile.
///This is needed since the only type tree sitter can parse is the source_file.
///
#[cfg(test)]
pub fn test_compile_command(source_code: &str) -> Vec<u8> {
    let source_code = source_code.replace('\n', "\n ");
    let source_code = &format!("tag {source_code}\n");
    compile(source_code)
}

pub fn compile(source_code: &str) -> Vec<u8> {
    use expression::ExpressionContext;
        let tree = models::create_tree(dbg!(source_code));
    let tree = models::type_tree(&tree, source_code).unwrap();

    let mut comp = vec![];
    let tags = tree.children();
    let block = tags[0].block().unwrap();
    for line in block.children(){
        let line = match line{
            models::BlockChildren::line(line)=>line,
            models::BlockChildren::Block(_line)=>continue,
        };
        let mut for_jumps = vec![];
        let commands = line.children();
        for command in &commands {
            let post_condition = match &command.children() {
                E::WriteCommand(command) => command.post_condition(),
                E::BrakeCommand(command) => command.post_condition(),
                E::CloseCommand(command) => command.post_condition(),
                E::DoCommand(command) => command.post_condition(),
                E::For(_) => None,
                E::ElseCommand(_) => None,
                E::NewCommand(_) => None,
                E::QUITCommand(_) => todo!(),
            }
            .map(|condition| {
                condition.compile(source_code, &mut comp, ExpressionContext::Eval);
                comp.push(crate::bindings::JMP0);
                reserve_jump(&mut comp)
            });
            use crate::models::commandChildren as E;
            match command.children() {
                E::WriteCommand(command) => {
                    for arg in command.args() {
                        use crate::models::WriteArgChildren as E;
                        match arg.children() {
                            E::Bang(_) => comp.push(bindings::CMWRTNL),
                            E::Clear(_) => comp.push(bindings::CMWRTFF),
                            E::Tab(tab) => {
                                tab.children().compile(
                                    source_code,
                                    &mut comp,
                                    ExpressionContext::Eval,
                                );
                                comp.push(bindings::CMWRTAB);
                            }
                            E::Expression(exp) => {
                                exp.compile(source_code, &mut comp, ExpressionContext::Write);
                                if !exp.is_inderect() {
                                    comp.push(bindings::CMWRTEX);
                                }
                            }
                        }
                    }
                }
                E::BrakeCommand(command) => {
                    let children = command.args();
                    if children.is_empty() {
                        comp.push(bindings::OPBRK0);
                    } else {
                        for arg in children {
                            arg.compile(source_code, &mut comp, ExpressionContext::Eval);
                            if !arg.is_inderect() {
                                comp.push(bindings::OPBRKN);
                            }
                        }
                    }
                }

                E::CloseCommand(command) => {
                    //TODO this should not allow for one 0 children;
                    let children = command.args();
                    for arg in children {
                        arg.compile(source_code, &mut comp, ExpressionContext::Close);
                        if !arg.is_inderect() {
                            comp.push(bindings::CMCLOSE);
                        }
                    }
                }

                E::ElseCommand(_) => {
                    comp.push(bindings::OPELSE);
                }
                E::NewCommand(_command) => {}
                E::DoCommand(command) => {
                    if command.args().is_empty() {
                        comp.push(crate::bindings::CMDON);
                    } else {
                        for arg in command.args() {
                            let post_condion = arg.post_condition().map(|x| {
                                x.compile(source_code, &mut comp, ExpressionContext::Eval);
                                comp.push(crate::bindings::JMP0);
                                reserve_jump(&mut comp)
                            });

                            arg.function().compile(
                                source_code,
                                &mut comp,
                                ExtrinsicFunctionContext::Do,
                            );
                            if let Some(jump) = post_condion {
                                write_jump(jump, comp.len(), &mut comp)
                            }
                        }
                    }
                }
                E::For(command) => match command.variable() {
                    Some(var) => {
                        var.compile(source_code, &mut comp, VarTypes::For);
                        let offset_for_code = reserve_jump(&mut comp);
                        let exit = reserve_jump(&mut comp);

                        for args in command.args() {
                            for exp in args.children() {
                                exp.compile(source_code, &mut comp, ExpressionContext::Eval);
                            }

                            comp.push(match args.children().len() {
                                1 => crate::bindings::CMFOR1,
                                2 => crate::bindings::CMFOR2,
                                3 => crate::bindings::CMFOR3,
                                _ => unreachable!(),
                            });
                        }

                        write_jump(offset_for_code, comp.len(), &mut comp);
                        for_jumps.push((exit, false));
                    }
                    None => {
                        comp.push(crate::bindings::CMFOR0);
                        for_jumps.push((reserve_jump(&mut comp), true));
                    }
                },
                E::QUITCommand(_) => todo!(),
            }
            //NOTE C bug?
            //if the command has arguments C dosent consume the trailing white space.
            //this causes extra end commands to be added.
            if !command.argumentless() {
                comp.push(bindings::OPENDC);
            }
            if let Some(jump) = post_condition {
                write_jump(jump, comp.len(), &mut comp)
            }
            //For commans only end at the end of the line.
            if !matches!(command.children(), E::For(_)) {
                comp.push(bindings::OPENDC);
            }
        }
        if let Some(command) = &commands.last() {
            if !command.argumentless() {
                //NOTE C bug?
                //The last command in a line is not subjected to the bug.
                //This removes the addtional OPENDC I added to compensate for the other bug.
                comp.pop();
            } else if matches!(
                command.children(),
                crate::models::commandChildren::DoCommand(_)
            ) {
                comp.push(bindings::OPENDC);
            }
        }
        for (exit, argless) in for_jumps.into_iter().rev() {
            comp.push(crate::bindings::OPENDC);
            if argless {
                //jump back to start of for loop.
                comp.push(crate::bindings::JMP);
                let jump = reserve_jump(&mut comp);
                write_jump(jump, exit, &mut comp);
            } else {
                comp.push(crate::bindings::CMFOREND);
            }
            //jump out of for loop
            write_jump(exit, comp.len(), &mut comp);
            comp.push(crate::bindings::OPNOP);

            comp.push(crate::bindings::OPENDC);
        }
        comp.push(bindings::ENDLIN);
    }
    comp
}



enum ExtrinsicFunctionContext {
    Eval,
    Do,
}

impl<'a> Compileable for crate::models::ExtrinsicFunction<'a> {
    type Context = ExtrinsicFunctionContext;
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: ExtrinsicFunctionContext) {
        use models::ExtrinsicFunctionArgs::*;
        use crate::expression::ExpressionContext;
        let mut args = self.args();
        let tag = self.tag();
        let routine = self.routine();

        //NOTE It is easier to  just remove the traling VarUndefined when compiling then then durring parseing
        if args.last().is_some_and(|x| matches!(x,VarUndefined(_))){
            args.pop();
        }
        for arg in &args {
            match arg {
                VarUndefined(_) => {
                    comp.push(crate::bindings::VARUNDF);
                }
                ByRef(var) => {
                    var.children().compile(source_code, comp, VarTypes::Build);
                    comp.push(crate::bindings::NEWBREF);
                }
                Expression(exp) => {
                    exp.compile(source_code, comp, ExpressionContext::Eval);
                }
            };
        }

        let opcode = match (tag.is_some(), routine.is_some()) {
            (true, false) => crate::bindings::CMDOTAG,
            (false, true) => crate::bindings::CMDOROU,
            (true, true) => crate::bindings::CMDORT,
            _ => unreachable!(),
        };
        comp.push(opcode as u8);
        use crate::bindings::var_u;
        if let Some(routine) = routine {
            let routine = routine.node().utf8_text(source_code.as_bytes()).unwrap();
            let tag = var_u::from(routine);
            comp.extend(tag.as_array());
        }
        if let Some(tag) = &tag {
            use crate::models::TagNameChildren::*;
            let tag = tag.children();
            let node = match &tag {
                identifier(x) => x.node(),
                NumericIdentifier(x) => x.node(),
            };

            let tag = node.utf8_text(source_code.as_bytes()).unwrap();
            let tag = var_u::from(tag);
            comp.extend(tag.as_array());
        }

        let marker = match context {
            ExtrinsicFunctionContext::Do => {
                //NOTE on line parse.c:241
                //args is incremented before we check for ")"
                //Therefor the args value is 1 higher then it should be
                let source = self.node().utf8_text(source_code.as_bytes()).unwrap();
                if source.contains('(') {
                    1
                } else {
                    0
                }
            }
            ExtrinsicFunctionContext::Eval => 129,
        };
        comp.push(args.len() as u8+ marker);
    }
}
