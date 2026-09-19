use chumsky::{error::EmptyErr, ParseResult, Parser};
use frontend::parser::routine;
use ir::{Routine, Spanned};
use tower_lsp::lsp_types::{
    Position, TextDocumentContentChangeEvent, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tree_sitter::{Query, QueryCursor, QueryMatches};

use crate::{tokens::AbsolutToken, TokenTypes};
pub const DOCUMENT_SYNC_CAPABILITY: Option<TextDocumentSyncCapability> = Some(
    TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL),
);

pub struct Document {
    ///Note the document and tree must always stay in sync.
    source: String,
    ir: ParseResult<Routine, EmptyErr>,
}

impl Document {
    pub fn new(source: String) -> Self {
        let ir = routine().parse(&source);
        Self { source, ir }
    }

    /*
    pub fn query<'a, 'query>(
        &'a self,
        query: &'query Query,
        query_cursor: &'a mut QueryCursor,
    ) -> QueryMatches<'query, 'a, &'a [u8], &'a [u8]> {
        query_cursor.matches(query, self.tree.root_node(), self.source.as_bytes())
    }
    */

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

    pub fn update(&mut self, changes: &[TextDocumentContentChangeEvent]) {
        for change in changes {
            let start = self
                .position_to_index(change.range.unwrap().start)
                .expect("Changed range must be present in the document");
            let end = self
                .position_to_index(change.range.unwrap().end)
                .expect("Changed range must be present in the document");

            self.source.replace_range(start..end, &change.text);
        }

        self.ir = routine().parse(&self.source);
    }

    pub fn text(&self) -> &str {
        &self.source
    }
    pub fn ir(&self) -> &ParseResult<Routine, EmptyErr> {
        &self.ir
    }
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use tower_lsp::lsp_types::{Position, Range, TextDocumentContentChangeEvent};

    use crate::document::Document;

    const DOC_BEFORE_EDIT: &str = "tag w \"before loop\",!\n f i=1:1:5 w \"foo \"\n w !,\"after loop\"\n w \"foo\" \n w test,!,!\n q  \n s foo=te\n\n";
    const DOC_AFTER_EDIT:  &str = "tag w \"before loop\",!\n f i=1:1:5 w \"foo \"\n w !,\"after loop\"\n w \"foo\" \n w test,!,!\n q  \n s foo=test\n\n";
    static FIRST_EDIT: LazyLock<TextDocumentContentChangeEvent> =
        LazyLock::new(|| TextDocumentContentChangeEvent {
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
        });
    static SECOND_EDIT: LazyLock<TextDocumentContentChangeEvent> =
        LazyLock::new(|| TextDocumentContentChangeEvent {
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
        });

    #[test]
    fn sequential_updates() {
        let mut document = Document::new(DOC_BEFORE_EDIT.to_owned());
        document.update(&[FIRST_EDIT.clone()]);
        document.update(&[SECOND_EDIT.clone()]);
        assert_eq!(document.text(), DOC_AFTER_EDIT)
    }
    #[test]
    fn batched_updates() {
        let mut document = Document::new(DOC_BEFORE_EDIT.to_owned());
        document.update(&[FIRST_EDIT.clone(), SECOND_EDIT.clone()]);
        assert_eq!(document.text(), DOC_AFTER_EDIT)
    }
}
