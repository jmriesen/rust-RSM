/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
#![feature(array_windows)]
#![warn(clippy::pedantic)]
use lang_model::{commandChildren, BlockChildren};
use std::{fs, sync::RwLock};
#[allow(clippy::wildcard_imports)]
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};

fn to_lsp_int(num: usize) -> u32 {
    num.try_into().expect("The LSP protical only allows for specifying position using 32 bit integers. If you somehow have overflowed a i32::max, sorry, please restrucutre your program.")
}

struct ServerState {
    client: Client,
    documents: std::sync::RwLock<std::collections::HashMap<Url, Document>>,
}

struct Document {
    ///Note the document and tree must always stay in sync.
    source: String,
    tree: tree_sitter::Tree,
}

impl Document {
    fn new(source: String) -> Self {
        Self {
            tree: lang_model::create_tree(&source),
            source,
        }
    }

    fn query<'a>(
        &'a self,
        query: &'a tree_sitter::Query,
        query_cursor: &'a mut tree_sitter::QueryCursor,
    ) -> tree_sitter::QueryMatches<'a, 'a, &[u8]> {
        query_cursor.matches(query, self.tree.root_node(), self.source.as_bytes())
    }

    fn update(&mut self, mut changes: Vec<TextDocumentContentChangeEvent>) {
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

        //Go from back to front prevents indexes from changeing underneath us.
        //NOTE I have not tested with two concurrent changes
        for change in changes.iter().rev() {
            let get_index = |position: Position| {
                line_index[position.line as usize] + position.character as usize
            };
            let start = get_index(change.range.unwrap().start);
            let end = get_index(change.range.unwrap().end);

            self.source.replace_range(start..end, &change.text);
        }

        //TODO tree sitter suports updating the tree based on edits.
        //If I figure out the api I could make this more effieciant.
        self.tree = lang_model::create_tree(&self.source);
    }

    fn lint_tags_end_in_quit(&self) -> Vec<Diagnostic> {
        //Linting warnings for quits in a tag.
        //TODO unconditional quits befor the last line of a routine.
        //TODO earily return should be QUIT.
        //TODO all tags should end with a quit.
        //TODO either all quits should return a value, or non should.

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
                        if matches!(command, commandChildren::QUITCommand(_)) {
                            None
                        } else {
                            Some(*line.node())
                        }
                    }
                })
                .map(|node| {
                    dbg!(node.to_sexp());
                    Diagnostic {
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
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn lines_after_unconditional_quit(&self) -> Vec<Diagnostic> {
        //TODO unconditional quits befor the last line of a routine.
        //TODO this should really apply to blocks.

        if let Ok(routine) = lang_model::type_tree(&self.tree, &self.source) {
            routine
                .children()
                .iter()
                //Grab all lines
                .filter_map(|x| x.block().map(|x| x.children()))
                .flatten()
                .skip_while(|x| !match x {
                    BlockChildren::Block(_) => false, //TODO deal with nessted blocks
                    BlockChildren::line(line) => {
                        //Look for unconditional quit.
                        use lang_model::commandChildren as E;
                        line
                            //commands
                            .children()
                            .into_iter()
                            .map(|x| x.children())
                            //ignore anything after controlflow command
                            //.take_while(|x| !matches!(x, E::IfCommand(_)))
                            .take_while(|x| !matches!(x, E::ElseCommand(_)))
                            .take_while(|x| !matches!(x, E::For(_)))
                            .any(|x| matches!(x, E::QUITCommand(_)))
                    }
                })
                //Skip over the quit.
                .skip(1)
                .map(|x| match x {
                    BlockChildren::Block(block) => *block.node(), //TODO deal with nessted blocks
                    BlockChildren::line(line) => *line.node(),
                })
                .map(|node| {
                    dbg!(node.to_sexp());
                    Diagnostic {
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
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn validate(&self) -> Vec<Diagnostic> {
        use tree_sitter::{Query, QueryCursor};
        let mut query_cursor = QueryCursor::new();
        let error_query = Query::new(tree_sitter_mumps::language(), "(ERROR)@error").unwrap();
        let expressions = self.query(&error_query, &mut query_cursor);

        expressions
            .map(|exp| {
                let node = exp.captures[0].node;
                dbg!(node.to_sexp());
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

trait PointExt {
    fn to_position(&self) -> Position;
}

impl PointExt for tree_sitter::Point {
    fn to_position(&self) -> Position {
        Position {
            line: to_lsp_int(self.row),
            character: to_lsp_int(self.column),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for ServerState {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            legend: SemanticTokensLegend {
                                token_types: vec![
                                    SemanticTokenType::NUMBER,
                                    SemanticTokenType::STRING,
                                    SemanticTokenType::VARIABLE,
                                    SemanticTokenType::METHOD,
                                    SemanticTokenType::KEYWORD,
                                ],
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ),
                ),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: None,
                        inter_file_dependencies: true,
                        workspace_diagnostics: false,
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                    },
                )),
                folding_range_provider: Some(FoldingRangeProviderCapability::FoldingProvider(
                    FoldingProviderOptions {},
                )),
                ..ServerCapabilities::default()
            },
            server_info: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let documents = self.documents.read().unwrap();
        let document = documents.get(&params.text_document.uri).unwrap();
        use tree_sitter::{Query, QueryCursor};

        let key = |node_kind| match node_kind {
            "number" => 0,
            "string" => 1,
            "Variable" => 2,
            "TagName" => 3,
            _ => 4,
        };

        let query = Query::new(
            tree_sitter_mumps::language(),
            "[(number) (string) (Variable) (TagName)]@token",
        )
        .unwrap();
        let mut query_cursor = QueryCursor::new();
        //when I tried to use one query I was misssing nodes. I am not sure why.
        let command_query =
            Query::new(tree_sitter_mumps::language(), "(command . (_ . (_)@token))").unwrap();
        let mut command_query_cursor = QueryCursor::new();

        let mut tokens: Vec<_> = document
            .query(&query, &mut query_cursor)
            .chain(document.query(&command_query, &mut command_query_cursor))
            .map(|x| x.captures[0].node)
            .map(|node| {
                let start = node.start_position();
                let end = node.end_position();

                SemanticToken {
                    //NOTE Using absolutes position for now
                    //will convert into deltas latter.
                    delta_line: to_lsp_int(start.row),
                    delta_start: to_lsp_int(start.column),
                    length: to_lsp_int(end.column - start.column),
                    token_type: key(node.kind()), //TODO
                    token_modifiers_bitset: 0,
                }
            })
            .collect();

        //order matters since token location is specified using offesets.
        tokens.sort_by_key(|x| (x.delta_line, x.delta_start));

        //Inserting dummy inital token so that I can use windows to calculate offsets.

        let data: Vec<_> = tokens
            .array_windows()
            .map(|[previuse, current]| {
                //NOTE converting from absolute pos to deltas.
                let mut current = *current;
                current.delta_line -= previuse.delta_line;
                if current.delta_line == 0 {
                    current.delta_start -= previuse.delta_start;
                }
                current
            })
            .collect();
        dbg!(&data[0]);

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data,
        })))
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let documents = self.documents.read().unwrap();
        let document = documents.get(&params.text_document.uri).unwrap();
        use tree_sitter::{Query, QueryCursor};

        let query = Query::new(tree_sitter_mumps::language(), "(Block)@token").unwrap();
        let mut query_cursor = QueryCursor::new();

        Ok(Some(
            document
                .query(&query, &mut query_cursor)
                .map(|x| x.captures[0].node)
                .map(|block| FoldingRange {
                    start_line: to_lsp_int(block.start_position().row),
                    start_character: Some(0),
                    end_line: to_lsp_int(block.end_position().row),
                    end_character: Some(0),
                    kind: Some(FoldingRangeKind::Region),
                    collapsed_text: Some(String::new()),
                })
                .collect(),
        ))
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        let documents = self.documents.read().unwrap();

        let routine = documents
            .get(&params.text_document.uri)
            .expect("diagnostic can only be requested for open documents");
        let warnings = routine.lint_tags_end_in_quit();
        let warnings2 = routine.lines_after_unconditional_quit();
        let errors = routine.validate();

        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None, //TODO this could change
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    items: [warnings, errors, warnings2].concat(),
                    result_id: None,
                },
            }),
        ))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let source = fs::read_to_string(params.text_document.uri.to_file_path().unwrap()).unwrap();
        self.documents
            .write()
            .unwrap()
            .insert(params.text_document.uri, Document::new(source));
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {}

    async fn did_change(&self, change: DidChangeTextDocumentParams) {
        self.documents
            .write()
            .unwrap()
            .get_mut(&change.text_document.uri)
            .expect("The document should allready be open before changes are made")
            //It is fine to unwrap since the document must have been opend for there to be changes.
            .update(change.content_changes);
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| ServerState {
        client,
        documents: RwLock::default(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
