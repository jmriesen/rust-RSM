function repeatDel(rep,del) {
    return prec.left(1,seq(rep,repeat(seq(del,rep))));
}

var mumps_grammer = {
    name: 'mumps',
    word:$=> $.identifier,
    externals: $ => [
        $.VarUndefined,
        $._indent,
        $._dedent,
        $._line_level,
        $.error_sentinel
    ],

    precedences: ($) => [
        [
            $.number,
            $.UnaryOpp,
            $.UnaryExpression,

        ],
    ],

    rules: {
        source_file: $ => repeat1($.Tag),
        Tag:$=> seq(field("name",$.TagName),choice(field("block",$.Block),"\n")),
        Block:$=> seq(
            $._indent,
            repeat1(choice(
                //NOTE including a newline here forces the routine to include a new line at the end.
                seq($._line_level,$.line,"\n"),
                $.Block
            )),
            $._dedent
        ),
        WriteArg:$=> choice(
            $.Bang,
            $.Clear,
            $.Tab,
            $.Expression
        ),
        ForArg:$=> seq(
            $.Expression,
            optional(
                seq(":",$.Expression,
                    optional(
                        seq(":",$.Expression)
                    )
                   )
            )
        ),
        Bang:$=> "!",
        Clear:$=> "#",
        Tab:$=> seq("?",$.Expression),
        DoArg:$=> seq(
            field('function',$.ExtrinsicFunction),
            optional(seq(":",field('post_condition',$.Expression)))
        ),

        line: $=> seq(repeatDel($.command," "),repeat(" ")),
        TagName: $=> choice($.identifier,$.NumericIdentifier),
        NumericIdentifier:$=> /\d{1,32}/,
        ExtrinsicFunction:$=> seq(
            choice(
                field("tag",$.TagName),
                seq("^",field("routine",$.identifier)),
                seq(field("tag",$.TagName),"^",field("routine",$.identifier)),
            ),
            optional(
                seq(
                    "(",
                    //NOTE It is easier to just remove the traling VarUndefined when compiling then then durring parseing
                    field("args",
                          repeatDel(
                              choice($.ByRef,$.Expression,$.VarUndefined),
                              ",")
                         ),
                    ")",
                )
            ),
        ),
        ByRef :$=>seq(".",$.Variable),

        //TODO identifiers should be constraind to 32 digets.
        identifier: $ => /([A-Z]|[a-z])([A-Z]|[a-z]|\d)*/,
        sign:$=> repeat1(prec(1,choice("+","-"))),

        number: $ => choice(
            seq(/\d+/,optional(seq(".",optional(/\d+/)))),
            seq(".",/\d+/),
        ),

        string: $=> /(\"([#-~]|[ ])*\")+/,  // all charactors from # to ~ excluding quote. //TODO check the actural specs latter.

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
            $.IntrinsicFunction,
            $.IntrinsicVar,
            seq("$$",$.ExtrinsicFunction),
            $.XCall
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
        Space:$=> " ",


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
        ),
        Select:$=>seq(
            fn_regex("select",1),
            "(",
            repeatDel(seq($.Expression,":",$.Expression),","),
            ")")
    },
    extras: $ => [
    ],
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
        return seq(fn_regex(fn,abreviation_len),"(",choice(...options),")");
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
    ["Stack",1,2,2],
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
mumps_grammer.rules["Char"] = $=>seq(fn_regex("Char",1),"(",repeatDel(field('args',$.Expression),","),")");
mumps_grammer.rules["ExpFunctions"] = $=>choice(
    $.Char,
    ...exp_functions.map(x=> $[x[0]]));

mumps_grammer.rules["IntrinsicFunction"] = $=>seq("$",choice(
    $.Select,
    $.VarFunctions,
    $.ExpFunctions
));

IntrinsicVar =[
    ["Device",1],
    ["Ecode",2],
    ["Estack",2],
    ["Etrap",2],
    ["Horolog",1],
    ["Io",1],
    ["Job",1],
    ["Key",1],
    ["Principal",1],
    ["Quit",1],
    ["Reference",1],
    //NOTE I using stackVar rather then stack to avoid name collition.
    //["Stack",2],
    ["System",2],
    ["Storage",1],
    ["Test",1],
    ["X",1],
    ["Y",1],
];


IntrinsicVar.forEach(
    variable => mumps_grammer.rules[variable[0]] = $ => fn_regex(...variable)
);
mumps_grammer.rules["StackVar"] = $ => fn_regex("stack",2);

mumps_grammer.rules["IntrinsicVar"] = $=> seq("$",choice(
    $.StackVar,
    ...IntrinsicVar.map(x =>{return $[x[0]];})
));

Xcall= [
    ["Directory",true],
    ["Host",true],
    ["File",true],
    ["ErrMsg",true],
    ["OpCom",true],
    ["Signal",true],
    ["Spawn",true],
    ["Version",true],
    ["Zwrite",true],
    ["E",false],
    ["Paschk",false],
    ["V",false],
    //TODO using X again would cause name colition.
    //["X",false],
    ["Xrsm",false],
    ["SetEnv",true],
    ["GetEnv",true],
    ["RouChk",true],
    ["Fork",true],
    ["IC",true],
    ["Wait",true],
    ["Debug",false],
    ["Compress",true],
];
Xcall.forEach(
    x => mumps_grammer.rules[x[0]] = $ => {
        const name = x[0].toUpperCase();
        if (x[1]){
            return seq("%",name);
        }else{
            return name;
        }
    }
);

mumps_grammer.rules["XCallX"] = $ => "X";

mumps_grammer.rules["XCall"] = $=>seq(
    "$&",
    field("code",choice(
        $.XCallX,
        ...Xcall.map(x =>{return $[x[0]];})
    )),
    "(",
    field("args",$.Expression),
    optional(seq(
        ",",
        field("args",$.Expression)
    )),
    ")"
);


let commandTypes =
    [
        ["Write","WriteArg",true],
        ["Brake","Expression",true],
        ["Else",null,false],
        ["Close","Expression",true],
        ["Do","DoArg",true],
        ["New","identifier",true],
        ["QUIT","Expression",true],
    ];

commandTypes.forEach(
    x => {

        mumps_grammer.rules[x[0]] = $ => fn_regex(x[0],1);
        mumps_grammer.rules[x[0]+"Command"] = $ => {
            var postcondition = [];
            if (x[2]){
                postcondition.push(
                    optional(seq(":",field('post_condition',$.Expression))));
            }
            var args = [];
            if (x[1] != null){
                args.push(optional(repeatDel(field('args',$[x[1]]),",")));
            }
            return seq(
                $[x[0]],
                ...postcondition,
                choice(
                    seq(
                        " ",
                        ...args,
                    ),
                ),

            );
        }
    }
);
mumps_grammer.rules["For"] = $=>seq(
    fn_regex("For",1),
    choice(
        " ",
        seq(
            " ",
            field('variable',$.Variable),
            "=",
            repeatDel(field('args',$.ForArg),",")
        ),
    ),
)
mumps_grammer.rules["command"] = $=>choice(
    $.For,
    ...commandTypes.map(x=> $[x[0]+"Command"])
);

module.exports = grammar(mumps_grammer);
