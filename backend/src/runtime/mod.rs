use crate::{
    byte_code,
    commands::{
        r#do::DoArgLess,
        r#for::{ForEnd, ForFrame, ForMetaData, ForParameterType},
        r#if::IfElse,
        kill::KillInstruction,
        new::NewStackAsm,
        quit::QuitCodes,
        set::SetCodes,
        write::WriteCodes,
    },
    conditional_jumps::{Jump, JumpCodes},
    extrinsic_function::ExtrinsicFunctionCall,
    line_info::{EndLine, StartLine},
    macros::StackAssembally,
    operators::{BinaryApply, UnaryApply},
    routine::CompiledRoutine,
    runtime::{
        do_calls::{PushFrameArgs, build_arguments, push_do_frame},
        program_counter::{AssemblyDecoder, ProgramCounter},
    },
    variable::{BuildVarInstructions, LoadVar, PushVar},
};
use ir::{
    commands::NewKind,
    operators::{Binary, Unary},
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Debug};
use symbol_table::{MVar, SymbolTable, key::Path};
use thiserror::Error;
use value::Value;
mod do_calls;
pub mod program_counter;

#[derive(Error, PartialEq, Debug)]
pub(crate) enum RuntimeError {
    #[error("Undefined Index variable")]
    UndefinedIndexVariable,
    #[error("Not yet supported {}",.0)]
    NotYetSupported(&'static str),
    #[error("Too few formal parameters")]
    ToManyArgumentsPassed,
    #[error("Line must have a formal list")]
    NoParameterListDefined,
    #[error("Invalid line reference (line not found)")]
    CouldNotFindTagOrCouldNotFindRoutine,
}

#[derive(Debug)]
struct DoFrame<'a> {
    pc: ProgramCounter<'a>,
    // Metadata for all for loops.
    for_stack: Vec<ForFrame>,
    /// Stores the last result of the most resent if predicate.
    /// Used by else.
    test: bool,

    /// How indented are we.
    line_level: u16,
}

#[derive(Debug, PartialEq)]
enum StackValue {
    Value(value::Value),
    Var(MVar<Path>),
}
#[derive(Debug, PartialEq)]
pub struct Stack(Vec<StackValue>);
impl Stack {
    pub fn push_val(&mut self, value: value::Value) {
        self.0.push(StackValue::Value(value))
    }
    pub fn pop_val(&mut self) -> value::Value {
        match self.0.pop() {
            Some(StackValue::Value(value)) => value,
            Some(_) => panic!("Top of stack was not a mumps value"),
            None => panic!("Tried to pop empty Stack"),
        }
    }
    pub fn push_var(&mut self, value: MVar<Path>) {
        self.0.push(StackValue::Var(value))
    }
    pub fn pop_var(&mut self) -> MVar<Path> {
        match self.0.pop() {
            Some(StackValue::Var(value)) => value,
            Some(_) => panic!("Top of stack was not a mumps var"),
            None => panic!("Tried to pop empty Stack"),
        }
    }
}

pub struct Job<'a> {
    //TODO: Replace with a proper output device later.
    pub buffer: String,
    /// Stack of values
    stack: Stack,
    symbol_table: SymbolTable,
    /// Do Stack
    call_stack: Vec<DoFrame<'a>>,

    error: Option<RuntimeError>,
    routine: &'a CompiledRoutine,
}
/// This encoding scheme is based of what the original C code does.
/// NOTE: It is not 100% comparable and I have deviated when I found the original encoding cumbersome .
/// This is still used by regression testing code, but should not be considered stable.
/// This will likely be replaced by a more Rusty (serde) implementation once decide that I can
/// delete the reference encoding tests.
pub trait LegacyEncoding: Sized {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self>;
    fn encode(&self, decoder: &mut byte_code::ByteCode);
}

