#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::all)]
#[allow(dead_code)]
pub mod bindings;

mod command;
mod dollar;
pub mod eval;
pub mod ffi;
pub mod function;
pub mod localvar;
mod op_code;
mod routine;
mod var;

pub use parser::{pest, Rule, SyntaxParser};

pub mod models {
    use tree_sitter::Node;
    lang_models::models!();

    pub unsafe fn type_tree(tree: &tree_sitter::Tree) -> source_file<'_> {
        source_file::create(tree.root_node())
    }
}

impl<'a> models::Expression<'a> {
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>) {
        use crate::bindings::PARTAB;
        use eval::ncopy;
        use models::ExpressionChildren::*;
        match self.children() {
            BinaryExpression(bin_exp) => {
                use models::BinaryOppChildren::*;
                bin_exp.exp_left().compile(source_code, comp);
                bin_exp.exp_right().compile(source_code, comp);
                comp.push(match bin_exp.opp().children() {
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
                } as u8);
            }
            Expression(exp) => exp.compile(source_code, comp),
            InderectExpression(exp) => {
                exp.children().compile(source_code, comp);
                //TODO note hardcoded at the moment
                comp.push(bindings::INDWRIT as u8);
            }
            PaternMatchExpression(pat_exp) => {
                use models::{PaternMatchExpressionExp_right::*, PatternOppChildren::*};
                pat_exp.exp_left().compile(source_code, comp);
                match pat_exp.exp_right() {
                    Expression(exp) => exp.compile(source_code, comp),
                    Patern(value) => {
                        //TODO remove duplication
                        use crate::eval::compile_string_literal;
                        let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                        compile_string_literal(&format!("\"{}\"", value), comp);
                    }
                }
                comp.push(match pat_exp.opp().children() {
                    OPPAT(_) => bindings::OPPAT,
                    OPNPAT(_) => bindings::OPNPAT,
                } as u8);
            }
            //TODO should inderect be considered a special case of unary?
            UnaryExpression(unary_exp) => {
                use models::UnaryOppChildren::*;
                unary_exp.exp().compile(source_code, comp);
                comp.push(match unary_exp.opp().children() {
                    OPMINUS(_) => bindings::OPMINUS,
                    OPNOT(_) => bindings::OPNOT,
                    OPPLUS(_) => bindings::OPPLUS,
                } as u8);
            }

            Variable(var) => var.compile(source_code,comp,VarTypes::Eval),
            number(num) => {
                let num = num.node().utf8_text(source_code.as_bytes()).unwrap();
                ncopy(num, &mut PARTAB::default(), comp);
            },

            string(value) => {
                use crate::eval::compile_string_literal;
                let value = value.node().utf8_text(source_code.as_bytes()).unwrap();
                compile_string_literal(value, comp);
            },

            IntrinsicFunction(intrinsic) => {
                use models::IntrinsicFunctionChildren::*;
                match &intrinsic.children() {
                    ExpFunctions(exp_fun) => {
                        use models::ExpFunctionsChildren::*;
                        let children = &exp_fun.children();
                        let (opcode, args) = match children {
                            View(x) => (crate::bindings::FUNV2 - 2, x.args()),
                            //TODO Text handling should be more detailed.
                            Text(x) => (crate::bindings::FUNT - 1, vec![x.args()]),
                            Translate(x) => (crate::bindings::FUNTR2 - 2, x.args()),
                            Find(x) => (crate::bindings::FUNF2 - 2, x.args()),
                            Fnumber(x) => (crate::bindings::FUNFN2 - 2, x.args()),
                            Random(x) => (crate::bindings::FUNR - 1, vec![x.args()]),
                            Reverse(x) => (crate::bindings::FUNRE - 1, vec![x.args()]),
                            Piece(x) => (crate::bindings::FUNP2 - 2, x.args()),
                            Justify(x) => (crate::bindings::FUNJ2 - 2, x.args()),
                            Extract(x) => (crate::bindings::FUNE1 - 1, x.args()),
                            Ascii(x) => (crate::bindings::FUNA1 - 1, x.args()),
                            Length(x) => (crate::bindings::FUNL1 - 1, x.args()),
                            Stack(x) => (crate::bindings::FUNST1 - 1, x.args()),
                            Char(x) => (crate::bindings::FUNC, x.args()),
                        };
                        let count = args.iter().map(|x| x.compile(source_code, comp)).count();
                        if opcode == crate::bindings::FUNC {
                            if count > 254 {
                                panic!("Char has too many args");
                            } else {
                                comp.push(opcode as u8);
                                comp.push(count as u8);
                            }
                        } else {
                            comp.push((opcode + count as u32) as u8);
                        }
                    }
                    VarFunctions(exp_fun) => {
                        use models::VarFunctionsChildren::*;
                        let children = &exp_fun.children();
                        let (opcode, var,args) = match children {
                            Name(x) => (crate::bindings::FUNNA1- 1, x.var(), x.args()),
                            Order(x) => (crate::bindings::FUNO1- 1, x.var(), x.args()),
                            Query(x) => (crate::bindings::FUNQ1- 1,  x.var(),x.args()),
                            Increment(x) => (crate::bindings::FUNI1- 1,  x.var(),x.args()),
                            Get(x) => (crate::bindings::FUNG1- 1,  x.var(),x.args()),
                            //TODO Next is an allisas for Order + hard coded param.
                            Next(x) => (crate::bindings::FUNO2- 1,  x.var(),None),
                            Data(x) => (crate::bindings::FUND- 1,  x.var(),None),
                            Qlength(x) => (crate::bindings::FUNQL- 1,  x.var(),None),
                            Qsubscript(x) => (crate::bindings::FUNQS- 2,  x.var(),Some(x.args())),
                        };
                        let var_type = match children {
                            Data(_) | Get(_) | Increment(_) => VarTypes::Build,
                            Name(_) | Order(_) | Query(_) | Next(_) => VarTypes::BuildNullable,
                            Qlength(_) | Qsubscript(_)=>VarTypes::Eval,

                        };

                        var.compile(source_code,comp,var_type);
                        let count = args.iter().map(|x| x.compile(source_code, comp)).count();
                        if let Next(_) = children{
                            ncopy("2", &mut PARTAB::default(), comp);
                        }

                        comp.push((opcode + count as u32+1) as u8);
                    }
                };
            }
        }
    }

    fn is_inderect(&self) -> bool {
        if let models::ExpressionChildren::InderectExpression(_) = self.children() {
            true
        } else {
            false
        }
    }
}

