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
int    DB_Get(const mvar *var, u_char *buf);                                    // get global data
int    DB_Set(const mvar *var, cstring *data);                                  // set global data
short  DB_Data(const mvar *var, u_char *buf);                                   // get $DATA
short  DB_Kill(const mvar *var);                                                // remove sub-tree
int    DB_Daemon(int slot, int vol);                                            // proto DB_Daemon
short  DB_Mount(char *file, int vol, u_int gmb, u_int rmb);                     // mount an environment
short  DB_Order(const mvar *var, u_char *buf, int dir);                         // get next subscript
short  DB_Query(mvar *var, u_char *buf, int dir);                               // get next key
short  DB_QueryD(mvar *var, u_char *buf);                                       // get next key and data
int    DB_GetLen(const mvar *var, int lock, u_char *buf);                       // return length of global
short  DB_Compress(mvar *var, int flags);                                       // on line compressor
int    DB_Free(int vol);                                                        // return total free blocks
short  DB_UCISet(int vol, int uci, var_u name);                                 // set UCI name
short  DB_UCIKill(int vol, int uci);                                            // kill UCI entry
short  DB_Expand(int vol, u_int vsiz);                                          // expand db
int    DB_Dismount(int vol);                                                    // dismount a volume
void   ClearJournal(int vol);                                                   // clear journal
void   DB_StopJournal(int vol, u_char action);                                  // stop journal
int    DB_GetFlags(const mvar *var);                                            // get flags
int    DB_SetFlags(const mvar *var, int flags);                                 // set flags
int    DB_ic(int vol, int block);                                               // integrity checker
struct GBD *DB_ViewGet(u_int vol, u_int block);                                 // return GBD address of specified block null on err
void   DB_ViewPut(u_int vol, struct GBD *ptr);                                  // queue block for write
void   DB_ViewRel(u_int vol, struct GBD *ptr);                                  // release block, GBD -> free

// Sequential IO
short SQ_Init(void);                                                            // init chan 0 etc.

// open on this channel, this file/device, in this mode, timeout (-1 = unlimited)
short SQ_Open(int chan, cstring *object, const cstring *op, int tout);

// set chan as current $IO, input terminators or NULL, output terminators or NULL, parameters see rsm/include/rsm.h
short SQ_Use(int chan, cstring *interm, const cstring *outerm, int par);
short SQ_Close(int chan);                                                       // close channel
int   SQ_Write(cstring *buf);                                                   // write to current $IO
short SQ_WriteStar(u_char c);                                                   // output one character
int   SQ_WriteFormat(int count);                                                // write format chars

// read from current $IO to buf, timeout (-1 = unlimited), maximum bytes (-1 = unlimited)
int   SQ_Read(u_char *buf, int tout, int maxbyt);

// read one character, timeout (-1 = unlimited)
short SQ_ReadStar(int *result, int timeout);
short SQ_Flush(void);                                                           // flush input on $IO
int   SQ_Device(u_char *buf);                                                   // return attributes
short SQ_Force(const cstring *device, const cstring *msg);                      // force data to a device

// Compiler
int   Compile_Routine(mvar *rou, mvar *src, u_char *stack);                     // whole routine
void  eval(void);                                                               // compiler
void  parse(void);                                                              // ditto
void  dodollar(void);                                                           // parse var, func etc.
short routine(int runtime);                                                     // parse routine ref

// Runtime utilities
int     cstringtoi(cstring *str);                                               // convert cstring to int
int     cstringtob(cstring *str);                                               // convert cstring to boolean
u_short ltocstring(u_char *buf, long n);                                        // convert long to string
u_short ultocstring(u_char *buf, u_long n);                                     // convert u_long to string
int     short_version(u_char *ret_buffer, int i);                               // return short version string
int     rsm_version(u_char *ret_buffer);                                        // return version string
int     Set_Error(int err, cstring *user, cstring *space);                      // Set $ECODE
time_t  current_time(short local);                                              // get current time with/without local offset
int     run(int asp, int ssp);                                                  // run compiled code
short   buildmvar(mvar *var, int nul_ok, int asp);                              // build an mvar
short   getvol(const cstring *vol);                                             // get vol number for vol
short   getuci(const cstring *uci, int vol);                                    // get uci number
short   patmat(cstring *str, cstring *code);                                    // pattern match
short   attention(void);                                                        // process attention
int     ForkIt(int cft);                                                        // Fork (copy file table)
void    SchedYield(u_char sleep);                                               // do a sched_yield() or a nanosleep() or nothing
void    DoInfo(void);                                                           // for <Control-T>

