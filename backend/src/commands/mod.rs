use ir::commands::{Command, PostCondition};

use crate::{Compile, bite_code::BiteCode};

pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod write;

impl<T, C> Compile for PostCondition<T>
where
    T: Compile,
    T: Compile<Context = C>,
{
    type Context = C;
    fn compile(&self, bite_code: &mut BiteCode, context: &Self::Context) {
        if let Some(condition) = &self.condition {
            bite_code.conditional_jump(condition, |x| self.value.compile(x, context))
        } else {
            self.value.compile(bite_code, context);
        }
    }
}

impl Compile for Command {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match self {
            Command::Write(x) => x.compile(bite_code, &()),
            Command::Close(x) => x.compile(bite_code, &()),
            Command::Do(x) => x.compile(bite_code, &()),
            Command::Break(x) => x.compile(bite_code, &()),
            Command::Else => bite_code.push(ffi::OPELSE),
            Command::For(x) => x.compile(bite_code, &()),
        }
        bite_code.push(ffi::OPENDC);
    }
}