use crate::macros::OpCode;
//TODO: Consider if there should be a better abstraction for intrinsic variables.
OpCode! {Test=94}
OpCode! {EndCommand=4}
OpCode! {NoOpCode=179}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TEMP(pub u8);
#[cfg_attr(test, mutants::skip)]
impl LegacyEncoding for TEMP {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        let [code] = decoder.consume_n();
        //Always accept remove before production but helps during testing adding new types
        Some(Self(code))
    }
    fn encode(&self, byte_code: &mut byte_code::ByteCode) {
        use byte_code::EncoderExt;
        byte_code.push_raw(self.0);
    }
}

/// Marks something as a whole assembly instruction
pub(crate) trait StackAssemblyTrait: LegacyEncoding {}
StackAssembally! {
    LoadVar,
    SetCodes,
    Value,
    WriteCodes,
    Binary,
    Unary,
    EndLine,
    StartLine,
    EndCommand,
    ForMetaData,
    ForParameterType,
    ForEnd,
    NoOpCode,
    IfElse,
    KillInstruction,
    PushVar,
    QuitCodes,
    Jump,
    DoArgLess,
    Test,
    NewStackAsm,
    ExtrinsicFunctionCall,
    TEMP,
}
impl<'a> Job<'a> {
    pub fn new(routine: &'a CompiledRoutine) -> Self {
        let mut symbol_table = SymbolTable::default();
        symbol_table.push_new_frame();
        let mut job = Self {
            buffer: String::new(),
            stack: Stack(vec![]),
            symbol_table,
            error: None,
            call_stack: vec![],
            routine,
        };
        push_do_frame(
            &mut job.symbol_table,
            &mut job.call_stack,
            routine.beginning_of_file(),
            PushFrameArgs {
                arguments: [].into_iter(),
                line_level_zero: true,
            },
        );
        job
    }
    pub fn run(&mut self) {
        while let Some(do_frame) = self.call_stack.last_mut() {
            if let Some(instruction) = do_frame.pc.next() {
                match instruction {
                    StackAssembally::Value(value) => {
                        self.stack.push_val(value);
                    }
                    StackAssembally::WriteCodes(write_codes) => match write_codes {
                        WriteCodes::Bang => self.buffer.push('\n'),
                        WriteCodes::Clear => todo!(),
                        WriteCodes::Tab => todo!(),
                        WriteCodes::Expression => {
                            let value = self.stack.pop_val();
                            self.buffer
                                .push_str(core::str::from_utf8(value.content()).unwrap());
                        }
                    },
                    StackAssembally::Binary(op) => {
                        let second = self.stack.pop_val();
                        let first = self.stack.pop_val();
                        self.stack.push_val(op.apply(first, second));
                    }
                    StackAssembally::Unary(op) => {
                        let value = self.stack.pop_val();
                        self.stack.push_val(op.apply(value));
                    }
                    StackAssembally::StartLine(line_info) => {
                        match line_info.level.cmp(&do_frame.line_level) {
                            Ordering::Less => {
                                self.call_stack.pop();
                                self.symbol_table.pop_new_frame();
                            }
                            Ordering::Equal => { /*continue*/ }
                            Ordering::Greater => {
                                do_frame.pc.advance_to_next_line();
                            }
                        }
                    }
                    StackAssembally::EndLine(_) | StackAssembally::EndCommand(_) => {}
                    StackAssembally::DoArgLess(_) => {
                        let mut pc = do_frame.pc.clone();
                        pc.advance_to_next_line();
                        push_do_frame(
                            &mut self.symbol_table,
                            &mut self.call_stack,
                            pc,
                            PushFrameArgs {
                                arguments: [].into_iter(),
                                line_level_zero: false,
                            },
                        );
                    }
                    StackAssembally::ExtrinsicFunctionCall(call) => {
                        match self
                            .routine
                            .tag(call.tag)
                            .ok_or(RuntimeError::CouldNotFindTagOrCouldNotFindRoutine)
                            .map(|(pc, parameters)| {
                                (
                                    pc,
                                    build_arguments(
                                        call.number_of_arguments_passed,
                                        parameters,
                                        &mut self.stack,
                                    ),
                                )
                            })
                            .and_then(|(pc, arguments)| arguments.map(|x| (pc, x)))
                        {
                            Ok((pc, arguments)) => push_do_frame(
                                &mut self.symbol_table,
                                &mut self.call_stack,
                                pc,
                                PushFrameArgs {
                                    arguments,
                                    line_level_zero: true,
                                },
                            ),
                            Err(error) => self.error = Some(error),
                        }
                    }
                    StackAssembally::ForMetaData(meta_data) => {
                        Self::initialize_for_loop(
                            do_frame.pc.current_location(),
                            &mut do_frame.for_stack,
                            &mut self.stack,
                            meta_data,
                        );
                    }
                    StackAssembally::ForParameterType(r#type) => {
                        Self::initialize_for_range(
                            &mut do_frame.for_stack,
                            r#type,
                            &mut self.symbol_table,
                            &mut self.stack,
                            &mut do_frame.pc,
                        );
                    }
                    StackAssembally::ForEnd(_for_end) => {
                        Self::loop_condition_check_slash_increment(
                            &mut do_frame.for_stack,
                            &mut self.symbol_table,
                            &mut do_frame.pc,
                            &mut self.error,
                        );
                    }
                    StackAssembally::NoOpCode(_no_op_code) => {}
                    StackAssembally::LoadVar(load_var) => {
                        let var = Self::build_var(&mut self.stack, load_var.var);
                        let val = self.symbol_table.get(&var).cloned().unwrap_or_default();
                        self.stack.push_val(val);
                    }
                    StackAssembally::SetCodes(code) => match code {
                        SetCodes::Var => {
                            let var = self.stack.pop_var();
                            let val = self.stack.pop_val();
                            self.symbol_table.set(&var, &val).unwrap();
                        }
                    },
                    StackAssembally::TEMP { .. } => {}
                    StackAssembally::IfElse(command) => {
                        use IfElse as E;
                        if let E::If = command {
                            let condition = self.stack.pop_val();
                            do_frame.test = bool::from(condition);
                        }
                        if match command {
                            E::IfNoArgs | E::If => !do_frame.test,
                            E::Else => do_frame.test,
                        } {
                            Self::quit_line(
                                &mut do_frame.for_stack,
                                &mut self.symbol_table,
                                &mut do_frame.pc,
                                &mut self.error,
                            );
                        };
                    }
                    StackAssembally::KillInstruction(kill) => {
                        use ir::commands::kill::KillType as E;
                        let mut l_values = vec![];
                        for _ in 0..kill.number_of_variables {
                            l_values.push(self.stack.pop_var());
                        }
                        match kill.r#type {
                            E::Inclusive => {
                                for var in l_values {
                                    self.symbol_table.kill(&var);
                                }
                            }
                            E::Exclusive => {
                                let names: Vec<_> = l_values.into_iter().map(|x| x.name).collect();
                                self.symbol_table.keep(&names);
                            }
                        }
                    }
                    StackAssembally::PushVar(push_var) => {
                        let l_value = Self::build_var(&mut self.stack, push_var.var);
                        self.stack.push_var(l_value);
                    }
                    StackAssembally::QuitCodes(quit_codes) => match quit_codes {
                        QuitCodes::WithoutArg => {
                            if let Some(for_frame) = do_frame.for_stack.pop() {
                                do_frame.pc.jump(for_frame.r#break);
                            } else {
                                self.call_stack.pop();
                            }
                        }
                        QuitCodes::WithArg => {
                            let _ = self.stack.pop_val();
                            self.error = Some(RuntimeError::NotYetSupported("quit with args"))
                        }
                    },
                    StackAssembally::Jump(jump) => {
                        let should_jump = match jump.r#type {
                            JumpCodes::Conditional => !bool::from(self.stack.pop_val()),
                            JumpCodes::Unconditional => {
                                // Should just be "true", but leaving as unimplemented!() until I have a
                                // test case that uses this.
                                unimplemented!()
                            }
                        };
                        if should_jump {
                            do_frame.pc.jump_offset(jump.target)
                        }
                    }
                    StackAssembally::Test(_) => {
                        self.stack.push_val(do_frame.test.into());
                    }
                    StackAssembally::NewStackAsm(new) => {
                        let NewStackAsm {
                            number_of_variables,
                            kind,
                        } = new;

                        let vars = self
                            .stack
                            .0
                            .drain((self.stack.0.len() - number_of_variables as usize)..)
                            .map(|x| match x {
                                StackValue::Value(_) => {
                                    panic!("Tried to new something that was not a variable")
                                }
                                StackValue::Var(mvar) => mvar,
                            })
                            .map(|x| x.name);

                        match kind {
                            NewKind::Inclusive => self.symbol_table.new_var(vars).unwrap(),
                            NewKind::Exclusive => self.symbol_table.new_all_but(vars).unwrap(),
                        }
                    }
                }
            } else {
                self.call_stack.pop();
                self.symbol_table.pop_new_frame();
            }
        }
    }

    /// Exit the current line.
    /// This handles the case where we are in a for loop.
    pub(crate) fn quit_line(
        for_stack: &mut Vec<ForFrame>,
        symbol_table: &mut SymbolTable,
        pc: &mut ProgramCounter<'_>,
        error: &mut Option<RuntimeError>,
    ) {
        if !for_stack.is_empty() {
            Self::loop_condition_check_slash_increment(for_stack, symbol_table, pc, error);
        } else {
            pc.advance_to_next_line();
        }
    }

    pub fn build_var(r_values: &mut Stack, var: BuildVarInstructions) -> MVar<Path> {
        let mut subscripts = vec![];
        for _ in 0..var.subscripts {
            subscripts.push(r_values.pop_val());
        }
        MVar::new(var.name, Path::new(subscripts.iter()).unwrap())
    }
}