// Runtime math (decimal ex FreeMUMPS)
short runtime_add(char *a, char *b);                                            // add b to a
short runtime_mul(char *a, char *b);                                            // mul a by b
short runtime_div(char *uu, char *v, short typ);                                // divide string arithmetic
short runtime_power(char *a, char *b);                                          // raise a to the b-th power
short runtime_comp(char *s, char *t);                                           // compare

// Runtime functions
short Dascii1(u_char *ret_buffer, cstring *expr);
short Dascii2(u_char *ret_buffer, cstring *expr, int posn);
short Dchar(u_char *ret_buffer, int i);
short Ddata(u_char *ret_buffer, mvar *var, int update);
int   Dextract(u_char *ret_buffer, cstring *expr, int start, int stop);
int   Dfind2(u_char *ret_buffer, cstring *expr1, cstring *expr2);
int   Dfind3(u_char *ret_buffer, cstring *expr1, cstring *expr2, int start);
int   Dfind3x(cstring *expr1, cstring *expr2, int start);
int   Dfnumber2(u_char *ret_buffer, const cstring *numexp, const cstring *code);
int   Dfnumber3(u_char *ret_buffer, cstring *numexp, const cstring *code, int rnd);
int   Dget1(u_char *ret_buffer, mvar *var);
int   Dget2(u_char *ret_buffer, mvar *var, cstring *expr);
short Dincrement1(u_char *ret_buffer, mvar *var);
short Dincrement2(u_char *ret_buffer, mvar *var, cstring *numexpr);
int   Djustify2(u_char *ret_buffer, cstring *expr, int size);
int   Djustify3(u_char *ret_buffer, cstring *expr, int size, int round);
short Dlength1(u_char *ret_buffer, const cstring *expr);
short Dlength2(u_char *ret_buffer, cstring *expr, cstring *delim);
int   Dlength2x(cstring *expr, cstring *delim);
short Dname1(u_char *ret_buffer, mvar *var);
short Dname2(u_char *ret_buffer, mvar *var, int sub);
short Dorder1(u_char *ret_buffer, mvar *var);
short Dorder2(u_char *ret_buffer, mvar *var, int dir);
int   Dpiece2(u_char *ret_buffer, cstring *expr, const cstring *delim);
int   Dpiece3(u_char *ret_buffer, cstring *expr, const cstring *delim, int i1);
int   Dpiece4(u_char *ret_buffer, cstring *expr, const cstring *delim, int i1, int i2);
short Dquery1(u_char *ret_buffer, mvar *var);
short Dquery2(u_char *ret_buffer, mvar *var, int dir);
short Drandom(u_char *ret_buffer, int seed);
int   Dreverse(u_char *ret_buffer, cstring *expr);
short Dstack1(u_char *ret_buffer, int level);
short Dstack1x(u_char *ret_buffer, int level, int job);
int   Dstack2(u_char *ret_buffer, int level, cstring *code);
int   Dstack2x(u_char *ret_buffer, int level, cstring *code, int job);
int   Dtext(u_char *ret_buffer, cstring *str);
int   Dtranslate2(u_char *ret_buffer, cstring *expr1, const cstring *expr2);
int   Dtranslate3(u_char *ret_buffer, cstring *expr1, const cstring *expr2, cstring *expr3);
int   Dview(u_char *ret_buffer, int chan, int loc, int size, cstring *value); // $VIEW()
int   DSetextract(u_char *tmp, cstring *cptr, mvar *var, int i1, int i2); // Set $EXTRACT()
int   DSetpiece(u_char *tmp, cstring *cptr, mvar *var, cstring *dptr, int i1, int i2); // Set $PIECE()

// Runtime variables
int   Vecode(u_char *ret_buffer);
int   Vetrap(u_char *ret_buffer);
short Vhorolog(u_char *ret_buffer);
short Vkey(u_char *ret_buffer);
short Vreference(u_char *ret_buffer);
short Vsystem(u_char *ret_buffer);
short Vx(u_char *ret_buffer);
short Vy(u_char *ret_buffer);
int   Vset(mvar *var, cstring *cptr);                                           // set a special variable

// Symbol table
int   ST_Get(mvar *var, u_char *buf);                                           // get local data
int   ST_GetAdd(mvar *var, cstring **add);                                      // get local data address
int   ST_Set(mvar *var, const cstring *data);                                   // set local data
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
//int   ST_SymGet(short syment, u_char *buf);                                   // get using syment
short ST_SymSet(short syment, const cstring *data);                             // set using syment
short ST_SymKill(short syment);                                                 // kill var using syment
short ST_New(int count, var_u *list);                                           // new a list of vars
short ST_NewAll(int count, var_u *list);                                        // new all other than listed
short ST_ConData(const mvar *var, u_char *data);                                // connect reference to data

