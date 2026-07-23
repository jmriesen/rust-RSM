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
#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs, sync::RwLock};
#[allow(clippy::wildcard_imports)]
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};

use crate::{document::Document, tokens::{AbsolutToken, TokenNode}, util::to_lsp_int};

mod document;
mod util;
mod tokens;
pub use tokens::TokenTypes;

struct ServerState {
    client: Client,
    documents: RwLock<HashMap<Url, Document>>,
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
                                token_types: TokenTypes::reference_ordering(),
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
                folding_range_provider: None,
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
        use tree_sitter::QueryCursor;

        let mut query_cursor = QueryCursor::new();
        let tokens: Vec<_> = document
            .query(&TokenTypes::query(), &mut query_cursor)
            .map(|x| TokenNode(x.captures[0].node))
            .map(|x| AbsolutToken::from(x)).collect();
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data:AbsolutToken::to_relitive(tokens),
        })))
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        let documents = self.documents.read().unwrap();

        let routine = documents
            .get(&params.text_document.uri)
            .expect("diagnostic can only be requested for open documents");
        let errors = routine.validate();

        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None, //TODO: this could change
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    items: errors,
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
            //It is fine to unwrap since the document must have been opened for there to be changes.
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
