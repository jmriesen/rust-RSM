#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    pub type GBD;
    pub type RBD;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn __error() -> *mut libc::c_int;
    fn ftok(_: *const libc::c_char, _: libc::c_int) -> key_t;
    fn shmat(
        _: libc::c_int,
        _: *const libc::c_void,
        _: libc::c_int,
    ) -> *mut libc::c_void;
    fn shmdt(_: *const libc::c_void) -> libc::c_int;
    fn shmget(_: key_t, _: size_t, _: libc::c_int) -> libc::c_int;
    fn semget(_: key_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn semop(_: libc::c_int, _: *mut sembuf, _: size_t) -> libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn panic(msg: *mut libc::c_char);
    static mut curr_lock: libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
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
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type key_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sembuf {
    pub sem_num: libc::c_ushort,
    pub sem_op: libc::c_short,
    pub sem_flg: libc::c_short,
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
pub unsafe extern "C" fn UTIL_Share(mut dbf: *mut libc::c_char) -> libc::c_int {
    let mut shar_mem_key: key_t = 0;
    let mut shar_mem_id: libc::c_int = 0;
    let mut sem_id: libc::c_int = 0;
    let mut sad: *mut systab_struct = 0 as *mut systab_struct;
    shar_mem_key = ftok(dbf, 50 as libc::c_int);
    if shar_mem_key == -(1 as libc::c_int) {
        return *__error();
    }
    shar_mem_id = shmget(shar_mem_key, 0 as libc::c_int as size_t, 0 as libc::c_int);
    if shar_mem_id == -(1 as libc::c_int) {
        return *__error();
    }
    sad = shmat(
        shar_mem_id,
        0x200000000 as libc::c_long as *mut libc::c_void,
        0 as libc::c_int,
    ) as *mut systab_struct;
    systab = (*sad).address as *mut systab_struct;
    if sad != systab {
        let mut i: libc::c_int = shmdt(sad as *const libc::c_void);
        if i == -(1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"shmdt return = %X\n\0" as *const u8 as *const libc::c_char,
                i,
            );
        }
        sad = shmat(shar_mem_id, systab as *mut libc::c_void, 0 as libc::c_int)
            as *mut systab_struct;
        if sad == -(1 as libc::c_int) as *mut libc::c_void as *mut systab_struct {
            fprintf(
                __stderrp,
                b"systab = %lX  attach = %lX\n\0" as *const u8 as *const libc::c_char,
                systab as u_long,
                sad as u_long,
            );
        }
        if systab != sad {
            return *__error();
        }
    }
    sem_id = semget(shar_mem_key, 0 as libc::c_int, 0 as libc::c_int);
    if sem_id < 0 as libc::c_int {
        return *__error();
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SemOp(
    mut sem_num: libc::c_int,
    mut numb: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut buf: sembuf = {
        let mut init = sembuf {
            sem_num: 0 as libc::c_int as libc::c_ushort,
            sem_op: 0 as libc::c_int as libc::c_short,
            sem_flg: 0o10000 as libc::c_int as libc::c_short,
        };
        init
    };
    if numb == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    buf.sem_num = sem_num as u_short;
    buf.sem_op = numb as libc::c_short;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut s: libc::c_short = semop(
            (*systab).sem_id,
            &mut buf,
            1 as libc::c_int as size_t,
        ) as libc::c_short;
        if s as libc::c_int == 0 as libc::c_int {
            if sem_num == 2 as libc::c_int {
                curr_lock += numb;
            }
            return 0 as libc::c_int as libc::c_short;
        }
        if numb < 1 as libc::c_int {
            if (partab.jobtab).is_null() {
                panic(
                    b"SemOp: Error in write daemon\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        if (*partab.jobtab).trap != 0 {
            return -(51 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        i += 1;
        i;
    }
    if (*systab).start_user == -(1 as libc::c_int) {
        exit(0 as libc::c_int);
    }
    if sem_num != 1 as libc::c_int || numb != 1 as libc::c_int {
        panic(
            b"SemOp: Failed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int as libc::c_short;
}
