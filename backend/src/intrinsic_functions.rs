use crate::{Compile, bite_code::BiteCode, expression::ExpressionContext, variable::VarContext};
use ir::{
    Expression, IntrinsicFunction,
    intrinsic_functions::{Function, SelectTerm, VarFunction},
};
use value::Value;

impl<const REQUIRED: usize, const OPTIONAL: usize> Compile for Function<REQUIRED, OPTIONAL> {
    type Context = u8;
    fn compile(&self, comp: &mut BiteCode, fn_code_base: &u8) {
        let required_args = self.required.iter();
        let optional_args = self.optional.iter().filter_map(|x| x.as_ref());
        for arg in required_args.chain(optional_args.clone()) {
            arg.compile(comp, &ExpressionContext::Eval);
        }
        comp.push(fn_code_base + optional_args.count() as u8);
    }
}

impl<const REQUIRED: usize, const OPTIONAL: usize> Compile for VarFunction<REQUIRED, OPTIONAL> {
    type Context = (VarContext, u8);
    fn compile(&self, comp: &mut BiteCode, (context, fn_code_base): &Self::Context) {
        //TODO handle other context types
        self.variable.compile(comp, &context);
        //TODO handle Next case.
        self.function.compile(comp, &fn_code_base);
    }
}

impl Compile for IntrinsicFunction {
    type Context = ();
    fn compile(&self, comp: &mut BiteCode, _: &()) {
        match self {
            IntrinsicFunction::Select { terms } => {
                let jump_indexs: Vec<_> = terms
                    .iter()
                    .map(|SelectTerm { condition, value }| {
                        comp.conditional_jump(condition, |bite_code| {
                            value.compile(bite_code, &ExpressionContext::Eval);
                            bite_code.unconditional_jump()
                        })
                    })
                    .collect();
                comp.push(ffi::OPERROR);
                let errm4 = (-(ffi::ERRM4 as i16)).to_le_bytes();
                comp.extend(errm4.into_iter());

                for jump_to_end in jump_indexs {
                    comp.write_jump(jump_to_end, comp.current_location());
                }
            }
            IntrinsicFunction::Char { args } => {
                if args.len() > 254 {
                    panic!("Char has too many args");
                } else {
                    for arg in args {
                        arg.compile(comp, &ExpressionContext::Eval);
                    }
                    comp.push(ffi::FUNC);
                    comp.push(args.len() as u8);
                }
            }
            IntrinsicFunction::View(function) => function.compile(comp, &ffi::FUNV2),
            IntrinsicFunction::Ascii(function) => function.compile(comp, &ffi::FUNA1),
            IntrinsicFunction::Extract(function) => function.compile(comp, &ffi::FUNE1),
            IntrinsicFunction::Find(function) => function.compile(comp, &ffi::FUNF2),
            IntrinsicFunction::Fnumber(function) => function.compile(comp, &ffi::FUNFN2),
            IntrinsicFunction::Justify(function) => function.compile(comp, &ffi::FUNJ2),
            IntrinsicFunction::Length(function) => function.compile(comp, &ffi::FUNL1),
            IntrinsicFunction::Piece(function) => function.compile(comp, &ffi::FUNP2),
            IntrinsicFunction::Random(function) => function.compile(comp, &ffi::FUNR),
            IntrinsicFunction::Reverse(function) => function.compile(comp, &ffi::FUNRE),
            IntrinsicFunction::Stack(function) => function.compile(comp, &ffi::FUNST1),
            IntrinsicFunction::Text(function) => function.compile(comp, &ffi::FUNT),
            IntrinsicFunction::Translate(function) => function.compile(comp, &ffi::FUNTR2),
            IntrinsicFunction::QLength(function) => {
                function.compile(comp, &(VarContext::Eval, ffi::FUNQL))
            }
            IntrinsicFunction::QSubscript(function) => {
                function.compile(comp, &(VarContext::Eval, ffi::FUNQS))
            }
            IntrinsicFunction::Data(function) => {
                function.compile(comp, &(VarContext::Build, ffi::FUND))
            }
            IntrinsicFunction::Get(function) => {
                function.compile(comp, &(VarContext::Build, ffi::FUNG1))
            }
            IntrinsicFunction::Increment(function) => {
                function.compile(comp, &(VarContext::Build, ffi::FUNI1))
            }
            IntrinsicFunction::Name(function) => {
                function.compile(comp, &(VarContext::BuildNullable, ffi::FUNNA1))
            }
            IntrinsicFunction::Order(function) => {
                function.compile(comp, &(VarContext::BuildNullable, ffi::FUNO1))
            }
            IntrinsicFunction::Query(function) => {
                function.compile(comp, &(VarContext::BuildNullable, ffi::FUNQ1))
            }
            IntrinsicFunction::Next(function) => {
                //Next is just an Order with a hard coded argument
                use std::str::FromStr;
                let two = Expression::Number(Value::from_str("2").unwrap().into());
                IntrinsicFunction::Order(VarFunction {
                    variable: function.variable.clone(),
                    function: Function {
                        required: [],
                        optional: [Some(two); 1],
                    },
                })
                .compile(comp, &());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use ir::Expression;

    use crate::{Compile, bite_code};

    use super::IntrinsicFunction;

    #[test]
    #[should_panic]
    fn max_number_of_args_pluse_1() {
        let exps = std::iter::repeat(Expression::String("0".parse().unwrap()));
        IntrinsicFunction::Char {
            args: exps.take(255).collect(),
        }
        .compile(&mut bite_code::BiteCode::new(), &());
    }
    #[test]
    fn max_number_of_args() {
        let exps = std::iter::repeat(Expression::String("0".parse().unwrap()));
        IntrinsicFunction::Char {
            args: exps.take(254).collect(),
        }
        .compile(&mut bite_code::BiteCode::new(), &());
    }
}
