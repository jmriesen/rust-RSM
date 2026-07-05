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

    pub fn line_start_index(&self, line_number: usize) -> Option<usize> {
        std::iter::once(0)
            .chain(self.source.match_indices('\n').map(
                |(x, _)| x + 1, /*The +1 moves us to start of next line.*/
            ))
            .nth(line_number)
    }
    pub fn position_to_index(&self, position: Position) -> Option<usize> {
        self.line_start_index(position.line as usize)
            .map(|line_start| line_start + position.character as usize)
    }

    pub fn update(&mut self, changes: Vec<TextDocumentContentChangeEvent>) {
        for change in changes.iter() {
            let start = self
                .position_to_index(change.range.unwrap().start)
                .expect("Changed range must be pressent in the document");
            let end = self
                .position_to_index(change.range.unwrap().end)
                .expect("Changed range must be pressent in the document");

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
    pub fn text(&self) -> &str {
        &self.source
    }
}
#[cfg(test)]
mod test {
    use tower_lsp::lsp_types::{Position, Range, TextDocumentContentChangeEvent};

    use crate::document::Document;

    #[test]
    fn debug_update_sequencal() {
        let mut document = Document::new(
"tag w \"before loop\",!\n f i=1:1:5 w \"foo \"\n w !,\"after loop\"\n w \"foo\" \n w test,!,!\n q  \n s foo=te\n\n".to_owned()
            );
        document.update(vec![TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 6,
                    character: 9,
                },
                end: Position {
                    line: 6,
                    character: 9,
                },
            }),
            range_length: Some(0),
            text: "s".to_owned(),
        }]);
        document.update(vec![TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 6,
                    character: 10,
                },
                end: Position {
                    line: 6,
                    character: 10,
                },
            }),
            range_length: Some(0),
            text: "t".to_owned(),
        }]);
        assert_eq!(document.text(), "tag w \"before loop\",!\n f i=1:1:5 w \"foo \"\n w !,\"after loop\"\n w \"foo\" \n w test,!,!\n q  \n s foo=test\n\n")
    }
    #[test]
    fn bug_update_bach() {
        let mut document = Document::new(
"tag w \"before loop\",!\n f i=1:1:5 w \"foo \"\n w !,\"after loop\"\n w \"foo\" \n w test,!,!\n q  \n s foo=te\n\n".to_owned()
            );
        document.update(vec![
            TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 6,
                        character: 9,
                    },
                    end: Position {
                        line: 6,
                        character: 9,
                    },
                }),
                range_length: Some(0),
                text: "s".to_owned(),
            },
            TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 6,
                        character: 10,
                    },
                    end: Position {
                        line: 6,
                        character: 10,
                    },
                }),
                range_length: Some(0),
                text: "t".to_owned(),
            },
        ]);
        assert_eq!(document.text(), "tag w \"before loop\",!\n f i=1:1:5 w \"foo \"\n w !,\"after loop\"\n w \"foo\" \n w test,!,!\n q  \n s foo=test\n\n")
    }
}
