#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <tree_sitter/parser.h>

enum TokenType {
  VarUndefined,
  INDENT,
  DEDENT,
  LINE_LEVEL,
  ERROR_SENTINEL
};


struct ParseStateStruct {
  int indentation;
  /*
   * For now I want the entire document to be a block.
   * this will be redone when I introduce proper tag handlaing.
   */
  bool docInitialized;
} typedef ParseState;

void *tree_sitter_mumps_external_scanner_create() {
  ParseState *state = malloc(sizeof(ParseState));
  state->indentation = 0;
  state->docInitialized= false;
  return state;
}

void tree_sitter_mumps_external_scanner_destroy(void *payload) {
  free(payload);
}

unsigned tree_sitter_mumps_external_scanner_serialize(void *payload,
                                                      char *buffer) {
  memcpy(buffer, payload, sizeof(ParseState));
  return sizeof(ParseState);
}

void tree_sitter_mumps_external_scanner_deserialize(void *payload,
                                                    const char *buffer,
                                                    unsigned length) {
  if (length) {
    memcpy(payload, buffer, sizeof(ParseState));
  }
}

bool tree_sitter_mumps_external_scanner_scan(void *payload, TSLexer *lexer,
                                             const bool *valid_symbols) {
  // are we in error recoveray mode
  if (valid_symbols[ERROR_SENTINEL]) {
    return false;
  }

  // NOTE only valid as part of an args list.
  // Therefore only valide if trailed by a ) or ,
  if (valid_symbols[VarUndefined] &&
      (lexer->lookahead == ',' || lexer->lookahead == ')')) {
    lexer->result_symbol = VarUndefined;
    return true;
  } else if (valid_symbols[INDENT] || valid_symbols[DEDENT] ||
             valid_symbols[LINE_LEVEL]) {

    ParseState *state = payload;
    lexer->mark_end(lexer);

    // calculating indentation
    //------------------------------------------------
    int count = 0;

    // indentation must start with space or TODO tab.
    if (lexer->lookahead == ' ') {
      count++;
      while (lexer->lookahead == '.' || lexer->lookahead == ' ') {
        //after the initial indentation only . count
        if (lexer->lookahead == '.') {
          count++;
        }
        lexer->advance(lexer, false);
      }
    }

    if (state->indentation < count && valid_symbols[INDENT]) {
      lexer->result_symbol = INDENT;
      state->indentation++;
    } else if (state->indentation == count && valid_symbols[LINE_LEVEL]) {
      lexer->result_symbol = LINE_LEVEL;
      lexer->mark_end(lexer);
    } else if (valid_symbols[DEDENT]) {
      lexer->result_symbol = DEDENT;
      state->indentation--;
    } else {
      return false;
    }
    return true;
  }
  return false;
}
