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

    pub fn lint_tags_end_in_quit(&self) -> Vec<Diagnostic> {
        //Linting warnings for quits in a tag.
        //TODO: unconditional quits before the last line of a routine.
        //TODO: early return should be QUIT.
        //TODO: all tags should end with a quit.
        //TODO: either all quits should return a value, or non should.

        if let Ok(routine) = lang_model::type_tree(&self.tree, &self.source) {
            routine
                .children()
                .iter()
                //Pull out last line of each tag if it exists.
                .filter_map(|x| x.block().and_then(|x| x.children().last().cloned()))
                .filter_map(|x| match x {
                    BlockChildren::Block(block) => Some(*block.node()),
                    BlockChildren::line(line) => {
                        let commands = line.children();
                        let command = commands.last().unwrap().children();
                        if matches!(command, commandChildren::QuitCommand(_)) {
                            None
                        } else {
                            Some(*line.node())
                        }
                    }
                })
                .map(|node| Diagnostic {
                    code_description: None,
                    code: None,
                    message: "tags should end with a quit command".to_string(),
                    source: None,
                    tags: None,
                    data: None,
                    related_information: None,
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: Range {
                        start: node.start_position().to_position(),
                        end: node.end_position().to_position(),
                    },
                })
                .collect()
        } else {
            vec![]
        }
    }

    pub fn lines_after_unconditional_quit(&self) -> Vec<Diagnostic> {
        //TODO: unconditional quits before the last line of a routine.
        //TODO: this should really apply to blocks.

        if let Ok(routine) = lang_model::type_tree(&self.tree, &self.source) {
            routine
                .children()
                .iter()
                //Grab all lines
                .filter_map(|x| x.block().map(|x| x.children()))
                .flatten()
                .skip_while(|x| !match x {
                    BlockChildren::Block(_) => false, //TODO: deal with nested blocks
                    BlockChildren::line(line) => {
                        //Look for unconditional quit.
                        use lang_model::commandChildren as E;
                        line
                            //commands
                            .children()
                            .into_iter()
                            .map(|x| x.children())
                            //Ignore anything after control flow command
                            //.take_while(|x| !matches!(x, E::IfCommand(_)))
                            .take_while(|x| !matches!(x, E::ElseCommand(_)))
                            .take_while(|x| !matches!(x, E::For(_)))
                            .any(|x| matches!(x, E::QuitCommand(_)))
                    }
                })
                //Skip over the quit.
                .skip(1)
                .map(|x| match x {
                    BlockChildren::Block(block) => *block.node(), //TODO: deal with nested blocks
                    BlockChildren::line(line) => *line.node(),
                })
                .map(|node| Diagnostic {
                    code_description: None,
                    code: None,
                    message: "Lines after an unconditional quite will be ignored.".to_string(),
                    source: None,
                    tags: None,
                    data: None,
                    related_information: None,
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: Range {
                        start: node.start_position().to_position(),
                        end: node.end_position().to_position(),
                    },
                })
                .collect()
        } else {
            vec![]
        }
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
