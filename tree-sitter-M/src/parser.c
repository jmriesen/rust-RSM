#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#ifdef _MSC_VER
#pragma optimize("", off)
#elif defined(__clang__)
#pragma clang optimize off
#elif defined(__GNUC__)
#pragma GCC optimize ("O0")
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 367
#define LARGE_STATE_COUNT 3
#define SYMBOL_COUNT 253
#define ALIAS_COUNT 0
#define TOKEN_COUNT 146
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 12
#define MAX_ALIAS_SEQUENCE_LENGTH 10
#define PRODUCTION_ID_COUNT 29

enum {
  anon_sym_LF = 1,
  anon_sym_ = 2,
  sym_NumericIdentifier = 3,
  anon_sym_DOLLAR_DOLLAR = 4,
  anon_sym_CARET = 5,
  anon_sym_LPAREN = 6,
  anon_sym_COMMA = 7,
  anon_sym_RPAREN = 8,
  sym_VarUndefined = 9,
  anon_sym_DOT = 10,
  sym_identifier = 11,
  anon_sym_PLUS = 12,
  anon_sym_DASH = 13,
  aux_sym_number_token1 = 14,
  anon_sym_DQUOTE = 15,
  aux_sym_string_token1 = 16,
  anon_sym_DQUOTE_DQUOTE = 17,
  anon_sym_AT = 18,
  sym_OPMUL = 19,
  sym_OPDIV = 20,
  sym_OPINT = 21,
  sym_OPMOD = 22,
  sym_OPCAT = 23,
  sym_OPGTR = 24,
  sym_OPAND = 25,
  sym_OPCON = 26,
  anon_sym_RBRACK = 27,
  sym_OPEQL = 28,
  sym_OPLES = 29,
  sym_OPPOW = 30,
  sym_OPNEQL = 31,
  sym_OPNLES = 32,
  sym_OPNGTR = 33,
  sym_OPNAND = 34,
  sym_OPNCON = 35,
  sym_OPNFOL = 36,
  sym_OPSAF = 37,
  sym_OPNSAF = 38,
  anon_sym_SQUOTE = 39,
  sym_OPPAT = 40,
  sym_OPNPAT = 41,
  aux_sym_PaternRepetitionCount_token1 = 42,
  aux_sym_PaternRepetitionCount_token2 = 43,
  aux_sym_PaternElement_token1 = 44,
  anon_sym_CARET_PIPE = 45,
  anon_sym_PIPE = 46,
  anon_sym_CARET_LBRACK = 47,
  aux_sym_Select_token1 = 48,
  aux_sym_Select_token2 = 49,
  anon_sym_COLON = 50,
  aux_sym_View_token1 = 51,
  aux_sym_View_token2 = 52,
  aux_sym_Text_token1 = 53,
  aux_sym_Text_token2 = 54,
  aux_sym_Translate_token1 = 55,
  aux_sym_Translate_token2 = 56,
  aux_sym_Find_token1 = 57,
  aux_sym_Find_token2 = 58,
  aux_sym_Fnumber_token1 = 59,
  aux_sym_Fnumber_token2 = 60,
  aux_sym_Random_token1 = 61,
  aux_sym_Random_token2 = 62,
  aux_sym_Reverse_token1 = 63,
  aux_sym_Reverse_token2 = 64,
  aux_sym_Piece_token1 = 65,
  aux_sym_Piece_token2 = 66,
  aux_sym_Justify_token1 = 67,
  aux_sym_Justify_token2 = 68,
  aux_sym_Extract_token1 = 69,
  aux_sym_Extract_token2 = 70,
  aux_sym_Ascii_token1 = 71,
  aux_sym_Ascii_token2 = 72,
  aux_sym_Length_token1 = 73,
  aux_sym_Length_token2 = 74,
  aux_sym_Stack_token1 = 75,
  aux_sym_Stack_token2 = 76,
  aux_sym_Name_token1 = 77,
  aux_sym_Name_token2 = 78,
  aux_sym_Order_token1 = 79,
  aux_sym_Order_token2 = 80,
  aux_sym_Query_token1 = 81,
  aux_sym_Query_token2 = 82,
  aux_sym_Get_token1 = 83,
  aux_sym_Get_token2 = 84,
  aux_sym_Increment_token1 = 85,
  aux_sym_Increment_token2 = 86,
  aux_sym_Next_token1 = 87,
  aux_sym_Next_token2 = 88,
  aux_sym_Data_token1 = 89,
  aux_sym_Data_token2 = 90,
  aux_sym_Qlength_token1 = 91,
  aux_sym_Qlength_token2 = 92,
  aux_sym_Qsubscript_token1 = 93,
  aux_sym_Qsubscript_token2 = 94,
  aux_sym_Char_token1 = 95,
  aux_sym_Char_token2 = 96,
  anon_sym_DOLLAR = 97,
  aux_sym_Device_token1 = 98,
  aux_sym_Ecode_token1 = 99,
  aux_sym_Ecode_token2 = 100,
  aux_sym_Estack_token1 = 101,
  aux_sym_Estack_token2 = 102,
  aux_sym_Etrap_token1 = 103,
  aux_sym_Etrap_token2 = 104,
  aux_sym_Horolog_token1 = 105,
  aux_sym_Horolog_token2 = 106,
  aux_sym_Io_token1 = 107,
  aux_sym_Job_token1 = 108,
  aux_sym_Key_token1 = 109,
  aux_sym_Key_token2 = 110,
  aux_sym_Principal_token1 = 111,
  aux_sym_Quit_token1 = 112,
  aux_sym_Reference_token1 = 113,
  aux_sym_System_token1 = 114,
  aux_sym_System_token2 = 115,
  aux_sym_Storage_token1 = 116,
  aux_sym_Test_token1 = 117,
  aux_sym_X_token1 = 118,
  aux_sym_Y_token1 = 119,
  anon_sym_PERCENT = 120,
  anon_sym_DIRECTORY = 121,
  anon_sym_HOST = 122,
  anon_sym_FILE = 123,
  anon_sym_ERRMSG = 124,
  anon_sym_OPCOM = 125,
  anon_sym_SIGNAL = 126,
  anon_sym_SPAWN = 127,
  anon_sym_VERSION = 128,
  anon_sym_ZWRITE = 129,
  sym_E = 130,
  sym_Paschk = 131,
  sym_V = 132,
  sym_Xrsm = 133,
  anon_sym_SETENV = 134,
  anon_sym_GETENV = 135,
  anon_sym_ROUCHK = 136,
  anon_sym_FORK = 137,
  anon_sym_IC = 138,
  anon_sym_WAIT = 139,
  sym_Debug = 140,
  anon_sym_COMPRESS = 141,
  sym_XCallX = 142,
  anon_sym_DOLLAR_AMP = 143,
  aux_sym_Write_token1 = 144,
  aux_sym_Write_token2 = 145,
  sym_source_file = 146,
  sym_commandArg = 147,
  sym_line = 148,
  sym__Tag = 149,
  sym_ExtrinsicFunction = 150,
  sym_ByRef = 151,
  sym__FunctionArg = 152,
  sym_number = 153,
  sym_string = 154,
  sym_Expression = 155,
  sym_InderectExpression = 156,
  sym_UnaryExpression = 157,
  sym_BinaryExpression = 158,
  sym_PaternMatchExpression = 159,
  sym_OPADD = 160,
  sym_OPSUB = 161,
  sym_OPFOL = 162,
  sym_BinaryOpp = 163,
  sym_OPNOT = 164,
  sym_OPPLUS = 165,
  sym_OPMINUS = 166,
  sym_UnaryOpp = 167,
  sym_PatternOpp = 168,
  sym_Patern = 169,
  sym_PaternRepetitionCount = 170,
  sym_PaternElement = 171,
  sym_IndirectVariable = 172,
  sym_NakedVariable = 173,
  sym_GlobalVariable = 174,
  sym_GlobalUciVariable = 175,
  sym_GlobalUciEnvVariable = 176,
  sym__VariableSubscripts = 177,
  sym_Variable = 178,
  sym_Select = 179,
  sym_View = 180,
  sym_Text = 181,
  sym_Translate = 182,
  sym_Find = 183,
  sym_Fnumber = 184,
  sym_Random = 185,
  sym_Reverse = 186,
  sym_Piece = 187,
  sym_Justify = 188,
  sym_Extract = 189,
  sym_Ascii = 190,
  sym_Length = 191,
  sym_Stack = 192,
  sym_Name = 193,
  sym_Order = 194,
  sym_Query = 195,
  sym_Get = 196,
  sym_Increment = 197,
  sym_Next = 198,
  sym_Data = 199,
  sym_Qlength = 200,
  sym_Qsubscript = 201,
  sym_VarFunctions = 202,
  sym_Char = 203,
  sym_ExpFunctions = 204,
  sym_IntrinsicFunction = 205,
  sym_Device = 206,
  sym_Ecode = 207,
  sym_Estack = 208,
  sym_Etrap = 209,
  sym_Horolog = 210,
  sym_Io = 211,
  sym_Job = 212,
  sym_Key = 213,
  sym_Principal = 214,
  sym_Quit = 215,
  sym_Reference = 216,
  sym_System = 217,
  sym_Storage = 218,
  sym_Test = 219,
  sym_X = 220,
  sym_Y = 221,
  sym_StackVar = 222,
  sym_IntrinsicVar = 223,
  sym_Directory = 224,
  sym_Host = 225,
  sym_File = 226,
  sym_ErrMsg = 227,
  sym_OpCom = 228,
  sym_Signal = 229,
  sym_Spawn = 230,
  sym_Version = 231,
  sym_Zwrite = 232,
  sym_SetEnv = 233,
  sym_GetEnv = 234,
  sym_RouChk = 235,
  sym_Fork = 236,
  sym_IC = 237,
  sym_Wait = 238,
  sym_Compress = 239,
  sym_XCall = 240,
  sym_Write = 241,
  sym_command = 242,
  aux_sym_source_file_repeat1 = 243,
  aux_sym_line_repeat1 = 244,
  aux_sym_ExtrinsicFunction_repeat1 = 245,
  aux_sym_string_repeat1 = 246,
  aux_sym_Patern_repeat1 = 247,
  aux_sym_PaternElement_repeat1 = 248,
  aux_sym__VariableSubscripts_repeat1 = 249,
  aux_sym_Select_repeat1 = 250,
  aux_sym_Char_repeat1 = 251,
  aux_sym_Write_repeat1 = 252,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_LF] = "\n",
  [anon_sym_] = " ",
  [sym_NumericIdentifier] = "NumericIdentifier",
  [anon_sym_DOLLAR_DOLLAR] = "$$",
  [anon_sym_CARET] = "^",
  [anon_sym_LPAREN] = "(",
  [anon_sym_COMMA] = ",",
  [anon_sym_RPAREN] = ")",
  [sym_VarUndefined] = "VarUndefined",
  [anon_sym_DOT] = ".",
  [sym_identifier] = "identifier",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [aux_sym_number_token1] = "number_token1",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_token1] = "string_token1",
  [anon_sym_DQUOTE_DQUOTE] = "\"\"",
  [anon_sym_AT] = "@",
  [sym_OPMUL] = "OPMUL",
  [sym_OPDIV] = "OPDIV",
  [sym_OPINT] = "OPINT",
  [sym_OPMOD] = "OPMOD",
  [sym_OPCAT] = "OPCAT",
  [sym_OPGTR] = "OPGTR",
  [sym_OPAND] = "OPAND",
  [sym_OPCON] = "OPCON",
  [anon_sym_RBRACK] = "]",
  [sym_OPEQL] = "OPEQL",
  [sym_OPLES] = "OPLES",
  [sym_OPPOW] = "OPPOW",
  [sym_OPNEQL] = "OPNEQL",
  [sym_OPNLES] = "OPNLES",
  [sym_OPNGTR] = "OPNGTR",
  [sym_OPNAND] = "OPNAND",
  [sym_OPNCON] = "OPNCON",
  [sym_OPNFOL] = "OPNFOL",
  [sym_OPSAF] = "OPSAF",
  [sym_OPNSAF] = "OPNSAF",
  [anon_sym_SQUOTE] = "'",
  [sym_OPPAT] = "OPPAT",
  [sym_OPNPAT] = "OPNPAT",
  [aux_sym_PaternRepetitionCount_token1] = "PaternRepetitionCount_token1",
  [aux_sym_PaternRepetitionCount_token2] = "PaternRepetitionCount_token2",
  [aux_sym_PaternElement_token1] = "PaternElement_token1",
  [anon_sym_CARET_PIPE] = "^|",
  [anon_sym_PIPE] = "|",
  [anon_sym_CARET_LBRACK] = "^[",
  [aux_sym_Select_token1] = "Select_token1",
  [aux_sym_Select_token2] = "Select_token2",
  [anon_sym_COLON] = ":",
  [aux_sym_View_token1] = "View_token1",
  [aux_sym_View_token2] = "View_token2",
  [aux_sym_Text_token1] = "Text_token1",
  [aux_sym_Text_token2] = "Text_token2",
  [aux_sym_Translate_token1] = "Translate_token1",
  [aux_sym_Translate_token2] = "Translate_token2",
  [aux_sym_Find_token1] = "Find_token1",
  [aux_sym_Find_token2] = "Find_token2",
  [aux_sym_Fnumber_token1] = "Fnumber_token1",
  [aux_sym_Fnumber_token2] = "Fnumber_token2",
  [aux_sym_Random_token1] = "Random_token1",
  [aux_sym_Random_token2] = "Random_token2",
  [aux_sym_Reverse_token1] = "Reverse_token1",
  [aux_sym_Reverse_token2] = "Reverse_token2",
  [aux_sym_Piece_token1] = "Piece_token1",
  [aux_sym_Piece_token2] = "Piece_token2",
  [aux_sym_Justify_token1] = "Justify_token1",
  [aux_sym_Justify_token2] = "Justify_token2",
  [aux_sym_Extract_token1] = "Extract_token1",
  [aux_sym_Extract_token2] = "Extract_token2",
  [aux_sym_Ascii_token1] = "Ascii_token1",
  [aux_sym_Ascii_token2] = "Ascii_token2",
  [aux_sym_Length_token1] = "Length_token1",
  [aux_sym_Length_token2] = "Length_token2",
  [aux_sym_Stack_token1] = "Stack_token1",
  [aux_sym_Stack_token2] = "Stack_token2",
  [aux_sym_Name_token1] = "Name_token1",
  [aux_sym_Name_token2] = "Name_token2",
  [aux_sym_Order_token1] = "Order_token1",
  [aux_sym_Order_token2] = "Order_token2",
  [aux_sym_Query_token1] = "Query_token1",
  [aux_sym_Query_token2] = "Query_token2",
  [aux_sym_Get_token1] = "Get_token1",
  [aux_sym_Get_token2] = "Get_token2",
  [aux_sym_Increment_token1] = "Increment_token1",
  [aux_sym_Increment_token2] = "Increment_token2",
  [aux_sym_Next_token1] = "Next_token1",
  [aux_sym_Next_token2] = "Next_token2",
  [aux_sym_Data_token1] = "Data_token1",
  [aux_sym_Data_token2] = "Data_token2",
  [aux_sym_Qlength_token1] = "Qlength_token1",
  [aux_sym_Qlength_token2] = "Qlength_token2",
  [aux_sym_Qsubscript_token1] = "Qsubscript_token1",
  [aux_sym_Qsubscript_token2] = "Qsubscript_token2",
  [aux_sym_Char_token1] = "Char_token1",
  [aux_sym_Char_token2] = "Char_token2",
  [anon_sym_DOLLAR] = "$",
  [aux_sym_Device_token1] = "Device_token1",
  [aux_sym_Ecode_token1] = "Ecode_token1",
  [aux_sym_Ecode_token2] = "Ecode_token2",
  [aux_sym_Estack_token1] = "Estack_token1",
  [aux_sym_Estack_token2] = "Estack_token2",
  [aux_sym_Etrap_token1] = "Etrap_token1",
  [aux_sym_Etrap_token2] = "Etrap_token2",
  [aux_sym_Horolog_token1] = "Horolog_token1",
  [aux_sym_Horolog_token2] = "Horolog_token2",
  [aux_sym_Io_token1] = "Io_token1",
  [aux_sym_Job_token1] = "Job_token1",
  [aux_sym_Key_token1] = "Key_token1",
  [aux_sym_Key_token2] = "Key_token2",
  [aux_sym_Principal_token1] = "Principal_token1",
  [aux_sym_Quit_token1] = "Quit_token1",
  [aux_sym_Reference_token1] = "Reference_token1",
  [aux_sym_System_token1] = "System_token1",
  [aux_sym_System_token2] = "System_token2",
  [aux_sym_Storage_token1] = "Storage_token1",
  [aux_sym_Test_token1] = "Test_token1",
  [aux_sym_X_token1] = "X_token1",
  [aux_sym_Y_token1] = "Y_token1",
  [anon_sym_PERCENT] = "%",
  [anon_sym_DIRECTORY] = "DIRECTORY",
  [anon_sym_HOST] = "HOST",
  [anon_sym_FILE] = "FILE",
  [anon_sym_ERRMSG] = "ERRMSG",
  [anon_sym_OPCOM] = "OPCOM",
  [anon_sym_SIGNAL] = "SIGNAL",
  [anon_sym_SPAWN] = "SPAWN",
  [anon_sym_VERSION] = "VERSION",
  [anon_sym_ZWRITE] = "ZWRITE",
  [sym_E] = "E",
  [sym_Paschk] = "Paschk",
  [sym_V] = "V",
  [sym_Xrsm] = "Xrsm",
  [anon_sym_SETENV] = "SETENV",
  [anon_sym_GETENV] = "GETENV",
  [anon_sym_ROUCHK] = "ROUCHK",
  [anon_sym_FORK] = "FORK",
  [anon_sym_IC] = "IC",
  [anon_sym_WAIT] = "WAIT",
  [sym_Debug] = "Debug",
  [anon_sym_COMPRESS] = "COMPRESS",
  [sym_XCallX] = "XCallX",
  [anon_sym_DOLLAR_AMP] = "$&",
  [aux_sym_Write_token1] = "Write_token1",
  [aux_sym_Write_token2] = "Write_token2",
  [sym_source_file] = "source_file",
  [sym_commandArg] = "commandArg",
  [sym_line] = "line",
  [sym__Tag] = "_Tag",
  [sym_ExtrinsicFunction] = "ExtrinsicFunction",
  [sym_ByRef] = "ByRef",
  [sym__FunctionArg] = "_FunctionArg",
  [sym_number] = "number",
  [sym_string] = "string",
  [sym_Expression] = "Expression",
  [sym_InderectExpression] = "InderectExpression",
  [sym_UnaryExpression] = "UnaryExpression",
  [sym_BinaryExpression] = "BinaryExpression",
  [sym_PaternMatchExpression] = "PaternMatchExpression",
  [sym_OPADD] = "OPADD",
  [sym_OPSUB] = "OPSUB",
  [sym_OPFOL] = "OPFOL",
  [sym_BinaryOpp] = "BinaryOpp",
  [sym_OPNOT] = "OPNOT",
  [sym_OPPLUS] = "OPPLUS",
  [sym_OPMINUS] = "OPMINUS",
  [sym_UnaryOpp] = "UnaryOpp",
  [sym_PatternOpp] = "PatternOpp",
  [sym_Patern] = "Patern",
  [sym_PaternRepetitionCount] = "PaternRepetitionCount",
  [sym_PaternElement] = "PaternElement",
  [sym_IndirectVariable] = "IndirectVariable",
  [sym_NakedVariable] = "NakedVariable",
  [sym_GlobalVariable] = "GlobalVariable",
  [sym_GlobalUciVariable] = "GlobalUciVariable",
  [sym_GlobalUciEnvVariable] = "GlobalUciEnvVariable",
  [sym__VariableSubscripts] = "_VariableSubscripts",
  [sym_Variable] = "Variable",
  [sym_Select] = "Select",
  [sym_View] = "View",
  [sym_Text] = "Text",
  [sym_Translate] = "Translate",
  [sym_Find] = "Find",
  [sym_Fnumber] = "Fnumber",
  [sym_Random] = "Random",
  [sym_Reverse] = "Reverse",
  [sym_Piece] = "Piece",
  [sym_Justify] = "Justify",
  [sym_Extract] = "Extract",
  [sym_Ascii] = "Ascii",
  [sym_Length] = "Length",
  [sym_Stack] = "Stack",
  [sym_Name] = "Name",
  [sym_Order] = "Order",
  [sym_Query] = "Query",
  [sym_Get] = "Get",
  [sym_Increment] = "Increment",
  [sym_Next] = "Next",
  [sym_Data] = "Data",
  [sym_Qlength] = "Qlength",
  [sym_Qsubscript] = "Qsubscript",
  [sym_VarFunctions] = "VarFunctions",
  [sym_Char] = "Char",
  [sym_ExpFunctions] = "ExpFunctions",
  [sym_IntrinsicFunction] = "IntrinsicFunction",
  [sym_Device] = "Device",
  [sym_Ecode] = "Ecode",
  [sym_Estack] = "Estack",
  [sym_Etrap] = "Etrap",
  [sym_Horolog] = "Horolog",
  [sym_Io] = "Io",
  [sym_Job] = "Job",
  [sym_Key] = "Key",
  [sym_Principal] = "Principal",
  [sym_Quit] = "Quit",
  [sym_Reference] = "Reference",
  [sym_System] = "System",
  [sym_Storage] = "Storage",
  [sym_Test] = "Test",
  [sym_X] = "X",
  [sym_Y] = "Y",
  [sym_StackVar] = "StackVar",
  [sym_IntrinsicVar] = "IntrinsicVar",
  [sym_Directory] = "Directory",
  [sym_Host] = "Host",
  [sym_File] = "File",
  [sym_ErrMsg] = "ErrMsg",
  [sym_OpCom] = "OpCom",
  [sym_Signal] = "Signal",
  [sym_Spawn] = "Spawn",
  [sym_Version] = "Version",
  [sym_Zwrite] = "Zwrite",
  [sym_SetEnv] = "SetEnv",
  [sym_GetEnv] = "GetEnv",
  [sym_RouChk] = "RouChk",
  [sym_Fork] = "Fork",
  [sym_IC] = "IC",
  [sym_Wait] = "Wait",
  [sym_Compress] = "Compress",
  [sym_XCall] = "XCall",
  [sym_Write] = "Write",
  [sym_command] = "command",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_line_repeat1] = "line_repeat1",
  [aux_sym_ExtrinsicFunction_repeat1] = "ExtrinsicFunction_repeat1",
  [aux_sym_string_repeat1] = "string_repeat1",
  [aux_sym_Patern_repeat1] = "Patern_repeat1",
  [aux_sym_PaternElement_repeat1] = "PaternElement_repeat1",
  [aux_sym__VariableSubscripts_repeat1] = "_VariableSubscripts_repeat1",
  [aux_sym_Select_repeat1] = "Select_repeat1",
  [aux_sym_Char_repeat1] = "Char_repeat1",
  [aux_sym_Write_repeat1] = "Write_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_LF] = anon_sym_LF,
  [anon_sym_] = anon_sym_,
  [sym_NumericIdentifier] = sym_NumericIdentifier,
  [anon_sym_DOLLAR_DOLLAR] = anon_sym_DOLLAR_DOLLAR,
  [anon_sym_CARET] = anon_sym_CARET,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_VarUndefined] = sym_VarUndefined,
  [anon_sym_DOT] = anon_sym_DOT,
  [sym_identifier] = sym_identifier,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [aux_sym_number_token1] = aux_sym_number_token1,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_token1] = aux_sym_string_token1,
  [anon_sym_DQUOTE_DQUOTE] = anon_sym_DQUOTE_DQUOTE,
  [anon_sym_AT] = anon_sym_AT,
  [sym_OPMUL] = sym_OPMUL,
  [sym_OPDIV] = sym_OPDIV,
  [sym_OPINT] = sym_OPINT,
  [sym_OPMOD] = sym_OPMOD,
  [sym_OPCAT] = sym_OPCAT,
  [sym_OPGTR] = sym_OPGTR,
  [sym_OPAND] = sym_OPAND,
  [sym_OPCON] = sym_OPCON,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [sym_OPEQL] = sym_OPEQL,
  [sym_OPLES] = sym_OPLES,
  [sym_OPPOW] = sym_OPPOW,
  [sym_OPNEQL] = sym_OPNEQL,
  [sym_OPNLES] = sym_OPNLES,
  [sym_OPNGTR] = sym_OPNGTR,
  [sym_OPNAND] = sym_OPNAND,
  [sym_OPNCON] = sym_OPNCON,
  [sym_OPNFOL] = sym_OPNFOL,
  [sym_OPSAF] = sym_OPSAF,
  [sym_OPNSAF] = sym_OPNSAF,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [sym_OPPAT] = sym_OPPAT,
  [sym_OPNPAT] = sym_OPNPAT,
  [aux_sym_PaternRepetitionCount_token1] = aux_sym_PaternRepetitionCount_token1,
  [aux_sym_PaternRepetitionCount_token2] = aux_sym_PaternRepetitionCount_token2,
  [aux_sym_PaternElement_token1] = aux_sym_PaternElement_token1,
  [anon_sym_CARET_PIPE] = anon_sym_CARET_PIPE,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_CARET_LBRACK] = anon_sym_CARET_LBRACK,
  [aux_sym_Select_token1] = aux_sym_Select_token1,
  [aux_sym_Select_token2] = aux_sym_Select_token2,
  [anon_sym_COLON] = anon_sym_COLON,
  [aux_sym_View_token1] = aux_sym_View_token1,
  [aux_sym_View_token2] = aux_sym_View_token2,
  [aux_sym_Text_token1] = aux_sym_Text_token1,
  [aux_sym_Text_token2] = aux_sym_Text_token2,
  [aux_sym_Translate_token1] = aux_sym_Translate_token1,
  [aux_sym_Translate_token2] = aux_sym_Translate_token2,
  [aux_sym_Find_token1] = aux_sym_Find_token1,
  [aux_sym_Find_token2] = aux_sym_Find_token2,
  [aux_sym_Fnumber_token1] = aux_sym_Fnumber_token1,
  [aux_sym_Fnumber_token2] = aux_sym_Fnumber_token2,
  [aux_sym_Random_token1] = aux_sym_Random_token1,
  [aux_sym_Random_token2] = aux_sym_Random_token2,
  [aux_sym_Reverse_token1] = aux_sym_Reverse_token1,
  [aux_sym_Reverse_token2] = aux_sym_Reverse_token2,
  [aux_sym_Piece_token1] = aux_sym_Piece_token1,
  [aux_sym_Piece_token2] = aux_sym_Piece_token2,
  [aux_sym_Justify_token1] = aux_sym_Justify_token1,
  [aux_sym_Justify_token2] = aux_sym_Justify_token2,
  [aux_sym_Extract_token1] = aux_sym_Extract_token1,
  [aux_sym_Extract_token2] = aux_sym_Extract_token2,
  [aux_sym_Ascii_token1] = aux_sym_Ascii_token1,
  [aux_sym_Ascii_token2] = aux_sym_Ascii_token2,
  [aux_sym_Length_token1] = aux_sym_Length_token1,
  [aux_sym_Length_token2] = aux_sym_Length_token2,
  [aux_sym_Stack_token1] = aux_sym_Stack_token1,
  [aux_sym_Stack_token2] = aux_sym_Stack_token2,
  [aux_sym_Name_token1] = aux_sym_Name_token1,
  [aux_sym_Name_token2] = aux_sym_Name_token2,
  [aux_sym_Order_token1] = aux_sym_Order_token1,
  [aux_sym_Order_token2] = aux_sym_Order_token2,
  [aux_sym_Query_token1] = aux_sym_Query_token1,
  [aux_sym_Query_token2] = aux_sym_Query_token2,
  [aux_sym_Get_token1] = aux_sym_Get_token1,
  [aux_sym_Get_token2] = aux_sym_Get_token2,
  [aux_sym_Increment_token1] = aux_sym_Increment_token1,
  [aux_sym_Increment_token2] = aux_sym_Increment_token2,
  [aux_sym_Next_token1] = aux_sym_Next_token1,
  [aux_sym_Next_token2] = aux_sym_Next_token2,
  [aux_sym_Data_token1] = aux_sym_Data_token1,
  [aux_sym_Data_token2] = aux_sym_Data_token2,
  [aux_sym_Qlength_token1] = aux_sym_Qlength_token1,
  [aux_sym_Qlength_token2] = aux_sym_Qlength_token2,
  [aux_sym_Qsubscript_token1] = aux_sym_Qsubscript_token1,
  [aux_sym_Qsubscript_token2] = aux_sym_Qsubscript_token2,
  [aux_sym_Char_token1] = aux_sym_Char_token1,
  [aux_sym_Char_token2] = aux_sym_Char_token2,
  [anon_sym_DOLLAR] = anon_sym_DOLLAR,
  [aux_sym_Device_token1] = aux_sym_Device_token1,
  [aux_sym_Ecode_token1] = aux_sym_Ecode_token1,
  [aux_sym_Ecode_token2] = aux_sym_Ecode_token2,
  [aux_sym_Estack_token1] = aux_sym_Estack_token1,
  [aux_sym_Estack_token2] = aux_sym_Estack_token2,
  [aux_sym_Etrap_token1] = aux_sym_Etrap_token1,
  [aux_sym_Etrap_token2] = aux_sym_Etrap_token2,
  [aux_sym_Horolog_token1] = aux_sym_Horolog_token1,
  [aux_sym_Horolog_token2] = aux_sym_Horolog_token2,
  [aux_sym_Io_token1] = aux_sym_Io_token1,
  [aux_sym_Job_token1] = aux_sym_Job_token1,
  [aux_sym_Key_token1] = aux_sym_Key_token1,
  [aux_sym_Key_token2] = aux_sym_Key_token2,
  [aux_sym_Principal_token1] = aux_sym_Principal_token1,
  [aux_sym_Quit_token1] = aux_sym_Quit_token1,
  [aux_sym_Reference_token1] = aux_sym_Reference_token1,
  [aux_sym_System_token1] = aux_sym_System_token1,
  [aux_sym_System_token2] = aux_sym_System_token2,
  [aux_sym_Storage_token1] = aux_sym_Storage_token1,
  [aux_sym_Test_token1] = aux_sym_Test_token1,
  [aux_sym_X_token1] = aux_sym_X_token1,
  [aux_sym_Y_token1] = aux_sym_Y_token1,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_DIRECTORY] = anon_sym_DIRECTORY,
  [anon_sym_HOST] = anon_sym_HOST,
  [anon_sym_FILE] = anon_sym_FILE,
  [anon_sym_ERRMSG] = anon_sym_ERRMSG,
  [anon_sym_OPCOM] = anon_sym_OPCOM,
  [anon_sym_SIGNAL] = anon_sym_SIGNAL,
  [anon_sym_SPAWN] = anon_sym_SPAWN,
  [anon_sym_VERSION] = anon_sym_VERSION,
  [anon_sym_ZWRITE] = anon_sym_ZWRITE,
  [sym_E] = sym_E,
  [sym_Paschk] = sym_Paschk,
  [sym_V] = sym_V,
  [sym_Xrsm] = sym_Xrsm,
  [anon_sym_SETENV] = anon_sym_SETENV,
  [anon_sym_GETENV] = anon_sym_GETENV,
  [anon_sym_ROUCHK] = anon_sym_ROUCHK,
  [anon_sym_FORK] = anon_sym_FORK,
  [anon_sym_IC] = anon_sym_IC,
  [anon_sym_WAIT] = anon_sym_WAIT,
  [sym_Debug] = sym_Debug,
  [anon_sym_COMPRESS] = anon_sym_COMPRESS,
  [sym_XCallX] = sym_XCallX,
  [anon_sym_DOLLAR_AMP] = anon_sym_DOLLAR_AMP,
  [aux_sym_Write_token1] = aux_sym_Write_token1,
  [aux_sym_Write_token2] = aux_sym_Write_token2,
  [sym_source_file] = sym_source_file,
  [sym_commandArg] = sym_commandArg,
  [sym_line] = sym_line,
  [sym__Tag] = sym__Tag,
  [sym_ExtrinsicFunction] = sym_ExtrinsicFunction,
  [sym_ByRef] = sym_ByRef,
  [sym__FunctionArg] = sym__FunctionArg,
  [sym_number] = sym_number,
  [sym_string] = sym_string,
  [sym_Expression] = sym_Expression,
  [sym_InderectExpression] = sym_InderectExpression,
  [sym_UnaryExpression] = sym_UnaryExpression,
  [sym_BinaryExpression] = sym_BinaryExpression,
  [sym_PaternMatchExpression] = sym_PaternMatchExpression,
  [sym_OPADD] = sym_OPADD,
  [sym_OPSUB] = sym_OPSUB,
  [sym_OPFOL] = sym_OPFOL,
  [sym_BinaryOpp] = sym_BinaryOpp,
  [sym_OPNOT] = sym_OPNOT,
  [sym_OPPLUS] = sym_OPPLUS,
  [sym_OPMINUS] = sym_OPMINUS,
  [sym_UnaryOpp] = sym_UnaryOpp,
  [sym_PatternOpp] = sym_PatternOpp,
  [sym_Patern] = sym_Patern,
  [sym_PaternRepetitionCount] = sym_PaternRepetitionCount,
  [sym_PaternElement] = sym_PaternElement,
  [sym_IndirectVariable] = sym_IndirectVariable,
  [sym_NakedVariable] = sym_NakedVariable,
  [sym_GlobalVariable] = sym_GlobalVariable,
  [sym_GlobalUciVariable] = sym_GlobalUciVariable,
  [sym_GlobalUciEnvVariable] = sym_GlobalUciEnvVariable,
  [sym__VariableSubscripts] = sym__VariableSubscripts,
  [sym_Variable] = sym_Variable,
  [sym_Select] = sym_Select,
  [sym_View] = sym_View,
  [sym_Text] = sym_Text,
  [sym_Translate] = sym_Translate,
  [sym_Find] = sym_Find,
  [sym_Fnumber] = sym_Fnumber,
  [sym_Random] = sym_Random,
  [sym_Reverse] = sym_Reverse,
  [sym_Piece] = sym_Piece,
  [sym_Justify] = sym_Justify,
  [sym_Extract] = sym_Extract,
  [sym_Ascii] = sym_Ascii,
  [sym_Length] = sym_Length,
  [sym_Stack] = sym_Stack,
  [sym_Name] = sym_Name,
  [sym_Order] = sym_Order,
  [sym_Query] = sym_Query,
  [sym_Get] = sym_Get,
  [sym_Increment] = sym_Increment,
  [sym_Next] = sym_Next,
  [sym_Data] = sym_Data,
  [sym_Qlength] = sym_Qlength,
  [sym_Qsubscript] = sym_Qsubscript,
  [sym_VarFunctions] = sym_VarFunctions,
  [sym_Char] = sym_Char,
  [sym_ExpFunctions] = sym_ExpFunctions,
  [sym_IntrinsicFunction] = sym_IntrinsicFunction,
  [sym_Device] = sym_Device,
  [sym_Ecode] = sym_Ecode,
  [sym_Estack] = sym_Estack,
  [sym_Etrap] = sym_Etrap,
  [sym_Horolog] = sym_Horolog,
  [sym_Io] = sym_Io,
  [sym_Job] = sym_Job,
  [sym_Key] = sym_Key,
  [sym_Principal] = sym_Principal,
  [sym_Quit] = sym_Quit,
  [sym_Reference] = sym_Reference,
  [sym_System] = sym_System,
  [sym_Storage] = sym_Storage,
  [sym_Test] = sym_Test,
  [sym_X] = sym_X,
  [sym_Y] = sym_Y,
  [sym_StackVar] = sym_StackVar,
  [sym_IntrinsicVar] = sym_IntrinsicVar,
  [sym_Directory] = sym_Directory,
  [sym_Host] = sym_Host,
  [sym_File] = sym_File,
  [sym_ErrMsg] = sym_ErrMsg,
  [sym_OpCom] = sym_OpCom,
  [sym_Signal] = sym_Signal,
  [sym_Spawn] = sym_Spawn,
  [sym_Version] = sym_Version,
  [sym_Zwrite] = sym_Zwrite,
  [sym_SetEnv] = sym_SetEnv,
  [sym_GetEnv] = sym_GetEnv,
  [sym_RouChk] = sym_RouChk,
  [sym_Fork] = sym_Fork,
  [sym_IC] = sym_IC,
  [sym_Wait] = sym_Wait,
  [sym_Compress] = sym_Compress,
  [sym_XCall] = sym_XCall,
  [sym_Write] = sym_Write,
  [sym_command] = sym_command,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_line_repeat1] = aux_sym_line_repeat1,
  [aux_sym_ExtrinsicFunction_repeat1] = aux_sym_ExtrinsicFunction_repeat1,
  [aux_sym_string_repeat1] = aux_sym_string_repeat1,
  [aux_sym_Patern_repeat1] = aux_sym_Patern_repeat1,
  [aux_sym_PaternElement_repeat1] = aux_sym_PaternElement_repeat1,
  [aux_sym__VariableSubscripts_repeat1] = aux_sym__VariableSubscripts_repeat1,
  [aux_sym_Select_repeat1] = aux_sym_Select_repeat1,
  [aux_sym_Char_repeat1] = aux_sym_Char_repeat1,
  [aux_sym_Write_repeat1] = aux_sym_Write_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_LF] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_] = {
    .visible = true,
    .named = false,
  },
  [sym_NumericIdentifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DOLLAR_DOLLAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_VarUndefined] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_number_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DQUOTE_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [sym_OPMUL] = {
    .visible = true,
    .named = true,
  },
  [sym_OPDIV] = {
    .visible = true,
    .named = true,
  },
  [sym_OPINT] = {
    .visible = true,
    .named = true,
  },
  [sym_OPMOD] = {
    .visible = true,
    .named = true,
  },
  [sym_OPCAT] = {
    .visible = true,
    .named = true,
  },
  [sym_OPGTR] = {
    .visible = true,
    .named = true,
  },
  [sym_OPAND] = {
    .visible = true,
    .named = true,
  },
  [sym_OPCON] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [sym_OPEQL] = {
    .visible = true,
    .named = true,
  },
  [sym_OPLES] = {
    .visible = true,
    .named = true,
  },
  [sym_OPPOW] = {
    .visible = true,
    .named = true,
  },
  [sym_OPNEQL] = {
    .visible = true,
    .named = true,
  },
  [sym_OPNLES] = {
    .visible = true,
    .named = true,
  },
  [sym_OPNGTR] = {
    .visible = true,
    .named = true,
  },
  [sym_OPNAND] = {
    .visible = true,
    .named = true,
  },
  [sym_OPNCON] = {
    .visible = true,
    .named = true,
  },
  [sym_OPNFOL] = {
    .visible = true,
    .named = true,
  },
  [sym_OPSAF] = {
    .visible = true,
    .named = true,
  },
  [sym_OPNSAF] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [sym_OPPAT] = {
    .visible = true,
    .named = true,
  },
  [sym_OPNPAT] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_PaternRepetitionCount_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_PaternRepetitionCount_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_PaternElement_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_CARET_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_Select_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Select_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_View_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_View_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Text_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Text_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Translate_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Translate_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Find_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Find_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Fnumber_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Fnumber_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Random_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Random_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Reverse_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Reverse_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Piece_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Piece_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Justify_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Justify_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Extract_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Extract_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Ascii_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Ascii_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Length_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Length_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Stack_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Stack_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Name_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Name_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Order_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Order_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Query_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Query_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Get_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Get_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Increment_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Increment_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Next_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Next_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Data_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Data_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Qlength_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Qlength_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Qsubscript_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Qsubscript_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Char_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Char_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DOLLAR] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_Device_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Ecode_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Ecode_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Estack_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Estack_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Etrap_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Etrap_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Horolog_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Horolog_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Io_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Job_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Key_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Key_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Principal_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Quit_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Reference_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_System_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_System_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Storage_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Test_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_X_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Y_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DIRECTORY] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_HOST] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_FILE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ERRMSG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_OPCOM] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SIGNAL] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SPAWN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_VERSION] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ZWRITE] = {
    .visible = true,
    .named = false,
  },
  [sym_E] = {
    .visible = true,
    .named = true,
  },
  [sym_Paschk] = {
    .visible = true,
    .named = true,
  },
  [sym_V] = {
    .visible = true,
    .named = true,
  },
  [sym_Xrsm] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_SETENV] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GETENV] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ROUCHK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_FORK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_IC] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_WAIT] = {
    .visible = true,
    .named = false,
  },
  [sym_Debug] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COMPRESS] = {
    .visible = true,
    .named = false,
  },
  [sym_XCallX] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DOLLAR_AMP] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_Write_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Write_token2] = {
    .visible = false,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_commandArg] = {
    .visible = true,
    .named = true,
  },
  [sym_line] = {
    .visible = true,
    .named = true,
  },
  [sym__Tag] = {
    .visible = false,
    .named = true,
  },
  [sym_ExtrinsicFunction] = {
    .visible = true,
    .named = true,
  },
  [sym_ByRef] = {
    .visible = true,
    .named = true,
  },
  [sym__FunctionArg] = {
    .visible = false,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_Expression] = {
    .visible = true,
    .named = true,
  },
  [sym_InderectExpression] = {
    .visible = true,
    .named = true,
  },
  [sym_UnaryExpression] = {
    .visible = true,
    .named = true,
  },
  [sym_BinaryExpression] = {
    .visible = true,
    .named = true,
  },
  [sym_PaternMatchExpression] = {
    .visible = true,
    .named = true,
  },
  [sym_OPADD] = {
    .visible = true,
    .named = true,
  },
  [sym_OPSUB] = {
    .visible = true,
    .named = true,
  },
  [sym_OPFOL] = {
    .visible = true,
    .named = true,
  },
  [sym_BinaryOpp] = {
    .visible = true,
    .named = true,
  },
  [sym_OPNOT] = {
    .visible = true,
    .named = true,
  },
  [sym_OPPLUS] = {
    .visible = true,
    .named = true,
  },
  [sym_OPMINUS] = {
    .visible = true,
    .named = true,
  },
  [sym_UnaryOpp] = {
    .visible = true,
    .named = true,
  },
  [sym_PatternOpp] = {
    .visible = true,
    .named = true,
  },
  [sym_Patern] = {
    .visible = true,
    .named = true,
  },
  [sym_PaternRepetitionCount] = {
    .visible = true,
    .named = true,
  },
  [sym_PaternElement] = {
    .visible = true,
    .named = true,
  },
  [sym_IndirectVariable] = {
    .visible = true,
    .named = true,
  },
  [sym_NakedVariable] = {
    .visible = true,
    .named = true,
  },
  [sym_GlobalVariable] = {
    .visible = true,
    .named = true,
  },
  [sym_GlobalUciVariable] = {
    .visible = true,
    .named = true,
  },
  [sym_GlobalUciEnvVariable] = {
    .visible = true,
    .named = true,
  },
  [sym__VariableSubscripts] = {
    .visible = false,
    .named = true,
  },
  [sym_Variable] = {
    .visible = true,
    .named = true,
  },
  [sym_Select] = {
    .visible = true,
    .named = true,
  },
  [sym_View] = {
    .visible = true,
    .named = true,
  },
  [sym_Text] = {
    .visible = true,
    .named = true,
  },
  [sym_Translate] = {
    .visible = true,
    .named = true,
  },
  [sym_Find] = {
    .visible = true,
    .named = true,
  },
  [sym_Fnumber] = {
    .visible = true,
    .named = true,
  },
  [sym_Random] = {
    .visible = true,
    .named = true,
  },
  [sym_Reverse] = {
    .visible = true,
    .named = true,
  },
  [sym_Piece] = {
    .visible = true,
    .named = true,
  },
  [sym_Justify] = {
    .visible = true,
    .named = true,
  },
  [sym_Extract] = {
    .visible = true,
    .named = true,
  },
  [sym_Ascii] = {
    .visible = true,
    .named = true,
  },
  [sym_Length] = {
    .visible = true,
    .named = true,
  },
  [sym_Stack] = {
    .visible = true,
    .named = true,
  },
  [sym_Name] = {
    .visible = true,
    .named = true,
  },
  [sym_Order] = {
    .visible = true,
    .named = true,
  },
  [sym_Query] = {
    .visible = true,
    .named = true,
  },
  [sym_Get] = {
    .visible = true,
    .named = true,
  },
  [sym_Increment] = {
    .visible = true,
    .named = true,
  },
  [sym_Next] = {
    .visible = true,
    .named = true,
  },
  [sym_Data] = {
    .visible = true,
    .named = true,
  },
  [sym_Qlength] = {
    .visible = true,
    .named = true,
  },
  [sym_Qsubscript] = {
    .visible = true,
    .named = true,
  },
  [sym_VarFunctions] = {
    .visible = true,
    .named = true,
  },
  [sym_Char] = {
    .visible = true,
    .named = true,
  },
  [sym_ExpFunctions] = {
    .visible = true,
    .named = true,
  },
  [sym_IntrinsicFunction] = {
    .visible = true,
    .named = true,
  },
  [sym_Device] = {
    .visible = true,
    .named = true,
  },
  [sym_Ecode] = {
    .visible = true,
    .named = true,
  },
  [sym_Estack] = {
    .visible = true,
    .named = true,
  },
  [sym_Etrap] = {
    .visible = true,
    .named = true,
  },
  [sym_Horolog] = {
    .visible = true,
    .named = true,
  },
  [sym_Io] = {
    .visible = true,
    .named = true,
  },
  [sym_Job] = {
    .visible = true,
    .named = true,
  },
  [sym_Key] = {
    .visible = true,
    .named = true,
  },
  [sym_Principal] = {
    .visible = true,
    .named = true,
  },
  [sym_Quit] = {
    .visible = true,
    .named = true,
  },
  [sym_Reference] = {
    .visible = true,
    .named = true,
  },
  [sym_System] = {
    .visible = true,
    .named = true,
  },
  [sym_Storage] = {
    .visible = true,
    .named = true,
  },
  [sym_Test] = {
    .visible = true,
    .named = true,
  },
  [sym_X] = {
    .visible = true,
    .named = true,
  },
  [sym_Y] = {
    .visible = true,
    .named = true,
  },
  [sym_StackVar] = {
    .visible = true,
    .named = true,
  },
  [sym_IntrinsicVar] = {
    .visible = true,
    .named = true,
  },
  [sym_Directory] = {
    .visible = true,
    .named = true,
  },
  [sym_Host] = {
    .visible = true,
    .named = true,
  },
  [sym_File] = {
    .visible = true,
    .named = true,
  },
  [sym_ErrMsg] = {
    .visible = true,
    .named = true,
  },
  [sym_OpCom] = {
    .visible = true,
    .named = true,
  },
  [sym_Signal] = {
    .visible = true,
    .named = true,
  },
  [sym_Spawn] = {
    .visible = true,
    .named = true,
  },
  [sym_Version] = {
    .visible = true,
    .named = true,
  },
  [sym_Zwrite] = {
    .visible = true,
    .named = true,
  },
  [sym_SetEnv] = {
    .visible = true,
    .named = true,
  },
  [sym_GetEnv] = {
    .visible = true,
    .named = true,
  },
  [sym_RouChk] = {
    .visible = true,
    .named = true,
  },
  [sym_Fork] = {
    .visible = true,
    .named = true,
  },
  [sym_IC] = {
    .visible = true,
    .named = true,
  },
  [sym_Wait] = {
    .visible = true,
    .named = true,
  },
  [sym_Compress] = {
    .visible = true,
    .named = true,
  },
  [sym_XCall] = {
    .visible = true,
    .named = true,
  },
  [sym_Write] = {
    .visible = true,
    .named = true,
  },
  [sym_command] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_line_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_ExtrinsicFunction_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Patern_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_PaternElement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__VariableSubscripts_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Select_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Char_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_Write_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_args = 1,
  field_code = 2,
  field_exp = 3,
  field_exp_left = 4,
  field_exp_right = 5,
  field_heading = 6,
  field_name = 7,
  field_opp = 8,
  field_routine = 9,
  field_subs = 10,
  field_tag = 11,
  field_var = 12,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_args] = "args",
  [field_code] = "code",
  [field_exp] = "exp",
  [field_exp_left] = "exp_left",
  [field_exp_right] = "exp_right",
  [field_heading] = "heading",
  [field_name] = "name",
  [field_opp] = "opp",
  [field_routine] = "routine",
  [field_subs] = "subs",
  [field_tag] = "tag",
  [field_var] = "var",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
  [3] = {.index = 3, .length = 2},
  [4] = {.index = 5, .length = 2},
  [5] = {.index = 7, .length = 2},
  [6] = {.index = 9, .length = 3},
  [7] = {.index = 12, .length = 3},
  [8] = {.index = 15, .length = 1},
  [9] = {.index = 16, .length = 3},
  [10] = {.index = 19, .length = 1},
  [11] = {.index = 20, .length = 2},
  [12] = {.index = 22, .length = 1},
  [13] = {.index = 23, .length = 1},
  [14] = {.index = 24, .length = 2},
  [15] = {.index = 26, .length = 2},
  [16] = {.index = 28, .length = 2},
  [17] = {.index = 30, .length = 1},
  [18] = {.index = 31, .length = 3},
  [19] = {.index = 34, .length = 2},
  [20] = {.index = 36, .length = 2},
  [21] = {.index = 38, .length = 3},
  [22] = {.index = 41, .length = 3},
  [23] = {.index = 44, .length = 2},
  [24] = {.index = 46, .length = 2},
  [25] = {.index = 48, .length = 3},
  [26] = {.index = 51, .length = 4},
  [27] = {.index = 55, .length = 3},
  [28] = {.index = 58, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 0},
  [1] =
    {field_name, 0},
    {field_subs, 1},
  [3] =
    {field_exp, 1},
    {field_opp, 0},
  [5] =
    {field_heading, 0},
    {field_subs, 1},
  [7] =
    {field_heading, 0},
    {field_name, 1},
  [9] =
    {field_exp_left, 0},
    {field_exp_right, 2},
    {field_opp, 1},
  [12] =
    {field_heading, 0},
    {field_name, 1},
    {field_subs, 2},
  [15] =
    {field_tag, 1},
  [16] =
    {field_exp_left, 0},
    {field_exp_right, 3},
    {field_opp, 1},
  [19] =
    {field_routine, 2},
  [20] =
    {field_args, 3},
    {field_tag, 1},
  [22] =
    {field_args, 2},
  [23] =
    {field_var, 2},
  [24] =
    {field_args, 3},
    {field_code, 1},
  [26] =
    {field_args, 4},
    {field_routine, 2},
  [28] =
    {field_routine, 3},
    {field_tag, 1},
  [30] =
    {field_args, 1},
  [31] =
    {field_args, 3},
    {field_args, 4, .inherited = true},
    {field_tag, 1},
  [34] =
    {field_args, 0, .inherited = true},
    {field_args, 1, .inherited = true},
  [36] =
    {field_args, 2},
    {field_args, 3, .inherited = true},
  [38] =
    {field_args, 4},
    {field_args, 5, .inherited = true},
    {field_routine, 2},
  [41] =
    {field_args, 5},
    {field_routine, 3},
    {field_tag, 1},
  [44] =
    {field_args, 2},
    {field_args, 4},
  [46] =
    {field_args, 4},
    {field_var, 2},
  [48] =
    {field_args, 3},
    {field_args, 5},
    {field_code, 1},
  [51] =
    {field_args, 5},
    {field_args, 6, .inherited = true},
    {field_routine, 3},
    {field_tag, 1},
  [55] =
    {field_args, 2},
    {field_args, 4},
    {field_args, 6},
  [58] =
    {field_args, 2},
    {field_args, 4},
    {field_args, 6},
    {field_args, 8},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 14,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 17,
  [24] = 12,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 11,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 25,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 32,
  [73] = 13,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 76,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 149,
  [150] = 150,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 157,
  [158] = 158,
  [159] = 159,
  [160] = 160,
  [161] = 161,
  [162] = 162,
  [163] = 163,
  [164] = 164,
  [165] = 165,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 169,
  [170] = 170,
  [171] = 171,
  [172] = 172,
  [173] = 173,
  [174] = 174,
  [175] = 175,
  [176] = 176,
  [177] = 177,
  [178] = 178,
  [179] = 179,
  [180] = 180,
  [181] = 181,
  [182] = 182,
  [183] = 183,
  [184] = 184,
  [185] = 185,
  [186] = 186,
  [187] = 187,
  [188] = 188,
  [189] = 189,
  [190] = 190,
  [191] = 191,
  [192] = 192,
  [193] = 193,
  [194] = 194,
  [195] = 195,
  [196] = 196,
  [197] = 197,
  [198] = 198,
  [199] = 199,
  [200] = 200,
  [201] = 201,
  [202] = 202,
  [203] = 203,
  [204] = 204,
  [205] = 205,
  [206] = 206,
  [207] = 207,
  [208] = 208,
  [209] = 209,
  [210] = 210,
  [211] = 211,
  [212] = 212,
  [213] = 213,
  [214] = 214,
  [215] = 215,
  [216] = 216,
  [217] = 217,
  [218] = 218,
  [219] = 219,
  [220] = 220,
  [221] = 221,
  [222] = 222,
  [223] = 223,
  [224] = 224,
  [225] = 225,
  [226] = 226,
  [227] = 227,
  [228] = 228,
  [229] = 229,
  [230] = 230,
  [231] = 231,
  [232] = 232,
  [233] = 233,
  [234] = 234,
  [235] = 235,
  [236] = 236,
  [237] = 237,
  [238] = 238,
  [239] = 239,
  [240] = 240,
  [241] = 241,
  [242] = 242,
  [243] = 243,
  [244] = 244,
  [245] = 245,
  [246] = 246,
  [247] = 247,
  [248] = 248,
  [249] = 249,
  [250] = 250,
  [251] = 251,
  [252] = 252,
  [253] = 253,
  [254] = 254,
  [255] = 255,
  [256] = 256,
  [257] = 257,
  [258] = 258,
  [259] = 259,
  [260] = 260,
  [261] = 261,
  [262] = 262,
  [263] = 263,
  [264] = 264,
  [265] = 265,
  [266] = 264,
  [267] = 267,
  [268] = 268,
  [269] = 269,
  [270] = 270,
  [271] = 271,
  [272] = 272,
  [273] = 273,
  [274] = 274,
  [275] = 275,
  [276] = 276,
  [277] = 277,
  [278] = 278,
  [279] = 279,
  [280] = 280,
  [281] = 281,
  [282] = 282,
  [283] = 283,
  [284] = 284,
  [285] = 285,
  [286] = 286,
  [287] = 287,
  [288] = 288,
  [289] = 289,
  [290] = 290,
  [291] = 291,
  [292] = 292,
  [293] = 293,
  [294] = 294,
  [295] = 295,
  [296] = 296,
  [297] = 297,
  [298] = 298,
  [299] = 299,
  [300] = 300,
  [301] = 301,
  [302] = 302,
  [303] = 303,
  [304] = 304,
  [305] = 305,
  [306] = 306,
  [307] = 307,
  [308] = 308,
  [309] = 309,
  [310] = 310,
  [311] = 311,
  [312] = 312,
  [313] = 313,
  [314] = 314,
  [315] = 315,
  [316] = 316,
  [317] = 317,
  [318] = 318,
  [319] = 319,
  [320] = 320,
  [321] = 321,
  [322] = 322,
  [323] = 323,
  [324] = 324,
  [325] = 325,
  [326] = 326,
  [327] = 327,
  [328] = 328,
  [329] = 329,
  [330] = 330,
  [331] = 331,
  [332] = 332,
  [333] = 333,
  [334] = 334,
  [335] = 335,
  [336] = 336,
  [337] = 337,
  [338] = 338,
  [339] = 339,
  [340] = 340,
  [341] = 341,
  [342] = 342,
  [343] = 343,
  [344] = 344,
  [345] = 345,
  [346] = 346,
  [347] = 347,
  [348] = 348,
  [349] = 349,
  [350] = 350,
  [351] = 351,
  [352] = 352,
  [353] = 353,
  [354] = 354,
  [355] = 355,
  [356] = 356,
  [357] = 357,
  [358] = 358,
  [359] = 359,
  [360] = 360,
  [361] = 361,
  [362] = 362,
  [363] = 363,
  [364] = 364,
  [365] = 365,
  [366] = 366,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(213);
      if (lookahead == '\n') ADVANCE(214);
      if (lookahead == ' ') ADVANCE(215);
      if (lookahead == '"') ADVANCE(485);
      if (lookahead == '#') ADVANCE(493);
      if (lookahead == '$') ADVANCE(569);
      if (lookahead == '%') ADVANCE(592);
      if (lookahead == '&') ADVANCE(496);
      if (lookahead == '\'') ADVANCE(511);
      if (lookahead == '(') ADVANCE(283);
      if (lookahead == ')') ADVANCE(285);
      if (lookahead == '*') ADVANCE(490);
      if (lookahead == '+') ADVANCE(481);
      if (lookahead == ',') ADVANCE(284);
      if (lookahead == '-') ADVANCE(482);
      if (lookahead == '.') ADVANCE(289);
      if (lookahead == '/') ADVANCE(491);
      if (lookahead == ':') ADVANCE(522);
      if (lookahead == '<') ADVANCE(500);
      if (lookahead == '=') ADVANCE(499);
      if (lookahead == '>') ADVANCE(495);
      if (lookahead == '?') ADVANCE(512);
      if (lookahead == '@') ADVANCE(489);
      if (lookahead == 'A') ADVANCE(459);
      if (lookahead == 'C') ADVANCE(407);
      if (lookahead == 'D') ADVANCE(358);
      if (lookahead == 'E') ADVANCE(612);
      if (lookahead == 'F') ADVANCE(310);
      if (lookahead == 'G') ADVANCE(298);
      if (lookahead == 'H') ADVANCE(327);
      if (lookahead == 'I') ADVANCE(294);
      if (lookahead == 'J' ||
          lookahead == 'j') ADVANCE(438);
      if (lookahead == 'K' ||
          lookahead == 'k') ADVANCE(385);
      if (lookahead == 'L') ADVANCE(480);
      if (lookahead == 'N') ADVANCE(480);
      if (lookahead == 'O') ADVANCE(331);
      if (lookahead == 'P') ADVANCE(414);
      if (lookahead == 'Q' ||
          lookahead == 'q') ADVANCE(422);
      if (lookahead == 'R') ADVANCE(368);
      if (lookahead == 'S') ADVANCE(299);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(387);
      if (lookahead == 'U') ADVANCE(480);
      if (lookahead == 'V') ADVANCE(615);
      if (lookahead == 'W') ADVANCE(290);
      if (lookahead == 'X') ADVANCE(634);
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(480);
      if (lookahead == 'Z') ADVANCE(355);
      if (lookahead == '[') ADVANCE(497);
      if (lookahead == '\\') ADVANCE(492);
      if (lookahead == ']') ADVANCE(498);
      if (lookahead == '^') ADVANCE(282);
      if (lookahead == '_') ADVANCE(494);
      if (lookahead == 'a') ADVANCE(459);
      if (lookahead == 'c') ADVANCE(408);
      if (lookahead == 'd') ADVANCE(359);
      if (lookahead == 'e') ADVANCE(466);
      if (lookahead == 'f') ADVANCE(410);
      if (lookahead == 'g') ADVANCE(391);
      if (lookahead == 'h') ADVANCE(442);
      if (lookahead == 'i') ADVANCE(431);
      if (lookahead == 'l') ADVANCE(480);
      if (lookahead == 'n') ADVANCE(480);
      if (lookahead == 'o') ADVANCE(448);
      if (lookahead == 'p') ADVANCE(414);
      if (lookahead == 'r') ADVANCE(369);
      if (lookahead == 's') ADVANCE(386);
      if (lookahead == 'v') ADVANCE(417);
      if (lookahead == 'w') ADVANCE(451);
      if (lookahead == 'x') ADVANCE(480);
      if (lookahead == '|') ADVANCE(518);
      if (('B' <= lookahead && lookahead <= 'M') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(279);
      END_STATE();
    case 1:
      if (lookahead == '%') ADVANCE(592);
      if (lookahead == 'D') ADVANCE(14);
      if (lookahead == 'E') ADVANCE(611);
      if (lookahead == 'P') ADVANCE(3);
      if (lookahead == 'V') ADVANCE(614);
      if (lookahead == 'X') ADVANCE(635);
      END_STATE();
    case 2:
      if (lookahead == '&') ADVANCE(505);
      if (lookahead == '<') ADVANCE(503);
      if (lookahead == '=') ADVANCE(502);
      if (lookahead == '>') ADVANCE(504);
      if (lookahead == '?') ADVANCE(513);
      if (lookahead == '[') ADVANCE(506);
      if (lookahead == ']') ADVANCE(507);
      END_STATE();
    case 3:
      if (lookahead == 'A') ADVANCE(64);
      END_STATE();
    case 4:
      if (lookahead == 'A') ADVANCE(31);
      END_STATE();
    case 5:
      if (lookahead == 'A') ADVANCE(81);
      END_STATE();
    case 6:
      if (lookahead == 'A') ADVANCE(37);
      END_STATE();
    case 7:
      if (lookahead == 'B') ADVANCE(77);
      END_STATE();
    case 8:
      if (lookahead == 'C') ADVANCE(27);
      END_STATE();
    case 9:
      if (lookahead == 'C') ADVANCE(49);
      if (lookahead == 'D') ADVANCE(30);
      if (lookahead == 'E') ADVANCE(60);
      if (lookahead == 'F') ADVANCE(29);
      if (lookahead == 'G') ADVANCE(15);
      if (lookahead == 'H') ADVANCE(50);
      if (lookahead == 'I') ADVANCE(10);
      if (lookahead == 'O') ADVANCE(54);
      if (lookahead == 'R') ADVANCE(48);
      if (lookahead == 'S') ADVANCE(23);
      if (lookahead == 'V') ADVANCE(21);
      if (lookahead == 'W') ADVANCE(4);
      if (lookahead == 'Z') ADVANCE(82);
      END_STATE();
    case 10:
      if (lookahead == 'C') ADVANCE(626);
      END_STATE();
    case 11:
      if (lookahead == 'C') ADVANCE(73);
      END_STATE();
    case 12:
      if (lookahead == 'C') ADVANCE(28);
      END_STATE();
    case 13:
      if (lookahead == 'C') ADVANCE(51);
      END_STATE();
    case 14:
      if (lookahead == 'E') ADVANCE(7);
      END_STATE();
    case 15:
      if (lookahead == 'E') ADVANCE(74);
      END_STATE();
    case 16:
      if (lookahead == 'E') ADVANCE(597);
      END_STATE();
    case 17:
      if (lookahead == 'E') ADVANCE(609);
      END_STATE();
    case 18:
      if (lookahead == 'E') ADVANCE(43);
      END_STATE();
    case 19:
      if (lookahead == 'E') ADVANCE(46);
      END_STATE();
    case 20:
      if (lookahead == 'E') ADVANCE(11);
      END_STATE();
    case 21:
      if (lookahead == 'E') ADVANCE(62);
      END_STATE();
    case 22:
      if (lookahead == 'E') ADVANCE(70);
      END_STATE();
    case 23:
      if (lookahead == 'E') ADVANCE(75);
      if (lookahead == 'I') ADVANCE(25);
      if (lookahead == 'P') ADVANCE(5);
      END_STATE();
    case 24:
      if (lookahead == 'G') ADVANCE(630);
      END_STATE();
    case 25:
      if (lookahead == 'G') ADVANCE(47);
      END_STATE();
    case 26:
      if (lookahead == 'G') ADVANCE(599);
      END_STATE();
    case 27:
      if (lookahead == 'H') ADVANCE(34);
      END_STATE();
    case 28:
      if (lookahead == 'H') ADVANCE(36);
      END_STATE();
    case 29:
      if (lookahead == 'I') ADVANCE(38);
      if (lookahead == 'O') ADVANCE(57);
      END_STATE();
    case 30:
      if (lookahead == 'I') ADVANCE(61);
      END_STATE();
    case 31:
      if (lookahead == 'I') ADVANCE(72);
      END_STATE();
    case 32:
      if (lookahead == 'I') ADVANCE(52);
      END_STATE();
    case 33:
      if (lookahead == 'I') ADVANCE(76);
      END_STATE();
    case 34:
      if (lookahead == 'K') ADVANCE(613);
      END_STATE();
    case 35:
      if (lookahead == 'K') ADVANCE(624);
      END_STATE();
    case 36:
      if (lookahead == 'K') ADVANCE(622);
      END_STATE();
    case 37:
      if (lookahead == 'L') ADVANCE(603);
      END_STATE();
    case 38:
      if (lookahead == 'L') ADVANCE(16);
      END_STATE();
    case 39:
      if (lookahead == 'M') ADVANCE(616);
      END_STATE();
    case 40:
      if (lookahead == 'M') ADVANCE(601);
      END_STATE();
    case 41:
      if (lookahead == 'M') ADVANCE(55);
      END_STATE();
    case 42:
      if (lookahead == 'M') ADVANCE(68);
      END_STATE();
    case 43:
      if (lookahead == 'N') ADVANCE(79);
      END_STATE();
    case 44:
      if (lookahead == 'N') ADVANCE(605);
      END_STATE();
    case 45:
      if (lookahead == 'N') ADVANCE(607);
      END_STATE();
    case 46:
      if (lookahead == 'N') ADVANCE(80);
      END_STATE();
    case 47:
      if (lookahead == 'N') ADVANCE(6);
      END_STATE();
    case 48:
      if (lookahead == 'O') ADVANCE(78);
      END_STATE();
    case 49:
      if (lookahead == 'O') ADVANCE(41);
      END_STATE();
    case 50:
      if (lookahead == 'O') ADVANCE(67);
      END_STATE();
    case 51:
      if (lookahead == 'O') ADVANCE(40);
      END_STATE();
    case 52:
      if (lookahead == 'O') ADVANCE(45);
      END_STATE();
    case 53:
      if (lookahead == 'O') ADVANCE(56);
      END_STATE();
    case 54:
      if (lookahead == 'P') ADVANCE(13);
      END_STATE();
    case 55:
      if (lookahead == 'P') ADVANCE(63);
      END_STATE();
    case 56:
      if (lookahead == 'R') ADVANCE(83);
      END_STATE();
    case 57:
      if (lookahead == 'R') ADVANCE(35);
      END_STATE();
    case 58:
      if (lookahead == 'R') ADVANCE(42);
      END_STATE();
    case 59:
      if (lookahead == 'R') ADVANCE(33);
      END_STATE();
    case 60:
      if (lookahead == 'R') ADVANCE(58);
      END_STATE();
    case 61:
      if (lookahead == 'R') ADVANCE(20);
      END_STATE();
    case 62:
      if (lookahead == 'R') ADVANCE(69);
      END_STATE();
    case 63:
      if (lookahead == 'R') ADVANCE(22);
      END_STATE();
    case 64:
      if (lookahead == 'S') ADVANCE(8);
      END_STATE();
    case 65:
      if (lookahead == 'S') ADVANCE(39);
      END_STATE();
    case 66:
      if (lookahead == 'S') ADVANCE(632);
      END_STATE();
    case 67:
      if (lookahead == 'S') ADVANCE(71);
      END_STATE();
    case 68:
      if (lookahead == 'S') ADVANCE(26);
      END_STATE();
    case 69:
      if (lookahead == 'S') ADVANCE(32);
      END_STATE();
    case 70:
      if (lookahead == 'S') ADVANCE(66);
      END_STATE();
    case 71:
      if (lookahead == 'T') ADVANCE(595);
      END_STATE();
    case 72:
      if (lookahead == 'T') ADVANCE(628);
      END_STATE();
    case 73:
      if (lookahead == 'T') ADVANCE(53);
      END_STATE();
    case 74:
      if (lookahead == 'T') ADVANCE(18);
      END_STATE();
    case 75:
      if (lookahead == 'T') ADVANCE(19);
      END_STATE();
    case 76:
      if (lookahead == 'T') ADVANCE(17);
      END_STATE();
    case 77:
      if (lookahead == 'U') ADVANCE(24);
      END_STATE();
    case 78:
      if (lookahead == 'U') ADVANCE(12);
      END_STATE();
    case 79:
      if (lookahead == 'V') ADVANCE(620);
      END_STATE();
    case 80:
      if (lookahead == 'V') ADVANCE(618);
      END_STATE();
    case 81:
      if (lookahead == 'W') ADVANCE(44);
      END_STATE();
    case 82:
      if (lookahead == 'W') ADVANCE(59);
      END_STATE();
    case 83:
      if (lookahead == 'Y') ADVANCE(593);
      END_STATE();
    case 84:
      if (lookahead == '^') ADVANCE(281);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(278);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 85:
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(561);
      END_STATE();
    case 86:
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(168);
      END_STATE();
    case 87:
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(172);
      END_STATE();
    case 88:
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(136);
      END_STATE();
    case 89:
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(149);
      END_STATE();
    case 90:
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(97);
      END_STATE();
    case 91:
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(105);
      END_STATE();
    case 92:
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(203);
      END_STATE();
    case 93:
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(580);
      END_STATE();
    case 94:
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(186);
      END_STATE();
    case 95:
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(130);
      END_STATE();
    case 96:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(147);
      END_STATE();
    case 97:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(148);
      END_STATE();
    case 98:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(144);
      END_STATE();
    case 99:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(178);
      END_STATE();
    case 100:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(141);
      END_STATE();
    case 101:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(114);
      END_STATE();
    case 102:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(179);
      END_STATE();
    case 103:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(116);
      END_STATE();
    case 104:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(193);
      END_STATE();
    case 105:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(194);
      END_STATE();
    case 106:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(119);
      END_STATE();
    case 107:
      if (lookahead == 'D' ||
          lookahead == 'd') ADVANCE(529);
      END_STATE();
    case 108:
      if (lookahead == 'D' ||
          lookahead == 'd') ADVANCE(166);
      END_STATE();
    case 109:
      if (lookahead == 'D' ||
          lookahead == 'd') ADVANCE(128);
      END_STATE();
    case 110:
      if (lookahead == 'D' ||
          lookahead == 'd') ADVANCE(113);
      END_STATE();
    case 111:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(205);
      END_STATE();
    case 112:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(549);
      END_STATE();
    case 113:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(571);
      END_STATE();
    case 114:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(537);
      END_STATE();
    case 115:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(637);
      END_STATE();
    case 116:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(570);
      END_STATE();
    case 117:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(535);
      END_STATE();
    case 118:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(588);
      END_STATE();
    case 119:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(585);
      END_STATE();
    case 120:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(527);
      END_STATE();
    case 121:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(156);
      END_STATE();
    case 122:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(101);
      END_STATE();
    case 123:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(154);
      END_STATE();
    case 124:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(164);
      END_STATE();
    case 125:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(175);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(189);
      END_STATE();
    case 126:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(104);
      END_STATE();
    case 127:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(161);
      END_STATE();
    case 128:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(173);
      END_STATE();
    case 129:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(176);
      END_STATE();
    case 130:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(174);
      END_STATE();
    case 131:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(180);
      END_STATE();
    case 132:
      if (lookahead == 'F' ||
          lookahead == 'f') ADVANCE(209);
      END_STATE();
    case 133:
      if (lookahead == 'G' ||
          lookahead == 'g') ADVANCE(577);
      END_STATE();
    case 134:
      if (lookahead == 'G' ||
          lookahead == 'g') ADVANCE(192);
      END_STATE();
    case 135:
      if (lookahead == 'G' ||
          lookahead == 'g') ADVANCE(198);
      END_STATE();
    case 136:
      if (lookahead == 'G' ||
          lookahead == 'g') ADVANCE(118);
      END_STATE();
    case 137:
      if (lookahead == 'H' ||
          lookahead == 'h') ADVANCE(545);
      END_STATE();
    case 138:
      if (lookahead == 'H' ||
          lookahead == 'h') ADVANCE(563);
      END_STATE();
    case 139:
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(543);
      END_STATE();
    case 140:
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(132);
      END_STATE();
    case 141:
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(169);
      END_STATE();
    case 142:
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(162);
      END_STATE();
    case 143:
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(170);
      END_STATE();
    case 144:
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(139);
      END_STATE();
    case 145:
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(103);
      END_STATE();
    case 146:
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(202);
      END_STATE();
    case 147:
      if (lookahead == 'K' ||
          lookahead == 'k') ADVANCE(547);
      END_STATE();
    case 148:
      if (lookahead == 'K' ||
          lookahead == 'k') ADVANCE(573);
      END_STATE();
    case 149:
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(583);
      END_STATE();
    case 150:
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(167);
      END_STATE();
    case 151:
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(92);
      END_STATE();
    case 152:
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(126);
      END_STATE();
    case 153:
      if (lookahead == 'M' ||
          lookahead == 'm') ADVANCE(533);
      END_STATE();
    case 154:
      if (lookahead == 'M' ||
          lookahead == 'm') ADVANCE(586);
      END_STATE();
    case 155:
      if (lookahead == 'M' ||
          lookahead == 'm') ADVANCE(95);
      END_STATE();
    case 156:
      if (lookahead == 'M' ||
          lookahead == 'm') ADVANCE(127);
      END_STATE();
    case 157:
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(134);
      END_STATE();
    case 158:
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(107);
      END_STATE();
    case 159:
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(182);
      END_STATE();
    case 160:
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(108);
      END_STATE();
    case 161:
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(195);
      END_STATE();
    case 162:
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(100);
      END_STATE();
    case 163:
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(135);
      END_STATE();
    case 164:
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(106);
      END_STATE();
    case 165:
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(150);
      END_STATE();
    case 166:
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(153);
      END_STATE();
    case 167:
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(133);
      END_STATE();
    case 168:
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(575);
      END_STATE();
    case 169:
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(89);
      END_STATE();
    case 170:
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(196);
      END_STATE();
    case 171:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(165);
      END_STATE();
    case 172:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(567);
      END_STATE();
    case 173:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(551);
      END_STATE();
    case 174:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(531);
      END_STATE();
    case 175:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(208);
      END_STATE();
    case 176:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(185);
      END_STATE();
    case 177:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(88);
      END_STATE();
    case 178:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(121);
      END_STATE();
    case 179:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(143);
      END_STATE();
    case 180:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(124);
      END_STATE();
    case 181:
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(91);
      END_STATE();
    case 182:
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(151);
      END_STATE();
    case 183:
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(200);
      END_STATE();
    case 184:
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(190);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(191);
      END_STATE();
    case 185:
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(117);
      END_STATE();
    case 186:
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(102);
      END_STATE();
    case 187:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(555);
      END_STATE();
    case 188:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(559);
      END_STATE();
    case 189:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(584);
      END_STATE();
    case 190:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(589);
      END_STATE();
    case 191:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(525);
      END_STATE();
    case 192:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(137);
      END_STATE();
    case 193:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(520);
      END_STATE();
    case 194:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(541);
      END_STATE();
    case 195:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(557);
      END_STATE();
    case 196:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(565);
      END_STATE();
    case 197:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(85);
      END_STATE();
    case 198:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(138);
      END_STATE();
    case 199:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(181);
      END_STATE();
    case 200:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(140);
      END_STATE();
    case 201:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(123);
      END_STATE();
    case 202:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(115);
      END_STATE();
    case 203:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(120);
      END_STATE();
    case 204:
      if (lookahead == 'V' ||
          lookahead == 'v') ADVANCE(145);
      END_STATE();
    case 205:
      if (lookahead == 'W' ||
          lookahead == 'w') ADVANCE(523);
      END_STATE();
    case 206:
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(188);
      END_STATE();
    case 207:
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(581);
      END_STATE();
    case 208:
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(553);
      END_STATE();
    case 209:
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(539);
      END_STATE();
    case 210:
      if (eof) ADVANCE(213);
      if (lookahead == '\n') ADVANCE(214);
      if (lookahead == ' ') ADVANCE(215);
      if (lookahead == '"') ADVANCE(484);
      if (lookahead == '#') ADVANCE(493);
      if (lookahead == '&') ADVANCE(496);
      if (lookahead == '\'') ADVANCE(2);
      if (lookahead == '(') ADVANCE(283);
      if (lookahead == ')') ADVANCE(285);
      if (lookahead == '*') ADVANCE(490);
      if (lookahead == '+') ADVANCE(481);
      if (lookahead == ',') ADVANCE(284);
      if (lookahead == '-') ADVANCE(482);
      if (lookahead == '.') ADVANCE(515);
      if (lookahead == '/') ADVANCE(491);
      if (lookahead == ':') ADVANCE(522);
      if (lookahead == '<') ADVANCE(500);
      if (lookahead == '=') ADVANCE(499);
      if (lookahead == '>') ADVANCE(495);
      if (lookahead == '?') ADVANCE(512);
      if (lookahead == '@') ADVANCE(489);
      if (lookahead == 'A' ||
          lookahead == 'C' ||
          lookahead == 'E' ||
          lookahead == 'L' ||
          lookahead == 'N' ||
          lookahead == 'P' ||
          lookahead == 'U') ADVANCE(516);
      if (lookahead == '[') ADVANCE(497);
      if (lookahead == '\\') ADVANCE(492);
      if (lookahead == ']') ADVANCE(498);
      if (lookahead == '_') ADVANCE(494);
      if (lookahead == '|') ADVANCE(518);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(514);
      END_STATE();
    case 211:
      if (eof) ADVANCE(213);
      if (lookahead == '\n') ADVANCE(214);
      if (lookahead == ' ') ADVANCE(215);
      if (lookahead == '"') ADVANCE(484);
      if (lookahead == '$') ADVANCE(569);
      if (lookahead == '\'') ADVANCE(510);
      if (lookahead == '(') ADVANCE(283);
      if (lookahead == '+') ADVANCE(481);
      if (lookahead == '-') ADVANCE(482);
      if (lookahead == '.') ADVANCE(289);
      if (lookahead == '@') ADVANCE(489);
      if (lookahead == '^') ADVANCE(282);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(483);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 212:
      if (eof) ADVANCE(213);
      if (lookahead == '\n') ADVANCE(214);
      if (lookahead == ' ') ADVANCE(215);
      if (lookahead == '#') ADVANCE(493);
      if (lookahead == '&') ADVANCE(496);
      if (lookahead == '\'') ADVANCE(2);
      if (lookahead == '(') ADVANCE(283);
      if (lookahead == ')') ADVANCE(285);
      if (lookahead == '*') ADVANCE(490);
      if (lookahead == '+') ADVANCE(481);
      if (lookahead == ',') ADVANCE(284);
      if (lookahead == '-') ADVANCE(482);
      if (lookahead == '.') ADVANCE(289);
      if (lookahead == '/') ADVANCE(491);
      if (lookahead == ':') ADVANCE(522);
      if (lookahead == '<') ADVANCE(500);
      if (lookahead == '=') ADVANCE(499);
      if (lookahead == '>') ADVANCE(495);
      if (lookahead == '?') ADVANCE(512);
      if (lookahead == '@') ADVANCE(489);
      if (lookahead == '[') ADVANCE(497);
      if (lookahead == '\\') ADVANCE(492);
      if (lookahead == ']') ADVANCE(498);
      if (lookahead == '_') ADVANCE(494);
      if (lookahead == '|') ADVANCE(518);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(544);
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(568);
      if (lookahead == 'D' ||
          lookahead == 'd') ADVANCE(562);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(542);
      if (lookahead == 'F' ||
          lookahead == 'f') ADVANCE(530);
      if (lookahead == 'G' ||
          lookahead == 'g') ADVANCE(556);
      if (lookahead == 'H' ||
          lookahead == 'h') ADVANCE(578);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(558);
      if (lookahead == 'J' ||
          lookahead == 'j') ADVANCE(540);
      if (lookahead == 'K' ||
          lookahead == 'k') ADVANCE(582);
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(546);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(560);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(552);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(538);
      if (lookahead == 'Q' ||
          lookahead == 'q') ADVANCE(554);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(534);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(521);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(526);
      if (lookahead == 'V' ||
          lookahead == 'v') ADVANCE(524);
      if (lookahead == 'W' ||
          lookahead == 'w') ADVANCE(638);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(590);
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(591);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(483);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(anon_sym_LF);
      END_STATE();
    case 215:
      ACCEPT_TOKEN(anon_sym_);
      END_STATE();
    case 216:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      END_STATE();
    case 217:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(483);
      END_STATE();
    case 218:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(216);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(217);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(218);
      END_STATE();
    case 221:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(219);
      END_STATE();
    case 222:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(220);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(221);
      END_STATE();
    case 224:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(222);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(223);
      END_STATE();
    case 226:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(224);
      END_STATE();
    case 227:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(225);
      END_STATE();
    case 228:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      END_STATE();
    case 229:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(227);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(228);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(229);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(230);
      END_STATE();
    case 233:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(231);
      END_STATE();
    case 234:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(232);
      END_STATE();
    case 235:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(233);
      END_STATE();
    case 236:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(234);
      END_STATE();
    case 237:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(235);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(236);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(237);
      END_STATE();
    case 240:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(238);
      END_STATE();
    case 241:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(239);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(240);
      END_STATE();
    case 243:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(241);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(242);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(243);
      END_STATE();
    case 246:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(244);
      END_STATE();
    case 247:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(245);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(246);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(247);
      END_STATE();
    case 250:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(248);
      END_STATE();
    case 251:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(249);
      END_STATE();
    case 252:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(250);
      END_STATE();
    case 253:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(251);
      END_STATE();
    case 254:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(252);
      END_STATE();
    case 255:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(253);
      END_STATE();
    case 256:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(254);
      END_STATE();
    case 257:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(255);
      END_STATE();
    case 258:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(256);
      END_STATE();
    case 259:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(257);
      END_STATE();
    case 260:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(258);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(259);
      END_STATE();
    case 262:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      END_STATE();
    case 263:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(261);
      END_STATE();
    case 264:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(262);
      END_STATE();
    case 265:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(263);
      END_STATE();
    case 266:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(264);
      END_STATE();
    case 267:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(265);
      END_STATE();
    case 268:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(266);
      END_STATE();
    case 269:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(267);
      END_STATE();
    case 270:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(268);
      END_STATE();
    case 271:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(269);
      END_STATE();
    case 272:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(270);
      END_STATE();
    case 273:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(271);
      END_STATE();
    case 274:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(272);
      END_STATE();
    case 275:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(273);
      END_STATE();
    case 276:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(274);
      END_STATE();
    case 277:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(275);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(276);
      END_STATE();
    case 279:
      ACCEPT_TOKEN(sym_NumericIdentifier);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(277);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(anon_sym_DOLLAR_DOLLAR);
      END_STATE();
    case 281:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 282:
      ACCEPT_TOKEN(anon_sym_CARET);
      if (lookahead == '[') ADVANCE(519);
      if (lookahead == '|') ADVANCE(517);
      END_STATE();
    case 283:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 284:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 285:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 286:
      ACCEPT_TOKEN(sym_VarUndefined);
      if (lookahead == '"') ADVANCE(484);
      if (lookahead == '$') ADVANCE(569);
      if (lookahead == '\'') ADVANCE(510);
      if (lookahead == '(') ADVANCE(283);
      if (lookahead == ')') ADVANCE(285);
      if (lookahead == '+') ADVANCE(481);
      if (lookahead == '-') ADVANCE(482);
      if (lookahead == '.') ADVANCE(289);
      if (lookahead == '@') ADVANCE(489);
      if (lookahead == '^') ADVANCE(282);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(288);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(483);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 287:
      ACCEPT_TOKEN(sym_VarUndefined);
      if (lookahead == '"') ADVANCE(484);
      if (lookahead == '$') ADVANCE(569);
      if (lookahead == '\'') ADVANCE(510);
      if (lookahead == '(') ADVANCE(283);
      if (lookahead == '+') ADVANCE(481);
      if (lookahead == '-') ADVANCE(482);
      if (lookahead == '.') ADVANCE(289);
      if (lookahead == '@') ADVANCE(489);
      if (lookahead == '^') ADVANCE(282);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(288);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(483);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(sym_VarUndefined);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(288);
      END_STATE();
    case 289:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 290:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A') ADVANCE(311);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(419);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 291:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A') ADVANCE(316);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 292:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A') ADVANCE(354);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'B') ADVANCE(351);
      if (lookahead == 'V' ||
          lookahead == 'v') ADVANCE(415);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C') ADVANCE(627);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(377);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C') ADVANCE(328);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C') ADVANCE(309);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C') ADVANCE(348);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 298:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E') ADVANCE(347);
      if (lookahead == 'e') ADVANCE(467);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E') ADVANCE(425);
      if (lookahead == 'I') ADVANCE(306);
      if (lookahead == 'P') ADVANCE(292);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(362);
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(463);
      if (lookahead == 'e') ADVANCE(426);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 300:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E') ADVANCE(598);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 301:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E') ADVANCE(610);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E') ADVANCE(322);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E') ADVANCE(297);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 304:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E') ADVANCE(326);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 305:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E') ADVANCE(344);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 306:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'G') ADVANCE(325);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'G') ADVANCE(631);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'G') ADVANCE(600);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'H') ADVANCE(315);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 310:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I') ADVANCE(317);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(475);
      if (lookahead == 'O') ADVANCE(334);
      if (lookahead == 'i') ADVANCE(432);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 311:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I') ADVANCE(346);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 312:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I') ADVANCE(329);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 313:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I') ADVANCE(349);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'K') ADVANCE(625);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 315:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'K') ADVANCE(623);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'L') ADVANCE(604);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 317:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'L') ADVANCE(300);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(381);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 318:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'M') ADVANCE(332);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 319:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'M') ADVANCE(617);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 320:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'M') ADVANCE(602);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 321:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'M') ADVANCE(343);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 322:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N') ADVANCE(352);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 323:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N') ADVANCE(606);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 324:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N') ADVANCE(608);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 325:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N') ADVANCE(291);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 326:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N') ADVANCE(353);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 327:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'O') ADVANCE(449);
      if (lookahead == 'o') ADVANCE(450);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'O') ADVANCE(320);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 329:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'O') ADVANCE(324);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 330:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'O') ADVANCE(335);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'P') ADVANCE(295);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(383);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'P') ADVANCE(338);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R') ADVANCE(303);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 334:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R') ADVANCE(314);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 335:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R') ADVANCE(356);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R') ADVANCE(321);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R') ADVANCE(341);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R') ADVANCE(305);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R') ADVANCE(313);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 340:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S') ADVANCE(633);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S') ADVANCE(312);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 342:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S') ADVANCE(319);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 343:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S') ADVANCE(308);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 344:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S') ADVANCE(340);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 345:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T') ADVANCE(596);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T') ADVANCE(629);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T') ADVANCE(302);
      if (lookahead == 't') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T') ADVANCE(330);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 349:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T') ADVANCE(301);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 350:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'U') ADVANCE(296);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 351:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'U') ADVANCE(307);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'V') ADVANCE(621);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'V') ADVANCE(619);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 354:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'W') ADVANCE(323);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'W') ADVANCE(339);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 356:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'Y') ADVANCE(594);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(469);
      if (lookahead == 'E') ADVANCE(293);
      if (lookahead == 'I') ADVANCE(333);
      if (lookahead == 'e') ADVANCE(477);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(469);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(477);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(443);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(405);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(374);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(455);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(374);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(433);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(421);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(446);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(375);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(436);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(402);
      if (lookahead == 'O') ADVANCE(350);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(436);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(402);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(472);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(394);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(464);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(420);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(467);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(418);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 377:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(456);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(413);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(384);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(457);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'D' ||
          lookahead == 'd') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'D' ||
          lookahead == 'd') ADVANCE(440);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'D' ||
          lookahead == 'd') ADVANCE(394);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(479);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(426);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(362);
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(463);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(460);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(364);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 388:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(478);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 389:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(429);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(379);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(467);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 392:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(430);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(427);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(446);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 395:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(375);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 396:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(434);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 397:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(435);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 398:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(447);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(467);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 399:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(452);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(458);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 401:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'F' ||
          lookahead == 'f') ADVANCE(479);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 402:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'F' ||
          lookahead == 'f') ADVANCE(400);
      if (lookahead == 'V' ||
          lookahead == 'v') ADVANCE(399);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'G' ||
          lookahead == 'g') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 404:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'G' ||
          lookahead == 'g') ADVANCE(468);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 405:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'G' ||
          lookahead == 'g') ADVANCE(384);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'H' ||
          lookahead == 'h') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'H' ||
          lookahead == 'h') ADVANCE(366);
      if (lookahead == 'O') ADVANCE(318);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 408:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'H' ||
          lookahead == 'h') ADVANCE(366);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 409:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 410:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(432);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(475);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 411:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(401);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(437);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(445);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 414:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(390);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(412);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 415:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(379);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 416:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(444);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 417:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(388);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 418:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(409);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 419:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(472);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'K' ||
          lookahead == 'k') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 421:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(392);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(476);
      if (lookahead == 'U' ||
          lookahead == 'u') ADVANCE(398);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 423:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(441);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 424:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(370);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 425:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(395);
      if (lookahead == 'T') ADVANCE(304);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 426:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(395);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 427:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'M' ||
          lookahead == 'm') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 428:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'M' ||
          lookahead == 'm') ADVANCE(372);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 429:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'M' ||
          lookahead == 'm') ADVANCE(397);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 430:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(404);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 431:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(377);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 432:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(381);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 433:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(461);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 434:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(379);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 435:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(467);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 436:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(382);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 437:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(378);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 438:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(371);
      if (lookahead == 'U' ||
          lookahead == 'u') ADVANCE(462);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 439:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(423);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 440:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(427);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 441:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(403);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 442:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(450);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 443:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 444:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(467);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 445:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(365);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 446:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 447:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(479);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 448:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(383);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 449:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(439);
      if (lookahead == 'S') ADVANCE(345);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 450:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(439);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 451:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(419);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 452:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(465);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 453:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(360);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 454:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(367);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 455:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(361);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 456:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(389);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 457:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(416);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 458:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(396);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 459:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(376);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 460:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(467);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(467);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 461:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(424);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 462:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(471);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 463:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(473);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 464:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(380);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 465:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(384);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 466:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(470);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(453);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(474);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 467:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 468:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(406);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 469:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(357);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 470:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(363);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 471:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(411);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 472:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(384);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 473:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(393);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 474:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(454);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 475:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'U' ||
          lookahead == 'u') ADVANCE(428);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 476:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'U' ||
          lookahead == 'u') ADVANCE(373);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 477:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'V' ||
          lookahead == 'v') ADVANCE(415);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 478:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'W' ||
          lookahead == 'w') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 479:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(480);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 480:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 481:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 482:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 483:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(483);
      END_STATE();
    case 484:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 485:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      if (lookahead == '"') ADVANCE(488);
      END_STATE();
    case 486:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '"') ADVANCE(485);
      if (lookahead != 0) ADVANCE(487);
      END_STATE();
    case 487:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(487);
      END_STATE();
    case 488:
      ACCEPT_TOKEN(anon_sym_DQUOTE_DQUOTE);
      END_STATE();
    case 489:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 490:
      ACCEPT_TOKEN(sym_OPMUL);
      if (lookahead == '*') ADVANCE(501);
      END_STATE();
    case 491:
      ACCEPT_TOKEN(sym_OPDIV);
      END_STATE();
    case 492:
      ACCEPT_TOKEN(sym_OPINT);
      END_STATE();
    case 493:
      ACCEPT_TOKEN(sym_OPMOD);
      END_STATE();
    case 494:
      ACCEPT_TOKEN(sym_OPCAT);
      END_STATE();
    case 495:
      ACCEPT_TOKEN(sym_OPGTR);
      END_STATE();
    case 496:
      ACCEPT_TOKEN(sym_OPAND);
      END_STATE();
    case 497:
      ACCEPT_TOKEN(sym_OPCON);
      END_STATE();
    case 498:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      if (lookahead == ']') ADVANCE(508);
      END_STATE();
    case 499:
      ACCEPT_TOKEN(sym_OPEQL);
      END_STATE();
    case 500:
      ACCEPT_TOKEN(sym_OPLES);
      END_STATE();
    case 501:
      ACCEPT_TOKEN(sym_OPPOW);
      END_STATE();
    case 502:
      ACCEPT_TOKEN(sym_OPNEQL);
      END_STATE();
    case 503:
      ACCEPT_TOKEN(sym_OPNLES);
      END_STATE();
    case 504:
      ACCEPT_TOKEN(sym_OPNGTR);
      END_STATE();
    case 505:
      ACCEPT_TOKEN(sym_OPNAND);
      END_STATE();
    case 506:
      ACCEPT_TOKEN(sym_OPNCON);
      END_STATE();
    case 507:
      ACCEPT_TOKEN(sym_OPNFOL);
      if (lookahead == ']') ADVANCE(509);
      END_STATE();
    case 508:
      ACCEPT_TOKEN(sym_OPSAF);
      END_STATE();
    case 509:
      ACCEPT_TOKEN(sym_OPNSAF);
      END_STATE();
    case 510:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 511:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      if (lookahead == '&') ADVANCE(505);
      if (lookahead == '<') ADVANCE(503);
      if (lookahead == '=') ADVANCE(502);
      if (lookahead == '>') ADVANCE(504);
      if (lookahead == '?') ADVANCE(513);
      if (lookahead == '[') ADVANCE(506);
      if (lookahead == ']') ADVANCE(507);
      END_STATE();
    case 512:
      ACCEPT_TOKEN(sym_OPPAT);
      END_STATE();
    case 513:
      ACCEPT_TOKEN(sym_OPNPAT);
      END_STATE();
    case 514:
      ACCEPT_TOKEN(aux_sym_PaternRepetitionCount_token1);
      if (lookahead == '.') ADVANCE(515);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(514);
      END_STATE();
    case 515:
      ACCEPT_TOKEN(aux_sym_PaternRepetitionCount_token2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(515);
      END_STATE();
    case 516:
      ACCEPT_TOKEN(aux_sym_PaternElement_token1);
      END_STATE();
    case 517:
      ACCEPT_TOKEN(anon_sym_CARET_PIPE);
      END_STATE();
    case 518:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 519:
      ACCEPT_TOKEN(anon_sym_CARET_LBRACK);
      END_STATE();
    case 520:
      ACCEPT_TOKEN(aux_sym_Select_token1);
      END_STATE();
    case 521:
      ACCEPT_TOKEN(aux_sym_Select_token2);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(152);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(548);
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(587);
      END_STATE();
    case 522:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 523:
      ACCEPT_TOKEN(aux_sym_View_token1);
      END_STATE();
    case 524:
      ACCEPT_TOKEN(aux_sym_View_token2);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(111);
      END_STATE();
    case 525:
      ACCEPT_TOKEN(aux_sym_Text_token1);
      END_STATE();
    case 526:
      ACCEPT_TOKEN(aux_sym_Text_token2);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(184);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(528);
      END_STATE();
    case 527:
      ACCEPT_TOKEN(aux_sym_Translate_token1);
      END_STATE();
    case 528:
      ACCEPT_TOKEN(aux_sym_Translate_token2);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(159);
      END_STATE();
    case 529:
      ACCEPT_TOKEN(aux_sym_Find_token1);
      END_STATE();
    case 530:
      ACCEPT_TOKEN(aux_sym_Find_token2);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(158);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(532);
      END_STATE();
    case 531:
      ACCEPT_TOKEN(aux_sym_Fnumber_token1);
      END_STATE();
    case 532:
      ACCEPT_TOKEN(aux_sym_Fnumber_token2);
      if (lookahead == 'U' ||
          lookahead == 'u') ADVANCE(155);
      END_STATE();
    case 533:
      ACCEPT_TOKEN(aux_sym_Random_token1);
      END_STATE();
    case 534:
      ACCEPT_TOKEN(aux_sym_Random_token2);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(160);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(536);
      END_STATE();
    case 535:
      ACCEPT_TOKEN(aux_sym_Reverse_token1);
      END_STATE();
    case 536:
      ACCEPT_TOKEN(aux_sym_Reverse_token2);
      if (lookahead == 'F' ||
          lookahead == 'f') ADVANCE(131);
      if (lookahead == 'V' ||
          lookahead == 'v') ADVANCE(129);
      END_STATE();
    case 537:
      ACCEPT_TOKEN(aux_sym_Piece_token1);
      END_STATE();
    case 538:
      ACCEPT_TOKEN(aux_sym_Piece_token2);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(122);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(142);
      END_STATE();
    case 539:
      ACCEPT_TOKEN(aux_sym_Justify_token1);
      END_STATE();
    case 540:
      ACCEPT_TOKEN(aux_sym_Justify_token2);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(93);
      if (lookahead == 'U' ||
          lookahead == 'u') ADVANCE(183);
      END_STATE();
    case 541:
      ACCEPT_TOKEN(aux_sym_Extract_token1);
      END_STATE();
    case 542:
      ACCEPT_TOKEN(aux_sym_Extract_token2);
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(572);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(574);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(576);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(199);
      END_STATE();
    case 543:
      ACCEPT_TOKEN(aux_sym_Ascii_token1);
      END_STATE();
    case 544:
      ACCEPT_TOKEN(aux_sym_Ascii_token2);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(98);
      END_STATE();
    case 545:
      ACCEPT_TOKEN(aux_sym_Length_token1);
      END_STATE();
    case 546:
      ACCEPT_TOKEN(aux_sym_Length_token2);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(157);
      END_STATE();
    case 547:
      ACCEPT_TOKEN(aux_sym_Stack_token1);
      END_STATE();
    case 548:
      ACCEPT_TOKEN(aux_sym_Stack_token2);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(96);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(177);
      END_STATE();
    case 549:
      ACCEPT_TOKEN(aux_sym_Name_token1);
      END_STATE();
    case 550:
      ACCEPT_TOKEN(aux_sym_Name_token2);
      if (lookahead == 'M' ||
          lookahead == 'm') ADVANCE(112);
      END_STATE();
    case 551:
      ACCEPT_TOKEN(aux_sym_Order_token1);
      END_STATE();
    case 552:
      ACCEPT_TOKEN(aux_sym_Order_token2);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(109);
      END_STATE();
    case 553:
      ACCEPT_TOKEN(aux_sym_Query_token1);
      END_STATE();
    case 554:
      ACCEPT_TOKEN(aux_sym_Query_token2);
      if (lookahead == 'L' ||
          lookahead == 'l') ADVANCE(564);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(566);
      if (lookahead == 'U' ||
          lookahead == 'u') ADVANCE(125);
      END_STATE();
    case 555:
      ACCEPT_TOKEN(aux_sym_Get_token1);
      END_STATE();
    case 556:
      ACCEPT_TOKEN(aux_sym_Get_token2);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(187);
      END_STATE();
    case 557:
      ACCEPT_TOKEN(aux_sym_Increment_token1);
      END_STATE();
    case 558:
      ACCEPT_TOKEN(aux_sym_Increment_token2);
      if (lookahead == 'N' ||
          lookahead == 'n') ADVANCE(99);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(579);
      END_STATE();
    case 559:
      ACCEPT_TOKEN(aux_sym_Next_token1);
      END_STATE();
    case 560:
      ACCEPT_TOKEN(aux_sym_Next_token2);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(550);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(206);
      END_STATE();
    case 561:
      ACCEPT_TOKEN(aux_sym_Data_token1);
      END_STATE();
    case 562:
      ACCEPT_TOKEN(aux_sym_Data_token2);
      if (lookahead == 'A' ||
          lookahead == 'a') ADVANCE(197);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(204);
      END_STATE();
    case 563:
      ACCEPT_TOKEN(aux_sym_Qlength_token1);
      END_STATE();
    case 564:
      ACCEPT_TOKEN(aux_sym_Qlength_token2);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(163);
      END_STATE();
    case 565:
      ACCEPT_TOKEN(aux_sym_Qsubscript_token1);
      END_STATE();
    case 566:
      ACCEPT_TOKEN(aux_sym_Qsubscript_token2);
      if (lookahead == 'U' ||
          lookahead == 'u') ADVANCE(94);
      END_STATE();
    case 567:
      ACCEPT_TOKEN(aux_sym_Char_token1);
      END_STATE();
    case 568:
      ACCEPT_TOKEN(aux_sym_Char_token2);
      if (lookahead == 'H' ||
          lookahead == 'h') ADVANCE(87);
      END_STATE();
    case 569:
      ACCEPT_TOKEN(anon_sym_DOLLAR);
      if (lookahead == '$') ADVANCE(280);
      if (lookahead == '&') ADVANCE(636);
      END_STATE();
    case 570:
      ACCEPT_TOKEN(aux_sym_Device_token1);
      END_STATE();
    case 571:
      ACCEPT_TOKEN(aux_sym_Ecode_token1);
      END_STATE();
    case 572:
      ACCEPT_TOKEN(aux_sym_Ecode_token2);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(110);
      END_STATE();
    case 573:
      ACCEPT_TOKEN(aux_sym_Estack_token1);
      END_STATE();
    case 574:
      ACCEPT_TOKEN(aux_sym_Estack_token2);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(90);
      END_STATE();
    case 575:
      ACCEPT_TOKEN(aux_sym_Etrap_token1);
      END_STATE();
    case 576:
      ACCEPT_TOKEN(aux_sym_Etrap_token2);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(86);
      END_STATE();
    case 577:
      ACCEPT_TOKEN(aux_sym_Horolog_token1);
      END_STATE();
    case 578:
      ACCEPT_TOKEN(aux_sym_Horolog_token2);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(171);
      END_STATE();
    case 579:
      ACCEPT_TOKEN(aux_sym_Io_token1);
      END_STATE();
    case 580:
      ACCEPT_TOKEN(aux_sym_Job_token1);
      END_STATE();
    case 581:
      ACCEPT_TOKEN(aux_sym_Key_token1);
      END_STATE();
    case 582:
      ACCEPT_TOKEN(aux_sym_Key_token2);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(207);
      END_STATE();
    case 583:
      ACCEPT_TOKEN(aux_sym_Principal_token1);
      END_STATE();
    case 584:
      ACCEPT_TOKEN(aux_sym_Quit_token1);
      END_STATE();
    case 585:
      ACCEPT_TOKEN(aux_sym_Reference_token1);
      END_STATE();
    case 586:
      ACCEPT_TOKEN(aux_sym_System_token1);
      END_STATE();
    case 587:
      ACCEPT_TOKEN(aux_sym_System_token2);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(201);
      END_STATE();
    case 588:
      ACCEPT_TOKEN(aux_sym_Storage_token1);
      END_STATE();
    case 589:
      ACCEPT_TOKEN(aux_sym_Test_token1);
      END_STATE();
    case 590:
      ACCEPT_TOKEN(aux_sym_X_token1);
      END_STATE();
    case 591:
      ACCEPT_TOKEN(aux_sym_Y_token1);
      END_STATE();
    case 592:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 593:
      ACCEPT_TOKEN(anon_sym_DIRECTORY);
      END_STATE();
    case 594:
      ACCEPT_TOKEN(anon_sym_DIRECTORY);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 595:
      ACCEPT_TOKEN(anon_sym_HOST);
      END_STATE();
    case 596:
      ACCEPT_TOKEN(anon_sym_HOST);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 597:
      ACCEPT_TOKEN(anon_sym_FILE);
      END_STATE();
    case 598:
      ACCEPT_TOKEN(anon_sym_FILE);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 599:
      ACCEPT_TOKEN(anon_sym_ERRMSG);
      END_STATE();
    case 600:
      ACCEPT_TOKEN(anon_sym_ERRMSG);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 601:
      ACCEPT_TOKEN(anon_sym_OPCOM);
      END_STATE();
    case 602:
      ACCEPT_TOKEN(anon_sym_OPCOM);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 603:
      ACCEPT_TOKEN(anon_sym_SIGNAL);
      END_STATE();
    case 604:
      ACCEPT_TOKEN(anon_sym_SIGNAL);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 605:
      ACCEPT_TOKEN(anon_sym_SPAWN);
      END_STATE();
    case 606:
      ACCEPT_TOKEN(anon_sym_SPAWN);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 607:
      ACCEPT_TOKEN(anon_sym_VERSION);
      END_STATE();
    case 608:
      ACCEPT_TOKEN(anon_sym_VERSION);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 609:
      ACCEPT_TOKEN(anon_sym_ZWRITE);
      END_STATE();
    case 610:
      ACCEPT_TOKEN(anon_sym_ZWRITE);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 611:
      ACCEPT_TOKEN(sym_E);
      END_STATE();
    case 612:
      ACCEPT_TOKEN(sym_E);
      if (lookahead == 'R') ADVANCE(336);
      if (lookahead == 'S' ||
          lookahead == 's') ADVANCE(470);
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(453);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(474);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 613:
      ACCEPT_TOKEN(sym_Paschk);
      END_STATE();
    case 614:
      ACCEPT_TOKEN(sym_V);
      END_STATE();
    case 615:
      ACCEPT_TOKEN(sym_V);
      if (lookahead == 'E') ADVANCE(337);
      if (lookahead == 'I' ||
          lookahead == 'i') ADVANCE(388);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 616:
      ACCEPT_TOKEN(sym_Xrsm);
      END_STATE();
    case 617:
      ACCEPT_TOKEN(sym_Xrsm);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 618:
      ACCEPT_TOKEN(anon_sym_SETENV);
      END_STATE();
    case 619:
      ACCEPT_TOKEN(anon_sym_SETENV);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 620:
      ACCEPT_TOKEN(anon_sym_GETENV);
      END_STATE();
    case 621:
      ACCEPT_TOKEN(anon_sym_GETENV);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 622:
      ACCEPT_TOKEN(anon_sym_ROUCHK);
      END_STATE();
    case 623:
      ACCEPT_TOKEN(anon_sym_ROUCHK);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 624:
      ACCEPT_TOKEN(anon_sym_FORK);
      END_STATE();
    case 625:
      ACCEPT_TOKEN(anon_sym_FORK);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 626:
      ACCEPT_TOKEN(anon_sym_IC);
      END_STATE();
    case 627:
      ACCEPT_TOKEN(anon_sym_IC);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 628:
      ACCEPT_TOKEN(anon_sym_WAIT);
      END_STATE();
    case 629:
      ACCEPT_TOKEN(anon_sym_WAIT);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 630:
      ACCEPT_TOKEN(sym_Debug);
      END_STATE();
    case 631:
      ACCEPT_TOKEN(sym_Debug);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 632:
      ACCEPT_TOKEN(anon_sym_COMPRESS);
      END_STATE();
    case 633:
      ACCEPT_TOKEN(anon_sym_COMPRESS);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 634:
      ACCEPT_TOKEN(sym_XCallX);
      if (lookahead == 'R') ADVANCE(342);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(480);
      END_STATE();
    case 635:
      ACCEPT_TOKEN(sym_XCallX);
      if (lookahead == 'R') ADVANCE(65);
      END_STATE();
    case 636:
      ACCEPT_TOKEN(anon_sym_DOLLAR_AMP);
      END_STATE();
    case 637:
      ACCEPT_TOKEN(aux_sym_Write_token1);
      END_STATE();
    case 638:
      ACCEPT_TOKEN(aux_sym_Write_token2);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(146);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 212},
  [2] = {.lex_state = 212},
  [3] = {.lex_state = 211},
  [4] = {.lex_state = 286},
  [5] = {.lex_state = 286},
  [6] = {.lex_state = 286},
  [7] = {.lex_state = 287},
  [8] = {.lex_state = 210},
  [9] = {.lex_state = 210},
  [10] = {.lex_state = 211},
  [11] = {.lex_state = 212},
  [12] = {.lex_state = 212},
  [13] = {.lex_state = 212},
  [14] = {.lex_state = 212},
  [15] = {.lex_state = 212},
  [16] = {.lex_state = 211},
  [17] = {.lex_state = 211},
  [18] = {.lex_state = 211},
  [19] = {.lex_state = 211},
  [20] = {.lex_state = 211},
  [21] = {.lex_state = 211},
  [22] = {.lex_state = 211},
  [23] = {.lex_state = 211},
  [24] = {.lex_state = 212},
  [25] = {.lex_state = 211},
  [26] = {.lex_state = 211},
  [27] = {.lex_state = 211},
  [28] = {.lex_state = 211},
  [29] = {.lex_state = 211},
  [30] = {.lex_state = 211},
  [31] = {.lex_state = 211},
  [32] = {.lex_state = 211},
  [33] = {.lex_state = 211},
  [34] = {.lex_state = 212},
  [35] = {.lex_state = 211},
  [36] = {.lex_state = 211},
  [37] = {.lex_state = 211},
  [38] = {.lex_state = 211},
  [39] = {.lex_state = 211},
  [40] = {.lex_state = 211},
  [41] = {.lex_state = 211},
  [42] = {.lex_state = 211},
  [43] = {.lex_state = 211},
  [44] = {.lex_state = 211},
  [45] = {.lex_state = 211},
  [46] = {.lex_state = 211},
  [47] = {.lex_state = 211},
  [48] = {.lex_state = 211},
  [49] = {.lex_state = 211},
  [50] = {.lex_state = 211},
  [51] = {.lex_state = 211},
  [52] = {.lex_state = 211},
  [53] = {.lex_state = 211},
  [54] = {.lex_state = 211},
  [55] = {.lex_state = 211},
  [56] = {.lex_state = 211},
  [57] = {.lex_state = 211},
  [58] = {.lex_state = 211},
  [59] = {.lex_state = 211},
  [60] = {.lex_state = 211},
  [61] = {.lex_state = 211},
  [62] = {.lex_state = 211},
  [63] = {.lex_state = 211},
  [64] = {.lex_state = 211},
  [65] = {.lex_state = 211},
  [66] = {.lex_state = 211},
  [67] = {.lex_state = 211},
  [68] = {.lex_state = 211},
  [69] = {.lex_state = 211},
  [70] = {.lex_state = 211},
  [71] = {.lex_state = 211},
  [72] = {.lex_state = 211},
  [73] = {.lex_state = 212},
  [74] = {.lex_state = 211},
  [75] = {.lex_state = 211},
  [76] = {.lex_state = 211},
  [77] = {.lex_state = 211},
  [78] = {.lex_state = 211},
  [79] = {.lex_state = 211},
  [80] = {.lex_state = 210},
  [81] = {.lex_state = 210},
  [82] = {.lex_state = 210},
  [83] = {.lex_state = 210},
  [84] = {.lex_state = 210},
  [85] = {.lex_state = 212},
  [86] = {.lex_state = 212},
  [87] = {.lex_state = 212},
  [88] = {.lex_state = 212},
  [89] = {.lex_state = 212},
  [90] = {.lex_state = 212},
  [91] = {.lex_state = 212},
  [92] = {.lex_state = 212},
  [93] = {.lex_state = 212},
  [94] = {.lex_state = 212},
  [95] = {.lex_state = 212},
  [96] = {.lex_state = 212},
  [97] = {.lex_state = 212},
  [98] = {.lex_state = 212},
  [99] = {.lex_state = 212},
  [100] = {.lex_state = 212},
  [101] = {.lex_state = 212},
  [102] = {.lex_state = 212},
  [103] = {.lex_state = 212},
  [104] = {.lex_state = 212},
  [105] = {.lex_state = 212},
  [106] = {.lex_state = 212},
  [107] = {.lex_state = 212},
  [108] = {.lex_state = 212},
  [109] = {.lex_state = 212},
  [110] = {.lex_state = 212},
  [111] = {.lex_state = 212},
  [112] = {.lex_state = 212},
  [113] = {.lex_state = 212},
  [114] = {.lex_state = 212},
  [115] = {.lex_state = 212},
  [116] = {.lex_state = 212},
  [117] = {.lex_state = 212},
  [118] = {.lex_state = 212},
  [119] = {.lex_state = 212},
  [120] = {.lex_state = 212},
  [121] = {.lex_state = 212},
  [122] = {.lex_state = 212},
  [123] = {.lex_state = 212},
  [124] = {.lex_state = 212},
  [125] = {.lex_state = 212},
  [126] = {.lex_state = 212},
  [127] = {.lex_state = 212},
  [128] = {.lex_state = 212},
  [129] = {.lex_state = 212},
  [130] = {.lex_state = 212},
  [131] = {.lex_state = 212},
  [132] = {.lex_state = 212},
  [133] = {.lex_state = 212},
  [134] = {.lex_state = 212},
  [135] = {.lex_state = 212},
  [136] = {.lex_state = 212},
  [137] = {.lex_state = 212},
  [138] = {.lex_state = 212},
  [139] = {.lex_state = 212},
  [140] = {.lex_state = 212},
  [141] = {.lex_state = 212},
  [142] = {.lex_state = 212},
  [143] = {.lex_state = 212},
  [144] = {.lex_state = 212},
  [145] = {.lex_state = 212},
  [146] = {.lex_state = 212},
  [147] = {.lex_state = 212},
  [148] = {.lex_state = 212},
  [149] = {.lex_state = 212},
  [150] = {.lex_state = 212},
  [151] = {.lex_state = 212},
  [152] = {.lex_state = 212},
  [153] = {.lex_state = 212},
  [154] = {.lex_state = 212},
  [155] = {.lex_state = 212},
  [156] = {.lex_state = 212},
  [157] = {.lex_state = 212},
  [158] = {.lex_state = 212},
  [159] = {.lex_state = 212},
  [160] = {.lex_state = 212},
  [161] = {.lex_state = 212},
  [162] = {.lex_state = 212},
  [163] = {.lex_state = 212},
  [164] = {.lex_state = 212},
  [165] = {.lex_state = 212},
  [166] = {.lex_state = 212},
  [167] = {.lex_state = 212},
  [168] = {.lex_state = 212},
  [169] = {.lex_state = 212},
  [170] = {.lex_state = 212},
  [171] = {.lex_state = 212},
  [172] = {.lex_state = 212},
  [173] = {.lex_state = 212},
  [174] = {.lex_state = 212},
  [175] = {.lex_state = 212},
  [176] = {.lex_state = 212},
  [177] = {.lex_state = 212},
  [178] = {.lex_state = 212},
  [179] = {.lex_state = 212},
  [180] = {.lex_state = 212},
  [181] = {.lex_state = 212},
  [182] = {.lex_state = 212},
  [183] = {.lex_state = 212},
  [184] = {.lex_state = 212},
  [185] = {.lex_state = 212},
  [186] = {.lex_state = 212},
  [187] = {.lex_state = 212},
  [188] = {.lex_state = 212},
  [189] = {.lex_state = 212},
  [190] = {.lex_state = 212},
  [191] = {.lex_state = 212},
  [192] = {.lex_state = 212},
  [193] = {.lex_state = 212},
  [194] = {.lex_state = 212},
  [195] = {.lex_state = 212},
  [196] = {.lex_state = 212},
  [197] = {.lex_state = 212},
  [198] = {.lex_state = 212},
  [199] = {.lex_state = 212},
  [200] = {.lex_state = 212},
  [201] = {.lex_state = 212},
  [202] = {.lex_state = 212},
  [203] = {.lex_state = 212},
  [204] = {.lex_state = 212},
  [205] = {.lex_state = 212},
  [206] = {.lex_state = 212},
  [207] = {.lex_state = 212},
  [208] = {.lex_state = 212},
  [209] = {.lex_state = 212},
  [210] = {.lex_state = 212},
  [211] = {.lex_state = 212},
  [212] = {.lex_state = 212},
  [213] = {.lex_state = 212},
  [214] = {.lex_state = 212},
  [215] = {.lex_state = 212},
  [216] = {.lex_state = 212},
  [217] = {.lex_state = 212},
  [218] = {.lex_state = 212},
  [219] = {.lex_state = 212},
  [220] = {.lex_state = 212},
  [221] = {.lex_state = 212},
  [222] = {.lex_state = 212},
  [223] = {.lex_state = 212},
  [224] = {.lex_state = 212},
  [225] = {.lex_state = 212},
  [226] = {.lex_state = 212},
  [227] = {.lex_state = 212},
  [228] = {.lex_state = 212},
  [229] = {.lex_state = 212},
  [230] = {.lex_state = 212},
  [231] = {.lex_state = 212},
  [232] = {.lex_state = 212},
  [233] = {.lex_state = 212},
  [234] = {.lex_state = 212},
  [235] = {.lex_state = 212},
  [236] = {.lex_state = 212},
  [237] = {.lex_state = 212},
  [238] = {.lex_state = 1},
  [239] = {.lex_state = 9},
  [240] = {.lex_state = 211},
  [241] = {.lex_state = 211},
  [242] = {.lex_state = 211},
  [243] = {.lex_state = 211},
  [244] = {.lex_state = 211},
  [245] = {.lex_state = 211},
  [246] = {.lex_state = 211},
  [247] = {.lex_state = 211},
  [248] = {.lex_state = 211},
  [249] = {.lex_state = 211},
  [250] = {.lex_state = 211},
  [251] = {.lex_state = 211},
  [252] = {.lex_state = 211},
  [253] = {.lex_state = 211},
  [254] = {.lex_state = 211},
  [255] = {.lex_state = 211},
  [256] = {.lex_state = 211},
  [257] = {.lex_state = 211},
  [258] = {.lex_state = 211},
  [259] = {.lex_state = 211},
  [260] = {.lex_state = 210},
  [261] = {.lex_state = 0},
  [262] = {.lex_state = 0},
  [263] = {.lex_state = 212},
  [264] = {.lex_state = 210},
  [265] = {.lex_state = 0},
  [266] = {.lex_state = 210},
  [267] = {.lex_state = 486},
  [268] = {.lex_state = 210},
  [269] = {.lex_state = 0},
  [270] = {.lex_state = 210},
  [271] = {.lex_state = 0},
  [272] = {.lex_state = 84},
  [273] = {.lex_state = 0},
  [274] = {.lex_state = 212},
  [275] = {.lex_state = 486},
  [276] = {.lex_state = 486},
  [277] = {.lex_state = 0},
  [278] = {.lex_state = 0},
  [279] = {.lex_state = 0},
  [280] = {.lex_state = 0},
  [281] = {.lex_state = 0},
  [282] = {.lex_state = 0},
  [283] = {.lex_state = 0},
  [284] = {.lex_state = 0},
  [285] = {.lex_state = 0},
  [286] = {.lex_state = 0},
  [287] = {.lex_state = 210},
  [288] = {.lex_state = 0},
  [289] = {.lex_state = 0},
  [290] = {.lex_state = 0},
  [291] = {.lex_state = 210},
  [292] = {.lex_state = 0},
  [293] = {.lex_state = 0},
  [294] = {.lex_state = 0},
  [295] = {.lex_state = 0},
  [296] = {.lex_state = 0},
  [297] = {.lex_state = 0},
  [298] = {.lex_state = 0},
  [299] = {.lex_state = 0},
  [300] = {.lex_state = 0},
  [301] = {.lex_state = 0},
  [302] = {.lex_state = 0},
  [303] = {.lex_state = 0},
  [304] = {.lex_state = 0},
  [305] = {.lex_state = 0},
  [306] = {.lex_state = 0},
  [307] = {.lex_state = 0},
  [308] = {.lex_state = 0},
  [309] = {.lex_state = 0},
  [310] = {.lex_state = 0},
  [311] = {.lex_state = 0},
  [312] = {.lex_state = 211},
  [313] = {.lex_state = 0},
  [314] = {.lex_state = 0},
  [315] = {.lex_state = 0},
  [316] = {.lex_state = 0},
  [317] = {.lex_state = 0},
  [318] = {.lex_state = 0},
  [319] = {.lex_state = 0},
  [320] = {.lex_state = 0},
  [321] = {.lex_state = 0},
  [322] = {.lex_state = 0},
  [323] = {.lex_state = 0},
  [324] = {.lex_state = 0},
  [325] = {.lex_state = 0},
  [326] = {.lex_state = 0},
  [327] = {.lex_state = 211},
  [328] = {.lex_state = 0},
  [329] = {.lex_state = 0},
  [330] = {.lex_state = 0},
  [331] = {.lex_state = 0},
  [332] = {.lex_state = 0},
  [333] = {.lex_state = 0},
  [334] = {.lex_state = 0},
  [335] = {.lex_state = 0},
  [336] = {.lex_state = 0},
  [337] = {.lex_state = 0},
  [338] = {.lex_state = 0},
  [339] = {.lex_state = 0},
  [340] = {.lex_state = 0},
  [341] = {.lex_state = 0},
  [342] = {.lex_state = 0},
  [343] = {.lex_state = 211},
  [344] = {.lex_state = 0},
  [345] = {.lex_state = 0},
  [346] = {.lex_state = 0},
  [347] = {.lex_state = 0},
  [348] = {.lex_state = 0},
  [349] = {.lex_state = 0},
  [350] = {.lex_state = 0},
  [351] = {.lex_state = 0},
  [352] = {.lex_state = 212},
  [353] = {.lex_state = 0},
  [354] = {.lex_state = 0},
  [355] = {.lex_state = 0},
  [356] = {.lex_state = 211},
  [357] = {.lex_state = 0},
  [358] = {.lex_state = 0},
  [359] = {.lex_state = 0},
  [360] = {.lex_state = 0},
  [361] = {.lex_state = 0},
  [362] = {.lex_state = 0},
  [363] = {.lex_state = 211},
  [364] = {.lex_state = 0},
  [365] = {.lex_state = 0},
  [366] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_LF] = ACTIONS(1),
    [anon_sym_] = ACTIONS(1),
    [sym_NumericIdentifier] = ACTIONS(1),
    [anon_sym_DOLLAR_DOLLAR] = ACTIONS(1),
    [anon_sym_CARET] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [aux_sym_number_token1] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE_DQUOTE] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [sym_OPMUL] = ACTIONS(1),
    [sym_OPDIV] = ACTIONS(1),
    [sym_OPINT] = ACTIONS(1),
    [sym_OPMOD] = ACTIONS(1),
    [sym_OPCAT] = ACTIONS(1),
    [sym_OPGTR] = ACTIONS(1),
    [sym_OPAND] = ACTIONS(1),
    [sym_OPCON] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [sym_OPEQL] = ACTIONS(1),
    [sym_OPLES] = ACTIONS(1),
    [sym_OPPOW] = ACTIONS(1),
    [sym_OPNEQL] = ACTIONS(1),
    [sym_OPNLES] = ACTIONS(1),
    [sym_OPNGTR] = ACTIONS(1),
    [sym_OPNAND] = ACTIONS(1),
    [sym_OPNCON] = ACTIONS(1),
    [sym_OPNFOL] = ACTIONS(1),
    [sym_OPSAF] = ACTIONS(1),
    [sym_OPNSAF] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [sym_OPPAT] = ACTIONS(1),
    [sym_OPNPAT] = ACTIONS(1),
    [aux_sym_PaternRepetitionCount_token1] = ACTIONS(1),
    [aux_sym_PaternElement_token1] = ACTIONS(1),
    [anon_sym_CARET_PIPE] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_CARET_LBRACK] = ACTIONS(1),
    [aux_sym_Select_token1] = ACTIONS(1),
    [aux_sym_Select_token2] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [aux_sym_View_token1] = ACTIONS(1),
    [aux_sym_View_token2] = ACTIONS(1),
    [aux_sym_Text_token1] = ACTIONS(1),
    [aux_sym_Text_token2] = ACTIONS(1),
    [aux_sym_Translate_token1] = ACTIONS(1),
    [aux_sym_Translate_token2] = ACTIONS(1),
    [aux_sym_Find_token1] = ACTIONS(1),
    [aux_sym_Find_token2] = ACTIONS(1),
    [aux_sym_Fnumber_token1] = ACTIONS(1),
    [aux_sym_Fnumber_token2] = ACTIONS(1),
    [aux_sym_Random_token1] = ACTIONS(1),
    [aux_sym_Random_token2] = ACTIONS(1),
    [aux_sym_Reverse_token1] = ACTIONS(1),
    [aux_sym_Reverse_token2] = ACTIONS(1),
    [aux_sym_Piece_token1] = ACTIONS(1),
    [aux_sym_Piece_token2] = ACTIONS(1),
    [aux_sym_Justify_token1] = ACTIONS(1),
    [aux_sym_Justify_token2] = ACTIONS(1),
    [aux_sym_Extract_token1] = ACTIONS(1),
    [aux_sym_Extract_token2] = ACTIONS(1),
    [aux_sym_Ascii_token1] = ACTIONS(1),
    [aux_sym_Ascii_token2] = ACTIONS(1),
    [aux_sym_Length_token2] = ACTIONS(1),
    [aux_sym_Stack_token1] = ACTIONS(1),
    [aux_sym_Stack_token2] = ACTIONS(1),
    [aux_sym_Order_token1] = ACTIONS(1),
    [aux_sym_Order_token2] = ACTIONS(1),
    [aux_sym_Query_token1] = ACTIONS(1),
    [aux_sym_Query_token2] = ACTIONS(1),
    [aux_sym_Get_token1] = ACTIONS(1),
    [aux_sym_Get_token2] = ACTIONS(1),
    [aux_sym_Increment_token1] = ACTIONS(1),
    [aux_sym_Increment_token2] = ACTIONS(1),
    [aux_sym_Next_token2] = ACTIONS(1),
    [aux_sym_Data_token1] = ACTIONS(1),
    [aux_sym_Data_token2] = ACTIONS(1),
    [aux_sym_Qlength_token1] = ACTIONS(1),
    [aux_sym_Qlength_token2] = ACTIONS(1),
    [aux_sym_Qsubscript_token1] = ACTIONS(1),
    [aux_sym_Qsubscript_token2] = ACTIONS(1),
    [aux_sym_Char_token1] = ACTIONS(1),
    [aux_sym_Char_token2] = ACTIONS(1),
    [anon_sym_DOLLAR] = ACTIONS(1),
    [aux_sym_Device_token1] = ACTIONS(1),
    [aux_sym_Estack_token1] = ACTIONS(1),
    [aux_sym_Estack_token2] = ACTIONS(1),
    [aux_sym_Etrap_token1] = ACTIONS(1),
    [aux_sym_Etrap_token2] = ACTIONS(1),
    [aux_sym_Horolog_token1] = ACTIONS(1),
    [aux_sym_Horolog_token2] = ACTIONS(1),
    [aux_sym_Io_token1] = ACTIONS(1),
    [aux_sym_Job_token1] = ACTIONS(1),
    [aux_sym_Key_token1] = ACTIONS(1),
    [aux_sym_Key_token2] = ACTIONS(1),
    [aux_sym_Principal_token1] = ACTIONS(1),
    [aux_sym_Quit_token1] = ACTIONS(1),
    [aux_sym_Reference_token1] = ACTIONS(1),
    [aux_sym_System_token1] = ACTIONS(1),
    [aux_sym_System_token2] = ACTIONS(1),
    [aux_sym_Storage_token1] = ACTIONS(1),
    [aux_sym_Test_token1] = ACTIONS(1),
    [aux_sym_X_token1] = ACTIONS(1),
    [aux_sym_Y_token1] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_DIRECTORY] = ACTIONS(1),
    [anon_sym_HOST] = ACTIONS(1),
    [anon_sym_FILE] = ACTIONS(1),
    [anon_sym_ERRMSG] = ACTIONS(1),
    [anon_sym_OPCOM] = ACTIONS(1),
    [anon_sym_SIGNAL] = ACTIONS(1),
    [anon_sym_SPAWN] = ACTIONS(1),
    [anon_sym_VERSION] = ACTIONS(1),
    [anon_sym_ZWRITE] = ACTIONS(1),
    [sym_E] = ACTIONS(1),
    [sym_V] = ACTIONS(1),
    [sym_Xrsm] = ACTIONS(1),
    [anon_sym_SETENV] = ACTIONS(1),
    [anon_sym_GETENV] = ACTIONS(1),
    [anon_sym_ROUCHK] = ACTIONS(1),
    [anon_sym_FORK] = ACTIONS(1),
    [anon_sym_IC] = ACTIONS(1),
    [anon_sym_WAIT] = ACTIONS(1),
    [sym_Debug] = ACTIONS(1),
    [anon_sym_COMPRESS] = ACTIONS(1),
    [sym_XCallX] = ACTIONS(1),
    [anon_sym_DOLLAR_AMP] = ACTIONS(1),
    [aux_sym_Write_token1] = ACTIONS(1),
    [aux_sym_Write_token2] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(365),
    [sym_line] = STATE(281),
    [sym_Write] = STATE(288),
    [sym_command] = STATE(277),
    [aux_sym_Write_token1] = ACTIONS(3),
    [aux_sym_Write_token2] = ACTIONS(5),
  },
  [2] = {
    [sym_Select] = STATE(105),
    [sym_View] = STATE(106),
    [sym_Text] = STATE(106),
    [sym_Translate] = STATE(106),
    [sym_Find] = STATE(106),
    [sym_Fnumber] = STATE(106),
    [sym_Random] = STATE(106),
    [sym_Reverse] = STATE(106),
    [sym_Piece] = STATE(106),
    [sym_Justify] = STATE(106),
    [sym_Extract] = STATE(106),
    [sym_Ascii] = STATE(106),
    [sym_Length] = STATE(106),
    [sym_Stack] = STATE(106),
    [sym_Name] = STATE(107),
    [sym_Order] = STATE(107),
    [sym_Query] = STATE(107),
    [sym_Get] = STATE(107),
    [sym_Increment] = STATE(107),
    [sym_Next] = STATE(107),
    [sym_Data] = STATE(107),
    [sym_Qlength] = STATE(107),
    [sym_Qsubscript] = STATE(107),
    [sym_VarFunctions] = STATE(105),
    [sym_Char] = STATE(106),
    [sym_ExpFunctions] = STATE(105),
    [sym_Device] = STATE(108),
    [sym_Ecode] = STATE(108),
    [sym_Estack] = STATE(108),
    [sym_Etrap] = STATE(108),
    [sym_Horolog] = STATE(108),
    [sym_Io] = STATE(108),
    [sym_Job] = STATE(108),
    [sym_Key] = STATE(108),
    [sym_Principal] = STATE(108),
    [sym_Quit] = STATE(108),
    [sym_Reference] = STATE(108),
    [sym_System] = STATE(108),
    [sym_Storage] = STATE(108),
    [sym_Test] = STATE(108),
    [sym_X] = STATE(108),
    [sym_Y] = STATE(108),
    [sym_StackVar] = STATE(108),
    [aux_sym_Select_token1] = ACTIONS(7),
    [aux_sym_Select_token2] = ACTIONS(9),
    [aux_sym_View_token1] = ACTIONS(11),
    [aux_sym_View_token2] = ACTIONS(13),
    [aux_sym_Text_token1] = ACTIONS(15),
    [aux_sym_Text_token2] = ACTIONS(17),
    [aux_sym_Translate_token1] = ACTIONS(19),
    [aux_sym_Translate_token2] = ACTIONS(21),
    [aux_sym_Find_token1] = ACTIONS(23),
    [aux_sym_Find_token2] = ACTIONS(25),
    [aux_sym_Fnumber_token1] = ACTIONS(27),
    [aux_sym_Fnumber_token2] = ACTIONS(29),
    [aux_sym_Random_token1] = ACTIONS(31),
    [aux_sym_Random_token2] = ACTIONS(33),
    [aux_sym_Reverse_token1] = ACTIONS(35),
    [aux_sym_Reverse_token2] = ACTIONS(37),
    [aux_sym_Piece_token1] = ACTIONS(39),
    [aux_sym_Piece_token2] = ACTIONS(41),
    [aux_sym_Justify_token1] = ACTIONS(43),
    [aux_sym_Justify_token2] = ACTIONS(45),
    [aux_sym_Extract_token1] = ACTIONS(47),
    [aux_sym_Extract_token2] = ACTIONS(49),
    [aux_sym_Ascii_token1] = ACTIONS(51),
    [aux_sym_Ascii_token2] = ACTIONS(53),
    [aux_sym_Length_token1] = ACTIONS(55),
    [aux_sym_Length_token2] = ACTIONS(57),
    [aux_sym_Stack_token1] = ACTIONS(59),
    [aux_sym_Stack_token2] = ACTIONS(61),
    [aux_sym_Name_token1] = ACTIONS(63),
    [aux_sym_Name_token2] = ACTIONS(65),
    [aux_sym_Order_token1] = ACTIONS(67),
    [aux_sym_Order_token2] = ACTIONS(69),
    [aux_sym_Query_token1] = ACTIONS(71),
    [aux_sym_Query_token2] = ACTIONS(73),
    [aux_sym_Get_token1] = ACTIONS(75),
    [aux_sym_Get_token2] = ACTIONS(77),
    [aux_sym_Increment_token1] = ACTIONS(79),
    [aux_sym_Increment_token2] = ACTIONS(81),
    [aux_sym_Next_token1] = ACTIONS(83),
    [aux_sym_Next_token2] = ACTIONS(85),
    [aux_sym_Data_token1] = ACTIONS(87),
    [aux_sym_Data_token2] = ACTIONS(89),
    [aux_sym_Qlength_token1] = ACTIONS(91),
    [aux_sym_Qlength_token2] = ACTIONS(93),
    [aux_sym_Qsubscript_token1] = ACTIONS(95),
    [aux_sym_Qsubscript_token2] = ACTIONS(97),
    [aux_sym_Char_token1] = ACTIONS(99),
    [aux_sym_Char_token2] = ACTIONS(101),
    [aux_sym_Device_token1] = ACTIONS(103),
    [aux_sym_Ecode_token1] = ACTIONS(105),
    [aux_sym_Ecode_token2] = ACTIONS(107),
    [aux_sym_Estack_token1] = ACTIONS(109),
    [aux_sym_Estack_token2] = ACTIONS(111),
    [aux_sym_Etrap_token1] = ACTIONS(113),
    [aux_sym_Etrap_token2] = ACTIONS(115),
    [aux_sym_Horolog_token1] = ACTIONS(117),
    [aux_sym_Horolog_token2] = ACTIONS(119),
    [aux_sym_Io_token1] = ACTIONS(121),
    [aux_sym_Job_token1] = ACTIONS(123),
    [aux_sym_Key_token1] = ACTIONS(125),
    [aux_sym_Key_token2] = ACTIONS(127),
    [aux_sym_Principal_token1] = ACTIONS(129),
    [aux_sym_Quit_token1] = ACTIONS(131),
    [aux_sym_Reference_token1] = ACTIONS(133),
    [aux_sym_System_token1] = ACTIONS(135),
    [aux_sym_System_token2] = ACTIONS(137),
    [aux_sym_Storage_token1] = ACTIONS(139),
    [aux_sym_Test_token1] = ACTIONS(141),
    [aux_sym_X_token1] = ACTIONS(143),
    [aux_sym_Y_token1] = ACTIONS(145),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 23,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(89), 1,
      sym_Expression,
    STATE(261), 1,
      sym_commandArg,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    ACTIONS(147), 3,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [87] = 24,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(179), 1,
      anon_sym_RPAREN,
    ACTIONS(181), 1,
      sym_VarUndefined,
    ACTIONS(183), 1,
      anon_sym_DOT,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(187), 1,
      sym_Expression,
    STATE(292), 2,
      sym_ByRef,
      sym__FunctionArg,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [176] = 24,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(183), 1,
      anon_sym_DOT,
    ACTIONS(185), 1,
      anon_sym_RPAREN,
    ACTIONS(187), 1,
      sym_VarUndefined,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(187), 1,
      sym_Expression,
    STATE(283), 2,
      sym_ByRef,
      sym__FunctionArg,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [265] = 24,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(183), 1,
      anon_sym_DOT,
    ACTIONS(189), 1,
      anon_sym_RPAREN,
    ACTIONS(191), 1,
      sym_VarUndefined,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(187), 1,
      sym_Expression,
    STATE(278), 2,
      sym_ByRef,
      sym__FunctionArg,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [354] = 23,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(183), 1,
      anon_sym_DOT,
    ACTIONS(193), 1,
      sym_VarUndefined,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(187), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(308), 2,
      sym_ByRef,
      sym__FunctionArg,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [440] = 7,
    ACTIONS(197), 1,
      anon_sym_LPAREN,
    ACTIONS(200), 1,
      anon_sym_DQUOTE,
    ACTIONS(205), 1,
      aux_sym_PaternElement_token1,
    STATE(81), 1,
      sym_string,
    STATE(8), 2,
      sym_PaternElement,
      aux_sym_Patern_repeat1,
    ACTIONS(203), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(195), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [493] = 7,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(210), 1,
      anon_sym_LPAREN,
    ACTIONS(214), 1,
      aux_sym_PaternElement_token1,
    STATE(81), 1,
      sym_string,
    STATE(8), 2,
      sym_PaternElement,
      aux_sym_Patern_repeat1,
    ACTIONS(212), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(208), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [546] = 22,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(89), 1,
      sym_Expression,
    STATE(269), 1,
      sym_commandArg,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [628] = 5,
    STATE(79), 1,
      sym_BinaryOpp,
    STATE(266), 1,
      sym_PatternOpp,
    ACTIONS(218), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(216), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [676] = 5,
    STATE(79), 1,
      sym_BinaryOpp,
    STATE(266), 1,
      sym_PatternOpp,
    ACTIONS(222), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(220), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [724] = 5,
    STATE(79), 1,
      sym_BinaryOpp,
    STATE(266), 1,
      sym_PatternOpp,
    ACTIONS(226), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(224), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [772] = 5,
    STATE(79), 1,
      sym_BinaryOpp,
    STATE(266), 1,
      sym_PatternOpp,
    ACTIONS(230), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(228), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [820] = 6,
    ACTIONS(232), 1,
      anon_sym_AT,
    STATE(79), 1,
      sym_BinaryOpp,
    STATE(266), 1,
      sym_PatternOpp,
    ACTIONS(230), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(228), 28,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [870] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(161), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [949] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(234), 1,
      anon_sym_AT,
    STATE(11), 1,
      sym_Expression,
    STATE(72), 1,
      sym_UnaryOpp,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1028] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(196), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1107] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(236), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1186] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(234), 1,
      anon_sym_AT,
    STATE(72), 1,
      sym_UnaryOpp,
    STATE(226), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1265] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(191), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1344] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(207), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1423] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(34), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1502] = 5,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(222), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(220), 28,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [1549] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(234), 1,
      anon_sym_AT,
    STATE(15), 1,
      sym_Expression,
    STATE(72), 1,
      sym_UnaryOpp,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1628] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(235), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1707] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(234), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1786] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(198), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1865] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(201), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [1944] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(189), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2023] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(232), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2102] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(73), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2181] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(197), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2260] = 5,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(218), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(216), 28,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [2307] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(190), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2386] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(113), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2465] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(193), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2544] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(194), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2623] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(208), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2702] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(209), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2781] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(210), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2860] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(123), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [2939] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(211), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3018] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(206), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3097] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(192), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3176] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(212), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3255] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(234), 1,
      anon_sym_AT,
    STATE(14), 1,
      sym_Expression,
    STATE(72), 1,
      sym_UnaryOpp,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3334] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(213), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3413] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(199), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3492] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(188), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3571] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(204), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3650] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(203), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3729] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(223), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3808] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(222), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3887] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(227), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [3966] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(221), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4045] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(228), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4124] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(220), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4203] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(219), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4282] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(202), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4361] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(218), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4440] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(217), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4519] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(229), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4598] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(216), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4677] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(215), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4756] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(230), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4835] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(214), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4914] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(237), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [4993] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(231), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5072] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(224), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5151] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(200), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5230] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(234), 1,
      anon_sym_AT,
    STATE(13), 1,
      sym_Expression,
    STATE(72), 1,
      sym_UnaryOpp,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5309] = 5,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(226), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(224), 28,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [5356] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(205), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5435] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(225), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5514] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(24), 1,
      sym_Expression,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5593] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(195), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5672] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(167), 1,
      anon_sym_AT,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    STATE(32), 1,
      sym_UnaryOpp,
    STATE(233), 1,
      sym_Expression,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5751] = 21,
    ACTIONS(149), 1,
      anon_sym_DOLLAR_DOLLAR,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(153), 1,
      anon_sym_LPAREN,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(159), 1,
      anon_sym_PLUS,
    ACTIONS(161), 1,
      anon_sym_DASH,
    ACTIONS(163), 1,
      aux_sym_number_token1,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(169), 1,
      anon_sym_SQUOTE,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(175), 1,
      anon_sym_DOLLAR,
    ACTIONS(177), 1,
      anon_sym_DOLLAR_AMP,
    ACTIONS(234), 1,
      anon_sym_AT,
    STATE(12), 1,
      sym_Expression,
    STATE(72), 1,
      sym_UnaryOpp,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(247), 3,
      sym_OPNOT,
      sym_OPPLUS,
      sym_OPMINUS,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
    STATE(168), 11,
      sym_ExtrinsicFunction,
      sym_number,
      sym_string,
      sym_InderectExpression,
      sym_UnaryExpression,
      sym_BinaryExpression,
      sym_PaternMatchExpression,
      sym_Variable,
      sym_IntrinsicFunction,
      sym_IntrinsicVar,
      sym_XCall,
  [5830] = 2,
    ACTIONS(238), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(236), 32,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DQUOTE,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      aux_sym_PaternElement_token1,
      anon_sym_PIPE,
      anon_sym_COLON,
  [5870] = 2,
    ACTIONS(242), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(240), 32,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DQUOTE,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      aux_sym_PaternElement_token1,
      anon_sym_PIPE,
      anon_sym_COLON,
  [5910] = 2,
    ACTIONS(246), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(244), 32,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DQUOTE,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      aux_sym_PaternElement_token1,
      anon_sym_PIPE,
      anon_sym_COLON,
  [5950] = 2,
    ACTIONS(250), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(248), 32,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DQUOTE,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      aux_sym_PaternElement_token1,
      anon_sym_PIPE,
      anon_sym_COLON,
  [5990] = 2,
    ACTIONS(254), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(252), 32,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DQUOTE,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      aux_sym_PaternElement_token1,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6030] = 4,
    ACTIONS(258), 1,
      anon_sym_LPAREN,
    STATE(150), 1,
      sym__VariableSubscripts,
    ACTIONS(260), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(256), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6073] = 4,
    ACTIONS(258), 1,
      anon_sym_LPAREN,
    STATE(176), 1,
      sym__VariableSubscripts,
    ACTIONS(264), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(262), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6116] = 3,
    ACTIONS(268), 1,
      anon_sym_LPAREN,
    ACTIONS(270), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(266), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6156] = 3,
    ACTIONS(274), 1,
      anon_sym_DOT,
    ACTIONS(276), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(272), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6196] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(278), 4,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [6250] = 3,
    ACTIONS(294), 1,
      aux_sym_number_token1,
    ACTIONS(296), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(292), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6290] = 3,
    ACTIONS(300), 1,
      anon_sym_LPAREN,
    ACTIONS(302), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(298), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6330] = 3,
    ACTIONS(306), 1,
      anon_sym_LPAREN,
    ACTIONS(308), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(304), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6370] = 3,
    ACTIONS(312), 1,
      anon_sym_LPAREN,
    ACTIONS(314), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(310), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6410] = 3,
    ACTIONS(318), 1,
      anon_sym_LPAREN,
    ACTIONS(320), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(316), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6450] = 3,
    ACTIONS(324), 1,
      anon_sym_LPAREN,
    ACTIONS(326), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(322), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6490] = 3,
    ACTIONS(330), 1,
      anon_sym_LPAREN,
    ACTIONS(332), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(328), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6530] = 3,
    ACTIONS(336), 1,
      anon_sym_LPAREN,
    ACTIONS(338), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(334), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6570] = 3,
    ACTIONS(342), 1,
      anon_sym_LPAREN,
    ACTIONS(344), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(340), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6610] = 2,
    ACTIONS(348), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(346), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6647] = 2,
    ACTIONS(352), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(350), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6684] = 2,
    ACTIONS(302), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(298), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6721] = 2,
    ACTIONS(308), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(304), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6758] = 2,
    ACTIONS(356), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(354), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6795] = 2,
    ACTIONS(360), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(358), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6832] = 2,
    ACTIONS(364), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(362), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6869] = 2,
    ACTIONS(368), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(366), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6906] = 2,
    ACTIONS(372), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(370), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6943] = 2,
    ACTIONS(376), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(374), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [6980] = 2,
    ACTIONS(380), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(378), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7017] = 2,
    ACTIONS(384), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(382), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7054] = 2,
    ACTIONS(388), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(386), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7091] = 2,
    ACTIONS(392), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(390), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7128] = 12,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(394), 1,
      anon_sym_COMMA,
    ACTIONS(396), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    STATE(299), 1,
      aux_sym_Select_repeat1,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [7185] = 2,
    ACTIONS(400), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(398), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7222] = 2,
    ACTIONS(404), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(402), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7259] = 2,
    ACTIONS(408), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(406), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7296] = 2,
    ACTIONS(412), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(410), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7333] = 2,
    ACTIONS(416), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(414), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7370] = 2,
    ACTIONS(420), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(418), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7407] = 2,
    ACTIONS(424), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(422), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7444] = 2,
    ACTIONS(428), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(426), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7481] = 2,
    ACTIONS(432), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(430), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7518] = 12,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(434), 1,
      anon_sym_COMMA,
    ACTIONS(436), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    STATE(293), 1,
      aux_sym__VariableSubscripts_repeat1,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [7575] = 2,
    ACTIONS(440), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(438), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7612] = 2,
    ACTIONS(444), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(442), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7649] = 2,
    ACTIONS(448), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(446), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7686] = 2,
    ACTIONS(452), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(450), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7723] = 2,
    ACTIONS(456), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(454), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7760] = 2,
    ACTIONS(460), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(458), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7797] = 2,
    ACTIONS(464), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(462), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7834] = 2,
    ACTIONS(338), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(334), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7871] = 2,
    ACTIONS(314), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(310), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7908] = 2,
    ACTIONS(468), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(466), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7945] = 2,
    ACTIONS(344), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(340), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [7982] = 2,
    ACTIONS(472), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(470), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8019] = 2,
    ACTIONS(476), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(474), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8056] = 2,
    ACTIONS(320), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(316), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8093] = 2,
    ACTIONS(480), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(478), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8130] = 2,
    ACTIONS(484), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(482), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8167] = 2,
    ACTIONS(488), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(486), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8204] = 2,
    ACTIONS(492), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(490), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8241] = 2,
    ACTIONS(496), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(494), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8278] = 2,
    ACTIONS(500), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(498), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8315] = 2,
    ACTIONS(504), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(502), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8352] = 2,
    ACTIONS(508), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(506), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8389] = 2,
    ACTIONS(512), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(510), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8426] = 2,
    ACTIONS(516), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(514), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8463] = 2,
    ACTIONS(326), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(322), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8500] = 2,
    ACTIONS(520), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(518), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8537] = 2,
    ACTIONS(524), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(522), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8574] = 2,
    ACTIONS(296), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(292), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8611] = 2,
    ACTIONS(528), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(526), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8648] = 2,
    ACTIONS(532), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(530), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8685] = 2,
    ACTIONS(536), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(534), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8722] = 2,
    ACTIONS(540), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(538), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8759] = 2,
    ACTIONS(544), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(542), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8796] = 2,
    ACTIONS(548), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(546), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8833] = 2,
    ACTIONS(552), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(550), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8870] = 2,
    ACTIONS(556), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(554), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8907] = 2,
    ACTIONS(270), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(266), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [8944] = 12,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(558), 1,
      anon_sym_COMMA,
    ACTIONS(560), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    STATE(286), 1,
      aux_sym_Char_repeat1,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [9001] = 2,
    ACTIONS(564), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(562), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9038] = 2,
    ACTIONS(568), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(566), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9075] = 2,
    ACTIONS(572), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(570), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9112] = 2,
    ACTIONS(576), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(574), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9149] = 2,
    ACTIONS(580), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(578), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9186] = 2,
    ACTIONS(584), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(582), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9223] = 2,
    ACTIONS(588), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(586), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9260] = 2,
    ACTIONS(592), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(590), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9297] = 2,
    ACTIONS(596), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(594), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9334] = 2,
    ACTIONS(600), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(598), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9371] = 2,
    ACTIONS(604), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(602), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9408] = 2,
    ACTIONS(608), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(606), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9445] = 2,
    ACTIONS(612), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(610), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9482] = 2,
    ACTIONS(616), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(614), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9519] = 2,
    ACTIONS(620), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(618), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9556] = 2,
    ACTIONS(624), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(622), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9593] = 2,
    ACTIONS(628), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(626), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9630] = 2,
    ACTIONS(632), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(630), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9667] = 2,
    ACTIONS(636), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(634), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9704] = 2,
    ACTIONS(640), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(638), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9741] = 2,
    ACTIONS(644), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(642), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9778] = 2,
    ACTIONS(648), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(646), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9815] = 2,
    ACTIONS(652), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(650), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9852] = 2,
    ACTIONS(656), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(654), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9889] = 2,
    ACTIONS(660), 3,
      sym_OPMUL,
      anon_sym_RBRACK,
      sym_OPNFOL,
    ACTIONS(658), 29,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_AT,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
      sym_OPPAT,
      sym_OPNPAT,
      anon_sym_PIPE,
      anon_sym_COLON,
  [9926] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    ACTIONS(662), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [9978] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(664), 1,
      anon_sym_COMMA,
    ACTIONS(666), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10032] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(668), 1,
      anon_sym_COMMA,
    ACTIONS(670), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10086] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(672), 1,
      anon_sym_COMMA,
    ACTIONS(674), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10140] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(676), 1,
      anon_sym_COMMA,
    ACTIONS(678), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10194] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(680), 1,
      anon_sym_COMMA,
    ACTIONS(682), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10248] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(684), 1,
      anon_sym_COMMA,
    ACTIONS(686), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10302] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(688), 1,
      anon_sym_COMMA,
    ACTIONS(690), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10356] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    ACTIONS(692), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10408] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    ACTIONS(694), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10460] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    ACTIONS(696), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10512] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(698), 1,
      anon_sym_COMMA,
    ACTIONS(700), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10566] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(702), 1,
      anon_sym_COMMA,
    ACTIONS(704), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10620] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(706), 1,
      anon_sym_COMMA,
    ACTIONS(708), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10674] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(710), 1,
      anon_sym_COMMA,
    ACTIONS(712), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10728] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(714), 1,
      anon_sym_COMMA,
    ACTIONS(716), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10782] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(718), 1,
      anon_sym_COMMA,
    ACTIONS(720), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10836] = 11,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(722), 1,
      anon_sym_COMMA,
    ACTIONS(724), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10890] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(726), 1,
      anon_sym_COLON,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10941] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(728), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [10992] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(730), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11043] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(732), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11094] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(734), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11145] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(736), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11196] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(738), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11247] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(740), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11298] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(742), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11349] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(744), 1,
      anon_sym_COLON,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11400] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(746), 1,
      anon_sym_COMMA,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11451] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(748), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11502] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(750), 1,
      anon_sym_COMMA,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11553] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(752), 1,
      anon_sym_COMMA,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11604] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(754), 1,
      anon_sym_COMMA,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11655] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(756), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11706] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(758), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11757] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(760), 1,
      anon_sym_COMMA,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11808] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(762), 1,
      anon_sym_COMMA,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11859] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(764), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11910] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(766), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [11961] = 10,
    ACTIONS(232), 1,
      anon_sym_AT,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    STATE(79), 1,
      sym_BinaryOpp,
    STATE(266), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12012] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(768), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12063] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(770), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12114] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(772), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12165] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(774), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12216] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(776), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12267] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(778), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12318] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(780), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12369] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(782), 1,
      anon_sym_COMMA,
    ACTIONS(784), 1,
      anon_sym_RBRACK,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12420] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(786), 1,
      anon_sym_PIPE,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12471] = 10,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(288), 1,
      anon_sym_RBRACK,
    ACTIONS(788), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12522] = 9,
    ACTIONS(280), 1,
      anon_sym_PLUS,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(790), 1,
      anon_sym_RBRACK,
    STATE(76), 1,
      sym_BinaryOpp,
    STATE(264), 1,
      sym_PatternOpp,
    ACTIONS(284), 2,
      sym_OPMUL,
      sym_OPNFOL,
    ACTIONS(290), 2,
      sym_OPPAT,
      sym_OPNPAT,
    STATE(245), 3,
      sym_OPADD,
      sym_OPSUB,
      sym_OPFOL,
    ACTIONS(286), 17,
      sym_OPDIV,
      sym_OPINT,
      sym_OPMOD,
      sym_OPCAT,
      sym_OPGTR,
      sym_OPAND,
      sym_OPCON,
      sym_OPEQL,
      sym_OPLES,
      sym_OPPOW,
      sym_OPNEQL,
      sym_OPNLES,
      sym_OPNGTR,
      sym_OPNAND,
      sym_OPNCON,
      sym_OPSAF,
      sym_OPNSAF,
  [12570] = 4,
    ACTIONS(792), 1,
      anon_sym_PERCENT,
    ACTIONS(796), 1,
      sym_XCallX,
    ACTIONS(794), 5,
      sym_E,
      sym_Paschk,
      sym_V,
      sym_Xrsm,
      sym_Debug,
    STATE(326), 16,
      sym_Directory,
      sym_Host,
      sym_File,
      sym_ErrMsg,
      sym_OpCom,
      sym_Signal,
      sym_Spawn,
      sym_Version,
      sym_Zwrite,
      sym_SetEnv,
      sym_GetEnv,
      sym_RouChk,
      sym_Fork,
      sym_IC,
      sym_Wait,
      sym_Compress,
  [12602] = 16,
    ACTIONS(798), 1,
      anon_sym_DIRECTORY,
    ACTIONS(800), 1,
      anon_sym_HOST,
    ACTIONS(802), 1,
      anon_sym_FILE,
    ACTIONS(804), 1,
      anon_sym_ERRMSG,
    ACTIONS(806), 1,
      anon_sym_OPCOM,
    ACTIONS(808), 1,
      anon_sym_SIGNAL,
    ACTIONS(810), 1,
      anon_sym_SPAWN,
    ACTIONS(812), 1,
      anon_sym_VERSION,
    ACTIONS(814), 1,
      anon_sym_ZWRITE,
    ACTIONS(816), 1,
      anon_sym_SETENV,
    ACTIONS(818), 1,
      anon_sym_GETENV,
    ACTIONS(820), 1,
      anon_sym_ROUCHK,
    ACTIONS(822), 1,
      anon_sym_FORK,
    ACTIONS(824), 1,
      anon_sym_IC,
    ACTIONS(826), 1,
      anon_sym_WAIT,
    ACTIONS(828), 1,
      anon_sym_COMPRESS,
  [12651] = 2,
    ACTIONS(832), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(830), 13,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12671] = 2,
    ACTIONS(836), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(834), 13,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12691] = 2,
    ACTIONS(840), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(838), 13,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12711] = 3,
    ACTIONS(846), 1,
      sym_identifier,
    ACTIONS(844), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(842), 12,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12733] = 2,
    ACTIONS(850), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(848), 13,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12753] = 2,
    ACTIONS(854), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(852), 13,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12773] = 2,
    ACTIONS(844), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(842), 13,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12793] = 2,
    ACTIONS(858), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(856), 13,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12813] = 2,
    ACTIONS(862), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(860), 13,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      sym_identifier,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12833] = 3,
    ACTIONS(864), 1,
      sym_identifier,
    ACTIONS(844), 2,
      anon_sym_CARET,
      anon_sym_DOLLAR,
    ACTIONS(842), 12,
      anon_sym_DOLLAR_DOLLAR,
      anon_sym_LPAREN,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_number_token1,
      anon_sym_DQUOTE,
      anon_sym_AT,
      anon_sym_SQUOTE,
      anon_sym_CARET_PIPE,
      anon_sym_CARET_LBRACK,
      anon_sym_DOLLAR_AMP,
  [12855] = 9,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(866), 1,
      aux_sym_number_token1,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(303), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [12886] = 8,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(338), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [12914] = 8,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(302), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [12942] = 8,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(304), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [12970] = 8,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(307), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [12998] = 8,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(309), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [13026] = 8,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(301), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [13054] = 8,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(339), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [13082] = 8,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(314), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [13110] = 8,
    ACTIONS(151), 1,
      anon_sym_CARET,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(171), 1,
      anon_sym_CARET_PIPE,
    ACTIONS(173), 1,
      anon_sym_CARET_LBRACK,
    ACTIONS(868), 1,
      anon_sym_AT,
    STATE(341), 1,
      sym_Variable,
    STATE(305), 2,
      sym_IndirectVariable,
      sym_NakedVariable,
    STATE(343), 3,
      sym_GlobalVariable,
      sym_GlobalUciVariable,
      sym_GlobalUciEnvVariable,
  [13138] = 5,
    ACTIONS(165), 1,
      anon_sym_DQUOTE,
    ACTIONS(210), 1,
      anon_sym_LPAREN,
    ACTIONS(214), 1,
      aux_sym_PaternElement_token1,
    STATE(81), 1,
      sym_string,
    STATE(9), 2,
      sym_PaternElement,
      aux_sym_Patern_repeat1,
  [13155] = 3,
    ACTIONS(872), 1,
      anon_sym_COMMA,
    STATE(265), 1,
      aux_sym_Write_repeat1,
    ACTIONS(870), 3,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
  [13167] = 3,
    ACTIONS(876), 1,
      anon_sym_COMMA,
    STATE(262), 1,
      aux_sym_Write_repeat1,
    ACTIONS(874), 3,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
  [13179] = 5,
    ACTIONS(3), 1,
      aux_sym_Write_token1,
    ACTIONS(5), 1,
      aux_sym_Write_token2,
    STATE(277), 1,
      sym_command,
    STATE(288), 1,
      sym_Write,
    STATE(306), 1,
      sym_line,
  [13195] = 5,
    ACTIONS(879), 1,
      anon_sym_AT,
    ACTIONS(881), 1,
      aux_sym_PaternRepetitionCount_token1,
    ACTIONS(883), 1,
      aux_sym_PaternRepetitionCount_token2,
    STATE(174), 1,
      sym_Patern,
    STATE(260), 1,
      sym_PaternRepetitionCount,
  [13211] = 3,
    ACTIONS(872), 1,
      anon_sym_COMMA,
    STATE(262), 1,
      aux_sym_Write_repeat1,
    ACTIONS(885), 3,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
  [13223] = 5,
    ACTIONS(881), 1,
      aux_sym_PaternRepetitionCount_token1,
    ACTIONS(883), 1,
      aux_sym_PaternRepetitionCount_token2,
    ACTIONS(887), 1,
      anon_sym_AT,
    STATE(174), 1,
      sym_Patern,
    STATE(260), 1,
      sym_PaternRepetitionCount,
  [13239] = 4,
    ACTIONS(889), 1,
      anon_sym_DQUOTE,
    ACTIONS(891), 1,
      aux_sym_string_token1,
    ACTIONS(894), 1,
      anon_sym_DQUOTE_DQUOTE,
    STATE(267), 1,
      aux_sym_string_repeat1,
  [13252] = 4,
    ACTIONS(881), 1,
      aux_sym_PaternRepetitionCount_token1,
    ACTIONS(883), 1,
      aux_sym_PaternRepetitionCount_token2,
    STATE(260), 1,
      sym_PaternRepetitionCount,
    STATE(311), 1,
      sym_Patern,
  [13265] = 1,
    ACTIONS(874), 4,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
      anon_sym_COMMA,
  [13272] = 4,
    ACTIONS(881), 1,
      aux_sym_PaternRepetitionCount_token1,
    ACTIONS(883), 1,
      aux_sym_PaternRepetitionCount_token2,
    STATE(260), 1,
      sym_PaternRepetitionCount,
    STATE(285), 1,
      sym_Patern,
  [13285] = 3,
    ACTIONS(899), 1,
      anon_sym_,
    STATE(271), 1,
      aux_sym_line_repeat1,
    ACTIONS(897), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [13296] = 3,
    ACTIONS(904), 1,
      anon_sym_CARET,
    STATE(310), 1,
      sym__Tag,
    ACTIONS(902), 2,
      sym_NumericIdentifier,
      sym_identifier,
  [13307] = 3,
    ACTIONS(908), 1,
      anon_sym_,
    STATE(271), 1,
      aux_sym_line_repeat1,
    ACTIONS(906), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [13318] = 4,
    ACTIONS(3), 1,
      aux_sym_Write_token1,
    ACTIONS(5), 1,
      aux_sym_Write_token2,
    STATE(288), 1,
      sym_Write,
    STATE(290), 1,
      sym_command,
  [13331] = 4,
    ACTIONS(910), 1,
      anon_sym_DQUOTE,
    ACTIONS(912), 1,
      aux_sym_string_token1,
    ACTIONS(914), 1,
      anon_sym_DQUOTE_DQUOTE,
    STATE(267), 1,
      aux_sym_string_repeat1,
  [13344] = 4,
    ACTIONS(916), 1,
      anon_sym_DQUOTE,
    ACTIONS(918), 1,
      aux_sym_string_token1,
    ACTIONS(920), 1,
      anon_sym_DQUOTE_DQUOTE,
    STATE(275), 1,
      aux_sym_string_repeat1,
  [13357] = 3,
    ACTIONS(908), 1,
      anon_sym_,
    STATE(273), 1,
      aux_sym_line_repeat1,
    ACTIONS(922), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [13368] = 3,
    ACTIONS(924), 1,
      anon_sym_COMMA,
    ACTIONS(926), 1,
      anon_sym_RPAREN,
    STATE(284), 1,
      aux_sym_ExtrinsicFunction_repeat1,
  [13378] = 3,
    ACTIONS(928), 1,
      ts_builtin_sym_end,
    ACTIONS(930), 1,
      anon_sym_LF,
    STATE(296), 1,
      aux_sym_source_file_repeat1,
  [13388] = 3,
    ACTIONS(932), 1,
      anon_sym_COMMA,
    ACTIONS(935), 1,
      anon_sym_RPAREN,
    STATE(280), 1,
      aux_sym_ExtrinsicFunction_repeat1,
  [13398] = 3,
    ACTIONS(930), 1,
      anon_sym_LF,
    ACTIONS(937), 1,
      ts_builtin_sym_end,
    STATE(279), 1,
      aux_sym_source_file_repeat1,
  [13408] = 3,
    ACTIONS(694), 1,
      anon_sym_RPAREN,
    ACTIONS(939), 1,
      anon_sym_COMMA,
    STATE(282), 1,
      aux_sym__VariableSubscripts_repeat1,
  [13418] = 3,
    ACTIONS(924), 1,
      anon_sym_COMMA,
    ACTIONS(942), 1,
      anon_sym_RPAREN,
    STATE(298), 1,
      aux_sym_ExtrinsicFunction_repeat1,
  [13428] = 3,
    ACTIONS(924), 1,
      anon_sym_COMMA,
    ACTIONS(944), 1,
      anon_sym_RPAREN,
    STATE(280), 1,
      aux_sym_ExtrinsicFunction_repeat1,
  [13438] = 3,
    ACTIONS(946), 1,
      anon_sym_COMMA,
    ACTIONS(948), 1,
      anon_sym_RPAREN,
    STATE(297), 1,
      aux_sym_PaternElement_repeat1,
  [13448] = 3,
    ACTIONS(558), 1,
      anon_sym_COMMA,
    ACTIONS(950), 1,
      anon_sym_RPAREN,
    STATE(294), 1,
      aux_sym_Char_repeat1,
  [13458] = 2,
    ACTIONS(954), 1,
      aux_sym_PaternRepetitionCount_token1,
    ACTIONS(952), 2,
      anon_sym_AT,
      aux_sym_PaternRepetitionCount_token2,
  [13466] = 1,
    ACTIONS(956), 3,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
  [13472] = 3,
    ACTIONS(924), 1,
      anon_sym_COMMA,
    ACTIONS(958), 1,
      anon_sym_RPAREN,
    STATE(280), 1,
      aux_sym_ExtrinsicFunction_repeat1,
  [13482] = 1,
    ACTIONS(897), 3,
      ts_builtin_sym_end,
      anon_sym_LF,
      anon_sym_,
  [13488] = 1,
    ACTIONS(960), 3,
      anon_sym_LPAREN,
      anon_sym_DQUOTE,
      aux_sym_PaternElement_token1,
  [13494] = 3,
    ACTIONS(924), 1,
      anon_sym_COMMA,
    ACTIONS(962), 1,
      anon_sym_RPAREN,
    STATE(289), 1,
      aux_sym_ExtrinsicFunction_repeat1,
  [13504] = 3,
    ACTIONS(434), 1,
      anon_sym_COMMA,
    ACTIONS(964), 1,
      anon_sym_RPAREN,
    STATE(282), 1,
      aux_sym__VariableSubscripts_repeat1,
  [13514] = 3,
    ACTIONS(966), 1,
      anon_sym_COMMA,
    ACTIONS(969), 1,
      anon_sym_RPAREN,
    STATE(294), 1,
      aux_sym_Char_repeat1,
  [13524] = 3,
    ACTIONS(971), 1,
      anon_sym_COMMA,
    ACTIONS(974), 1,
      anon_sym_RPAREN,
    STATE(295), 1,
      aux_sym_PaternElement_repeat1,
  [13534] = 3,
    ACTIONS(976), 1,
      ts_builtin_sym_end,
    ACTIONS(978), 1,
      anon_sym_LF,
    STATE(296), 1,
      aux_sym_source_file_repeat1,
  [13544] = 3,
    ACTIONS(946), 1,
      anon_sym_COMMA,
    ACTIONS(981), 1,
      anon_sym_RPAREN,
    STATE(295), 1,
      aux_sym_PaternElement_repeat1,
  [13554] = 3,
    ACTIONS(924), 1,
      anon_sym_COMMA,
    ACTIONS(983), 1,
      anon_sym_RPAREN,
    STATE(280), 1,
      aux_sym_ExtrinsicFunction_repeat1,
  [13564] = 3,
    ACTIONS(394), 1,
      anon_sym_COMMA,
    ACTIONS(985), 1,
      anon_sym_RPAREN,
    STATE(300), 1,
      aux_sym_Select_repeat1,
  [13574] = 3,
    ACTIONS(987), 1,
      anon_sym_COMMA,
    ACTIONS(990), 1,
      anon_sym_RPAREN,
    STATE(300), 1,
      aux_sym_Select_repeat1,
  [13584] = 2,
    ACTIONS(992), 1,
      anon_sym_COMMA,
    ACTIONS(994), 1,
      anon_sym_RPAREN,
  [13591] = 2,
    ACTIONS(996), 1,
      anon_sym_COMMA,
    ACTIONS(998), 1,
      anon_sym_RPAREN,
  [13598] = 1,
    ACTIONS(1000), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [13603] = 2,
    ACTIONS(1002), 1,
      anon_sym_COMMA,
    ACTIONS(1004), 1,
      anon_sym_RPAREN,
  [13610] = 2,
    ACTIONS(258), 1,
      anon_sym_LPAREN,
    STATE(118), 1,
      sym__VariableSubscripts,
  [13617] = 1,
    ACTIONS(976), 2,
      ts_builtin_sym_end,
      anon_sym_LF,
  [13622] = 2,
    ACTIONS(1006), 1,
      anon_sym_COMMA,
    ACTIONS(1008), 1,
      anon_sym_RPAREN,
  [13629] = 1,
    ACTIONS(1010), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [13634] = 2,
    ACTIONS(1012), 1,
      anon_sym_COMMA,
    ACTIONS(1014), 1,
      anon_sym_RPAREN,
  [13641] = 2,
    ACTIONS(1016), 1,
      anon_sym_CARET,
    ACTIONS(1018), 1,
      anon_sym_LPAREN,
  [13648] = 1,
    ACTIONS(974), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [13653] = 2,
    ACTIONS(1020), 1,
      anon_sym_LPAREN,
    ACTIONS(1022), 1,
      sym_identifier,
  [13660] = 1,
    ACTIONS(336), 1,
      anon_sym_LPAREN,
  [13664] = 1,
    ACTIONS(1024), 1,
      anon_sym_RPAREN,
  [13668] = 1,
    ACTIONS(312), 1,
      anon_sym_LPAREN,
  [13672] = 1,
    ACTIONS(1026), 1,
      anon_sym_LPAREN,
  [13676] = 1,
    ACTIONS(1028), 1,
      anon_sym_LPAREN,
  [13680] = 1,
    ACTIONS(1030), 1,
      anon_sym_LPAREN,
  [13684] = 1,
    ACTIONS(306), 1,
      anon_sym_LPAREN,
  [13688] = 1,
    ACTIONS(1032), 1,
      anon_sym_LPAREN,
  [13692] = 1,
    ACTIONS(300), 1,
      anon_sym_LPAREN,
  [13696] = 1,
    ACTIONS(1034), 1,
      anon_sym_LPAREN,
  [13700] = 1,
    ACTIONS(1036), 1,
      anon_sym_LPAREN,
  [13704] = 1,
    ACTIONS(1038), 1,
      anon_sym_LPAREN,
  [13708] = 1,
    ACTIONS(1040), 1,
      anon_sym_LPAREN,
  [13712] = 1,
    ACTIONS(1042), 1,
      anon_sym_LPAREN,
  [13716] = 1,
    ACTIONS(1044), 1,
      sym_identifier,
  [13720] = 1,
    ACTIONS(318), 1,
      anon_sym_LPAREN,
  [13724] = 1,
    ACTIONS(324), 1,
      anon_sym_LPAREN,
  [13728] = 1,
    ACTIONS(1046), 1,
      anon_sym_LPAREN,
  [13732] = 1,
    ACTIONS(1048), 1,
      anon_sym_LPAREN,
  [13736] = 1,
    ACTIONS(1050), 1,
      anon_sym_LPAREN,
  [13740] = 1,
    ACTIONS(1052), 1,
      anon_sym_LPAREN,
  [13744] = 1,
    ACTIONS(1054), 1,
      anon_sym_LPAREN,
  [13748] = 1,
    ACTIONS(342), 1,
      anon_sym_LPAREN,
  [13752] = 1,
    ACTIONS(1056), 1,
      anon_sym_LPAREN,
  [13756] = 1,
    ACTIONS(1058), 1,
      anon_sym_LPAREN,
  [13760] = 1,
    ACTIONS(1060), 1,
      anon_sym_COMMA,
  [13764] = 1,
    ACTIONS(1062), 1,
      anon_sym_RPAREN,
  [13768] = 1,
    ACTIONS(1064), 1,
      anon_sym_LPAREN,
  [13772] = 1,
    ACTIONS(1066), 1,
      anon_sym_RPAREN,
  [13776] = 1,
    ACTIONS(268), 1,
      anon_sym_LPAREN,
  [13780] = 1,
    ACTIONS(1068), 1,
      sym_identifier,
  [13784] = 1,
    ACTIONS(1070), 1,
      anon_sym_LPAREN,
  [13788] = 1,
    ACTIONS(1072), 1,
      anon_sym_LPAREN,
  [13792] = 1,
    ACTIONS(1074), 1,
      anon_sym_LPAREN,
  [13796] = 1,
    ACTIONS(1076), 1,
      anon_sym_LPAREN,
  [13800] = 1,
    ACTIONS(1078), 1,
      anon_sym_LPAREN,
  [13804] = 1,
    ACTIONS(1080), 1,
      anon_sym_LPAREN,
  [13808] = 1,
    ACTIONS(1082), 1,
      anon_sym_LPAREN,
  [13812] = 1,
    ACTIONS(1084), 1,
      anon_sym_LPAREN,
  [13816] = 1,
    ACTIONS(866), 1,
      aux_sym_number_token1,
  [13820] = 1,
    ACTIONS(1086), 1,
      anon_sym_,
  [13824] = 1,
    ACTIONS(1088), 1,
      anon_sym_LPAREN,
  [13828] = 1,
    ACTIONS(1090), 1,
      anon_sym_LPAREN,
  [13832] = 1,
    ACTIONS(864), 1,
      sym_identifier,
  [13836] = 1,
    ACTIONS(1092), 1,
      anon_sym_LPAREN,
  [13840] = 1,
    ACTIONS(1094), 1,
      anon_sym_LPAREN,
  [13844] = 1,
    ACTIONS(1096), 1,
      anon_sym_LPAREN,
  [13848] = 1,
    ACTIONS(1098), 1,
      anon_sym_LPAREN,
  [13852] = 1,
    ACTIONS(1100), 1,
      anon_sym_LPAREN,
  [13856] = 1,
    ACTIONS(1102), 1,
      anon_sym_LPAREN,
  [13860] = 1,
    ACTIONS(1104), 1,
      sym_identifier,
  [13864] = 1,
    ACTIONS(1106), 1,
      anon_sym_LPAREN,
  [13868] = 1,
    ACTIONS(1108), 1,
      ts_builtin_sym_end,
  [13872] = 1,
    ACTIONS(1110), 1,
      anon_sym_LPAREN,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(3)] = 0,
  [SMALL_STATE(4)] = 87,
  [SMALL_STATE(5)] = 176,
  [SMALL_STATE(6)] = 265,
  [SMALL_STATE(7)] = 354,
  [SMALL_STATE(8)] = 440,
  [SMALL_STATE(9)] = 493,
  [SMALL_STATE(10)] = 546,
  [SMALL_STATE(11)] = 628,
  [SMALL_STATE(12)] = 676,
  [SMALL_STATE(13)] = 724,
  [SMALL_STATE(14)] = 772,
  [SMALL_STATE(15)] = 820,
  [SMALL_STATE(16)] = 870,
  [SMALL_STATE(17)] = 949,
  [SMALL_STATE(18)] = 1028,
  [SMALL_STATE(19)] = 1107,
  [SMALL_STATE(20)] = 1186,
  [SMALL_STATE(21)] = 1265,
  [SMALL_STATE(22)] = 1344,
  [SMALL_STATE(23)] = 1423,
  [SMALL_STATE(24)] = 1502,
  [SMALL_STATE(25)] = 1549,
  [SMALL_STATE(26)] = 1628,
  [SMALL_STATE(27)] = 1707,
  [SMALL_STATE(28)] = 1786,
  [SMALL_STATE(29)] = 1865,
  [SMALL_STATE(30)] = 1944,
  [SMALL_STATE(31)] = 2023,
  [SMALL_STATE(32)] = 2102,
  [SMALL_STATE(33)] = 2181,
  [SMALL_STATE(34)] = 2260,
  [SMALL_STATE(35)] = 2307,
  [SMALL_STATE(36)] = 2386,
  [SMALL_STATE(37)] = 2465,
  [SMALL_STATE(38)] = 2544,
  [SMALL_STATE(39)] = 2623,
  [SMALL_STATE(40)] = 2702,
  [SMALL_STATE(41)] = 2781,
  [SMALL_STATE(42)] = 2860,
  [SMALL_STATE(43)] = 2939,
  [SMALL_STATE(44)] = 3018,
  [SMALL_STATE(45)] = 3097,
  [SMALL_STATE(46)] = 3176,
  [SMALL_STATE(47)] = 3255,
  [SMALL_STATE(48)] = 3334,
  [SMALL_STATE(49)] = 3413,
  [SMALL_STATE(50)] = 3492,
  [SMALL_STATE(51)] = 3571,
  [SMALL_STATE(52)] = 3650,
  [SMALL_STATE(53)] = 3729,
  [SMALL_STATE(54)] = 3808,
  [SMALL_STATE(55)] = 3887,
  [SMALL_STATE(56)] = 3966,
  [SMALL_STATE(57)] = 4045,
  [SMALL_STATE(58)] = 4124,
  [SMALL_STATE(59)] = 4203,
  [SMALL_STATE(60)] = 4282,
  [SMALL_STATE(61)] = 4361,
  [SMALL_STATE(62)] = 4440,
  [SMALL_STATE(63)] = 4519,
  [SMALL_STATE(64)] = 4598,
  [SMALL_STATE(65)] = 4677,
  [SMALL_STATE(66)] = 4756,
  [SMALL_STATE(67)] = 4835,
  [SMALL_STATE(68)] = 4914,
  [SMALL_STATE(69)] = 4993,
  [SMALL_STATE(70)] = 5072,
  [SMALL_STATE(71)] = 5151,
  [SMALL_STATE(72)] = 5230,
  [SMALL_STATE(73)] = 5309,
  [SMALL_STATE(74)] = 5356,
  [SMALL_STATE(75)] = 5435,
  [SMALL_STATE(76)] = 5514,
  [SMALL_STATE(77)] = 5593,
  [SMALL_STATE(78)] = 5672,
  [SMALL_STATE(79)] = 5751,
  [SMALL_STATE(80)] = 5830,
  [SMALL_STATE(81)] = 5870,
  [SMALL_STATE(82)] = 5910,
  [SMALL_STATE(83)] = 5950,
  [SMALL_STATE(84)] = 5990,
  [SMALL_STATE(85)] = 6030,
  [SMALL_STATE(86)] = 6073,
  [SMALL_STATE(87)] = 6116,
  [SMALL_STATE(88)] = 6156,
  [SMALL_STATE(89)] = 6196,
  [SMALL_STATE(90)] = 6250,
  [SMALL_STATE(91)] = 6290,
  [SMALL_STATE(92)] = 6330,
  [SMALL_STATE(93)] = 6370,
  [SMALL_STATE(94)] = 6410,
  [SMALL_STATE(95)] = 6450,
  [SMALL_STATE(96)] = 6490,
  [SMALL_STATE(97)] = 6530,
  [SMALL_STATE(98)] = 6570,
  [SMALL_STATE(99)] = 6610,
  [SMALL_STATE(100)] = 6647,
  [SMALL_STATE(101)] = 6684,
  [SMALL_STATE(102)] = 6721,
  [SMALL_STATE(103)] = 6758,
  [SMALL_STATE(104)] = 6795,
  [SMALL_STATE(105)] = 6832,
  [SMALL_STATE(106)] = 6869,
  [SMALL_STATE(107)] = 6906,
  [SMALL_STATE(108)] = 6943,
  [SMALL_STATE(109)] = 6980,
  [SMALL_STATE(110)] = 7017,
  [SMALL_STATE(111)] = 7054,
  [SMALL_STATE(112)] = 7091,
  [SMALL_STATE(113)] = 7128,
  [SMALL_STATE(114)] = 7185,
  [SMALL_STATE(115)] = 7222,
  [SMALL_STATE(116)] = 7259,
  [SMALL_STATE(117)] = 7296,
  [SMALL_STATE(118)] = 7333,
  [SMALL_STATE(119)] = 7370,
  [SMALL_STATE(120)] = 7407,
  [SMALL_STATE(121)] = 7444,
  [SMALL_STATE(122)] = 7481,
  [SMALL_STATE(123)] = 7518,
  [SMALL_STATE(124)] = 7575,
  [SMALL_STATE(125)] = 7612,
  [SMALL_STATE(126)] = 7649,
  [SMALL_STATE(127)] = 7686,
  [SMALL_STATE(128)] = 7723,
  [SMALL_STATE(129)] = 7760,
  [SMALL_STATE(130)] = 7797,
  [SMALL_STATE(131)] = 7834,
  [SMALL_STATE(132)] = 7871,
  [SMALL_STATE(133)] = 7908,
  [SMALL_STATE(134)] = 7945,
  [SMALL_STATE(135)] = 7982,
  [SMALL_STATE(136)] = 8019,
  [SMALL_STATE(137)] = 8056,
  [SMALL_STATE(138)] = 8093,
  [SMALL_STATE(139)] = 8130,
  [SMALL_STATE(140)] = 8167,
  [SMALL_STATE(141)] = 8204,
  [SMALL_STATE(142)] = 8241,
  [SMALL_STATE(143)] = 8278,
  [SMALL_STATE(144)] = 8315,
  [SMALL_STATE(145)] = 8352,
  [SMALL_STATE(146)] = 8389,
  [SMALL_STATE(147)] = 8426,
  [SMALL_STATE(148)] = 8463,
  [SMALL_STATE(149)] = 8500,
  [SMALL_STATE(150)] = 8537,
  [SMALL_STATE(151)] = 8574,
  [SMALL_STATE(152)] = 8611,
  [SMALL_STATE(153)] = 8648,
  [SMALL_STATE(154)] = 8685,
  [SMALL_STATE(155)] = 8722,
  [SMALL_STATE(156)] = 8759,
  [SMALL_STATE(157)] = 8796,
  [SMALL_STATE(158)] = 8833,
  [SMALL_STATE(159)] = 8870,
  [SMALL_STATE(160)] = 8907,
  [SMALL_STATE(161)] = 8944,
  [SMALL_STATE(162)] = 9001,
  [SMALL_STATE(163)] = 9038,
  [SMALL_STATE(164)] = 9075,
  [SMALL_STATE(165)] = 9112,
  [SMALL_STATE(166)] = 9149,
  [SMALL_STATE(167)] = 9186,
  [SMALL_STATE(168)] = 9223,
  [SMALL_STATE(169)] = 9260,
  [SMALL_STATE(170)] = 9297,
  [SMALL_STATE(171)] = 9334,
  [SMALL_STATE(172)] = 9371,
  [SMALL_STATE(173)] = 9408,
  [SMALL_STATE(174)] = 9445,
  [SMALL_STATE(175)] = 9482,
  [SMALL_STATE(176)] = 9519,
  [SMALL_STATE(177)] = 9556,
  [SMALL_STATE(178)] = 9593,
  [SMALL_STATE(179)] = 9630,
  [SMALL_STATE(180)] = 9667,
  [SMALL_STATE(181)] = 9704,
  [SMALL_STATE(182)] = 9741,
  [SMALL_STATE(183)] = 9778,
  [SMALL_STATE(184)] = 9815,
  [SMALL_STATE(185)] = 9852,
  [SMALL_STATE(186)] = 9889,
  [SMALL_STATE(187)] = 9926,
  [SMALL_STATE(188)] = 9978,
  [SMALL_STATE(189)] = 10032,
  [SMALL_STATE(190)] = 10086,
  [SMALL_STATE(191)] = 10140,
  [SMALL_STATE(192)] = 10194,
  [SMALL_STATE(193)] = 10248,
  [SMALL_STATE(194)] = 10302,
  [SMALL_STATE(195)] = 10356,
  [SMALL_STATE(196)] = 10408,
  [SMALL_STATE(197)] = 10460,
  [SMALL_STATE(198)] = 10512,
  [SMALL_STATE(199)] = 10566,
  [SMALL_STATE(200)] = 10620,
  [SMALL_STATE(201)] = 10674,
  [SMALL_STATE(202)] = 10728,
  [SMALL_STATE(203)] = 10782,
  [SMALL_STATE(204)] = 10836,
  [SMALL_STATE(205)] = 10890,
  [SMALL_STATE(206)] = 10941,
  [SMALL_STATE(207)] = 10992,
  [SMALL_STATE(208)] = 11043,
  [SMALL_STATE(209)] = 11094,
  [SMALL_STATE(210)] = 11145,
  [SMALL_STATE(211)] = 11196,
  [SMALL_STATE(212)] = 11247,
  [SMALL_STATE(213)] = 11298,
  [SMALL_STATE(214)] = 11349,
  [SMALL_STATE(215)] = 11400,
  [SMALL_STATE(216)] = 11451,
  [SMALL_STATE(217)] = 11502,
  [SMALL_STATE(218)] = 11553,
  [SMALL_STATE(219)] = 11604,
  [SMALL_STATE(220)] = 11655,
  [SMALL_STATE(221)] = 11706,
  [SMALL_STATE(222)] = 11757,
  [SMALL_STATE(223)] = 11808,
  [SMALL_STATE(224)] = 11859,
  [SMALL_STATE(225)] = 11910,
  [SMALL_STATE(226)] = 11961,
  [SMALL_STATE(227)] = 12012,
  [SMALL_STATE(228)] = 12063,
  [SMALL_STATE(229)] = 12114,
  [SMALL_STATE(230)] = 12165,
  [SMALL_STATE(231)] = 12216,
  [SMALL_STATE(232)] = 12267,
  [SMALL_STATE(233)] = 12318,
  [SMALL_STATE(234)] = 12369,
  [SMALL_STATE(235)] = 12420,
  [SMALL_STATE(236)] = 12471,
  [SMALL_STATE(237)] = 12522,
  [SMALL_STATE(238)] = 12570,
  [SMALL_STATE(239)] = 12602,
  [SMALL_STATE(240)] = 12651,
  [SMALL_STATE(241)] = 12671,
  [SMALL_STATE(242)] = 12691,
  [SMALL_STATE(243)] = 12711,
  [SMALL_STATE(244)] = 12733,
  [SMALL_STATE(245)] = 12753,
  [SMALL_STATE(246)] = 12773,
  [SMALL_STATE(247)] = 12793,
  [SMALL_STATE(248)] = 12813,
  [SMALL_STATE(249)] = 12833,
  [SMALL_STATE(250)] = 12855,
  [SMALL_STATE(251)] = 12886,
  [SMALL_STATE(252)] = 12914,
  [SMALL_STATE(253)] = 12942,
  [SMALL_STATE(254)] = 12970,
  [SMALL_STATE(255)] = 12998,
  [SMALL_STATE(256)] = 13026,
  [SMALL_STATE(257)] = 13054,
  [SMALL_STATE(258)] = 13082,
  [SMALL_STATE(259)] = 13110,
  [SMALL_STATE(260)] = 13138,
  [SMALL_STATE(261)] = 13155,
  [SMALL_STATE(262)] = 13167,
  [SMALL_STATE(263)] = 13179,
  [SMALL_STATE(264)] = 13195,
  [SMALL_STATE(265)] = 13211,
  [SMALL_STATE(266)] = 13223,
  [SMALL_STATE(267)] = 13239,
  [SMALL_STATE(268)] = 13252,
  [SMALL_STATE(269)] = 13265,
  [SMALL_STATE(270)] = 13272,
  [SMALL_STATE(271)] = 13285,
  [SMALL_STATE(272)] = 13296,
  [SMALL_STATE(273)] = 13307,
  [SMALL_STATE(274)] = 13318,
  [SMALL_STATE(275)] = 13331,
  [SMALL_STATE(276)] = 13344,
  [SMALL_STATE(277)] = 13357,
  [SMALL_STATE(278)] = 13368,
  [SMALL_STATE(279)] = 13378,
  [SMALL_STATE(280)] = 13388,
  [SMALL_STATE(281)] = 13398,
  [SMALL_STATE(282)] = 13408,
  [SMALL_STATE(283)] = 13418,
  [SMALL_STATE(284)] = 13428,
  [SMALL_STATE(285)] = 13438,
  [SMALL_STATE(286)] = 13448,
  [SMALL_STATE(287)] = 13458,
  [SMALL_STATE(288)] = 13466,
  [SMALL_STATE(289)] = 13472,
  [SMALL_STATE(290)] = 13482,
  [SMALL_STATE(291)] = 13488,
  [SMALL_STATE(292)] = 13494,
  [SMALL_STATE(293)] = 13504,
  [SMALL_STATE(294)] = 13514,
  [SMALL_STATE(295)] = 13524,
  [SMALL_STATE(296)] = 13534,
  [SMALL_STATE(297)] = 13544,
  [SMALL_STATE(298)] = 13554,
  [SMALL_STATE(299)] = 13564,
  [SMALL_STATE(300)] = 13574,
  [SMALL_STATE(301)] = 13584,
  [SMALL_STATE(302)] = 13591,
  [SMALL_STATE(303)] = 13598,
  [SMALL_STATE(304)] = 13603,
  [SMALL_STATE(305)] = 13610,
  [SMALL_STATE(306)] = 13617,
  [SMALL_STATE(307)] = 13622,
  [SMALL_STATE(308)] = 13629,
  [SMALL_STATE(309)] = 13634,
  [SMALL_STATE(310)] = 13641,
  [SMALL_STATE(311)] = 13648,
  [SMALL_STATE(312)] = 13653,
  [SMALL_STATE(313)] = 13660,
  [SMALL_STATE(314)] = 13664,
  [SMALL_STATE(315)] = 13668,
  [SMALL_STATE(316)] = 13672,
  [SMALL_STATE(317)] = 13676,
  [SMALL_STATE(318)] = 13680,
  [SMALL_STATE(319)] = 13684,
  [SMALL_STATE(320)] = 13688,
  [SMALL_STATE(321)] = 13692,
  [SMALL_STATE(322)] = 13696,
  [SMALL_STATE(323)] = 13700,
  [SMALL_STATE(324)] = 13704,
  [SMALL_STATE(325)] = 13708,
  [SMALL_STATE(326)] = 13712,
  [SMALL_STATE(327)] = 13716,
  [SMALL_STATE(328)] = 13720,
  [SMALL_STATE(329)] = 13724,
  [SMALL_STATE(330)] = 13728,
  [SMALL_STATE(331)] = 13732,
  [SMALL_STATE(332)] = 13736,
  [SMALL_STATE(333)] = 13740,
  [SMALL_STATE(334)] = 13744,
  [SMALL_STATE(335)] = 13748,
  [SMALL_STATE(336)] = 13752,
  [SMALL_STATE(337)] = 13756,
  [SMALL_STATE(338)] = 13760,
  [SMALL_STATE(339)] = 13764,
  [SMALL_STATE(340)] = 13768,
  [SMALL_STATE(341)] = 13772,
  [SMALL_STATE(342)] = 13776,
  [SMALL_STATE(343)] = 13780,
  [SMALL_STATE(344)] = 13784,
  [SMALL_STATE(345)] = 13788,
  [SMALL_STATE(346)] = 13792,
  [SMALL_STATE(347)] = 13796,
  [SMALL_STATE(348)] = 13800,
  [SMALL_STATE(349)] = 13804,
  [SMALL_STATE(350)] = 13808,
  [SMALL_STATE(351)] = 13812,
  [SMALL_STATE(352)] = 13816,
  [SMALL_STATE(353)] = 13820,
  [SMALL_STATE(354)] = 13824,
  [SMALL_STATE(355)] = 13828,
  [SMALL_STATE(356)] = 13832,
  [SMALL_STATE(357)] = 13836,
  [SMALL_STATE(358)] = 13840,
  [SMALL_STATE(359)] = 13844,
  [SMALL_STATE(360)] = 13848,
  [SMALL_STATE(361)] = 13852,
  [SMALL_STATE(362)] = 13856,
  [SMALL_STATE(363)] = 13860,
  [SMALL_STATE(364)] = 13864,
  [SMALL_STATE(365)] = 13868,
  [SMALL_STATE(366)] = 13872,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(353),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(353),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(321),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(91),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(320),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(320),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(319),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(92),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(318),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(318),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(317),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(317),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(316),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(316),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(315),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(93),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(340),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(340),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(328),
  [41] = {.entry = {.count = 1, .reusable = false}}, SHIFT(94),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(329),
  [45] = {.entry = {.count = 1, .reusable = false}}, SHIFT(95),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(330),
  [49] = {.entry = {.count = 1, .reusable = false}}, SHIFT(330),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(331),
  [53] = {.entry = {.count = 1, .reusable = false}}, SHIFT(331),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(332),
  [57] = {.entry = {.count = 1, .reusable = false}}, SHIFT(332),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [61] = {.entry = {.count = 1, .reusable = false}}, SHIFT(96),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(333),
  [65] = {.entry = {.count = 1, .reusable = false}}, SHIFT(333),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(334),
  [69] = {.entry = {.count = 1, .reusable = false}}, SHIFT(334),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(335),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(98),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(336),
  [77] = {.entry = {.count = 1, .reusable = false}}, SHIFT(336),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(342),
  [81] = {.entry = {.count = 1, .reusable = false}}, SHIFT(87),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(344),
  [85] = {.entry = {.count = 1, .reusable = false}}, SHIFT(344),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(313),
  [89] = {.entry = {.count = 1, .reusable = false}}, SHIFT(97),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(346),
  [93] = {.entry = {.count = 1, .reusable = false}}, SHIFT(346),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(347),
  [97] = {.entry = {.count = 1, .reusable = false}}, SHIFT(347),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(350),
  [101] = {.entry = {.count = 1, .reusable = false}}, SHIFT(350),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [107] = {.entry = {.count = 1, .reusable = false}}, SHIFT(177),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [111] = {.entry = {.count = 1, .reusable = false}}, SHIFT(173),
  [113] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [115] = {.entry = {.count = 1, .reusable = false}}, SHIFT(166),
  [117] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [119] = {.entry = {.count = 1, .reusable = false}}, SHIFT(165),
  [121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [123] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [127] = {.entry = {.count = 1, .reusable = false}}, SHIFT(140),
  [129] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [133] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [135] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [137] = {.entry = {.count = 1, .reusable = false}}, SHIFT(142),
  [139] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [141] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [143] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [145] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Write, 2),
  [149] = {.entry = {.count = 1, .reusable = true}}, SHIFT(272),
  [151] = {.entry = {.count = 1, .reusable = false}}, SHIFT(312),
  [153] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [155] = {.entry = {.count = 1, .reusable = true}}, SHIFT(352),
  [157] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [159] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [161] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [163] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(276),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(248),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [175] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [177] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [181] = {.entry = {.count = 1, .reusable = false}}, SHIFT(292),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(250),
  [185] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [187] = {.entry = {.count = 1, .reusable = false}}, SHIFT(283),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [191] = {.entry = {.count = 1, .reusable = false}}, SHIFT(278),
  [193] = {.entry = {.count = 1, .reusable = false}}, SHIFT(308),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_Patern_repeat1, 2),
  [197] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_Patern_repeat1, 2), SHIFT_REPEAT(270),
  [200] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_Patern_repeat1, 2), SHIFT_REPEAT(276),
  [203] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_Patern_repeat1, 2),
  [205] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_Patern_repeat1, 2), SHIFT_REPEAT(81),
  [208] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Patern, 2),
  [210] = {.entry = {.count = 1, .reusable = true}}, SHIFT(270),
  [212] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Patern, 2),
  [214] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [216] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_PaternMatchExpression, 4, .production_id = 9),
  [218] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_PaternMatchExpression, 4, .production_id = 9),
  [220] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_BinaryExpression, 3, .production_id = 6),
  [222] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_BinaryExpression, 3, .production_id = 6),
  [224] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_UnaryExpression, 2, .production_id = 3),
  [226] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_UnaryExpression, 2, .production_id = 3),
  [228] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_InderectExpression, 2),
  [230] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_InderectExpression, 2),
  [232] = {.entry = {.count = 1, .reusable = true}}, SHIFT(360),
  [234] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [236] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2),
  [238] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2),
  [240] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_PaternElement, 1),
  [242] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_PaternElement, 1),
  [244] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_PaternElement, 4),
  [246] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_PaternElement, 4),
  [248] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_PaternElement, 3),
  [250] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_PaternElement, 3),
  [252] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3),
  [254] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3),
  [256] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Variable, 1, .production_id = 1),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [260] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Variable, 1, .production_id = 1),
  [262] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Variable, 2, .production_id = 5),
  [264] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Variable, 2, .production_id = 5),
  [266] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Io, 1),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(252),
  [270] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Io, 1),
  [272] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [276] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 1),
  [278] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_commandArg, 1),
  [280] = {.entry = {.count = 1, .reusable = true}}, SHIFT(241),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [284] = {.entry = {.count = 1, .reusable = false}}, SHIFT(245),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(245),
  [288] = {.entry = {.count = 1, .reusable = false}}, SHIFT(246),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(287),
  [292] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 2),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [296] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 2),
  [298] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Storage, 1),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [302] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Storage, 1),
  [304] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Test, 1),
  [306] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [308] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Test, 1),
  [310] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Reference, 1),
  [312] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [314] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Reference, 1),
  [316] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Principal, 1),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [320] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Principal, 1),
  [322] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Job, 1),
  [324] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [326] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Job, 1),
  [328] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_StackVar, 1),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [332] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_StackVar, 1),
  [334] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Device, 1),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(258),
  [338] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Device, 1),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Quit, 1),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(254),
  [344] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Quit, 1),
  [346] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Char, 5, .production_id = 20),
  [348] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Char, 5, .production_id = 20),
  [350] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Stack, 6, .production_id = 23),
  [352] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Stack, 6, .production_id = 23),
  [354] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_X, 1),
  [356] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_X, 1),
  [358] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Y, 1),
  [360] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Y, 1),
  [362] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_IntrinsicFunction, 2),
  [364] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_IntrinsicFunction, 2),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExpFunctions, 1),
  [368] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExpFunctions, 1),
  [370] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_VarFunctions, 1),
  [372] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_VarFunctions, 1),
  [374] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_IntrinsicVar, 2),
  [376] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_IntrinsicVar, 2),
  [378] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExtrinsicFunction, 6, .production_id = 18),
  [380] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExtrinsicFunction, 6, .production_id = 18),
  [382] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExtrinsicFunction, 6, .production_id = 16),
  [384] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExtrinsicFunction, 6, .production_id = 16),
  [386] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Piece, 10, .production_id = 28),
  [388] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Piece, 10, .production_id = 28),
  [390] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExtrinsicFunction, 6, .production_id = 15),
  [392] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExtrinsicFunction, 6, .production_id = 15),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [396] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [398] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_XCall, 5, .production_id = 14),
  [400] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_XCall, 5, .production_id = 14),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Char, 4, .production_id = 12),
  [404] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Char, 4, .production_id = 12),
  [406] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExtrinsicFunction, 7, .production_id = 21),
  [408] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExtrinsicFunction, 7, .production_id = 21),
  [410] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExtrinsicFunction, 7, .production_id = 22),
  [412] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExtrinsicFunction, 7, .production_id = 22),
  [414] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Variable, 2, .production_id = 4),
  [416] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Variable, 2, .production_id = 4),
  [418] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Qlength, 4, .production_id = 13),
  [420] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Qlength, 4, .production_id = 13),
  [422] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Data, 4, .production_id = 13),
  [424] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Data, 4, .production_id = 13),
  [426] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Select, 6),
  [428] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Select, 6),
  [430] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Expression, 3),
  [432] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Expression, 3),
  [434] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [438] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 3),
  [440] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 3),
  [442] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Next, 4, .production_id = 13),
  [444] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Next, 4, .production_id = 13),
  [446] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Increment, 4, .production_id = 13),
  [448] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Increment, 4, .production_id = 13),
  [450] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_View, 6, .production_id = 23),
  [452] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_View, 6, .production_id = 23),
  [454] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Get, 4, .production_id = 13),
  [456] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Get, 4, .production_id = 13),
  [458] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Translate, 6, .production_id = 23),
  [460] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Translate, 6, .production_id = 23),
  [462] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Find, 6, .production_id = 23),
  [464] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Find, 6, .production_id = 23),
  [466] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Fnumber, 6, .production_id = 23),
  [468] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Fnumber, 6, .production_id = 23),
  [470] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Piece, 6, .production_id = 23),
  [472] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Piece, 6, .production_id = 23),
  [474] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Justify, 6, .production_id = 23),
  [476] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Justify, 6, .production_id = 23),
  [478] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Extract, 6, .production_id = 23),
  [480] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Extract, 6, .production_id = 23),
  [482] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Ascii, 6, .production_id = 23),
  [484] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Ascii, 6, .production_id = 23),
  [486] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Key, 1),
  [488] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Key, 1),
  [490] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Length, 6, .production_id = 23),
  [492] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Length, 6, .production_id = 23),
  [494] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_System, 1),
  [496] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_System, 1),
  [498] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Query, 4, .production_id = 13),
  [500] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Query, 4, .production_id = 13),
  [502] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Name, 6, .production_id = 24),
  [504] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Name, 6, .production_id = 24),
  [506] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Order, 4, .production_id = 13),
  [508] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Order, 4, .production_id = 13),
  [510] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Order, 6, .production_id = 24),
  [512] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Order, 6, .production_id = 24),
  [514] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Name, 4, .production_id = 13),
  [516] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Name, 4, .production_id = 13),
  [518] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Stack, 4, .production_id = 12),
  [520] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Stack, 4, .production_id = 12),
  [522] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Variable, 2, .production_id = 2),
  [524] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Variable, 2, .production_id = 2),
  [526] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Length, 4, .production_id = 12),
  [528] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Length, 4, .production_id = 12),
  [530] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Query, 6, .production_id = 24),
  [532] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Query, 6, .production_id = 24),
  [534] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Ascii, 4, .production_id = 12),
  [536] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Ascii, 4, .production_id = 12),
  [538] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Get, 6, .production_id = 24),
  [540] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Get, 6, .production_id = 24),
  [542] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Extract, 4, .production_id = 12),
  [544] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Extract, 4, .production_id = 12),
  [546] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Increment, 6, .production_id = 24),
  [548] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Increment, 6, .production_id = 24),
  [550] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Qsubscript, 6, .production_id = 24),
  [552] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Qsubscript, 6, .production_id = 24),
  [554] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_XCall, 7, .production_id = 25),
  [556] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_XCall, 7, .production_id = 25),
  [558] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [560] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [562] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Text, 4, .production_id = 12),
  [564] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Text, 4, .production_id = 12),
  [566] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExtrinsicFunction, 8, .production_id = 26),
  [568] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExtrinsicFunction, 8, .production_id = 26),
  [570] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Select, 7),
  [572] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Select, 7),
  [574] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Horolog, 1),
  [576] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Horolog, 1),
  [578] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Etrap, 1),
  [580] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Etrap, 1),
  [582] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExtrinsicFunction, 5, .production_id = 10),
  [584] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExtrinsicFunction, 5, .production_id = 10),
  [586] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Expression, 1),
  [588] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Expression, 1),
  [590] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_View, 8, .production_id = 27),
  [592] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_View, 8, .production_id = 27),
  [594] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Translate, 8, .production_id = 27),
  [596] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Translate, 8, .production_id = 27),
  [598] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Find, 8, .production_id = 27),
  [600] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Find, 8, .production_id = 27),
  [602] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Fnumber, 8, .production_id = 27),
  [604] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Fnumber, 8, .production_id = 27),
  [606] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Estack, 1),
  [608] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Estack, 1),
  [610] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_PaternMatchExpression, 3, .production_id = 6),
  [612] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_PaternMatchExpression, 3, .production_id = 6),
  [614] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Piece, 8, .production_id = 27),
  [616] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Piece, 8, .production_id = 27),
  [618] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Variable, 3, .production_id = 7),
  [620] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Variable, 3, .production_id = 7),
  [622] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Ecode, 1),
  [624] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Ecode, 1),
  [626] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Justify, 8, .production_id = 27),
  [628] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Justify, 8, .production_id = 27),
  [630] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExtrinsicFunction, 4, .production_id = 8),
  [632] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExtrinsicFunction, 4, .production_id = 8),
  [634] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ExtrinsicFunction, 5, .production_id = 11),
  [636] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ExtrinsicFunction, 5, .production_id = 11),
  [638] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Random, 4, .production_id = 12),
  [640] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Random, 4, .production_id = 12),
  [642] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Extract, 8, .production_id = 27),
  [644] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Extract, 8, .production_id = 27),
  [646] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_View, 10, .production_id = 28),
  [648] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_View, 10, .production_id = 28),
  [650] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__VariableSubscripts, 3),
  [652] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__VariableSubscripts, 3),
  [654] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Reverse, 4, .production_id = 12),
  [656] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_Reverse, 4, .production_id = 12),
  [658] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__VariableSubscripts, 4),
  [660] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__VariableSubscripts, 4),
  [662] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__FunctionArg, 1),
  [664] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [666] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [668] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [670] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [672] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [674] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [676] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [678] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [680] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [682] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [684] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [686] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [688] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [690] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [692] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_Char_repeat1, 2, .production_id = 17),
  [694] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__VariableSubscripts_repeat1, 2),
  [696] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_Select_repeat1, 4),
  [698] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [700] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [702] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [704] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [706] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [708] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [710] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [712] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [714] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [716] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [718] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [720] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [722] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [724] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [726] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [728] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [730] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [732] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [734] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [736] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [738] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [740] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [742] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [744] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [746] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [748] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [750] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [752] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [754] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [756] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [758] = {.entry = {.count = 1, .reusable = true}}, SHIFT(185),
  [760] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [762] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [764] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [766] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [768] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [770] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [772] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [774] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [776] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [778] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [780] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [782] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [784] = {.entry = {.count = 1, .reusable = false}}, SHIFT(249),
  [786] = {.entry = {.count = 1, .reusable = true}}, SHIFT(356),
  [788] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [790] = {.entry = {.count = 1, .reusable = false}}, SHIFT(243),
  [792] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [794] = {.entry = {.count = 1, .reusable = true}}, SHIFT(326),
  [796] = {.entry = {.count = 1, .reusable = false}}, SHIFT(326),
  [798] = {.entry = {.count = 1, .reusable = true}}, SHIFT(349),
  [800] = {.entry = {.count = 1, .reusable = true}}, SHIFT(348),
  [802] = {.entry = {.count = 1, .reusable = true}}, SHIFT(322),
  [804] = {.entry = {.count = 1, .reusable = true}}, SHIFT(323),
  [806] = {.entry = {.count = 1, .reusable = true}}, SHIFT(324),
  [808] = {.entry = {.count = 1, .reusable = true}}, SHIFT(325),
  [810] = {.entry = {.count = 1, .reusable = true}}, SHIFT(337),
  [812] = {.entry = {.count = 1, .reusable = true}}, SHIFT(345),
  [814] = {.entry = {.count = 1, .reusable = true}}, SHIFT(366),
  [816] = {.entry = {.count = 1, .reusable = true}}, SHIFT(362),
  [818] = {.entry = {.count = 1, .reusable = true}}, SHIFT(361),
  [820] = {.entry = {.count = 1, .reusable = true}}, SHIFT(359),
  [822] = {.entry = {.count = 1, .reusable = true}}, SHIFT(358),
  [824] = {.entry = {.count = 1, .reusable = true}}, SHIFT(357),
  [826] = {.entry = {.count = 1, .reusable = true}}, SHIFT(355),
  [828] = {.entry = {.count = 1, .reusable = true}}, SHIFT(354),
  [830] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_OPSUB, 1),
  [832] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_OPSUB, 1),
  [834] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_OPADD, 1),
  [836] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_OPADD, 1),
  [838] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_OPPLUS, 1),
  [840] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_OPPLUS, 1),
  [842] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_OPFOL, 1),
  [844] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_OPFOL, 1),
  [846] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_GlobalUciEnvVariable, 5),
  [848] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_OPMINUS, 1),
  [850] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_OPMINUS, 1),
  [852] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_BinaryOpp, 1),
  [854] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_BinaryOpp, 1),
  [856] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_UnaryOpp, 1),
  [858] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_UnaryOpp, 1),
  [860] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_OPNOT, 1),
  [862] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_OPNOT, 1),
  [864] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_GlobalUciVariable, 3),
  [866] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [868] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [870] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Write, 3),
  [872] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [874] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_Write_repeat1, 2),
  [876] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_Write_repeat1, 2), SHIFT_REPEAT(10),
  [879] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [881] = {.entry = {.count = 1, .reusable = false}}, SHIFT(291),
  [883] = {.entry = {.count = 1, .reusable = true}}, SHIFT(291),
  [885] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Write, 4),
  [887] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [889] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_repeat1, 2),
  [891] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_repeat1, 2), SHIFT_REPEAT(267),
  [894] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_repeat1, 2), SHIFT_REPEAT(267),
  [897] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_line_repeat1, 2),
  [899] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_line_repeat1, 2), SHIFT_REPEAT(274),
  [902] = {.entry = {.count = 1, .reusable = true}}, SHIFT(310),
  [904] = {.entry = {.count = 1, .reusable = true}}, SHIFT(327),
  [906] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line, 2),
  [908] = {.entry = {.count = 1, .reusable = true}}, SHIFT(274),
  [910] = {.entry = {.count = 1, .reusable = false}}, SHIFT(84),
  [912] = {.entry = {.count = 1, .reusable = false}}, SHIFT(267),
  [914] = {.entry = {.count = 1, .reusable = true}}, SHIFT(267),
  [916] = {.entry = {.count = 1, .reusable = false}}, SHIFT(80),
  [918] = {.entry = {.count = 1, .reusable = false}}, SHIFT(275),
  [920] = {.entry = {.count = 1, .reusable = true}}, SHIFT(275),
  [922] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line, 1),
  [924] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [926] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [928] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2),
  [930] = {.entry = {.count = 1, .reusable = true}}, SHIFT(263),
  [932] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ExtrinsicFunction_repeat1, 2, .production_id = 19), SHIFT_REPEAT(7),
  [935] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ExtrinsicFunction_repeat1, 2, .production_id = 19),
  [937] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [939] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__VariableSubscripts_repeat1, 2), SHIFT_REPEAT(18),
  [942] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [944] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [946] = {.entry = {.count = 1, .reusable = true}}, SHIFT(268),
  [948] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [950] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [952] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_PatternOpp, 1),
  [954] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_PatternOpp, 1),
  [956] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_command, 1),
  [958] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [960] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_PaternRepetitionCount, 1),
  [962] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [964] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [966] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_Char_repeat1, 2, .production_id = 19), SHIFT_REPEAT(77),
  [969] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_Char_repeat1, 2, .production_id = 19),
  [971] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_PaternElement_repeat1, 2), SHIFT_REPEAT(268),
  [974] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_PaternElement_repeat1, 2),
  [976] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [978] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(263),
  [981] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [983] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [985] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [987] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_Select_repeat1, 2), SHIFT_REPEAT(74),
  [990] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_Select_repeat1, 2),
  [992] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [994] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [996] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [998] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [1000] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ByRef, 2),
  [1002] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [1004] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [1006] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [1008] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [1010] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ExtrinsicFunction_repeat1, 2, .production_id = 17),
  [1012] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [1014] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [1016] = {.entry = {.count = 1, .reusable = true}}, SHIFT(363),
  [1018] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [1020] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_NakedVariable, 1),
  [1022] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_GlobalVariable, 1),
  [1024] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [1026] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [1028] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [1030] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [1032] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [1034] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_File, 2),
  [1036] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ErrMsg, 2),
  [1038] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_OpCom, 2),
  [1040] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Signal, 2),
  [1042] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [1044] = {.entry = {.count = 1, .reusable = true}}, SHIFT(364),
  [1046] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [1048] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [1050] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [1052] = {.entry = {.count = 1, .reusable = true}}, SHIFT(256),
  [1054] = {.entry = {.count = 1, .reusable = true}}, SHIFT(255),
  [1056] = {.entry = {.count = 1, .reusable = true}}, SHIFT(253),
  [1058] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Spawn, 2),
  [1060] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [1062] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [1064] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [1066] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [1068] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [1070] = {.entry = {.count = 1, .reusable = true}}, SHIFT(259),
  [1072] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Version, 2),
  [1074] = {.entry = {.count = 1, .reusable = true}}, SHIFT(257),
  [1076] = {.entry = {.count = 1, .reusable = true}}, SHIFT(251),
  [1078] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Host, 2),
  [1080] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Directory, 2),
  [1082] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [1084] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [1086] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [1088] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Compress, 2),
  [1090] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Wait, 2),
  [1092] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_IC, 2),
  [1094] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Fork, 2),
  [1096] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_RouChk, 2),
  [1098] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_IndirectVariable, 3),
  [1100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_GetEnv, 2),
  [1102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_SetEnv, 2),
  [1104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(351),
  [1106] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [1108] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [1110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_Zwrite, 2),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_mumps(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
