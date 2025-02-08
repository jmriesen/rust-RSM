use super::expression::Expression;
use crate::{bite_code::BiteCode, expression::ExpressionContext, localvar::VarContext};
use value::Value;

#[derive(Clone)]
pub struct SelectTerm {
    condition: Expression,
    value: Expression,
}

#[derive(Clone)]
pub struct Function<const REQUIRED: usize, const OPTIONAL: usize> {
    required: [Expression; REQUIRED],
    //Note this should really be thought of as a Vec with a fixed capacity but... whatever, this
    //will work, just skip any None values.
    optional: [Option<Expression>; OPTIONAL],
}

impl<const REQUIRED: usize, const OPTIONAL: usize> Function<REQUIRED, OPTIONAL> {
    fn new(args: Vec<lang_model::Expression<'_>>, source_code: &str) -> Self {
        use std::array::from_fn;
        let num_of_args = REQUIRED..=REQUIRED + OPTIONAL;
        assert!(
            num_of_args.contains(&args.len()),
            "Exceded maximum arguments. Expected: {num_of_args:?}, Found:{}",
            args.len()
        );
        let mut iter = args.iter().map(|x| Expression::new(x, source_code));
        Function {
            required: from_fn(|_| iter.next().expect("required argument")),
            optional: from_fn(|_| iter.next()),
        }
    }
}

impl<const REQUIRED: usize, const OPTIONAL: usize> Function<REQUIRED, OPTIONAL> {
    fn compile(&self, comp: &mut BiteCode, fn_code_base: u8) {
        let required_args = self.required.iter();
        let optional_args = self.optional.iter().filter_map(|x| x.as_ref());
        for arg in required_args.chain(optional_args.clone()) {
            arg.compile(comp, ExpressionContext::Eval);
        }
        comp.push(fn_code_base + optional_args.count() as u8);
    }
}

#[derive(Clone)]
pub struct VarFunction<const REQUIRED: usize, const OPTIONAL: usize> {
    variable: super::Variable,
    function: Function<REQUIRED, OPTIONAL>,
}

impl<const REQUIRED: usize, const OPTIONAL: usize> VarFunction<REQUIRED, OPTIONAL> {
    fn new(
        var: lang_model::Variable<'_>,
        args: Vec<lang_model::Expression<'_>>,
        source_code: &str,
    ) -> Self {
        VarFunction {
            variable: super::Variable::new(&var, source_code),
            function: Function::new(args, source_code),
        }
    }
}

impl<const REQUIRED: usize, const OPTIONAL: usize> VarFunction<REQUIRED, OPTIONAL> {
    fn compile(&self, comp: &mut BiteCode, context: VarContext, fn_code_base: u8) {
        //TODO handle other context types
        self.variable.compile(comp, context);
        //TODO handle Next case.
        self.function.compile(comp, fn_code_base);
    }
}

#[derive(Clone)]
pub enum IntrinsicFunction {
    Select {
        terms: Vec<SelectTerm>,
    },
    ///Max number of arguments is 254
    Char {
        args: Vec<Expression>,
    },
    View(Function<2, 2>),
    Ascii(Function<1, 1>),
    Extract(Function<1, 2>),
    Find(Function<2, 1>),
    Fnumber(Function<2, 1>),
    Justify(Function<2, 1>),
    Length(Function<1, 1>),
    Piece(Function<2, 2>),
    Random(Function<1, 0>),
    Reverse(Function<1, 0>),
    Stack(Function<1, 1>),
    Text(Function<1, 0>),
    Translate(Function<2, 1>),

    QLength(VarFunction<0, 0>),
    QSubscript(VarFunction<1, 0>),

    Data(VarFunction<0, 0>),
    Get(VarFunction<0, 1>),
    Increment(VarFunction<0, 1>),

    Name(VarFunction<0, 3>),
    Order(VarFunction<0, 1>),
    Query(VarFunction<0, 1>),
    Next(VarFunction<0, 0>),
}

