use chumsky::{IterParser, prelude::*};
use ir::{Line, Spanned, Tag, commands::Command};

use super::separated_by::separated_by_with_recovery;
use crate::{
    ParsingError,
    parser::{Extra, commands::command},
};
pub fn line_parser<'src>() -> impl Parser<'src, &'src str, Line, Extra<'src>> {
    let tag = text::ascii::ident()
        .map(|tag: &str| Tag {
            name: tag.to_owned(),
        })
        .spanned()
        .map(|x| Spanned::from(x));

    let line_level = just(".").repeated().count().map(|level| level as u16);
    let commands = separated_by_with_recovery(command(), just(" ").ignored(), |span| Spanned {
        span: span.into(),
        inner: Command::Error,
    });

    tag.or_not()
        .then(
            just(" ")
                .ignore_then(line_level)
                .then(commands)
                .or_not()
                .map(|level_commands| level_commands.unwrap_or((0, vec![]))),
        )
        .map(|(tag, (level, commands))| Line {
            tag,
            level,
            commands,
        })
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
}
