use chumsky::{IterParser, combinator::AndIs, prelude::*, primitive::Empty};
use ir::{
    Expression::{self},
    ExtrinsicFunction, Line, Routine, Spanned, Tag,
    commands::{
        self, Command, PostCondition,
        Write::{self},
        r#break::Break,
        close::Close,
        r#do::Do,
        r#for::{Argument, For, ForKind},
        r#if::If,
        set::Set,
    },
    extrinsic_function::{
        Args,
        Location::{self},
    },
    variable::VariableType,
};
mod expression;
mod variable;
use expression::expression;

type Error<'src> = chumsky::extra::Err<Rich<'src, char>>;

use crate::parser::variable::{identifier, local_variable_no_subscripts, variable};
pub fn peek<'src, U, B>(item: B) -> AndIs<Empty<&'src str, Error<'src>>, B, U>
where
    B: Parser<'src, &'src str, U, Error<'src>>,
{
    empty().and_is(item)
}

pub fn keyword<'src>(keyword: &'static str) -> impl Parser<'src, &'src str, (), Error<'src>> {
    let (abriveation, _) = keyword.split_at(1);
    choice((
        just(keyword.to_lowercase()),
        just(keyword.to_ascii_uppercase()),
        just(abriveation.to_lowercase()),
        just(abriveation.to_ascii_uppercase()),
    ))
    .ignored()
    .labelled(keyword)
    .as_terminal()
}
fn args_list<'src, T>(
    term: impl Parser<'src, &'src str, T, Error<'src>>,
) -> impl Parser<'src, &'src str, Vec<T>, Error<'src>> {
    term.separated_by(just(","))
        .collect::<Vec<_>>()
        .delimited_by(just("("), just(")"))
}

pub fn routine<'src>() -> impl Parser<'src, &'src str, Routine, Error<'src>> {
    line_parser()
        .separated_by(just("\n"))
        .allow_trailing()
        .collect::<Vec<_>>()
        //Ignore the trailing metadata in the test files.
        .then_ignore(just("---").then(any().repeated()).or_not())
}

fn line_parser<'src>() -> impl Parser<'src, &'src str, Line, Error<'src>> {
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
    let line_level = just(".").repeated().count();

    let commands = command().separated_by(just(" ")).allow_trailing().collect();
    let commands2 = command().separated_by(just(" ")).allow_trailing().collect();

    //Tag\n
    //Tag Commands\n
    // line_levelCommands\n
    //TODO: This feels like this can be simplified
    choice((
        tag.then_ignore(just(" "))
            .then(commands)
            .map(|(tag, commands)| Line {
                tag: Some(tag),
                level: 0,
                commands,
            }),
        just(" ")
            .ignore_then(line_level)
            .then(commands2)
            .map(|(level, commands)| Line {
                tag: None,
                level: level as u16,
                commands,
            }),
        tag.map(|tag| Line {
            tag: Some(tag),
            level: 0,
            commands: vec![],
        }),
    ))
}

fn command<'src>() -> impl Parser<'src, &'src str, Spanned<Command>, Error<'src>> {
    recursive(|cmd| {
        choice((
            write(),
            if_parser(),
            else_parser(),
            set_parser(),
            for_parser(cmd),
            quit_parser(),
            kill_parser(),
            do_parser(),
            break_parser(),
            close_parser(),
        ))
        .boxed()
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
            peek(just(" ")).to(commands::Command::Error),
        ))))
        .map_with(|inner, extra| Spanned {
            inner,
            start: extra.span().start,
            end: extra.span().end,
        })
    })
}

