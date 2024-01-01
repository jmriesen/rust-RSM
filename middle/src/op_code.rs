use super::*;

pub enum ExpressionContext {
    Write = bindings::INDWRIT as isize,
    Eval = bindings::INDEVAL as isize,
    Close = bindings::INDCLOS as isize,
}

impl <'a> models::UnaryOpp<'a> {
    pub fn op_code(&self)-> u8{
        use models::UnaryOppChildren as E;
        match self.children() {
            E::OPMINUS(_) => bindings::OPMINUS,
            E::OPPLUS(_) => bindings::OPPLUS,
            E::OPNOT(_) => bindings::OPNOT,
        }
    }
}

impl<'a> crate::models::BinaryOpp<'a> {
    pub fn op_code(&self) -> u8 {
        use models::BinaryOppChildren::*;
        match self.children() {
            OPADD(_) => bindings::OPADD,
            OPSUB(_) => bindings::OPSUB,
            OPMUL(_) => bindings::OPMUL,
            OPDIV(_) => bindings::OPDIV,
            OPINT(_) => bindings::OPINT,
            OPMOD(_) => bindings::OPMOD,
            OPPOW(_) => bindings::OPPOW,
            OPCAT(_) => bindings::OPCAT,
            OPGTR(_) => bindings::OPGTR,
            OPAND(_) => bindings::OPAND,
            OPCON(_) => bindings::OPCON,
            OPFOL(_) => bindings::OPFOL,
            OPEQL(_) => bindings::OPEQL,
            OPLES(_) => bindings::OPLES,
            OPNEQL(_) => bindings::OPNEQL,
            OPNLES(_) => bindings::OPNLES,
            OPNGTR(_) => bindings::OPNGTR,
            OPNAND(_) => bindings::OPNAND,
            OPNCON(_) => bindings::OPNCON,
            OPNFOL(_) => bindings::OPNFOL,
            OPNSAF(_) => bindings::OPNSAF,
            OPSAF(_) => bindings::OPSAF,
        }
    }
}

#[cfg(test)]
mod test {
    
    
    /*
    #[test]
    pub fn parse_op_code_test() {
        parse_op_code("+", OPADD);
        parse_op_code("-", OPSUB);
        parse_op_code("*", OPMUL);
        parse_op_code("/", OPDIV);
        parse_op_code("\\", OPINT);
        parse_op_code("#", OPMOD);
        parse_op_code("**", OPPOW);
        parse_op_code("_", OPCAT);
        parse_not_able_op_code("=", OPEQL, OPNEQL);
        parse_not_able_op_code("<", OPLES, OPNLES);
        parse_not_able_op_code(">", OPGTR, OPNGTR);
        parse_not_able_op_code("&", OPAND, OPNAND);
        parse_not_able_op_code("[", OPCON, OPNCON);
        parse_not_able_op_code("]", OPFOL, OPNFOL);
        parse_not_able_op_code("]]", OPSAF, OPNSAF);
        parse_not_able_op_code("?", OPPAT, OPNPAT);
    }

    pub fn parse_not_able_op_code(src: &str, opcode: u8, not_opcode: u8) {
        parse_op_code(src, opcode);
        parse_op_code(&format!("'{}", src), not_opcode);
    }
    */
}
