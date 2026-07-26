use core::{fmt::Display, future::Future, marker::Send };

use tower_lsp::lsp_types::MessageType;

/// This trait is intentionally seam where I can inject a mock implementation.
/// When running in release mode this should be a transparent pass though to `tower_lsp::Client`.
/// During testing this will most offend be a no-op.
pub trait Client : Send + Sync + 'static {
    fn log_message<M: Display + Send>(&self, typ: MessageType, message: M) -> impl Future<Output=()> + Send;
}

impl Client for tower_lsp::Client{
    async fn log_message<M: Display>(&self, typ: MessageType, message: M) {
        tower_lsp::Client::log_message(self, typ, message).await
    }
}

impl Client for (){
    async fn log_message<M: Display>(&self, _: MessageType, _: M) {
    }
}
