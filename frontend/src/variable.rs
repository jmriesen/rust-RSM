use ir::{Expression, Variable, variable::VariableType};

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
            Some(E::GlobalVariable(_)) => GlobalVariable {
                name: name.unwrap(),
            },
            Some(E::GlobalUciVariable(x)) => GlobalUciVariable {
                name: name.unwrap(),
                uci: Box::new(Expression::new(&x.children(), source_code)),
            },
            Some(E::GlobalUciEnvVariable(x)) => {
                let args = x.children();
                assert_eq!(args.len(), 2);
                let mut args = args.into_iter().map(|x| Expression::new(&x, source_code));
                GlobalUciEnvVariable {
                    name: name.unwrap(),
                    uci: Box::new(args.next().expect("allready did bounds checking")),
                    env: Box::new(args.next().expect("allready did bounds checking")),
                }
            }
            None => Local {
                name: name.unwrap(),
            },
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
