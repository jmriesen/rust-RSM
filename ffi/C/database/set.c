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
#include <string.h>                                                             // for memcpy
#include <unistd.h>                                                             // for file reading
#include <ctype.h>                                                              // for GBD stuff
#include <sys/types.h>                                                          // for semaphores
#include <sys/ipc.h>                                                            // for semaphores
#include <sys/sem.h>                                                            // for semaphores
#include "rsm.h"                                                                // standard includes
#include "database.h"                                                           // database protos
#include "proto.h"                                                              // standard prototypes
#include "error.h"                                                              // error strings

/*
 *                 DATABASE INSERT DIAGRAM
 *
 *         Insert point for G2 (a single node)
 *         v
 * +-------+-------+       +---------------+
 * |  G1   |   G3  +--->---+      G4       |
 * +-------+-------+       +---------------+
 *
 *
 * Option **0** - A simple insert
 *
 * +----+-----+----+       +---------------+
 * | G1 | G2  | G3 +--->---+      G4       |
 * +----+-----+----+       +---------------+
 *
 *
 * Option **1** - New record and trailings from original in right link
 *
 * +---------------+       +-----+----+----+
 * |      G1       +--->---+  G2 | G3 | G4 |
 * +---------------+       +-----+----+----+
 *
 *
 * Option **2** - New record in original, trailings in right link
 *
 * +-------+-------+       +-------+-------+
 * |  G1   |   G2  +--->---+  G3   |   G4  |
 * +-------+-------+       +-------+-------+
 *
 *
 * Option **3** - New record in new block, trailings in right link
 *
 * +---------------+       +---------------+       +-------+-------+
 * |      G1       +--->---+      G2       +--->---+  G3   |  G4   |
 * +---------------+       +---------------+       +-------+-------+
 *
 *
 * Option **4** - New record in original, trailings in new block
 *
 * +-------+-------+       +---------------+       +---------------+
 * |  G1   |   G2  +--->---+      G3       +--->---+      G4       |
 * +-------+-------+       +---------------+       +---------------+
 *
 *
 * Option **5** - New record and trailings in one new block
 *
 * +---------------+       +-------+-------+       +---------------+
 * |      G1       +--->---+  G2   |  G3   +--->---+      G4       |
 * +---------------+       +-------+-------+       +---------------+
 *
 *
 * Option **6** - New record and trailings in a new block each
 * NOTE: Data only - not applicable to pointer blocks
 *
 * +---------------+       +---------------+       +---------------+       +---------------+
 * |      G1       +--->---+      G2       +--->---+      G3       +--->---+      G4       |
 * +---------------+       +---------------+       +---------------+       +---------------+
 */

/*
 * Function: Set_data
 * Summary:  Set the supplied data to location pointed to by db_var
 * Input(s): Pointer to the data to set
 * Return:   String length -> Ok, negative M error
 */
