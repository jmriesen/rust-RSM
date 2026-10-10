use super::*;
pub mod set;

pub fn recover_parser<'src>() -> impl Parser<'src, &'src str, &'src str, Extra<'src>> + Clone {
    choice((
        //Consumes arbitrary text and treats is as a command.
        //We don't want to consume our delimiters
        none_of(" \n").repeated().at_least(1).to_slice(),
        //Handles detecting "extra spaces"
        //Rather than trying to consume the space I am just injecting an error command (without
        //consuming anything and letting the "extra" space be treated as a deliminator.
        peek(just(" ")).to_slice(),
    ))
}

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
        .recover_with(via_parser(recover_parser().to(Command::Error)))
        .spanned()
        .map(|x| Spanned::from(x))
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn unknown_command() {
        insta::assert_debug_snapshot!(command().parse("foo").into_output_errors());
    }
}
