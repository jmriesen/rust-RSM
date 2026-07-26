#![warn(clippy::pedantic)]
use std::{collections::HashMap, sync::RwLock};
use tokio::sync::oneshot::error;
#[allow(clippy::wildcard_imports)]
use tower_lsp::{jsonrpc::Result, lsp_types::*, LanguageServer};
use tree_sitter::QueryCursor;

use crate::{
    document::{Document, DOCUMENT_SYNC_CAPABILITY},
    errors::{ErrorNode, DIAGNOSTIC_CAPACITIES},
    tokens::SEMANTIC_TOKENS_CAPABILITIES,
};

mod client;
mod document;
mod errors;
mod tokens;
mod util;
pub use tokens::TokenTypes;

pub struct MumpsLsp<Client: client::Client> {
    client: Client,
    documents: RwLock<HashMap<Url, Document>>,
}
impl<Client: client::Client> MumpsLsp<Client> {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: RwLock::default(),
        }
    }
    pub fn did_open(&self, url: Url, text: String) {
        self.documents
            .write()
            .unwrap()
            .insert(url, Document::new(text));
    }
    pub fn tokens(&self, document: TextDocumentIdentifier) -> Vec<SemanticToken> {
        let documents = self.documents.read().unwrap();
        let document = documents.get(&document.uri).unwrap();
        document.tokens()
    }
}

#[tower_lsp::async_trait]
impl<Client: client::Client + 'static> LanguageServer for MumpsLsp<Client> {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: DOCUMENT_SYNC_CAPABILITY,
                semantic_tokens_provider: SEMANTIC_TOKENS_CAPABILITIES.clone(),
                diagnostic_provider: DIAGNOSTIC_CAPACITIES,
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
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: document.tokens(),
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

        let mut query_cursor = QueryCursor::new();
        let errors = routine
            .query(&errors::ERROR_QUERY, &mut query_cursor)
            .map(|x| ErrorNode(x.captures[0].node))
            .map(|x| x.into())
            .collect();

        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    items: errors,
                    result_id: None,
                },
            }),
        ))
    }

    async fn did_open(
        &self,
        DidOpenTextDocumentParams {
            text_document: TextDocumentItem { uri, text, .. },
        }: DidOpenTextDocumentParams,
    ) {
        Self::did_open(&self, uri, text);
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
#[cfg(test)]
pub mod test {
    /// Creates a file based off the calling file+line number.
    /// Intended as an easy way to get a unique Url for unit tests.
    #[macro_export]
    macro_rules! test_url {
        () => {
            concat!("file:///", core::file!(), ".", line!())
                .parse()
                .unwrap()
        };
    }
}
