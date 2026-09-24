use std::{array::from_fn, str::FromStr};

use chumsky::{prelude::*, text::digits};
use ir::{
    Expression::{self},
    ExternalCalls, IntrinsicFunction, IntrinsicVar, Variable,
    intrinsic_functions::{Function, SelectTerm, VarFunction},
    operators::{Binary, Unary},
};
use value::{Number, Value};

use crate::parser::parse_extrinsic_function;

use super::{
    peek,
    variable::{identifier, variable},
};

use super::Error;
fn str_literal<'src>() -> impl Parser<'src, &'src str, Value, Error<'src>> {
    choice((
        none_of("\"").ignored(),
        just("\"").then(just("\"")).ignored(),
    ))
    .repeated()
    .collect::<Vec<_>>()
    .delimited_by(just('"'), just('"'))
    .to_slice()
    .map(|x: &str| {
        let striped = x
            .strip_prefix('\"')
            .unwrap()
            .strip_suffix('\"')
            .unwrap()
            .to_owned()
            .replace("\"\"", "\"");
        Value::from_str(&striped).unwrap()
    })
    .labelled("String Literal")
    .as_terminal()
}
fn number<'src>() -> impl Parser<'src, &'src str, Number, Error<'src>> {
    choice((
        digits(10)
            .then(just(".").then(digits(10)).or_not())
            .to_slice(),
        just(".").then(digits(10)).to_slice(),
    ))
    .map(|x: &str| Number::from_str(x).unwrap())
    .labelled("String Literal")
    .as_terminal()
}

fn op_u_code<'src>() -> impl Parser<'src, &'src str, Unary, Error<'src>> {
    choice((
        //
        just("+").to(Unary::Plus),
        just("-").to(Unary::Minus),
        just("'").to(Unary::Not),
    ))
}
fn op_b_code<'src>() -> impl Parser<'src, &'src str, Binary, Error<'src>> {
    choice((
        //
        just("+").to(Binary::Add),
        just("-").to(Binary::Sub),
        just("=").to(Binary::Equal),
        just("\\").to(Binary::IntDivide),
        // Indirect pattern matching is handle just like any other binary expression.
        // Note We do want to consume the indirect marker as what follows is just the expression to
        // evaluate.
        //NOTE: literal patterns are handled differently
        // Indirect patterns are handle like any other binary expression (after we remove the
        // indirect marker.
        pattern_match_op_code().then_ignore(just("@")),
    ))
}

fn pattern_match_op_code<'src>() -> impl Parser<'src, &'src str, Binary, Error<'src>> {
    choice((
        //
        just("?").to(Binary::Pattern),
        just("'?").to(Binary::NotPattern),
    ))
}

pub fn pattern<'src>() -> impl Parser<'src, &'src str, &'src str, Error<'src>> {
    let repetition = choice((
        // between
        text::int(10).then(just(".")).then(text::int(10)).ignored(),
        // at most
        just(".").then(text::int(10)).ignored(),
        // at least
        text::int(10).then(just(".")).ignored(),
        // exact number
        text::int(10).ignored(),
        // any number
        just(".").ignored(),
    ));
    let codes = one_of("ACELNPUA").ignored();

    recursive(|pat| {
        let atom = choice((
            //normal
            codes.ignored(),
            str_literal().ignored(),
            pat
                //Or-ing
                .separated_by(just(","))
                .at_least(1)
                //Grouping
                .delimited_by(just('('), just(')')),
        ));
        let atom = atom.boxed();
        repetition.then(atom).repeated().at_least(1).to_slice()
    })
}

