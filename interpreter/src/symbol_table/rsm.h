/*
 * Package: Reference Standard M
 * File:    rsm/include/rsm.h
 * Summary: module RSM header file - standard includes
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

#ifndef RSM_RSM_H
#define RSM_RSM_H

// General constant definitions
#define RSM_STRING(num)    RSM_STRINGIFY(num)
#define RSM_STRINGIFY(num) #num

#define FALSE 0                                                                 // nicer than using 0
#define TRUE  1                                                                 // or 1
#define OFF   -1                                                                // Buffer history is turned off

#define RSM_MAGIC           4155766917U                                         // Seems unique
#define RSM_SYSTEM          50                                                  // MDC assigned number
#define MAX_DATABASE_BLKS   2147483647U                                         // Maximum of 2**31-1 unsigned for now
#define MAX_BLOCK_SIZE      256U                                                // Maximum block size in KiB
#define VERSION_MAJOR       1                                                   // Major version number
#define VERSION_MINOR       81                                                  // Minor version number
#define VERSION_PATCH       0                                                   // Patch version number
#define VERSION_PRE         0                                                   // Pre-release number
#define VERSION_TEST        0                                                   // Test version number
#define MBYTE               1048576                                             // 1024*1024
#define MAX_JOBS            1024                                                // Maximum number of jobs
#define DAEMONS             16                                                  // Jobs per daemon
#define MIN_DAEMONS         2                                                   // Minimum of these
#define MAX_DAEMONS         16                                                  // Maximum of these
#define MAX_GLOBAL_BUFFERS  131072                                              // Maximum global buffers in MiB
#define MAX_ROUTINE_BUFFERS 4095                                                // Maximum routine buffers in MiB

//#define MAX_INT_DIGITS 10                                                     // can be an int at 10 - not currently used
#define DEFAULT_PREC   18                                                       // default number of decimal places
#define MAX_PREC       128                                                      // max number of decimal places
#define MAX_NUM_BYTES  256                                                      // max size of a number
#define MAX_STR_LEN    65534                                                    // max size of a string (65535 VAR/NODE_UNDEFINED)
#define MAX_KEY_SIZE   255                                                      // max size of a key
#define MAX_SUB_LEN    127                                                      // max size of a subscript
#define MAX_NUM_SUBS   63                                                       // max number of subscripts
#define MAX_NUM_ARGS   (127 - 1)                                                // max number of arguments
#define MAX_NUM_TAGS   256                                                      // max number of tags/labels
#define MAX_NUM_VARS   255                                                      // max number of routine variables
// TODO keep striping stuff above here

#if RSM_DBVER == 1
#   define VAR_LEN 8                                                            // length of var_u - must be multiple of 8
#else
#   define VAR_LEN 32                                                           // length of var_u - must be multiple of 8
#endif


#define MAX_VOL 1                                                               // max number of vols
#define UCIS    64                                                              // always 64

// KeyCmp outputs
#define KEQUAL     0                                                            // outputs for..
#define K2_LESSER  1                                                            // .. KeyCmp function
#define K2_GREATER -1                                                           // ***

#define MAX_DO_FRAMES 127                                                       // maximum permitted do_frame (0 - 126)

#define MIN_SEQ_IO   0                                                         // Minimum sequential IO channel
#define MAX_SEQ_IO   64                                                        // maximum sequential IO channel
#define MAX_SEQ_NAME 256                                                       // max file name size
#define MAX_SEQ_OUT  6                                                         // max output terminator size
#define MAX_DKEY_LEN 16                                                        // max $KEY seq stored
#define SQ_FREE      0                                                         // SQ_Chan->type - free
#define SQ_FILE      1                                                         // SQ_Chan->type - disk file
#define SQ_SOCK      2                                                         // SQ_Chan->type - socket
#define SQ_PIPE      3                                                         // SQ_Chan->type - named pipe
#define SQ_TERM      4                                                         // SQ_Chan->type - terminal/character device
#define SQ_LF        -1                                                        // WRITE !
#define SQ_FF        -2                                                        // WRITE #



// Note the following three MUST be a power of 2 as they are masks for &
#define GBD_HASH  1024                                                          // hash size for global buffers
#define NUM_DIRTY 1024                                                          // max queued dirty chains
#define NUM_GARB  8192                                                          // max queued garbage blocks
#define RBD_HASH  1023                                                          // hash size for routine names


#define LOCKTAB_SIZE    32768                                                   // 32 KiB per job
#define UCI_IS_LOCALVAR 255                                                     // for struct mvar
#define VAR_UNDEFINED   (MAX_STR_LEN + 1)                                       // undefined variable (also NODE_UNDEFINED)

#define VOL_FILENAME_MAX 255                                                    // max chars in stored filename
#define JNL_FILENAME_MAX 226                                                    // max chars in journal filename

#define MAX_TRANTAB 64                                                          // total number of entries

typedef unsigned long long u_int64;                                             // UNIX unsigned quadword

typedef union __attribute__ ((__packed__)) VAR_U {                              // get at this three ways
    u_int64 var_q;                                                              // variable name (quadword) for casting
    u_int64 var_qu[VAR_LEN / 8];                                                // variable name (quadword array)
    u_char  var_cu[VAR_LEN];                                                    // variable name (as u_char[])
} var_u;                                                                        // variable name union

typedef struct __attribute__ ((__packed__)) CSTRING {                           // our string type
    u_short len;                                                                // length of it
    u_char  buf[MAX_STR_LEN + 1];                                               // and the content
} cstring;                                                                      // end counted string

typedef struct __attribute__ ((__packed__)) MVAR {                              // subscripted M var
    var_u  name;                                                                // variable name
    u_char volset;                                                              // volset number
    u_char uci;                                                                 // UCI# -> 255 = local var
    u_char slen;                                                                // subs (key) length
    u_char key[MAX_KEY_SIZE + 1];                                               // the subs (key) - allow for 0
} mvar;                                                                         // end M subs var


typedef struct __attribute__ ((__packed__)) UCI_TAB {
    var_u name;                                                                 // UCI name
    u_int global;                                                               // pointer to global directory
} uci_tab;                                                                      // define the UCI table

typedef union __attribute__ ((__packed__)) DATA_UNION {                         // diff types of msg data
    struct GBD *gbddata;                                                        // a GBD pointer
    u_int      intdata;                                                         // or an integer (block number)
} msg_data;                                                                     // end data msg union

typedef struct __attribute__ ((__packed__)) WD_TAB {                            // write daemon table
    int      pid;                                                               // the wd's pid
    int      doing;                                                             // what we are doing
    msg_data currmsg;                                                           // the current GBD */block#
} wdtab_struct;                                                                 // end write daemon structure

