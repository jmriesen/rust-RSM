/*
 * Package: Reference Standard M
 * File:    rsm/symbol/util.c
 * Summary: module symbol - symbol table utilities
 *
 * David Wicksell <dlw@linux.com>
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2016
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
 * SPDX-FileCopyrightText:  © 2020 David Wicksell <dlw@linux.com>
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

#include <stdio.h>                                                              // always include
#include <stdlib.h>                                                             // these two
#include <sys/types.h>                                                          // for u_char def
#include <string.h>                                                             // for string ops
#include <ctype.h>
#include "rsm.h"                                                                // standard includes
#include "symbol.h"                                                             // our definitions
#include "error.h"                                                              // errors
#include "proto.h"                                                              // standard prototypes
#include "compile.h"                                                            // for routine buffer stuff
#include "rust.h"

/*
 * Function: TMP_LocateIdx - locate in symbol table by index
 * returns short pointer or -1 on fail
 */
short TMP_LocateIdx(int idx,table_struct * table)                                                     // var index
{
    short fwd;                                                                  // fwd link pointer
    var_u var;                                                                  // var name (if required)
    rbd   *p;                                                                   // for looking at routines
    var_u *vt;                                                                  // for the var table

    fwd = partab.jobtab->dostk[partab.jobtab->cur_do].symbol[idx];
    if (fwd > -1) return fwd;                                                   // got it
    p = (rbd *) SOA(partab.jobtab->dostk[partab.jobtab->cur_do].routine);
    vt = (var_u *) (((u_char *) p) + p->var_tbl);                               // point at var table
    VAR_COPY(var, vt[idx]);                                                     // get the var name
    fwd = TMP_SymAtt(var,table);                                                       // attach and get index
    if (fwd < 0) return fwd;                                                    // error if none free
    partab.jobtab->dostk[partab.jobtab->cur_do].symbol[idx] = fwd;              // save idx
    return fwd;                                                                 // return index
}                                                                               // end of TMP_LocateIdx()

/*
 * Function: FixData(TMP_data *old, ST_data *new, int count)
 * When the data pointer changes, this fixes count pointers to it
 * Only called internally from this file
 */
void FixData(const ST_data *old, ST_data *new, int count, table_struct * table)
{
    int       i;                                                                // for loops
    ST_newtab *newtab;                                                          // for NEW tables

    for (i = 0; i < TMP_MAX; i++) {                                              // scan symtab[]
        if (table->sym_tab[i].data == old) {                                            // same as our old one
            table->sym_tab[i].data = new;                                               // change to new one
            count--;                                                            // decrement the count
            if (count == 0) return;                                             // quit when done
        }
    }

    i = partab.jobtab->cur_do;                                                  // get current do level

    while (i) {                                                                 // for each one
        newtab = (ST_newtab *) partab.jobtab->dostk[i--].newtab;                // get newtab

        while (newtab != NULL) {                                                // for each table
            for (int c = 0; c < newtab->count_new; c++) {                       // for each variable
                if (newtab->locdata[c].data == old) {                           // if this is one
                    newtab->locdata[c].data = new;                              // copy in new value
                    count--;                                                    // count that
                    if (count == 0) return;                                     // quit when done
                }
            }

            newtab = newtab->fwd_link;                                          // get next
        }
    }
}
/*
 * Function: TMP_Data - examine type of variable
 * returns pointer to length of type of data
 *
 * *buf is set to 0, 1, 10, 11 depending on the type of data *var is
 * returned is the length of *buf, not including the /0 terminator
 *      0 = The variable has no data and no descendants (i.e., undef)
 *      1 = The variable has data but no descendants
 *     10 = The variable has no data but has descendants
 *     11 = The variable has data and has descendants
 */
