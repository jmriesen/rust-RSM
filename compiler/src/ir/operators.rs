pub enum Unary {
    Minus,
    Plus,
    Not,
}

impl Unary {
    pub fn new(sitter: lang_model::UnaryOpp) -> Self {
        use lang_model::UnaryOppChildren as E;
        match sitter.children() {
            E::OPMINUS(_) => Self::Minus,
            E::OPPLUS(_) => Self::Plus,
            E::OPNOT(_) => Self::Not,
        }
    }
    pub fn op_code(&self) -> u8 {
        match self {
            Self::Minus => ffi::OPMINUS,
            Self::Plus => ffi::OPPLUS,
            Self::Not => ffi::OPNOT,
        }
    }
}

pub enum Pattern {
    Pat,
    NotPat,
}

impl Pattern {
    pub fn new(sitter: lang_model::PatternOpp) -> Self {
        use lang_model::PatternOppChildren as E;
        match sitter.children() {
            E::OPPAT(_) => Self::Pat,
            E::OPNPAT(_) => Self::NotPat,
        }
    }

    pub fn op_code(&self) -> u8 {
        match self {
            Self::Pat => ffi::OPPAT,
            Self::NotPat => ffi::OPNPAT,
        }
    }
}

pub enum Binary {
    Add,
    Sub,
    Multiply,
    Divide,
    IntDivide,
    Modulus,
    Power,
    Concatenate,
    GreaterThan,
    And,
    Contains,
    Follows,
    Equal,
    LessThan,
    NotEqual,
    NotLessThen,
    NotGreaterThan,
    NotAnd,
    NotContains,
    NotFollows,
    NotSortsAfter,
    SortsAfter,
}

impl Binary {
    pub fn new(sitter: lang_model::BinaryOpp) -> Self {
        use lang_model::BinaryOppChildren::*;
        match sitter.children() {
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
    pub fn op_code(&self) -> u8 {
        use Binary::*;
        match self {
            Add => ffi::OPADD,
            Sub => ffi::OPSUB,
            Multiply => ffi::OPMUL,
            Divide => ffi::OPDIV,
            IntDivide => ffi::OPINT,
            Modulus => ffi::OPMOD,
            Power => ffi::OPPOW,
            Concatenate => ffi::OPCAT,
            GreaterThan => ffi::OPGTR,
            And => ffi::OPAND,
            Contains => ffi::OPCON,
            Follows => ffi::OPFOL,
            Equal => ffi::OPEQL,
            LessThan => ffi::OPLES,
            NotEqual => ffi::OPNEQL,
            NotLessThen => ffi::OPNLES,
            NotGreaterThan => ffi::OPNGTR,
            NotAnd => ffi::OPNAND,
            NotContains => ffi::OPNCON,
            NotFollows => ffi::OPNFOL,
            NotSortsAfter => ffi::OPNSAF,
            SortsAfter => ffi::OPSAF,
        }
    }
}
