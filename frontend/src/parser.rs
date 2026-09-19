use std::str::FromStr;

use chumsky::{IterParser, prelude::*};
use ir::{
    Expression, Line, Routine, Tag,
    commands::{
        PostCondition,
        Write::{self},
        r#if::If,
    },
    operators::{Binary, Unary},
};
use value::{Number, Value};
pub fn routine<'src>() -> impl Parser<'src, &'src str, Routine> {
    line_parser()
        .separated_by(just("\n"))
        .allow_trailing()
        .collect::<Vec<_>>()
}

fn line_parser<'src>() -> impl Parser<'src, &'src str, Line> {
    let tag = text::ascii::ident().map_with(|tag: &str, extra| {
        use ir::Spanned;
        let temp: SimpleSpan = extra.span();
        Spanned {
            inner: Tag {
                name: tag.to_owned(),
            },
            start: temp.start(),
            end: temp.end(),
        }
    });
    tag.or_not()
        .then_ignore(just(" "))
        .then(
            choice((write(), if_parser(), else_parser()))
                //Consume next space unless it is a new line
                .then_ignore(choice((just(" ").to(()), empty().and_is(just("\n")))))
                .repeated()
                .collect(),
        )
        .map(|(tag, x)| Line {
            tag: tag,
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
fn op_b_code<'src>() -> impl Parser<'src, &'src str, Binary> {
    choice((
        //
        just("+").to(Binary::Add),
        just("-").to(Binary::Sub),
    ))
}
fn expression<'src>() -> impl Parser<'src, &'src str, Expression> {
    recursive(|expr| {
        let atom = choice((
            //Note: must either terminate or move the cursor before a recursive call.
            //To do otherwise will result in infinite recursion.
            expr.clone().delimited_by(just("("), just(")")),
            str_literal(),
            text::int(10).map(|x| Expression::Number(Number::from_str(x).unwrap())),
        ))
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
