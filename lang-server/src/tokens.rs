use tower_lsp::lsp_types::* ;

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

/// SemanticToken but position is mesure in absolute rather then reletive terms
pub struct AbsolutToken(SemanticToken);

impl From<TokenNode<'_>> for AbsolutToken{
    fn from(TokenNode(node): TokenNode) -> Self {
        let start = node.start_position();
        let end = node.end_position();
        AbsolutToken(
            SemanticToken {
                //NOTE Using absolutes position for now
                //will convert into deltas latter.
                delta_line: to_lsp_int(start.row),
                delta_start: to_lsp_int(start.column),
                length: to_lsp_int(end.column - start.column),
                token_type: TokenTypes::from_node_type(node.kind()) as u32,
                token_modifiers_bitset: 0,
            })
    }
}
impl AbsolutToken{
    pub fn to_relitive(tokens:Vec<Self>)->Vec<SemanticToken>{
        // Stringing off the wrapper type
        let mut tokens :Vec<_>= tokens.iter().map(|x| x.0).collect();
        tokens.sort_by_key(|x| (x.delta_line, x.delta_start));
        // Inserting starting values.
        tokens.insert(
            0,
                SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 0,
                    token_type: TokenTypes::Other as u32,
                    token_modifiers_bitset: 0,
                },
        );
        tokens
            .array_windows()
            .map(|[previuse, current]| {
                //NOTE: converting from absolute POS to deltas.
                let mut current = *current;
                current.delta_line -= previuse.delta_line;
                if current.delta_line == 0 {
                    current.delta_start -= previuse.delta_start;
                }
                current
            })
            .collect()
    }
}

