use chumsky::{IterParser, combinator::AndIs, prelude::*, primitive::Empty};
use ir::{
    Expression::{self},
    ExtrinsicFunction, Line, Routine, Spanned, Tag,
    commands::{
        Command, New, NewKind, PostCondition, Quit,
        Write::{self},
        r#break::Break,
        close::Close,
        r#do::Do,
        r#for::{Argument, For, ForKind},
        r#if::If,
        kill,
    },
    extrinsic_function::{
        Args,
        Location::{self},
    },
    variable::Ident,
};
mod commands;
mod expression;
mod line;
mod separated_by;
mod variable;
use expression::expression;

pub type Extra<'src> = chumsky::extra::Err<Rich<'src, char, SimpleSpan, ParsingError>>;

use crate::{
    ParsingError,
    parser::{
        line::line_parser,
        variable::{identifier, local_variable_no_subscripts, variable},
    },
};

pub fn peek<'src, U, B>(item: B) -> AndIs<Empty<&'src str, Extra<'src>>, B, U>
where
    B: Parser<'src, &'src str, U, Extra<'src>>,
{
    empty().and_is(item)
}

pub fn keyword<'src>(keyword: &'static str) -> impl Parser<'src, &'src str, (), Extra<'src>> {
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
    term: impl Parser<'src, &'src str, T, Extra<'src>>,
) -> impl Parser<'src, &'src str, Vec<T>, Extra<'src>> {
    term.separated_by(just(","))
        .collect::<Vec<_>>()
        .delimited_by(just("("), just(")"))
}

pub fn routine<'src>() -> impl Parser<'src, &'src str, Routine, Extra<'src>> {
    line_parser()
        .repeated()
        .collect::<Vec<_>>()
        .then_ignore(just("\n").repeated())
        //Ignore the trailing metadata in the test files.
        .then_ignore(just("---").then(any().repeated()).or_not())
}

fn space_or_eol<'src>() -> impl Parser<'src, &'src str, (), Extra<'src>> {
    choice((just(" ").ignored(), peek(just("\n")).ignored(), end()))
}

fn eol<'src>() -> impl Parser<'src, &'src str, (), Extra<'src>> {
    choice((just("\n").ignored(), end()))
}

/// Command with a optional post condition
fn cmd<'src>(name: &'static str) -> impl Parser<'src, &'src str, (), Extra<'src>> {
    keyword(name).then_ignore(space_or_eol())
}
fn post_condition<'src>() -> impl Parser<'src, &'src str, Option<Expression>, Extra<'src>> {
    just(":").ignore_then(expression()).or_not()
}
/// Command with a optional post condition
fn cmd_p<'src>(
    name: &'static str,
) -> impl Parser<'src, &'src str, Option<Expression>, Extra<'src>> {
    keyword(name)
        .ignore_then(post_condition())
        .then_ignore(space_or_eol())
}