typedef struct __attribute__ ((__packed__)) LABEL_BLOCK {
    u_int   magic;                                                              // RSM magic number
    u_int   max_block;                                                          // maximum block number
    u_int   header_bytes;                                                       // bytes in label/map
    u_int   block_size;                                                         // bytes per data block
#if RSM_DBVER == 1
    var_u   volnam;                                                             // volume name (VAR_LEN bytes)
    u_short db_ver;                                                             // database version
#else
    u_int64 creation_time;                                                      // time database was created
    u_short db_ver;                                                             // database version
    var_u   volnam;                                                             // volume name (VAR_LEN bytes)
#endif
    u_char  journal_available;                                                  // jrnl turned on at startup
    u_char  journal_requested;                                                  // && journal_available = ON
    u_char  clean;                                                              // clean dismount flag
    char    journal_file[JNL_FILENAME_MAX + 1];                                 // journal file name
    uci_tab uci[UCIS];                                                          // current UCIs (256 + VAR_LEN - (VAR_LEN % 32)!!)
} label_block;                                                                  // define the label block


typedef struct __attribute__ ((__packed__)) DB_STAT {
    u_int dbget;                                                                // Global Gets
    u_int dbset;                                                                // Global Sets
    u_int dbkil;                                                                // Global Kills
    u_int dbdat;                                                                // Global $DATAs
    u_int dbord;                                                                // Global $ORDERs
    u_int dbqry;                                                                // Global $QUERYs
    u_int lasttry;                                                              // Search Last Tries
    u_int lastok;                                                               // Search Last Successes
    u_int logrd;                                                                // Logical Block Reads
    u_int phyrd;                                                                // Physical Block Reads
    u_int logwt;                                                                // Logical Block Writes
    u_int phywt;                                                                // Physical Block Writes
    u_int blkalloc;                                                             // Block Allocates
    u_int blkdeall;                                                             // Block Deallocates
    u_int blkreorg;                                                             // Block Reorganizes
    u_int diskerrors;                                                           // Disk write errors
} db_stat;                                                                      // database statistics