short TMP_Data(mvar *var, u_char *buf, table_struct * table)                                           // put var type in buf
{
    int       ptr1;                                                             // position in symtab
    ST_depend *depPtr = ST_DEPEND_NULL;                                         // active pointer
    ST_depend *prevPtr = ST_DEPEND_NULL;                                        // pointer to previous element

    if (var->volset) {                                                          // if by index
        ptr1 = TMP_LocateIdx(var->volset - 1,table);                                   // get it this way
    } else {                                                                    // if no volset, use name
        ptr1 = TMP_Locate(var->name,table);                                            // locate the variable by name
    }

    if (ptr1 == -1) {                                                           // not found
        memcpy(buf, "0\0", 2);
        return 1;                                                               // end if-no data&no descendants
    }                                                                           // end if not found

    if (table->sym_tab[ptr1].data == ST_DATA_NULL) {                                    // not found
        memcpy(buf, "0\0", 2);
        return 1;                                                               // return length of buf
    }                                                                           // end if not found

    if (var->slen > 0) {                                                        // going into dependents
        int       i;
        ST_depend *lastkey;

        i = var->slen;                                                          // get the length
        lastkey = table->sym_tab[ptr1].data->last_key;                                  // pointer to last used key
        depPtr = table->sym_tab[ptr1].data->deplnk;                                     // get first dependent

        if (depPtr != ST_DEPEND_NULL) {                                         // only if we should go on
            if (depPtr->keylen < i) i = depPtr->keylen;                         // adjust length if needed
        }                                                                       // end if

        // start search at last used key, rather than at the beginning (var after lastkey)
        if ((lastkey != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, lastkey->bytes, var->slen, lastkey->keylen) > 0)) {
            depPtr = lastkey;
        }

        while ((depPtr != ST_DEPEND_NULL) && (memcmp(depPtr->bytes, var->key, i) < 0)) { // while not yet found or past
            prevPtr = depPtr;                                                   // save previous
            depPtr = depPtr->deplnk;                                            // go to next
            if (depPtr == ST_DEPEND_NULL) break;                                // have we run out
            i = var->slen;                                                      // get the length again
            if (depPtr->keylen < i) i = depPtr->keylen;                         // adjust length if needed
        }                                                                       // end while

        table->sym_tab[ptr1].data->last_key = prevPtr;                                  // add last used key

        if (depPtr == ST_DEPEND_NULL) {                                         // if we ran out of deps
            memcpy(buf, "0\0", 2);
            return 1;                                                           // return same
        }                                                                       // end if

        i = var->slen;                                                          // get the length again
        if (depPtr->keylen < i) i = depPtr->keylen;                             // adjust length if needed

        while (((depPtr != ST_DEPEND_NULL) && (memcmp(depPtr->bytes, var->key, i) == 0) && // while more deps and matches ok for i
          (depPtr->keylen < var->slen)) || ((depPtr != ST_DEPEND_NULL) &&       // an exact match and var slen still longer
          (memcmp(depPtr->bytes, var->key, i) < 0))) {                          // while more deps and
            prevPtr = depPtr;                                                   // save previous
            depPtr = depPtr->deplnk;                                            // go to next
            if (depPtr == ST_DEPEND_NULL) break;                                // have we run out
            i = var->slen;                                                      // get the length again
            if (depPtr->keylen < i) i = depPtr->keylen;                         // adjust length if needed
        }                                                                       // end while

        table->sym_tab[ptr1].data->last_key = prevPtr;                                  // add last used key again

        if (depPtr == ST_DEPEND_NULL) {                                         // if we ran out
            memcpy(buf, "0\0", 2);
            return 1;                                                           // return same
        }                                                                       // end if

        if ((depPtr != ST_DEPEND_NULL) && (memcmp(depPtr->bytes, var->key, i) == 0)) { // if matches ok for i
            if (depPtr->keylen == var->slen) {                                  // exact match
                prevPtr = depPtr;                                               // save previous
                depPtr = depPtr->deplnk;                                        // go to next
                table->sym_tab[ptr1].data->last_key = prevPtr;                          // add last used key again

                if (depPtr == ST_DEPEND_NULL) {                                 // have we run out
                    memcpy(buf, "1\0", 2);
                    return 1;                                                   // return same
                }                                                               // end if

                i = var->slen;                                                  // get the length
                if (depPtr->keylen < i) i = depPtr->keylen;                     // adjust len if needed

                if (memcmp(depPtr->bytes, var->key, i) == 0) {                  // if match ok for i
                    memcpy(buf, "11\0", 3);
                    return 2;                                                   // return same
                } else {                                                        // end if
                    memcpy(buf, "1\0", 2);
                    return 1;                                                   // return same
                }
            } else {                                                            // end if exact match - beyond exact match
                memcpy(buf, "10\0", 3);
                return 2;                                                       // return same
            }                                                                   // end else beyond exact match
        } else {                                                                // end if match to i - no longer matches for i
            memcpy(buf, "0\0", 2);
            return 1;                                                           // return same
        }
    } else {                                                                    // end if-going to dependents - work on data block
        // dbc defined with data and no descendants
        if ((table->sym_tab[ptr1].data->dbc != VAR_UNDEFINED) && (table->sym_tab[ptr1].data->deplnk == ST_DEPEND_NULL)) {
            memcpy(buf, "1\0", 2);
            return 1;
        }                                                                       // end if-data&no descendants

        // dbc not defined with no data and descendants
        if ((table->sym_tab[ptr1].data->dbc == VAR_UNDEFINED) && (table->sym_tab[ptr1].data->deplnk != ST_DEPEND_NULL)) {
            memcpy(buf, "10\0", 3);
            return 2;
        }                                                                       // end if-no data&descendants

        // dbc defined with data and descendants
        if ((table->sym_tab[ptr1].data->dbc != VAR_UNDEFINED) && (table->sym_tab[ptr1].data->deplnk != ST_DEPEND_NULL)) {
            memcpy(buf, "11\0", 3);
            return 2;
        }                                                                       // end if-data&descendants
    }                                                                           // end else-ops on data block

    memcpy(buf, "0\0", 2);
    return 1;                                                                   // return
}                                                                               // end TMP_Data

/*
 * Function: TMP_Order - get next subscript in sequence, forward or reverse
 * returns pointer to length of next subscript
 */