int Set_data(cstring *data)                                                     // set a record
{
    int      t;                                                                 // for returns
    u_int    i;                                                                 // a handy unsigned int
    u_char   tmp[VAR_LEN + 4];                                                  // spare string
    u_char   cstr[8];                                                           // and another
    u_char   fk[MAX_KEY_SIZE + 5];                                              // for keys
    cstring  *ptr;                                                              // spare pointer
    gbd      *cblk[4];                                                          // current level blocks
    int      rs;                                                                // required space
    int      ts;                                                                // trailing size
    int      rls;                                                               // RL space
    u_int    trailings;                                                         // pointer to original trail
    int      this_level;                                                        // to save level
    DB_Block *btmp;                                                             // ditto

    Get_GBDs(MAXTREEDEPTH * 2);                                                 // ensure this many
    t = Get_data(0);                                                            // try to find that

    // WARNING: Leaves GBDs reserved
    if ((t < 0) && (t != -ERRM7)) return t;                                     // check for errors and return the error

    if ((t == -ERRM7) && !level) {                                              // is global there
        level++;                                                                // no - where it goes
        t = New_block();                                                        // get a new block
        if (t < 0) return t;                                                    // if that failed then return error
        Index = IDX_START;                                                      // first one
        SOA(blk[level]->mem)->type = db_var.uci + 64;                           // data block
        SOA(blk[level]->mem)->last_idx = Index;                                 // first Index
        SOA(blk[level]->mem)->last_free = (SOA(partab.vol[volnum - 1]->vollab)->block_size >> 2) - 3; // use 2 words
        memcpy(&SOA(blk[level]->mem)->global, &db_var.name.var_cu[0], VAR_LEN); // name
        idx[Index] = SOA(blk[level]->mem)->last_free + 1;                       // the data
        chunk = (cstring *) &iidx[idx[Index]];                                  // point at it
        chunk->len = 8;                                                         // used two words
        chunk->buf[0] = 0;                                                      // CCC
        chunk->buf[1] = 0;                                                      // UCC
        record = (cstring *) &chunk->buf[chunk->buf[1] + 2];                    // setup record pointer
        record->len = 0;                                                        // no data
        level = 0;                                                              // clear level
        t = Get_data(0);                                                        // try the get again
        if ((t != -ERRM7) || level) panic("Set_data: Get_data() on non-ex global wrong!"); // must be this
        tmp[1] = 128;                                                           // start string key

        for (i = 0; i < VAR_LEN; i++) {                                         // for each char
            if (db_var.name.var_cu[i] == '\0') break;                           // check for null and break if found
            tmp[i + 2] = db_var.name.var_cu[i];                                 // copy char
        }

        i += 2;                                                                 // correct count
        tmp[i] = '\0';                                                          // null terminate
        tmp[0] = (u_char) i;                                                    // add the count
        ptr = (cstring *) cstr;                                                 // point at spare
DISABLE_WARN(-Warray-bounds)
        ptr->len = 4;                                                           // one int
ENABLE_WARN
        memcpy((u_int *) ptr->buf, &blk[level + 1]->block, sizeof(u_int));      // point the int here, get the block#
        t = Insert(tmp, ptr);                                                   // insert a node

        if (t < 0) {                                                            // if that failed
            level++;                                                            // point at new block
            Free_block(blk[level]->block);                                      // give block back
            Free_GBD(blk[level]);                                               // free GBD
            level--;                                                            // back at the dir
            if (blk[level]->dirty == (gbd *) 1) blk[level]->dirty = NULL;       // if reserved then clear it
            return t;                                                           // return the error
        }

        if (blk[level]->dirty == (gbd *) 1) {                                   // if reserved
            blk[level]->dirty = SBA(blk[level]);                                // terminate list
            blk[level + 1]->dirty = SBA(blk[level]);                            // point new here
        } else {
            blk[level + 1]->dirty = SBA(blk[level + 1]);                        // point new at self
        }

        level++;                                                                // back to new block
        idx = (u_short *) SOA(blk[level]->mem);                                 // point at the block
        iidx = (int *) SOA(blk[level]->mem);                                    // point at the block
        Index = IDX_START;                                                      // start at the start
        Queit();                                                                // queue for write
        t = -ERRM7;                                                             // new node undefined
    }                                                                           // end of create global code

    if ((SOA(partab.vol[volnum - 1]->vollab)->journal_available) &&
      (SOA(partab.vol[volnum - 1]->vollab)->journal_requested) &&
      (partab.jobtab->last_block_flags & GL_JOURNAL)) {                         // if journaling
        jrnrec jj;                                                              // jrn structure
        jj.action = JRN_SET;                                                    // doing set
        jj.uci = db_var.uci;                                                    // copy UCI
        VAR_COPY(jj.name, db_var.name);                                         // global name
        jj.slen = db_var.slen;                                                  // subs length
        memcpy(jj.key, db_var.key, jj.slen);                                    // copy key
        DoJournal(&jj, data);                                                   // and do it
    }

    if (db_var.slen == 0) {                                                     // changing top node?
        if ((partab.jobtab->last_block_flags & GL_TOP_DEFINED) == 0) {
            if (blk[0] == NULL) {                                               // was it a trylast?
                if (blk[level]->dirty == (gbd *) 1) blk[level]->dirty = NULL;   // if we reserved it then clear that
                blk[level] = NULL;                                              // clear that
                level = 0;                                                      // reset level
                systab->last_blk_used[LBU_OFF(volnum - 1)] = 0;                 // clear last
                Get_data(0);                                                    // try to find that
            }

            this_level = level;                                                 // save level
            level = 0;                                                          // point at GD
            tmp[1] = 128;                                                       // start string key

            for (i = 0; i < VAR_LEN; i++) {                                     // for each char
                if (db_var.name.var_cu[i] == '\0') break;                       // check for null and break if found
                tmp[i + 2] = db_var.name.var_cu[i];                             // copy char
            }

            i += 2;                                                             // correct count
            tmp[i] = '\0';                                                      // null terminate
            tmp[0] = (u_char) i;                                                // add the count
            t = Locate(tmp);                                                    // locate GD entry
            if (t < 0) panic("Set_data: Lost the global directory entry");
            Align_record();                                                     // align to 4 byte
            ((u_int *) record)[1] |= GL_TOP_DEFINED;                            // mark defined

            if (blk[level]->dirty == (gbd *) 1) {                               // if reserved
                blk[level]->dirty = SBA(blk[level]);                            // point at self
                Queit();                                                        // queue for write
            }

            partab.jobtab->last_block_flags |= GL_TOP_DEFINED;                  // mark top defined
            level = this_level;                                                 // restore level
        }

        t = 0;                                                                  // actually a modify
        idx = (u_short *) SOA(blk[level]->mem);                                 // point at the block
        iidx = (int *) SOA(blk[level]->mem);                                    // point at the block
        Index = IDX_START;
        chunk = (cstring *) &iidx[idx[Index]];                                  // point at the chunk
        record = (cstring *) &chunk->buf[chunk->buf[1] + 2];                    // point at the dbc
    }

    if (t < 0) {                                                                // a new node
        t = Insert(&db_var.slen, data);                                         // try it

        if (t != -(ERRZ62 + ERRMLAST)) {                                        // if it did fit
            if (t < 0) return t;                                                // exit on error

            if (blk[level]->dirty == (gbd *) 1) {                               // if reserved
                blk[level]->dirty = SBA(blk[level]);                            // point at self
                Queit();                                                        // queue for write
            }

            level--;                                                            // point up a level

            while (level >= 0) {                                                // for each
                if (blk[level] != NULL) {                                       // if one there
                    if (blk[level]->dirty == (gbd *) 1) blk[level]->dirty = NULL; // if we reserved it then clear that
                }

                level--;                                                        // previous
            }

            return data->len;                                                   // and return length
        }
    } else {                                                                    // end new node code - it's a replacement
        i = chunk->len - chunk->buf[1] - 6;                                     // available size

        if (data->len <= i) {                                                   // if it will fit
            if (data->len < record->len) SOA(blk[level]->mem)->flags |= BLOCK_DIRTY; // if new record smaller then block needs tidy
            record->len = data->len;                                            // copy length
            memcpy(record->buf, data->buf, data->len);                          // and the data

            if (blk[level]->dirty == (gbd *) 1) {                               // if reserved
                blk[level]->dirty = SBA(blk[level]);                            // point at self
                Queit();                                                        // queue for write
            }

            level--;                                                            // point up a level

            while (level >= 0) {                                                // for each
                if (blk[level] != NULL) {                                       // if one there
                    if (blk[level]->dirty == (gbd *) 1) blk[level]->dirty = NULL; // if we reserved it then clear that
                }

                level--;                                                        // previous
            }

            return data->len;                                                   // and return length
        }

        if (Index == IDX_START) {                                               // if it's 1st node
            if (blk[0] == NULL) {                                               // was it a trylast?
                if (blk[level]->dirty == (gbd *) 1) blk[level]->dirty = NULL;   // if we reserved it then clear that
                blk[level] = NULL;                                              // clear that
                level = 0;                                                      // reset level
                systab->last_blk_used[LBU_OFF(volnum - 1)] = 0;                 // clear last
                t = Get_data(0);                                                // try to find that
                if (t < 0) return -(ERRZ61 + ERRMLAST);                         // if error then database stuffed
            }
        }

        record->len = NODE_UNDEFINED;                                           // zot current data
        Tidy_block();                                                           // tidy it
        t = Insert(&db_var.slen, data);                                         // try it

        if (blk[level]->dirty == (gbd *) 1) {                                   // if reserved
            blk[level]->dirty = SBA(blk[level]);                                // point at self
            Queit();                                                            // queue for write
        }

        if (t >= 0) {                                                           // if that worked
            level--;                                                            // point up a level

            while (level >= 0) {                                                // for each
                if (blk[level] != NULL) {                                       // if one there
                    if (blk[level]->dirty == (gbd *) 1) blk[level]->dirty = NULL; // if we reserved it then clear that
                }

                level--;                                                        // previous
            }

            return data->len;                                                   // return length
        }
    }                                                                           // end simple replace (original node missing)

    if (blk[0] == NULL) {                                                       // was it a trylast?
        if (blk[level]->dirty == (gbd *) 1) blk[level]->dirty = NULL;           // if we reserved it then clear that
        blk[level] = NULL;                                                      // clear that
        level = 0;                                                              // reset level
        systab->last_blk_used[LBU_OFF(volnum - 1)] = 0;                         // clear last
        t = Get_data(0);                                                        // try to find that
        if (t != -ERRM7) return -(ERRZ61 + ERRMLAST);                           // must be undefined or database stuffed
    }

    /*
     * We get here with everything setup to do a split.
     * The block at blk[level] has been tidied, Index etc. is setup
     * If this is a replace, the original node has been deleted
     * BUT is possibly still in the pointer blocks above.
     */

    memset(rekey_blk, 0, MAXREKEY * sizeof(u_int));                             // clear that table
    memset(rekey_lvl, 0, MAXREKEY * sizeof(int));                               // and that table
    rs = 4 + db_var.slen + 2 + data->len;                                       // max required space
    if (rs & 3) rs += (4 - (rs & 3));                                           // if required then align
    rs += 4;                                                                    // allow for index
    t = Locate(&db_var.slen);                                                   // locate it again
    fk[0] = 0;                                                                  // clear for now
    ts = 0;                                                                     // trail size zot
    trailings = Index;                                                          // for ron

    if (trailings <= SOA(blk[level]->mem)->last_idx) {                          // if any point
        for (i = IDX_START; i < trailings; i++) {                               // scan front of block
            chunk = (cstring *) &iidx[idx[i]];                                  // point at chunk
            memcpy(&fk[chunk->buf[0] + 1], &chunk->buf[2], chunk->buf[1]);
        }                                                                       // get fk[] correct

        i = Index;                                                              // start here
        chunk = (cstring *) &iidx[idx[i]];                                      // point at first chunk
        ts = chunk->buf[0] + 2;                                                 // get CCC plus a bit

        while (i <= SOA(blk[level]->mem)->last_idx) {                           // loop through trailings
            chunk = (cstring *) &iidx[idx[i]];                                  // point at chunk
            ts += (chunk->len + 2);                                             // add the chunk + idx
            i++;                                                                // increment idx
        }
    }                                                                           // end ts calculate

    for (i = 0; i < 4; cblk[i++] = NULL) continue;                              // clear current level blocks
    cblk[0] = blk[level];                                                       // save this here
    blk[level] = NULL;                                                          // and zot it
    rls = 0;                                                                    // no RL yet
    i = SOA(cblk[0]->mem)->right_ptr;                                           // get RL

    if (i) {                                                                    // if there is one
        t = Get_block(i);                                                       // get it
        if (t < 0) return t;
        cblk[3] = blk[level];                                                   // remember RL here
        if (SOA(blk[level]->mem)->flags & BLOCK_DIRTY) Tidy_block();            // check state and ensure tidy if needed
        rls = (SOA(blk[level]->mem)->last_free * 2 + 1 - SOA(blk[level]->mem)->last_idx) * 2;
    }

    this_level = level;                                                         // save level

    if ((ts < rls) && ts) {                                                     // if trailings -> RL
        Un_key();                                                               // un key RL
        Get_GBD();                                                              // get another
        memset(SOM(blk[level]->mem), 0, SOA(partab.vol[volnum - 1]->vollab)->block_size); // zot
        SOA(blk[level]->mem)->type = SOA(cblk[3]->mem)->type;                   // copy type
        SOA(blk[level]->mem)->right_ptr = SOA(cblk[3]->mem)->right_ptr;         // copy RL
        VAR_COPY(SOA(blk[level]->mem)->global, SOA(cblk[3]->mem)->global);      // copy global name
        SOA(blk[level]->mem)->last_idx = IDX_START - 1;                         // unused block
        SOA(blk[level]->mem)->last_free = (SOA(partab.vol[volnum - 1]->vollab)->block_size >> 2) - 1; // set this up
        keybuf[0] = 0;                                                          // clear this

        if (((ts + rs) < rls) && (trailings != IDX_START)) {                    // if new record fits
            t = Insert(&db_var.slen, data);                                     // insert it
            if (t < 0) panic("Set_data: Insert in new block (RL) failed");      // failed ?
            memcpy(keybuf, &chunk->buf[1], chunk->buf[1] + 1);                  // save key
        }

        Copy_data(cblk[0], trailings);                                          // copy trailings
        Copy_data(cblk[3], IDX_START);                                          // and old RL
        btmp = SOA(blk[level]->mem);                                            // save this
        blk[level]->mem = cblk[3]->mem;                                         // copy in this
        cblk[3]->mem = SBA(btmp);                                               // end swap 'mem'
        Free_GBD(blk[level]);                                                   // give it back
        blk[level] = cblk[0];                                                   // original block again
        idx = (u_short *) SOA(blk[level]->mem);                                 // point at it
        iidx = (int *) SOA(blk[level]->mem);                                    // point at it

        for (i = trailings; i <= SOA(blk[level]->mem)->last_idx; i++) {
            chunk = (cstring *) &iidx[idx[i]];                                  // point at the chunk
            record = (cstring *) &chunk->buf[chunk->buf[1] + 2];                // point at the dbc
            record->len = NODE_UNDEFINED;                                       // junk it
        }

        Tidy_block();                                                           // tidy it
        if (((ts + rs) < rls) && (trailings != IDX_START)) goto fix_keys;       // if new record done then exit **1**
        t = Insert(&db_var.slen, data);                                         // attempt to insert

        if (t >= 0) {                                                           // if OK
            goto fix_keys;                                                      // exit **2**
        } else if (t != -(ERRZ62 + ERRMLAST)) {
            return t;                                                           // error
        }

        if (trailings == IDX_START) return -(ERRZ61 + ERRMLAST);                // if was first node so stuffed
        t = New_block();                                                        // new block for insert
        if (t < 0) panic("Set_data: Failed to get new block for insert");       // if failed
        SOA(blk[level]->mem)->type = SOA(cblk[0]->mem)->type;                   // copy type
        SOA(blk[level]->mem)->right_ptr = SOA(cblk[0]->mem)->right_ptr;         // copy RL
        VAR_COPY(SOA(blk[level]->mem)->global, SOA(cblk[0]->mem)->global);      // copy global name
        SOA(blk[level]->mem)->last_idx = IDX_START - 1;                         // unused block
        SOA(blk[level]->mem)->last_free = (SOA(partab.vol[volnum - 1]->vollab)->block_size >> 2) - 1; // set this up
        keybuf[0] = 0;                                                          // clear this
        SOA(cblk[0]->mem)->right_ptr = blk[level]->block;                       // point at it
        t = Insert(&db_var.slen, data);                                         // insert it
        if (t < 0) panic("Set_data: Insert in new block (insert) failed");      // failed ?
        cblk[1] = blk[level];                                                   // remember this
        goto fix_keys;                                                          // exit **3**
    }                                                                           // end trailings in RL

    if (!ts && (rs < rls)) {                                                    // no trail, will fit
        Un_key();
        t = Insert(&db_var.slen, data);                                         // attempt to insert
        if (t >= 0) goto fix_keys;                                              // if OK then exit **2**
    } else if (cblk[3] != NULL) {                                               // if RL allocated
        if (cblk[3]->dirty == (gbd *) 1) cblk[3]->dirty = NULL;                 // if reserved then clear it
        cblk[3] = NULL;                                                         // flag not used
    }

    t = New_block();                                                            // new blk for trail
    if (t < 0) panic("Set_data: Failed to get new block for trailings");        // if failed
    SOA(blk[level]->mem)->type = SOA(cblk[0]->mem)->type;                       // copy type
    SOA(blk[level]->mem)->right_ptr = SOA(cblk[0]->mem)->right_ptr;             // copy RL
    VAR_COPY(SOA(blk[level]->mem)->global, SOA(cblk[0]->mem)->global);          // copy global name
    SOA(blk[level]->mem)->last_idx = IDX_START - 1;                             // unused block
    SOA(blk[level]->mem)->last_free = (SOA(partab.vol[volnum - 1]->vollab)->block_size >> 2) - 1; // set this up
    keybuf[0] = 0;                                                              // clear this
    SOA(cblk[0]->mem)->right_ptr = blk[level]->block;                           // point at it
    Copy_data(cblk[0], trailings);                                              // copy trailings
    cblk[2] = blk[level];                                                       // save this one
    blk[level] = cblk[0];                                                       // original block again
    idx = (u_short *) SOA(blk[level]->mem);                                     // point at it
    iidx = (int *) SOA(blk[level]->mem);                                        // point at it

    for (i = trailings; i <= SOA(blk[level]->mem)->last_idx; i++) {
        chunk = (cstring *) &iidx[idx[i]];                                      // point at the chunk
        record = (cstring *) &chunk->buf[chunk->buf[1] + 2];                    // point at the dbc
        record->len = NODE_UNDEFINED;                                           // junk it
    }

    Tidy_block();                                                               // tidy it
    t = Insert(&db_var.slen, data);                                             // attempt to insert

    if (t >= 0) {                                                               // if OK
        goto fix_keys;                                                          // exit **4**
    } else if (t != -(ERRZ62 + ERRMLAST)) {
        return t;                                                               // error!
    }

    if (trailings == IDX_START) return -(ERRZ61 + ERRMLAST);                    // if was first node so stuffed
    blk[level] = cblk[2];                                                       // new block again
    t = Insert(&db_var.slen, data);                                             // attempt to insert

    if (t >= 0) {                                                               // if OK
        goto fix_keys;                                                          // exit **5**
    } else if (t != -(ERRZ62 + ERRMLAST)) {
        return t;                                                               // error!
    }

    t = New_block();                                                            // new block for insert
    if (t < 0) panic("Set_data: Failed to get new block for insert");           // if failed
    SOA(blk[level]->mem)->type = SOA(cblk[0]->mem)->type;                       // copy type
    SOA(blk[level]->mem)->right_ptr = SOA(cblk[0]->mem)->right_ptr;             // copy RL
    VAR_COPY(SOA(blk[level]->mem)->global, SOA(cblk[0]->mem)->global);          // copy global name
    SOA(blk[level]->mem)->last_idx = IDX_START - 1;                             // unused block
    SOA(blk[level]->mem)->last_free = (SOA(partab.vol[volnum - 1]->vollab)->block_size >> 2) - 1; // set this up
    keybuf[0] = 0;                                                              // clear this
    SOA(cblk[0]->mem)->right_ptr = blk[level]->block;                           // point at it
    cblk[1] = blk[level];                                                       // remember it
    t = Insert(&db_var.slen, data);                                             // insert it
    if (t < 0) panic("Set_data: Insert of new in new failed (opt=6)");          // failed? then exit **6**

fix_keys:
    blk[level] = NULL;                                                          // clear this

    for (int j = level - 1; j >= 0; j--) {                                      // scan pointer blocks
        if (blk[j]->dirty == (gbd *) 2) {                                       // if changed
            if (blk[level] == NULL) {                                           // list empty
                blk[j]->dirty = SBA(blk[j]);                                    // point at self
            } else {
                blk[j]->dirty = SBA(blk[level]);                                // else point at previous
            }

            blk[level] = blk[j];                                                // remember this one
        } else if (blk[j]->dirty == (gbd *) 1) {                                // if reserved
            blk[j]->dirty = NULL;                                               // clear it
        }
    }

    for (i = 0; i < 4; i++) {                                                   // scan cblk[]
        if (cblk[i] == NULL) continue;                                          // if empty then ignore it

        if (cblk[i]->dirty == (gbd *) 1) {                                      // not queued
            if (blk[level] == NULL) {                                           // list empty
                cblk[i]->dirty = SBA(cblk[i]);                                  // point at self
            } else {
                cblk[i]->dirty = SBA(blk[level]);                               // else point at previous
            }

            blk[level] = cblk[i];                                               // remember this one
        }
    }

    if (blk[level] != NULL) Queit();                                            // if something there then queue that lot

    for (i = 1; i < 4; i++) {                                                   // scan cblk[] again
        if (cblk[i] != NULL) Add_rekey(cblk[i]->block, this_level);             // if there then queue a fix
    }                                                                           // end fix key loop

    return Re_key();                                                            // re-key and return
}
