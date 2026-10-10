use crate::parser::commands::set::set_parser;

use super::*;
pub mod set;

pub fn command<'src>() -> impl Parser<'src, &'src str, Spanned<Command>, Extra<'src>> {
    recursive(|cmd| {
        choice((
            write(),
            if_parser(),
            else_parser(),
            set_parser().map(Command::Set),
            for_parser(cmd),
            quit_parser(),
            kill_parser(),
            do_parser(),
            break_parser(),
            close_parser(),
            new(),
        ))
        .boxed()
        //.recover_with(via_parser(recover_parser().to(Command::Error)))
        .spanned()
        .map(|x| Spanned::from(x))
    })
}