short TMP_Order(mvar *var, u_char *buf, int dir, table_struct * table)
{
    int       ptr1;                                                             // position in symtab
    ST_depend *current = ST_DEPEND_NULL;                                        // active pointer
    ST_depend *prev = ST_DEPEND_NULL;                                           // pointer to previous element
    ST_depend *lastkey = ST_DEPEND_NULL;                                        // pointer to last used key
    int       pieces = 0;                                                       // subscripts in key
    int       subs;
    char      keysub[256];                                                      // current key subscript
    u_char    upOneLev[256];
    u_char    crud[256];
    int       index = 0;                                                        // where up to in key extract
    short     ret = 0;                                                          // current position in key

    if (var->volset) {                                                          // if by index
        ptr1 = TMP_LocateIdx(var->volset - 1,table);                                   // get it this way
    } else {                                                                    // no volset, so use var name
        ptr1 = TMP_Locate(var->name,table);                                            // locate the variable by name
    }

    buf[0] = '\0';                                                              // JIC
    if (ptr1 < 0) return 0;
    if (var->slen == 0) return 0;                                               // if can't $ORDER a data block then return null
    UTIL_Key_Chars_In_Subs((char *) var->key, (int) var->slen, 255, &pieces, (char *) NULL); // Return num of sub in pieces

    /*
     * Return characters in all but last subscript, number of subscripts in subs
     * and key string (less last subscript) at upOneLev[1] on for upOneLev[0] bytes
     */
    upOneLev[0] = (u_char) UTIL_Key_Chars_In_Subs((char *) var->key, (int) var->slen, pieces - 1, &subs, (char *) &upOneLev[1]);

    if (table->sym_tab[ptr1].data == ST_DATA_NULL) return 0;                            // no data
    current = table->sym_tab[ptr1].data->deplnk;                                        // go to first dependent
    lastkey = table->sym_tab[ptr1].data->last_key;                                      // pointer to last used key

    // start search at last used key, rather than at the beginning (var after lastkey)
    if ((lastkey != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, lastkey->bytes, var->slen, lastkey->keylen) > 0) && (dir == 1)) {
        current = lastkey;
    }

    // compare keys - while we have dependent and key match fails (var after current)
    while ((current != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, current->bytes, var->slen, current->keylen) > 0)) {
        prev = current;                                                         // set prev pointer
        current = current->deplnk;                                              // go to next dependent pointer
    }                                                                           // end while-compare keys

    table->sym_tab[ptr1].data->last_key = prev;                                         // add last used key

    if (current == ST_DEPEND_NULL) {                                            // nothing past our key
        if (dir == 1) return 0;
    }                                                                           // output same, return length

    if (dir == -1) {                                                            // reverse order
        current = prev;                                                         // save current
        if (current == ST_DEPEND_NULL) return 0;                                // if pointing nowhere then return length of zero
        crud[0] = UTIL_Key_Chars_In_Subs((char *) current->bytes, (int) current->keylen, pieces - 1, &subs, (char *) &crud[1]);

        if ((crud[0] != 0) && (upOneLev[0] != 0)) {
            if (crud[0] != upOneLev[0]) return 0;
            if (memcmp(&crud[1], &upOneLev[1], upOneLev[0]) != 0) return 0;
            // Ensure higher level subscripts (if any) are equal
        }
    } else {                                                                    // end if reverse order - forward order
        // while we have dependents and key cmp fails
        while ((current != ST_DEPEND_NULL) && (memcmp(current->bytes, var->key, var->slen) == 0)) {
            current = current->deplnk;                                          // go to next dependent
        }                                                                       // end while

        if (current == ST_DEPEND_NULL) return 0;                                // if current points nowhere, return length of zero

        // compare keys, if compare fails to match (var before or after current)
        if (UTIL_Key_KeyCmp(var->key, current->bytes, var->slen, current->keylen) != 0) {
            crud[0] = UTIL_Key_Chars_In_Subs((char *) current->bytes, (int) current->keylen, pieces - 1, &subs, (char *) &crud[1]);

            if ((crud[0] != 0) && (upOneLev[0] != 0)) {                         // if lengths aren't 0
                if (memcmp(&crud[1], &upOneLev[1], upOneLev[0]) != 0) return 0; // & cmp fails then return a length of zero
            }                                                                   // end if slen's non zero
        }                                                                       // end if keys dont equal
    }                                                                           // end else-forward order

    if (current == ST_DEPEND_NULL) return 0;                                    // nothing past our key then return length

    for (int i = 1; i <= pieces; i++) {                                         // number of keys
        int upto = 0;                                                           // clear flag

        ret = UTIL_Key_Extract(&current->bytes[index], (u_char *) keysub, &upto); // next key
        index += upto;                                                          // increment index
        if ((index >= current->keylen) && (i < pieces)) return 0;               // hit end of key & !found then return null
    }                                                                           // end for-pieces to level required

    // Now have ASCII key in desired position number, put the ASCII value of that key in *buf and return the length of it
    return (short) mcopy((u_char *) keysub, buf, ret);
}                                                                               // end function - TMP_Order

/*
 * Function: TMP_Query - return next whole key in sequence, forward or reverse
 * returns pointer to length of next key
 */
short TMP_Query(mvar *var, u_char *buf, int dir,table_struct * table)
{
    int       ptr1;                                                             // position in symtab
    ST_depend *current = ST_DEPEND_NULL;                                        // active pointer
    ST_depend *prev = ST_DEPEND_NULL;                                           // pointer to previous element
    ST_depend *lastkey = ST_DEPEND_NULL;                                        // pointer to last used key
    mvar      outputVar = *var;                                                 // copy of supplied mvar

    if (var->volset) {                                                          // if by index
        ptr1 = TMP_LocateIdx(var->volset - 1,table);                                   // get it this way
    } else {                                                                    // no volset, use var name
        ptr1 = TMP_Locate(var->name,table);                                            // locate the variable by name
    }

    buf[0] = '\0';                                                              // JIC
    if (ptr1 < 0) return 0;
    if (table->sym_tab[ptr1].data == ST_DATA_NULL) return 0;                            // no data block, err
    current = table->sym_tab[ptr1].data->deplnk;                                        // first dependent pointed at
    if (current == ST_DEPEND_NULL) return 0;                                    // not found
    lastkey = table->sym_tab[ptr1].data->last_key;                                      // pointer to last used key

    // start search at last used key, rather than at the beginning (var after lastkey)
    if ((lastkey != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, lastkey->bytes, var->slen, lastkey->keylen) > 0) && (dir == 1)) {
        current = lastkey;
    }

    // while more exist with keys that are larger, get next dep (var after current)
    while ((current != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, current->bytes, var->slen, current->keylen) > 0)) {
        prev = current;                                                         // save prev pointer
        current = current->deplnk;                                              // go to next dependent pointer
    }                                                                           // end while-compare keys

    table->sym_tab[ptr1].data->last_key = prev;                                         // add last used key

    if (var->slen > 0) {                                                        // looking in dependents
        if (dir == -1) {                                                        // reverse order
            if (prev != ST_DEPEND_NULL) current = prev;                         // only if previous ptr defined, go back one
        } else {                                                                // end if reverse order - start forward order
            // not going past non exist last and keys are equal (var equal current)
            if ((current != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, current->bytes, var->slen, current->keylen) == 0)) {
                current = current->deplnk;                                      // go to next
            }                                                                   // end if exact match
        }                                                                       // end else-forward
    }                                                                           // finished looking

    if (current == ST_DEPEND_NULL) return 0;                                    // if we have gone past end, return length of zero
    if ((dir == -1) && (var->slen == 0)) return 0;                              // reverse dir and data block, return length of zero
    outputVar.slen = current->keylen;                                           // key and length of set
    memcpy(outputVar.key, current->bytes, (int) current->keylen);               // setup mvar

    if ((current == table->sym_tab[ptr1].data->deplnk) && (prev != current) && (dir == -1) && (var->slen > 0)) { // previous is a data block
        outputVar.slen = 0;                                                     // flag is as such
    }                                                                           // end if back to a data block

    return UTIL_String_Mvar(&outputVar, buf, MAX_NUM_SUBS);                     // convert mvar and return length of key
}                                                                               // end TMP_Query