use crate::localvar::VarTypes;
impl <'a>models::Variable<'a>{
    pub fn compile(&self, source_code: &str, comp: &mut Vec<u8>,context:VarTypes){
        let subscripts = self.subs();

        use models::VariableHeading::*;
        let var_type = self
            .heading()
            .map(|heading| match &heading {
                NakedVariable(_) => bindings::TYPVARNAKED,
                IndirectVariable(exp) => {
                    exp.children().compile(source_code, comp);
                    comp.push(bindings::INDMVAR as u8);
                    bindings::TYPVARIND
                }
                GlobalVariable(_) => bindings::TYPVARGBL,
                GlobalUciVariable(exp) => {
                    exp.children().compile(source_code, comp);
                    bindings::TYPVARGBLUCI
                }
                GlobalUciEnvVariable(exps) => {
                    exps.children()
                        .iter()
                        .for_each(|x| x.compile(source_code, comp));
                    bindings::TYPVARGBLUCIENV
                }
            })
            .unwrap_or(bindings::TYPVARNAM);

        //NOTE c docs says subscripts heading,
        //but that is not what the code outputs
        subscripts.iter().for_each(|x| x.compile(source_code, comp));

        comp.push(context.code());
        match var_type {
            crate::bindings::TYPVARGBL | crate::bindings::TYPVARNAM => {
                comp.push((var_type | subscripts.len() as u32) as u8);
            }
            _ => {
                comp.push(var_type as u8);
                comp.push(subscripts.len() as u8);
            }
        }
        if let Some(name) = self.name() {
            let name = name.node().utf8_text(source_code.as_bytes()).unwrap();
            let name = bindings::VAR_U::from(name);
            comp.extend(name.as_array())
        }
    }
}

