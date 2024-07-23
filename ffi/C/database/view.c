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
#include <string.h>                                                             // for memset
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
 * Function: DB_ViewGet
 * Summary:  Return GBD address of specified block, null on error
 * Input(s): Vol# and Block# to get
 * Return:   Address of GBD or null on error
 */
gbd *DB_ViewGet(u_int vol, u_int block)                                         // return GBD for blk
{
    short s;                                                                    // for func

    if ((vol < 1) || (vol > MAX_VOL) || systab->vol[vol - 1] == NULL) return NULL; // validate volume
    if ((block < 1) || (block > SOA(partab.vol[vol - 1]->vollab)->max_block)) return NULL; // validate block
    level = 0;                                                                  // where it goes
    volnum = vol;                                                               // need this
    writing = 0;                                                                // clear this
    s = SemOp(SEM_GLOBAL, SEM_READ);                                            // write lock
    if (s < 0) return NULL;                                                     // check error then quit if so
    s = Get_block(block);                                                       // get it
    if (s >= 0) blk[level]->last_accessed = current_time(TRUE) + 86400;         // push last access
    if (curr_lock) SemOp(SEM_GLOBAL, -curr_lock);                               // unlock the globals
    return ((s < 0) ? NULL : blk[level]);                                       // return whatever
}

/*
 * Function: DB_ViewPut
 * Summary:  Queue a block for write
 * Input(s): Vol# and GBD ptr of block
 * Return:   None
 */
void DB_ViewPut(u_int vol, gbd *ptr)                                            // queue block for write
{
    short s;                                                                    // for funcs

    volnum = vol;                                                               // for ron
    writing = 0;                                                                // clear this
    s = SemOp(SEM_GLOBAL, SEM_WRITE);                                           // write lock
    if (s < 0) return;                                                          // check error then quit if so
    ptr->last_accessed = current_time(TRUE);                                    // reset access

    if (SOA(ptr->mem)->type) {                                                  // if used
        Used_block(ptr->block);                                                 // mark it so
    } else {                                                                    // trying to free it
        Free_block(ptr->block);                                                 // do so
        memset(SOM(ptr->mem), 0, SOA(partab.vol[vol - 1]->vollab)->block_size); // clear it
    }

    level = 0;                                                                  // for Queit

    if (ptr->dirty == NULL) {                                                   // check dirty ptr
        ptr->dirty = SBA(ptr);                                                  // set if required
        blk[level] = ptr;                                                       // ditto
        Queit();                                                                // do this
    }

    if (curr_lock) SemOp(SEM_GLOBAL, -curr_lock);                               // release lock
    return;                                                                     // and exit
}

/*
 * Function: DB_ViewRel
 * Summary:  Release specified GBD
 * Input(s): Vol# and GBD ptr of block
 * Return:   None
 */
void DB_ViewRel(u_int vol, gbd *ptr)                                            // release block, GBD
{
    writing = 0;                                                                // clear this
    volnum = vol;                                                               // need this
    ptr->last_accessed = current_time(TRUE);                                    // reset access

    if (ptr->dirty != NULL) {                                                   // not owned elsewhere
        short s;

        s = SemOp(SEM_GLOBAL, SEM_WRITE);                                       // write lock

        if (s < 0) {                                                            // check error
            return;                                                             // quit if so
            // PROBABLY SHOULD PERSIST HERE
        }

        Free_GBD(ptr);                                                          // free it
        SemOp(SEM_GLOBAL, -curr_lock);                                          // release lock
    }

    return;                                                                     // and exit
}