int TMP_GetAdd(mvar *var, cstring **add,table_struct * table)                                         // get local data address
{
    ST_depend *depPtr = ST_DEPEND_NULL;                                         // active pointer
    ST_depend *prev = ST_DEPEND_NULL;                                           // pointer to previous element
    int       ptr1;                                                             // position in symtab
    int       i;                                                                // generic counter

    if (var->volset) {                                                          // if by index
        ptr1 = TMP_LocateIdx(var->volset - 1,table);                                   // get it this way
    } else {                                                                    // no volset, use var name
        ptr1 = TMP_Locate(var->name,table);                                            // locate the variable by name
    }

    if ((ptr1 > TMP_MAX) || (ptr1 < -1)) {
        panic("TMP_GetAdd: Junk pointer returned from ST_LocateIdx");
    } else if (ptr1 >= 0) {                                                     // think we found it
        if (table->sym_tab[ptr1].data == ST_DATA_NULL) return -ERRM6;                   // not found

        //If we have a subscript
        if (var->slen > 0) {                                                    // go to dependents
            ST_depend *lastkey = table->sym_tab[ptr1].data->last_key;                   // pointer to last used key

            depPtr = table->sym_tab[ptr1].data->deplnk;                                 // get first dependent

            // start search at last used key, rather than at the beginning (var after lastkey)
            if ((lastkey != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, lastkey->bytes, var->slen, lastkey->keylen) > 0)) {
                depPtr = lastkey;
            }

            //Iterate linked list until w find the write data block
            while (depPtr != ST_DEPEND_NULL) {                                  // while dep ok, compare keys
                // Impl partialOd on KeyList (just memory)
                i = UTIL_Key_KeyCmp(var->key, depPtr->bytes, var->slen, depPtr->keylen);
                //passed key therefor it does not exist.
                if (i == K2_GREATER) return -ERRM6;                             // error if we passed it (var before depPtr)
                if (i == KEQUAL) break;                                         // found it (var equal depPtr)
                prev = depPtr;                                                  // save previous pointer
                depPtr = depPtr->deplnk;                                        // get next
            }                                                                   // end while - compare keys

            // never found the data
            if (depPtr == ST_DEPEND_NULL) return -ERRM6;                        // if no exist then return same
            i = (int) depPtr->keylen;                                           // get key length
            // Why must it be even?
            // This mite be for alignment of the cstring
            if (i & 1) i++;                                                     // ensure even
            *add = (cstring *) &depPtr->bytes[i];                               // send data addr as cstring
            table->sym_tab[ptr1].data->last_key = prev;                                 // add last used key
            return (*add)->len;                                                 // and return the length
        } else /* if there is no subscript*/{                                                                // data block
            *add = (cstring *) &table->sym_tab[ptr1].data->dbc;                         // setup the address
            i = table->sym_tab[ptr1].data->dbc;                                         // get dbc
            if (i == VAR_UNDEFINED) return -ERRM6;                              // dbc not defined and no int so return same
            return i;                                                           // and return the count
        }                                                                       // finished with data block
    }                                                                           // end if - symtab posi valid

    return -ERRM6;                                                              // return if failed
}                                                                               // end TMP_GetAdd

// Return next key in supplied mvar and data at buf
int TMP_QueryD(mvar *var, u_char *buf, table_struct * table)                                           // get next key and data
{
    cstring   *cdata;                                                           // temporary data access
    ST_depend *current = ST_DEPEND_NULL;                                        // active pointer
    ST_depend *prev = ST_DEPEND_NULL;                                           // pointer to previous element
    ST_depend *lastkey = ST_DEPEND_NULL;                                        // pointer to last used key
    int       ptr1;                                                             // position in symtab
    int       i;                                                                // generic counter

    if (var->volset) {                                                          // if by index
        ptr1 = TMP_LocateIdx(var->volset - 1,table);                                   // get it this way
    } else {                                                                    // no volset, use name
        ptr1 = TMP_Locate(var->name,table);                                            // locate the variable by name
    }

    buf[0] = '\0';                                                              // JIC
    if (ptr1 < 0) return -ERRM6;                                                // not found
    if (table->sym_tab[ptr1].data == ST_DATA_NULL) return -ERRM6;                       // no data, err
    current = table->sym_tab[ptr1].data->deplnk;                                        // first dependent pointed at
    if (current == ST_DEPEND_NULL) return -(ERRZ55 + ERRMLAST);                 // not found so no data below
    lastkey = table->sym_tab[ptr1].data->last_key;

    // start search at last used key, rather than at the beginning (var after lastkey)
    if ((lastkey != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, lastkey->bytes, var->slen, lastkey->keylen) > 0)) {
        current = lastkey;
    }

    // more deps exist and key compare fails - compare keys (var after current)
    while ((current != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, current->bytes, var->slen, current->keylen) > 0)) {
        prev = current;                                                         // set prev pointer
        current = current->deplnk;                                              // to next dependent pointer
    }                                                                           // end while-compare keys

    // while more deps exist and keys match exactly - (var equal current)
    if ((current != ST_DEPEND_NULL) && (UTIL_Key_KeyCmp(var->key, current->bytes, var->slen, current->keylen) == 0)) {
        prev = current;                                                         // set prev pointer
        current = current->deplnk;                                              // go to next
    }                                                                           // end if keys equal

    if (current == ST_DEPEND_NULL) return -(ERRZ55 + ERRMLAST);                 // if no more deps then out of data, error
    memcpy(var->key, current->bytes, current->keylen);                          // set up mvar
    var->slen = current->keylen;                                                // key and len
    i = (int) current->keylen;                                                  // get key length
    if ((i & 1) != 0) i++;                                                      // up it to next even boundary
    cdata = (cstring *) &current->bytes[i];                                     // convert to cstring
    table->sym_tab[ptr1].data->last_key = prev;                                         // add last used key
    return mcopy(cdata->buf, buf, cdata->len);                                  // get data into buf
}                                                                               // end TMP_QueryD