typedef struct __attribute__ ((__packed__)) VOL_DEF {
    label_block  *vollab;                                                       // pointer to volset label block
    void         *map;                                                          // start of map area
    void         *first_free;                                                   // first word with free bits
    struct GBD   *gbd_hash[GBD_HASH + 1];                                       // GBD hash table
    struct GBD   *gbd_head;                                                     // head of global buffer desc
    u_int        num_gbd;                                                       // number of global buffers
    void         *global_buf;                                                   // start of global buffers
    void         *zero_block;                                                   // empty block in memory
    struct RBD   *rbd_hash[RBD_HASH + 1];                                       // head of routine buffer desc
    void         *rbd_head;                                                     // head of routine buffer desc
    void         *rbd_end;                                                      // first address past routine area
    int          num_of_daemons;                                                // number of daemons
    wdtab_struct wd_tab[MAX_DAEMONS];                                           // write daemon info table
    int          dismount_flag;                                                 // flag to indicate dismounting
    int          map_dirty_flag;                                                // set if map is dirty
    int          writelock;                                                     // RSM write lock
    u_int        upto;                                                          // validating map up-to block
    int          shm_id;                                                        // GBD share memory ID
    struct GBD   *dirtyQ[NUM_DIRTY];                                            // dirty queue (for daemons)
    int          dirtyQw;                                                       // write pointer for dirty queue
    int          dirtyQr;                                                       // read pointer for dirty queue
    u_int        garbQ[NUM_GARB];                                               // garbage queue (for daemons)
    int          garbQw;                                                        // write pointer for garbage queue
    int          garbQr;                                                        // read pointer for garbage queue
    off_t        jrn_next;                                                      // next free offset in journal file
    char         file_name[VOL_FILENAME_MAX + 1];                               // absolute pathname of volume file
    db_stat      stats;                                                         // database statistics
} vol_def;                                                                      // end of volume def

typedef struct __attribute__ ((__packed__)) DO_FRAME {
    u_char  *routine;                                                           // address of routine (or xecute source)
    u_char  *pc;                                                                // current RSM pc
    short   *symbol;                                                            // process space symbol table pointers
    u_char  *newtab;                                                            // process space new table
    u_char  *endlin;                                                            // address of current ENDLIN
    var_u   rounam;                                                             // routine name
    u_char  vol;                                                                // rou source vol set #
    u_char  uci;                                                                // rou source UCI #
    u_short line_num;                                                           // current routine line #
    u_char  estack;                                                             // current estack offset
    u_char  type;                                                               // see TYPE_??? def
    u_char  level;                                                              // current argless do level
    u_char  flags;                                                              // flags for this frame
    long    savasp;                                                             // saved asp
    long    savssp;                                                             // saved ssp
    long    asp;                                                                // entry asp
    long    ssp;                                                                // entry ssp
    long    isp;                                                                // entry indirect pointer
} do_frame;                                                                     // do frame

// Sequential IO specific
typedef struct __attribute__ ((__packed__)) FORKTAB {
    int job_no;
    int pid;
} forktab;

