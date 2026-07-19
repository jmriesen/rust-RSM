pub use bite_code::BiteCode;

use crate::runtime::EndLine;

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
pub fn compile_routine(routine: ir::Routine) -> Vec<u8> {
    let mut comp = BiteCode::new();
    for line in routine {
        line.compile(&mut comp, &());
        comp.push(EndLine.encode());
    }
    comp.get_raw()
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub mod test {
    use ir::commands::Command;

    use crate::compile_routine;
    pub fn test_compile_command(source_code: &str) -> Vec<u8> {
        let commands = frontend::parse_routine(&format!("tag {source_code}\n")).unwrap();
        compile_routine(commands)
    }

    pub fn parse_routine(source_code: &str) -> Vec<Vec<Command>> {
        frontend::parse_routine(source_code).unwrap()
    }
}