// kill all local variables except those whose names appear in var_u *keep
short TMP_KillAll(int count, var_u *keep, table_struct * table)
{
    partab.src_var.uci = UCI_IS_LOCALVAR;                                       // init UCI as LOCAL
    partab.src_var.slen = 0;                                                    // init subscript length
    partab.src_var.volset = 0;                                                  // init volume set

    for (int i = 0; i < TMP_MAX; i++) {                                          // for each entry in symbol table
        int j;                                                                  // generic counter

        if ((table->sym_tab[i].varnam.var_cu[0] == '$') || (table->sym_tab[i].varnam.var_cu[0] == '\0')) continue; // dont touch $ vars
        if (table->sym_tab[i].data == ST_DATA_NULL) continue;                           // ditto if it's undefined

        for (j = 0; j < count; j++) {                                           // scan the keep list
            if (var_equal(table->sym_tab[i].varnam, keep[j])) break;                    // if we want it then quit the loop
        }

        if (j < count) continue;                                                // continue if we want it
        VAR_COPY(partab.src_var.name, table->sym_tab[i].varnam);                        // init varnam
        TMP_Kill(&partab.src_var,table);                                               // kill it and all under
    }                                                                           // end for all in symtab

    return 0;                                                                   // finished OK
}                                                                               // end TMP_KillAll

/*
 * Locate variable 'var' - create the symtab entry if it doesn't exist.
 * Increment usage.
 * If TMP_data block does not exist, create it.
 * Return the symtab entry number or negative error number
 */
short TMP_SymAtt(var_u var,table_struct * table)
{
    short pos;

    pos = TMP_Create(var,table);                                                       // position in symtab - locate/create variable
    if (pos >= 0) table->sym_tab[pos].usage++;                                          // if ok, increment usage
    return pos;                                                                 // return whatever we found
}

/*
 * For each symtab entry in list (ignoring those that are VAR_UNDEFINED),
 * decrement usage. Remove the entry if it is undefined.
 */
void TMP_SymDet(int count, short *list,table_struct * table)
{
    for (int i = 0; i < count; i++) {                                           // for all supplied vars
        if (list[i] >= 0) {                                                     // if this got attached
            table->sym_tab[list[i]].usage--;                                            // decrement usage
            if (table->sym_tab[list[i]].usage > 0) continue;                            // still NEWed or whatever

            if (table->sym_tab[list[i]].data != ST_DATA_NULL) {                         // data?
                if (table->sym_tab[list[i]].data->dbc != VAR_UNDEFINED) continue;
                if (table->sym_tab[list[i]].data->deplnk != ST_DEPEND_NULL) continue;
                free(table->sym_tab[list[i]].data);                                     // free the data block
                table->sym_tab[list[i]].data = ST_DATA_NULL;                            // and remember this
            }

            if (table->sym_tab[list[i]].data == ST_DATA_NULL) {                         // no data?
                TMP_Free(table->sym_tab[list[i]].varnam,table);                                // not in use - free it
            }
        }
    }

    free(list);                                                                 // dump the memory
    return;
}                                                                               // end function TMP_SymDet

// get local data - symtab entry number provided
/*
int TMP_SymGet(short syment, u_char *buf)
{
    if (symtab[syment].data == ST_DATA_NULL) return -ERRM6;                     // if no data (undefined) then complain
    if (symtab[syment].data->dbc == VAR_UNDEFINED) return -ERRM6;               // complain
    return mcopy(symtab[syment].data->data, buf, symtab[syment].data->dbc);     // go to the data and copy data (if any)
}                                                                               // end function TMP_SymGet
*/

// set local data - symtab entry number provided
short TMP_SymSet(short pos, cstring *data, table_struct * table)
{
    u_int   u;                                                                  // a handy int
    ST_data *ptr;                                                               // and a pointer

    u = DTBLKSIZE + data->len;                                                  // size required
    if (u < DTMINSIZE) u = DTMINSIZE;                                           // check for minimum

    if (table->sym_tab[pos].data == ST_DATA_NULL) {                                     // if no data block
        table->sym_tab[pos].data = malloc(u);                                           // get some memory
        if (table->sym_tab[pos].data == ST_DATA_NULL) return -(ERRZ56 + ERRMLAST);      // no mem
        table->sym_tab[pos].data->last_key = ST_DEPEND_NULL;                            // init last used key
        table->sym_tab[pos].data->deplnk = ST_DEPEND_NULL;                              // init dep link
        table->sym_tab[pos].data->attach = 1;                                           // init attach count
    } else if (table->sym_tab[pos].data->dbc < data->len) {                             // enough space?
        ptr = realloc(table->sym_tab[pos].data, u);                                     // attempt to increase it
        if (ptr == ST_DATA_NULL) return -(ERRZ56 + ERRMLAST);                   // no memory available

        if ((ptr != table->sym_tab[pos].data) && (ptr->attach > 1)) {                   // did it move and many attached?
            FixData(table->sym_tab[pos].data, ptr, ptr->attach,table);                        // fix it
        } else {
            table->sym_tab[pos].data = ptr;                                             // or just do this one
        }
    }

    table->sym_tab[pos].data->dbc = data->len;                                          // set the dbc
    memcpy(&table->sym_tab[pos].data->data, &data->buf[0], data->len + 1);              // set it
    return 0;                                                                   // and exit
}                                                                               // end function TMP_SymSet

// kill a local var - symtab entry number provided
short TMP_SymKill(short pos,table_struct * table)
{
    ST_depend *dptr;                                                            // dependent ptr
    ST_depend *fptr;                                                            // dependent ptr

    if (table->sym_tab[pos].data != ST_DATA_NULL) {                                     // there is data
        dptr = table->sym_tab[pos].data->deplnk;                                        // get dependent ptr
        table->sym_tab[pos].data->last_key = ST_DEPEND_NULL;                            // clear it
        table->sym_tab[pos].data->deplnk = ST_DEPEND_NULL;                              // clear it

        while (dptr != ST_DEPEND_NULL) {                                        // for each dependent
            fptr = dptr;                                                        // save this one
            dptr = fptr->deplnk;                                                // get next
            free(fptr);                                                         // free this
        }

        if (table->sym_tab[pos].data->attach <= 1) {                                    // if no more attached
            free(table->sym_tab[pos].data);                                             // free data block
            table->sym_tab[pos].data = ST_DATA_NULL;                                    // clear the pointer
        }
    }

    if (table->sym_tab[pos].usage < 1) TMP_Free(table->sym_tab[pos].varnam,table);                     // any NEWs etc.? if no - dong it
    return 0;                                                                   // and exit
}