fn write<'src>() -> impl Parser<'src, &'src str, Command, Error<'src>> {
    keyword("write")
        .then_ignore(just(" "))
        .ignore_then(
            write_arg()
                .separated_by(just(","))
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .map(|x| {
            Command::Write(PostCondition {
                condition: None,
                value: x,
            })
        })
}
fn if_parser<'src>() -> impl Parser<'src, &'src str, Command, Error<'src>> {
    keyword("if")
        .then_ignore(just(" "))
        .ignore_then(
            expression()
                .map(If)
                .separated_by(just(","))
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .map(|x| Command::If(dbg!(x)))
}
fn else_parser<'src>() -> impl Parser<'src, &'src str, Command, Error<'src>> {
    keyword("else")
        .then_ignore(just(" "))
        .map(|_| Command::Else)
}

fn write_arg<'src>() -> impl Parser<'src, &'src str, Spanned<Write>, Error<'src>> {
    choice((
        //
        just("!").to(Write::Bang),
        just("#").to(Write::Clear),
        just("?").ignore_then(expression()).map(|x| Write::Tab(x)),
        expression().map(|x| Write::Expression(x)),
    ))
    .map_with(|x, exra| Spanned {
        inner: x,
        start: exra.span().start,
        end: exra.span().end,
    })
}

fn set_parser<'src>() -> impl Parser<'src, &'src str, Command, Error<'src>> {
    keyword("set")
        .then_ignore(just(" "))
        .ignore_then(variable(expression().boxed()))
        .then_ignore(just("="))
        .then(expression())
        .map(|(variable, value)| Command::Set(Set { variable, value }))
}

fn post_condition<'src>() -> impl Parser<'src, &'src str, Option<Expression>, Error<'src>> {
    just(":").ignore_then(expression()).or_not()
}

fn quit_parser<'src>() -> impl Parser<'src, &'src str, Command, Error<'src>> {
    keyword("quit")
        .ignore_then(post_condition())
        .then_ignore(space_or_eol())
        .then(
            expression()
                .separated_by(just(","))
                .collect::<Vec<_>>()
                .validate(|mut return_value, extra, emiter| {
                    if return_value.len() > 1 {
                        emiter.emit(Rich::custom(
                            extra.span(),
                            "Quit can only have zero or one argument",
                        ));
                    }
                    if return_value.len() == 1 {
                        emiter.emit(Rich::custom(
                            extra.span(),
                            "Not yet supported quit with args",
                        ));
                    }
                    commands::Quit(return_value.pop())
                }),
        )
        .map(|(condition, value)| Command::Quit(PostCondition { condition, value }))
}
fn do_parser<'src>() -> impl Parser<'src, &'src str, Command, Error<'src>> {
    keyword("do")
        .ignore_then(post_condition())
        .then(
            space_or_eol()
                .ignore_then(
                    parse_extrinsic_function(expression())
                        .then(post_condition())
                        .map(|(value, condition)| PostCondition { condition, value })
                        .separated_by(just(","))
                        .collect::<Vec<_>>(),
                )
                .map(|x| {
                    if x.is_empty() {
                        Do::ArgumentLess
                    } else {
                        Do::FunctionCall(x)
                    }
                }),
        )
        .map(|(condition, value)| Command::Do(PostCondition { condition, value }))
}

fn parse_extrinsic_function<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>>,
) -> impl Parser<'src, &'src str, ExtrinsicFunction, Error<'src>> {
    choice((
        identifier()
            .then_ignore(just("^"))
            .then(identifier())
            .map(|(tag, routine)| Location::TagRoutine(tag.to_owned(), routine.to_owned())),
        just("^")
            .ignore_then(identifier())
            .map(|x| Location::Routine(x.to_owned())),
        identifier().map(|x| Location::Tag(x.to_owned())),
    ))
    .then(function_args(exp).or_not())
    .map(|(location, args)| ExtrinsicFunction {
        location,
        arguments: args.unwrap_or_default(),
    })
}
fn function_args<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>>,
) -> impl Parser<'src, &'src str, Vec<Args>, Error<'src>> {
    args_list(choice((
        exp.map(Args::Expression),
        just(".")
            .ignore_then(local_variable_no_subscripts())
            .map(Args::ByRef),
        empty().to(Args::VarUndefined),
    )))
    .map(|mut args| {
        // VarUndefined is not allowed if it is the last argument in the argument list.
        // It is easier to parse it as if it was allowed and then remove it after the fact. (fewer
        // special cases needed)
        args.pop_if(|x| x == &Args::VarUndefined);
        args
    })
}

