use lang_model::Qlength;
use value::Value;

use crate::{
    expression::ExpressionContext,
    function::{reserve_jump, write_jump},
    localvar::VarContext,
};

use super::expression::Expression;

pub struct SelectTerm<'a> {
    condition: Expression<'a>,
    value: Expression<'a>,
}

pub struct Function<'a, const REQUIRED: usize, const OPTIONAL: usize> {
    required: [Expression<'a>; REQUIRED],
    //Note this should really be thought of as a Vec with a fixed capacity but... whatever, this
    //will work, just skip any None values.
    optional: [Option<Expression<'a>>; OPTIONAL],
}

impl<'a, const REQUIRED: usize, const OPTIONAL: usize> Function<'a, REQUIRED, OPTIONAL> {
    fn new(args: Vec<lang_model::Expression<'a>>, source_code: &str) -> Self {
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

impl<'a, const REQUIRED: usize, const OPTIONAL: usize> Function<'a, REQUIRED, OPTIONAL> {
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, fn_code_base: u8) {
        let required_args = self.required.iter();
        let optional_args = self.optional.iter().filter_map(|x| x.as_ref());
        for arg in required_args.chain(optional_args.clone()) {
            super::expression::compile(arg, source_code, comp, ExpressionContext::Eval);
        }
        comp.push(fn_code_base + optional_args.count() as u8);
    }
}

pub struct VarFunction<'a, const REQUIRED: usize, const OPTIONAL: usize> {
    variable: super::variable::Variable<'a>,
    function: Function<'a, REQUIRED, OPTIONAL>,
}

impl<'a, const REQUIRED: usize, const OPTIONAL: usize> VarFunction<'a, REQUIRED, OPTIONAL> {
    fn new(
        var: lang_model::Variable<'a>,
        args: Vec<lang_model::Expression<'a>>,
        source_code: &str,
    ) -> Self {
        VarFunction {
            variable: super::variable::Variable::new(&var, &source_code),
            function: Function::new(args, source_code),
        }
    }
}

impl<'a, const REQUIRED: usize, const OPTIONAL: usize> VarFunction<'a, REQUIRED, OPTIONAL> {
    fn compile(
        &self,
        source_code: &str,
        comp: &mut Vec<u8>,
        context: VarContext,
        fn_code_base: u8,
    ) {
        //TODO handle other context types
        super::variable::compile(&self.variable, source_code, comp, context);
        //TODO handle Next case.
        self.function.compile(source_code, comp, fn_code_base);
    }
}

pub enum IntrinsicFunction<'a> {
    Select {
        terms: Vec<SelectTerm<'a>>,
    },
    ///Max number of arguments is 254
    Char {
        args: Vec<Expression<'a>>,
    },
    View(Function<'a, 2, 2>),
    Ascii(Function<'a, 1, 1>),
    Extract(Function<'a, 1, 2>),
    Find(Function<'a, 2, 1>),
    Fnumber(Function<'a, 2, 1>),
    Justify(Function<'a, 2, 1>),
    Length(Function<'a, 1, 1>),
    Piece(Function<'a, 2, 2>),
    Random(Function<'a, 1, 0>),
    Reverse(Function<'a, 1, 0>),
    Stack(Function<'a, 1, 1>),
    Text(Function<'a, 1, 0>),
    Translate(Function<'a, 2, 1>),

    QLength(VarFunction<'a, 0, 0>),
    QSubscript(VarFunction<'a, 1, 0>),

    Data(VarFunction<'a, 0, 0>),
    Get(VarFunction<'a, 0, 1>),
    Increment(VarFunction<'a, 0, 1>),

    Name(VarFunction<'a, 0, 3>),
    Order(VarFunction<'a, 0, 1>),
    Query(VarFunction<'a, 0, 1>),
    Next(VarFunction<'a, 0, 0>),
}

impl<'a> IntrinsicFunction<'a> {
    pub fn new(sitter: &lang_model::IntrinsicFunction<'a>, source_code: &str) -> Self {
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
}

pub fn compile(function: &IntrinsicFunction, source_code: &str, comp: &mut Vec<u8>) {
    match function {
        IntrinsicFunction::Select { terms } => {
            let jump_indexs: Vec<_> = terms
                .iter()
                .map(|SelectTerm { condition, value }| {
                    super::expression::compile(
                        &condition,
                        source_code,
                        comp,
                        ExpressionContext::Eval,
                    );
                    comp.push(ffi::JMP0);
                    let try_next = reserve_jump(comp);

                    super::expression::compile(&value, source_code, comp, ExpressionContext::Eval);
                    comp.push(ffi::JMP);
                    let exit = reserve_jump(comp);

                    (try_next, exit)
                })
                .collect();
            comp.push(ffi::OPERROR);
            let errm4 = (-(ffi::ERRM4 as i16)).to_le_bytes();
            comp.extend_from_slice(&errm4);

            for (try_next, exit) in jump_indexs {
                write_jump(try_next, exit, comp);
                write_jump(exit, comp.len(), comp);
            }
        }
        IntrinsicFunction::Char { args } => {
            if args.len() > 254 {
                panic!("Char has too many args");
            } else {
                for arg in args {
                    super::expression::compile(arg, source_code, comp, ExpressionContext::Eval);
                }
                comp.push(ffi::FUNC);
                comp.push(args.len() as u8);
            }
        }
        IntrinsicFunction::View(function) => function.compile(source_code, comp, ffi::FUNV2),
        IntrinsicFunction::Ascii(function) => function.compile(source_code, comp, ffi::FUNA1),
        IntrinsicFunction::Extract(function) => function.compile(source_code, comp, ffi::FUNE1),
        IntrinsicFunction::Find(function) => function.compile(source_code, comp, ffi::FUNF2),
        IntrinsicFunction::Fnumber(function) => function.compile(source_code, comp, ffi::FUNFN2),
        IntrinsicFunction::Justify(function) => function.compile(source_code, comp, ffi::FUNJ2),
        IntrinsicFunction::Length(function) => function.compile(source_code, comp, ffi::FUNL1),
        IntrinsicFunction::Piece(function) => function.compile(source_code, comp, ffi::FUNP2),
        IntrinsicFunction::Random(function) => function.compile(source_code, comp, ffi::FUNR),
        IntrinsicFunction::Reverse(function) => function.compile(source_code, comp, ffi::FUNRE),
        IntrinsicFunction::Stack(function) => function.compile(source_code, comp, ffi::FUNST1),
        IntrinsicFunction::Text(function) => function.compile(source_code, comp, ffi::FUNT),
        IntrinsicFunction::Translate(function) => function.compile(source_code, comp, ffi::FUNTR2),
        IntrinsicFunction::QLength(function) => {
            function.compile(source_code, comp, VarContext::Eval, ffi::FUNQL)
        }
        IntrinsicFunction::QSubscript(function) => {
            function.compile(source_code, comp, VarContext::Eval, ffi::FUNQS)
        }
        IntrinsicFunction::Data(function) => {
            function.compile(source_code, comp, VarContext::Build, ffi::FUND)
        }
        IntrinsicFunction::Get(function) => {
            function.compile(source_code, comp, VarContext::Build, ffi::FUNG1)
        }
        IntrinsicFunction::Increment(function) => {
            function.compile(source_code, comp, VarContext::Build, ffi::FUNI1)
        }
        IntrinsicFunction::Name(function) => {
            function.compile(source_code, comp, VarContext::BuildNullable, ffi::FUNNA1)
        }
        IntrinsicFunction::Order(function) => {
            function.compile(source_code, comp, VarContext::BuildNullable, ffi::FUNO1)
        }
        IntrinsicFunction::Query(function) => {
            function.compile(source_code, comp, VarContext::BuildNullable, ffi::FUNQ1)
        }
        IntrinsicFunction::Next(function) => {
            //Next is just an Order with a hard coded argument
            use std::str::FromStr;
            let two = Expression::Number(Value::from_str("2").unwrap().into());
            let fun = IntrinsicFunction::Order(VarFunction {
                variable: function.variable.clone(),
                function: Function {
                    required: [],
                    optional: [Some(two); 1],
                },
            });
            compile(&fun, source_code, comp);
        }
    }
}
