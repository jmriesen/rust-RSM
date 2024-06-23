/*
 * Package: Reference Standard M
 * File:    rsm/include/compile.h
 * Summary: module RSM header file - routine structures etc.
 *
 * David Wicksell <dlw@linux.com>
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Based on MUMPS V1 by Raymond Douglas Newman
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
 * SPDX-FileCopyrightText:  © 2020 David Wicksell <dlw@linux.com>
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

#ifndef RSM_COMPILE_H
#define RSM_COMPILE_H

typedef struct __attribute__ ((__packed__)) RBD {                               // define routine buf desciptor
    struct RBD *fwd_link;                                                       // forward link this hash
    u_int      chunk_size;                                                      // bytes in this chunk
    u_int      attached;                                                        // processes attached
    time_t     last_access;                                                     // last used (sec since 1970)
    var_u      rnam;                                                            // routine name
    u_char     uci;                                                             // UCI num for this rou
    u_char     vol;                                                             // vol num for this rou
    u_short    rou_size;                                                        // rou->len of routine node
    // what follows is the routine from disk (up to MAX_STR_LEN bytes + a NULL)
    u_short    comp_ver;                                                        // compiler version
    u_short    comp_user;                                                       // compiled by user#
    int        comp_date;                                                       // date compiled (M form)
    int        comp_time;                                                       // time compiled (M form)
    u_short    tag_tbl;                                                         // offset to tag table
    u_short    num_tags;                                                        // number of tags in table
    u_short    var_tbl;                                                         // offset to var table
    u_short    num_vars;                                                        // number of vars in table
    u_short    code;                                                            // offset to compiled code
    u_short    code_size;                                                       // bytes of code
} rbd;                                                                          // end RBD struct

#endif
