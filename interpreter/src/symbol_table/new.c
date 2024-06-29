/*
 * Package: Reference Standard M
 * File:    rsm/symbol/new.c
 * Summary: module symbol - symbol table new'ing and un-new'ing utilities
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
#include <unistd.h>
#include "rsm.h"                                                                // standard includes
#include "symbol.h"                                                             // our definitions
#include "error.h"                                                              // errors
#include "init.h"                                                               // init prototypes
#include "proto.h"                                                              // standard prototypes

/*
 * Function: TMP_New(int count, var_u *list) - new one or more vars
 * Returns : 0 on success or -'ve error
 */
short TMP_New(int count, var_u *list,table_struct*table)
{
    ST_newtab *newtab;                                                          // our new table

    newtab = malloc(sizeof(ST_newtab) + (count * sizeof(ST_locdata)));          // try to get enough memory
    if (newtab == NULL) return -(ERRZ56 + ERRMLAST);                            // no memory available
    newtab->fwd_link = (ST_newtab *) partab.jobtab->dostk[partab.jobtab->cur_do].newtab; // setup for link in
    newtab->count_enn = 0;                                                      // not applicable
    newtab->stindex = NULL;                                                     // not needed
    newtab->count_new = count;                                                  // how many we are to new
    newtab->locdata = (ST_locdata *) (((u_char *) &newtab->locdata) + sizeof(ST_locdata *)); // point at next free address

    for (int i = (count - 1); i >= 0; i--) {                                    // for all vars in list
        short s;

        s = TMP_SymAtt(list[i],table);                                                 // attach to variable

        if (s < 0) {                                                            // check for error
            free(newtab);                                                       // free memory
            return s;
        }

        newtab->locdata[i].stindex = s;                                         // save the index
        newtab->locdata[i].data = table->sym_tab[s].data;                               // and the data address
        table->sym_tab[s].data = ST_DATA_NULL;                                          // remove data link
    }

    partab.jobtab->dostk[partab.jobtab->cur_do].newtab = (u_char *) newtab;     // link it to the do stack
    return 0;                                                                   // finished OK
}                                                                               // end function TMP_New

/*
 * Function: TMP_NewAll(int count, var_u *list) - new all vars except listed
 * Returns : 0 on success, or -'ve error
 */
