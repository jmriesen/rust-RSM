use chumsky::{IterParser, prelude::*};
use ir::{
    Expression, Line, Routine, Spanned, Tag,
    commands::{
        self, Command, PostCondition,
        Write::{self},
        r#for::{Argument, For, ForKind},
        r#if::If,
        set::Set,
    },
};
mod expression;
mod variable;
use expression::expression;

use crate::parser::variable::{local_variable_no_subscripts, variable};

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
}

type Error<'src> = chumsky::extra::Err<Rich<'src, char>>;
pub fn routine<'src>() -> impl Parser<'src, &'src str, Routine, Error<'src>> {
    line_parser()
        .separated_by(just("\n"))
        .allow_trailing()
        .collect::<Vec<_>>()
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
            empty().and_is(just(" ")).to(commands::Command::Error),
        ))))
        .map_with(|inner, extra| Spanned {
            inner,
            start: extra.span().start,
            end: extra.span().end,
        })
    })
}

fn write<'src>() -> impl Parser<'src, &'src str, ir::commands::Command, Error<'src>> {
    keyword("write")
        .then_ignore(just(" "))
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
fn if_parser<'src>() -> impl Parser<'src, &'src str, ir::commands::Command, Error<'src>> {
    keyword("if")
        .then_ignore(just(" "))
        .ignore_then(
            expression()
                .map(If)
                .separated_by(just(","))
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .map(|x| ir::commands::Command::If(dbg!(x)))
}
fn else_parser<'src>() -> impl Parser<'src, &'src str, ir::commands::Command, Error<'src>> {
    keyword("else")
        .then_ignore(just(" "))
        .map(|_| ir::commands::Command::Else)
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

fn set_parser<'src>() -> impl Parser<'src, &'src str, ir::commands::Command, Error<'src>> {
    keyword("set")
        .then_ignore(just(" "))
        .ignore_then(variable(expression()))
        .then_ignore(just("="))
        .then(expression())
        .map(|(variable, value)| ir::commands::Command::Set(Set { variable, value }))
}

fn post_condition<'src>() -> impl Parser<'src, &'src str, Option<Expression>, Error<'src>> {
    just(":").ignore_then(expression()).or_not()
}

fn quit_parser<'src>() -> impl Parser<'src, &'src str, ir::commands::Command, Error<'src>> {
    keyword("quit")
        .ignore_then(post_condition())
        .then_ignore(argument_less())
        .map(|condition| {
            ir::commands::Command::Quit(PostCondition {
                condition,
                value: commands::Quit(None),
            })
        })
}
fn do_parser<'src>() -> impl Parser<'src, &'src str, ir::commands::Command, Error<'src>> {
    keyword("do")
        .ignore_then(post_condition())
        .then_ignore(argument_less())
        .map(|condition| {
            ir::commands::Command::Do(PostCondition {
                condition,
                value: commands::r#do::Do::ArgumentLess,
            })
        })
}

fn kill_parser<'src>() -> impl Parser<'src, &'src str, ir::commands::Command, Error<'src>> {
    use commands::kill::KillType as E;
    keyword("kill")
        .ignore_then(choice((
            just(" ").ignore_then(
                choice((
                    variable(expression()).map(|var| commands::kill::Kill {
                        r#type: E::Inclusive,
                        variables: vec![var],
                    }),
                    local_variable_no_subscripts()
                        .delimited_by(just("("), just(")"))
                        .map(|var| commands::kill::Kill {
                            r#type: E::Exclusive,
                            variables: vec![var],
                        }),
                ))
                .separated_by(just(","))
                .collect::<Vec<_>>(),
            ),
            argument_less().to(vec![commands::kill::Kill {
                r#type: E::Exclusive,
                variables: vec![],
            }]),
        )))
        .map(|value| ir::commands::Command::Kill(value))
}

fn argument_less<'src>() -> impl Parser<'src, &'src str, (), Error<'src>> {
    choice((just(" ").ignored(), empty().and_is(just("\n")).ignored()))
}

///WARN: Look at warning on `variable`
fn for_parser<'src>(
    cmd: impl Parser<'src, &'src str, Spanned<Command>, Error<'src>>,
) -> impl Parser<'src, &'src str, ir::commands::Command, Error<'src>> {
    let for_args = expression()
        .separated_by(just(":"))
        .at_least(1)
        .at_most(3)
        .collect();

    keyword("for")
        .then_ignore(just(" "))
        .ignore_then(choice((
            variable(expression())
                .then_ignore(just("="))
                .then(
                    for_args
                        .map(|args: Vec<_>| {
                            //
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
                        })
                        .separated_by(just(","))
                        .at_least(1)
                        .collect(),
                )
                .map(|(variable, arguments)| ForKind::VarLoop {
                    variable,
                    arguments,
                }),
            empty().to(ForKind::Infinite),
        )))
        .then_ignore(just(" "))
        .then(cmd.separated_by(just(" ")).collect())
        .map(|(kind, commands)| ir::commands::Command::For(For { kind, commands }))
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