// 0 to TMP_MAX - 1 (i.e., ((ST_HASH + 1) * 3))
short TMP_Dump(table_struct * table)                                                             // dump entire symbol table to $IO
{
    int       j;                                                                // generic counter
    int       t;                                                                // for functions
    int       escape;
    int       string;
    int       dot;
    cstring   *cdata;                                                           // variable data gets dumped
    cstring   *ckey;                                                            // variable key data gets dumped
    u_char    dump[VAR_LEN + MAX_KEY_SIZE + MAX_NUM_SUBS + 12];                 // variable name gets dumped
    u_char    dumpk[VAR_LEN + MAX_KEY_SIZE + MAX_NUM_SUBS + 12];                // variable key name gets dumped
    ST_depend *depPtr = ST_DEPEND_NULL;                                         // active dependent ptr

    for (int i = 0; i < TMP_MAX; i++) {                                          // for each entry in symbol table
        if (table->sym_tab[i].data == ST_DATA_NULL) continue;                           // get out if nothing to dump
        if (table->sym_tab[i].varnam.var_cu[0] == '$') continue;                        // dont spit out $ vars
        VAR_COPY(partab.src_var.name, table->sym_tab[i].varnam);                        // init var name
        partab.src_var.uci = UCI_IS_LOCALVAR;                                   // init UCI as LOCAL
        partab.src_var.slen = 0;                                                // init subscript length
        partab.src_var.volset = 0;                                              // init volume set
        cdata = (cstring *) &dump[0];                                           // make it a cstring
        escape = FALSE;
        string = FALSE;
        dot = 0;

        if (table->sym_tab[i].data->dbc != VAR_UNDEFINED) {                             // valid dbc
            t = UTIL_String_Mvar(&partab.src_var, cdata->buf, MAX_NUM_SUBS);    // get var name and dump data block
            if (t < 0) return (short) t;                                        // die on error
            cdata->len = t;
DISABLE_WARN(-Warray-bounds)
            cdata->buf[cdata->len++] = '=';                                     // tack on equal sign
ENABLE_WARN
            t = SQ_Write(cdata);                                                // dump var name =
            if (t < 0) return (short) t;                                        // die on error

            for (int k = 0; k < table->sym_tab[i].data->dbc; k++) {
                if (table->sym_tab[i].data->data[k] == '.') {
                    dot++;

                    if ((dot > 1) || (k == (table->sym_tab[i].data->dbc - 1))) {
                        string = TRUE;
                        break;
                    }
                } else if ((table->sym_tab[i].data->dbc > 1) && (k == 0) && (table->sym_tab[i].data->data[k] == '0')) {
                    string = TRUE;
                    break;
                } else if ((table->sym_tab[i].data->dbc > 1) && dot &&
                  (k == (table->sym_tab[i].data->dbc - 1)) && (table->sym_tab[i].data->data[k] == '0')) {
                    string = TRUE;
                    break;
                } else if ((k == 0) && (table->sym_tab[i].data->data[k] == '-')) {
                    if (table->sym_tab[i].data->data[k + 1] != '0') continue;
                    string = TRUE;
                    break;
                } else if (!isdigit(table->sym_tab[i].data->data[k])) {
                    string = TRUE;
                    break;
                }
            }

            for (int k = 0; k < table->sym_tab[i].data->dbc; k++) {
                if (!isprint(table->sym_tab[i].data->data[k])) {
                    if (escape) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 1;
ENABLE_WARN
                        strcpy((char *) cdata->buf, ",");
                    } else if (k == 0) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 3;
ENABLE_WARN
                        strcpy((char *) cdata->buf, "$C(");
                    } else {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 5;
ENABLE_WARN
                        strcpy((char *) cdata->buf, "\"_$C(");
                    }

                    t = SQ_Write(cdata);                                        // dump data character
                    if (t < 0) return (short) t;                                // die on error
DISABLE_WARN(-Warray-bounds)
                    cdata->len = ltocstring(cdata->buf, table->sym_tab[i].data->data[k]);
ENABLE_WARN
                    t = SQ_Write(cdata);                                        // dump data character
                    if (t < 0) return (short) t;                                // die on error

                    if (k == (table->sym_tab[i].data->dbc - 1)) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 1;
ENABLE_WARN
                        strcpy((char *) cdata->buf, ")");
                        t = SQ_Write(cdata);                                    // dump data character
                        if (t < 0) return (short) t;                            // die on error
                    }

                    escape = TRUE;
                } else {
                    if (escape) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 3;
ENABLE_WARN
                        strcpy((char *) cdata->buf, ")_\"");
                        t = SQ_Write(cdata);                                    // dump data character
                        if (t < 0) return (short) t;                            // die on error
                    } else if ((string && (k == 0)) || (table->sym_tab[i].data->data[k] == '"')) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 1;
ENABLE_WARN
                        strcpy((char *) cdata->buf, "\"");
                        t = SQ_Write(cdata);                                    // dump data character
                        if (t < 0) return (short) t;                            // die on error
                    }

                    t = SQ_WriteStar(table->sym_tab[i].data->data[k]);                  // dump data character
                    if (t < 0) return (short) t;                                // die on error
                    escape = FALSE;

                    if (string && (k == (table->sym_tab[i].data->dbc - 1))) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 1;