short TMP_NewAll(int count, var_u *list,table_struct * table)
{
    int       j;                                                                // generic counter
    int       new = 0;                                                          // to be new'd flag
    int       cntnew = 0;                                                       // new count
    int       cntnon = 0;                                                       // non new count
    ST_newtab *newtab;                                                          // pointer to the new table

    for (int k = 0; k < count; k++) TMP_Create(list[k],table);                         // for all supplied vars, create if not existent

    for (int i = 0; i < TMP_MAX; i++) {                                          // for each entry in ST
        if (table->sym_tab[i].varnam.var_cu[0] == '$') continue;                        // ignore $ vars
        if (table->sym_tab[i].varnam.var_cu[0] == '\0') continue;                       // ignore unused

        if (count > 0) {                                                        // if there are vars to keep
            for (j = 0; j < count; j++) {                                       // for all keep vars
                new = 1;                                                        // init delete flag

                if (var_equal(table->sym_tab[i].varnam, list[j])) {
                    new = 0;                                                    // don't new it
                    break;
                }
            }                                                                   // if var is another non new

            if (new == 1) {                                                     // if new flag set
                cntnew += 1;                                                    // incr num new'd vars
            } else {                                                            // setup done for var, don't new, add to enn
                cntnon += 1;                                                    // incr num non new'd vars
            }                                                                   // end else add to enn
        } else {                                                                // end if vars to not new, no vars to keep
            cntnew += 1;                                                        // incr count of new'd vars
        }                                                                       // end else new everything
    }                                                                           // end for all in symtab

    newtab = malloc(sizeof(ST_newtab) + (cntnew * sizeof(ST_locdata)) + (cntnon * sizeof(short))); // try allocate some memory
    if (newtab == NULL) return -(ERRZ56 + ERRMLAST);                            // no memory available
    newtab->fwd_link = (ST_newtab *) partab.jobtab->dostk[partab.jobtab->cur_do].newtab; // setup for link in
    newtab->count_enn = cntnon;                                                 // existing non new count
    newtab->count_new = 0;                                                      // num vars new'd
    newtab->stindex = (short *) (((u_char *) &newtab->locdata) + sizeof(ST_locdata *));
    newtab->locdata = (ST_locdata *) (((u_char *) &newtab->locdata) + sizeof(ST_locdata *) + (cntnon * sizeof(short)));

    for (int i = 0; i < TMP_MAX; i++) {                                          // for each entry in ST
        if (table->sym_tab[i].varnam.var_cu[0] == '$') continue;                        // ignore $ vars, so go to next one
        if (table->sym_tab[i].varnam.var_cu[0] == '\0') continue;                       // ignore unused

        if (count > 0) {                                                        // if there are vars to keep
            for (j = 0; j < count; j++) {                                       // for all keep vars
                new = 1;                                                        // init delete flag

                if (var_equal(table->sym_tab[i].varnam, list[j])) {
                    new = 0;                                                    // don't new it
                    break;
                }
            }                                                                   // if var is another non new

            if (new == 1) {                                                     // if new flag set
                newtab->locdata[newtab->count_new].stindex = i;                 // create index entry

                //point at current data
                newtab->locdata[newtab->count_new].data = table->sym_tab[newtab->locdata[newtab->count_new].stindex].data;
                table->sym_tab[newtab->locdata[newtab->count_new].stindex].data = ST_DATA_NULL; // wipe out current data link
                table->sym_tab[newtab->locdata[newtab->count_new].stindex].usage++;
                newtab->count_new++;                                            // incr num new'd vars & usage
            } else {                                                            // setup done for var, don't new, add to enn
                newtab->stindex[j] = i;                                         // set pos to symtab index
            }                                                                   // end else add to enn
        } else {                                                                // end if vars to not new, no vars to keep
            newtab->locdata[newtab->count_new].stindex = i;                     // create index entry

            // point at current data
            newtab->locdata[newtab->count_new].data = table->sym_tab[newtab->locdata[newtab->count_new].stindex].data;
            table->sym_tab[newtab->locdata[newtab->count_new].stindex].data = ST_DATA_NULL; // wipe out current data link
            table->sym_tab[newtab->locdata[newtab->count_new].stindex].usage++;
            newtab->count_new++;                                                // incr count of new'd vars
        }                                                                       // end else new everything
    }                                                                           // end for all in symtab

    partab.jobtab->dostk[partab.jobtab->cur_do].newtab = (u_char *) newtab;     // link it off partab
    return 0;                                                                   // finished OK
}                                                                               // end TMP_NewAll

/*
 * Function: TMP_Restore(ST_newtab *) - restore vars in newtab and its links
 * Returns : nothing
 */
