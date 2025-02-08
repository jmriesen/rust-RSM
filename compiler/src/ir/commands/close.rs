use lang_model::CloseCommand;

use crate::ir::Expression;

pub struct Close(Expression);

impl Close {
    pub fn new(sitter: &CloseCommand, source_code: &str) -> Vec<Self> {
        assert!(
            sitter.args().len() > 0,
            "Close always takes at least one argument"
        );
        sitter
            .args()
            .iter()
            .map(|x| Expression::new(x, source_code))
            .map(|x| Self(x))
            .collect()
    }
    pub fn compile(&self, comp: &mut Vec<u8>) {
        self.0
            .compile(comp, crate::expression::ExpressionContext::Close);
        if !matches!(self.0, Expression::InderectExpression { .. }) {
            comp.push(ffi::CMCLOSE);
        }
    }
}
