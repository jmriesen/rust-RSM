/*
 * Package:  Reference Standard M
 * File:     rsm/compile/dollar.c
 * Summary:  module compile - evaluate functions, vars etc.
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
#include "compile.h"                                                            // compiler stuff
#include "compile_temp.h"                                                            // compiler stuff
#include "rust.h"

void dodollar_temp(u_char **src,u_char **comp,partab_struct *partab_ptr)                                                             // parse var, func etc.
{
    int     len;                                                                // length of name
    u_short us;                                                                 // a handy unsigned short
    int     i = 0;                                                              // a handy int
    int     sel;                                                                // and another
    int     args = 0;                                                           // function args
    u_char  *ptr;                                                               // a handy pointer
    u_char  *selj[256];                                                         // a heap of them for $SELECT()
    char    name[20];                                                           // where to put the name
    char    c;                                                                  // current character
    u_char  save[1024];                                                         // a useful save area
    int     savecount;                                                          // number of bytes saved
    short   errm4 = -ERRM4;                                                     // useful error number

    c = toupper(*(*src)++);                                                 // get the character in upper

    if (c == '$') {                                                             // extrinsic
        ptr = (*comp);                                                         // save compile pointer
        *(*comp)++ = CMDOTAG;                                                  // assume a do tag
        i = routine_temp(-1,src,comp,partab_ptr);                                                        // parse the rouref

        if ((i > -1) || (i == -4)) {                                            // indirect etc. not on here
            (*comp) = ptr;                                                     // back where we started for error
            SYNTX;
        }

        if (i < -4) {                                                           // check for error
          comperror_temp(src,comp,partab_ptr,i);                                                       // complain
            return;                                                             // and exit
        }

        args = 129;                                                             // number of args (128=$$)

        if (i == -2) {
            *ptr = CMDORT;                                                      // routine and tag
        } else if (i == -3) {
            *ptr = CMDOROU;                                                     // just a routine
        }

        if (*(*src) == '(') {                                               // any args?
            args--;                                                             // back to 128
            savecount = (*comp) - ptr;                                         // bytes that got compiled
            memcpy(save, ptr, savecount);                                       // save that lot
            (*comp) = ptr;                                                     // back where we started
            (*src)++;                                                       // skip the (

            while (TRUE) {                                                      // while we have args
                if (args > (MAX_NUM_ARGS | 128)) SYNTX;                         // too many (128=$$)
                args++;                                                         // count an argument

                if (*(*src) == ')') {                                       // trailing bracket ?
                    (*src)++;                                               // skip the )
                    break;                                                      // and exit
                }

                if ((*(*src) == ',') || (*source_ptr == ')')) {             // if empty argument
                    *(*comp)++ = VARUNDF;                                      // flag it
                } else if ((*(*src) == '.') && (isdigit(source_ptr[1]) == 0)) { // by reference and not .numeric?
                    (*src)++;                                               // skip the dot

                    if (*(*src) == '@') {                                   // if indirection
                        (*src)++;                                           // skip the @
                        atom_ffi(src,comp,partab_ptr);                                                 // eval the string
                        *(*comp)++ = INDMVAR;
                    } else {
                      parse_local_var_ffi(src,comp,partab_ptr);                                         // get a variable
                    }

                    *(*comp)++ = NEWBREF;                                      // flag 'by reference'
                } else {                                                        // by value
                  eval_ffi(src,comp,partab_ptr);                                                     // leave the value on the stack
                }

                if (*(*src) == ')') continue;                               // trailing bracket? - do it above

                if (*(*src) == ',') {                                       // a comma ?
                    (*src)++;                                               // skip the ,
                    continue;                                                   // go for more
                }

                SYNTX;                                                          // all else is an error
            }                                                                   // end of while

            memcpy((*comp), save, savecount);                                  // copy the code back
            (*comp) += savecount;                                              // and add to the pointer
        }                                                                       // end of argument decode

        *(*comp)++ = (u_char) args;                                            // store number of args
        return;                                                                 // and exit
    }

    //TODO next
    if (c == '&') {                                                             // xcall
        c = toupper(*(*src)++);                                             // get next

        if (c == '%') {                                                         // if it's a percent
            name[i++] = c;                                                      // save it
            c = toupper(*(*src)++);                                         // get next
        }

        while (isalpha((int) c) != 0) {                                         // while we have alphas
            name[i++] = c;                                                      // save it
            c = toupper(*(*src)++);                                         // get next
        }

        name[i] = '\0';                                                         // null terminate

        if (c == '(') {                                                         // if it has args
            while (TRUE) {                                                      // loop
              eval_ffi(src,comp,partab_ptr);                                                         // get next argument
                args++;                                                         // count an argument
                c = *(*src)++;                                              // get term char
                if (c == ')') break;                                            // all done if closing )
                if (c != ',') EXPRE;                                            // if it's not a comma
            }                                                                   // end of args loop
        } else {
            (*src)--;                                                       // else backup the source ptr
        }

        if (args > 2) {                                                         // all XCalls take two args
          comperror_temp(src,comp,partab_ptr,-(ERRZ18 + ERRMLAST));                                    // junk
            return;
        }

        for (i = args; i < 2; i++) {                                            // force two arguments
            *(*comp)++ = OPSTR;                                                // say string follows
            *(*comp)++ = 0;                                                    // endian doesn't matter here
            *(*comp)++ = 0;                                                    // endian doesn't matter here
            *(*comp)++ = '\0';                                                 // null terminated
        }

        if (strcmp(name, "%DIRECTORY") == 0) {                                  // $&%DIRECTORY()
            *(*comp)++ = XCDIR;                                                // save the opcode
        } else if (strcmp(name, "%HOST") == 0) {                                // $&%HOST()
            *(*comp)++ = XCHOST;                                               // save the opcode
        } else if (strcmp(name, "%FILE") == 0) {                                // $&%FILE()
            *(*comp)++ = XCFILE;                                               // save the opcode
        } else if (strcmp(name, "%ERRMSG") == 0) {                              // $&%ERRMSG()
            *(*comp)++ = XCERR;                                                // save the opcode
        } else if (strcmp(name, "%OPCOM") == 0) {                               // $&%OPCOM()
            *(*comp)++ = XCOPC;                                                // save the opcode
        } else if (strcmp(name, "%SIGNAL") == 0) {                              // $&%SIGNAL()
            *(*comp)++ = XCSIG;                                                // save the opcode
        } else if (strcmp(name, "%SPAWN") == 0) {                               // $&%SPAWN()
            *(*comp)++ = XCSPA;                                                // save the opcode
        } else if (strcmp(name, "%VERSION") == 0) {                             // $&%VERSION()
            *(*comp)++ = XCVER;                                                // save the opcode
        } else if (strcmp(name, "%ZWRITE") == 0) {                              // $&%ZWRITE()
            *(*comp)++ = XCZWR;                                                // save the opcode
        } else if (strcmp(name, "E") == 0) {                                    // $&E()
            *(*comp)++ = XCE;                                                  // save the opcode
        } else if (strcmp(name, "PASCHK") == 0) {                               // $&PASCHK()
            *(*comp)++ = XCPAS;                                                // save the opcode
        } else if (strcmp(name, "V") == 0) {                                    // $&V()
            *(*comp)++ = XCV;                                                  // save the opcode
        } else if (strcmp(name, "X") == 0) {                                    // $&X()
            *(*comp)++ = XCX;                                                  // save the opcode
        } else if (strcmp(name, "XRSM") == 0) {                                 // $&XRSM()
            *(*comp)++ = XCXRSM;                                               // save the opcode
        } else if (strcmp(name, "%SETENV") == 0) {                              // $&%SETENV()
            *(*comp)++ = XCSETENV;                                             // save the opcode
        } else if (strcmp(name, "%GETENV") == 0) {                              // $&%GETENV()
            *(*comp)++ = XCGETENV;                                             // save the opcode
        } else if (strcmp(name, "%ROUCHK") == 0) {                              // $&%ROUCHK()
            *(*comp)++ = XCROUCHK;                                             // save the opcode
        } else if (strcmp(name, "%FORK") == 0) {                                // $&%FORK()
            *(*comp)++ = XCFORK;                                               // save the opcode
        } else if (strcmp(name, "%IC") == 0) {                                  // $&%IC()
            *(*comp)++ = XCIC;                                                 // save the opcode
        } else if (strcmp(name, "%WAIT") == 0) {                                // $&%WAIT()
            *(*comp)++ = XCWAIT;                                               // save the opcode
        } else if (strcmp(name, "DEBUG") == 0) {                                // $&DEBUG()
            *(*comp)++ = XCDEBUG;                                              // save the opcode
        } else if (strcmp(name, "%COMPRESS") == 0) {                            // $&%COMPRESS()
            *(*comp)++ = XCCOMP;                                               // save the opcode
        } else {
          comperror_temp(src,comp,partab_ptr,-(ERRZ18 + ERRMLAST));                                    // junk
        }

        return;                                                                 // end of xcalls
    }

    name[0] = c;                                                                // save first char

    for (len = 0; isalpha((*src)[len]) != 0; len++) {                       // scan string
        name[len + 1] = (*src)[len];                                        // copy alphas
    }

    (*src) += len;                                                          // move source along
    len++;                                                                      // add in first character
    name[len] = '\0';                                                           // null terminate name
    if (*(*src) == '(') goto function;                                      // check for a function

function:                                                                       // function code starts here
    (*src)++;                                                               // incr past the bracket
    ptr = (*comp);                                                             // remember where this goes
    sel = ((name[0] == 'S') && (toupper((int) name[1]) != 'T'));                // is $SELECT

    // $DATA, $GET, $INCREMENT, $NAME/$NEXT, $ORDER
    if ((name[0] == 'D') || (name[0] == 'G') || (name[0] == 'I') || (name[0] == 'N') || (name[0] == 'O') ||
      // $QUERY, but not $QSUBSCRIPT, and not $QLENGTH
      ((name[0] == 'Q') && (toupper((int) name[1]) != 'S') && (toupper((int) name[1]) != 'L'))) {
        if (*(*src) == '@') {                                               // indirection ?
          atom_ffi(src,comp,partab_ptr);                                                             // eval it
            ptr = (*comp) - 1;                                                 // remember where this goes

            if (*ptr == INDEVAL) {                                              // if it's going to eval it
                if ((name[0] == 'N') || (name[0] == 'O') || (name[0] == 'Q')) { // $NAME, $ORDER or $QUERY
                    *ptr = INDMVARN;                                            // allow null subs
                } else {
                    *ptr = INDMVAR;                                             // make an mvar from it
                }
            } else {                                                            // experimental for $ORDER(@.@())
                ptr -= 2;                                                       // back up over subs to type

                if (*ptr == OPVAR) {
                    if ((name[0] == 'N') || (name[0] == 'O') || (name[0] == 'Q')) { // $NAME, $ORDER or $QUERY
                        *ptr = OPMVARN;                                         // allow null subs
                    } else {
                        *ptr = OPMVAR;                                          // change to OPMVAR
                    }
                }
            }
        } else {
          parse_local_var_ffi(src,comp,partab_ptr);                                                     // we need a var

          if ((name[0] == 'N') || (name[0] == 'O') || (name[0] == 'Q')) {     // $NAME, $ORDER or $QUERY
            *ptr = OPMVARN;                                                 // allow null subs
          } else {
            *ptr = OPMVAR;                                                  // change to a OPMVAR
          }
        }
    } else if ((name[0] == 'T') && (toupper((int) name[1]) != 'R')) {           // $TEXT
      i = routine(-2);                                                        // parse to strstk

      if (i < -4) {                                                           // check for error
        comperror_temp(src,comp,partab_ptr,i);                                                       // complain
        return;                                                             // and exit
      }
    } else {
      eval_ffi(src,comp,partab_ptr);                                                                 // for other functions
    }

    while (TRUE) {
      args++;                                                                 // count an argument
      if (args > 255) EXPRE;                                                  // too many args
      c = *(*src)++;                                                      // get term char
      if (c == ')') break;                                                    // all done if closing )

      if (sel) {                                                              // if in a $SELECT()
        if (c != ((args & 1) ? ':' : ',')) EXPRE;                           // must be colon or comma
        *(*comp)++ = ((args & 1) ? JMP0 : JMP);                            // the opcode
        selj[args] = (*comp);                                              // remember for offset
        (*comp) += sizeof(short);                                          // leave space for it
      } else if (c != ',') {                                                  // else must be a comma
        EXPRE;
      }                                                                       // end special $SELECT() stuff

      eval_ffi(src,comp,partab_ptr);                                                                 // get next argument
    }                                                                           // end of args loop

    switch (name[0]) {                                                          // dispatch on initial
    case 'A':                                                                   // $A[SCII]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "ascii\0", 6) != 0) EXPRE;
      }

      if (args == 1) {
        *(*comp)++ = FUNA1;                                                // one arg form
        return;                                                             // and exit
      }

      if (args == 2) {
        *(*comp)++ = FUNA2;                                                // two arg form
        return;                                                             // and exit
      }

      EXPRE;

    case 'C':                                                                   // $C[HARACTER]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "char\0", 5) != 0) EXPRE;
      }

      if (args > 255) EXPRE;                                                  // check number of args
      *(*comp)++ = FUNC;                                                     // push the opcode
      *(*comp)++ = (u_char) args;                                            // number of arguments
      return;                                                                 // and give up

    case 'D':                                                                   // $D[ATA]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "data\0", 5) != 0) EXPRE;
      }

      if (args > 1) EXPRE;                                                    // check number of args
      *(*comp)++ = FUND;                                                     // set the opcode
      return;                                                                 // and give up

    case 'E':                                                                   // $E[XTRACT]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "extract\0", 8) != 0) EXPRE;
      }

      if (args == 1) {
        *(*comp)++ = FUNE1;                                                // one arg form
        return;                                                             // and exit
      }

      if (args == 2) {
        *(*comp)++ = FUNE2;                                                // two arg form
        return;                                                             // and exit
      }

      if (args == 3) {
        *(*comp)++ = FUNE3;                                                // two arg form
        return;                                                             // and exit
      }

      EXPRE;

    case 'F':                                                                   // $F[IND] and $FN[UMBER]
      if ((len == 1) || (strncasecmp(name, "find\0", 5) == 0)) {              // $F[IND]
        if (args == 2) {
          *(*comp)++ = FUNF2;                                            // two arg form
          return;                                                         // and exit
        }

        if (args == 3) {
          *(*comp)++ = FUNF3;                                            // three arg form
          return;                                                         // and exit
        }

        EXPRE;
      }                                                                       // end $FIND

      if (((len == 2) && (toupper((int) name[1]) == 'N')) || (strncasecmp(name, "fnumber\0", 8) == 0)) { // $FNUMBER
        if (args == 2) {
          *(*comp)++ = FUNFN2;                                           // two arg form
          return;                                                         // and exit
        }

        if (args == 3) {
          *(*comp)++ = FUNFN3;                                           // two arg form
          return;                                                         // and exit
        }

        EXPRE;
      }                                                                       // end $FIND

      EXPRE;

    case 'G':                                                                   // $G[ET]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "get\0", 4) != 0) EXPRE;
      }

      if (args == 1) {
        *(*comp)++ = FUNG1;                                                // one arg form
      } else if (args == 2) {
        *(*comp)++ = FUNG2;                                                // the two arg opcode
      } else {
        EXPRE;                                                              // all others junk
      }

      return;                                                                 // done

    case 'I':                                                                   // $I[NCREMENT]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "increment\0", 10) != 0) EXPRE;
      }

      if (args == 1) {
        *(*comp)++ = FUNI1;                                                // one arg form
      } else if (args == 2) {
        *(*comp)++ = FUNI2;                                                // the two arg opcode
      } else {
        EXPRE;                                                              // all others junk
      }

      return;                                                                 // done

    case 'J':                                                                   // $J[USTIFY]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "justify\0", 8) != 0) EXPRE;
      }

      if (args == 2) {
        *(*comp)++ = FUNJ2;                                                // two arg form
      } else if (args == 3) {
        *(*comp)++ = FUNJ3;                                                // three arg form
      } else {                                                                // all else is junk
        EXPRE;
      }

      return;                                                                 // and exit

    case 'L':                                                                   // $L[ENGTH]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "length\0", 7) != 0) EXPRE;
      }

      if (args == 1) {
        *(*comp)++ = FUNL1;                                                // one arg form
      } else if (args == 2) {
        *(*comp)++ = FUNL2;                                                // two arg form
      } else {
        EXPRE;
      }

      return;

    case 'N':                                                                   // $NA[ME] or $N[EXT]
      if (toupper((int) name[1]) != 'A') {                                    // check second letter
        if (len > 1) {
          if (strncasecmp(name, "next\0", 5) != 0) EXPRE;
        }

        if (!(systab->historic & HISTORIC_DNOK)) EXPRE;
        if (args != 1) EXPRE;
        *(*comp)++ = OPSTR;
        us = 1;                                                             // the string length
        assert(sizeof(us) == sizeof(u_short));
        memcpy((*comp), &us, sizeof(u_short));
        (*comp) += sizeof(u_short);
        *(*comp)++ = '2';                                                  // $NEXT kludge
        *(*comp)++ = '\0';                                                 // null terminated
        *(*comp)++ = FUNO2;                                                // two arg form of $ORDER()
        return;
      }

      if (len > 2) {                                                          // check for extended name
        if (strncasecmp(name, "name\0", 5) != 0) EXPRE;
      }

      if (args == 1) {
        *(*comp)++ = FUNNA1;                                               // one arg form
      } else if (args == 2) {
        *(*comp)++ = FUNNA2;                                               // two arg opcode
      } else {                                                                // all else is junk
        EXPRE;
      }

      return;                                                                 // and exit

    case 'O':                                                                   // $O[RDER]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "order\0", 6) != 0) EXPRE;
      }

      if (args == 1) {
        *(*comp)++ = FUNO1;                                                // one arg form
      } else if (args == 2) {
        *(*comp)++ = FUNO2;                                                // two arg form
      } else {
        EXPRE;
      }

      return;

    case 'P':                                                                   // $P[IECE]
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "piece\0", 6) != 0) {
          comperror_temp(src,comp,partab_ptr,-(ERRZ12 + ERRMLAST));                                // compile an error
          return;                                                         // and give up
        }
      }

      if (args == 2) {
        *(*comp)++ = FUNP2;                                                // two arg form
      } else if (args == 3) {
        *(*comp)++ = FUNP3;                                                // three arg form
      } else if (args == 4) {
        *(*comp)++ = FUNP4;                                                // four arg form
      } else {
        EXPRE;
      }

      return;

    case 'Q':                                                                   // $Q[UERY], $QS[UBSCRIPT], and $QL[ENGTH]
      if ((len == 1) || (strncasecmp(name, "query\0", 6) == 0)) {             // $Q[UERY]
        if (args == 1) {
          *(*comp)++ = FUNQ1;                                            // one arg form
        } else if (args == 2) {
          *(*comp)++ = FUNQ2;                                            // two arg form
        } else {
          EXPRE;
        }

        return;                                                             // and exit
      }                                                                       // end $Q[UERY]

      if (((len == 2) && (toupper((int) name[1]) == 'L')) || (strncasecmp(name, "qlength\0", 8) == 0)) { // $QLENGTH
        if (args == 1) {
          *(*comp)++ = FUNQL;
          return;                                                         // and exit
        }

        EXPRE;
      }                                                                       // end $FIND

      if (((len == 2) && (toupper((int) name[1]) == 'S')) || (strncasecmp(name, "qsubscript\0", 11) == 0)) { // $QSUBSCRIPT
        if (args == 2) {
          *(*comp)++ = FUNQS;
          return;                                                         // and exit
        }

        EXPRE;
      }                                                                       // end $FIND

      EXPRE;

    case 'R':                                                                   // $R[ANDOM], $RE[VERSE]
      if ((len == 1) || (strncasecmp(name, "random\0", 7) == 0)) {            // $R[ANDOM]
        if (args == 1) {
          *(*comp)++ = FUNR;                                             // one arg form
          return;                                                         // and exit
        }

        EXPRE;
      }

      if (((len == 2) && (toupper((int) name[1]) == 'E')) || (strncasecmp(name, "reverse\0", 8) == 0)) { // $REVERSE
        if (args == 1) {
          *(*comp)++ = FUNRE;
          return;                                                         // and exit
        }

        EXPRE;
      }

      EXPRE;

    case 'S':                                                                   // $S[ELECT], $ST[ACK]
      if ((len == 1) || (strncasecmp(name, "select\0", 7) == 0)) {            // $S[ELECT]
        if (args & 1) {                                                     // must be even number
          (*comp) = ptr;                                                 // start of this
          EXPRE;                                                          // and error it
        }

        *(*comp)++ = JMP;                                                  // for the last expr
        selj[args] = (*comp);                                              // remember for offset
        (*comp) += sizeof(short);                                          // leave space for it
        selj[args + 1] = (*comp);                                          // for the last JMP0
        *(*comp)++ = OPERROR;                                              // no TVE is an error
        assert(sizeof(errm4) == sizeof(short));
        memcpy((*comp), &errm4, sizeof(short));
        (*comp) += sizeof(short);

        for (i = 1; i <= args; i++) {                                       // scan the addr array
          if (i & 1) {
            *((short *) selj[i]) = (short) (selj[i + 1] - selj[i]);
          } else {
            *((short *) selj[i]) = (short) ((*comp) - selj[i]) - sizeof(short);
          }
        }

        return;                                                             // end of $SELECT()
      }

      if (((len == 2) && (toupper((int) name[1]) == 'T')) || (strncasecmp(name, "stack\0", 6) == 0)) { // $ST[ACK]
        if (args == 1) {
          *(*comp)++ = FUNST1;
          return;                                                         // and exit
        }

        if (args == 2) {
          *(*comp)++ = FUNST2;
          return;                                                         // and exit
        }

        EXPRE;
      }

      EXPRE;

    case 'T':                                                                   // $T[EXT], $TR[ANSLATE]
      if ((len == 1) || (strncasecmp(name, "text\0", 5) == 0)) {              // $T[EXT]
        if (args == 1) {
          *(*comp)++ = FUNT;                                             // one arg form
          return;                                                         // and exit
        }

        EXPRE;
      }

      if (((len == 2) && (toupper((int) name[1]) == 'R')) || (strncasecmp(name, "translate\0", 10) == 0)) { // $TR[ANSLATE]
        if (args == 2) {
          *(*comp)++ = FUNTR2;
          return;                                                         // and exit
        }

        if (args == 3) {
          *(*comp)++ = FUNTR3;
          return;                                                         // and exit
        }

        EXPRE;
      }

      EXPRE;

    case 'V':                                                                   // $VIEW
      if (len > 1) {                                                          // check for extended name
        if (strncasecmp(name, "view\0", 5) != 0) EXPRE;
      }

      if (args == 2) {
        *(*comp)++ = FUNV2;                                                // two arg form
        return;                                                             // and exit
      }

      if (args == 3) {
        *(*comp)++ = FUNV3;                                                // three arg form
        return;                                                             // and exit
      }

      if (args == 4) {
        *(*comp)++ = FUNV4;                                                // four arg form
        return;                                                             // and exit
      }

      EXPRE;

    default:
      EXPRE;
    }                                                                           // end of switch
}
