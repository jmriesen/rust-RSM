#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn srandomdev();
    fn kill(_: pid_t, _: libc::c_int) -> libc::c_int;
    fn __error() -> *mut libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(_: libc::c_int) -> libc::c_int;
    fn getgroups(_: libc::c_int, _: *mut gid_t) -> libc::c_int;
    fn getpid() -> pid_t;
    fn getuid() -> uid_t;
    fn shmdt(_: *const libc::c_void) -> libc::c_int;
    fn tcgetattr(_: libc::c_int, _: *mut termios) -> libc::c_int;
    fn tcsetattr(_: libc::c_int, _: libc::c_int, _: *const termios) -> libc::c_int;
    fn SQ_Init() -> libc::c_short;
    fn SQ_Write(buf: *mut cstring) -> libc::c_int;
    fn SQ_WriteFormat(count: libc::c_int) -> libc::c_short;
    fn SQ_Read(buf: *mut u_char, tout: libc::c_int, maxbyt: libc::c_int) -> libc::c_int;
    fn parse();
    fn run(asp: libc::c_int, ssp: libc::c_int) -> libc::c_short;
    fn attention() -> libc::c_short;
    fn Vhorolog(ret_buffer: *mut u_char) -> libc::c_short;
    fn ST_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn ST_Kill(var: *mut mvar) -> libc::c_short;
    fn UTIL_strerror(err: libc::c_int, buf: *mut u_char) -> u_short;
    fn CleanJob(job: libc::c_int);
    fn panic(msg: *mut libc::c_char);
    fn UTIL_Share(dbf: *mut libc::c_char) -> libc::c_int;
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
    fn Xcall_errmsg(
        ret_buffer: *mut libc::c_char,
        err: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_short;
    fn ST_Init();
    static mut source_ptr: *mut u_char;
    static mut comp_ptr: *mut u_char;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
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
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type gid_t = __darwin_gid_t;
pub type time_t = __darwin_time_t;
pub type tcflag_t = libc::c_ulong;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_cc: [cc_t; 20],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
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
pub struct GBD {
    pub block: u_int,
    pub next: *mut GBD,
    pub mem: *mut DB_BLOCK,
    pub dirty: *mut GBD,
    pub last_accessed: time_t,
}
#[derive(Copy, Clone)]
#[repr(C, align(4))]
pub struct DB_BLOCK(pub DB_BLOCK_Inner);
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct DB_BLOCK_Inner {
    pub type_0: u_char,
    pub flags: u_char,
    pub spare: u_short,
    pub right_ptr: u_int,
    pub last_idx: u_short,
    pub last_free: u_short,
    pub global: var_u,
}
#[allow(dead_code, non_upper_case_globals)]
const DB_BLOCK_PADDING: usize = ::core::mem::size_of::<DB_BLOCK>()
    - ::core::mem::size_of::<DB_BLOCK_Inner>();
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct RBD {
    pub fwd_link: *mut RBD,
    pub chunk_size: u_int,
    pub attached: u_int,
    pub last_access: time_t,
    pub rnam: var_u,
    pub uci: u_char,
    pub vol: u_char,
    pub rou_size: u_short,
    pub comp_ver: u_short,
    pub comp_user: u_short,
    pub comp_date: libc::c_int,
    pub comp_time: libc::c_int,
    pub tag_tbl: u_short,
    pub num_tags: u_short,
    pub var_tbl: u_short,
    pub num_vars: u_short,
    pub code: u_short,
    pub code_size: u_short,
}
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
#[inline]
unsafe extern "C" fn var_equal(mut var1: var_u, mut var2: var_u) -> u_int {
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        if var1.var_qu[var_i as usize] != var2.var_qu[var_i as usize] {
            return 0 as libc::c_int as u_int;
        }
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    return 1 as libc::c_int as u_int;
}
#[no_mangle]
pub static mut partab: partab_struct = PARTAB {
    jobtab: 0 as *const jobtab as *mut jobtab,
    vol_fds: [0; 1],
    jnl_fds: [0; 1],
    debug: 0,
    strstk_start: 0 as *const u_char as *mut u_char,
    strstk_last: 0 as *const u_char as *mut u_char,
    varlst: 0 as *const var_u as *mut var_u,
    checkonly: 0,
    errors: 0,
    sp: 0 as *const *mut u_char as *mut *mut u_char,
    lp: 0 as *const *mut cstring as *mut *mut cstring,
    ln: 0 as *const libc::c_int as *mut libc::c_int,
    src_var: MVAR {
        name: VAR_U { var_q: 0 },
        volset: 0,
        uci: 0,
        slen: 0,
        key: [0; 256],
    },
};
#[no_mangle]
pub static mut systab: *mut systab_struct = 0 as *const systab_struct
    as *mut systab_struct;
#[no_mangle]
pub static mut tty_settings: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_cc: [0; 20],
    c_ispeed: 0,
    c_ospeed: 0,
};
#[no_mangle]
pub static mut addstk: [*mut u_char; 1024] = [0 as *const u_char as *mut u_char; 1024];
#[no_mangle]
pub static mut strstk: [u_char; 2097152] = [0; 2097152];
#[no_mangle]
pub static mut indstk: [u_char; 65536] = [0; 65536];
#[no_mangle]
pub static mut isp: libc::c_long = 0;
#[no_mangle]
pub static mut failed_tty: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut gbd_expired: libc::c_int = 60 as libc::c_int;
#[no_mangle]
pub static mut rsmpc: *mut u_char = 0 as *const u_char as *mut u_char;
#[no_mangle]
pub static mut history: [[libc::c_char; 65534]; 128] = [[0; 65534]; 128];
#[no_mangle]
pub static mut hist_next: u_short = 0 as libc::c_int as u_short;
#[no_mangle]
pub static mut hist_curr: u_short = 0 as libc::c_int as u_short;
#[no_mangle]
pub static mut in_hist: libc::c_short = 0 as libc::c_int as libc::c_short;
#[no_mangle]
pub static mut prompt_len: u_short = 8 as libc::c_int as u_short;
#[no_mangle]
pub unsafe extern "C" fn ser(mut s: libc::c_int) {
    let mut cptr: *mut cstring = 0 as *mut cstring;
    let mut junk: [u_char; 100] = [0; 100];
    if s == -(27 as libc::c_int + 200 as libc::c_int) {
        panic(
            b"Chanel zero has gone away\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if s == -(200 as libc::c_int + 200 as libc::c_int + 5 as libc::c_int) {
        panic(
            b"Input/output error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    cptr = junk.as_mut_ptr() as *mut cstring;
    if s < 0 as libc::c_int {
        s = -s;
    }
    UTIL_strerror(s, &mut *((*cptr).buf).as_mut_ptr().offset(0 as libc::c_int as isize));
    fprintf(
        __stderrp,
        b"\r\nERROR occurred %d\r\n%s\r\n\0" as *const u8 as *const libc::c_char,
        s,
        &mut *((*cptr).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn controlc() {
    let mut sptr: *mut cstring = 0 as *mut cstring;
    let mut junk: [u_char; 10] = [0; 10];
    let mut s: libc::c_int = 0;
    sptr = junk.as_mut_ptr() as *mut cstring;
    s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
    if s < 0 as libc::c_int {
        ser(s);
    }
    memcpy(
        ((*sptr).buf).as_mut_ptr() as *mut libc::c_void,
        b"^C\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    (*sptr).buf[3 as libc::c_int as usize] = '\0' as i32 as u_char;
    (*sptr).len = 2 as libc::c_int as u_short;
    s = SQ_Write(sptr);
    if s < 0 as libc::c_int {
        ser(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn INIT_Run(
    mut file: *mut libc::c_char,
    mut env: *mut libc::c_char,
    mut cmd: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dbfd: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut env_num: libc::c_int = 1 as libc::c_int;
    let mut tmp: var_u = VAR_U { var_q: 0 };
    let mut uci_ptr: *mut uci_tab = 0 as *mut uci_tab;
    let mut volnam: *mut u_char = 0 as *mut u_char;
    let mut pid: libc::c_int = 0;
    let mut ssp: libc::c_int = 0 as libc::c_int;
    let mut asp: libc::c_int = 0 as libc::c_int;
    let mut var: *mut mvar = 0 as *mut mvar;
    let mut cptr: *mut cstring = 0 as *mut cstring;
    let mut sptr: *mut cstring = 0 as *mut cstring;
    let mut s: libc::c_int = 0;
    let mut start_type: u_char = 1 as libc::c_int as u_char;
    let mut gidset: [gid_t; 32] = [0; 32];
    '_start: loop {
        srandomdev();
        partab.jobtab = 0 as *mut libc::c_void as *mut jobtab;
        if start_type as libc::c_int == 1 as libc::c_int {
            dbfd = open(file, 0 as libc::c_int);
            if dbfd == -(1 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"RSM database error - %s\n\0" as *const u8 as *const libc::c_char,
                    file,
                );
                return *__error();
            }
            i = UTIL_Share(file);
            if i != 0 as libc::c_int {
                fprintf(
                    __stderrp,
                    b"RSM environment is not initialized.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return i;
            }
        }
        if ((*systab).vol[0 as libc::c_int as usize]).is_null() {
            fprintf(
                __stderrp,
                b"Error occurred in process - Environment does not match runtime image version\n\0"
                    as *const u8 as *const libc::c_char,
            );
            ret = -(1 as libc::c_int);
            break;
        } else {
            if !env.is_null() {
                env_num = 0 as libc::c_int;
                uci_ptr = &mut *((*(**((*systab).vol)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .vollab)
                    .uci)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut uci_tab;
                let mut var_i: u_int = 0 as libc::c_int as u_int;
                while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    tmp.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
                    var_i = var_i.wrapping_add(1);
                    var_i;
                }
                i = 0 as libc::c_int;
                while i < 32 as libc::c_int {
                    if *env.offset(i as isize) as libc::c_int == '\0' as i32 {
                        break;
                    }
                    tmp.var_cu[i as usize] = *env.offset(i as isize) as u_char;
                    i += 1;
                    i;
                }
                i = 0 as libc::c_int;
                while i < 64 as libc::c_int {
                    if var_equal((*uci_ptr.offset(i as isize)).name, tmp) != 0 {
                        env_num = i + 1 as libc::c_int;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                if env_num == 0 as libc::c_int {
                    ret = 2 as libc::c_int;
                    break;
                }
            }
            pid = getpid();
            let mut j: u_int = 0 as libc::c_int as u_int;
            while j < (*systab).maxjob {
                ret = (*((*systab).jobtab).offset(j as isize)).pid;
                if !(ret != pid && ret != 0) {
                    break;
                }
                if kill(ret, 0 as libc::c_int) != 0 {
                    if *__error() == 3 as libc::c_int {
                        CleanJob(
                            j.wrapping_add(1 as libc::c_int as u_int) as libc::c_int,
                        );
                        break;
                    }
                }
                j = j.wrapping_add(1);
                j;
            }
            ret = SemOp(
                0 as libc::c_int,
                ((*systab).maxjob).wrapping_neg() as libc::c_int,
            ) as libc::c_int;
            if ret < 0 as libc::c_int {
                break;
            }
            let mut j_0: u_int = 0 as libc::c_int as u_int;
            while j_0 < (*systab).maxjob {
                if (*((*systab).jobtab).offset(j_0 as isize)).pid == 0 as libc::c_int
                    && start_type as libc::c_int == 1 as libc::c_int
                    || (*((*systab).jobtab).offset(j_0 as isize)).pid == pid
                        && start_type as libc::c_int == 2 as libc::c_int
                {
                    memset(
                        &mut *((*systab).jobtab).offset(j_0 as isize) as *mut jobtab
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<jobtab>() as libc::c_ulong,
                    );
                    partab
                        .jobtab = &mut *((*systab).jobtab).offset(j_0 as isize)
                        as *mut jobtab;
                    (*partab.jobtab).pid = pid;
                    break;
                } else {
                    j_0 = j_0.wrapping_add(1);
                    j_0;
                }
            }
            ret = SemOp(0 as libc::c_int, (*systab).maxjob as libc::c_int)
                as libc::c_int;
            if (partab.jobtab).is_null() {
                ret = 12 as libc::c_int;
                break;
            } else {
                (*partab.jobtab).user = getuid() as libc::c_int;
                if (*partab.jobtab).user == (*systab).start_user
                    || (*partab.jobtab).user == 0 as libc::c_int
                {
                    (*partab.jobtab).priv_0 = 1 as libc::c_int as libc::c_short;
                } else if (*systab).maxjob == 1 as libc::c_int as u_int {
                    ret = 12 as libc::c_int;
                    partab.jobtab = 0 as *mut jobtab;
                    break;
                } else {
                    i = getgroups(32 as libc::c_int, gidset.as_mut_ptr());
                    if i < 0 as libc::c_int {
                        ret = *__error();
                        break;
                    } else {
                        while i > 0 as libc::c_int {
                            if gidset[(i - 1 as libc::c_int) as usize]
                                == 80 as libc::c_int as gid_t
                            {
                                (*partab.jobtab).priv_0 = 1 as libc::c_int as libc::c_short;
                                break;
                            } else {
                                i -= 1;
                                i;
                            }
                        }
                    }
                }
                (*partab.jobtab).precision = (*systab).precision as libc::c_short;
                (*partab.jobtab).uci = env_num as u_char;
                (*partab.jobtab).vol = 1 as libc::c_int as u_char;
                (*partab.jobtab).luci = env_num as u_char;
                (*partab.jobtab).lvol = 1 as libc::c_int as u_char;
                (*partab.jobtab).ruci = env_num as u_char;
                (*partab.jobtab).rvol = 1 as libc::c_int as u_char;
                (*partab.jobtab)
                    .start_len = Vhorolog(((*partab.jobtab).start_dh).as_mut_ptr());
                (*partab.jobtab)
                    .dostk[0 as libc::c_int as usize]
                    .type_0 = 1 as libc::c_int as u_char;
                failed_tty = tcgetattr(0 as libc::c_int, &mut tty_settings);
                i = SQ_Init() as libc::c_int;
                if i < 0 as libc::c_int {
                    ser(i);
                }
                i = 0 as libc::c_int;
                while i < 1 as libc::c_int {
                    if !((*systab).vol[i as usize]).is_null() {
                        *((*systab).last_blk_used)
                            .as_mut_ptr()
                            .offset(
                                ((partab.jobtab).offset_from((*systab).jobtab)
                                    as libc::c_long
                                    + ((*systab).maxjob * i as u_int) as libc::c_long) as isize,
                            ) = 0 as libc::c_int as u_int;
                    }
                    i += 1;
                    i;
                }
                partab.debug = 0 as libc::c_int;
                partab
                    .strstk_start = &mut *strstk
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut u_char;
                partab
                    .strstk_last = &mut *strstk
                    .as_mut_ptr()
                    .offset((1048576 as libc::c_int * 2 as libc::c_int) as isize)
                    as *mut u_char;
                partab.varlst = 0 as *mut var_u;
                partab.vol_fds[0 as libc::c_int as usize] = dbfd;
                ST_Init();
                i = 0 as libc::c_int;
                while i < 1 as libc::c_int {
                    if !((*systab).vol[i as usize]).is_null() {
                        if (*(*(*systab).vol[i as usize]).vollab).journal_available
                            as libc::c_int != 0
                            && (*(*(*systab).vol[i as usize]).vollab).journal_requested
                                as libc::c_int != 0
                        {
                            partab
                                .jnl_fds[i
                                as usize] = open(
                                ((*(*(*systab).vol[i as usize]).vollab).journal_file)
                                    .as_mut_ptr(),
                                0x2 as libc::c_int,
                            );
                            if partab.jnl_fds[i as usize] == -(1 as libc::c_int) {
                                fprintf(
                                    __stderrp,
                                    b"Failed to open journal file: %s\r\nerrno = %d\r\n\0"
                                        as *const u8 as *const libc::c_char,
                                    ((*(*(*systab).vol[i as usize]).vollab).journal_file)
                                        .as_mut_ptr(),
                                    *__error(),
                                );
                                ret = *__error();
                                if !cmd.is_null() {
                                    break '_start;
                                }
                            }
                        }
                    }
                    i += 1;
                    i;
                }
                if start_type as libc::c_int == 1 as libc::c_int {
                    i = 1 as libc::c_int;
                    while i < 1 as libc::c_int {
                        if !((*systab).vol[i as usize]).is_null() {
                            dbfd = open(
                                ((*(*systab).vol[i as usize]).file_name).as_mut_ptr(),
                                0 as libc::c_int,
                            );
                            if dbfd == -(1 as libc::c_int) {
                                fprintf(
                                    __stderrp,
                                    b"RSM database error - %s\r\nerrno = %d\r\n\0" as *const u8
                                        as *const libc::c_char,
                                    ((*(*systab).vol[i as usize]).file_name).as_mut_ptr(),
                                    *__error(),
                                );
                                ret = *__error();
                                if !cmd.is_null() {
                                    break '_start;
                                }
                            }
                            partab.vol_fds[i as usize] = dbfd;
                            if (*(*(*systab).vol[i as usize]).vollab).journal_available
                                as libc::c_int != 0
                                && (*(*(*systab).vol[i as usize]).vollab).journal_requested
                                    as libc::c_int != 0
                            {
                                partab
                                    .jnl_fds[i
                                    as usize] = open(
                                    ((*(*(*systab).vol[i as usize]).vollab).journal_file)
                                        .as_mut_ptr(),
                                    0x2 as libc::c_int,
                                );
                                if partab.jnl_fds[i as usize] == -(1 as libc::c_int) {
                                    fprintf(
                                        __stderrp,
                                        b"Failed to open journal file: %s\r\nerrno = %d\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                        ((*(*(*systab).vol[i as usize]).vollab).journal_file)
                                            .as_mut_ptr(),
                                        *__error(),
                                    );
                                    ret = *__error();
                                    if !cmd.is_null() {
                                        break '_start;
                                    }
                                }
                            }
                        }
                        i += 1;
                        i;
                    }
                }
                if !cmd.is_null() {
                    source_ptr = cmd as *mut u_char;
                    if start_type as libc::c_int == 1 as libc::c_int {
                        sptr = &mut *strstk
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut u_char
                            as *mut cstring;
                        (*sptr).len = strlen(cmd) as u_short;
                        memcpy(
                            ((*sptr).buf).as_mut_ptr() as *mut libc::c_void,
                            source_ptr as *const libc::c_void,
                            (*sptr).len as libc::c_ulong,
                        );
                        let fresh0 = asp;
                        asp = asp + 1;
                        addstk[fresh0 as usize] = sptr as *mut u_char;
                        ssp = ((*sptr).len as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<u_short>() as libc::c_ulong,
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_int;
                    } else {
                        let fresh1 = asp;
                        asp = asp + 1;
                        addstk[fresh1
                            as usize] = &mut *strstk
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut u_char;
                    }
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    comp_ptr = ((*cptr).buf).as_mut_ptr();
                    parse();
                    let fresh2 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh2 = 157 as libc::c_int as u_char;
                    let fresh3 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh3 = 0 as libc::c_int as u_char;
                    let fresh4 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh4 = 0 as libc::c_int as u_char;
                    i = (&mut *comp_ptr.offset(0 as libc::c_int as isize) as *mut u_char)
                        .offset_from(
                            &mut *((*cptr).buf)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut u_char,
                        ) as libc::c_long as libc::c_int;
                    (*cptr).len = i as u_short;
                    ssp = ((ssp + i) as libc::c_ulong)
                        .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                    rsmpc = &mut *((*cptr).buf)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut u_char;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .routine = cmd as *mut u_char;
                    (*partab.jobtab).dostk[0 as libc::c_int as usize].pc = rsmpc;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .symbol = 0 as *mut libc::c_short;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .newtab = 0 as *mut u_char;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .endlin = rsmpc
                        .offset(i as isize)
                        .offset(-(4 as libc::c_int as isize));
                    let mut var_i_0: u_int = 0 as libc::c_int as u_int;
                    while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .rounam
                            .var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
                        var_i_0 = var_i_0.wrapping_add(1);
                        var_i_0;
                    }
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .vol = (*partab.jobtab).vol;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .uci = (*partab.jobtab).uci;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .line_num = 0 as libc::c_int as u_short;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .type_0 = start_type;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .estack = 0 as libc::c_int as u_char;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .level = 0 as libc::c_int as u_char;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .flags = 0 as libc::c_int as u_char;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .savasp = asp as libc::c_long;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .savssp = ssp as libc::c_long;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .asp = asp as libc::c_long;
                    (*partab.jobtab)
                        .dostk[0 as libc::c_int as usize]
                        .ssp = ssp as libc::c_long;
                    (*partab.jobtab).attention = 0 as libc::c_int;
                    (*partab.jobtab).trap = 0 as libc::c_int as u_int;
                    (*partab.jobtab).async_error = 0 as libc::c_int as libc::c_short;
                    isp = 0 as libc::c_int as libc::c_long;
                    s = run(asp, ssp) as libc::c_int;
                    if s == 1 as libc::c_int {
                        break;
                    }
                    if !(s == 512 as libc::c_int) {
                        (*partab.jobtab).io = 0 as libc::c_int as u_char;
                        var = &mut *strstk.as_mut_ptr().offset(0 as libc::c_int as isize)
                            as *mut u_char as *mut mvar;
                        let mut var_i_1: u_int = 0 as libc::c_int as u_int;
                        while var_i_1 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                            (*var)
                                .name
                                .var_qu[var_i_1 as usize] = 0 as libc::c_int as u_int64;
                            var_i_1 = var_i_1.wrapping_add(1);
                            var_i_1;
                        }
                        memcpy(
                            &mut *((*var).name.var_cu)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut u_char
                                as *mut libc::c_void,
                            b"$ECODE\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            6 as libc::c_int as libc::c_ulong,
                        );
                        (*var).volset = 0 as libc::c_int as u_char;
                        (*var).uci = 255 as libc::c_int as u_char;
                        (*var).slen = 0 as libc::c_int as u_char;
                        cptr = &mut *strstk
                            .as_mut_ptr()
                            .offset(
                                ::core::mem::size_of::<mvar>() as libc::c_ulong as isize,
                            ) as *mut u_char as *mut cstring;
                        memcpy(
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_void,
                            b"$ECODE=\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            7 as libc::c_int as libc::c_ulong,
                        );
                        s = ST_Get(
                            var,
                            &mut *((*cptr).buf)
                                .as_mut_ptr()
                                .offset(7 as libc::c_int as isize),
                        );
                        if s > 1 as libc::c_int {
                            (*cptr).len = (s + 7 as libc::c_int) as u_short;
                            s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                            if s < 0 as libc::c_int {
                                ser(s);
                            }
                            s = SQ_Write(cptr);
                            if s < 0 as libc::c_int {
                                ser(s);
                            }
                            s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                            if s < 0 as libc::c_int {
                                ser(s);
                            }
                            cptr = (cptr as *mut u_char)
                                .offset(8 as libc::c_int as isize) as *mut cstring;
                            if (*cptr).buf[0 as libc::c_int as usize] as libc::c_int
                                != 'U' as i32
                            {
                                (*cptr).len = 4 as libc::c_int as u_short;
                                (*cptr)
                                    .len = Xcall_errmsg(
                                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                                    cptr,
                                    cptr,
                                ) as u_short;
                                s = SQ_Write(cptr);
                                if s < 0 as libc::c_int {
                                    ser(s);
                                }
                                s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                                if s < 0 as libc::c_int {
                                    ser(s);
                                }
                            }
                            ret = 1 as libc::c_int;
                        }
                        break;
                    }
                } else {
                    loop {
                        if in_hist as libc::c_int == 0 as libc::c_int {
                            sptr = &mut *strstk
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut u_char
                                as *mut cstring;
                            asp = 0 as libc::c_int;
                            ssp = 0 as libc::c_int;
                            volnam = ((*(*(*systab)
                                .vol[((*partab.jobtab).vol as libc::c_int
                                - 1 as libc::c_int) as usize])
                                .vollab)
                                .volnam
                                .var_cu)
                                .as_mut_ptr();
                            uci_ptr = &mut *((*(**((*systab).vol)
                                .as_mut_ptr()
                                .offset(
                                    ((*partab.jobtab).vol as libc::c_int - 1 as libc::c_int)
                                        as isize,
                                ))
                                .vollab)
                                .uci)
                                .as_mut_ptr()
                                .offset(
                                    ((*partab.jobtab).uci as libc::c_int - 1 as libc::c_int)
                                        as isize,
                                ) as *mut uci_tab;
                            (*sptr)
                                .len = (strlen(volnam as *mut libc::c_char))
                                .wrapping_add(
                                    strlen(
                                        ((*uci_ptr).name.var_cu).as_mut_ptr() as *mut libc::c_char,
                                    ),
                                )
                                .wrapping_add(9 as libc::c_int as libc::c_ulong) as u_short;
                            prompt_len = (*sptr).len;
                            if snprintf(
                                ((*sptr).buf).as_mut_ptr() as *mut libc::c_char,
                                ((*sptr).len as libc::c_int + 1 as libc::c_int)
                                    as libc::c_ulong,
                                b"RSM [%s,%s]> \0" as *const u8 as *const libc::c_char,
                                ((*uci_ptr).name.var_cu).as_mut_ptr(),
                                volnam,
                            ) < 0 as libc::c_int
                            {
                                return *__error();
                            }
                            (*partab.jobtab).io = 0 as libc::c_int as u_char;
                            if (*partab.jobtab).seqio[0 as libc::c_int as usize].dx != 0
                            {
                                s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                                if s < 0 as libc::c_int {
                                    ser(s);
                                }
                            }
                            s = SQ_Write(sptr);
                            if s < 0 as libc::c_int {
                                ser(s);
                            }
                        }
                        s = SQ_Read(
                            ((*sptr).buf).as_mut_ptr(),
                            -(1 as libc::c_int),
                            -(1 as libc::c_int),
                        );
                        i = attention() as libc::c_int;
                        if i == 1 as libc::c_int {
                            break '_start;
                        }
                        if i == -(51 as libc::c_int + 200 as libc::c_int) {
                            controlc();
                        }
                        if s < 0 as libc::c_int {
                            ser(s);
                            s = 0 as libc::c_int;
                        }
                        (*sptr).len = s as u_short;
                        if s == 0 as libc::c_int {
                            continue;
                        }
                        if hist_next == 0
                            || strcmp(
                                (history[(hist_next as libc::c_int - 1 as libc::c_int)
                                    as usize])
                                    .as_mut_ptr(),
                                ((*sptr).buf).as_mut_ptr() as *mut libc::c_char,
                            ) != 0
                        {
                            strcpy(
                                (history[hist_next as usize]).as_mut_ptr(),
                                ((*sptr).buf).as_mut_ptr() as *mut libc::c_char,
                            );
                            if hist_next as libc::c_int
                                == 128 as libc::c_int - 1 as libc::c_int
                            {
                                hist_next = 0 as libc::c_int as u_short;
                            } else {
                                hist_next = hist_next.wrapping_add(1);
                                hist_next;
                            }
                        }
                        hist_curr = hist_next;
                        let fresh5 = asp;
                        asp = asp + 1;
                        addstk[fresh5 as usize] = sptr as *mut u_char;
                        ssp = ((ssp + s) as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<u_short>() as libc::c_ulong,
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_int;
                        s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                        if s < 0 as libc::c_int {
                            ser(s);
                        }
                        source_ptr = ((*sptr).buf).as_mut_ptr();
                        cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut cstring;
                        comp_ptr = ((*cptr).buf).as_mut_ptr();
                        parse();
                        let fresh6 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh6 = 157 as libc::c_int as u_char;
                        let fresh7 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh7 = 0 as libc::c_int as u_char;
                        let fresh8 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh8 = 0 as libc::c_int as u_char;
                        i = (&mut *comp_ptr.offset(0 as libc::c_int as isize)
                            as *mut u_char)
                            .offset_from(
                                &mut *((*cptr).buf)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as *mut u_char,
                            ) as libc::c_long as libc::c_int;
                        (*cptr).len = i as u_short;
                        ssp = ((ssp + i) as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<u_short>() as libc::c_ulong,
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_int;
                        rsmpc = &mut *((*cptr).buf)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut u_char;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .routine = ((*sptr).buf).as_mut_ptr();
                        (*partab.jobtab).dostk[0 as libc::c_int as usize].pc = rsmpc;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .symbol = 0 as *mut libc::c_short;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .newtab = 0 as *mut u_char;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .endlin = rsmpc
                            .offset(i as isize)
                            .offset(-(4 as libc::c_int as isize));
                        let mut var_i_2: u_int = 0 as libc::c_int as u_int;
                        while var_i_2 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                            (*partab.jobtab)
                                .dostk[0 as libc::c_int as usize]
                                .rounam
                                .var_qu[var_i_2 as usize] = 0 as libc::c_int as u_int64;
                            var_i_2 = var_i_2.wrapping_add(1);
                            var_i_2;
                        }
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .vol = (*partab.jobtab).vol;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .uci = (*partab.jobtab).uci;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .line_num = 0 as libc::c_int as u_short;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .type_0 = 1 as libc::c_int as u_char;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .estack = 0 as libc::c_int as u_char;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .level = 0 as libc::c_int as u_char;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .flags = 0 as libc::c_int as u_char;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .savasp = asp as libc::c_long;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .savssp = ssp as libc::c_long;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .asp = asp as libc::c_long;
                        (*partab.jobtab)
                            .dostk[0 as libc::c_int as usize]
                            .ssp = ssp as libc::c_long;
                        (*partab.jobtab).attention = 0 as libc::c_int;
                        (*partab.jobtab).trap = 0 as libc::c_int as u_int;
                        (*partab.jobtab).async_error = 0 as libc::c_int as libc::c_short;
                        isp = 0 as libc::c_int as libc::c_long;
                        s = run(asp, ssp) as libc::c_int;
                        if s == 512 as libc::c_int {
                            break;
                        }
                        if s == 1 as libc::c_int {
                            break '_start;
                        }
                        (*partab.jobtab).io = 0 as libc::c_int as u_char;
                        if s == -(51 as libc::c_int + 200 as libc::c_int) {
                            controlc();
                        } else if s < 0 as libc::c_int {
                            ser(s);
                        }
                        (*partab.jobtab).error_frame = 0 as libc::c_int as libc::c_short;
                        var = &mut *strstk.as_mut_ptr().offset(0 as libc::c_int as isize)
                            as *mut u_char as *mut mvar;
                        let mut var_i_3: u_int = 0 as libc::c_int as u_int;
                        while var_i_3 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                            (*var)
                                .name
                                .var_qu[var_i_3 as usize] = 0 as libc::c_int as u_int64;
                            var_i_3 = var_i_3.wrapping_add(1);
                            var_i_3;
                        }
                        memcpy(
                            &mut *((*var).name.var_cu)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut u_char
                                as *mut libc::c_void,
                            b"$ECODE\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            6 as libc::c_int as libc::c_ulong,
                        );
                        (*var).volset = 0 as libc::c_int as u_char;
                        (*var).uci = 255 as libc::c_int as u_char;
                        (*var).slen = 0 as libc::c_int as u_char;
                        cptr = &mut *strstk
                            .as_mut_ptr()
                            .offset(
                                ::core::mem::size_of::<mvar>() as libc::c_ulong as isize,
                            ) as *mut u_char as *mut cstring;
                        memcpy(
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_void,
                            b"$ECODE=\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            7 as libc::c_int as libc::c_ulong,
                        );
                        s = ST_Get(
                            var,
                            &mut *((*cptr).buf)
                                .as_mut_ptr()
                                .offset(7 as libc::c_int as isize),
                        );
                        if s < 1 as libc::c_int {
                            continue;
                        }
                        (*cptr).len = (s + 7 as libc::c_int) as u_short;
                        s = SQ_Write(cptr);
                        if s < 0 as libc::c_int {
                            ser(s);
                        }
                        s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                        if s < 0 as libc::c_int {
                            ser(s);
                        }
                        ST_Kill(var);
                        cptr = (cptr as *mut u_char).offset(8 as libc::c_int as isize)
                            as *mut cstring;
                        if (*cptr).buf[0 as libc::c_int as usize] as libc::c_int
                            != 'U' as i32
                        {
                            (*cptr).len = 4 as libc::c_int as u_short;
                            (*cptr)
                                .len = Xcall_errmsg(
                                ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                                cptr,
                                cptr,
                            ) as u_short;
                            s = SQ_Write(cptr);
                            if s < 0 as libc::c_int {
                                ser(s);
                            }
                            s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                            if s < 0 as libc::c_int {
                                ser(s);
                            }
                        }
                    }
                }
                start_type = 2 as libc::c_int as u_char;
                env_num = (*partab.jobtab).ruci as libc::c_int;
                cmd = &mut *strstk.as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_char;
                ssp = strlen(strstk.as_mut_ptr() as *const libc::c_char) as libc::c_int;
                isp = 0 as libc::c_int as libc::c_long;
                asp = 0 as libc::c_int;
                ret = 0 as libc::c_int;
                env = 0 as *mut libc::c_char;
            }
        }
    }
    if !(partab.jobtab).is_null() {
        CleanJob(0 as libc::c_int);
    }
    shmdt(systab as *const libc::c_void);
    i = 0 as libc::c_int;
    while i < 1 as libc::c_int {
        if partab.vol_fds[i as usize] != 0 {
            close(partab.vol_fds[i as usize]);
        }
        i += 1;
        i;
    }
    if failed_tty == 0 {
        failed_tty = tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut tty_settings);
    }
    if start_type as libc::c_int == 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    return ret;
}
