use std::sync::LazyLock;

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity,Range};
use tree_sitter::Query;

use crate::util::PointExt;

pub const ERROR_QUERY : LazyLock<Query> = LazyLock::new(||{
    Query::new(tree_sitter_mumps::language(), "(ERROR)@error").unwrap()
});
pub struct ErrorNode<'a>(pub tree_sitter::Node<'a>);

impl From<ErrorNode<'_>> for Diagnostic{
    fn from(ErrorNode(node): ErrorNode<'_>) -> Self {
        Diagnostic {
            code_description: None,
            code: None,
            message: node.to_sexp(),
            source: None,
            tags: None,
            data: None,
            related_information: None,
            severity: Some(DiagnosticSeverity::ERROR),
            range: Range {
                start: node.start_position().to_position(),
                end: node.end_position().to_position(),
            },
        }
    }
}