fn kill_parser<'src>() -> impl Parser<'src, &'src str, Command, Error<'src>> {
    use commands::kill::KillType as E;
    keyword("kill")
        .ignore_then(
            space_or_eol().ignore_then(
                choice((
                    variable(expression().boxed()).map(|var| commands::kill::Kill {
                        r#type: E::Inclusive,
                        variables: vec![var],
                    }),
                    variable(expression().boxed())
                        .delimited_by(just("("), just(")"))
                        .validate(|var, extra, emiter| {
                            if matches!(&var.var_type,VariableType::Named { name:_, globle_ident:None }) && var.subscripts.is_empty(){
                                var
                            }else{
                                emiter.emit(Rich::custom(extra.span(), "kill exclusive is only supported for local variables with no subscripts"));
                                variable::dummy_variable()
                            }

                        })
                        .map(|var| commands::kill::Kill {
                            r#type: E::Exclusive,
                            variables: vec![var],
                        }),
                ))
                    .separated_by(just(","))
                    .collect::<Vec<_>>()
                    .map(|x| {
                        if x.is_empty() {
                            vec![commands::kill::Kill {
                                r#type: E::Exclusive,
                                variables: vec![],
                            }]
                        } else {
                            x
                        }
                    }),
            ),
        )
        .map(|value| Command::Kill(value))
}

fn break_parser<'src>() -> impl Parser<'src, &'src str, Command, Error<'src>> {
    keyword("brake")
        .ignore_then(post_condition())
        .then(
            just(" ")
                .ignore_then(expression().separated_by(just(",")).collect::<Vec<_>>())
                .map(|x| {
                    if x.is_empty() {
                        Break::ArgumentLess
                    } else {
                        Break::Arg(x)
                    }
                }),
        )
        .map(|(condition, value)| Command::Break(PostCondition { condition, value }))
}
fn close_parser<'src>() -> impl Parser<'src, &'src str, Command, Error<'src>> {
    keyword("close")
        .ignore_then(post_condition())
        .then(
            just(" ")
                .ignore_then(
                    expression()
                        .separated_by(just(","))
                        .at_least(1)
                        .collect::<Vec<_>>(),
                )
                .map(|x| x.into_iter().map(Close).collect()),
        )
        .map(|(condition, value)| Command::Close(PostCondition { condition, value }))
}

fn space_or_eol<'src>() -> impl Parser<'src, &'src str, (), Error<'src>> {
    choice((just(" ").ignored(), peek(just("\n")).ignored(), end()))
}

///WARN: Look at warning on `variable`
fn for_parser<'src>(
    cmd: impl Parser<'src, &'src str, Spanned<Command>, Error<'src>>,
) -> impl Parser<'src, &'src str, Command, Error<'src>> {
    let for_args = expression()
        .separated_by(just(":"))
        .at_least(1)
        .at_most(3)
        .collect()
        .map(|args: Vec<_>| {
            let mut args = args.into_iter();
            let start = args
                .next()
                .expect("already bounds checked by at_least call");
            let increment = args.next();
            let increment_end = increment.map(|inc| (inc, args.next()));
            Argument {
                start,
                increment_end,
            }
        });

    keyword("for")
        .then_ignore(just(" "))
        .ignore_then(choice((
            variable(expression().boxed())
                .then_ignore(just("="))
                .then(for_args.separated_by(just(",")).at_least(1).collect())
                .map(|(variable, arguments)| ForKind::VarLoop {
                    variable,
                    arguments,
                }),
            empty().to(ForKind::Infinite),
        )))
        .then_ignore(just(" "))
        .then(cmd.separated_by(just(" ")).collect())
        .map(|(kind, commands)| Command::For(For { kind, commands }))
}

#[cfg(test)]
mod test {

    use std::{fs::File, io::Write};

    use chumsky::Parser;

    use super::{line_parser, write};

    #[test]
    fn write_command() {
        insta::assert_debug_snapshot!(write().parse("w !,#,\"test\"").into_output_errors());
    }

    #[test]
    fn full_line() {
        insta::assert_debug_snapshot!(line_parser().parse("tag w !").into_output_errors());
    }

    #[test]
    fn recover_from_extra_space() {
        insta::assert_debug_snapshot!(line_parser().parse(" w !  w ! ").into_output_errors());
    }
    #[test]
    fn recover_from_unknown_command() {
        insta::assert_debug_snapshot!(line_parser().parse(" foo bar").into_output_errors());
    }
    #[test]
    fn tmp() {
        let mut file = File::create("temp.svg").unwrap();
        file.write_all(format!("{}", line_parser().debug().to_railroad_svg()).as_bytes())
            .unwrap()
    }
}