pub fn compile(source_code: &str) -> Vec<u8> {
        use tree_sitter::Parser;
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_mumps::language()).unwrap();
        let tree = parser.parse(source_code, None).unwrap();
        #[cfg(test)]
        dbg!(&tree.root_node().to_sexp());
        let tree = unsafe { models::type_tree(&tree) };

        let mut comp = vec![];

        let lines = tree.children();
        for line in lines {
            let commands = line.children();
            for command in commands {
                for arg in command.children() {
                    let arg = arg.children();
                    arg.compile(source_code, &mut comp);
                    if !arg.is_inderect() {
                        comp.push(bindings::CMWRTEX as u8);
                    }
                }
                comp.push(bindings::OPENDC as u8);
                comp.push(bindings::OPENDC as u8);
            }
            comp.pop();
            comp.push(bindings::ENDLIN as u8);
        }
        comp
    }

#[cfg(test)]
mod test {
    use super::*;

    use crate::ffi::test::compile_c;
    #[test]
    fn write_command() {
        let source_code = "w 9 w 8 w 7 w 6 w 5 w 4 w 3";
        let (orignal, _lock) = compile_c(source_code, bindings::parse);

        assert_eq!(orignal, compile(source_code));
    }

    use rstest::rstest;
    #[rstest]
    #[case("9")]
    #[case("10000000")]
    #[case("00000001")]
    #[case("0.1")]
    #[case("0.00001")]
    #[case("0.0")]
    #[case(".0000000")]
    #[case("0.0000000")]
    #[case("0.000010000")]
    #[case("00000000.00000000")]
    //TODO implement HISTORIC_EOK
    //#[case("1E100")]
    //#[case("1E-100")]
    //#[case("1.90E-100")]
    fn parse_number(#[case] num: &str) {
        let source_code = format!("w {}", num);
        let (orignal, _lock) = compile_c(&source_code, bindings::parse);

        assert_eq!(orignal, compile(&source_code));
    }
    #[rstest]
    #[case("+-+-+-+-+-234")]
    #[case("-10000")]
    #[case("--45")]
    #[case("'45")]
    #[case("-'-45")]
    fn parse_unary_exp(#[case] num: &str) {
        let source_code = format!("w {}", num);
        let (orignal, _lock) = compile_c(&source_code, bindings::parse);

        assert_eq!(orignal, compile(&source_code));
    }
    #[rstest]
    #[case("\"Some string\"")]
    #[case("\"Some numbers89097\"")]
    #[case("\" string with quote\"\"quote\"\" some text\"")]
    fn parse_string(#[case] num: &str) {
        let source_code = format!("w {}", num);
        let (orignal, _lock) = compile_c(&source_code, bindings::parse);

        assert_eq!(orignal, compile(&source_code));
    }

    #[rstest]
    #[case("98+9")]
    #[case("-98\\var(7,9)")]
    #[case("98+(something+9)")]
    fn parse_binary(#[case] num: &str) {
        let source_code = format!("w {}", num);
        let (orignal, _lock) = compile_c(&source_code, bindings::parse);

        assert_eq!(orignal, compile(&source_code));
    }
    #[rstest]
    #[case("SomeString?.A")]
    #[case("SomeString?1.3A")]
    #[case("SomeString?.(8A,1(1N))")]
    #[case("SomeString?.2A")]
    #[case("SomeString?1.A")]
    #[case("SomeString?@var")]
    fn parse_pattern(#[case] num: &str) {
        let source_code = format!("w {}", num);
        let (orignal, _lock) = compile_c(&source_code, bindings::parse);

        assert_eq!(orignal, compile(&source_code));
    }

