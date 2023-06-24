/*
 * Package:  Reference Standard M
 * File:     rsm/compile/eval.c
 * Summary:  module compile - evaluate
 *
 * David Wicksell <dlw@linux.com>
 * Copyright © 2020-2023 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Based on MUMPS V1 by Raymond Douglas Newman
 * Copyright (c) 1999-2018
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
 * along with this program. If not, see http://www.gnu.org/licenses/.
 */

#include <stdio.h>                                                              // always include
#include <stdlib.h>                                                             // these two
#include <sys/types.h>                                                          // for u_char def
#include <string.h>
#include <ctype.h>
#include <errno.h>                                                              // error stuff
#include <limits.h>                                                             // for LONG_MAX etc.
#include <math.h>
#include <assert.h>
#include "rsm.h"                                                                // standard includes
#include "proto.h"                                                              // standard prototypes
#include "error.h"                                                              // and the error defs
#include "opcode.h"                                                             // and the opcodes
#include "compile_temp.h"                                                            // compiler stuff
#include "rust.h"


void comperror_temp(u_char **src,u_char **comp,partab_struct *partition_tab,short err)                                                       // compile error
{
    int     s;                                                                  // for functions
    u_short us;                                                                 // for functions
    cstring *line;                                                              // line of code
    u_char  *src_old;                                                               // current src ptr
    int     i;                                                                  // a handy int
    u_char  tmp[128];                                                           // some space

    *(*comp)++ = OPERROR;                                                      // say it's an error
    assert(sizeof(err) == sizeof(short));
    memcpy((*comp), &err, sizeof(short));
    (*comp) += sizeof(short);
    *(*comp)++ = OPNOP;                                                        // in case of IF etc.
    *(*comp)++ = OPNOP;                                                        // in case of IF etc.
    if (!(*partition_tab).checkonly) goto scan;                                           // done
    if ((*partition_tab).checkonly == *(*partition_tab).ln) return;                                 // done this one once
    (*partition_tab).checkonly = *(*partition_tab).ln;                                              // record done
    line = *(*partition_tab).lp;                                                          // get the line address
    src_old = *(*partition_tab).sp;                                                           // and the current source
    s = SQ_Write(line);                                                         // write the line
    if (s < 0) goto scan;                                                       // exit on error
    s = SQ_WriteFormat(SQ_LF);                                                  // return
    if (s < 0) goto scan;                                                       // exit on error
    i = src_old - line->buf - 1;                                                    // get the offset

    if (i > 0) {
        s = SQ_WriteFormat(i);                                                  // tab
        if (s < 0) goto scan;                                                   // exit on error
    }

    line = (cstring *) tmp;                                                     // some space
DISABLE_WARN(-Warray-bounds)
    line->buf[0] = '^';                                                         // point
    line->buf[1] = ' ';                                                         // and a space
    us = UTIL_strerror(err, &line->buf[2]);                                     // get the error
    line->len = us + 2;                                                         // the length
    memcpy(&line->buf[line->len], " - At line ", 11);                           // front bit
    us = itocstring(&line->buf[line->len + 11], *(*partition_tab).ln);                    // format line number
    line->len += us + 11;                                                       // the length
ENABLE_WARN
    s = SQ_Write(line);                                                         // write the line
    if (s >= 0) SQ_WriteFormat(SQ_LF);                                          // if no error return
    if ((*partition_tab).checkonly) (*partition_tab).errors++;                                      // syntax check so increment error count

scan:
    while (*(*src)) (*src)++;                                           // skip rest of line
    return;                                                                     // and done
}

/*
 * Function atom entered with (*src) pointing at the source
 * atom to evaluate and (*comp) pointing at where to put the code.
 */