#[cfg(test)]
mod test {
    use std::{
        fs::{self},
        path::PathBuf,
    };

    use crate::{compile_routine, runtime::Job};
    use frontend::parse_routine_print_errors;
    use rstest::rstest;

    fn run_code_check_output(source: &str, output: &str, error: &str) {
        let routine = parse_routine_print_errors(source).unwrap();
        let byte_code = compile_routine(routine);

        let mut job = Job::new(&byte_code);
        job.run();
        assert_eq!(job.buffer, output);
        // All values must be used if they were added
        assert_eq!(job.stack.0, vec![]);
        // We should exit all the do stacks
        assert!(job.call_stack.is_empty());

        let error_message = job.error.map(|x| x.to_string()).unwrap_or(String::new());
        assert_eq!(error_message, error);
    }

    #[rstest]
    fn runtime_tests(#[files("tests/*/*.test")] file: PathBuf) {
        let content = fs::read_to_string(file).unwrap();
        let [src, output, error] = content
            // Remove trailing newline that is automatically added by my text editor.
            .strip_suffix("\n")
            .unwrap()
            // src vs expected output separator
            .split("\n---\n")
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        println!("Test Case:\nsrc:\n{}\nexpected:\n{}", src, output);
        run_code_check_output(src, output, error);
    }

    #[rstest]
    fn runtime_errors(#[files("tests/*/runtime_errors/*.test")] file: PathBuf) {
        let content = fs::read_to_string(file).unwrap();
        let [src, output, error] = content
            // Remove trailing newline that is automatically added by my text editor.
            .strip_suffix("\n")
            .unwrap()
            // src vs expected output separator
            .split("\n---\n")
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        println!("Test Case:\nsrc:\n{}", src,);
        run_code_check_output(src, output, error);
    }

    #[rstest]
    fn syntax_errors(#[files("tests/*/syntax_errors/*.test")] file: PathBuf) {
        let content = fs::read_to_string(file).unwrap();
        let [src, output] = content
            // Remove trailing newline that is automatically added by my text editor.
            .strip_suffix("\n")
            .unwrap()
            // src vs expected output separator
            .split("\n---\n")
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        println!("Test Case:\nsrc:\n{}", src,);
        let err = parse_routine_print_errors(src).unwrap_err();
        assert_eq!(
            err.iter().map(|x| x.to_string()).collect::<String>(),
            output
        );
    }
}
