use std::str::FromStr;

use chumsky::{prelude::*, text::digits};
use ir::{
    Expression::{self, ExtrinsicFunction},
    IntrinsicFunction, IntrinsicVar,
    intrinsic_functions::{Function, VarFunction},
    operators::{Binary, Unary},
};
use value::{Number, Value};

use crate::parser::parse_extrinsic_function;

use super::variable::{identifier, variable};

use super::Error;
fn str_literal<'src>() -> impl Parser<'src, &'src str, Value, Error<'src>> {
    none_of("\"")
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map(|x| Value::from_str(&x).unwrap())
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
    ))
}
pub fn expression<'src>() -> impl Parser<'src, &'src str, Expression, Error<'src>> {
    recursive(|expr| {
        let expr = expr.boxed();
        let atom = choice((
            //Note: must either terminate or move the cursor before a recursive call.
            //To do otherwise will result in infinite recursion.
            expr.clone().delimited_by(just("("), just(")")),
            just("@")
                .ignore_then(expr.clone())
                .map(|x| Expression::InderectExpression(Box::new(x))),
            str_literal().map(Expression::String),
            number().map(Expression::Number),
            variable(expr.clone()).map(Expression::Variable),
            intrinsic_fn(expr.clone()).map(|x| Expression::IntrinsicFunction(Box::new(x))),
            just("$$")
                .ignore_then(parse_extrinsic_function(expr))
                .map(Expression::ExtrinsicFunction),
            intrinsic_var().map(Expression::IntrinsicVar),
        ))
        .labelled("expression atom")
        .boxed();
        // Handle operator cases.
        // Note: To prevent infinite recursion operators are applied to atoms not expressions.
        // If the first thing we do to parse a binary expression is try and parse another (binary)
        // expression we are in for infinite recursion.
        // `atom` is guarantied to move the cursor before trying to recurs.
        choice((
            //
            //NOTE: This also handles the atom case (no trailing operator + atom) since the first
            //argument to fold matches on zero repetitions
            atom.clone()
                .foldl(
                    op_b_code().then(atom.clone()).repeated(),
                    |lhs, (op, rhs)| Expression::BinaryExpression {
                        left: Box::new(lhs),
                        op_code: op,
                        right: Box::new(rhs),
                    },
                )
                .boxed(),
            op_u_code()
                .repeated()
                .foldr(atom.clone(), |op_code, expression| {
                    Expression::UnaryExpression {
                        op_code,
                        expresstion: Box::new(expression),
                    }
                })
                .boxed(),
        ))
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
