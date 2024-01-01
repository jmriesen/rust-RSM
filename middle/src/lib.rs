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
mod eval;
mod ffi;
mod function;
mod localvar;
mod op_code;
mod routine;
mod var;

use models::UnaryOpp;

use crate::function::{reserve_jump, write_jump};

pub mod models {
    use tree_sitter::Node;
    lang_models::models!();

    pub fn create_tree(source_code: &str) -> tree_sitter::Tree {
        use tree_sitter::Parser;
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_mumps::language()).unwrap();

        #[cfg(test)]
        let tree = parser.parse(source_code, None).unwrap();
        #[cfg(test)]
        dbg!(&tree.root_node().to_sexp());

        parser.parse(source_code, None).unwrap()
    }

    pub fn type_tree<'a>(
        tree: &'a tree_sitter::Tree,
        source_code: &'a str,
    ) -> Result<source_file<'a>, ()> {
        use tree_sitter::{Query, QueryCursor};
        let mut query_cursor = QueryCursor::new();
        let error_query = Query::new(tree_sitter_mumps::language(), "(ERROR)").unwrap();
        let errors = query_cursor.matches(&error_query, tree.root_node(), source_code.as_bytes());
        if errors.count() != 0 {
            Err(())
        } else {
            Ok(source_file::create(tree.root_node()))
        }
    }
}

use op_code::ExpressionContext;
impl<'a> models::Expression<'a> {
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: ExpressionContext) {
        use crate::bindings::PARTAB;
        use eval::ncopy;
        use models::ExpressionChildren::*;
        match self.children() {
            number(num) => {
                let num = num.node().utf8_text(source_code.as_bytes()).unwrap();
                ncopy(num, &mut PARTAB::default(), comp);
            }
            string(value) => {
                use crate::eval::compile_string_literal;
                let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                compile_string_literal(value, comp);
            }
            Variable(var) => var.compile(source_code, comp, VarTypes::Eval),
            IntrinsicVar(var) => {
                comp.push(var.op_code());
            }
            Expression(exp) => exp.compile(source_code, comp, ExpressionContext::Eval),
            InderectExpression(exp) => {
                exp.children()
                    .compile(source_code, comp, ExpressionContext::Eval);
                comp.push(context as u8);
            }
            UnaryExpression(unary_exp) => {
                unary_exp
                    .exp()
                    .compile(source_code, comp, ExpressionContext::Eval);
                comp.push(unary_exp.opp().op_code());
            }
            BinaryExpression(bin_exp) => {
                bin_exp
                    .exp_left()
                    .compile(source_code, comp, ExpressionContext::Eval);
                bin_exp
                    .exp_right()
                    .compile(source_code, comp, ExpressionContext::Eval);
                comp.push(bin_exp.opp().op_code());
            }
            PaternMatchExpression(pat_exp) => {
                use models::{PaternMatchExpressionExp_right::*, PatternOppChildren::*};
                pat_exp
                    .exp_left()
                    .compile(source_code, comp, ExpressionContext::Eval);
                match pat_exp.exp_right() {
                    Expression(exp) => exp.compile(source_code, comp, ExpressionContext::Eval),
                    Patern(value) => {
                        //TODO remove duplication
                        use crate::eval::compile_string_literal;
                        let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                        compile_string_literal(&format!("\"{}\"", value), comp);
                    }
                }
                comp.push(match pat_exp.opp().children() {
                    OPPAT(_) => bindings::OPPAT,
                    OPNPAT(_) => bindings::OPNPAT,
                });
            }
            ExtrinsicFunction(x) => {
                x.compile(source_code, comp, ExtrinsicFunctionContext::Eval);
            }
            XCall(x) => {
                use crate::eval::compile_string_literal;
                x.args()
                    .iter()
                    .for_each(|x| x.compile(source_code, comp, ExpressionContext::Eval));

                for _ in x.args().len()..2 {
                    compile_string_literal("\"\"", comp);
                }
                comp.push(x.code().op_code());
            }

            IntrinsicFunction(intrinsic) => {
                use models::IntrinsicFunctionChildren::*;
                match &intrinsic.children() {
                    ExpFunctions(exp_fun) => {
                        use models::ExpFunctionsChildren::*;
                        let children = &exp_fun.children();
                        let (opcode, args) = match children {
                            View(x) => (crate::bindings::FUNV2 - 2, x.args()),
                            //TODO Text handling should be more detailed.
                            Text(x) => (crate::bindings::FUNT - 1, vec![x.args()]),
                            Translate(x) => (crate::bindings::FUNTR2 - 2, x.args()),
                            Find(x) => (crate::bindings::FUNF2 - 2, x.args()),
                            Fnumber(x) => (crate::bindings::FUNFN2 - 2, x.args()),
                            Random(x) => (crate::bindings::FUNR - 1, vec![x.args()]),
                            Reverse(x) => (crate::bindings::FUNRE - 1, vec![x.args()]),
                            Piece(x) => (crate::bindings::FUNP2 - 2, x.args()),
                            Justify(x) => (crate::bindings::FUNJ2 - 2, x.args()),
                            Extract(x) => (crate::bindings::FUNE1 - 1, x.args()),
                            Ascii(x) => (crate::bindings::FUNA1 - 1, x.args()),
                            Length(x) => (crate::bindings::FUNL1 - 1, x.args()),
                            Stack(x) => (crate::bindings::FUNST1 - 1, x.args()),
                            Char(x) => (crate::bindings::FUNC, x.args()),
                            //TODO handle select. It dose not work like the others.
                        };
                        let count = args
                            .iter()
                            .map(|x| x.compile(source_code, comp, ExpressionContext::Eval))
                            .count();
                        if opcode == crate::bindings::FUNC {
                            if count > 254 {
                                panic!("Char has too many args");
                            } else {
                                comp.push(opcode);
                                comp.push(count as u8);
                            }
                        } else {
                            comp.push(opcode + count as u8);
                        }
                    }
                    VarFunctions(exp_fun) => {
                        use models::VarFunctionsChildren::*;
                        let children = &exp_fun.children();
                        let (opcode, var, args) = match children {
                            Name(x) => (crate::bindings::FUNNA1 - 1, x.var(), x.args()),
                            Order(x) => (crate::bindings::FUNO1 - 1, x.var(), x.args()),
                            Query(x) => (crate::bindings::FUNQ1 - 1, x.var(), x.args()),
                            Increment(x) => (crate::bindings::FUNI1 - 1, x.var(), x.args()),
                            Get(x) => (crate::bindings::FUNG1 - 1, x.var(), x.args()),
                            //TODO Next is an allisas for Order + hard coded param.
                            Next(x) => (crate::bindings::FUNO2 - 1, x.var(), None),
                            Data(x) => (crate::bindings::FUND - 1, x.var(), None),
                            Qlength(x) => (crate::bindings::FUNQL - 1, x.var(), None),
                            Qsubscript(x) => (crate::bindings::FUNQS - 2, x.var(), Some(x.args())),
                        };
                        let var_type = match children {
                            Data(_) | Get(_) | Increment(_) => VarTypes::Build,
                            Name(_) | Order(_) | Query(_) | Next(_) => VarTypes::BuildNullable,
                            Qlength(_) | Qsubscript(_) => VarTypes::Eval,
                        };

                        var.compile(source_code, comp, var_type);
                        let count = args
                            .iter()
                            .map(|x| x.compile(source_code, comp, ExpressionContext::Eval))
                            .count();
                        if let Next(_) = children {
                            ncopy("2", &mut PARTAB::default(), comp);
                        }

                        comp.push(opcode + count as u8 + 1);
                    }
                    Select(select) => {
                        let jump_indexs = select
                            .children()
                            .array_chunks::<2>()
                            .map(|[condition, value]| {
                                condition.compile(source_code, comp, ExpressionContext::Eval);
                                comp.push(crate::bindings::JMP0);
                                let try_next = reserve_jump(comp);

                                value.compile(source_code, comp, ExpressionContext::Eval);
                                comp.push(crate::bindings::JMP);
                                let exit = reserve_jump(comp);

                                (try_next, exit)
                            })
                            .collect::<Vec<_>>();

                        comp.push(crate::bindings::OPERROR);
                        let errm4 = (-(crate::bindings::ERRM4 as i16)).to_le_bytes();
                        comp.extend_from_slice(&errm4);

                        for (try_next, exit) in jump_indexs {
                            write_jump(try_next, exit, comp);
                            write_jump(exit, comp.len(), comp);
                        }
                    }
                }
            }
        }
    }

    fn is_inderect(&self) -> bool {
        matches!(
            self.children(),
            models::ExpressionChildren::InderectExpression(_)
        )
    }
}

