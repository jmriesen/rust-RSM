use ir::{
    Expression, ExtrinsicFunction, Variable,
    extrinsic_function::{Args, Location},
};

use crate::TreeSitterParser;

impl<'a> TreeSitterParser<'a> for Args {
    type NodeType = lang_model::ExtrinsicFunctionArgs<'a>;
    fn new(sitter: &lang_model::ExtrinsicFunctionArgs<'_>, source_code: &str) -> Self {
        use lang_model::ExtrinsicFunctionArgs as E;
        match sitter {
            E::VarUndefined(_) => Self::VarUndefined,
            E::ByRef(var) => Self::ByRef(Variable::new(&var.children(), source_code)),
            E::Expression(exp) => Self::Expression(Expression::new(exp, source_code)),
        }
    }
}

impl<'a> TreeSitterParser<'a> for ExtrinsicFunction {
    type NodeType = lang_model::ExtrinsicFunction<'a>;
    fn new(sitter: &lang_model::ExtrinsicFunction<'_>, source_code: &str) -> Self {
        let tag = sitter.tag().map(|x| {
            let x = x.children();
            use lang_model::TagNameChildren as E;
            let tag = match &x {
                E::NumericIdentifier(x) => x.node(),
                E::identifier(x) => x.node(),
            };
            tag.utf8_text(source_code.as_bytes())
                .expect("tree sitter would have failed in not utf-8")
                .to_owned()
        });
        let routine = sitter.routine().map(|x| {
            x.node()
                .utf8_text(source_code.as_bytes())
                .expect("tree sitter would have failed in not utf-8")
                .to_owned()
        });

        let location = match (tag, routine) {
            (None, Some(routine)) => Location::Routine(routine),
            (Some(tag), None) => Location::Tag(tag),
            (Some(tag), Some(routine)) => Location::TagRoutine(tag, routine),
            (None, None) => unreachable!("Tree-sitter should prevent this"),
        };

        //NOTE It is easier to just remove the trailing VarUndefined when compiling then during parsing
        let mut arguments: Vec<_> = sitter
            .args()
            .iter()
            .map(|x| Args::new(x, source_code))
            .collect();
        if arguments
            .last()
            .is_some_and(|x| matches!(x, Args::VarUndefined))
        {
            arguments.pop();
        }
        Self {
            arguments,
            location,
        }
    }
}