pub fn expression<'src>() -> impl Parser<'src, &'src str, Expression, Error<'src>> {
    recursive(|expr| {
        let expr = expr.boxed();
        let atom = choice((
            //Note: must either terminate or move the cursor before a recursive call.
            //To do otherwise will result in infinite recursion.
            external_calls(expr.clone()),
            expr.clone().delimited_by(just("("), just(")")),
            variable(expr.clone()).map(Expression::Variable),
            just("@")
                .ignore_then(expr.clone())
                .map(|x| Expression::InderectExpression(Box::new(x))),
            str_literal().map(Expression::String),
            number().map(Expression::Number),
            intrinsic_fn(expr.clone()).map(|x| Expression::IntrinsicFunction(Box::new(x))),
            just("$$")
                .ignore_then(parse_extrinsic_function(expr.clone()))
                .map(Expression::ExtrinsicFunction),
            intrinsic_var().map(Expression::IntrinsicVar),
        ))
        .labelled("expression atom")
        .as_terminal()
        .boxed();

        let unary = op_u_code()
            .repeated()
            //NOTE: Use of atom hear instead of expression is important.
            //if we used expression parsing order could be messed up.
            //-a+b should be (-1)+(b)
            //If I used expression here I would get -(a+b)
            .foldr(atom.clone(), |op_code, expression| {
                Expression::UnaryExpression {
                    op_code,
                    expresstion: Box::new(expression),
                }
            })
            .boxed();
        // Handle operator cases.
        // Note: To prevent infinite recursion operators are applied to atoms not expressions.
        // If the first thing we do to parse a binary expression is try and parse another (binary)
        // expression we are in for infinite recursion.
        // `atom` is guarantied to move the cursor before trying to recurs.
        //
        //NOTE: This also handles the atom case (no trailing operator + atom) since the first
        //argument to fold matches on zero repetitions
        choice(
            //first (and possible only expression)
            (atom.clone(), unary.clone()),
        )
        .foldl(
            choice((
                pattern_match_op_code()
                    // Only grab when a literal value
                    .then_ignore(peek(just("@")).not())
                    .then(pattern().map(|x| Expression::String(Value::from_str(x).unwrap()))),
                op_b_code().then(expr),
            ))
            .repeated(),
            //there are two types of binary
            //Expression based and pattern based.
            //Expression based I can handle by just checking that the next
            //character is a @
            //literal based I can do vie the opposite approach.
            //I kind of question if they shoudl be the same type? no it will
            //be fine
            |lhs, (op, rhs)| Expression::BinaryExpression {
                left: Box::new(lhs),
                op_code: op,
                right: Box::new(rhs),
            },
        )
        .boxed()
    })
    .labelled("expression")
}

pub fn intrinsic_var<'src>() -> impl Parser<'src, &'src str, IntrinsicVar, Error<'src>> {
    just("$").ignore_then(identifier().filter_map(|x| {
        //
        match x.to_lowercase().as_str() {
            "sy" | "system" => Some(IntrinsicVar::System),
            "ec" | "ecode" => Some(IntrinsicVar::Ecode),
            "st" | "stack" => Some(IntrinsicVar::StackVar),
            "es" | "estack" => Some(IntrinsicVar::Estack),
            "et" | "etrap" => Some(IntrinsicVar::Etrap),
            "t" | "test" => Some(IntrinsicVar::Test),
            "d" | "device" => Some(IntrinsicVar::Device),
            "h" | "horolog" => Some(IntrinsicVar::Horolog),
            "i" | "io" => Some(IntrinsicVar::Io),
            "j" | "job" => Some(IntrinsicVar::Job),
            "k" | "key" => Some(IntrinsicVar::Key),
            "p" | "principal" => Some(IntrinsicVar::Principal),
            "q" | "quit" => Some(IntrinsicVar::Quit),
            "r" | "reference" => Some(IntrinsicVar::Reference),
            "s" | "storage" => Some(IntrinsicVar::Storage),
            "x" => Some(IntrinsicVar::X),
            "y" => Some(IntrinsicVar::Y),
            _ => None,
        }
    }))
}

