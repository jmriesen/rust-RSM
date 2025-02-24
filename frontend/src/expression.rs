use std::str::FromStr;

use ir::{
    Expression, ExternalCalls, ExtrinsicFunction, IntrinsicFunction, IntrinsicVar, Variable,
    operators,
};
use value::{CreationError, Number, Value};

use crate::{TreeSitterParser, operators::BinaryType};

pub fn parse_string_litteral(string: &str) -> Result<Value, CreationError> {
    let string = string
        //Strip off outer quotes.
        .strip_prefix('"')
        .unwrap()
        .strip_suffix('"')
        .unwrap()
        //Replace "" with " quote.
        .replace("\"\"", "\"");
    Value::from_str(&string)
}
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::parse_string_litteral;
    use value::Value;

    #[test]
    fn parseing_string_litteral() {
        let value = parse_string_litteral(r#""this "is" a string""#).unwrap();
        assert_eq!(value, Value::from_str("this \"is\" a string").unwrap());
    }
}

impl<'a> TreeSitterParser<'a> for Expression {
    type NodeType = lang_model::Expression<'a>;
    fn new(sitter: &lang_model::Expression<'a>, source_code: &str) -> Self {
        let nested_new = |exp| Box::new(Self::new(&exp, source_code));
        use lang_model::ExpressionChildren as S;
        use std::str::FromStr;
        match sitter.children() {
            S::number(num) => {
                let num = num.node().utf8_text(source_code.as_bytes()).unwrap();
                Self::Number(Number::from_str(num).unwrap().into())
            }
            S::string(value) => {
                let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                Self::String(parse_string_litteral(value).unwrap())
            }
            S::Variable(var) => Self::Variable(Variable::new(&var, source_code)),
            S::IntrinsicVar(var) => Self::IntrinsicVar(IntrinsicVar::new(&var, source_code)),
            S::Expression(exp) => Self::new(&exp, source_code),
            S::InderectExpression(exp) => Self::InderectExpression(nested_new(exp.children())),
            S::UnaryExpression(unary_exp) => Self::UnaryExpression {
                op_code: operators::Unary::new(&unary_exp.opp(), source_code),
                expresstion: nested_new(unary_exp.exp()),
            },
            S::BinaryExpression(bin_exp) => Self::BinaryExpression {
                left: nested_new(bin_exp.exp_left()),
                op_code: operators::Binary::new(&BinaryType::Binary(bin_exp.opp()), source_code),
                right: nested_new(bin_exp.exp_right()),
            },
            S::PaternMatchExpression(pat_exp) => {
                use lang_model::PaternMatchExpressionExp_right as E;
                let right = match pat_exp.exp_right() {
                    E::Expression(exp) => nested_new(exp),
                    E::Patern(value) => {
                        let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                        Box::new(Self::String(Value::from_str(value).unwrap()))
                    }
                };
                Self::BinaryExpression {
                    left: nested_new(pat_exp.exp_left()),
                    op_code: operators::Binary::new(
                        &BinaryType::Pattern(pat_exp.opp()),
                        source_code,
                    ),
                    right,
                }
            }
            S::ExtrinsicFunction(x) => {
                Self::ExtrinsicFunction(ExtrinsicFunction::new(&x, source_code))
            }
            S::XCall(xcall) => Self::ExternalCalls {
                args: xcall
                    .args()
                    .iter()
                    .map(|x| Self::new(x, source_code))
                    .collect(),
                op_code: ExternalCalls::new(&xcall.code(), source_code),
            },
            S::IntrinsicFunction(intrinsic) => {
                Self::IntrinsicFunction(Box::new(IntrinsicFunction::new(&intrinsic, source_code)))
            }
        }
    }
}
