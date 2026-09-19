use std::{
    collections::HashMap,
    hint::black_box,
    str::FromStr,
    sync::{Mutex, RwLock},
};

use backend::{bite_code, runtime::Job};
use serde_json::Value;
use tower_lsp::lsp_types::{MessageType, Url};

use crate::{client::Client, document::Document};

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
    pub async fn run(
        &self,
        client: &impl Client,
        args: Vec<Value>,
        documents: &Mutex<HashMap<Url, Document>>,
    ) -> Option<Value> {
        match self {
            Commands::HelloWorld => {
                let uri = match args.first().unwrap() {
                    Value::String(uri) => Url::parse(uri).unwrap(),
                    _ => panic!(),
                };

                let text = black_box({
                    let documents = documents.lock().expect("The lock is not poisoned.");
                    let document = documents.get(&uri).unwrap();
                    let text = document.text().to_owned();
                    drop(documents);
                    text
                });

                //TODO: FIX CAUSING ISSUES WIHT WEB
                // Start of problematic section
                if let Ok(routine) = frontend::parse_routine(&text) {
                    let byte_code = backend::compile_routine(routine);
                    let mut job = Job::new(&byte_code);
                    job.run();
                    // End of problematic section

                    let output = &job.buffer;
                    client.log_message(MessageType::ERROR, output).await;
                    client
                        .show_message(MessageType::INFO, format!("Result{}", output))
                        .await;
                } else {
                    client
                        .log_message(MessageType::ERROR, "could not compile")
                        .await;
                    client
                        .show_message(MessageType::INFO, format!("Result{}", "could not compile"))
                        .await;
                };
                Some(Value::String("Hello world".to_string()))
            }
        }
    }
}
