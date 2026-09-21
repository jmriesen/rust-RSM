use chumsky::{ParseResult, Parser};
use frontend::parser::routine;
pub use ir::Routine;
use tower_lsp::lsp_types::{
    Position, TextDocumentContentChangeEvent, TextDocumentSyncCapability, TextDocumentSyncKind,
};

pub const DOCUMENT_SYNC_CAPABILITY: Option<TextDocumentSyncCapability> = Some(
    TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL),
);

pub struct Document {
    ///Note the document and tree must always stay in sync.
    source: String,
}

impl Document {
    pub fn new(source: String) -> Self {
        Self { source }
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
    //Returns a function that can preform the conversations.
    //Returning a closure since normally you need to do a lot of conversations in a batch
    //And this lets me reuse the calculated newlines.
    //Lifetime bound is there to prevent the converter from being used after our immutable barrow ends
    pub fn index_converter<'a>(&'a self) -> impl Fn(usize) -> Position + 'a {
        let new_lines: Vec<_> = std::iter::once(0)
            .chain(self.text().match_indices('\n').map(
                |(x, _)| x + 1, /*The +1 moves us to start of next line.*/
            ))
            .collect();
        let index_to_position = move |index: usize| {
            let line = new_lines
                .iter()
                .rposition(|line_pos| *line_pos <= index)
                .unwrap_or(0);

            Position {
                line: line as u32,
                character: (index - new_lines[line]) as u32,
            }
        };
        index_to_position
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
    }

    pub fn text(&self) -> &str {
        &self.source
    }
    pub fn ir(&self) -> ParseResult<Routine, chumsky::error::Rich<'_, char>> {
        //TODO: Might be nice to pre-compute/cash
        //Not doing it right now due to lifetimes of the error bounds
        routine().parse(&self.source)
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