pub fn external_calls<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>>,
) -> impl Parser<'src, &'src str, Expression, Error<'src>> {
    just("$&")
        .ignore_then(
            identifier().filter_map(|x| match x.to_uppercase().as_str() {
                "%DIRECTORY" => Some(ExternalCalls::Directory),
                "%HOST" => Some(ExternalCalls::Host),
                "%FILE" => Some(ExternalCalls::File),
                "%ERRMSG" => Some(ExternalCalls::ErrMsg),
                "%OPCOM" => Some(ExternalCalls::OpCom),
                "%SIGNAL" => Some(ExternalCalls::Signal),
                "%SPAWN" => Some(ExternalCalls::Spawn),
                "%VERSION" => Some(ExternalCalls::Version),
                "%ZWRITE" => Some(ExternalCalls::Zwrite),
                "E" => Some(ExternalCalls::E),
                "PASCHK" => Some(ExternalCalls::Paschk),
                "V" => Some(ExternalCalls::V),
                "X" => Some(ExternalCalls::XCallX),
                "XRSM" => Some(ExternalCalls::Xrsm),
                "%SETENV" => Some(ExternalCalls::SetEnv),
                "%GETENV" => Some(ExternalCalls::GetEnv),
                "%ROUCHK" => Some(ExternalCalls::RouChk),
                "%FORK" => Some(ExternalCalls::Fork),
                "%IC" => Some(ExternalCalls::IC),
                "%WAIT" => Some(ExternalCalls::Wait),
                "DEBUG" => Some(ExternalCalls::Debug),
                "%COMPRESS" => Some(ExternalCalls::Compress),
                _ => None,
            }),
        )
        .then(
            exp.separated_by(just(","))
                .collect()
                .delimited_by(just("("), just(")")),
        )
        .map(|(op_code, args)| Expression::ExternalCalls { args, op_code })
}

struct Output<T> {
    value: T,
    err: Option<&'static str>,
}
impl<T> Output<T> {
    fn map<U>(self, map: impl FnOnce(T) -> U) -> Output<U> {
        Output {
            value: map(self.value),
            err: self.err,
        }
    }
}

pub fn intrinsic_fn<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>> + Clone,
) -> impl Parser<'src, &'src str, IntrinsicFunction, Error<'src>> {
    choice((
        intrinsic_var_fn(exp.clone()),
        intrinsic_non_var_fn(exp.clone()),
        select(exp),
    ))
    .validate(|output, extra, emiter| {
        if let Some(msg) = output.err {
            emiter.emit(Rich::custom(extra.span(), msg));
        }
        output.value
    })
}