ENABLE_WARN
                        strcpy((char *) cdata->buf, "\"");
                        t = SQ_Write(cdata);                                    // dump data character
                        if (t < 0) return (short) t;                            // die on error
                    }
                }
            }

            if (!table->sym_tab[i].data->dbc) {
DISABLE_WARN(-Warray-bounds)
                cdata->len = 2;
ENABLE_WARN
                strcpy((char *) cdata->buf, "\"\"");
                t = SQ_Write(cdata);                                            // dump data character
                if (t < 0) return (short) t;                                    // die on error
            }

            t = SQ_WriteFormat(SQ_LF);                                          // line feed
            if (t < 0) return (short) t;                                        // die on error
        }                                                                       // end if valid dbc

        cdata = NULL;                                                           // nullify the cstring
        ckey = NULL;                                                            // nullify the cstring
        depPtr = table->sym_tab[i].data->deplnk;                                        // get first dependent

        while (depPtr != ST_DEPEND_NULL) {                                      // while dependents exist
            int     paren = 0;
            u_short datalen;

            escape = FALSE;
            string = FALSE;
            dot = 0;
            VAR_COPY(partab.src_var.name, table->sym_tab[i].varnam);                    // init var name
            partab.src_var.uci = UCI_IS_LOCALVAR;                               // init UCI as LOCAL
            partab.src_var.slen = depPtr->keylen;                               // init subscript length
            partab.src_var.volset = 0;                                          // init volume set
            memcpy(partab.src_var.key, depPtr->bytes, depPtr->keylen);          // init key
            cdata = (cstring *) &dump[0];                                       // get into a cstring
            ckey = (cstring *) &dumpk[0];                                       // get into a cstring
            t = UTIL_String_Mvar(&partab.src_var, cdata->buf, MAX_NUM_SUBS);    // get var name and dump dependent block
            if (t < 0) return (short) t;                                        // die on error
            cdata->len = t;
DISABLE_WARN(-Warray-bounds)
            cdata->buf[cdata->len++] = '=';                                     // tack on an equal sign

            for (int k = 0; k < cdata->len; k++) {
                if (cdata->buf[k] == '(') paren = k;
ENABLE_WARN

DISABLE_WARN(-Warray-bounds)
                if (!isprint(cdata->buf[k])) {
                    if (escape) {
                        ckey->len = 1;
ENABLE_WARN
                        strcpy((char *) ckey->buf, ",");
                    } else {
                        if (k == (paren + 2)) {
DISABLE_WARN(-Warray-bounds)
                            ckey->len = 3;
ENABLE_WARN
                            strcpy((char *) ckey->buf, "$C(");
                        } else {
DISABLE_WARN(-Warray-bounds)
                            ckey->len = 5;
ENABLE_WARN
                            strcpy((char *) ckey->buf, "\"_$C(");
                        }
                    }

                    t = SQ_Write(ckey);                                         // dump data character
                    if (t < 0) return (short) t;                                // die on error
DISABLE_WARN(-Warray-bounds)
                    ckey->len = ltocstring(ckey->buf, cdata->buf[k]);
ENABLE_WARN
                    t = SQ_Write(ckey);                                         // dump data character
                    if (t < 0) return (short) t;                                // die on error
                    escape = TRUE;
                } else {
                    if (escape) {
DISABLE_WARN(-Warray-bounds)
                        if (k == (cdata->len - 3)) {
                            ckey->len = 1;
ENABLE_WARN
                            strcpy((char *) ckey->buf, ")");
                        } else {
DISABLE_WARN(-Warray-bounds)
                            ckey->len = 3;
ENABLE_WARN
                            strcpy((char *) ckey->buf, ")_\"");
                        }

                        t = SQ_Write(ckey);                                     // dump data character
                        if (t < 0) return (short) t;                            // die on error
                    }

DISABLE_WARN(-Warray-bounds)
                    if (((k == (paren + 1)) && (!isprint(cdata->buf[k + 1]))) ||
                      ((k == (cdata->len - 3)) && (!isprint(cdata->buf[k - 1])))) {
ENABLE_WARN
                        escape = FALSE;
                        continue;
                    }

DISABLE_WARN(-Warray-bounds)
                    t = SQ_WriteStar(cdata->buf[k]);                            // dump data character
ENABLE_WARN
                    if (t < 0) return (short) t;                                // die on error
                    escape = FALSE;
                }
            }

            j = (int) depPtr->keylen;                                           // find key length
            if ((j & 1) != 0) j++;                                              // up it to next even boudary
            memcpy(&datalen, &depPtr->bytes[j], sizeof(u_short));               // find data length

            for (int k = j + 2; k < datalen + j + 2; k++) {
                if (depPtr->bytes[k] == '.') {
                    dot++;

                    if ((dot > 1) || (k == (datalen + j + 1))) {
                        string = TRUE;
                        break;
                    }
                } else if ((datalen > 1) && (k == (j + 2)) && (depPtr->bytes[k] == '0')) {
                    string = TRUE;
                    break;
                } else if ((datalen > 1) && dot && (k == (datalen + j + 1)) && (depPtr->bytes[k] == '0')) {
                    string = TRUE;
                    break;
                } else if ((k == (j + 2)) && (depPtr->bytes[k] == '-')) {
                    if (depPtr->bytes[k + 1] != '0') continue;
                    string = TRUE;
                    break;
                } else if (!isdigit(depPtr->bytes[k])) {
                    string = TRUE;
                    break;
                }
            }

            for (int k = j + 2; k < datalen + j + 2; k++) {
                if (!isprint(depPtr->bytes[k])) {
                    if (escape) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 1;
ENABLE_WARN
                        strcpy((char *) cdata->buf, ",");
                    } else if (k == (j + 2)) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 3;
ENABLE_WARN
                        strcpy((char *) cdata->buf, "$C(");
                    } else {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 5;
ENABLE_WARN
                        strcpy((char *) cdata->buf, "\"_$C(");
                    }

                    t = SQ_Write(cdata);                                        // dump data character
                    if (t < 0) return (short) t;                                // die on error
DISABLE_WARN(-Warray-bounds)
                    cdata->len = ltocstring(cdata->buf, depPtr->bytes[k]);
ENABLE_WARN
                    t = SQ_Write(cdata);                                        // dump data character
                    if (t < 0) return (short) t;                                // die on error

                    if (k == (datalen + j + 1)) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 1;
ENABLE_WARN
                        strcpy((char *) cdata->buf, ")");
                        t = SQ_Write(cdata);                                    // dump data character
                        if (t < 0) return (short) t;                            // die on error
                    }

                    escape = TRUE;
                } else {
                    if (escape) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 3;
ENABLE_WARN
                        strcpy((char *) cdata->buf, ")_\"");
                        t = SQ_Write(cdata);                                    // dump data character
                        if (t < 0) return (short) t;                            // die on error
                    } else if ((string && (k == (j + 2))) || (depPtr->bytes[k] == '"')) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 1;
