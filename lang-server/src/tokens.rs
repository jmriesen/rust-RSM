use std::sync::LazyLock;

use tower_lsp::lsp_types::*;
pub const SEMANTIC_TOKENS_CAPABILITIES: LazyLock<Option<SemanticTokensServerCapabilities>> =
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
                tree_sitter_mumps::language(),
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

/// SemanticToken but position is measure in absolute rather than relative terms
pub struct AbsolutToken {
    pub line: u32,
    pub column: u32,
    pub length: u32,
    pub token_type: u32,
    pub token_modifiers_bitset: u32,
}

impl From<TokenNode<'_>> for AbsolutToken {
    fn from(TokenNode(node): TokenNode) -> Self {
        let start = node.start_position();
        let end = node.end_position();
        AbsolutToken {
            line: to_lsp_int(start.row),
            column: to_lsp_int(start.column),
            length: to_lsp_int(end.column - start.column),
            token_type: TokenTypes::from_node_type(node.kind()) as u32,
            token_modifiers_bitset: 0,
        }
    }
}
impl AbsolutToken {
    pub fn to_relitive(mut tokens: Vec<Self>) -> Vec<SemanticToken> {
        // Tokens need to be in order for diff calculation.
        tokens.sort_by_key(|x| (x.line, x.column));
        // Inserting starting values.
        tokens.insert(
            0,
            AbsolutToken {
                line: 0,
                column: 0,
                length: 0,
                token_type: TokenTypes::Other as u32,
                token_modifiers_bitset: 0,
            },
        );
        tokens
            .array_windows()
            .map(|[previuse, current]| {
                SemanticToken {
                    delta_line: current.line - previuse.line,
                    delta_start: if current.line != previuse.line {
                        //If starting a newline just use the current column.
                        current.column
                    } else {
                        //Otherwise, calculate the diff.
                        current.column - previuse.column
                    },
                    length: current.length,
                    token_type: current.token_type,
                    token_modifiers_bitset: current.token_modifiers_bitset,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use insta::assert_debug_snapshot;
    use tower_lsp::{
        lsp_types::{InitializeParams, TextDocumentIdentifier, Url},
        LanguageServer,
    };

    use crate::{test_url, MumpsLsp};
    #[tokio::test]
    async fn test_tokenazation() {
        let uri: Url = test_url!();
        let source = fs::read_to_string("../backend/tests/for/for_each.test")
            .unwrap()
            .split_once("\n---\n")
            .unwrap()
            .0
            .to_owned();

        let lsp = MumpsLsp::new(());
        lsp.initialize(InitializeParams::default()).await.unwrap();
        lsp.did_open(uri.clone(), source);
        assert_debug_snapshot!(lsp.tokens(TextDocumentIdentifier::new(uri)));
    }
}