fn write<'src>() -> impl Parser<'src, &'src str, Command, Extra<'src>> {
    cmd_p("write")
        .then(
            write_arg()
                .separated_by(just(","))
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .map(|(condition, value)| Command::Write(PostCondition { condition, value }))
}
fn if_parser<'src>() -> impl Parser<'src, &'src str, Command, Extra<'src>> {
    cmd("if")
        .ignore_then(
            expression()
                .map(If)
                .separated_by(just(","))
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .map(|x| Command::If(x))
}
fn else_parser<'src>() -> impl Parser<'src, &'src str, Command, Extra<'src>> {
    cmd("else").map(|_| Command::Else)
}

fn write_arg<'src>() -> impl Parser<'src, &'src str, Spanned<Write>, Extra<'src>> {
    choice((
        just("!").to(Write::Bang),
        just("#").to(Write::Clear),
        just("?").ignore_then(expression()).map(|x| Write::Tab(x)),
        expression().map(|x| Write::Expression(x)),
    ))
    .spanned()
    .map(|x| Spanned::from(x))
}

fn quit_parser<'src>() -> impl Parser<'src, &'src str, Command, Extra<'src>> {
    cmd_p("quit")
        .then(
            expression()
                .separated_by(just(","))
                .collect::<Vec<_>>()
                //Validation logic
                .validate(|mut return_value, extra, emiter| {
                    if return_value.len() > 1 {
                        emiter.emit(Rich::custom(extra.span(), ParsingError::QuitExtraArgs));
                    }
                    if return_value.len() == 1 {
                        emiter.emit(Rich::custom(
                            extra.span(),
                            ParsingError::NotYetSupported("quit with args"),
                        ));
                    }
                    Quit(return_value.pop())
                }),
        )
        .map(|(condition, value)| Command::Quit(PostCondition { condition, value }))
}

fn do_parser<'src>() -> impl Parser<'src, &'src str, Command, Extra<'src>> {
    cmd_p("do")
        .then(
            (parse_extrinsic_function(expression().boxed())
                .then(post_condition())
                .map(|(value, condition)| PostCondition { condition, value })
                .separated_by(just(","))
                .collect::<Vec<_>>())
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
    exp: impl Parser<'src, &'src str, Expression, Extra<'src>> + Clone,
) -> impl Parser<'src, &'src str, ExtrinsicFunction, Extra<'src>> {
    choice((
        identifier()
            .then_ignore(just("^"))
            .then(identifier())
            .map(|(tag, routine)| {
                Location::TagRoutine(Ident(tag.to_owned()), Ident(routine.to_owned()))
            }),
        just("^")
            .ignore_then(identifier())
            .map(|x| Location::Routine(Ident(x.to_owned()))),
        identifier().map(|x| Location::Tag(Ident(x.to_owned()))),
    ))
    .then(function_args(exp).or_not())
    .map(|(location, args)| ExtrinsicFunction {
        location,
        arguments: args.unwrap_or_default(),
    })
}
fn function_args<'src>(
    exp: impl Parser<'src, &'src str, Expression, Extra<'src>> + Clone,
) -> impl Parser<'src, &'src str, Vec<Args>, Extra<'src>> {
    args_list(choice((
        exp.clone().map(Args::Expression),
        just(".")
            .ignore_then(local_variable_no_subscripts(exp))
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

fn kill_parser<'src>() -> impl Parser<'src, &'src str, Command, Extra<'src>> {
    use kill::{Kill, KillType as E};
    cmd("kill")
        .ignore_then(
            choice((
                variable(expression().boxed()).map(|var| Kill {
                    r#type: E::Inclusive,
                    variables: vec![var],
                }),
                local_variable_no_subscripts(expression().boxed())
                    .map(|var| Kill {
                        r#type: E::Exclusive,
                        variables: vec![var],
                    })
                    .delimited_by(just("("), just(")")),
            ))
            .separated_by(just(","))
            .collect::<Vec<_>>()
            .map(|x| {
                if x.is_empty() {
                    vec![Kill {
                        r#type: E::Exclusive,
                        variables: vec![],
                    }]
                } else {
                    x
                }
            }),
        )
        .map(|value| Command::Kill(value))
}

fn break_parser<'src>() -> impl Parser<'src, &'src str, Command, Extra<'src>> {
    cmd_p("brake")
        .then(
            (expression().separated_by(just(",")).collect::<Vec<_>>()).map(|x| {
                if x.is_empty() {
                    Break::ArgumentLess
                } else {
                    Break::Arg(x)
                }
            }),
        )
        .map(|(condition, value)| Command::Break(PostCondition { condition, value }))
}
fn close_parser<'src>() -> impl Parser<'src, &'src str, Command, Extra<'src>> {
    cmd_p("close")
        .then(
            (expression()
                .separated_by(just(","))
                .at_least(1)
                .collect::<Vec<_>>())
            .map(|x| x.into_iter().map(Close).collect()),
        )
        .map(|(condition, value)| Command::Close(PostCondition { condition, value }))
}
fn new<'src>() -> impl Parser<'src, &'src str, Command, Extra<'src>> {
    let vars = || {
        local_variable_no_subscripts(expression().boxed())
            .separated_by(just(","))
            .at_least(1)
            .collect::<Vec<_>>()
    };

    cmd("new")
        .ignore_then(
            choice((
                vars().map(|x| (x, NewKind::Inclusive)),
                vars()
                    .delimited_by(just("("), just(")"))
                    .map(|x| (x, NewKind::Exclusive)),
                empty().to((vec![], NewKind::Exclusive)),
            ))
            .map(|(vars, typ)| New { kind: typ, vars }),
        )
        .map(Command::New)
}

///WARN: Look at warning on `variable`
fn for_parser<'src>(
    cmd_parser: impl Parser<'src, &'src str, Spanned<Command>, Extra<'src>>,
) -> impl Parser<'src, &'src str, Command, Extra<'src>> {
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

    cmd("for")
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
        .then(cmd_parser.separated_by(just(" ")).collect())
        .map(|(kind, commands)| Command::For(For { kind, commands }))
}

#[cfg(test)]
mod test {

    use chumsky::Parser;

    use super::write;

    #[test]
    fn write_command() {
        insta::assert_debug_snapshot!(write().parse("w !,#,\"test\"").into_output_errors());
    }
}