    #[rstest]
    #[case("SomeString")]
    #[case("^SomeString")]
    #[case("^[atom]varName")]
    #[case("^|atom|varName")]
    #[case("^[atom1,atom2]varName")]
    #[case("SomeString(sub1)")]
    #[case("^SomeString(sub1)")]
    #[case("^(sub1)")]
    #[case("^|atom|varName(sub1)")]
    #[case("^[atom1,atom2]varName(sub1)")]
    #[case("SomeString(sub1,sub2)")]
    #[case("^SomeString(sub1,sub2)")]
    #[case("^|atom|varName(sub1,sub2)")]
    #[case("^[atom1,atom2]varName(sub1,sub2)")]
    #[case("@atom@(sub1,sub2)")]
    #[case("@varName")]
    //TODO index
    fn parse_var(#[case] num: &str) {
        let source_code = format!("w {}", num);
        let (orignal, _lock) = compile_c(&source_code, bindings::parse);

        assert_eq!(orignal, compile(&source_code));
    }
    use core::ops::RangeInclusive;
    #[rstest]
    #[case("View","V",2..=4)]
    #[case("Text","T",1..=1)]
    #[case("Translate","TR",2..=3)]
    #[case("Find","F",2..=3)]
    #[case("fnumber","Fn",2..=3)]
    #[case("Random","R",1..=1)]
    #[case("Reverse","Re",1..=1)]
    #[case("Piece","P",2..=4)]
    #[case("Justify","J",2..=3)]
    #[case("extract","E",1..=3)]
    #[case("ascii","a",1..=2)]
    #[case("char","c",1..=8)]
    //TODO test upper bounds of Char
    //currenrly getting segfale problby would need to increase the buffer.
    #[case("char","c",50..=50)]
    #[case("length","l",1..=2)]
    #[case("Stack","st",1..=2)]
    fn x_call(#[case] full: &str, #[case] abbreviated: &str, #[case] range: RangeInclusive<usize>) {
        use core::iter::repeat;
        for val in range {
            let args = repeat("11011").take(val).collect::<Vec<_>>().join(",");

            {
                let source_code = format!("w ${}({})", full, args);
                let (orignal, _lock) = compile_c(&source_code, bindings::parse);

                assert_eq!(orignal, compile(&source_code));
            }
            {
                let source_code = format!("w ${}({})", abbreviated, args);
                let (orignal, _lock) = compile_c(&source_code, bindings::parse);
                let temp = compile(&source_code);

                assert_eq!(orignal, temp);
            }
        }
    }
    #[rstest]
    #[case("Data","D",1..=1)]
    #[case("Get","G",1..=2)]
    #[case("increment","i",1..=2)]
    #[case("name","na",1..=2)]
    #[case("order","o",1..=2)]
    #[case("query","q",1..=2)]
    #[case("Next","n",1..=1)]
    #[case("Qlength","QL",1..=1)]
    #[case("QSUBSCRIPT","Qs",2..=2)]
    fn x_call_on_variable(
        #[case] full: &str,
        #[case] abbreviated: &str,
        #[case] range: RangeInclusive<usize>,
    ) {
        use core::iter::repeat;
        for val in range {
            let args = repeat("variable").take(val).collect::<Vec<_>>().join(",");
            {
                let source_code = format!("w ${}({})", full, args);
                let (orignal, _lock) = compile_c(&source_code, bindings::parse);

                assert_eq!(orignal, compile(&source_code));
            }
            {
                let source_code = format!("w ${}({})", abbreviated, args);
                let (orignal, _lock) = compile_c(&source_code, bindings::parse);
                let temp = compile(&source_code);

                assert_eq!(orignal, temp);
            }
        }
    }
}
