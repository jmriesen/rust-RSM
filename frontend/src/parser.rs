use chumsky::{IterParser, prelude::*, recovery};
use ir::{
    Expression, Line, Routine, Spanned, Tag,
    commands::{
        self, Command, PostCondition,
        Write::{self},
        r#if::If,
    },
    operators::{Binary, Unary},
};
use std::str::FromStr;
use value::{Number, Value};
pub fn routine<'src>()
-> impl Parser<'src, &'src str, Routine, chumsky::extra::Err<Rich<'src, char>>> {
    line_parser()
        .separated_by(just("\n"))
        .allow_trailing()
        .collect::<Vec<_>>()
}

fn line_parser<'src>() -> impl Parser<'src, &'src str, Line, chumsky::extra::Err<Rich<'src, char>>>
{
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
        .then(command().separated_by(just(" ")).allow_trailing().collect())
        .map(|(tag, x)| Line {
            tag: tag,
            level: 0,
            commands: x,
        })
}
fn command<'src>()
-> impl Parser<'src, &'src str, Spanned<Command>, chumsky::extra::Err<Rich<'src, char>>> {
    choice((write(), if_parser(), else_parser()))
        .recover_with(via_parser(choice((
            //Consumes arbitrary text and treats is as a command.
            //We don't want to consume our delimiters
            none_of(" \n")
                .repeated()
                .at_least(1)
                .to(commands::Command::Error),
            //Handles detecting "extra spaces"
            //Rather than trying to consume the space I am just injecting an error command (without
            //consuming anything and letting the "extra" space be treated as a deliminator.
            empty().and_is(just(" ")).to(commands::Command::Error),
        ))))
        .map_with(|inner, extra| Spanned {
            inner,
            start: extra.span().start,
            end: extra.span().end,
        })
}

fn write<'src>()
-> impl Parser<'src, &'src str, ir::commands::Command, chumsky::extra::Err<Rich<'src, char>>> {
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
fn if_parser<'src>()
-> impl Parser<'src, &'src str, ir::commands::Command, chumsky::extra::Err<Rich<'src, char>>> {
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
fn else_parser<'src>()
-> impl Parser<'src, &'src str, ir::commands::Command, chumsky::extra::Err<Rich<'src, char>>> {
    just("e ").map(|_| ir::commands::Command::Else)
}

fn write_arg<'src>()
-> impl Parser<'src, &'src str, Spanned<Write>, chumsky::extra::Err<Rich<'src, char>>> {
    choice((
        //
        just("!").to(Write::Bang),
        just("#").to(Write::Clear),
        expression().map(|x| Write::Expression(x)),
    ))
    .map_with(|x, exra| Spanned {
        inner: x,
        start: exra.span().start,
        end: exra.span().end,
    })
}

fn str_literal<'src>()
-> impl Parser<'src, &'src str, Expression, chumsky::extra::Err<Rich<'src, char>>> {
    none_of("\"")
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map(|x| Expression::String(Value::from_str(&x).unwrap()))
}

fn op_u_code<'src>() -> impl Parser<'src, &'src str, Unary, chumsky::extra::Err<Rich<'src, char>>> {
    choice((
        //
        just("+").to(Unary::Plus),
        just("-").to(Unary::Minus),
        just("'").to(Unary::Not),
    ))
}
fn op_b_code<'src>() -> impl Parser<'src, &'src str, Binary, chumsky::extra::Err<Rich<'src, char>>>
{
    choice((
        //
        just("+").to(Binary::Add),
        just("-").to(Binary::Sub),
    ))
}
fn expression<'src>()
-> impl Parser<'src, &'src str, Expression, chumsky::extra::Err<Rich<'src, char>>> {
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
        write().parse("w !,#,\"test\"").into_result(),
        Ok(ir::commands::Command::Write(PostCondition {
            condition: None,
            value: vec![
                Spanned {
                    inner: Write::Bang,
                    start: 2,
                    end: 3
                },
                Spanned {
                    inner: Write::Clear,
                    start: 4,
                    end: 5
                },
                Spanned {
                    inner: Write::Expression(Expression::String(Value::from_str("test").unwrap())),
                    start: 6,
                    end: 12
                },
            ]
        }))
    );
    assert_eq!(
        line_parser().parse("tag w \"\" ").into_result(),
        Ok(Line {
            tag: Some(Spanned {
                inner: Tag {
                    name: "tag".to_string()
                },
                start: 0,
                end: 3
            }),
            level: 0,
            commands: vec![Spanned {
                inner: ir::commands::Command::Write(PostCondition {
                    condition: None,
                    value: vec![Spanned {
                        inner: Write::Expression(Expression::String(Value::from_str("").unwrap())),
                        start: 6,
                        end: 8
                    },]
                }),
                start: 4,
                end: 8
            }]
        })
    );
}
