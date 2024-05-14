#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    pub type GBD;
    pub type RBD;
    static mut __stderrp: *mut FILE;
    fn fflush(_: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn freopen(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn abort() -> !;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn getpid() -> pid_t;
    fn ctime(_: *const time_t) -> *mut libc::c_char;
    fn __error() -> *mut libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn rsm_version(ret_buffer: *mut u_char) -> libc::c_int;
    fn current_time(local: libc::c_short) -> time_t;
    fn UTIL_String_Mvar(
        var: *mut mvar,
        str: *mut u_char,
        max_subs: libc::c_int,
    ) -> libc::c_short;
}
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_longlong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
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
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type time_t = __darwin_time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub err: libc::c_int,
    pub msg: *mut libc::c_char,
}
static mut merrtab: [C2RustUnnamed; 124] = [
    {
        let mut init = C2RustUnnamed {
            err: 0 as libc::c_int,
            msg: b"No error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 1 as libc::c_int,
            msg: b"Naked indicator undefined\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 2 as libc::c_int,
            msg: b"Invalid $FNUMBER P code string combination\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 3 as libc::c_int,
            msg: b"$RANDOM argument less than 1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 4 as libc::c_int,
            msg: b"No true condition in $SELECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 5 as libc::c_int,
            msg: b"Line reference less than 0\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 6 as libc::c_int,
            msg: b"Undefined local variable\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 7 as libc::c_int,
            msg: b"Undefined global variable\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 8 as libc::c_int,
            msg: b"Undefined special variable\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 9 as libc::c_int,
            msg: b"Divide by zero\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 10 as libc::c_int,
            msg: b"Invalid pattern match range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 11 as libc::c_int,
            msg: b"No parameters passed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 12 as libc::c_int,
            msg: b"Invalid line reference (negative offset)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 13 as libc::c_int,
            msg: b"Invalid line reference (line not found)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 14 as libc::c_int,
            msg: b"Line level not one\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 15 as libc::c_int,
            msg: b"Undefined index variable\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 16 as libc::c_int,
            msg: b"QUIT with an argument not allowed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 17 as libc::c_int,
            msg: b"QUIT with an argument required\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 18 as libc::c_int,
            msg: b"Fixed length READ not greater than 0\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 19 as libc::c_int,
            msg: b"Cannot merge a tree or subtree into itself\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 20 as libc::c_int,
            msg: b"Line must have a formal list\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 21 as libc::c_int,
            msg: b"Formal list name duplication\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 22 as libc::c_int,
            msg: b"SET or KILL to ^$GLOBAL when data in global\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 23 as libc::c_int,
            msg: b"SET or KILL to ^$JOB for non-existent job\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 25 as libc::c_int,
            msg: b"Attempt to modify currently executing routine\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 26 as libc::c_int,
            msg: b"Non-existent environment\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 28 as libc::c_int,
            msg: b"Mathematical function, parameter out of range\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 29 as libc::c_int,
            msg: b"SET or KILL on SSVN not allowed by implementation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 33 as libc::c_int,
            msg: b"SET or KILL to ^$ROUTINE when routine exists\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 35 as libc::c_int,
            msg: b"Device does not support mnemonicspace\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 36 as libc::c_int,
            msg: b"Incompatible mnemonicspaces\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 37 as libc::c_int,
            msg: b"READ from device identified by the empty string\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 38 as libc::c_int,
            msg: b"Invalid SSVN subscript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 39 as libc::c_int,
            msg: b"Invalid $NAME argument\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 40 as libc::c_int,
            msg: b"Call-by-reference in JOB actual\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 43 as libc::c_int,
            msg: b"Invalid range ($X, $Y)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 45 as libc::c_int,
            msg: b"Invalid GOTO reference\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 46 as libc::c_int,
            msg: b"Invalid attribute name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 47 as libc::c_int,
            msg: b"Invalid attribute name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 56 as libc::c_int,
            msg: b"Name length exceeds implementation's limit\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 57 as libc::c_int,
            msg: b"More than one defining occurrence of label in routine\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 58 as libc::c_int,
            msg: b"Too few formal parameters\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 59 as libc::c_int,
            msg: b"Environment reference not permitted for this SSVN\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 60 as libc::c_int,
            msg: b"Undefined SSVN\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 75 as libc::c_int,
            msg: b"String length exceeds implementation's limit\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 92 as libc::c_int,
            msg: b"Mathematical overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 93 as libc::c_int,
            msg: b"Mathematical underflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 99 as libc::c_int,
            msg: b"Invalid operation for context\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 101 as libc::c_int,
            msg: b"Attempt to assign incorrect value to $ECODE\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 1 as libc::c_int + 200 as libc::c_int,
            msg: b"Subscript too long (max 127)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 2 as libc::c_int + 200 as libc::c_int,
            msg: b"Key too long (max 255)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 3 as libc::c_int + 200 as libc::c_int,
            msg: b"Error in key\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 4 as libc::c_int + 200 as libc::c_int,
            msg: b"Error in database create \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 5 as libc::c_int + 200 as libc::c_int,
            msg: b"Null character not permitted in key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 6 as libc::c_int + 200 as libc::c_int,
            msg: b"Error when reading from database file\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 7 as libc::c_int + 200 as libc::c_int,
            msg: b"DO stack overflow (max 128)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 8 as libc::c_int + 200 as libc::c_int,
            msg: b"String stack overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 9 as libc::c_int + 200 as libc::c_int,
            msg: b"Invalid BREAK parameter\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 10 as libc::c_int + 200 as libc::c_int,
            msg: b"String stack underflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 11 as libc::c_int + 200 as libc::c_int,
            msg: b"Database file is full cannot SET\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 12 as libc::c_int + 200 as libc::c_int,
            msg: b"Expression syntax error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 13 as libc::c_int + 200 as libc::c_int,
            msg: b"Command syntax error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 14 as libc::c_int + 200 as libc::c_int,
            msg: b"Unknown opcode encountered\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 15 as libc::c_int + 200 as libc::c_int,
            msg: b"Too many subscripts (max 63)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 16 as libc::c_int + 200 as libc::c_int,
            msg: b"Null subscript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 17 as libc::c_int + 200 as libc::c_int,
            msg: b"Too many IF commands in one line (max 256)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 18 as libc::c_int + 200 as libc::c_int,
            msg: b"Unknown external routine\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 19 as libc::c_int + 200 as libc::c_int,
            msg: b"Too many nested FOR commands (max 256)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 20 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Unknown internal error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 21 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Unrecognised operation\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 22 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Timeout < -1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 23 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Operation timed out\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 24 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Device not supported\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 25 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Channel out of range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 26 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Channel not free\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 27 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Channel free\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 28 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Unexpected NULL value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 29 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Can not determine object from operation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 30 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Unrecognised object\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 31 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Set bit flag out of range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 33 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Number of bytes for buffer out of range\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 34 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: ASCII character expected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 35 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Unrecognised mode\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 36 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Maximum bytes to read < -1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 37 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Read buffer size exceeded\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 38 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: End of file has been reached\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 39 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: $KEY too long\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 40 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Bytes to write < 0\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 41 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Write format specifier < -2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 42 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Maximum number of jobs could be exceeded\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 43 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Device not found or a character special device\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 44 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Printf failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 45 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Unsigned integer value expected\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 46 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Peer has disconnected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 47 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: No peer connected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 48 as libc::c_int + 200 as libc::c_int,
            msg: b"IO: Invalid internet address\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 49 as libc::c_int + 200 as libc::c_int,
            msg: b"Job table is full\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 50 as libc::c_int + 200 as libc::c_int,
            msg: b"Invalid argument to $STACK()\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 51 as libc::c_int + 200 as libc::c_int,
            msg: b"Interrupt - Control-C Received\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 52 as libc::c_int + 200 as libc::c_int,
            msg: b"Insufficient space to load routine\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 53 as libc::c_int + 200 as libc::c_int,
            msg: b"Too many tags (max 256)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 54 as libc::c_int + 200 as libc::c_int,
            msg: b"Too many lines in routine (max 65534)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 55 as libc::c_int + 200 as libc::c_int,
            msg: b"End of linked data reached\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 56 as libc::c_int + 200 as libc::c_int,
            msg: b"Symbol table full\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 57 as libc::c_int + 200 as libc::c_int,
            msg: b"Invalid name indirection\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 58 as libc::c_int + 200 as libc::c_int,
            msg: b"Too many levels of indirection\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 59 as libc::c_int + 200 as libc::c_int,
            msg: b"Routine version mismatch - please recompile\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 60 as libc::c_int + 200 as libc::c_int,
            msg: b"Insufficient global buffer space\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 61 as libc::c_int + 200 as libc::c_int,
            msg: b"Database integrity violation found\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 62 as libc::c_int + 200 as libc::c_int,
            msg: b"Cannot create global - global directory full\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 63 as libc::c_int + 200 as libc::c_int,
            msg: b"Error in VIEW arguments\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 64 as libc::c_int + 200 as libc::c_int,
            msg: b"Parameter out of range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 65 as libc::c_int + 200 as libc::c_int,
            msg: b"Duplicate tag in routine\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 66 as libc::c_int + 200 as libc::c_int,
            msg: b"HUP signal received\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 67 as libc::c_int + 200 as libc::c_int,
            msg: b"USR1 signal received\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 68 as libc::c_int + 200 as libc::c_int,
            msg: b"USR2 signal received\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 69 as libc::c_int + 200 as libc::c_int,
            msg: b"Unknown signal received\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 70 as libc::c_int + 200 as libc::c_int,
            msg: b"Offset not permitted in entryref\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 71 as libc::c_int + 200 as libc::c_int,
            msg: b"No such host is known\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 72 as libc::c_int + 200 as libc::c_int,
            msg: b"Type h_errno error has occurred\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 73 as libc::c_int + 200 as libc::c_int,
            msg: b"Invalid database file specified\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 74 as libc::c_int + 200 as libc::c_int,
            msg: b"Too many variables (max 256)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 75 as libc::c_int + 200 as libc::c_int,
            msg: b"Too many arguments (max (127 - 1))\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            err: 0 as libc::c_int,
            msg: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn UTIL_strerror(
    mut err: libc::c_int,
    mut buf: *mut u_char,
) -> u_short {
    let mut none: [u_char; 21] = *::core::mem::transmute::<
        &[u8; 21],
        &mut [u_char; 21],
    >(b"No such error number\0");
    let mut ptr: *mut u_char = 0 as *mut u_char;
    ptr = none.as_mut_ptr();
    if err < 0 as libc::c_int {
        err = -err;
    }
    if err > 200 as libc::c_int + 200 as libc::c_int {
        ptr = strerror(err - (200 as libc::c_int + 200 as libc::c_int)) as *mut u_char;
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        while !(merrtab[i as usize].msg).is_null() {
            if merrtab[i as usize].err == err {
                ptr = merrtab[i as usize].msg as *mut u_char;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    strcpy(buf as *mut libc::c_char, ptr as *mut libc::c_char);
    return strlen(ptr as *mut libc::c_char) as u_short;
}
#[no_mangle]
pub unsafe extern "C" fn panic(mut msg: *mut libc::c_char) {
    let mut a: *mut FILE = 0 as *mut FILE;
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut t: time_t = 0;
    fprintf(
        __stderrp,
        b"\r\nFATAL RSM ERROR occurred!!\r\n%s\r\n\0" as *const u8
            as *const libc::c_char,
        msg,
    );
    if *__error() != 0 {
        fprintf(
            __stderrp,
            b"errno = %d - %s\n\r\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
    }
    fflush(__stderrp);
    a = freopen(
        b"RSM_CRASH\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
        __stderrp,
    );
    if !a.is_null() {
        t = current_time(0 as libc::c_int as libc::c_short);
        fprintf(
            __stderrp,
            b"RSM CRASH OCCURRED on %s\0" as *const u8 as *const libc::c_char,
            ctime(&mut t),
        );
        rsm_version(tmp.as_mut_ptr() as *mut u_char);
        fprintf(
            __stderrp,
            b"%s\0" as *const u8 as *const libc::c_char,
            tmp.as_mut_ptr(),
        );
        fprintf(
            __stderrp,
            b"\nFATAL RSM ERROR occurred - pid %d!!\n%s\n\0" as *const u8
                as *const libc::c_char,
            getpid(),
            msg,
        );
        if *__error() != 0 {
            fprintf(
                __stderrp,
                b"errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
                *__error(),
                strerror(*__error()),
            );
        }
        if !(partab.jobtab).is_null() {
            let mut j: libc::c_int = (*partab.jobtab).cur_do;
            fprintf(
                __stderrp,
                b"Job Number: %d\n\0" as *const u8 as *const libc::c_char,
                (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                    as libc::c_int + 1 as libc::c_int,
            );
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                tmp[i
                    as usize] = (*partab.jobtab)
                    .dostk[j as usize]
                    .rounam
                    .var_cu[i as usize] as libc::c_char;
                i += 1;
                i;
            }
            tmp[32 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            fprintf(
                __stderrp,
                b"UCI: %d  Routine: %s  Line: %d\n\0" as *const u8
                    as *const libc::c_char,
                (*partab.jobtab).dostk[j as usize].uci as libc::c_int,
                tmp.as_mut_ptr(),
                (*partab.jobtab).dostk[j as usize].line_num as libc::c_int,
            );
            if (*partab.jobtab).last_ref.name.var_cu[0 as libc::c_int as usize] != 0 {
                UTIL_String_Mvar(
                    &mut (*partab.jobtab).last_ref,
                    &mut *tmp.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut libc::c_char as *mut u_char,
                    63 as libc::c_int,
                );
                fprintf(
                    __stderrp,
                    b"Last Global: %s\n\0" as *const u8 as *const libc::c_char,
                    tmp.as_mut_ptr(),
                );
            }
        }
        fprintf(
            __stderrp,
            b"----------------------------------------------------------------------------\n\0"
                as *const u8 as *const libc::c_char,
        );
        fflush(__stderrp);
    }
    abort();
}
