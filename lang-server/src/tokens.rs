use core::iter::once;
use std::sync::LazyLock;

use ir::{
    commands::{Command, Write},
    Line, Spanned, Tag,
};
use tower_lsp::lsp_types::{
    Position, SemanticToken, SemanticTokenType, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, SemanticTokensServerCapabilities,
};
pub static SEMANTIC_TOKENS_CAPABILITIES: LazyLock<Option<SemanticTokensServerCapabilities>> =
    LazyLock::new(|| {
        Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                full: Some(SemanticTokensFullOptions::Bool(true)),
                legend: SemanticTokensLegend {
                    token_types: TokenTypes::reference_ordering(),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
    });

//NOTE: I am using a macro to define this type so the order of items always stays in sync.
//The reference ordering must mach the variant ordering for the client/server to understand
//each other.
// Old Tree-sitter query.
// Eventually want to get to feature parity again.

macro_rules! tokens {
    ($( {$name:ident, $str_rep:expr, $semantic:expr})*) => {
        #[repr(u32)]
        pub enum TokenTypes {
            $( $name, )*
            Other,
        }

        impl TokenTypes {
/*
            pub fn from_node_type(node_kind: &str) -> Self {
                match node_kind {
                    $( $str_rep => Self::$name, )*
                    _ => Self::Other,
                }
            }
*/

            pub fn reference_ordering() -> Vec<SemanticTokenType> {
                vec![
                    $( $semantic, )*
                    SemanticTokenType::KEYWORD
                ]
            }
/*
            pub fn query()->tree_sitter::Query{
            tree_sitter::Query::new(
                &tree_sitter_mumps::language(),
                concat!(
                    "[",
                        $( "(",$str_rep, ") ",)*
                    "]@token",
                    )
                )
                .unwrap()
            }
*/
        }

    };
}

tokens! {
    {Number,       "number",       SemanticTokenType::NUMBER}
    {String,       "string",       SemanticTokenType::STRING}
    {Variable,     "Variable",     SemanticTokenType::VARIABLE}
    {TagName,      "TagName",      SemanticTokenType::METHOD}
    {Command,      "command",      SemanticTokenType::KEYWORD}
    {Bang,         "Bang",         SemanticTokenType::OPERATOR}
    {BinOp,        "BinaryOpp",    SemanticTokenType::OPERATOR}
    {UnaryOpp,     "UnaryOpp",     SemanticTokenType::OPERATOR}
}

/// Wrapper around a Node that is known to correspond to a Token

/// `SemanticToken` but position is measure in absolute rather than relative terms
#[derive(Clone, Copy, Debug)]
pub struct AbsolutToken {
    pub start_position: Position,
    pub length: u32,
    pub token_type: u32,
    pub token_modifiers_bitset: u32,
}

impl AbsolutToken {
    pub fn to_relitive(mut tokens: Vec<Self>) -> Vec<SemanticToken> {
        // Tokens need to be in order for diff calculation.
        tokens.sort_by_key(|x| (x.start_position.line, x.start_position.character));
        // Inserting starting values.
        tokens.insert(
            0,
            AbsolutToken {
                start_position: Position {
                    line: 0,
                    character: 0,
                },
                length: 0,
                token_type: TokenTypes::Other as u32,
                token_modifiers_bitset: 0,
            },
        );
        tokens
            .array_windows()
            .map(|[previuse, current]| {
                SemanticToken {
                    delta_line: current.start_position.line - previuse.start_position.line,
                    delta_start: if current.start_position.line == previuse.start_position.line {
                        //Otherwise, calculate the diff.
                        current.start_position.character - previuse.start_position.character
                    } else {
                        //If starting a newline just use the current column.
                        current.start_position.character
                    },
                    length: current.length,
                    token_type: current.token_type,
                    token_modifiers_bitset: current.token_modifiers_bitset,
                }
            })
            .collect()
    }
}

pub fn remove_over_lapping(mut tokens: Vec<SemanticToken>) -> Vec<SemanticToken> {
    for i in 1..tokens.len() {
        // If token is to long clip it.
        // Only needed if overlapping tokes are not supported.
        if tokens[i].delta_line == 0 && tokens[i - 1].length > tokens[i].delta_start {
            tokens[i - 1].length = tokens[i].delta_start;
        }
    }
    tokens
}

#[cfg(test)]
mod test {
    use crate::tokens::remove_over_lapping;
    use tower_lsp::lsp_types::SemanticToken;

    #[test]
    fn de_overlap_tokens() {
        let overlapping = vec![
            SemanticToken {
                delta_line: 0,
                delta_start: 0,
                length: 5,
                token_type: 0,
                token_modifiers_bitset: 0,
            },
            //Not adjacent, not overlapping.
            SemanticToken {
                delta_line: 0,
                delta_start: 5,
                length: 10,
                token_type: 0,
                token_modifiers_bitset: 0,
            },
            //Overlapping.
            SemanticToken {
                delta_line: 0,
                delta_start: 5,
                length: 10,
                token_type: 0,
                token_modifiers_bitset: 0,
            },
            //On newline (never overlapping)
            //Mumps tokens should not overlap
            SemanticToken {
                delta_line: 1,
                delta_start: 10,
                length: 10,
                token_type: 0,
                token_modifiers_bitset: 0,
            },
        ];
        assert_eq!(
            remove_over_lapping(overlapping),
            vec![
                SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 5,
                    token_type: 0,
                    token_modifiers_bitset: 0,
                },
                //Not adjacent, not overlapping.
                SemanticToken {
                    delta_line: 0,
                    delta_start: 5,
                    // truncated to accommodate next token
                    length: 5,
                    token_type: 0,
                    token_modifiers_bitset: 0,
                },
                //Overlapping.
                SemanticToken {
                    delta_line: 0,
                    delta_start: 5,
                    length: 10,
                    token_type: 0,
                    token_modifiers_bitset: 0,
                },
                //On newline (never overlapping)
                //Mumps tokens should not overlap
                SemanticToken {
                    delta_line: 1,
                    delta_start: 10,
                    length: 10,
                    token_type: 0,
                    token_modifiers_bitset: 0,
                },
            ]
        );
    }
}
impl crate::Document {
    pub fn tokens(&self) -> Vec<AbsolutToken> {
        if let Some(routine) = self.ir().clone().into_output() {
            let index_to_position = &self.index_converter();
            routine
                .tokens()
                .into_iter()
                .map(|x| AbsolutToken {
                    start_position: index_to_position(x.start),
                    length: (x.end - x.start) as u32,
                    token_type: x.inner as u32,
                    token_modifiers_bitset: 0,
                })
                .collect()
        } else {
            vec![]
        }
    }
}

