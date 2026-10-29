use chumsky::{IterParser, prelude::*};
use ir::{Line, Spanned, Tag, variable::Ident};

use crate::{
    ParsingError,
    parser::{Extra, commands::command, eol, space_or_eol, variable::identifier},
};
pub fn line_parser<'src>() -> impl Parser<'src, &'src str, Line, Extra<'src>> {
    let tag = identifier()
        .then(
            (identifier()
                .map(|ident| Ident(ident.to_owned()))
                .separated_by(just(",")))
            .collect()
            .delimited_by(just("("), just(")"))
            .or_not(),
        )
        .map(|(tag, parameters)| Tag {
            name: tag.to_owned(),
            parameters: parameters,
        })
        .spanned()
        .map(|x| Spanned::from(x));

    let line_level = just(".").repeated().count().map(|level| level as u16);

    let commands = command()
        //Try to find next command
        .recover_with(skip_then_retry_until(
            any().and_is(eol().not()).ignored(),
            eol(),
        ))
        .repeated()
        .collect()
        .then_ignore(space_or_eol().or_not());

    (tag.or_not()
        .then(
            just(" ")
                .ignore_then(line_level)
                .then(commands)
                .or_not()
                .map(|level_commands| level_commands.unwrap_or((0, vec![]))),
        )
        //If there is anything left we don't understand (probably a bad command skip it and move to
        //end of line)
        .then_ignore(eol().recover_with(skip_then_retry_until(any().ignored(), eol())))
        // Explicitly disallowing matching to just end
        // Without this the routine parser can get into an infinite loop
        .and_is(end().not())
        .map(|(tag, (level, commands))| Line {
            tag,
            level,
            commands,
        }))
    .validate(|line, extra, emitter| {
        if line.tag.is_some() && line.level != 0 {
            emitter.emit(Rich::custom(
                extra.span(),
                ParsingError::TagsLineLevelMustBeZero,
            ));
        }
        line
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tag_no_commands() {
        insta::assert_debug_snapshot!(line_parser().parse("foo").into_output_errors());
    }

    #[test]
    fn tag_and_commands() {
        insta::assert_debug_snapshot!(line_parser().parse("foo w !").into_output_errors());
    }

    #[test]
    fn line_levels_and_command() {
        insta::assert_debug_snapshot!(line_parser().parse(" ...w !").into_output_errors());
    }

    #[test]
    fn tag_and_line_level_not_allowed() {
        //Note currently parsing as an error
        insta::assert_debug_snapshot!(line_parser().parse("foo ...w !").into_output_errors());
    }

    #[test]
    fn recover_from_extra_space() {
        //Note currently parsing as an error
        insta::assert_debug_snapshot!(line_parser().parse(" w !  w !").into_output_errors());
    }
    #[test]
    fn recover_by_skipping_to_end_of_line() {
        //Note currently parsing as an error
        insta::assert_debug_snapshot!(
            line_parser()
                //                -><- write is missing and argument
                .parse(" set i=1 w !, ")
                .into_output_errors()
        );
    }
    #[test]
    fn recover_from_bad_command_by_looking_for_next_good_one() {
        insta::assert_debug_snapshot!(
            line_parser()
                //                -><- write is missing and argument
                .parse(" set i=1 w !, s i=1")
                .into_output_errors()
        );
    }
}