// SSVN
short SS_Norm(mvar *var);                                                       // "normalize" SSVN
int   SS_Get(mvar *var, u_char *buf);                                           // get SSVN data
short SS_Set(mvar *var, cstring *data);                                         // set SSVN data
short SS_Data(mvar *var, u_char *buf);                                          // get $DATA()
short SS_Kill(mvar *var);                                                       // remove sub-tree
short SS_Order(mvar *var, u_char *buf, int dir);                                // get next subscript

// Key utility
short UTIL_Key_Build(cstring *src, u_char *dest);                               // location of source string
short UTIL_Key_Extract(u_char *key, u_char *str, int *cnt);                     // extract subscript
short UTIL_String_Key(u_char *key, u_char *str, int max_subs);                  // extract all keys
short UTIL_String_Mvar(mvar *var, u_char *str, int max_subs);                   // mvar -> string
int   UTIL_Key_Last(const mvar *var);                                           // point at last subs in mvar
short UTIL_MvarFromCStr(cstring *src, mvar *var);                               // convert cstring to mvar
int   UTIL_Key_KeyCmp(const u_char *key1, const u_char *key2, int kleng1, int kleng2);
int   UTIL_Key_Cmp(const u_char *key1, const u_char *key2, int kleng1, int kleng2);
int   UTIL_Key_Chars_In_Subs(char *Key, int keylen, int maxsubs, int *subs, char *KeyBuffer);

// General utility
u_short    UTIL_strerror(int err, u_char *buf);                                 // return string error msg
int        mcopy(u_char *src, u_char *dst, int bytes);                          // memmove with checking etc.
short      ncopy(u_char **src, u_char *dst);                                    // copy as number
void       CleanJob(int job);                                                   // tidy up a job
void       panic(const char *msg);                                              // die on error
void       Routine_Init(int vol);                                               // proto for routine setup
struct RBD *Routine_Attach(var_u routine);                                      // attach to routine
void       Routine_Detach(struct RBD *pointer);                                 // detach from routine
void       Routine_Delete(var_u routine, int vol, int uci);                     // mark mapped routine deleted
void       Dump_gbd(void);                                                      // dump global descriptors
void       Dump_rbd(void);                                                      // dump routine descriptors
void       Dump_ltd(void);                                                      // dump lock descriptors

// Lock string conversion utility
short UTIL_String_Lock(locktab *var, u_char  *str);                             // convert lock entry to full environment string
short UTIL_mvartolock(mvar *var, u_char *buf);                                  // convert mvar to string

// Share and semaphore stuff
int   UTIL_Share(char *dbf);                                                    // attach share and semaphores
short SemOp(int sem_num, int numb);                                             // add/remove semaphore
short LCK_Order(const cstring *ent, u_char *buf, int dir);
short LCK_Get(const cstring *ent, u_char *buf);
short LCK_Kill(const cstring *ent);
void  LCK_Remove(int job);
short LCK_Old(int count, cstring *list, int to);
short LCK_Add(int count, cstring *list, int to);
short LCK_Sub(int count, cstring *list);

// XCalls
short Xcall_debug(char *ret_buffer, cstring *arg, cstring *dummy);
short Xcall_directory(char *ret_buffer, const cstring *file, cstring *dummy);
short Xcall_errmsg(char *ret_buffer, cstring *err, cstring *dummy);
short Xcall_opcom(char *ret_buffer, cstring *msg, const cstring *device);
short Xcall_signal(char *ret_buffer, cstring *pid, cstring *sig);
short Xcall_spawn(char *ret_buffer, cstring *cmd, const cstring *type);
short Xcall_version(char *ret_buffer, cstring *dummy1, cstring *dummy2);
short Xcall_zwrite(char *ret_buffer, cstring *tmp, cstring *dummy);
short Xcall_e(char *ret_buffer, cstring *istr, cstring *STR_mask);
short Xcall_paschk(char *ret_buffer, const cstring *user, cstring *pwd);
int   Xcall_v(char *ret_buffer, cstring *lin, cstring *col);
int   Xcall_x(char *ret_buffer, cstring *str, const cstring *flag);
short Xcall_xrsm(char *ret_buffer, cstring *str, cstring *dummy);
int   Xcall_getenv(char *ret_buffer, const cstring *env, cstring *dummy);
short Xcall_setenv(char *ret_buffer, cstring *env, const cstring *value);
short Xcall_fork(char *ret_buffer, cstring *dummy1, cstring *dummy2);
short Xcall_file(char *ret_buffer, cstring *file, cstring *attr);
short Xcall_host(char *ret_buffer, cstring *name, cstring *arg);
short Xcall_wait(char *ret_buffer, cstring *arg1, const cstring *arg2);

#endif
