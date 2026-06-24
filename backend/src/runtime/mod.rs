use crate::{
    commands::{
        r#for::{ForEnd, ForSet, ForStart},
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
use value::Value;
mod r#for;
mod macros;
mod operators;
pub mod program_counter;

pub struct Job<'a> {
    //Replace with a proper output device later.
    buffer: String,
    /// Stack of values
    values: Vec<value::Value>,
    /// Stack of L-values (things that can be assigned to).
    l_values: Vec<MVar<Path>>,
    //Temporarily store loop metadata
    //This is needed since for loops are encoded as
    //Metadata expression expression expression loop body
    //so we need a place to put the metadata while evaluating the expressions
    for_preample: Option<ForSet>,
    // Metadata for all for loops.
    for_stack: Vec<ForFrame>,
    symbole_table: SymbolTable,

    /// Stores the last result of the most resent if predicate.
    /// Used by else.
    test: bool,
    pc: ProgramCounter<'a>,
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
OpCode! {EndCommand=4}
OpCode! {NoOpCode=179}

#[derive(Debug)]
pub struct JumpIfFalse {
    target: program_counter::Location,
}
impl Decode for JumpIfFalse {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        if [5] == decoder.consume_n() {
            Some(Self {
                target: Decode::decode(decoder)?,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct TEMP(u8);
#[cfg_attr(test, mutants::skip)]
impl Decode for TEMP {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        let [code] = decoder.consume_n();
        //Always accept remove before production but helps during testing adding new types
        Some(Self(code))
    }
}

pub(crate) trait StackAssemballyTrait: Decode {}
StackAssembally! {
    LoadVar,
    SetCodes,
    Value,
    WriteCodes,
    Binary,
    Unary,
    EndLine,
    EndCommand,
    ForSet,
    ForStart,
    ForEnd,
    NoOpCode,
    IfOp,
    ElseOp,
    KillInstruction,
    PushVar,
    QuitCodes,
    JumpIfFalse,
    TEMP,
}
/// Marks something as a whole assembly instruction

impl<'a> Job<'a> {
    pub fn new(byte_code: &'a [u8]) -> Self {
        Self {
            buffer: String::new(),
            values: vec![],
            l_values: vec![],
            for_preample: None,
            for_stack: vec![],
            symbole_table: SymbolTable::default(),
            test: false,
            pc: ProgramCounter::new(byte_code),
        }
    }
    pub fn run(&mut self) {
        while !self.pc.end() {
            match self.pc.next() {
                StackAssembally::Value(value) => {
                    self.values.push(value);
                }
                StackAssembally::WriteCodes(write_codes) => match write_codes {
                    WriteCodes::Bang => self.buffer.push('\n'),
                    WriteCodes::Clear => todo!(),
                    WriteCodes::Tab => todo!(),
                    WriteCodes::Expression => {
                        let value = self.values.pop().unwrap();
                        self.buffer
                            .push_str(core::str::from_utf8(value.content()).unwrap());
                    }
                },
                StackAssembally::Binary(op) => {
                    let second = self.values.pop().unwrap();
                    let first = self.values.pop().unwrap();
                    self.values.push(op.apply(first, second));
                }
                StackAssembally::Unary(op) => {
                    let value = self.values.pop().unwrap();
                    self.values.push(op.apply(value));
                }
                StackAssembally::EndLine(_) | StackAssembally::EndCommand(_) => {}
                StackAssembally::ForSet(for_set) => self.for_preample = Some(for_set),
                StackAssembally::ForStart(for_start) => {
                    Self::init_for_loop(
                        &mut self.for_stack,
                        &mut self.values,
                        &mut self.for_preample,
                        &mut self.symbole_table,
                        for_start,
                    );
                }
                StackAssembally::ForEnd(_for_end) => {
                    Self::handel_for_preamble(
                        &mut self.for_stack,
                        &mut self.symbole_table,
                        &mut self.pc,
                    );
                }
                StackAssembally::NoOpCode(_no_op_code) => {}
                StackAssembally::LoadVar(load_var) => {
                    let var = Self::build_var(&mut self.values, load_var.var);
                    let val = self.symbole_table.get(&var).cloned().unwrap_or_default();
                    self.values.push(val);
                }
                StackAssembally::SetCodes(code) => match code {
                    SetCodes::Var => {
                        let val = self.values.pop().expect("Value to store on the stack");
                        let var = self.l_values.pop().unwrap();
                        self.symbole_table.set(&var, &val).unwrap();
                    }
                },
                StackAssembally::TEMP { .. } => {}
                StackAssembally::IfOp(_) => {
                    let condition = self.values.pop().expect("Value to store on the stack");
                    self.test = bool::from(condition);
                    if !self.test {
                        self.pc.advance_to_next_line();
                    }
                }
                StackAssembally::ElseOp(_) => {
                    if self.test {
                        self.pc.advance_to_next_line();
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
                                self.symbole_table.kill(&var);
                            }
                        }
                        E::Exclusive => {
                            let names: Vec<_> = l_values.into_iter().map(|x| x.name).collect();
                            self.symbole_table.keep(&names);
                        }
                    }
                }
                StackAssembally::PushVar(push_var) => {
                    let l_value = Self::build_var(&mut self.values, push_var.var);
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
                    QuitCodes::WithArg => todo!(),
                },
                StackAssembally::JumpIfFalse(jump) => {
                    let condition = self.values.pop().expect("Value to store on the stack");
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

    use crate::{runtime::Job, test::compile_routine};
    use frontend::wrap_command_in_routine;
    use rstest::rstest;

    fn run_code_check_output(source: &str, output: &str) {
        let routine = wrap_command_in_routine(source);
        let byte_code = compile_routine(routine);

        let mut job = Job::new(&byte_code);
        job.run();
        assert_eq!(job.buffer, output);
        // All values must be used if they were added
        assert_eq!(job.values, vec![]);
        // We should exit all the for lops
        assert_eq!(job.for_stack, vec![]);
    }

    #[rstest]
    fn runtime_tests(#[files("tests/*/*.test")] file: PathBuf) {
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
        println!("Test Case:\nsrc:\n{}\nexpected:\n{}", src, output);
        run_code_check_output(src, output);
    }

    #[rstest]
    #[should_panic]
    fn errors(#[files("tests/*/errors/*.test")] file: PathBuf) {
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
        run_code_check_output(src, output);
    }
}
