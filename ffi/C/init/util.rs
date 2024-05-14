#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    pub type GBD;
    pub type RBD;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(_: libc::c_int) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn shmctl(_: libc::c_int, _: libc::c_int, _: *mut __shmid_ds_new) -> libc::c_int;
    fn shmdt(_: *const libc::c_void) -> libc::c_int;
    fn semctl(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __error() -> *mut libc::c_int;
    fn kill(_: pid_t, _: libc::c_int) -> libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn getpid() -> pid_t;
    fn getuid() -> uid_t;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn DB_Dismount(vol: libc::c_int) -> libc::c_int;
    fn rsm_version(ret_buffer: *mut u_char) -> libc::c_int;
    fn UTIL_Share(dbf: *mut libc::c_char) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub _read: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub _seek: Option::<
        unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t,
    >,
    pub _write: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type off_t = __darwin_off_t;
pub type pid_t = __darwin_pid_t;
pub type uid_t = __darwin_uid_t;
pub type mode_t = __darwin_mode_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type gid_t = __darwin_gid_t;
pub type key_t = __int32_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct ipc_perm {
    pub uid: uid_t,
    pub gid: gid_t,
    pub cuid: uid_t,
    pub cgid: gid_t,
    pub mode: mode_t,
    pub _seq: libc::c_ushort,
    pub _key: key_t,
}
pub type shmatt_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct __shmid_ds_new {
    pub shm_perm: ipc_perm,
    pub shm_segsz: size_t,
    pub shm_lpid: pid_t,
    pub shm_cpid: pid_t,
    pub shm_nattch: shmatt_t,
    pub shm_atime: time_t,
    pub shm_dtime: time_t,
    pub shm_ctime: time_t,
    pub shm_internal: *mut libc::c_void,
}
pub type u_int64 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union VAR_U {
    pub var_q: u_int64,
    pub var_qu: [u_int64; 4],
    pub var_cu: [u_char; 32],
}
pub type var_u = VAR_U;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CSTRING {
    pub len: u_short,
    pub buf: [u_char; 65535],
}
pub type cstring = CSTRING;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct MVAR {
    pub name: var_u,
    pub volset: u_char,
    pub uci: u_char,
    pub slen: u_char,
    pub key: [u_char; 256],
}
pub type mvar = MVAR;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct UCI_TAB {
    pub name: var_u,
    pub global: u_int,
}
pub type uci_tab = UCI_TAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union DATA_UNION {
    pub gbddata: *mut GBD,
    pub intdata: u_int,
}
pub type msg_data = DATA_UNION;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct WD_TAB {
    pub pid: libc::c_int,
    pub doing: libc::c_int,
    pub currmsg: msg_data,
}
pub type wdtab_struct = WD_TAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct LABEL_BLOCK {
    pub magic: u_int,
    pub max_block: u_int,
    pub header_bytes: u_int,
    pub block_size: u_int,
    pub creation_time: u_int64,
    pub db_ver: u_short,
    pub volnam: var_u,
    pub journal_available: u_char,
    pub journal_requested: u_char,
    pub clean: u_char,
    pub journal_file: [libc::c_char; 227],
    pub uci: [uci_tab; 64],
}
pub type label_block = LABEL_BLOCK;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct DB_STAT {
    pub dbget: u_int,
    pub dbset: u_int,
    pub dbkil: u_int,
    pub dbdat: u_int,
    pub dbord: u_int,
    pub dbqry: u_int,
    pub lasttry: u_int,
    pub lastok: u_int,
    pub logrd: u_int,
    pub phyrd: u_int,
    pub logwt: u_int,
    pub phywt: u_int,
    pub blkalloc: u_int,
    pub blkdeall: u_int,
    pub blkreorg: u_int,
    pub diskerrors: u_int,
}
pub type db_stat = DB_STAT;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct VOL_DEF {
    pub vollab: *mut label_block,
    pub map: *mut libc::c_void,
    pub first_free: *mut libc::c_void,
    pub gbd_hash: [*mut GBD; 1025],
    pub gbd_head: *mut GBD,
    pub num_gbd: u_int,
    pub global_buf: *mut libc::c_void,
    pub zero_block: *mut libc::c_void,
    pub rbd_hash: [*mut RBD; 1024],
    pub rbd_head: *mut libc::c_void,
    pub rbd_end: *mut libc::c_void,
    pub num_of_daemons: libc::c_int,
    pub wd_tab: [wdtab_struct; 20],
    pub dismount_flag: libc::c_int,
    pub map_dirty_flag: libc::c_int,
    pub writelock: libc::c_int,
    pub upto: u_int,
    pub shm_id: libc::c_int,
    pub dirtyQ: [*mut GBD; 1024],
    pub dirtyQw: libc::c_int,
    pub dirtyQr: libc::c_int,
    pub garbQ: [u_int; 8192],
    pub garbQw: libc::c_int,
    pub garbQr: libc::c_int,
    pub jrn_next: off_t,
    pub file_name: [libc::c_char; 256],
    pub stats: db_stat,
}
pub type vol_def = VOL_DEF;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct DO_FRAME {
    pub routine: *mut u_char,
    pub pc: *mut u_char,
    pub symbol: *mut libc::c_short,
    pub newtab: *mut u_char,
    pub endlin: *mut u_char,
    pub rounam: var_u,
    pub vol: u_char,
    pub uci: u_char,
    pub line_num: u_short,
    pub estack: u_char,
    pub type_0: u_char,
    pub level: u_char,
    pub flags: u_char,
    pub savasp: libc::c_long,
    pub savssp: libc::c_long,
    pub asp: libc::c_long,
    pub ssp: libc::c_long,
    pub isp: libc::c_long,
}
pub type do_frame = DO_FRAME;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct FORKTAB {
    pub job_no: libc::c_int,
    pub pid: libc::c_int,
}
pub type forktab = FORKTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SERVERTAB {
    pub slots: libc::c_int,
    pub taken: libc::c_int,
    pub cid: libc::c_int,
    pub name: [u_char; 256],
    pub forked: *mut forktab,
}
pub type servertab = SERVERTAB;
#[derive(Copy, Clone)]
#[repr(C)]
pub union IN_TERM {
    pub iterm: u_int64,
    pub interm: [u_int64; 2],
}
pub type IN_Term = IN_TERM;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SQ_CHAN {
    pub type_0: u_char,
    pub options: u_char,
    pub mode: u_char,
    pub fid: libc::c_int,
    pub s: servertab,
    pub dx: u_short,
    pub dy: u_short,
    pub name: [u_char; 256],
    pub dkey_len: libc::c_short,
    pub dkey: [u_char; 17],
    pub out_len: libc::c_short,
    pub out_term: [u_char; 6],
    pub in_term: IN_Term,
    pub namespace: var_u,
}
pub type SQ_Chan = SQ_CHAN;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct JOBTAB {
    pub pid: libc::c_int,
    pub cur_do: libc::c_int,
    pub commands: u_int,
    pub grefs: u_int,
    pub last_block_flags: u_int,
    pub error_frame: libc::c_short,
    pub etrap_at: libc::c_short,
    pub trap: u_int,
    pub attention: libc::c_int,
    pub async_error: libc::c_short,
    pub user: libc::c_int,
    pub priv_0: libc::c_short,
    pub precision: libc::c_short,
    pub io: u_char,
    pub test: u_char,
    pub uci: u_char,
    pub vol: u_char,
    pub luci: u_char,
    pub lvol: u_char,
    pub ruci: u_char,
    pub rvol: u_char,
    pub last_ref: mvar,
    pub start_len: libc::c_short,
    pub start_dh: [u_char; 14],
    pub dostk: [do_frame; 128],
    pub seqio: [SQ_Chan; 64],
    pub view: [*mut GBD; 1],
}
pub type jobtab = JOBTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct LOCKTAB {
    pub fwd_link: *mut LOCKTAB,
    pub size: libc::c_int,
    pub job: libc::c_short,
    pub lock_count: libc::c_short,
    pub byte_count: libc::c_short,
    pub vol: u_char,
    pub uci: u_char,
    pub name: var_u,
    pub key: [u_char; 256],
}
pub type locktab = LOCKTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct TRANTAB {
    pub from_global: var_u,
    pub from_vol: u_char,
    pub from_uci: u_char,
    pub to_global: var_u,
    pub to_vol: u_char,
    pub to_uci: u_char,
}
pub type trantab = TRANTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SYSTAB {
    pub address: *mut libc::c_void,
    pub jobtab: *mut jobtab,
    pub maxjob: u_int,
    pub sem_id: libc::c_int,
    pub historic: libc::c_int,
    pub precision: libc::c_int,
    pub max_tt: libc::c_int,
    pub tt: [trantab; 8],
    pub start_user: libc::c_int,
    pub lockstart: *mut libc::c_void,
    pub locksize: libc::c_int,
    pub lockhead: *mut locktab,
    pub lockfree: *mut locktab,
    pub addoff: u_long,
    pub addsize: u_long,
    pub vol: [*mut vol_def; 1],
    pub last_blk_used: [u_int; 1],
}
pub type systab_struct = SYSTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct PARTAB {
    pub jobtab: *mut jobtab,
    pub vol_fds: [libc::c_int; 1],
    pub jnl_fds: [libc::c_int; 1],
    pub debug: libc::c_int,
    pub strstk_start: *mut u_char,
    pub strstk_last: *mut u_char,
    pub varlst: *mut var_u,
    pub checkonly: libc::c_int,
    pub errors: u_int,
    pub sp: *mut *mut u_char,
    pub lp: *mut *mut cstring,
    pub ln: *mut libc::c_int,
    pub src_var: mvar,
}
pub type partab_struct = PARTAB;
#[no_mangle]
pub unsafe extern "C" fn help() {
    let mut version: [libc::c_char; 100] = [0; 100];
    rsm_version(version.as_mut_ptr() as *mut u_char);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, version.as_mut_ptr());
    printf(b"David Wicksell <dlw@linux.com>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Copyright (c) 2020-2023 Fourth Watch Software LC\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"https://gitlab.com/Reference-Standard-M/rsm\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"Show information:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  rsm -V\t\t\tOutput short version string\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"  rsm -h\t\t\tOutput help menu\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  rsm -i [<database-file>]\tOutput system info menu\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"Create database:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  rsm -v <volume-name>\t\tName of volume (1-%d alpha characters)\n\0"
            as *const u8 as *const libc::c_char,
        32 as libc::c_int,
    );
    printf(
        b"      -b <block-size>\t\tSize of database blocks (1-256 KiB)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"      -s <database-size>\tInitial size of database (100-%u blocks)\n\0"
            as *const u8 as *const libc::c_char,
        2147483647 as libc::c_uint,
    );
    printf(
        b"     [-m <map-size>]\t\tSize of map block (0-%u KiB)\n\0" as *const u8
            as *const libc::c_char,
        (((2147483647 as libc::c_uint).wrapping_div(8 as libc::c_int as libc::c_uint)
            as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<label_block>() as libc::c_ulong)
            as u_int / 1024 as libc::c_int as u_int)
            .wrapping_add(1 as libc::c_int as u_int),
    );
    printf(
        b"     [-e <environment-name>]\tName of manager UCI (1-%d alpha characters)\n\0"
            as *const u8 as *const libc::c_char,
        32 as libc::c_int,
    );
    printf(
        b"     [<database-file>]\t\tName of database file\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"Initialize and start environment:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  rsm -j <max-jobs>\t\tMaximum jobs allowed in environment (1-%d jobs)\n\0"
            as *const u8 as *const libc::c_char,
        512 as libc::c_int,
    );
    printf(
        b"     [-g <global-buffers>]\tSize of global buffers (1-%d MiB)\n\0" as *const u8
            as *const libc::c_char,
        131072 as libc::c_int,
    );
    printf(
        b"     [-r <routine-buffers>]\tSize of routine buffers (1-%d MiB)\n\0"
            as *const u8 as *const libc::c_char,
        4095 as libc::c_int,
    );
    printf(
        b"     [<database-file>]\t\tName of database file\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Start job and attach to environment:\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"  rsm \t\t\t\tStarts in direct mode in manager UCI\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"     [-e <environment-name>]\tName of initial UCI environment\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"     [-x <M-commands>]\t\tString of M commands to execute\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"     [-R]\t\t\tStarts in restricted mode\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"     [<database-file>]\t\tName of database file\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"Stop and shut down environment:\n\0" as *const u8 as *const libc::c_char);
    printf(b"  rsm -k\t\t\tKill environment\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"     [<database-file>]\t\tName of database file\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Set environment variable RSM_DBFILE=<database-file> or pass it to each command\n\0"
            as *const u8 as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn info(mut file: *mut libc::c_char) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut version: [libc::c_char; 100] = [0; 100];
    let mut pidlen: libc::c_char = 0;
    let mut margin: libc::c_char = 24 as libc::c_int as libc::c_char;
    rsm_version(version.as_mut_ptr() as *mut u_char);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, version.as_mut_ptr());
    printf(
        b"Database Version: %d\tCompiler Version: %d\n\n\0" as *const u8
            as *const libc::c_char,
        2 as libc::c_int,
        8 as libc::c_int,
    );
    printf(b"David Wicksell <dlw@linux.com>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Copyright (c) 2020-2023 Fourth Watch Software LC\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"https://gitlab.com/Reference-Standard-M/rsm\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Database and Environment Configuration Information:\n\n\0" as *const u8
            as *const libc::c_char,
    );
    if file.is_null() {
        fprintf(
            __stderrp,
            b"Please pass database file path or set RSM_DBFILE.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    i = UTIL_Share(file);
    if i != 0 as libc::c_int || ((*systab).vol[0 as libc::c_int as usize]).is_null() {
        fprintf(
            __stderrp,
            b"Cannot connect to environment.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    printf(
        b"Job Table Slots:\t%u\tJob%s\n\0" as *const u8 as *const libc::c_char,
        (*systab).maxjob,
        if (*systab).maxjob > 1 as libc::c_int as u_int {
            b"s\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"Lock Table Size:\t%d\tKiB\n\0" as *const u8 as *const libc::c_char,
        (*systab).locksize / 1024 as libc::c_int,
    );
    printf(
        b"Semaphore Array ID:\t%d\n\0" as *const u8 as *const libc::c_char,
        (*systab).sem_id,
    );
    i = 0 as libc::c_int;
    while i < 1 as libc::c_int {
        if !((*systab).vol[i as usize]).is_null() {
            printf(
                b"\n*** Volume %d ***\n\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
            );
            printf(
                b"DB File Path:\t\t%s\n\0" as *const u8 as *const libc::c_char,
                ((*(*systab).vol[i as usize]).file_name).as_mut_ptr(),
            );
            printf(
                b"DB Volume Name:\t\t%s \n\0" as *const u8 as *const libc::c_char,
                ((*(*(*systab).vol[i as usize]).vollab).volnam.var_cu).as_mut_ptr(),
            );
            printf(
                b"DB Manager UCI Name:\t%s \n\0" as *const u8 as *const libc::c_char,
                ((*(*(*systab).vol[i as usize]).vollab)
                    .uci[0 as libc::c_int as usize]
                    .name
                    .var_cu)
                    .as_mut_ptr(),
            );
            printf(
                b"DB Journal File Path:\t%s [%s]\n\0" as *const u8
                    as *const libc::c_char,
                if (*(*(*systab).vol[i as usize]).vollab)
                    .journal_file[0 as libc::c_int as usize] as libc::c_int
                    != '\0' as i32
                {
                    ((*(*(*systab).vol[i as usize]).vollab).journal_file).as_mut_ptr()
                        as *const libc::c_char
                } else {
                    b"--\0" as *const u8 as *const libc::c_char
                },
                if (*(*(*systab).vol[i as usize]).vollab).journal_available
                    as libc::c_int != 0
                {
                    b"ON\0" as *const u8 as *const libc::c_char
                } else {
                    b"OFF\0" as *const u8 as *const libc::c_char
                },
            );
            printf(
                b"DB Size in Blocks:\t%u\n\0" as *const u8 as *const libc::c_char,
                (*(*(*systab).vol[i as usize]).vollab).max_block,
            );
            printf(
                b"DB Map Block Size:\t%u\tKiB\n\0" as *const u8 as *const libc::c_char,
                (*(*(*systab).vol[i as usize]).vollab).header_bytes
                    / 1024 as libc::c_int as u_int,
            );
            printf(
                b"DB Block Size:\t\t%u\tKiB\n\0" as *const u8 as *const libc::c_char,
                (*(*(*systab).vol[i as usize]).vollab).block_size
                    / 1024 as libc::c_int as u_int,
            );
            printf(
                b"Global Buffers:\t\t%d\tMiB (%u Buffers)\n\0" as *const u8
                    as *const libc::c_char,
                (((*(*systab).vol[i as usize]).zero_block)
                    .offset_from((*(*systab).vol[i as usize]).global_buf) as libc::c_long
                    / 1048576 as libc::c_int as libc::c_long) as libc::c_int,
                (*(*systab).vol[i as usize]).num_gbd,
            );
            printf(
                b"Routine Buffer Space:\t%d\tMiB\n\0" as *const u8
                    as *const libc::c_char,
                (((*(*systab).vol[i as usize]).rbd_end)
                    .offset_from((*(*systab).vol[i as usize]).rbd_head) as libc::c_long
                    / 1048576 as libc::c_int as libc::c_long) as libc::c_int,
            );
            printf(
                b"Shared Memory ID:\t%d\n\0" as *const u8 as *const libc::c_char,
                (*(*systab).vol[i as usize]).shm_id,
            );
            printf(b"Daemon Process IDs:\t\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < 20 as libc::c_int {
                if (*(*systab).vol[i as usize]).wd_tab[j as usize].pid
                    == 0 as libc::c_int
                {
                    break;
                }
                pidlen = (floor(
                    log10(
                        (*(*systab).vol[i as usize]).wd_tab[j as usize].pid
                            as libc::c_double,
                    ),
                ) + 1 as libc::c_int as libc::c_double) as libc::c_char;
                if margin as libc::c_int + pidlen as libc::c_int > 80 as libc::c_int {
                    margin = (pidlen as libc::c_int + 25 as libc::c_int) as libc::c_char;
                    printf(b"\n\t\t\t\0" as *const u8 as *const libc::c_char);
                } else {
                    margin = (margin as libc::c_int
                        + (pidlen as libc::c_int + 1 as libc::c_int)) as libc::c_char;
                }
                printf(
                    b"%d \0" as *const u8 as *const libc::c_char,
                    (*(*systab).vol[i as usize]).wd_tab[j as usize].pid,
                );
                j += 1;
                j;
            }
            putchar('\n' as i32);
            margin = 24 as libc::c_int as libc::c_char;
        }
        i += 1;
        i;
    }
    shmdt(systab as *const libc::c_void);
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn shutdown(mut file: *mut libc::c_char) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut no_daemons: libc::c_int = 0;
    let mut user: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    let mut sbuf: __shmid_ds_new = __shmid_ds_new {
        shm_perm: ipc_perm {
            uid: 0,
            gid: 0,
            cuid: 0,
            cgid: 0,
            mode: 0,
            _seq: 0,
            _key: 0,
        },
        shm_segsz: 0,
        shm_lpid: 0,
        shm_cpid: 0,
        shm_nattch: 0,
        shm_atime: 0,
        shm_dtime: 0,
        shm_ctime: 0,
        shm_internal: 0 as *mut libc::c_void,
    };
    let mut semvals: *mut libc::c_void = 0 as *mut libc::c_void;
    if file.is_null() {
        fprintf(
            __stderrp,
            b"Please pass database file path or set RSM_DBFILE.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    i = UTIL_Share(file);
    if i != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"RSM environment is not initialized.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if ((*systab).vol[0 as libc::c_int as usize]).is_null() {
        fprintf(
            __stderrp,
            b"Error occurred in process - Environment does not match runtime image version\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 1 as libc::c_int {
        if !((*systab).vol[i as usize]).is_null() {
            if (*(*(*systab).vol[i as usize]).vollab).journal_available as libc::c_int
                != 0
                && (*(*(*systab).vol[i as usize]).vollab).journal_requested
                    as libc::c_int != 0
            {
                partab
                    .jnl_fds[i
                    as usize] = open(
                    ((*(*(*systab).vol[i as usize]).vollab).journal_file).as_mut_ptr(),
                    0x2 as libc::c_int,
                );
                if partab.jnl_fds[i as usize] == -(1 as libc::c_int) {
                    fprintf(
                        __stderrp,
                        b"Failed to open journal file: %s\nerrno = %d\n\0" as *const u8
                            as *const libc::c_char,
                        ((*(*(*systab).vol[i as usize]).vollab).journal_file)
                            .as_mut_ptr(),
                        *__error(),
                    );
                    (*(*(*systab).vol[i as usize]).vollab)
                        .journal_available = 0 as libc::c_int as u_char;
                }
            }
        }
        i += 1;
        i;
    }
    user = getuid() as libc::c_int;
    if user != (*systab).start_user && user != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"User does not have permission to shut down RSM.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    i = 1 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        no_daemons = 1 as libc::c_int;
        if !((*systab).vol[i as usize]).is_null() {
            (*(*systab).vol[i as usize])
                .writelock = -(512 as libc::c_int + 1 as libc::c_int);
            while (*(*systab).vol[i as usize]).writelock < 0 as libc::c_int {
                sleep(1 as libc::c_int as libc::c_uint);
                j = 0 as libc::c_int;
                while j < (*(*systab).vol[i as usize]).num_of_daemons {
                    if kill(
                        (*(*systab).vol[i as usize]).wd_tab[j as usize].pid,
                        0 as libc::c_int,
                    ) == 0 as libc::c_int
                    {
                        no_daemons = 0 as libc::c_int;
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
                if no_daemons != 0 {
                    break;
                }
            }
            (*(*systab).vol[i as usize])
                .writelock = 512 as libc::c_int + 1 as libc::c_int;
        }
        i -= 1;
        i;
    }
    if shmctl(
        (*(*systab).vol[0 as libc::c_int as usize]).shm_id,
        0 as libc::c_int,
        &mut sbuf,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            __stderrp,
            b"errno = %d %s\n\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
    }
    (*systab).start_user = -(1 as libc::c_int);
    pid = getpid();
    let mut k: u_int = 0 as libc::c_int as u_int;
    while k < (*systab).maxjob {
        let mut cnt: libc::c_int = (*((*systab).jobtab).offset(k as isize)).pid;
        if cnt != pid && cnt != 0 {
            if kill(cnt, 15 as libc::c_int) == 0 {
                (*((*systab).jobtab).offset(k as isize))
                    .trap = (1 as libc::c_uint) << 15 as libc::c_int;
                (*((*systab).jobtab).offset(k as isize)).attention = 1 as libc::c_int;
            }
        }
        k = k.wrapping_add(1);
        k;
    }
    i = 1 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if !((*systab).vol[i as usize]).is_null() {
            DB_Dismount(i + 1 as libc::c_int);
            if i == 0 as libc::c_int && no_daemons != 0 {
                if semctl((*systab).sem_id, 0 as libc::c_int, 0 as libc::c_int, semvals)
                    == -(1 as libc::c_int)
                {
                    fprintf(
                        __stderrp,
                        b"errno = %d %s\n\0" as *const u8 as *const libc::c_char,
                        *__error(),
                        strerror(*__error()),
                    );
                }
            }
        }
        i -= 1;
        i;
    }
    printf(b"RSM environment shut down.\n\0" as *const u8 as *const libc::c_char);
    exit(0 as libc::c_int);
}
