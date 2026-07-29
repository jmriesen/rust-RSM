/// Syntactic sugar for specifying one value in a config struct and letting everything else be
/// the default.
/// See `example_function` for usage.
#[macro_export]
macro_rules! partial {
    //Base rules.
    ($name:ident { $field:ident : $inner:tt }) => {
        $name {
            $field: $inner,
            ..Default::default()
        }
    };
    ($name:ident { $field:ident : Some($inner:tt) }) => {
        $name {
            $field: Some($inner),
            ..Default::default()
        }
    };
    // Recursive rules
    ($name:ident { $field:ident : Some($struct:ident $inner:tt) }) => {
        $name {
            $field: Some(partial!($struct $inner)),
            ..Default::default()
        }
    };
    ($name:ident { $field:ident : $struct:ident $inner:tt }) => {
        $name {
            $field: partial!($struct $inner),
            ..Default::default()
        }
    };
}

///Example of how to use the partial macro.
#[allow(dead_code)]
fn example_function() {
    use tower_lsp::lsp_types::*;

    partial!(InitializeParams {
        capabilities: ClientCapabilities {
            text_document: Some(TextDocumentClientCapabilities {
                semantic_tokens: Some(SemanticTokensClientCapabilities {
                    multiline_token_support: Some(true)
                })
            })
        }
    });
}
