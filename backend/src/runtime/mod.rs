use crate::{
    Compile,
    commands::{
        r#do::DoArgLess,
        r#for::{ForEnd, ForMetaData, ForRangeType},
        r#if::{ElseOp, IfOp},
        kill::KillInstruction,
        quit::QuitCodes,
        set::SetCodes,
        write::WriteCodes,
    },
    runtime::{
        r#for::ForFrame,
        macros::StackAssembally,
        operators::{BinaryApply, UnaryApply},
        program_counter::{AssemballyDecoder, ProgramCounter},
    },
    variable::{BuildVarInstructions, LoadVar, PushVar},
};
use ir::operators::{Binary, Unary};
use std::fmt::Debug;
use symbol_table::{MVar, SymbolTable, key::Path};
use thiserror::Error;
use value::Value;
mod r#for;
mod if_else;
mod macros;
mod operators;
pub mod program_counter;
#[derive(Error, PartialEq, Debug)]
pub(crate) enum RuntimeError {
    #[error("Undefined Index variable")]
    UndefinedIndexVariable,
    #[error("Not yet supported {}",.0)]
    NotYetSupported(&'static str),
}

pub struct Job<'a> {
    //Replace with a proper output device later.
    buffer: String,
    /// Stack of values
    r_values: Vec<value::Value>,
    /// Stack of L-values (things that can be assigned to).
    l_values: Vec<MVar<Path>>,
    // Metadata for all for loops.
    for_stack: Vec<ForFrame>,
    symbol_table: SymbolTable,

    /// Stores the last result of the most resent if predicate.
    /// Used by else.
    test: bool,
    pc: ProgramCounter<'a>,

    error: Option<RuntimeError>,
}
// Partial (or whole) assembly instruction.
pub trait Decode: Sized {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self>;
}
pub trait Encode: Sized {
    fn encode(&self) -> u8;
}

pub(crate) use macros::{OpCode, OpCodes, OpCodesForeign};
OpCode! {EndLine=0}
OpCode! {LineNum=170}
OpCode! {EndCommand=4}
OpCode! {NoOpCode=179}
OpCode! {JumpIfFalseCode=5}

#[derive(Debug)]
pub struct JumpIfFalse {
    target: program_counter::Location,
}
impl Decode for JumpIfFalse {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        JumpIfFalseCode::decode(decoder)?;
        Some(Self {
            target: Decode::decode(decoder)?,
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TEMP(u8);
#[cfg_attr(test, mutants::skip)]
impl Decode for TEMP {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        let [code] = decoder.consume_n();
        //Always accept remove before production but helps during testing adding new types
        Some(Self(code))
    }
}

#[derive(Debug)]
pub struct StartLine {
    pub line_numb: u16,
    pub level: u16,
}

impl Decode for StartLine {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        LineNum::decode(decoder)?;
        Some(StartLine {
            line_numb: u16::from_le_bytes(decoder.consume_n()),
            level: u16::from_le_bytes(decoder.consume_n()),
        })
    }
}

impl Compile for StartLine {
    type Context = ();

    fn compile(&self, bite_code: &mut crate::BiteCode, _context: &Self::Context) {
        bite_code.push(LineNum.encode());
        bite_code.extend(self.line_numb.to_le_bytes());
        bite_code.extend(self.level.to_le_bytes());
    }
}

pub(crate) trait StackAssemblyTrait: Decode {}
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
    ForRangeType,
    ForEnd,
    NoOpCode,
    IfOp,
    ElseOp,
    KillInstruction,
    PushVar,
    QuitCodes,
    JumpIfFalse,
    DoArgLess,
    TEMP,
}
/// Marks something as a whole assembly instruction