void  atom_temp(u_char **src,u_char **comp,partab_struct *partition_tab)                                                               // evaluate source
{
    char   c;                                                                   // current character

    c = *(*src)++;                                                          // get a character
    if (c == '@') {                                                             // indirection?
      atom_temp(src,comp,partition_tab);                                                                 // eval what follows

        if (*(*src) != '@') {                                               // another one?
            *(*comp)++ = INDEVAL;                                              // no, eval what follows
            return;                                                             // and just exit
        }

        *(*comp)++ = INDMVAR;                                                  // make an mvar out of it
        parse_local_var_ffi(src,comp,partition_tab);
        return;                                                                 // and exit
    }

    if ((isalpha((int) c) != 0) || (c == '%') || (c == '^')) {                  // check for local variable or a global var
        --(*src);                                                           // backup to first character
        parse_local_var_ffi(src,comp,partition_tab);

        return;                                                                 // and exit
    }                                                                           // end variable parse

    if (c == '$') {                                                             // check for a function
      dodollar();                                                             // eval it
        return;                                                                 // and exit
    }                                                                           // end function parse

    if ((isdigit((int) c) != 0) || (c == '.')) {                                // check for number or dot
        (*src)--;                                                           // back up the source ptr
        ncopy_ffi(src,comp,partition_tab);


        return;
    }                                                                           // end numeric parse

    if (c == '"') {                                                             // rabbit ear
        int    j = sizeof(u_short);                                             // point at p->buf[0]
        u_char *p;                                                              // a pointer

        *(*comp)++ = OPSTR;                                                    // say string following
        p = (*comp);                                                           // possible destination

        while (TRUE) {                                                          // scan the string
            if (*(*src) == '\0') {                                          // check for end of string
                (*comp)--;                                                     // remove the OPSTR
                comperror_temp(src,comp,partition_tab,-(ERRZ12 + ERRMLAST));                                // compile an error
                return;                                                         // and exit
            }                                                                   // end of error bit

            if ((*(*src) == '"') && ((*src)[1] != '"')) {               // check end of literal
                p[j] = '\0';                                                    // null terminate it
                (*src)++;                                                   // point past it
                break;                                                          // and exit
            }                                                                   // end 'end of str' code

            p[j++] = *(*src)++;                                             // copy the character
            if ((*((*src) - 1) == '"') && (*(*src) == '"')) (*src)++; // got rabbit ears? then point past the second one
        }                                                                       // end of copy loop

        *((u_short *) p) = (u_short) (j - sizeof(u_short));                     // store cstring count
        (*comp) += j + 1;                                                      // point past str and null
        return;
    }                                                                           // end string literal

    if (c == '\'') {                                                            // check for single quote
      atom_temp(src,comp,partition_tab);                                                                 // get the following
        *(*comp)++ = OPNOT;                                                    // do the NOT
        return;
    }                                                                           // end NOT parsing

    if (c == '+') {                                                             // check for plus
      atom_temp(src,comp,partition_tab);                                                                 // get the following
        *(*comp)++ = OPPLUS;                                                   // do the plus
        return;
    }                                                                           // end NOT parsing

    if (c == '-') {                                                             // check for unary minus
      atom_temp(src,comp,partition_tab);                                                                 // get the following
        *(*comp)++ = OPMINUS;                                                  // do the minus
        return;
    }                                                                           // end NOT parsing

    if (c == '(') {                                                             // open bracket
      eval_temp(src,comp,partition_tab);                                                                 // eval content of ()

        if (*(*src)++ != ')') {                                             // error if no trailing ) found
          comperror_temp(src,comp,partition_tab,-(ERRZ12 + ERRMLAST));                                    // compile an error
            return;                                                             // and exit
        }                                                                       // end error

        return;
    }                                                                           // end open bracket parse

    comperror_temp(src,comp,partition_tab,-(ERRZ12 + ERRMLAST));                                            // compile an error
    return;                                                                     // and exit
}

/*
 * Function eval entered with (*src) pointing at the source
 * expression to evaluate and (*comp) pointing at where to put the code.
 */
void eval_temp(u_char **src,u_char **comp,partab_struct *partition_tab)                                                                // evaluate source
{

  atom_temp(src,comp,partition_tab);                                                                     // get first operand

    // do it at a higher level
    if ((*(*src) == ')') || (*(*src) == ',') || (*(*src) == ':') ||
      // end of string or start of routine ref or end of name indirection or end of command
      (*(*src) == '\0') || (*(*src) == '^') || (*(*src) == '@') || (*(*src) == ' ')) {
        return;                                                                 // exit
    }

    while (TRUE) {                                                              // until the end
      int op = opp(src);                                                    // get the operator
        int pattern = 0;                                                        // for pattern match funny

        if (op == 0) {                                                          // an error??
          comperror_temp(src,comp,partition_tab,-(ERRZ12 + ERRMLAST));                                      // compile the error
          return;                                                               // and exit
        }

        pattern = ((op == OPPAT) || (op == OPNPAT));                            // bloody pattern match

        if (pattern && (*(*src) == '@')) {                                  // indirect pattern
            (*src)++;                                                       // skip the @
            pattern = 0;                                                        // clear funny pattern match
        }

        if (pattern) {                                                          // normal (not @) pattern match
          parse_pattern_ffi(src,comp);
        } else {
          atom_temp(src,comp,partition_tab);                                                             // else get next operand
        }

        *(*comp)++ = (u_char) op;                                              // store the operator

        // do it at a higher level
        if ((*(*src) == ')') || (*(*src) == ',') || (*(*src) == ':') ||
          // end of string or start of routine ref or end of command
          (*(*src) == '\0') || (*(*src) == '^') || (*(*src) == ' ')) {
            return;                                                             // exit
        }
    }
}
