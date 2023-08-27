function repeatDel(rep,del) {
    return prec.left(1,seq(rep,repeat(seq(del,rep))));
}

var mumps_grammer = {
    name: 'mumps',
    precedences: ($) => [
        [
            $.number,
            $.UnaryOpp,
            $.UnaryExpression,
        ],
    ],

    rules: {
        source_file: $ => repeatDel($.line,"\n"),
        command:$=> choice(
            seq("w ",repeatDel($.commandArg,",")),
        ),
        commandArg:$=> $.Expression,
        line: $=> repeatDel($.command," "),

        identifier: $ => /([A-Z]|[a-z])([A-Z]|[a-z]|\d)*/,
        sign:$=> repeat1(prec(1,choice("+","-"))),

        number: $ => choice(
            seq(/\d+/,optional(seq(".",optional(/\d+/)))),
            seq(".",/\d+/),
                                    ),
        //TODO this is to restrictive
        string: $=> seq("\"",optional(repeat(choice(/[^"]*/,"\"\""))),"\""),

        //-------------------------------
        //opcodes
        //Binary
        Expression:$ => choice(
            $.number,
            $.string,
            $.Variable,
            $.UnaryExpression,
            $.BinaryExpression,
            $.PaternMatchExpression,
            $.InderectExpression,
            seq("(",$.Expression,")"),
            $.IntrinsicFunction
        ),
        InderectExpression:$ => prec.left(seq("@",$.Expression)),
        UnaryExpression:$ => prec.left(1,seq(
            field('opp',$.UnaryOpp),
            field('exp',$.Expression)
        )),
        BinaryExpression:$ => prec.left(0,seq(
            field('exp_left',$.Expression),
            field('opp',$.BinaryOpp),
            field('exp_right',$.Expression)
        )),
        PaternMatchExpression:$ => prec.left(0,seq(
            field('exp_left',$.Expression),
            field('opp',$.PatternOpp),
            choice(
                //TODO this is to premissive.
                seq("@",field('exp_right',$.Expression)),
                field('exp_right',$.Patern)
            )
        )),
        OPADD:$ => "+",
        OPSUB:$ => "-",
        OPMUL:$ => "*",
        OPDIV:$ => "/",
        OPINT:$ => "\\",
        OPMOD:$ => "#",
        OPCAT:$ => "_",
        OPGTR:$ => ">",
        OPAND:$ => "&",
        OPCON:$ => "[",
        OPFOL:$ => "]",
        OPEQL:$ => "=",
        OPLES:$ => "<",
        OPPOW:$=> "**",
        OPNEQL:$ =>  "'=",
        OPNLES:$ =>  "'<",
        OPNGTR:$ =>  "'>",
        OPNAND:$ =>  "'&",
        OPNCON:$ =>  "'[",
        OPNFOL:$ =>  "']",
        OPSAF:$ => "]]",
        OPNSAF:$ =>  "']]",
        BinaryOpp:$=> choice($.OPADD, $.OPSUB ,$.OPMUL ,$.OPDIV,$.OPINT,$.OPMOD,$.OPCAT,$.OPGTR ,$.OPAND ,$.OPCON ,$.OPFOL ,$.OPEQL ,$.OPLES,$.OPPOW,$.OPNEQL ,$.OPNLES ,$.OPNGTR ,$.OPNAND ,$.OPNCON ,$.OPNFOL ,$.OPSAF ,$.OPNSAF),

        //Unary
        OPNOT:$   => prec.left(0,"'"),
        OPPLUS:$  => prec.left(0,"+"),
        OPMINUS:$ => prec.left(0,"-"),
        UnaryOpp:$=> choice($.OPNOT,$.OPPLUS,$.OPMINUS),

        //pattern
        OPPAT:$ => "?",
        OPNPAT:$ =>  "'?",
        PatternOpp:$=> choice($.OPPAT,$.OPNPAT),

        Patern:$=> seq($.PaternRepetitionCount,repeat1($.PaternElement)),
        PaternRepetitionCount: $=> choice(/[0-9]+/,/[0-9]*\.[0-9]*/),
        PaternElement:$=> choice($.string,seq("(",repeatDel($.Patern,","),")"),/A|C|E|L|N|P|U/),
        //-------------------------------
        IndirectVariable:$=> seq("@",$.Expression,"@"),
        NakedVariable:$=> "^",
        GlobalVariable:$=> seq("^"),
        GlobalUciVariable:$=> choice(
            seq("^|", $.Expression, "|"),
            //TODO check if square brackets are valid.
            prec(1,seq("^[", $.Expression, "]"))
        ),
        GlobalUciEnvVariable:$=> prec(1,seq("^[", $.Expression,",",$.Expression, "]")),
        _VariableSubscripts:$=> seq("(",repeatDel($.Expression,","),")"),


        Variable :$=> choice(
            seq(
                field('heading',choice(
                    $.IndirectVariable,
                    $.NakedVariable
                )),
                field('subs',$._VariableSubscripts)
            ),
            seq(
                optional(field('heading',choice(
                    $.GlobalVariable,
                    $.GlobalUciVariable,
                    $.GlobalUciEnvVariable
                ))),
                field('name',$.identifier),
                optional(field('subs',$._VariableSubscripts))
            )
        )
        //temp:$=>$.Expression,

    },
    extras: $ => [],
};

//NOTE caseInsensitive gramer not yet supported
//Folloing advice given hear [[https://github.com/tree-sitter/tree-sitter/issues/122#issuecomment-356370963][issue]]
function caseInsensitive (keyword) {
    return new RegExp(keyword
                      .split('')
                      .map(letter => `[${letter.toLowerCase()}${letter.toUpperCase()}]`)
                      .join('')
                     );
}
function fn_regex(fn,abreviation_len){
    return choice(caseInsensitive(fn),caseInsensitive(fn.substring(0, abreviation_len)));
}

function fn_rule(grammer,fn,lower,upper,abreviation_len,variable_fn){
    grammer.rules[fn] = $ =>{
        let options = Array.from({length: upper-lower+1}, (x, i) => i+lower)
            .map(num => {
                let exps = Array(num).fill(field('args',$.Expression));
                if (variable_fn){
                    exps[0] = field('var',$.Variable);
                }
                let args = exps.flatMap(exp=>[exp,","]).slice(0,-1);
                return seq(...args);
            });
        return seq("$",fn_regex(fn,abreviation_len),"(",choice(...options),")");
    };
}

let exp_functions = [
    ["View",2,4,1],
    ["Text",1,1,1],
    ["Translate",2,3,2],
    ["Find",2,3,1],
    ["Fnumber",2,3,2],
    ["Random",1,1,1],
    ["Reverse",1,1,2],
    ["Piece",2,4,1],
    ["Justify",2,3,1],
    ["Extract",1,3,1],
    ["Ascii",1,2,1],
    //["Char",1,254,1],
    ["Length",1,2,1],
    ["Stack",1,2,1],
];

let var_functions = [
    ["Name",1,2,2],
    ["Order",1,2,1],
    ["Query",1,2,1],
    ["Get",1,2,1],
    ["Increment",1,2,1],
    ["Next",1,1,1],
    ["Data",1,1,1],
    ["Qlength",1,1,2],
    ["Qsubscript",2,2,2],
];



exp_functions.forEach(fn =>fn_rule(mumps_grammer,...fn,false));
var_functions.forEach(fn =>fn_rule(mumps_grammer,...fn,true));


mumps_grammer.rules["VarFunctions"] = $=>choice(...var_functions.map(x=> $[x[0]]));
mumps_grammer.rules["Char"] = $=>seq("$",fn_regex("Char",1),"(",repeatDel(field('args',$.Expression),","),")");
mumps_grammer.rules["ExpFunctions"] = $=>choice(
    $.Char,
    ...exp_functions.map(x=> $[x[0]]));
mumps_grammer.rules["IntrinsicFunction"] = $=>choice($.VarFunctions,$.ExpFunctions);


module.exports = grammar(mumps_grammer);
