#![feature(array_windows)]

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use rsm::models;
use std::fs;


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
            tree: models::create_tree(&source),
            source,
        }
    }

    fn query<'a>(
        &'a self,
        query: &'a tree_sitter::Query,
        query_cursor: &'a mut tree_sitter::QueryCursor,
    ) -> tree_sitter::QueryMatches<'a, 'a, &[u8]> {
        query_cursor.matches(&query, self.tree.root_node(), &*self.source.as_bytes())
    }

    fn update(&mut self, mut changes: Vec<TextDocumentContentChangeEvent>) {
        //TODO it feels like this logic should be extracted out.
        //Mabe I should wrap the sting type. 
        // find the index that each line starts at.
        let line_index: Vec<_> = std::iter::once(0)
            .chain(self.source.match_indices('\n').map(|(x, _)| x + 1))
            .collect();

        changes.sort_by_key(|x| {
            let pos = x.range.expect("LSP is configured for incremental changes & they always provide a range").start;
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
        self.tree = models::create_tree(&self.source);
    }

    fn validate(&self) -> Vec<Diagnostic> {
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
                        start:node.start_position().to_position(),
                        end:node.end_position().to_position(),
                    },
                }
            })
            .collect()
    }
}

trait PointExt {
    fn to_position(&self)-> Position;
}

impl PointExt for tree_sitter::Point{
    fn to_position(&self)-> Position{
        Position {
            line: self.row as u32,
            character: self.column as u32,
        }
    }
}


#[tower_lsp::async_trait]
impl LanguageServer for ServerState{
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
                                        SemanticTokenType::KEYWORD,
                                    ],
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        ),
                    ),
                    diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions{
                        identifier: None,
                        inter_file_dependencies: true,
                        workspace_diagnostics: false,
                        work_done_progress_options: WorkDoneProgressOptions{
                            work_done_progress:None,
                        },
                    })),
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
            "identifier" => 2,
            _ => 3,
        };

        let query = Query::new(
            tree_sitter_mumps::language(),
            "[(number) (string) (identifier)]@token",
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
                    delta_line: start.row as u32,
                    delta_start: start.column as u32,
                    length: (end.column - start.column) as u32,
                    token_type: key(node.kind()), //TODO
                    token_modifiers_bitset: 0,
                }
            })
            .collect();

        //order matters since token location is specified using offesets.
        tokens.sort_by_key(|x| (x.delta_line, x.delta_start));

        //Inserting dummy inital token so that I can use windows to calculate offsets.

        let data :Vec<_> = tokens
            .array_windows()
            .map(|[previuse, current]| {
                //NOTE converting from absolute pos to deltas.
                let mut current = current.clone();
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

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams
    ) -> Result<DocumentDiagnosticReportResult>{
        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(
                RelatedFullDocumentDiagnosticReport{
                    related_documents:None,//TODO this could change
                    full_document_diagnostic_report:FullDocumentDiagnosticReport{
                        items:self.documents
                            .read()
                            .unwrap()
                            .get(&params.text_document.uri)
                            .expect("diagnostic can only be requested for open documents")
                            .validate(),
                        result_id:None,
                    }
                }
            )
        ))
    }

    async fn did_open(
        &self,
        params: DidOpenTextDocumentParams,
    ) {
        let source = fs::read_to_string(params.text_document.uri.to_file_path().unwrap()).unwrap();
        self.documents.write().unwrap()
            .insert(params.text_document.uri, Document::new(source));
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
    }

    async fn did_change(
        &self,
        change: DidChangeTextDocumentParams,
    ) {
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

    let (service, socket) = LspService::new(|client| ServerState{ client,documents:Default::default() });
    Server::new(stdin, stdout, socket).serve(service).await;
}
