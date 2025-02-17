use ir::{
    Expression, Variable,
    variable::{Env, GlobleIdent, UserClassIdentifiers, VariableType},
};

use crate::TreeSitterParser;

impl<'a> TreeSitterParser<'a> for Variable {
    type NodeType = lang_model::Variable<'a>;
    fn new(sitter: &lang_model::Variable<'_>, source_code: &str) -> Self {
        use VariableType::*;
        use lang_model::VariableHeading as E;
        let name = sitter.name().map(|x| {
            x.node()
                .utf8_text(source_code.as_bytes())
                .unwrap()
                .to_owned()
        });

        let var_type = match sitter.heading() {
            Some(E::IndirectVariable(x)) => IndirectVariable {
                expression: Box::new(Expression::new(&x.children(), source_code)),
            },
            Some(E::NakedVariable(_)) => NakedVariable,
            None => Named {
                name: name.unwrap(),
                globle_ident: None,
            },
            Some(E::GlobalVariable(_)) => Named {
                name: name.unwrap(),
                globle_ident: Some(GlobleIdent { user_class: None }),
            },
            Some(E::GlobalUciVariable(x)) => Named {
                name: name.unwrap(),
                globle_ident: Some(GlobleIdent {
                    user_class: Some(Box::new(UserClassIdentifiers {
                        uci: Expression::new(&x.children(), source_code),
                        env: None,
                    })),
                }),
            },
            Some(E::GlobalUciEnvVariable(x)) => {
                let args = x.children();
                assert_eq!(args.len(), 2);
                let mut args = args.into_iter().map(|x| Expression::new(&x, source_code));
                Named {
                    name: name.unwrap(),
                    globle_ident: Some(GlobleIdent {
                        user_class: Some(Box::new(UserClassIdentifiers {
                            uci: args.next().expect("Already bounds checked"),
                            env: Some(Env(args.next().expect("Already bounds checked"))),
                        })),
                    }),
                }
            }
        };
        Self {
            var_type,
            subscripts: sitter
                .subs()
                .iter()
                .map(|x| Expression::new(&x, source_code))
                .collect(),
        }
    }
}
