pub use bite_code::BiteCode;
use ir::Line;

use crate::runtime::{EndLine, StartLine};

pub mod bite_code;
pub mod commands;
pub mod expression;
pub mod external_calls;
pub mod extrinsic_function;
pub mod intrinsic_functions;
pub mod intrinsic_var;
pub mod operators;
pub mod runtime;
pub mod value;
pub mod var_u;
pub mod variable;

pub trait Compile {
    type Context;
    fn compile(&self, bite_code: &mut BiteCode, context: &Self::Context);
}

impl<T, C> Compile for Vec<T>
where
    T: Compile,
    T: Compile<Context = C>,
{
    type Context = C;
    fn compile(&self, bite_code: &mut BiteCode, context: &Self::Context) {
        for term in self {
            term.compile(bite_code, context)
        }
    }
}

impl<T, C> Compile for Option<T>
where
    T: Compile,
    T: Compile<Context = C>,
{
    type Context = C;
    fn compile(&self, bite_code: &mut BiteCode, context: &Self::Context) {
        if let Some(inner) = self {
            inner.compile(bite_code, context)
        }
    }
}

impl Compile for Line {
    type Context = u16;

    fn compile(&self, bite_code: &mut BiteCode, line_numb: &Self::Context) {
        StartLine {
            line_numb: *line_numb,
            level: self.level,
        }
        .compile(bite_code, &());
        //NOTE: I have decided to change the bite code layout compared to the original C code.
        //In the ordinal C the line level/check is part of the stack machine instructions.
        //In this version it will be unconditional included as part of the line encoding.
        self.commands.compile(bite_code, &());
        bite_code.push(EndLine.encode());
    }
}
pub fn compile_routine(routine: ir::Routine) -> Vec<u8> {
    let mut comp = BiteCode::new();
    for (i, line) in routine.iter().enumerate() {
        line.compile(&mut comp, &(i as u16));
    }
    comp.get_raw()
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub mod test {
    use ir::Routine;

    use crate::compile_routine;
    pub fn test_compile_command(source_code: &str) -> Vec<u8> {
        let commands = frontend::parse_routine(&format!("tag {source_code}\n")).unwrap();
        const LENGTH_OF_LINE_START: usize = 5;
        compile_routine(commands)[LENGTH_OF_LINE_START..].to_vec()
    }

    pub fn parse_routine(source_code: &str) -> Routine {
        frontend::parse_routine(source_code).unwrap()
    }
}
