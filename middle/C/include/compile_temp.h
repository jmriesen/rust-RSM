/*
  Copy that I will use to expose the c functions I am oxidizing. 
 */

#ifndef _RSM_COMPILE_TEMP_H_                                                         // only do this once
#define _RSM_COMPILE_TEMP_H_

// Compile only prototypes follow
void  parse_close_temp(u_char **src,u_char **comp);                                                        // CLOSE
void  parse_do_temp(int runtime);                                                    // DO
void  parse_goto_temp(int runtime);                                                  // GOTO
void  parse_hang_temp(u_char **src,u_char **comp);                                                         // HANG
void  parse_if_temp(long i);                                                         // IF
void  parse_job_temp(int runtime);                                                   // JOB
void  parse_kill_temp(u_char **src,u_char **comp);                                                         // KILL
void  parse_lock_temp(u_char **src,u_char **comp);                                                         // LOCK
void  parse_merge_temp(u_char **src,u_char **comp);                                                        // MERGE
void  parse_new_temp(u_char **src,u_char **comp);                                                          // NEW
void  parse_open_temp(u_char **src,u_char **comp);                                                         // OPEN
void  parse_read_temp(u_char **src,u_char **comp);                                                         // READ
void  parse_set_temp(u_char **src,u_char **comp);                                                          // SET
void  parse_use_temp(u_char **src,u_char **comp);                                                          // USE
void  parse_write_temp(u_char **src,u_char **comp);                                                        // WRITE
void  parse_xecute_temp(u_char **src,u_char **comp);                                                       // XECUTE
void  parse_temp(u_char **src,u_char **comp);                                                              // parse - main loop
void  eval_temp(u_char **src,u_char **comp,partab_struct *partab_ptr);                                                               // eval a string
void  atom_temp(u_char **src,u_char **comp,partab_struct *partab_ptr);                                                               // evaluate source

void comperror_temp(u_char **src,u_char **comp,partab_struct *partab_ptr,short err);                                                       // compile error

// Debug prototypes

/*
#ifdef __NetBSD__
void  Debug_GBD_temp(short e);
#endif
*/
void  Debug_off_temp(void);                                                          // turn off debugging
short Debug_on_temp(cstring *param);                                                 // turn on/modify debug
short Debug_temp(int savasp, int savssp, int dot);                                   // drop into debug

#endif                                                                          // !_RSM_COMPILE_TEMP_H_