impl IntrinsicFunction {
    pub fn new(sitter: &lang_model::IntrinsicFunction<'_>, source_code: &str) -> Self {
        use lang_model::IntrinsicFunctionChildren::*;

        match &sitter.children() {
            ExpFunctions(exp_fun) => {
                use lang_model::ExpFunctionsChildren::*;
                match exp_fun.children() {
                    Char(x) => Self::Char {
                        args: x
                            .args()
                            .iter()
                            .map(|x| Expression::new(x, source_code))
                            .collect(),
                    },
                    View(x) => Self::View(Function::new(x.args(), source_code)),
                    Ascii(x) => Self::Ascii(Function::new(x.args(), source_code)),
                    Extract(x) => Self::Extract(Function::new(x.args(), source_code)),
                    Find(x) => Self::Find(Function::new(x.args(), source_code)),
                    Fnumber(x) => Self::Fnumber(Function::new(x.args(), source_code)),
                    Justify(x) => Self::Justify(Function::new(x.args(), source_code)),
                    Length(x) => Self::Length(Function::new(x.args(), source_code)),
                    Piece(x) => Self::Piece(Function::new(x.args(), source_code)),
                    Random(x) => Self::Random(Function::new(vec![x.args()], source_code)),
                    Reverse(x) => Self::Reverse(Function::new(vec![x.args()], source_code)),
                    Stack(x) => Self::Stack(Function::new(x.args(), source_code)),
                    Text(x) => Self::Text(Function::new(vec![x.args()], source_code)),
                    Translate(x) => Self::Translate(Function::new(x.args(), source_code)),
                }
            }
            VarFunctions(var_fun) => {
                use lang_model::VarFunctionsChildren::*;
                match var_fun.children() {
                    Qlength(x) => Self::QLength(VarFunction::new(x.var(), vec![], source_code)),
                    Qsubscript(x) => {
                        Self::QSubscript(VarFunction::new(x.var(), vec![x.args()], source_code))
                    }
                    Data(x) => Self::Data(VarFunction::new(x.var(), vec![], source_code)),
                    Get(x) => Self::Get(VarFunction::new(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Increment(x) => Self::Increment(VarFunction::new(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Name(x) => Self::Name(VarFunction::new(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Order(x) => Self::Order(VarFunction::new(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Query(x) => Self::Query(VarFunction::new(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Next(x) => Self::Next(VarFunction::new(
                        x.var(),
                        [].into_iter().collect(),
                        source_code,
                    )),
                }
            }
            Select(select) => Self::Select {
                terms: select
                    .children()
                    .array_chunks::<2>()
                    .map(|[condition, value]| SelectTerm {
                        condition: Expression::new(condition, source_code),
                        value: Expression::new(value, source_code),
                    })
                    .collect(),
            },
        }
    }

    pub fn compile(&self, comp: &mut BiteCode) {
        match self {
            IntrinsicFunction::Select { terms } => {
                let jump_indexs: Vec<_> = terms
                    .iter()
                    .map(|SelectTerm { condition, value }| {
                        condition.compile(comp, ExpressionContext::Eval);
                        comp.push(ffi::JMP0);
                        let try_next = comp.reserve_jump();

                        value.compile(comp, ExpressionContext::Eval);
                        comp.push(ffi::JMP);
                        let jump_to_end = comp.reserve_jump();
                        comp.write_jump(try_next, comp.current_location());
                        jump_to_end
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
                        arg.compile(comp, ExpressionContext::Eval);
                    }
                    comp.push(ffi::FUNC);
                    comp.push(args.len() as u8);
                }
            }
            IntrinsicFunction::View(function) => function.compile(comp, ffi::FUNV2),
            IntrinsicFunction::Ascii(function) => function.compile(comp, ffi::FUNA1),
            IntrinsicFunction::Extract(function) => function.compile(comp, ffi::FUNE1),
            IntrinsicFunction::Find(function) => function.compile(comp, ffi::FUNF2),
            IntrinsicFunction::Fnumber(function) => function.compile(comp, ffi::FUNFN2),
            IntrinsicFunction::Justify(function) => function.compile(comp, ffi::FUNJ2),
            IntrinsicFunction::Length(function) => function.compile(comp, ffi::FUNL1),
            IntrinsicFunction::Piece(function) => function.compile(comp, ffi::FUNP2),
            IntrinsicFunction::Random(function) => function.compile(comp, ffi::FUNR),
            IntrinsicFunction::Reverse(function) => function.compile(comp, ffi::FUNRE),
            IntrinsicFunction::Stack(function) => function.compile(comp, ffi::FUNST1),
            IntrinsicFunction::Text(function) => function.compile(comp, ffi::FUNT),
            IntrinsicFunction::Translate(function) => function.compile(comp, ffi::FUNTR2),
            IntrinsicFunction::QLength(function) => {
                function.compile(comp, VarContext::Eval, ffi::FUNQL)
            }
            IntrinsicFunction::QSubscript(function) => {
                function.compile(comp, VarContext::Eval, ffi::FUNQS)
            }
            IntrinsicFunction::Data(function) => {
                function.compile(comp, VarContext::Build, ffi::FUND)
            }
            IntrinsicFunction::Get(function) => {
                function.compile(comp, VarContext::Build, ffi::FUNG1)
            }
            IntrinsicFunction::Increment(function) => {
                function.compile(comp, VarContext::Build, ffi::FUNI1)
            }
            IntrinsicFunction::Name(function) => {
                function.compile(comp, VarContext::BuildNullable, ffi::FUNNA1)
            }
            IntrinsicFunction::Order(function) => {
                function.compile(comp, VarContext::BuildNullable, ffi::FUNO1)
            }
            IntrinsicFunction::Query(function) => {
                function.compile(comp, VarContext::BuildNullable, ffi::FUNQ1)
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
                .compile(comp);
            }
        }
    }
}