void TMP_Restore(ST_newtab *newtab,table_struct * table)
{
    ST_newtab *ptr;                                                             // ptr-> current newtab
    ST_depend *dd;                                                              // depend data ptr
    ST_depend *ddf;                                                             // depend data ptr

    ptr = newtab;                                                               // go to first newtab
    if (ptr == NULL) return;                                                    // nothing to do

    if (ptr->stindex != NULL) {                                                 // check for newall
        for (int t = 0; t < ST_HASH; t++) {                                     // for all hash entries
            if (table->st_hash_temp[t] != -1) {                                             // only those defined
                int chk = table->st_hash_temp[t];                                           // get symtab link

                while (chk != -1) {                                             // while fwdlinks exist
                    int kill = chk;                                             // init kill flag

                    if (table->sym_tab[chk].varnam.var_cu[0] == '$') {
                        kill = -1;                                              // leave $...
                    } else {
                        for (int i = 0; i < ptr->count_enn; i++) {              // for all enn vars
                            if (var_equal(table->sym_tab[chk].varnam, table->sym_tab[ptr->stindex[i]].varnam)) { // if an ENN var
                                kill = -1;                                      // DONT KILL
                                break;                                          // and exit for
                            }
                        }                                                       // all enn vars checked
                    }

                    chk = table->sym_tab[chk].fwd_link;                                 // get next fwd link
                    if (kill > -1) TMP_SymKill(kill,table);                            // if ok to kill then kill by index
                }                                                               // end if end of fwd's
            }                                                                   // end if no hash link
        }                                                                       // end for all hash lnk
    }                                                                           // all enn vars done

    for (int i = 0; i < ptr->count_new; i++) {                                  // for all new'd vars
        if (table->sym_tab[ptr->locdata[i].stindex].data != ST_DATA_NULL) {             // if we have data blk
            table->sym_tab[ptr->locdata[i].stindex].data->attach--;                     // decrement attach

            if (table->sym_tab[ptr->locdata[i].stindex].data->attach < 1) {             // all gone?
                dd = table->sym_tab[ptr->locdata[i].stindex].data->deplnk;              // get dependents

                while (dd != ST_DEPEND_NULL) {
                    ddf = dd;                                                   // save a copy
                    dd = dd->deplnk;                                            // get next
                    free(ddf);                                                  // free this one
                }

                free(table->sym_tab[ptr->locdata[i].stindex].data);                     // free data
                table->sym_tab[ptr->locdata[i].stindex].data = ST_DATA_NULL;            // and remember
            }
        }

        table->sym_tab[ptr->locdata[i].stindex].data = ptr->locdata[i].data;            // old data
        table->sym_tab[ptr->locdata[i].stindex].usage--;                                // decrement usage

        if (table->sym_tab[ptr->locdata[i].stindex].data != ST_DATA_NULL) {             // any data?
            if ((table->sym_tab[ptr->locdata[i].stindex].data->deplnk == ST_DEPEND_NULL) &&
              (table->sym_tab[ptr->locdata[i].stindex].data->attach <= 1) &&
              (table->sym_tab[ptr->locdata[i].stindex].data->dbc == VAR_UNDEFINED)) {
                free(table->sym_tab[ptr->locdata[i].stindex].data);                     // free data memory
                table->sym_tab[ptr->locdata[i].stindex].data = ST_DATA_NULL;            // clear ptr
            }
        }

        if ((table->sym_tab[ptr->locdata[i].stindex].usage < 1) &&                      // can we dong it?
          (table->sym_tab[ptr->locdata[i].stindex].data == ST_DATA_NULL)) {             // any data?
            TMP_SymKill(ptr->locdata[i].stindex,table);                                // dong it
        }
    }                                                                           // all new'd vars done

    if (ptr->fwd_link != NULL) TMP_Restore(ptr->fwd_link, table);                       // if there are more then restore next newtab
    free(ptr);                                                                  // free the space

    if (ptr == (ST_newtab *) partab.jobtab->dostk[partab.jobtab->cur_do].newtab) {
        partab.jobtab->dostk[partab.jobtab->cur_do].newtab = NULL;              // clear doframe
    }
}                                                                               // end function Restore

/*
 * Function: TMP_ConData(mvar *, ST_data *) - connect reference to data ptr
 * Returns : 0 on success, or -'ve error
 */
short TMP_ConData(const mvar *var, u_char *data,table_struct * table)
{
    short cnct;                                                                 // connector var loc

    cnct = TMP_LocateIdx(var->volset - 1,table);                                       // find connecting var
    if (cnct < 0) return -ERRM6;                                                // if no exist, quit
    table->sym_tab[cnct].data = (ST_data *) data;                                       // lnk cnct var to src
    table->sym_tab[cnct].data->attach++;                                                // incr src attach cnt
    return 0;                                                                   // finished OK
}                                                                               // end TMP_ConRef
