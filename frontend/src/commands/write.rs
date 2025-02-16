use ir::{
    Expression,
    commands::{Command, PostCondition, Write},
};

use crate::TreeSitterParser;

pub fn new(sitter: &lang_model::WriteCommand, source_code: &str) -> Command {
    Command::Write(PostCondition {
        condition: sitter
            .post_condition()
            .map(|x| Expression::new(&x, source_code)),

        value: sitter
            .args()
            .iter()
            .map(|x| {
                use lang_model::WriteArgChildren as E;
                match x.children() {
                    E::Bang(_) => Write::Bang,
                    E::Clear(_) => Write::Clear,
                    E::Tab(tab) => Write::Tab(Expression::new(&tab.children(), source_code)),
                    E::Expression(expression) => {
                        Write::Expression(Expression::new(&expression, source_code))
                    }
                }
            })
            .collect(),
    })
}
