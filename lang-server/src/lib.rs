#![warn(clippy::pedantic)]
use std::{collections::HashMap, sync::RwLock};
#[allow(clippy::wildcard_imports)]
use tower_lsp::{jsonrpc::Result, lsp_types::*, LanguageServer};
use tree_sitter::{QueryCursor, StreamingIterator};

use crate::{
    document::{Document, DOCUMENT_SYNC_CAPABILITY},
    errors::{ErrorNode, DIAGNOSTIC_CAPACITIES},
    tokens::{remove_over_lapping, AbsolutToken, TokenNode, SEMANTIC_TOKENS_CAPABILITIES},
    util::collect,
};

mod client;
mod config;
mod document;
mod errors;
mod tokens;
mod util;
pub use tokens::TokenTypes;

pub struct MumpsLsp<Client: client::Client> {
    client: Client,
    documents: RwLock<HashMap<Url, Document>>,
    allow_overlapping_tokens: RwLock<bool>,
}
impl<Client: client::Client> MumpsLsp<Client> {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: RwLock::default(),
            allow_overlapping_tokens: RwLock::new(false),
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
        let mut query_cursor = QueryCursor::new();
        let tokens: Vec<_> = collect(
            document
                .query(&TokenTypes::query(), &mut query_cursor)
                .map(|x| TokenNode(x.captures[0].node))
                .map(|x| AbsolutToken::from(x)),
        );

        let tokens = AbsolutToken::to_relitive(tokens);
        if *self.allow_overlapping_tokens.read().unwrap() {
            tokens
        } else {
            remove_over_lapping(tokens)
        }
    }
}

#[tower_lsp::async_trait]
impl<Client: client::Client + 'static> LanguageServer for MumpsLsp<Client> {
    async fn initialize(&self, client_config: InitializeParams) -> Result<InitializeResult> {
        let supports_overlapping_tokens = client_config
            .capabilities
            .text_document
            .map(|x| x.semantic_tokens.map(|x| x.overlapping_token_support))
            .flatten()
            .flatten()
            .unwrap_or(false);
        *self.allow_overlapping_tokens.write().unwrap() = supports_overlapping_tokens;

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
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: self.tokens(params.text_document),
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
        let errors = collect(
            routine
                .query(&errors::ERROR_QUERY, &mut query_cursor)
                .map(|x| ErrorNode(x.captures[0].node))
                .map(|x| x.into()),
        );

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
