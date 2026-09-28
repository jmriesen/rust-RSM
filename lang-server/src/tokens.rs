use std::sync::LazyLock;

use derive_visitor::{Drive, Visitor};
use ir::{
    commands::{Command, Write},
    expression::{NumberLiteral, StringLiteral},
    Spanned, Tag, Variable,
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
    {Bang,         "Bang",         SemanticTokenType::KEYWORD}
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
            let mut tokens = TokensVisitor::default();
            for line in routine {
                line.drive(&mut tokens);
            }

            tokens
                .tokens
                .into_iter()
                .map(|x| AbsolutToken {
                    start_position: index_to_position(x.span.start),
                    length: (x.span.end - x.span.start) as u32,
                    token_type: x.inner as u32,
                    token_modifiers_bitset: 0,
                })
                .collect()
        } else {
            vec![]
        }
    }
}

use chumsky::span::SimpleSpan;

type SpanW = Spanned<Write>;
type SpanT = Spanned<Tag>;
type SpanC = Spanned<Command>;
type SpanN = Spanned<NumberLiteral>;
type SpanS = Spanned<StringLiteral>;
type SpanV = Spanned<Variable>;
#[derive(Visitor, Default)]
#[visitor(
    SpanW,
    SpanT,
    SpanC,
    SpanN,
    SpanS,
    SpanV,
    Tag,
    Write,
    Command,
    NumberLiteral,
    StringLiteral,
    Variable
)]
struct TokensVisitor {
    spans: Vec<SimpleSpan>,
    tokens: Vec<Spanned<TokenTypes>>,
}
use pastey::paste;
macro_rules! enter_exit_span {
    ($type:ident) => {
        paste! {

        fn [<enter_$type:snake>](&mut self, span: &$type) {
            self.enter_span(span);
        }
        fn [<exit_$type:snake>](&mut self, _span: &$type) {
            self.exit_span();
        }
        }
    };
}

impl TokensVisitor {
    fn exit_write(&mut self, _: &Write) {}
    fn enter_write(&mut self, write: &Write) {
        match write {
            Write::Bang => self.create_token(TokenTypes::Bang),
            Write::Clear => self.create_token(TokenTypes::Bang),
            Write::Tab(_) => self.create_token(TokenTypes::Bang),
            Write::Expression(_) => {}
        }
    }

    fn exit_tag(&mut self, _: &Tag) {}
    fn enter_tag(&mut self, _: &Tag) {
        self.create_token(TokenTypes::TagName);
    }
    fn exit_number_literal(&mut self, _: &NumberLiteral) {}
    fn enter_number_literal(&mut self, _: &NumberLiteral) {
        self.create_token(TokenTypes::Number);
    }
    fn exit_string_literal(&mut self, _: &StringLiteral) {}
    fn enter_string_literal(&mut self, _: &StringLiteral) {
        self.create_token(TokenTypes::String);
    }

    fn exit_command(&mut self, _: &Command) {}
    fn enter_command(&mut self, _: &Command) {
        self.create_token(TokenTypes::Command);
    }

    fn exit_variable(&mut self, _: &Variable) {}
    fn enter_variable(&mut self, _: &Variable) {
        self.create_token(TokenTypes::Variable);
    }

    enter_exit_span!(SpanW);
    enter_exit_span!(SpanT);
    enter_exit_span!(SpanC);
    enter_exit_span!(SpanN);
    enter_exit_span!(SpanS);
    enter_exit_span!(SpanV);

    fn enter_span<T>(&mut self, spanned: &Spanned<T>) {
        self.spans.push(spanned.span);
    }
    fn exit_span(&mut self) {
        self.spans.pop();
    }
    fn create_token(&mut self, token: TokenTypes) {
        let token = Spanned {
            inner: token,
            span: self.spans.last().unwrap().clone(),
        };
        self.tokens.push(token);
    }
}