use crate::localvar::VarTypes;

///Test harness that for commands
///
///Wraps the provided command in addtional formatting before calling compile.
///This is needed since the only type tree sitter can parse is the source_file.
///
#[cfg(test)]
pub fn test_compile_command(source_code: &str) -> Vec<u8> {
    let source_code = source_code.replace("\n", "\n ");
    let source_code = &format!("tag {source_code}\n");
    compile(&source_code)
}

pub fn compile(source_code: &str) -> Vec<u8> {
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

impl<'a> crate::models::command<'a> {
    fn argumentless(&self) -> bool {
        use crate::models::commandChildren as E;
        match self.children() {
            E::WriteCommand(command) => command.args().is_empty(),
            E::BrakeCommand(command) => command.args().is_empty(),
            E::CloseCommand(command) => command.args().is_empty(),
            E::For(command) => command.args().is_empty(),
            E::DoCommand(command) => command.args().is_empty(),
            E::ElseCommand(_) => true,
            E::NewCommand(_) => true,
            E::QUITCommand(_) => true,

        }
    }
}
enum ExtrinsicFunctionContext {
    Eval,
    Do,
}

impl<'a> crate::models::ExtrinsicFunction<'a> {
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: ExtrinsicFunctionContext) {
        use models::ExtrinsicFunctionArgs::*;
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
                //Therefor the args falue is 1 higher then it should be
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