trait ExtractTokens {
    fn tokens(&self) -> impl Iterator<Item = Spanned<TokenTypes>>;
}

impl ExtractTokens for Line {
    fn tokens(&self) -> impl Iterator<Item = Spanned<TokenTypes>> {
        self.tag.tokens().chain(self.commands.tokens())
    }
}
impl ExtractTokens for Spanned<Command> {
    fn tokens(&self) -> impl Iterator<Item = Spanned<TokenTypes>> {
        once(Spanned {
            inner: TokenTypes::Command,
            start: self.start,
            end: self.end,
        })
        .chain(match &self.inner {
            Command::Write(post_condition) => Box::new(post_condition.value.tokens())
                as Box<dyn Iterator<Item = Spanned<TokenTypes>>>,

            Command::Error => Box::new(once(Spanned {
                inner: TokenTypes::String,
                start: self.start,
                end: self.end,
            })),
            Command::For(for_cmd) => Box::new(for_cmd.commands.tokens()),
            _ => Box::new(Option::<Spanned<Tag>>::None.tokens()),
        })
    }
}

impl ExtractTokens for Spanned<Tag> {
    fn tokens(&self) -> impl Iterator<Item = Spanned<TokenTypes>> {
        once(Spanned {
            inner: TokenTypes::TagName,
            start: self.start,
            end: self.end,
        })
    }
}
impl ExtractTokens for Spanned<Write> {
    fn tokens(&self) -> impl Iterator<Item = Spanned<TokenTypes>> {
        match &self.inner {
            Write::Bang | Write::Clear => Box::new(once(Spanned {
                inner: TokenTypes::Variable,
                start: self.start,
                end: self.end,
            }))
                as Box<dyn Iterator<Item = Spanned<TokenTypes>>>,
            _ => Box::new(Option::<Spanned<Tag>>::None.tokens()),
        }
    }
}

impl<E: ExtractTokens> ExtractTokens for Option<E> {
    fn tokens(&self) -> impl Iterator<Item = Spanned<TokenTypes>> {
        self.as_ref().map(|x| x.tokens()).into_iter().flatten()
    }
}
impl<E: ExtractTokens> ExtractTokens for Vec<E> {
    fn tokens(&self) -> impl Iterator<Item = Spanned<TokenTypes>> {
        self.iter().map(|x| x.tokens()).flatten()
    }
}
