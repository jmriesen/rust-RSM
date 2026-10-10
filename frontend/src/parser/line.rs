use chumsky::{IterParser, prelude::*};
use ir::{Line, Spanned, Tag};

use crate::parser::{Extra, commands::command};
pub fn line_parser<'src>() -> impl Parser<'src, &'src str, Line, Extra<'src>> {
    let tag = text::ascii::ident()
        .map(|tag: &str| Tag {
            name: tag.to_owned(),
        })
        .spanned()
        .map(|x| Spanned::from(x));
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
