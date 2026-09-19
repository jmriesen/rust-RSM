use std::str::FromStr;

use chumsky::{
    IterParser,
    input::StrInput,
    prelude::*,
    text::{ascii::ident, newline},
};
use ir::{
    Expression, Line, Routine,
    commands::{
        PostCondition,
        Write::{self},
        r#if::If,
    },
    operators::Unary,
};
use value::{Number, Value};
pub fn routine<'src>() -> impl Parser<'src, &'src str, Routine> {
    line_parser()
        .separated_by(just("\n"))
        .allow_trailing()
        .collect::<Vec<_>>()
}

fn line_parser<'src>() -> impl Parser<'src, &'src str, Line> {
    let tag = text::ascii::ident().map(|_x: &str| ());
    tag.or_not()
        .ignore_then(just(" "))
        .ignore_then(
            choice((write(), if_parser(), else_parser()))
                //Consume next space unless it is a new line
                .then_ignore(choice((just(" ").to(()), empty().and_is(just("\n")))))
                .repeated()
                .collect(),
        )
        .map(|x| Line {
            level: 0,
            commands: x,
        })
}

fn write<'src>() -> impl Parser<'src, &'src str, ir::commands::Command> {
    just("w ")
        .ignore_then(
            write_arg()
                .separated_by(just(","))
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .map(|x| {
            ir::commands::Command::Write(PostCondition {
                condition: None,
                value: x,
            })
        })
}
fn if_parser<'src>() -> impl Parser<'src, &'src str, ir::commands::Command> {
    just("i ")
        .ignore_then(
            expression()
                .map(If)
                .separated_by(just(","))
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .map(|x| ir::commands::Command::If(x))
}
fn else_parser<'src>() -> impl Parser<'src, &'src str, ir::commands::Command> {
    just("e ").map(|_| ir::commands::Command::Else)
}

fn write_arg<'src>() -> impl Parser<'src, &'src str, Write> {
    choice((
        //
        just("!").to(Write::Bang),
        just("#").to(Write::Clear),
        expression().map(|x| Write::Expression(x)),
    ))
}

fn str_literal<'src>() -> impl Parser<'src, &'src str, Expression> {
    none_of("\"")
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map(|x| Expression::String(Value::from_str(&x).unwrap()))
}

fn op_u_code<'src>() -> impl Parser<'src, &'src str, Unary> {
    choice((
        //
        just("+").to(Unary::Plus),
        just("-").to(Unary::Minus),
        just("'").to(Unary::Not),
    ))
}
fn expression<'src>() -> impl Parser<'src, &'src str, Expression> {
    recursive(|inner| {
        choice((
            //
            str_literal().boxed(),
            op_u_code()
                .then(inner)
                .map(|(op_code, expresstion)| Expression::UnaryExpression {
                    op_code,
                    expresstion: Box::new(expresstion),
                })
                .boxed(),
            text::int(10).map(|x| Expression::Number(Number::from_str(x).unwrap())),
        ))
    })
}

#[test]
fn test_parser() {
    // Our parser expects empty strings, so this should parse successfully
    assert_eq!(
        write().parse("w !,#,\"test\" ").into_result(),
        Ok(ir::commands::Command::Write(PostCondition {
            condition: None,
            value: vec![
                Write::Bang,
                Write::Clear,
                Write::Expression(Expression::String(Value::from_str("test").unwrap()))
            ]
        }))
    );
    assert_eq!(
        line_parser().parse("tag w \"\" ").into_result(),
        Ok(Line {
            level: 0,
            commands: vec![]
        })
    );
}
