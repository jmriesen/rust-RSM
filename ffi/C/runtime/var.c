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


#include <stdio.h>                                                              // always include
#include <stdlib.h>                                                             // these two
#include <sys/types.h>                                                          // for u_char def
#include <string.h>
#include <ctype.h>
#include <errno.h>                                                              // error stuff
#include "rsm.h"                                                                // standard includes
#include "proto.h"                                                              // standard prototypes
#include "error.h"                                                              // standard errors
#include "opcode.h"                                                             // for OPNOP

/*
 * All variables use the following structure
 *
 * short Vname(u_char *ret_buffer)
 *   or
 * int Vname(u_char *ret_buffer)
 *
 * The argument is of type *u_char and is the destination
 * for the value returned by the function (max size is MAX_STR_LEN).
 * The function returns a count of characters in the return string
 * or a negative error number (to be defined).
 * The function name is Vvarname where the variable call is $varname.
 */

// $ECODE
int Vecode(u_char *ret_buffer)
{
    mvar *var;                                                                  // for ST_Get
    int  t;

    var = (mvar *) ret_buffer;                                                  // use here for the mvar
    VAR_CLEAR(var->name);
    memcpy(&var->name.var_cu[0], "$ECODE", 6);                                  // get the name
    var->volset = 0;                                                            // clear volset
    var->uci = UCI_IS_LOCALVAR;                                                 // local var
    var->slen = 0;                                                              // no subscripts
    t = ST_Get(var, ret_buffer);                                                // get it

    if (t == -ERRM6) {
        t = 0;                                                                  // ignore undef
        ret_buffer[0] = '\0';                                                   // null terminate
    }

    return t;
}

// $ETRAP
int Vetrap(u_char *ret_buffer)
{
    mvar *var;                                                                  // for ST_Get
    int  t;

    var = (mvar *) ret_buffer;                                                  // use here for the mvar
    VAR_CLEAR(var->name);
    memcpy(&var->name.var_cu[0], "$ETRAP", 6);                                  // get the name
    var->volset = 0;                                                            // clear volset
    var->uci = UCI_IS_LOCALVAR;                                                 // local var
    var->slen = 0;                                                              // no subscripts
    t = ST_Get(var, ret_buffer);                                                // exit with result
    if (t == -ERRM6) t = 0;                                                     // ignore undef
    return t;
}

// $HOROLOG
short Vhorolog(u_char *ret_buffer)
{
    time_t sec = current_time(TRUE);                                            // get secs from 1 Jan 1970 with local offset
    int    day = sec / SECDAY + YRADJ;                                          // get number of days

    sec %= SECDAY;                                                              // and number of seconds
    return (short) snprintf((char *) ret_buffer, 14, "%d,%d", day, (int) sec);  // return count and $HOROLOG
}

// $KEY
short Vkey(u_char *ret_buffer)
{
    SQ_Chan *ioptr;                                                             // ptr to current $IO

    ioptr = &partab.jobtab->seqio[(int) partab.jobtab->io];                     // point at it
    return (short) mcopy(&ioptr->dkey[0], ret_buffer, ioptr->dkey_len);         // return count and $KEY
}

// $REFERENCE
short Vreference(u_char *ret_buffer)
{
    mvar *var;                                                                  // variable pointer

    var = &partab.jobtab->last_ref;                                             // point at $R
    ret_buffer[0] = '\0';                                                       // null JIC
    if (var->name.var_cu[0] == '\0') return 0;                                  // return null string if null
    return UTIL_String_Mvar(var, ret_buffer, MAX_NUM_SUBS);                     // do it elsewhere
}

// $SYSTEM
short Vsystem(u_char *ret_buffer)
{
    int i = ultocstring(ret_buffer, RSM_SYSTEM);                                // copy assigned #

    ret_buffer[i++] = ',';                                                      // and a comma
    i += rsm_version(&ret_buffer[i]);                                           // do it elsewhere
    return (short) i;                                                           // return the count
}

// $X
short Vx(u_char *ret_buffer)
{
    const SQ_Chan *ioptr = &partab.jobtab->seqio[(int) partab.jobtab->io];      // ptr to current $IO

    return (short) ultocstring(ret_buffer, ioptr->dx);                          // return len with data in buf
}

// $Y
short Vy(u_char *ret_buffer)
{
    const SQ_Chan *ioptr = &partab.jobtab->seqio[(int) partab.jobtab->io];      // ptr to current $IO

    return (short) ultocstring(ret_buffer, ioptr->dy);                          // return len with data in buf
}

