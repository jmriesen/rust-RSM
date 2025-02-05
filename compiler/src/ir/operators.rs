pub enum Unary {
    OPMINUS,
    OPPLUS,
    OPNOT,
}

impl Unary {
    pub fn new(sitter: lang_model::UnaryOpp) -> Self {
        use lang_model::UnaryOppChildren as E;
        match sitter.children() {
            E::OPMINUS(_) => Self::OPMINUS,
            E::OPPLUS(_) => Self::OPPLUS,
            E::OPNOT(_) => Self::OPNOT,
        }
    }
    pub fn op_code(&self) -> u8 {
        match self {
            Self::OPMINUS => ffi::OPMINUS,
            Self::OPPLUS => ffi::OPPLUS,
            Self::OPNOT => ffi::OPNOT,
        }
    }
}

pub enum Pattern {
    OPPAT,
    OPNPAT,
}

impl Pattern {
    pub fn new(sitter: lang_model::PatternOpp) -> Self {
        use lang_model::PatternOppChildren as E;
        match sitter.children() {
            E::OPPAT(_) => Self::OPPAT,
            E::OPNPAT(_) => Self::OPNPAT,
        }
    }

    pub fn op_code(&self) -> u8 {
        match self {
            Self::OPPAT => ffi::OPPAT,
            Self::OPNPAT => ffi::OPNPAT,
        }
    }
}

pub enum Binary {
    OPADD,
    OPSUB,
    OPMUL,
    OPDIV,
    OPINT,
    OPMOD,
    OPPOW,
    OPCAT,
    OPGTR,
    OPAND,
    OPCON,
    OPFOL,
    OPEQL,
    OPLES,
    OPNEQL,
    OPNLES,
    OPNGTR,
    OPNAND,
    OPNCON,
    OPNFOL,
    OPNSAF,
    OPSAF,
}

impl Binary {
    pub fn new(sitter: lang_model::BinaryOpp) -> Self {
        use lang_model::BinaryOppChildren::*;
        match sitter.children() {
            OPADD(_) => Self::OPADD,
            OPSUB(_) => Self::OPSUB,
            OPMUL(_) => Self::OPMUL,
            OPDIV(_) => Self::OPDIV,
            OPINT(_) => Self::OPINT,
            OPMOD(_) => Self::OPMOD,
            OPPOW(_) => Self::OPPOW,
            OPCAT(_) => Self::OPCAT,
            OPGTR(_) => Self::OPGTR,
            OPAND(_) => Self::OPAND,
            OPCON(_) => Self::OPCON,
            OPFOL(_) => Self::OPFOL,
            OPEQL(_) => Self::OPEQL,
            OPLES(_) => Self::OPLES,
            OPNEQL(_) => Self::OPNEQL,
            OPNLES(_) => Self::OPNLES,
            OPNGTR(_) => Self::OPNGTR,
            OPNAND(_) => Self::OPNAND,
            OPNCON(_) => Self::OPNCON,
            OPNFOL(_) => Self::OPNFOL,
            OPNSAF(_) => Self::OPNSAF,
            OPSAF(_) => Self::OPSAF,
        }
    }
    pub fn op_code(&self) -> u8 {
        use Binary::*;
        match self {
            OPADD => ffi::OPADD,
            OPSUB => ffi::OPSUB,
            OPMUL => ffi::OPMUL,
            OPDIV => ffi::OPDIV,
            OPINT => ffi::OPINT,
            OPMOD => ffi::OPMOD,
            OPPOW => ffi::OPPOW,
            OPCAT => ffi::OPCAT,
            OPGTR => ffi::OPGTR,
            OPAND => ffi::OPAND,
            OPCON => ffi::OPCON,
            OPFOL => ffi::OPFOL,
            OPEQL => ffi::OPEQL,
            OPLES => ffi::OPLES,
            OPNEQL => ffi::OPNEQL,
            OPNLES => ffi::OPNLES,
            OPNGTR => ffi::OPNGTR,
            OPNAND => ffi::OPNAND,
            OPNCON => ffi::OPNCON,
            OPNFOL => ffi::OPNFOL,
            OPNSAF => ffi::OPNSAF,
            OPSAF => ffi::OPSAF,
        }
    }
}
