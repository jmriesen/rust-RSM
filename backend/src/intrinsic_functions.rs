use crate::{Compile, bite_code::BiteCode, expression::ExpressionContext, variable::VarContext};
use ir::{
    Expression, IntrinsicFunction,
    intrinsic_functions::{Function, SelectTerm, VarFunction},
};
use value::Value;

#[derive(Clone, Copy)]
pub enum FunctionCodes {
    Char = 102,
    View = 135,
    Ascii = 100,
    Extract = 104,
    Find = 107,
    Fnumber = 109,
    Justify = 113,
    Length = 115,
    Piece = 121,
    Random = 128,
    Reverse = 129,
    Stack = 130,
    Text = 132,
    Translate = 133,
    //Var functions
    QLength = 124,
    QSubscript = 125,
    Data = 103,
    Get = 111,
    Increment = 98,
    Name = 117,
    Order = 119,
    Query = 126,
}

impl<const REQUIRED: usize, const OPTIONAL: usize> Compile for Function<REQUIRED, OPTIONAL> {
    type Context = FunctionCodes;
    fn compile(&self, comp: &mut BiteCode, fn_code_base: &FunctionCodes) {
        let required_args = self.required.iter();
        let optional_args = self.optional.iter().filter_map(|x| x.as_ref());
        for arg in required_args.chain(optional_args.clone()) {
            arg.compile(comp, &ExpressionContext::Eval);
        }
        comp.push(*fn_code_base as u8 + optional_args.count() as u8);
    }
}

impl<const REQUIRED: usize, const OPTIONAL: usize> Compile for VarFunction<REQUIRED, OPTIONAL> {
    type Context = (VarContext, FunctionCodes);
    fn compile(&self, comp: &mut BiteCode, (context, fn_code_base): &Self::Context) {
        self.variable.compile(comp, &context);
        self.function.compile(comp, &fn_code_base);
    }
}
const ERROR_CODE_OP: u8 = 2;
const NO_TRUE_CONDITION_IN_SELECT: i16 = 4 as i16;

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
                comp.push(ERROR_CODE_OP);
                let errm4 = (-(NO_TRUE_CONDITION_IN_SELECT)).to_le_bytes();
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
                    comp.push(FunctionCodes::Char as u8);
                    comp.push(args.len() as u8);
                }
            }
            IntrinsicFunction::View(function) => function.compile(comp, &(FunctionCodes::View)),
            IntrinsicFunction::Ascii(function) => function.compile(comp, &(FunctionCodes::Ascii)),
            IntrinsicFunction::Extract(function) => {
                function.compile(comp, &(FunctionCodes::Extract))
            }
            IntrinsicFunction::Find(function) => function.compile(comp, &(FunctionCodes::Find)),
            IntrinsicFunction::Fnumber(function) => {
                function.compile(comp, &(FunctionCodes::Fnumber))
            }
            IntrinsicFunction::Justify(function) => {
                function.compile(comp, &(FunctionCodes::Justify))
            }
            IntrinsicFunction::Length(function) => function.compile(comp, &(FunctionCodes::Length)),
            IntrinsicFunction::Piece(function) => function.compile(comp, &(FunctionCodes::Piece)),
            IntrinsicFunction::Random(function) => function.compile(comp, &(FunctionCodes::Random)),
            IntrinsicFunction::Reverse(function) => {
                function.compile(comp, &(FunctionCodes::Reverse))
            }
            IntrinsicFunction::Stack(function) => function.compile(comp, &(FunctionCodes::Stack)),
            IntrinsicFunction::Text(function) => function.compile(comp, &(FunctionCodes::Text)),
            IntrinsicFunction::Translate(function) => {
                function.compile(comp, &(FunctionCodes::Translate))
            }
            IntrinsicFunction::QLength(function) => {
                function.compile(comp, &(VarContext::Eval, FunctionCodes::QLength))
            }
            IntrinsicFunction::QSubscript(function) => {
                function.compile(comp, &(VarContext::Eval, FunctionCodes::QSubscript))
            }
            IntrinsicFunction::Data(function) => {
                function.compile(comp, &(VarContext::Build, FunctionCodes::Data))
            }
            IntrinsicFunction::Get(function) => {
                function.compile(comp, &(VarContext::Build, FunctionCodes::Get))
            }
            IntrinsicFunction::Increment(function) => {
                function.compile(comp, &(VarContext::Build, FunctionCodes::Increment))
            }
            IntrinsicFunction::Name(function) => {
                function.compile(comp, &(VarContext::BuildNullable, FunctionCodes::Name))
            }
            IntrinsicFunction::Order(function) => {
                function.compile(comp, &(VarContext::BuildNullable, FunctionCodes::Order))
            }
            IntrinsicFunction::Query(function) => {
                function.compile(comp, &(VarContext::BuildNullable, FunctionCodes::Query))
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