ENABLE_WARN
                        strcpy((char *) cdata->buf, "\"");
                        t = SQ_Write(cdata);                                    // dump data character
                        if (t < 0) return (short) t;                            // die on error
                    }

                    t = SQ_WriteStar(depPtr->bytes[k]);                         // dump data character
                    if (t < 0) return (short) t;                                // die on error
                    escape = FALSE;

                    if (string && (k == (datalen + j + 1))) {
DISABLE_WARN(-Warray-bounds)
                        cdata->len = 1;
ENABLE_WARN
                        strcpy((char *) cdata->buf, "\"");
                        t = SQ_Write(cdata);                                    // dump data character
                        if (t < 0) return (short) t;                            // die on error
                    }
                }
            }

            if (!datalen) {
DISABLE_WARN(-Warray-bounds)
                cdata->len = 2;
ENABLE_WARN
                strcpy((char *) cdata->buf, "\"\"");
                t = SQ_Write(cdata);                                            // dump data character
                if (t < 0) return (short) t;                                    // die on error
            }

            t = SQ_WriteFormat(SQ_LF);                                          // write a line feed
            if (t < 0) return (short) t;                                        // die on error
            depPtr = depPtr->deplnk;                                            // get next if any
        }                                                                       // end while dependents exist
    }                                                                           // end for all symtab entries

    return 0;                                                                   // finished successfully
}                                                                               // end function TMP_Dump

// copy all variables in as subscripts to specified global
short TMP_DumpV(mvar *global,table_struct * table)
{
    int       j;                                                                // generic counter
    int       t;                                                                // for functions
    short     gs;                                                               // global slen save value
    u_char    gks[255];
    cstring   *cdata;                                                           // variable data gets dumped
    u_char    dump[1024];                                                       // variable name gets dumped
    ST_depend *depPtr = ST_DEPEND_NULL;                                         // active dependent ptr

    cdata = (cstring *) dump;                                                   // make it a cstring
    partab.src_var.uci = UCI_IS_LOCALVAR;                                       // init UCI as LOCAL
    partab.src_var.volset = 0;                                                  // init volume set
    gs = global->slen;                                                          // save original sub length
    memcpy(gks, global->key, global->slen);                                     // save original key

    for (int i = 0; i < TMP_MAX; i++) {                                          // for each entry in symbol table
        if (table->sym_tab[i].data == ST_DATA_NULL) continue;                           // get out if nothing to dump
        if (table->sym_tab[i].varnam.var_cu[0] == '$') continue;                        // no $ vars
        if (var_empty(table->sym_tab[i].varnam)) continue;                              // ensure something there
        VAR_COPY(partab.src_var.name, table->sym_tab[i].varnam);                        // init var name
        partab.src_var.slen = 0;                                                // init subscript length

        if (table->sym_tab[i].data->dbc != VAR_UNDEFINED) {                             // if data exists
            t = UTIL_String_Mvar(&partab.src_var, cdata->buf, MAX_NUM_SUBS);
            if (t < 0) return (short) t;                                        // if error, quit
DISABLE_WARN(-Warray-bounds)
            cdata->len = t;
ENABLE_WARN
            memcpy(global->key, gks, gs);                                       // restore initial key
            global->slen = gs;                                                  // restore initial length
            global->slen = global->slen + UTIL_Key_Build(cdata, &global->key[gs]);

            // set rest of global key and len
            t = DB_Set(global, (cstring *) &table->sym_tab[i].data->dbc,table);               // try to set it

            // block overhead - header (20 or 44) + index (2) + chunk (2) + CCC (1) + UCC (1) + key (~34) + DBC (2) + alignment (~4)
            if (t == -ERRM75) {                                                 // if string too long
                j = table->sym_tab[i].data->dbc;                                        // save this
                table->sym_tab[i].data->dbc = 934;                                      // that should work (1024 - 90 overhead)
                //symtab[i].data->dbc = SOA(partab.vol[global->volset - 1]->vollab)->block_size - 90; // that should work
                t = DB_Set(global, (cstring *) &table->sym_tab[i].data->dbc,table);           // try again
                table->sym_tab[i].data->dbc = j;                                        // restore this
            }
        }                                                                       // end if data exists

        depPtr = table->sym_tab[i].data->deplnk;                                        // get first dependent

        while (depPtr != ST_DEPEND_NULL) {                                      // while dependents exist
            partab.src_var.slen = depPtr->keylen;                               // init subscript length
            memcpy(partab.src_var.key, depPtr->bytes, depPtr->keylen);          // init key
            cdata = (cstring *) &dump[0];                                       // get it into a cstring
            t = UTIL_String_Mvar(&partab.src_var, cdata->buf, MAX_NUM_SUBS);
            if (t < 0) return (short) t;                                        // if error, quit
DISABLE_WARN(-Warray-bounds)
            cdata->len = t;
ENABLE_WARN
            j = (int) depPtr->keylen;                                           // find key length
            if ((j & 1) != 0) j++;                                              // up it to next even boudary
            memcpy(global->key, gks, gs);                                       // restore initial key
            global->slen = gs;                                                  // restore initial length
            global->slen += UTIL_Key_Build(cdata, &global->key[gs]);

            // set up global key
            t = DB_Set(global, (cstring *) &depPtr->bytes[j],table);                  // try to set it
            if (t < 0) return (short) t;
            depPtr = depPtr->deplnk;                                            // get next if any
        }                                                                       // end while dependents exist
    }                                                                           // end for all symtab entries

    return 0;                                                                   // finished successfully
}                                                                               // end function DumpV
