use super::OpCode;
use lang_model as models;
impl<'a> OpCode for models::UnaryOpp<'a> {
    fn op_code(&self) -> u8 {
        use models::UnaryOppChildren as E;
        match self.children() {
            E::OPMINUS(_) => ffi::OPMINUS,
            E::OPPLUS(_) => ffi::OPPLUS,
            E::OPNOT(_) => ffi::OPNOT,
        }
    }
}

impl<'a> OpCode for models::PatternOpp<'a> {
    fn op_code(&self) -> u8 {
        use models::PatternOppChildren as E;
        match self.children() {
            E::OPPAT(_) => ffi::OPPAT,
            E::OPNPAT(_) => ffi::OPNPAT,
        }
    }
}

impl<'a> OpCode for crate::models::BinaryOpp<'a> {
    fn op_code(&self) -> u8 {
        use models::BinaryOppChildren::*;
        match self.children() {
            OPADD(_) => ffi::OPADD,
            OPSUB(_) => ffi::OPSUB,
            OPMUL(_) => ffi::OPMUL,
            OPDIV(_) => ffi::OPDIV,
            OPINT(_) => ffi::OPINT,
            OPMOD(_) => ffi::OPMOD,
            OPPOW(_) => ffi::OPPOW,
            OPCAT(_) => ffi::OPCAT,
            OPGTR(_) => ffi::OPGTR,
            OPAND(_) => ffi::OPAND,
            OPCON(_) => ffi::OPCON,
            OPFOL(_) => ffi::OPFOL,
            OPEQL(_) => ffi::OPEQL,
            OPLES(_) => ffi::OPLES,
            OPNEQL(_) => ffi::OPNEQL,
            OPNLES(_) => ffi::OPNLES,
            OPNGTR(_) => ffi::OPNGTR,
            OPNAND(_) => ffi::OPNAND,
            OPNCON(_) => ffi::OPNCON,
            OPNFOL(_) => ffi::OPNFOL,
            OPNSAF(_) => ffi::OPNSAF,
            OPSAF(_) => ffi::OPSAF,
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