impl<'a> Job<'a> {
    pub fn new(byte_code: &'a [u8]) -> Self {
        Self {
            buffer: String::new(),
            r_values: vec![],
            l_values: vec![],
            for_stack: vec![],
            symbol_table: SymbolTable::default(),
            test: false,
            pc: ProgramCounter::new(byte_code),
            error: None,
        }
    }
    pub fn run(&mut self) {
        while !self.pc.end() {
            match self.pc.next() {
                StackAssembally::Value(value) => {
                    self.r_values.push(value);
                }
                StackAssembally::WriteCodes(write_codes) => match write_codes {
                    WriteCodes::Bang => self.buffer.push('\n'),
                    WriteCodes::Clear => todo!(),
                    WriteCodes::Tab => todo!(),
                    WriteCodes::Expression => {
                        let value = self.r_values.pop().unwrap();
                        self.buffer
                            .push_str(core::str::from_utf8(value.content()).unwrap());
                    }
                },
                StackAssembally::Binary(op) => {
                    let second = self.r_values.pop().unwrap();
                    let first = self.r_values.pop().unwrap();
                    self.r_values.push(op.apply(first, second));
                }
                StackAssembally::Unary(op) => {
                    let value = self.r_values.pop().unwrap();
                    self.r_values.push(op.apply(value));
                }
                StackAssembally::StartLine(line_info) => {
                    if line_info.level != 0 {
                        self.pc.advance_to_next_line();
                    }
                }
                StackAssembally::EndLine(_) | StackAssembally::EndCommand(_) => {}
                StackAssembally::DoArgLess(_) => {
                    // Push value on the due stack.
                    // Increment the line level.
                    // Reset program counter.
                }
                StackAssembally::ForMetaData(meta_data) => {
                    Self::initialize_for_loop(&mut self.for_stack, &mut self.r_values, meta_data);
                }
                StackAssembally::ForRangeType(r#type) => {
                    Self::initialize_for_range(
                        &mut self.for_stack.last_mut().as_mut().unwrap(),
                        r#type,
                        &mut self.symbol_table,
                        &mut self.r_values,
                        &mut self.pc,
                    );
                }
                StackAssembally::ForEnd(_for_end) => {
                    Self::loop_condition_check_slash_increment(
                        &mut self.for_stack,
                        &mut self.symbol_table,
                        &mut self.pc,
                        &mut self.error,
                    );
                }
                StackAssembally::NoOpCode(_no_op_code) => {}
                StackAssembally::LoadVar(load_var) => {
                    let var = Self::build_var(&mut self.r_values, load_var.var);
                    let val = self.symbol_table.get(&var).cloned().unwrap_or_default();
                    self.r_values.push(val);
                }
                StackAssembally::SetCodes(code) => match code {
                    SetCodes::Var => {
                        let val = self.r_values.pop().expect("Value to store on the stack");
                        let var = self.l_values.pop().unwrap();
                        self.symbol_table.set(&var, &val).unwrap();
                    }
                },
                StackAssembally::TEMP { .. } => {}
                StackAssembally::IfOp(_) => {
                    let condition = self.r_values.pop().expect("Value to store on the stack");
                    self.test = bool::from(condition);
                    if !self.test {
                        Self::if_jump(
                            &mut self.for_stack,
                            &mut self.symbol_table,
                            &mut self.pc,
                            &mut self.error,
                        );
                    }
                }
                StackAssembally::ElseOp(_) => {
                    if self.test {
                        Self::if_jump(
                            &mut self.for_stack,
                            &mut self.symbol_table,
                            &mut self.pc,
                            &mut self.error,
                        );
                    }
                }
                StackAssembally::KillInstruction(kill) => {
                    use ir::commands::kill::KillType as E;
                    let mut l_values = vec![];
                    for _ in 0..kill.number_of_variables {
                        l_values.push(self.l_values.pop().unwrap());
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
                    let l_value = Self::build_var(&mut self.r_values, push_var.var);
                    self.l_values.push(l_value);
                }
                StackAssembally::QuitCodes(quit_codes) => match quit_codes {
                    QuitCodes::WithoutArg => {
                        let for_stack = self
                            .for_stack
                            .pop()
                            .expect("Quits are currnly only supported in for loops");
                        self.pc.jump(for_stack.r#break);
                    }
                    QuitCodes::WithArg => {
                        let _ = self.r_values.pop().unwrap();
                        self.error = Some(RuntimeError::NotYetSupported("quit with args"))
                    }
                },
                StackAssembally::JumpIfFalse(jump) => {
                    let condition = self.r_values.pop().expect("Value to store on the stack");
                    if !bool::from(condition) {
                        self.pc.jump(jump.target)
                    }
                }
            }
        }
    }

    fn build_var(r_values: &mut Vec<Value>, var: BuildVarInstructions) -> MVar<Path> {
        let mut subscripts = vec![];
        for _ in 0..var.subscripts {
            subscripts.push(r_values.pop().unwrap());
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
    use frontend::parse_routine;
    use rstest::rstest;

    fn run_code_check_output(source: &str, output: &str, error: &str) {
        let routine = parse_routine(source).unwrap();
        let byte_code = compile_routine(routine);

        let mut job = Job::new(&byte_code);
        job.run();
        assert_eq!(job.buffer, output);
        // All values must be used if they were added
        assert_eq!(job.r_values, vec![]);
        // We should exit all the for lops
        assert_eq!(job.for_stack, vec![]);

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
        let err = parse_routine(src).unwrap_err();
        assert_eq!(err.to_string(), output);
    }
}
