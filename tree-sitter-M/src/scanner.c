/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <tree_sitter/parser.h>

enum TokenType {
  VarUndefined,
  ERROR_SENTINEL
};


struct ParseStateStruct {
  } typedef ParseState;

void *tree_sitter_mumps_external_scanner_create() {
  ParseState *state = malloc(sizeof(ParseState));
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
    // are we in error recovery mode
    if (valid_symbols[ERROR_SENTINEL]) {
        return false;
    }

    // NOTE only valid as part of an args list.
    // Therefore only validate if trailed by a ) or ,
    if (valid_symbols[VarUndefined] &&
        (lexer->lookahead == ',' || lexer->lookahead == ')')) {
        lexer->result_symbol = VarUndefined;
        return true;
    }  else{
        return false;
    }
}