typedef struct __attribute__ ((__packed__)) SERVERTAB {
    int     slots;
    int     taken;
    int     cid;
    u_char  name[MAX_SEQ_NAME];
    forktab *forked;
} servertab;

typedef union __attribute__ ((__packed__)) IN_TERM {
    u_int64 iterm;                                                              // input terminator bit mask
    u_int64 interm[2];                                                          // input terminator bit mask array
} IN_Term;

typedef struct __attribute__ ((__packed__)) SQ_CHAN {
    u_char    type;                                                             // type of device
    u_char    options;                                                          // type specific options
    u_char    mode;                                                             // how object is opened
    int       fid;                                                              // OS supplied file id
    servertab s;
    u_short   dx;                                                               // $X
    u_short   dy;                                                               // $Y
    u_char    name[MAX_SEQ_NAME];                                               // name of what was opened
    short     dkey_len;                                                         // $KEY length stored
    u_char    dkey[MAX_DKEY_LEN + 1];                                           // stored $KEY (null term)
    short     out_len;                                                          // length of output terminator
    u_char    out_term[MAX_SEQ_OUT];                                            // the output terminator
    IN_Term   in_term;                                                          // input terminator bit mask
    var_u     namespace;                                                        // routine for namespace
} SQ_Chan;                                                                      // define the $IO stuff
// End Sequential IO specific

typedef struct __attribute__ ((__packed__)) JOBTAB {
    int        pid;                                                             // OS PID (0 if unused)
    int        cur_do;                                                          // current do frame address
    u_int      commands;                                                        // commands executed
    u_int      grefs;                                                           // global references
    u_int      last_block_flags;                                                // journal etc. of last DB block
    short      error_frame;                                                     // frame error happened in
    short      etrap_at;                                                        // where $ET was invoked
    u_int      trap;                                                            // outstanding traps
    int        attention;                                                       // do something
    short      async_error;                                                     // async erors
    int        user;                                                            // user number
    short      priv;                                                            // privs this job
    short      precision;                                                       // decimal precision
    u_char     io;                                                              // current io index
    u_char     test;                                                            // current $TEST (0/1)
    u_char     uci;                                                             // current UCI number
    u_char     vol;                                                             // current volset number
    u_char     luci;                                                            // current lock UCI number
    u_char     lvol;                                                            // current lock volset number
    u_char     ruci;                                                            // current rou UCI number
    u_char     rvol;                                                            // current rou volset number
    mvar       last_ref;                                                        // $REFERENCE
    short      start_len;                                                       // length start data
    u_char     start_dh[14];                                                    // store start time here
    do_frame   dostk[MAX_DO_FRAMES + 1];                                        // the do stack (0 - 126 and STM1_FRAME)
    SQ_Chan    seqio[MAX_SEQ_IO];                                               // sequential IO stuff
    struct GBD *view[MAX_VOL];                                                  // locked view buffers
} jobtab;                                                                       // define jobtab

typedef struct __attribute__ ((__packed__)) LOCKTAB {                           // internal lock tables
    struct LOCKTAB *fwd_link;                                                   // point at next one
    int            size;                                                        // how many bytes
    short          job;                                                         // int job (-1 = free)
    short          lock_count;                                                  // how many times locked by job
    short          byte_count;                                                  // size of following reference
    u_char         vol;                                                         // vol number
    u_char         uci;                                                         // UCI number (255 = local)
    var_u          name;                                                        // var name
    u_char         key[MAX_KEY_SIZE + 1];                                       // and the key
} locktab;                                                                      // define locktab

typedef struct __attribute__ ((__packed__)) TRANTAB {                           // translation table
    var_u  from_global;                                                         // from global
    u_char from_vol;                                                            //      volumeset#
    u_char from_uci;                                                            //      UCI#
    var_u  to_global;                                                           //   to global
    u_char to_vol;                                                              //      volumeset#
    u_char to_uci;                                                              //      UCI#
} trantab;                                                                      // define trantab

