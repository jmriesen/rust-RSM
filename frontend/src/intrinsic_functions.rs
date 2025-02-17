use ir::{
    Expression, IntrinsicFunction, Variable,
    intrinsic_functions::{Function, SelectTerm, VarFunction},
};

use crate::TreeSitterParser;

fn function<const REQUIRED: usize, const OPTIONAL: usize>(
    args: Vec<lang_model::Expression<'_>>,
    source_code: &str,
) -> Function<REQUIRED, OPTIONAL> {
    use std::array::from_fn;
    let num_of_args = REQUIRED..=REQUIRED + OPTIONAL;
    //THis assert is here to satisfy mutation testing
    assert_eq!(num_of_args.clone().count(), OPTIONAL + 1);
    assert!(
        num_of_args.contains(&args.len()),
        "Exceded maximum arguments. Expected: {num_of_args:?}, Found:{}",
        args.len()
    );
    let mut iter = args.iter().map(|x| Expression::new(x, source_code));
    Function {
        required: from_fn(|_| iter.next().expect("required argument")),
        optional: from_fn(|_| iter.next()),
    }
}

fn var_function<const REQUIRED: usize, const OPTIONAL: usize>(
    var: lang_model::Variable<'_>,
    args: Vec<lang_model::Expression<'_>>,
    source_code: &str,
) -> VarFunction<REQUIRED, OPTIONAL> {
    VarFunction {
        variable: Variable::new(&var, source_code),
        function: function(args, source_code),
    }
}

impl<'a> TreeSitterParser<'a> for IntrinsicFunction {
    type NodeType = lang_model::IntrinsicFunction<'a>;
    fn new(sitter: &lang_model::IntrinsicFunction<'_>, source_code: &str) -> Self {
        use lang_model::IntrinsicFunctionChildren::*;

        match &sitter.children() {
            ExpFunctions(exp_fun) => {
                use lang_model::ExpFunctionsChildren::*;
                match exp_fun.children() {
                    Char(x) => Self::Char {
                        args: x
                            .args()
                            .iter()
                            .map(|x| Expression::new(x, source_code))
                            .collect(),
                    },
                    View(x) => Self::View(function(x.args(), source_code)),
                    Ascii(x) => Self::Ascii(function(x.args(), source_code)),
                    Extract(x) => Self::Extract(function(x.args(), source_code)),
                    Find(x) => Self::Find(function(x.args(), source_code)),
                    Fnumber(x) => Self::Fnumber(function(x.args(), source_code)),
                    Justify(x) => Self::Justify(function(x.args(), source_code)),
                    Length(x) => Self::Length(function(x.args(), source_code)),
                    Piece(x) => Self::Piece(function(x.args(), source_code)),
                    Random(x) => Self::Random(function(vec![x.args()], source_code)),
                    Reverse(x) => Self::Reverse(function(vec![x.args()], source_code)),
                    Stack(x) => Self::Stack(function(x.args(), source_code)),
                    Text(x) => Self::Text(function(vec![x.args()], source_code)),
                    Translate(x) => Self::Translate(function(x.args(), source_code)),
                }
            }
            VarFunctions(var_fun) => {
                use lang_model::VarFunctionsChildren::*;
                match var_fun.children() {
                    Qlength(x) => Self::QLength(var_function(x.var(), vec![], source_code)),
                    Qsubscript(x) => {
                        Self::QSubscript(var_function(x.var(), vec![x.args()], source_code))
                    }
                    Data(x) => Self::Data(var_function(x.var(), vec![], source_code)),
                    Get(x) => Self::Get(var_function(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Increment(x) => Self::Increment(var_function(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Name(x) => Self::Name(var_function(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Order(x) => Self::Order(var_function(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Query(x) => Self::Query(var_function(
                        x.var(),
                        x.args().into_iter().collect(),
                        source_code,
                    )),
                    Next(x) => {
                        Self::Next(var_function(x.var(), [].into_iter().collect(), source_code))
                    }
                }
            }
            Select(select) => Self::Select {
                terms: select
                    .children()
                    .array_chunks::<2>()
                    .map(|[condition, value]| SelectTerm {
                        condition: Expression::new(condition, source_code),
                        value: Expression::new(value, source_code),
                    })
                    .collect(),
            },
        }
    }
}
#[cfg(test)]
mod test {
    use crate::command_from_source;

    #[test]
    fn smoke_test() {
        command_from_source("W $O(foo)");
    }
}
