use crate::models::*;

use crate::{
    function::{reserve_jump, write_jump},
    localvar::VarTypes,
};

use super::{ncopy, ExpressionContext};

use crate::bindings::PARTAB;

trait ExpFunctionsExt<'a> {
    fn base_code_and_args(&self) -> (u8, Vec<Expression<'a>>);
}

impl<'a> ExpFunctionsExt<'a> for ExpFunctions<'a> {
    //NOTE the opcode is intrisicly tied to the number of args
    //so it does not really make since to seporate them
    fn base_code_and_args(&self) -> (u8, Vec<Expression<'a>>) {
        use ExpFunctionsChildren::*;
        match self.children() {
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
        }
    }
}

trait VarFunctionsExt {
    fn components(&self) -> (u8, Variable, Option<Expression>);
    fn var_types(&self) -> VarTypes;
}

impl<'a> VarFunctionsExt for VarFunctions<'a> {
    fn components(&self) -> (u8, Variable, Option<Expression>) {
        use VarFunctionsChildren::*;
        let children = &self.children();
        match children {
            Name(x) => (crate::bindings::FUNNA1 - 1, x.var(), x.args()),
            Order(x) => (crate::bindings::FUNO1 - 1, x.var(), x.args()),
            Query(x) => (crate::bindings::FUNQ1 - 1, x.var(), x.args()),
            Increment(x) => (crate::bindings::FUNI1 - 1, x.var(), x.args()),
            Get(x) => (crate::bindings::FUNG1 - 1, x.var(), x.args()),
            //TODO Next is an allisas for Order + hard coded param.
            Next(x) => (crate::bindings::FUNO2 - 1, x.var(), None),
            Data(x) => (crate::bindings::FUND - 1, x.var(), None),
            Qlength(x) => (crate::bindings::FUNQL - 1, x.var(), None),
            Qsubscript(x) => (crate::bindings::FUNQS - 2, x.var(), Some(x.args())),
        }
    }

    fn var_types(&self) -> VarTypes {
        use VarFunctionsChildren::*;
        match self.children() {
            Data(_) | Get(_) | Increment(_) => VarTypes::Build,
            Name(_) | Order(_) | Query(_) | Next(_) => VarTypes::BuildNullable,
            Qlength(_) | Qsubscript(_) => VarTypes::Eval,
        }
    }
}

use crate::Compileable;
impl<'a> Compileable for IntrinsicFunction<'a> {
    type Context = ();
    fn compile(&self, source_code: &str, comp: &mut Vec<u8>, _context: Self::Context) {
        use IntrinsicFunctionChildren::*;

        //TODO Consider Reducing Duplication betwen ExpFunc and VarFunc
        match &self.children() {
            ExpFunctions(exp_fun) => {
                let (base_code, args) = exp_fun.base_code_and_args();

                for arg in &args {
                    arg.compile(source_code, comp, ExpressionContext::Eval)
                }

                if matches!(exp_fun.children(), ExpFunctionsChildren::Char(_)) {
                    if args.len() > 254 {
                        panic!("Char has too many args");
                    } else {
                        comp.push(base_code);
                        comp.push(args.len() as u8);
                    }
                } else {
                    comp.push(base_code + args.len() as u8);
                }
            }
            VarFunctions(var_fun) => {
                let (opcode, var, args) = var_fun.components();
                var.compile(source_code, comp, var_fun.var_types());

                for arg in &args {
                    arg.compile(source_code, comp, ExpressionContext::Eval)
                }

                if matches!(var_fun.children(), VarFunctionsChildren::Next(_)) {
                    ncopy("2", &mut PARTAB::default(), comp);
                }

                comp.push(opcode + args.iter().len() as u8 + 1);
            }
            Select(select) => {
                let jump_indexs = select
                    .children()
                    .array_chunks::<2>()
                    .map(|[condition, value]| {
                        condition.compile(source_code, comp, ExpressionContext::Eval);
                        comp.push(crate::bindings::JMP0);
                        let try_next = reserve_jump(comp);

                        value.compile(source_code, comp, ExpressionContext::Eval);
                        comp.push(crate::bindings::JMP);
                        let exit = reserve_jump(comp);

                        (try_next, exit)
                    })
                    .collect::<Vec<_>>();

                comp.push(crate::bindings::OPERROR);
                let errm4 = (-(crate::bindings::ERRM4 as i16)).to_le_bytes();
                comp.extend_from_slice(&errm4);

                for (try_next, exit) in jump_indexs {
                    write_jump(try_next, exit, comp);
                    write_jump(exit, comp.len(), comp);
                }
            }
        }
    }
}
