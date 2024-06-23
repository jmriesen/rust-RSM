/*
 * Package: Reference Standard M
 * File:    rsm/include/proto.h
 * Summary: module RSM header file - prototypes
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

#ifndef RSM_PROTO_H
#define RSM_PROTO_H

// Database
int    DB_Set(mvar *var, cstring *data);                                        // set global data


// set chan as current $IO, input terminators or NULL, output terminators or NULL, parameters see rsm/include/rsm.h
int   SQ_Write(cstring *buf);                                                   // write to current $IO
short SQ_WriteStar(u_char c);                                                   // output one character
int   SQ_WriteFormat(int count);                                                // write format chars

// Runtime utilities
u_short ltocstring(u_char *buf, long n);                                        // convert long to string

// Symbol table
int   ST_Get(mvar *var, u_char *buf);                                           // get local data
int   ST_GetAdd(mvar *var, cstring **add);                                      // get local data address
int   ST_Set(mvar *var, cstring *data);                                         // set local data
short ST_Data(mvar *var, u_char *buf);                                          // get $DATA()
short ST_Kill(mvar *var);                                                       // remove sub-tree
short ST_KillAll(int count, var_u *keep);                                       // kill all except spec in keep
short ST_Order(mvar *var, u_char *buf, int dir);                                // get next subscript
short ST_Query(mvar *var, u_char *buf, int dir);                                // get next key
int   ST_QueryD(mvar *var, u_char *buf);                                        // get next key and data
short ST_Dump(void);                                                            // dump the symbol table
short ST_DumpV(mvar *global);                                                   // dump symtab vars as subs
short ST_SymAtt(var_u var);                                                     // attach to variable
void  ST_SymDet(int count, short *list);                                        // detach from variables
//int   ST_SymGet(short syment, u_char *buf);                                     // get using syment
short ST_SymSet(short syment, cstring *data);                                   // set using syment
short ST_SymKill(short syment);                                                 // kill var using syment
short ST_New(int count, var_u *list);                                           // new a list of vars
short ST_NewAll(int count, var_u *list);                                        // new all other than listed
short ST_ConData(const mvar *var, u_char *data);                                // connect reference to data

// Key utility
short UTIL_Key_Build(cstring *src, u_char *dest);                               // locn of source string
short UTIL_Key_Extract(u_char *key, u_char *str, int *cnt);                     // extract subscript
short UTIL_String_Key(u_char *key, u_char *str, int max_subs);                  // extract all keys
short UTIL_String_Mvar(mvar *var, u_char *str, int max_subs);                   // mvar -> string
int   UTIL_Key_Last(mvar *var);                                                 // point at last subs in mvar
short UTIL_MvarFromCStr(cstring *src, mvar *var);                               // cvt cstring to mvar
int   UTIL_Key_KeyCmp(u_char *key1, u_char *key2, int kleng1, int kleng2);
int   UTIL_Key_Chars_In_Subs(char *Key, int keylen, int maxsubs, int *subs, char *KeyBuffer);

// General utility
int        mcopy(u_char *src, u_char *dst, int bytes);                          // memmove with checking etc.
void       panic(char *msg);                                                    // die on error

#endif
