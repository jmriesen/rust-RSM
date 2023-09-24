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



use crate::function::{reserve_jump, write_jump};

pub mod models {
    use tree_sitter::Node;
    lang_models::models!();

    pub fn create_tree(source_code: &str) -> tree_sitter::Tree {
        use tree_sitter::Parser;
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_mumps::language()).unwrap();

        let tree = parser.parse(source_code, None).unwrap();

        #[cfg(test)]
        dbg!(&tree.root_node().to_sexp());
        tree
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

//TODO consider replacing the context with an enum.
pub enum ExpressionContext {
    Write,
    Eval,
    Close,
}

impl<'a> models::Expression<'a> {
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, context: ExpressionContext) {
        use crate::bindings::PARTAB;
        use eval::ncopy;
        use models::ExpressionChildren::*;
        match self.children() {
            BinaryExpression(bin_exp) => {
                bin_exp
                    .exp_left()
                    .compile(source_code, comp, ExpressionContext::Eval);
                bin_exp
                    .exp_right()
                    .compile(source_code, comp, ExpressionContext::Eval);
                comp.push(bin_exp.opp().op_code());
            }
            IntrinsicVar(var) => {
                comp.push(var.op_code());
            }
            Expression(exp) => exp.compile(source_code, comp, ExpressionContext::Eval),
            InderectExpression(exp) => {
                exp.children()
                    .compile(source_code, comp, ExpressionContext::Eval);
                //TODO note hardcoded at the moment
                use ExpressionContext as E;
                comp.push(match context {
                    E::Eval => bindings::INDEVAL,
                    E::Write => bindings::INDWRIT,
                    E::Close => bindings::INDCLOS,
                });
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
            //TODO should inderect be considered a special case of unary?
            UnaryExpression(unary_exp) => {
                use models::UnaryOppChildren::*;
                unary_exp
                    .exp()
                    .compile(source_code, comp, ExpressionContext::Eval);
                comp.push(match unary_exp.opp().children() {
                    OPMINUS(_) => bindings::OPMINUS,
                    OPNOT(_) => bindings::OPNOT,
                    OPPLUS(_) => bindings::OPPLUS,
                });
            }
            ExtrinsicFunction(x) => {
                x.compile(source_code,comp,ExtrinsicFunctionContext::Eval);
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
            Variable(var) => var.compile(source_code, comp, VarTypes::Eval),
            number(num) => {
                let num = num.node().utf8_text(source_code.as_bytes()).unwrap();
                ncopy(num, &mut PARTAB::default(), comp);
            }

            string(value) => {
                use crate::eval::compile_string_literal;
                let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                compile_string_literal(value, comp);
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


pub fn compile(source_code: &str) -> Vec<u8> {
    //Tree sitters regex lib is limmited.
    //in M you can have an argumentless comannd iff it is the end of the line.
    //however at the moment I can only look ahead for the newline char not the end of file char.
    //This may be solvable, but for now it is simply easier to alwyas append a newline to the end of the souce code.
    let source_code = &format!("{source_code}\n");
    let tree = models::create_tree(source_code);
    let tree = models::type_tree(&tree, source_code).unwrap();

    let mut comp = vec![];

    let lines = tree.children();
    for line in lines {
        let mut for_jumps = vec![];
        let commands = line.children();
        for command in &commands {
            let post_condition = match &command.children() {
                E::Write(command) => command.post_condition(),
                E::Brake(command) => command.post_condition(),
                E::Close(command) => command.post_condition(),
                E::Do(command) => command.post_condition(),
                E::For(_) => None,
                E::Else(_) => None,
            }
            .map(|condition| {
                condition.compile(source_code, &mut comp, ExpressionContext::Eval);
                comp.push(crate::bindings::JMP0);
                reserve_jump(&mut comp)
            });
            use crate::models::commandChildren as E;
            match command.children() {
                E::Write(command) => {
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
                E::Brake(command) => {
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

                E::Close(command) => {
                    //TODO this should not allow for one 0 children;
                    let children = command.args();
                    for arg in children {
                        arg.compile(source_code, &mut comp, ExpressionContext::Close);
                        if !arg.is_inderect() {
                            comp.push(bindings::CMCLOSE);
                        }
                    }
                }

                E::Else(_) => {
                    comp.push(bindings::OPELSE);
                }
                E::Do(command) => {
                    if command.args().is_empty(){
                        comp.push(crate::bindings::CMDON);
                    }else{
                        for arg in command.args(){
                            let post_condion = arg.post_condition().map(|x|{
                                x.compile(source_code,&mut comp,ExpressionContext::Eval);
                                comp.push(crate::bindings::JMP0);
                                reserve_jump(&mut comp)
                            });

                            arg.function().compile(source_code,&mut comp,ExtrinsicFunctionContext::Do);
                            if let Some(jump)= post_condion{
                                write_jump(jump, comp.len(), &mut comp)
                            }
                        }
                    }
                },
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
            }
            //NOTE C bug?
            //if the command has arguments C dosent consume the trailing white space.
            //this causes extra end commands to be added.
            if !command.argumentless(){
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
            }else{
                if matches!(command.children(), crate::models::commandChildren::Do(_)){
                    comp.push(bindings::OPENDC);
                }
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
            E::Write(command) => command.args().is_empty(),
            E::Brake(command) => command.args().is_empty(),
            E::Close(command) => command.args().is_empty(),
            E::For(command) => command.args().is_empty(),
            E::Do(command) => command.args().is_empty(),
            E::Else(_) => true,
        }
    }
}
enum ExtrinsicFunctionContext{
    Eval,
    Do,
}
impl <'a>crate::models::ExtrinsicFunction<'a>{
    fn compile(&self,source_code:&str,comp:&mut Vec<u8>,context:ExtrinsicFunctionContext){
        use models::ExtrinsicFunctionArgs::*;
        let args = self.args();
        let tag = self.tag();
        let routine = self.routine();

        //If an arg is not explicitly given thenn we add a VARUNDF marker.
        let mut need_arg = true;
        let mut arg_count = 0;

        for arg in &args {
            need_arg = match arg {
                ArgDelimenator(_) => {
                    //handles "(," and ",," cases
                    //NOTE ",)" case is handled as if it was just ")"
                    if need_arg {
                        arg_count += 1;
                        comp.push(crate::bindings::VARUNDF);
                    }
                    true
                }
                ByRef(var) => {
                    assert!(need_arg);
                    var.children().compile(source_code, comp, VarTypes::Build);
                    arg_count += 1;
                    comp.push(crate::bindings::NEWBREF);
                    false
                }
                Expression(exp) => {
                    assert!(need_arg);
                    arg_count += 1;
                    exp.compile(source_code, comp, ExpressionContext::Eval);
                    false
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
            use models::ExtrinsicFunctionTag::*;
            let node = match tag {
                identifier(x) => x.node(),
                NumericIdentifier(x) => x.node(),
            };

            let tag = node.utf8_text(source_code.as_bytes()).unwrap();
            let tag = var_u::from(tag);
            comp.extend(tag.as_array());
        }

        let marker = match context{
            ExtrinsicFunctionContext::Do=>{
                //NOTE on line parse.c:241
                //args is incremented before we check for ")"
                //Therefor the args falue is 1 higher then it should be
                let source = self.node().utf8_text(source_code.as_bytes()).unwrap();
                if source.contains("(")
                {
                    1
                }else{
                    0
                }
            },
            ExtrinsicFunctionContext::Eval=>{
                129
            },
        };
        comp.push(arg_count+marker);
    }
}

