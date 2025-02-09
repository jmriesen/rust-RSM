use lang_model::CloseCommand;

use crate::{
    bite_code::BiteCode,
    ir::{
        commands::{Command, PostCondition},
        Compile, Expression,
    },
};

pub struct Close(Expression);

impl Close {
    pub fn new(sitter: &CloseCommand, source_code: &str) -> Command {
        assert!(
            sitter.args().len() > 0,
            "Close always takes at least one argument"
        );
        Command::Close(PostCondition {
            condition: sitter
                .post_condition()
                .map(|x| Expression::new(&x, source_code)),
            value: sitter
                .args()
                .iter()
                .map(|x| Expression::new(x, source_code))
                .map(|x| Self(x))
                .collect(),
        })
    }
}
impl Compile for Close {
    type Context = ();
    fn compile(&self, comp: &mut BiteCode, _: &()) {
        self.0
            .compile(comp, crate::expression::ExpressionContext::Close);
        if !matches!(self.0, Expression::InderectExpression { .. }) {
            comp.push(ffi::CMCLOSE);
        }
    }
}
