use std::{collections::HashMap, str::FromStr, sync::RwLock};

use backend::runtime::Job;
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
        documents: &RwLock<HashMap<Url, Document>>,
    ) -> Option<Value> {
        match self {
            Commands::HelloWorld => {
                let uri = match args.first().unwrap() {
                    Value::String(uri) => Url::parse(uri).unwrap(),
                    _ => panic!(),
                };

                let output = {
                    let documents = documents.read().unwrap();
                    let document = documents.get(&uri).unwrap();
                    let parse_result = document.ir().clone();

                    match parse_result.into_result() {
                        Ok(routine) => {
                            let byte_code = backend::compile_routine(routine);
                            let mut job = Job::new(&byte_code);
                            job.run();
                            // End of problematic section

                            job.buffer.clone()
                        }
                        Err(errs) => {
                            format!(
                                "Could not compile{:?}",
                                errs.into_iter()
                                    .map(|x| format!("{x:?}"))
                                    .collect::<String>()
                            )
                        }
                    }
                };

                client
                    .show_message(MessageType::INFO, format!("Result{}", output))
                    .await;

                Some(Value::String("Hello world".to_string()))
            }
        }
    }
}
