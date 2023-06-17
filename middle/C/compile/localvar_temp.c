#include <stdio.h>                                                              // always include
#include <stdlib.h>                                                             // these two
#include <sys/types.h>                                                          // for u_char def
#include <string.h>
#include <ctype.h>
#include <errno.h>                                                              // error stuff
#include <limits.h>                                                             // for LONG_MAX etc.
#include <math.h>
#include "rsm.h"                                                                // standard includes
#include "proto.h"                                                              // standard prototypes
#include "error.h"                                                              // and the error defs
#include "opcode.h"                                                             // and the opcodes
#include "compile_temp.h"                                                            // compile stuff
#include "compile.h"                                                            // compile stuff

/*
 * Function localvar entered with source_ptr pointing at the source
 * variable to evaluate and comp_ptr pointing at where to put the code.
 *
 * Return    Means
 * -ERR      Nothing compiled, error returned
 * off       Offset from starting point of comp_ptr for OPVAR
 *
 * Following the OPVAR is a byte indicating type of
 * variable as per the following:
 *     TYPMAXSUB           63                                                   // max subscripts
 *     TYPVARNAM           0                                                    // name only (VAR_LEN bytes)
 *     TYPVARLOCMAX        TYPVARNAM+TYPMAXSUB                                  // local is 1->63 subs
 *     TYPVARIDX           64                                                   // 1 byte index (+ #subs)
 *     TYPVARGBL           128                                                  // first global
 *     TYPVARGBLMAX        TYPVARGBL+TYPMAXSUB                                  // global 128->191 subs
 *     TYPVARNAKED         252                                                  // global naked reference
 *     TYPVARGBLUCI        253                                                  // global with UCI
 *     TYPVARGBLUCIENV     254                                                  // global with UCI and env
 *     TYPVARIND           255                                                  // indirect
 *
 * For TYPVARNAM:          OPVAR TYPVARNAM (var_u) name
 *     TYPVARLOC:          subscripts OPVAR TYPVARNAM+#subs (var_u) name
 *     TYPVARIDX:          subscripts OPVAR TYPVARIDX+#subs (u_char) idx
 *     TYPVARGBL:          subscripts OPVAR TYPVARGBL+#subs (var_u) name
 *     TYPVARNAKED:        subscripts OPVAR TYPVARNAKED #subs
 *     TYPVARGBLUCI:       subscripts UCI OPVAR TYPVARGBLUCI #subs (var_u) name
 *     TYPVARGBLUCIENV:    subs UCI env OPVAR TYPVARGBLUCIENV #subs (var_u) name
 *     TYPVARIND:          (str on addstk[]) [subs] OPVAR TYPEVARIND #subs
*/


short localvar_temp(u_char **src,u_char **comp,partab_struct *partab_ptr)                                                            // evaluate local variable
{
    char   c;                                                                   // current character
    u_char idx = 0;                                                             // index
    int    i;                                                                   // a useful int
    int    v;                                                                   // for vertical bar
    int    count = 0;                                                           // count subscripts
    var_u  var;                                                                 // to hold variable names
    u_char *sptr;                                                               // to save source_ptr
    u_char *ptr;                                                                // to save comp_ptr
    short  type = TYPVARNAM;                                                    // the type code
    short  ret;                                                                 // the return


    ptr = (*comp);                                                             // save (*comp)
    c = *(*src)++;                                                          // get a character

    if (c == '^') {                                                             // if it's global
        type = TYPVARGBL;                                                       // say it's global
        c = *(*src)++;                                                      // point past it
    }

    // check for a variable and if none return the error
    if ((isalpha((int) c) == 0) && (c != '%') && (c != '$')) return -(ERRZ12 + ERRMLAST);

    if ((c == '$') && (type == TYPVARNAM)) {                                    // check $...
        if (isalpha(*(*src)) == 0) return -(ERRZ12 + ERRMLAST);             // next must be alpha or return the error
        i = toupper(*(*src));                                               // get the next character

        /*
         * TODO: Add check for real intrinsic variables, not just their first letter
         *       $device, $ecode, $estack, $etrap, $horolog, $io, $job, $key, $principal,
         *       $quit, $reference, $storage, $stack, $system, $test, $x, $y, $zbp
         *       cf. dodollar() in rsm/compile/dollar.c
        */
        if (strchr("DEHIJKPQRSTXYZ", i) == NULL) return -ERRM8;                 // if letter is invalid complain
    }

    VAR_CLEAR(var);                                                             // clear the variable name
    var.var_cu[0] = c;                                                          // save first char

    for (i = 1; i < VAR_LEN; i++) {                                             // scan for rest of name
        c = *(*src)++;                                                      // get next char

        if (isalnum((int) c) == 0) {                                            // if not alpha numeric
            --(*src);                                                       // point back at it
            break;                                                              // and exit
        }

        var.var_cu[i] = c;                                                      // save in the variable
    }

    if (isalnum(*(*src)) != 0) return -ERRM56;                              // complain about name length

subs:
    if (*(*src) == '(') {                                                   // see if it's subscripted
        (*src)++;                                                           // skip the bracket

        while (TRUE) {                                                          // loop
          eval_temp(src,comp,partab_ptr);                                                             // get a subscript
            count++;                                                            // count it
            c = *(*src)++;                                                  // get next character
            if (c == ')') break;                                                // quit when done
            if (c != ',') return -(ERRZ12 + ERRMLAST);                          // return the error
        }
    }

    if (count > TYPMAXSUB) return -(ERRZ15 + ERRMLAST);                         // too many then error
    ret = (*comp) - ptr;                                                       // remember here
    *(*comp)++ = OPVAR;                                                        // opcode

    // candidate for index? and in a routine compile and it's not $...
    if ((type < TYPVARGBL) && ((*partab_ptr).varlst != NULL) && (var.var_cu[0] != '$')) {
        for (i = 0; ; i++) {                                                    // scan list
            if (i == (MAX_KEY_SIZE + 1)) break;                                 // too many
            if (var_equal((*partab_ptr).varlst[i], var)) break;                        // found it

            if (var_empty((*partab_ptr).varlst[i])) {
                VAR_COPY((*partab_ptr).varlst[i], var);                                // set it
                break;
            }
        }

        if (i != (MAX_KEY_SIZE + 1)) {
            type |= TYPVARIDX;                                                  // change the type
            idx = i;                                                            // save index
        }
    }

    if (type < TYPVARNAKED) {                                                   // normal local or global var
        type += count;                                                          // add the count
        *(*comp)++ = (u_char) type;                                            // store it
        if (type & TYPVARIDX) *(*comp)++ = idx;                                // index type? then save the index
    } else {                                                                    // funny type
        *(*comp)++ = (u_char) type;                                            // store the type
        *(*comp)++ = count;                                                    // then the subscripts
    }

    // if simple local (not idx) or a 'normal' global and not naked or indirect
    if (((type < TYPVARIDX) || (type >= TYPVARGBL)) && (type != TYPVARNAKED) && (type != TYPVARIND)) {
        for (i = 0; i < VAR_LEN; i++) *(*comp)++ = var.var_cu[i];              // scan the name and copy into compiled code
    }

    return ret;                                                                 // say what we did
}                                                                               // end variable parse
