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

    //Only allow indentation at start of a line or at the end of the file.
    /*
    if (lexer->get_column(lexer) != 0 || lexer->eof(lexer) ||
        lexer->lookahead ==0) {
      return false;
    }*/

    ParseState *state = payload;
    lexer->mark_end(lexer);

    /*
    if (!state->docInitialized){
      state->docInitialized = true;
      lexer->result_symbol = INDENT;
      return true;
    }

    if (state->indentation==0 && (lexer->eof(lexer) || lexer->lookahead == 0)) {
      lexer->result_symbol = DEDENT;
      return true;
    }*/

    int count = 0;
    while (lexer->lookahead == '.') {
      count++;
      lexer->advance(lexer, false);
    }
    //Always try to indent before leveling off
    //Only match Dedent if it is valid;
    if (state->indentation < count) {
      lexer->result_symbol = INDENT;
      state->indentation++;
      return true;
    } else if (state->indentation == count) {
      //since line level is whitespace it can show up anywhere.
      //We will wind up in an infinely loop if it can match the empty string.
      if (count>0) {
        lexer->mark_end(lexer);
        lexer->result_symbol = LINE_LEVEL;
        return true;
      }else{
        return false;
      }
    } else if (valid_symbols[DEDENT] && state->indentation >0){
        lexer->result_symbol = DEDENT;
        state->indentation--;
        return true;
    }else{
      return false;
    }
  }
  return false;
}
