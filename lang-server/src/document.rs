use crate::{
    tokens::{AbsolutToken, TokenNode},
    TokenTypes,
};
use tower_lsp::lsp_types::{
    Position, SemanticToken, TextDocumentContentChangeEvent, TextDocumentSyncCapability,
    TextDocumentSyncKind,
};
use tree_sitter::{Query, QueryCursor, QueryMatches};
pub const DOCUMENT_SYNC_CAPABILITY: Option<TextDocumentSyncCapability> = Some(
    TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL),
);

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

        //OPTIMIZATION OPPORTUNITY: tree sitter supports updating the tree based on edits.
        self.tree = lang_model::create_tree(&self.source);
    }

    #[cfg(test)]
    pub fn text(&self) -> &str {
        &self.source
    }
    pub fn tokens(&self) -> Vec<SemanticToken> {
        let mut query_cursor = QueryCursor::new();
        let tokens: Vec<_> = self
            .query(&TokenTypes::query(), &mut query_cursor)
            .map(|x| TokenNode(x.captures[0].node))
            .map(|x| AbsolutToken::from(x))
            .collect();
        AbsolutToken::to_relitive(tokens)
    }
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use tower_lsp::lsp_types::{Position, Range, TextDocumentContentChangeEvent};

    use crate::document::Document;

    const DOC_BEFORE_EDIT: &str = "tag w \"before loop\",!\n f i=1:1:5 w \"foo \"\n w !,\"after loop\"\n w \"foo\" \n w test,!,!\n q  \n s foo=te\n\n";
    const DOC_AFTER_EDIT:  &str = "tag w \"before loop\",!\n f i=1:1:5 w \"foo \"\n w !,\"after loop\"\n w \"foo\" \n w test,!,!\n q  \n s foo=test\n\n";
    const FIRST_EDIT: LazyLock<TextDocumentContentChangeEvent> =
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
    const SECOND_EDIT: LazyLock<TextDocumentContentChangeEvent> =
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
    fn sequencal_updates() {
        let mut document = Document::new(DOC_BEFORE_EDIT.to_owned());
        document.update(vec![FIRST_EDIT.clone()]);
        document.update(vec![SECOND_EDIT.clone()]);
        assert_eq!(document.text(), DOC_AFTER_EDIT)
    }
    #[test]
    fn batched_updates() {
        let mut document = Document::new(DOC_BEFORE_EDIT.to_owned());
        document.update(vec![FIRST_EDIT.clone(), SECOND_EDIT.clone()]);
        assert_eq!(document.text(), DOC_AFTER_EDIT)
    }
}
