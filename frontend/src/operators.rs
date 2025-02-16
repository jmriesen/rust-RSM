use ir::operators::{Binary, Unary};

use crate::TreeSitterParser;

impl<'a> TreeSitterParser<'a> for Unary {
    type NodeType = lang_model::UnaryOpp<'a>;
    fn new(sitter: &lang_model::UnaryOpp<'a>, _: &str) -> Self {
        use lang_model::UnaryOppChildren as E;
        match sitter.children() {
            E::OPMINUS(_) => Self::Minus,
            E::OPPLUS(_) => Self::Plus,
            E::OPNOT(_) => Self::Not,
        }
    }
}

pub enum BinaryType<'a> {
    Binary(lang_model::BinaryOpp<'a>),
    Pattern(lang_model::PatternOpp<'a>),
}

impl<'a> TreeSitterParser<'a> for Binary {
    type NodeType = BinaryType<'a>;
    fn new(sitter: &BinaryType<'a>, _: &str) -> Self {
        match sitter {
            BinaryType::Binary(binary_opp) => {
                use lang_model::BinaryOppChildren::*;
                match binary_opp.children() {
                    OPADD(_) => Self::Add,
                    OPSUB(_) => Self::Sub,
                    OPMUL(_) => Self::Multiply,
                    OPDIV(_) => Self::Divide,
                    OPINT(_) => Self::IntDivide,
                    OPMOD(_) => Self::Modulus,
                    OPPOW(_) => Self::Power,
                    OPCAT(_) => Self::Concatenate,
                    OPGTR(_) => Self::GreaterThan,
                    OPAND(_) => Self::And,
                    OPCON(_) => Self::Contains,
                    OPFOL(_) => Self::Follows,
                    OPEQL(_) => Self::Equal,
                    OPLES(_) => Self::LessThan,
                    OPNEQL(_) => Self::NotEqual,
                    OPNLES(_) => Self::NotLessThen,
                    OPNGTR(_) => Self::NotGreaterThan,
                    OPNAND(_) => Self::NotAnd,
                    OPNCON(_) => Self::NotContains,
                    OPNFOL(_) => Self::NotFollows,
                    OPNSAF(_) => Self::NotSortsAfter,
                    OPSAF(_) => Self::SortsAfter,
                }
            }
            BinaryType::Pattern(pattern_opp) => {
                use lang_model::PatternOppChildren as E;
                match pattern_opp.children() {
                    E::OPPAT(_) => Self::Pattern,
                    E::OPNPAT(_) => Self::NotPattern,
                }
            }
        }
    }
}
