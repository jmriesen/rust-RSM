use std::{result::IterMut, sync::LazyLock};

use ir::{
    commands::{Command, Write},
    Line, Routine, Spanned, Tag,
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

use crate::util::to_lsp_int;
//NOTE: I am using a macro to define this type so the order of items always stays in sync.
//The reference ordering must mach the variant ordering for the client/server to understand
//each other.
macro_rules! tokens {
    ($( {$name:ident, $str_rep:expr, $semantic:expr})*) => {
        #[repr(u32)]
        pub enum TokenTypes {
            $( $name, )*
            Other,
        }

        impl TokenTypes {
            pub fn from_node_type(node_kind: &str) -> Self {
                match node_kind {
                    $( $str_rep => Self::$name, )*
                    _ => Self::Other,
                }
            }

            pub fn reference_ordering() -> Vec<SemanticTokenType> {
                vec![
                    $( $semantic, )*
                    SemanticTokenType::KEYWORD
                ]
            }
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
pub struct TokenNode<'a>(pub tree_sitter::Node<'a>);

/// `SemanticToken` but position is measure in absolute rather than relative terms
#[derive(Clone, Copy, Debug)]
pub struct AbsolutToken {
    pub start_position: Position,
    pub length: u32,
    pub token_type: u32,
    pub token_modifiers_bitset: u32,
}

impl From<&TokenNode<'_>> for AbsolutToken {
    fn from(TokenNode(node): &TokenNode) -> Self {
        let start = node.start_position();
        AbsolutToken {
            start_position: Position {
                line: to_lsp_int(start.row),
                character: to_lsp_int(start.column),
            },
            length: to_lsp_int(node.end_byte() - node.start_byte()),
            token_type: TokenTypes::from_node_type(node.kind()) as u32,
            token_modifiers_bitset: 0,
        }
    }
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
    use std::fs;

    use insta::assert_debug_snapshot;
    use tower_lsp::{
        lsp_types::{
            ClientCapabilities, InitializeParams, SemanticTokensClientCapabilities,
            TextDocumentClientCapabilities, TextDocumentIdentifier, Url,
        },
        LanguageServer,
    };

    use crate::{partial, test_url, MumpsLsp};
    #[tokio::test]
    async fn overlapping() {
        let uri: Url = test_url!();
        let source = fs::read_to_string("../backend/tests/for/for_each.test")
            .unwrap()
            .split_once("\n---\n")
            .unwrap()
            .0
            .to_owned();

        let lsp = MumpsLsp::new(());
        lsp.initialize(partial!(InitializeParams {
            capabilities: ClientCapabilities {
                text_document: Some(TextDocumentClientCapabilities {
                    semantic_tokens: Some(SemanticTokensClientCapabilities {
                        overlapping_token_support: Some(true)
                    })
                })
            }
        }))
        .await
        .unwrap();
        lsp.did_open(uri.clone(), source);
        assert_debug_snapshot!(lsp.tokens(&TextDocumentIdentifier::new(uri)));
    }
    #[tokio::test]
    async fn non_overlapping() {
        let uri: Url = test_url!();
        let source = fs::read_to_string("../backend/tests/for/for_each.test")
            .unwrap()
            .split_once("\n---\n")
            .unwrap()
            .0
            .to_owned();

        let lsp = MumpsLsp::new(());
        lsp.initialize(partial!(InitializeParams {
            capabilities: ClientCapabilities {
                text_document: Some(TextDocumentClientCapabilities {
                    semantic_tokens: Some(SemanticTokensClientCapabilities {
                        overlapping_token_support: Some(false)
                    })
                })
            }
        }))
        .await
        .unwrap();
        lsp.did_open(uri.clone(), source);
        assert_debug_snapshot!(lsp.tokens(&TextDocumentIdentifier::new(uri)));
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
        core::iter::once(Spanned {
            inner: TokenTypes::Command,
            start: self.start,
            end: self.end,
        })
        .chain(match &self.inner {
            Command::Write(post_condition) => Box::new(post_condition.value.tokens())
                as Box<dyn Iterator<Item = Spanned<TokenTypes>>>,

            Command::Error => Box::new(core::iter::once(Spanned {
                inner: TokenTypes::String,
                start: self.start,
                end: self.end,
            })),
            _ => Box::new(Option::<Spanned<Tag>>::None.tokens()),
        })
    }
}

impl ExtractTokens for Spanned<Tag> {
    fn tokens(&self) -> impl Iterator<Item = Spanned<TokenTypes>> {
        core::iter::once(Spanned {
            inner: TokenTypes::TagName,
            start: self.start,
            end: self.end,
        })
    }
}
impl ExtractTokens for Spanned<Write> {
    fn tokens(&self) -> impl Iterator<Item = Spanned<TokenTypes>> {
        match &self.inner {
            Write::Bang | Write::Clear => Box::new(core::iter::once(Spanned {
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
