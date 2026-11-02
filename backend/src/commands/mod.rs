use ir::commands::{Command, PostCondition};

use crate::{
    Compile,
    byte_code::ByteCode,
    commands::r#if::IfElse,
    runtime::{EndCommand, StackAssembally},
};

pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod r#if;
pub mod kill;
pub mod new;
pub mod quit;
pub mod set;
pub mod write;

impl<T, C> Compile for PostCondition<T>
where
    T: Compile,
    T: Compile<Context = C>,
{
    type Context = C;
    fn compile(&self, bite_code: &mut ByteCode, context: &Self::Context) {
        if let Some(condition) = &self.condition {
            bite_code.conditional_jump(condition, |x| self.value.compile(x, context))
        } else {
            self.value.compile(bite_code, context);
        }
    }
}

impl Compile for Command {
    type Context = ();
    fn compile(&self, byte_code: &mut ByteCode, _: &()) {
        match self {
            Command::Write(x) => x.compile(byte_code, &()),
            Command::Close(x) => x.compile(byte_code, &()),
            Command::Do(x) => x.compile(byte_code, &()),
            Command::Break(x) => x.compile(byte_code, &()),
            Command::Else => byte_code.push_asm(StackAssembally::IfElse(IfElse::Else)),
            Command::For(x) => x.compile(byte_code, &()),
            Command::Set(x) => x.compile(byte_code, &()),
            Command::If(x) => x.compile(byte_code, &()),
            Command::Kill(x) => x.compile(byte_code, &()),
            Command::Quit(x) => x.compile(byte_code, &()),
            Command::New(x) => x.compile(byte_code, &()),
            Command::Error => todo!("refactor so I can claim this in a type safe way"),
        }
        byte_code.push_asm(StackAssembally::EndCommand(EndCommand))
    }
}