/*
 * Set special variables - those that may be set are:
 *     $EC[ODE]
 *     $ET[RAP]
 *     $K[EY]
 *     $X
 *     $Y
 */
int Vset(mvar *var, cstring *cptr)                                              // set a special variable
{
    int i;                                                                      // a useful int

    if (var->slen != 0) return -ERRM8;                                          // no subscripts permitted

    if ((strncasecmp((char *) &var->name.var_cu[1], "ec\0", 3) == 0) ||
      (strncasecmp((char *) &var->name.var_cu[1], "ecode\0", 6) == 0)) {        // $EC[ODE]
        for (i = 0; i < cptr->len; i++) {                                       // if it isn't a graphic character
            if (iscntrl(cptr->buf[i])) return -ERRM101;
        }

        if ((cptr->len > 1) && (cptr->buf[0] == ',') && (cptr->buf[cptr->len - 1] == ',')) { // if it starts and ends with a comma
            cptr->len--;
            memmove(&cptr->buf[0], &cptr->buf[1], cptr->len--);                 // ignore the commas
            cptr->buf[cptr->len] = '\0';                                        // and nul terminate
            cptr->buf[cptr->len + 1] = OPNOP;                                   // don't confuse the interpreter
        }

        if ((cptr->len == 0) || (((cptr->buf[0] == 'M') ||                      // set to null ok or Manything,Manything,Manything
          (cptr->buf[0] == 'Z') || (cptr->buf[0] == 'U')) &&                    // set to Zanything,Zanything,Uanything,Uanything
          (cptr->buf[cptr->len - 1] != ','))) {                                 // and does not end with a comma
            char *code;

            code = strtok((char *) cptr->buf, ",");                             // check for error code format

            if ((code != NULL) && (code[0] != 'M') && (code[0] != 'Z') && (code[0] != 'U')) { // check for proper format
                return -ERRM101;
            }

            while ((code = strtok(NULL, ",")) != NULL) {                        // loop through each argument
                if ((code[0] != 'M') && (code[0] != 'Z') && (code[0] != 'U')) { // check for proper format
                    return -ERRM101;
                }

                code[-1] = ',';                                                 // change back to correct delimiter
            }

            VAR_CLEAR(var->name);
            memcpy(&var->name.var_cu[0], "$ECODE", 6);                          // ensure name correct
            partab.jobtab->error_frame = 0;                                     // and where the error happened
            partab.jobtab->etrap_at = 0;                                        // not required
            if (cptr->len == 0) return ST_Kill(var);                            // if we are clearing it then kill it
            return USRERR;                                                      // do it elsewhere
        }

        return -ERRM101;                                                        // can't do that
    }

    if ((strncasecmp((char *) &var->name.var_cu[1], "et\0", 3) == 0) ||
      (strncasecmp((char *) &var->name.var_cu[1], "etrap\0", 6) == 0)) {        // $ET[RAP]
        VAR_CLEAR(var->name);
        memcpy(&var->name.var_cu[0], "$ETRAP", 6);                              // ensure name correct
        if (cptr->len == 0) return ST_Kill(var);                                // kill it
        return ST_Set(var, cptr);                                               // do it in symbol
    }

    if ((strncasecmp((char *) &var->name.var_cu[1], "k\0", 2) == 0) ||
      (strncasecmp((char *) &var->name.var_cu[1], "key\0", 4) == 0)) {          // $K[EY]
        if (cptr->len > MAX_DKEY_LEN) return -ERRM75;                           // too big
        memcpy(partab.jobtab->seqio[partab.jobtab->io].dkey, cptr->buf, cptr->len + 1); // copy this many (incl null)
        partab.jobtab->seqio[partab.jobtab->io].dkey_len = cptr->len;
        return 0;
    }

    if (strncasecmp((char *) &var->name.var_cu[1], "x\0", 2) == 0) {            // $X
        i = cstringtoi(cptr);                                                   // get val
        if ((i < 0) || (i > (MAX_STR_LEN + 1))) return -ERRM43;                 // return range error
        partab.jobtab->seqio[partab.jobtab->io].dx = (u_short) i;
        return 0;                                                               // and return
    }

    if (strncasecmp((char *) &var->name.var_cu[1], "y\0", 2) == 0) {            // $Y
        i = cstringtoi(cptr);                                                   // get val
        if ((i < 0) || (i > (MAX_STR_LEN + 1))) return -ERRM43;                 // return range error
        partab.jobtab->seqio[partab.jobtab->io].dy = (u_short) i;
        return 0;                                                               // and return
    }

    return -ERRM8;                                                              // else junk
}