typedef struct __attribute__ ((__packed__)) SYSTAB {                            // system tables
    void    *address;
    jobtab  *jobtab;                                                            // address of jobtab
    u_int   maxjob;                                                             // maximum jobs permitted
    int     sem_id;                                                             // GBD semaphore ID
    int     historic;                                                           // E notation, tag+off, $NEXT, etc.
    int     precision;                                                          // decimal precision
    int     max_tt;                                                             // max TRANTAB used
    trantab tt[MAX_TRANTAB];                                                    // translation tables
    int     start_user;                                                         // UNIX user who started the environment
    void    *lockstart;                                                         // head of lock table
    int     locksize;                                                           // how many bytes
    locktab *lockhead;                                                          // head of used locks
    locktab *lockfree;                                                          // head of lock free space
    u_int64 addoff;                                                             // offset from systab to additional buffers
    u_int64 addsize;                                                            // additional buffer size
    vol_def *vol[MAX_VOL];                                                      // array of volume pointers
    u_int   last_blk_used[MAX_VOL];                                             // actually setup for real jobs for all volumes
} systab_struct;                                                                // end of systab

extern systab_struct *systab;                                                   // make its pointer external

// Process memory structures

// PARTAB definitions
typedef struct __attribute__ ((__packed__)) PARTAB {                            // define the partition table
    jobtab  *job_table;                                                         // head of job table
    jobtab  *jobtab;                                                            // our jobtab entry
    vol_def *vol[MAX_VOL];                                                      // the current volume pointers
    int     vol_fds[MAX_VOL];                                                   // the file descriptors for volumes
    int     jnl_fds[MAX_VOL];                                                   // the file descriptors for journals
    int     debug;                                                              // debug in progress
    u_char  *strstk_start;                                                      // start of string stack
    u_char  *strstk_last;                                                       // last byte of strstk
    var_u   *varlst;                                                            // var list for compiler
    int     checkonly;                                                          // used by compiler
    u_int   errors;                                                             // used by compiler
    u_char  **sp;                                                               // source pointer for compile
    cstring **lp;                                                               // start of the line (ditto)
    int     ln;                                                                 // line num for $&%ROUCHK()
    mvar    src_var;                                                            // temp space for source mvar
    u_char  src_ln[MAX_STR_LEN + 1];                                            // temp space for source line
} partab_struct;                                                                // end of partab type

extern partab_struct partab;                                                    // globalize partab

// Compiler warning suppression
#define PRAGMA(x) _Pragma(#x)

#if __GNUC__ >= 11
#   define ENABLE_WARN _Pragma("GCC diagnostic pop")
#   define DISABLE_WARN(name) _Pragma("GCC diagnostic push") PRAGMA(GCC diagnostic ignored #name)
#else
#   define ENABLE_WARN
#   define DISABLE_WARN(name)
#endif

// VAR_U macros and inline functions
#define VAR_CLEAR(var) \
    for (u_int var_i = 0; var_i < (VAR_LEN / 8); var_i++) var.var_qu[var_i] = 0

#define VAR_COPY(var_dst, var_src) \
    for (u_int var_i = 0; var_i < (VAR_LEN / 8); var_i++) var_dst.var_qu[var_i] = var_src.var_qu[var_i]

static inline u_int var_equal(var_u var1, var_u var2)
{
    for (u_int var_i = 0; var_i < (VAR_LEN / 8); var_i++) {
        if (var1.var_qu[var_i] != var2.var_qu[var_i]) return FALSE;
    }

    return TRUE;
}

static inline u_int var_empty(var_u var)
{
    if (var.var_q == 0) return TRUE;
    return FALSE;
}


// Adjust systab pointer for reading/writing when the address layout changes (Systab Offset/Base Address/Memory)
#define SOM(ptr) ((typeof(ptr)) ((char *) (ptr) + ((char *) systab - (char *) systab->address)))
#define SOA(ptr) ((ptr == NULL) ? NULL : SOM(ptr))

#endif