fn intrinsic_var_fn<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>> + Clone,
) -> impl Parser<'src, &'src str, Output<IntrinsicFunction>, Error<'src>> {
    just("$").ignore_then(
        identifier()
            .then(
                variable(exp.clone())
                    .then(
                        just(",")
                            .ignore_then(exp.clone())
                            .repeated()
                            .collect::<Vec<_>>(),
                    )
                    .delimited_by(just('('), just(')')),
            )
            //I want to keep parsing If I get the number of arguments wrong.
            //I don't want to keep parsing if I got the function wrong.
            .filter_map(|(name, (var, args))| match name.to_lowercase().as_str() {
                "d" | "data" => Some(init_var_fn(IntrinsicFunction::Data, var, args)),
                "ql" | "qlength" => Some(init_var_fn(IntrinsicFunction::QLength, var, args)),
                "g" | "get" => Some(init_var_fn(IntrinsicFunction::Get, var, args)),
                "i" | "increment" => Some(init_var_fn(IntrinsicFunction::Increment, var, args)),
                "q" | "query" => Some(init_var_fn(IntrinsicFunction::Query, var, args)),
                "o" | "order" => Some(init_var_fn(IntrinsicFunction::Order, var, args)),
                "qs" | "qsubscript" => Some(init_var_fn(IntrinsicFunction::QSubscript, var, args)),
                "na" | "name" => Some(init_var_fn(IntrinsicFunction::Name, var, args)),
                "n" | "next" => Some(init_var_fn(IntrinsicFunction::Next, var, args)),
                _ => None,
            }),
    )
}
fn intrinsic_non_var_fn<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>> + Clone,
) -> impl Parser<'src, &'src str, Output<IntrinsicFunction>, Error<'src>> {
    just("$").ignore_then(
        identifier()
            .then(
                exp.clone()
                    .separated_by(just(","))
                    .collect::<Vec<_>>()
                    .delimited_by(just('('), just(')')),
            )
            //I want to keep parsing If I get the number of arguments wrong.
            //I don't want to keep parsing if I got the function wrong.
            .filter_map(|(name, args)| match name.to_lowercase().as_str() {
                "v" | "view" => Some(init_fn(IntrinsicFunction::View, args)),
                "t" | "text" => Some(init_fn(IntrinsicFunction::Text, args)),
                "tr" | "translate" => Some(init_fn(IntrinsicFunction::Translate, args)),
                "f" | "find" => Some(init_fn(IntrinsicFunction::Find, args)),
                "fn" | "fnumber" => Some(init_fn(IntrinsicFunction::Fnumber, args)),
                "r" | "random" => Some(init_fn(IntrinsicFunction::Random, args)),
                "p" | "piece" => Some(init_fn(IntrinsicFunction::Piece, args)),
                "j" | "justify" => Some(init_fn(IntrinsicFunction::Justify, args)),
                "e" | "extract" => Some(init_fn(IntrinsicFunction::Extract, args)),
                "a" | "ascii" => Some(init_fn(IntrinsicFunction::Ascii, args)),
                "re" | "reverse" => Some(init_fn(IntrinsicFunction::Reverse, args)),
                "c" | "char" => Some(Output {
                    value: IntrinsicFunction::Char { args },
                    err: None,
                }),
                "l" | "length" => Some(init_fn(IntrinsicFunction::Length, args)),
                "st" | "stack" => Some(init_fn(IntrinsicFunction::Stack, args)),
                _ => None,
            }),
    )
}

//Should I make my own output/errors result type?
//Mach the style of above using monoids
fn init_var_fn<const REQUIRED: usize, const OPTIONAL: usize>(
    variant: fn(VarFunction<REQUIRED, OPTIONAL>) -> IntrinsicFunction,
    var: Variable,
    args: Vec<Expression>,
) -> Output<IntrinsicFunction> {
    map_args(args).map(move |args| {
        variant(VarFunction {
            variable: var,
            function: args,
        })
    })
}
fn init_fn<const REQUIRED: usize, const OPTIONAL: usize>(
    variant: fn(Function<REQUIRED, OPTIONAL>) -> IntrinsicFunction,
    args: Vec<Expression>,
) -> Output<IntrinsicFunction> {
    map_args(args).map(move |function| variant(function))
}

//Lets parse any number a variable and then validate to report an error.
fn map_args<'src, const REQUIRED: usize, const OPTIONAL: usize>(
    args: Vec<Expression>,
) -> Output<Function<REQUIRED, OPTIONAL>> {
    let err = if args.len() < REQUIRED {
        Some("Function Expects more arguments")
    } else if REQUIRED + OPTIONAL < args.len() {
        Some("Function Expects fewer arguments")
    } else {
        None
    };
    let mut iter = args.into_iter();
    Output {
        value: Function {
            //Fill with placeholder arguments so we can keep parsing
            required: from_fn(|_| iter.next().unwrap_or(Expression::String(Value::empty()))),
            optional: from_fn(|_| iter.next()),
        },
        err,
    }
}

fn select<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>> + Clone,
) -> impl Parser<'src, &'src str, Output<IntrinsicFunction>, Error<'src>> {
    just("$").ignore_then(
        identifier()
            .then(
                exp.clone()
                    .then_ignore(just(":"))
                    .then(exp)
                    .map(|(condition, value)| SelectTerm { condition, value })
                    .separated_by(just(","))
                    .collect::<Vec<_>>()
                    .delimited_by(just("("), just(")")),
            )
            .filter_map(|(name, terms)| match name.to_lowercase().as_str() {
                "s" | "select" => Some(Output {
                    value: IntrinsicFunction::Select { terms },
                    err: None,
                }),
                _ => None,
            }),
    )
}
