#![warn(clippy::pedantic)]
use commands::Commands as MyCommand;
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Mutex, RwLock},
};
#[allow(clippy::wildcard_imports)]
use tower_lsp::{jsonrpc::Result, lsp_types::*, LanguageServer};

use crate::{
    document::{Document, DOCUMENT_SYNC_CAPABILITY},
    errors::DIAGNOSTIC_CAPACITIES,
    tokens::{remove_over_lapping, AbsolutToken, SEMANTIC_TOKENS_CAPABILITIES},
};

mod client;
mod commands;
mod config;
mod document;
mod errors;
mod tokens;
pub use tokens::TokenTypes;

pub struct MumpsLsp<Client: client::Client> {
    client: Client,
    documents: Mutex<HashMap<Url, Document>>,
    allow_overlapping_tokens: RwLock<bool>,
}
impl<Client: client::Client> MumpsLsp<Client> {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Mutex::default(),
            allow_overlapping_tokens: RwLock::new(false),
        }
    }

    /// # Panics
    ///
    /// Will Panic if the document lock is poisoned.
    pub fn did_open(&self, url: Url, text: String) {
        self.documents
            .lock()
            .expect("The lock is not poisoned.")
            .insert(url, Document::new(text));
    }
    /// # Panics
    ///
    /// Will Panic if the document lock is poisoned.
    pub fn tokens(&self, document: &TextDocumentIdentifier) -> Vec<SemanticToken> {
        let documents = self.documents.lock().expect("The lock is not poisoned.");
        let document = documents.get(&document.uri).unwrap();
        let tokens = AbsolutToken::to_relitive(document.tokens());
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
            .and_then(|x| x.semantic_tokens.map(|x| x.overlapping_token_support))
            .flatten()
            .unwrap_or(false);
        *self.allow_overlapping_tokens.write().unwrap() = supports_overlapping_tokens;

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: DOCUMENT_SYNC_CAPABILITY,
                semantic_tokens_provider: SEMANTIC_TOKENS_CAPABILITIES.clone(),
                diagnostic_provider: DIAGNOSTIC_CAPACITIES,
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(false),
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![MyCommand::HelloWorld.into()],
                    ..Default::default()
                }),
                ..ServerCapabilities::default()
            },
            server_info: None,
        })
    }
    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let top_of_file = Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        };

        let lens = CodeLens {
            range: top_of_file,
            command: Some(Command {
                title: "▶ Run Hello World".to_string(),
                command: MyCommand::HelloWorld.into(),
                arguments: Some(vec![serde_json::Value::String(
                    params.text_document.uri.into(),
                )]),
            }),
            data: None,
        };

        Ok(Some(vec![lens]))
    }
    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> Result<Option<serde_json::Value>> {
        match MyCommand::from_str(&params.command) {
            Ok(comand) => Ok(comand
                .run(&self.client, params.arguments, &self.documents)
                .await),
            Err(_) => Ok(None),
        }
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
            data: self.tokens(&params.text_document),
        })))
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        let documents = self.documents.lock().unwrap();

        let routine = documents
            .get(&params.text_document.uri)
            .expect("diagnostic can only be requested for open documents");

        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    items: routine.errors(),
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
            .lock()
            .unwrap()
            .get_mut(&change.text_document.uri)
            .expect("The document should already be open before changes are made")
            //It is fine to unwrap since the document must have been opened for there to be changes.
            .update(&change.content_changes);
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
