use lang_model::{commandChildren, BlockChildren};
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, Position, Range, TextDocumentContentChangeEvent,
};
use tree_sitter::{Query, QueryCursor, QueryMatches};

use crate::util::PointExt;
pub struct Document {
    ///Note the document and tree must always stay in sync.
    source: String,
    tree: tree_sitter::Tree,
}

impl Document {
    pub fn new(source: String) -> Self {
        Self {
            tree: lang_model::create_tree(&source),
            source,
        }
    }

    pub fn query<'a>(
        &'a self,
        query: &'a Query,
        query_cursor: &'a mut QueryCursor,
    ) -> QueryMatches<'a, 'a, &'a [u8]> {
        query_cursor.matches(query, self.tree.root_node(), self.source.as_bytes())
    }

    pub fn update(&mut self, mut changes: Vec<TextDocumentContentChangeEvent>) {
        let line_index: Vec<_> = std::iter::once(0)
            .chain(self.source.match_indices('\n').map(|(x, _)| x + 1))
            .collect();

        changes.sort_by_key(|x| {
            let pos = x
                .range
                .expect("LSP is configured for incremental changes & they always provide a range")
                .start;
            (pos.line, pos.character)
        });

        //Go from back to front prevents indexes from changing underneath us.
        //NOTE I have not tested with two concurrent changes
        for change in changes.iter().rev() {
            let get_index = |position: Position| {
                line_index[position.line as usize] + position.character as usize
            };
            let start = get_index(change.range.unwrap().start);
            let end = get_index(change.range.unwrap().end);

            self.source.replace_range(start..end, &change.text);
        }

        //TODO: tree sitter supports updating the tree based on edits.
        //If I figure out the api I could make this more efficient.
        self.tree = lang_model::create_tree(&self.source);
    }

    pub fn validate(&self) -> Vec<Diagnostic> {
        use tree_sitter::{Query, QueryCursor};
        let mut query_cursor = QueryCursor::new();
        let error_query = Query::new(tree_sitter_mumps::language(), "(ERROR)@error").unwrap();
        let expressions = self.query(&error_query, &mut query_cursor);

        expressions
            .map(|exp| {
                let node = exp.captures[0].node;
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
            })
            .collect()
    }
}
