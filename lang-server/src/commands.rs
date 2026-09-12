use std::str::FromStr;

use serde_json::Value;
use tower_lsp::lsp_types::MessageType;

use crate::client::Client;

// Impl to/from string
macro_rules! commands {
    ($( {$name:ident, $str_rep:expr})*) => {
        pub enum Commands {
            $( $name, )*
        }

        impl FromStr for Commands {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $( $str_rep => Self::$name, )*
                    _ => return Err(()),
                })
            }
        }
        impl From<Commands> for String {
            fn from(value: Commands) -> Self {
                match value {
                    $( Commands::$name => $str_rep,)*
                }
                .to_string()
            }
        }
    };
}
commands!(
    {HelloWorld, "mumps.HelloWorld"}
);

impl Commands {
    pub async fn run(&self, client: &impl Client) -> Option<Value> {
        match self {
            Commands::HelloWorld => {
                client.log_message(MessageType::ERROR, "Hello world").await;
                client.show_message(MessageType::INFO, "Hello world").await;
                Some(Value::String("Hello world".to_string()))
            }
        }
    }
}
