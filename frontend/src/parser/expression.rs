use std::{cell::LazyCell, str::FromStr, sync::LazyLock};

use chumsky::{prelude::*, text::digits};
use ir::{
    Expression::{self},
    ExternalCalls, ExtrinsicFunction, IntrinsicFunction, IntrinsicVar,
    intrinsic_functions::{Function, VarFunction},
    operators::{Binary, Unary},
};
use value::{Number, Value};

use crate::parser::{function_args, parse_extrinsic_function};

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
        let string: String = x.to_owned();
        let striped = string
            .strip_prefix('\"')
            .unwrap()
            .strip_suffix('\"')
            .unwrap();
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

pub fn intrinsic_fn<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>> + Clone,
) -> impl Parser<'src, &'src str, IntrinsicFunction, Error<'src>> {
    just("$").ignore_then(
        //Note: Must be broken down into different cases to satisfy const generic bounds
        choice((
            var_fn_case(exp.clone(), |x| match x.to_lowercase().as_str() {
                "d" | "data" => Some(IntrinsicFunction::Data),
                "ql" | "qlength" => Some(IntrinsicFunction::QLength),
                _ => None,
            }),
            var_fn_case(exp.clone(), |x| match x.to_lowercase().as_str() {
                "g" | "get" => Some(IntrinsicFunction::Get),
                "i" | "increment" => Some(IntrinsicFunction::Increment),
                "q" | "query" => Some(IntrinsicFunction::Query),
                "o" | "order" => Some(IntrinsicFunction::Order),
                _ => None,
            }),
            var_fn_case(exp.clone(), |x| match x.to_lowercase().as_str() {
                "qs" | "qsubscript" => Some(
                    IntrinsicFunction::QSubscript as fn(VarFunction<1, 0>) -> IntrinsicFunction,
                ),
                _ => None,
            }),
            var_fn_case(exp.clone(), |x| match x.to_lowercase().as_str() {
                "na" | "name" => Some(IntrinsicFunction::Name),
                _ => None,
            }),
            var_fn_case(exp, |x| match x.to_lowercase().as_str() {
                "n" | "next" => Some(IntrinsicFunction::Next),
                _ => None,
            }),
        )),
    )
}

fn var_fn_case<'src, const REQUIRED: usize, const OPTIONAL: usize>(
    exp: impl Parser<'src, &'src str, Expression, extra::Full<Rich<'src, char>, (), ()>> + Clone,
    var_name: impl Fn(&'src str) -> Option<fn(VarFunction<REQUIRED, OPTIONAL>) -> IntrinsicFunction>,
) -> impl Parser<'src, &'src str, IntrinsicFunction, Error<'src>> {
    identifier()
        .filter_map(move |x| var_name(x))
        .then(var_fn_args(exp))
        .map(|(e_type, function)| e_type(function))
}

pub fn var_fn_args<'src, const REQUIRED: usize, const OPTIONAL: usize>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>> + Clone,
) -> impl Parser<'src, &'src str, VarFunction<REQUIRED, OPTIONAL>, Error<'src>> {
    variable(exp.clone())
        .then(
            just(",")
                .ignore_then(exp.clone())
                .repeated()
                .collect_exactly::<[_; REQUIRED]>(),
        )
        .then(
            just(",")
                .ignore_then(exp.clone())
                .or_not()
                .repeated()
                .collect_exactly::<[_; OPTIONAL]>(),
        )
        .map(|((variable, requred), optional)| VarFunction {
            variable,
            function: Function {
                required: requred,
                optional: optional,
            },
        })
        .delimited_by(just("("), just(")"))
}
