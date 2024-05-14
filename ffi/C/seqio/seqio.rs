#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type __sFILEX;
    pub type GBD;
    pub type RBD;
    fn __error() -> *mut libc::c_int;
    fn fstat(_: libc::c_int, _: *mut stat) -> libc::c_int;
    fn stat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    fn ioctl(_: libc::c_int, _: libc::c_ulong, _: ...) -> libc::c_int;
    fn getpeername(_: libc::c_int, _: *mut sockaddr, _: *mut socklen_t) -> libc::c_int;
    fn inet_ntoa(_: in_addr) -> *mut libc::c_char;
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn atoi(_: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn tcgetattr(_: libc::c_int, _: *mut termios) -> libc::c_int;
    fn tcsetattr(_: libc::c_int, _: libc::c_int, _: *const termios) -> libc::c_int;
    fn tcflush(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn alarm(_: libc::c_uint) -> libc::c_uint;
    fn close(_: libc::c_int) -> libc::c_int;
    fn isatty(_: libc::c_int) -> libc::c_int;
    fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn ttyname(_: libc::c_int) -> *mut libc::c_char;
    fn unlink(_: *const libc::c_char) -> libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn uitocstring(buf: *mut u_char, n: u_int) -> u_short;
    fn ForkIt(cft: libc::c_int) -> libc::c_int;
    fn UTIL_strerror(err: libc::c_int, buf: *mut u_char) -> u_short;
    fn setSignals() -> libc::c_int;
    fn getError(type_0: libc::c_int, errnum: libc::c_int) -> libc::c_int;
    fn SQ_File_Open(file: *mut libc::c_char, op: libc::c_int) -> libc::c_int;
    fn SQ_File_Write(
        fid: libc::c_int,
        writebuf: *mut u_char,
        nbytes: libc::c_int,
    ) -> libc::c_int;
    fn SQ_File_Read(fid: libc::c_int, readbuf: *mut u_char) -> libc::c_int;
    fn SQ_Device_Open(device: *mut libc::c_char, op: libc::c_int) -> libc::c_int;
    fn SQ_Device_Write(
        did: libc::c_int,
        writebuf: *mut u_char,
        nbytes: libc::c_int,
    ) -> libc::c_int;
    fn SQ_Device_Read(
        did: libc::c_int,
        readbuf: *mut u_char,
        tout: libc::c_int,
    ) -> libc::c_int;
    fn SQ_Pipe_Open(pipe: *mut libc::c_char, op: libc::c_int) -> libc::c_int;
    fn SQ_Pipe_Close(pid: libc::c_int, pipe: *mut libc::c_char) -> libc::c_int;
    fn SQ_Pipe_Read(
        pid: libc::c_int,
        readbuf: *mut u_char,
        tout: libc::c_int,
    ) -> libc::c_int;
    fn SQ_Pipe_Write(
        pid: libc::c_int,
        writebuf: *mut u_char,
        nbytes: libc::c_int,
    ) -> libc::c_int;
    fn SQ_Tcpip_Open(bind: *mut libc::c_char, op: libc::c_int) -> libc::c_int;
    fn SQ_Tcpip_Write(
        sid: libc::c_int,
        writebuf: *mut u_char,
        nbytes: libc::c_int,
    ) -> libc::c_int;
    fn SQ_Tcpip_Accept(sid: libc::c_int, tout: libc::c_int) -> libc::c_int;
    fn SQ_Tcpip_Read(
        sid: libc::c_int,
        readbuf: *mut u_char,
        tout: libc::c_int,
    ) -> libc::c_int;
    static mut history: [[libc::c_char; 65534]; 128];
    static mut hist_next: u_short;
    static mut hist_curr: u_short;
    static mut in_hist: libc::c_short;
    static mut prompt_len: u_short;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __uint64_t = libc::c_ulonglong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_uid_t = __uint32_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type dev_t = __darwin_dev_t;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type in_addr_t = __uint32_t;
pub type in_port_t = __uint16_t;
pub type mode_t = __darwin_mode_t;
pub type nlink_t = __uint16_t;
pub type off_t = __darwin_off_t;
pub type uid_t = __darwin_uid_t;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type sa_family_t = __uint8_t;
pub type socklen_t = __darwin_socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_len: __uint8_t,
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_len: __uint8_t,
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_char; 8],
}
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
unsafe extern "C" fn _OSSwapInt16(mut _data: __uint16_t) -> __uint16_t {
    return ((_data as libc::c_int) << 8 as libc::c_int
        | _data as libc::c_int >> 8 as libc::c_int) as __uint16_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut _c: libc::c_int) -> libc::c_int {
    return __tolower(_c);
}
static mut MASK: [u_int64; 64] = [0; 64];
static mut CRLF: u_int64 = 0;
#[no_mangle]
pub static mut proto_family: libc::c_short = 2 as libc::c_int as libc::c_short;
#[no_mangle]
pub static mut addr_family: libc::c_short = 2 as libc::c_int as libc::c_short;
#[no_mangle]
pub static mut sock_type: libc::c_short = 1 as libc::c_int as libc::c_short;
#[no_mangle]
pub static mut sock_proto: libc::c_short = 6 as libc::c_int as libc::c_short;
#[no_mangle]
pub unsafe extern "C" fn SQ_Init() -> libc::c_short {
    let mut index: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut typ: libc::c_int = 0;
    index = 0 as libc::c_int;
    while index < 64 as libc::c_int {
        MASK[index as usize] = ((1 as libc::c_ulong) << index) as u_int64;
        index += 1;
        index;
    }
    mask = ((1 as libc::c_uint) << 13 as libc::c_int) as libc::c_int;
    CRLF = mask as u_int64;
    mask = ((1 as libc::c_uint) << 10 as libc::c_int) as libc::c_int;
    CRLF |= mask as u_int64;
    ret = setSignals();
    if ret < 0 as libc::c_int {
        return ret as libc::c_short;
    }
    ret = getObjectMode(0 as libc::c_int);
    if ret < 0 as libc::c_int {
        return ret as libc::c_short;
    }
    typ = getModeCategory(ret);
    if typ < 0 as libc::c_int {
        return typ as libc::c_short;
    }
    ret = initObject(0 as libc::c_int, typ);
    if ret < 0 as libc::c_int {
        return ret as libc::c_short;
    }
    if typ == 2 as libc::c_int {
        let mut len: libc::c_int = 0;
        let mut sin: sockaddr_in = sockaddr_in {
            sin_len: 0,
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        let mut s: *mut servertab = 0 as *mut servertab;
        s = &mut (*((*partab.jobtab).seqio)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .s;
        ret = openSERVER(
            0 as libc::c_int,
            b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if ret < 0 as libc::c_int {
            return ret as libc::c_short;
        }
        (*s).cid = 0 as libc::c_int;
        len = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_int;
        ret = getpeername(
            (*s).cid,
            &mut sin as *mut sockaddr_in as *mut sockaddr,
            &mut len as *mut libc::c_int as *mut socklen_t,
        );
        if ret == -(1 as libc::c_int) {
            return getError(0 as libc::c_int, *__error()) as libc::c_short;
        }
        len = ::core::mem::size_of::<in_addr>() as libc::c_ulong as libc::c_int;
        snprintf(
            ((*s).name).as_mut_ptr() as *mut libc::c_char,
            256 as libc::c_int as libc::c_ulong,
            b"%s %u\0" as *const u8 as *const libc::c_char,
            inet_ntoa(sin.sin_addr),
            (if 0 != 0 {
                ((sin.sin_port as libc::c_uint & 0xff00 as libc::c_uint)
                    >> 8 as libc::c_int
                    | (sin.sin_port as libc::c_uint & 0xff as libc::c_uint)
                        << 8 as libc::c_int) as __uint16_t as libc::c_int
            } else {
                _OSSwapInt16(sin.sin_port) as libc::c_int
            }) as __uint16_t as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_short;
    }
    if isatty(0 as libc::c_int) != 0 {
        snprintf(
            ((*partab.jobtab).seqio[0 as libc::c_int as usize].name).as_mut_ptr()
                as *mut libc::c_char,
            256 as libc::c_int as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            ttyname(0 as libc::c_int),
        );
    } else {
        snprintf(
            ((*partab.jobtab).seqio[0 as libc::c_int as usize].name).as_mut_ptr()
                as *mut libc::c_char,
            256 as libc::c_int as libc::c_ulong,
            b"Not a tty\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Open(
    mut chan: libc::c_int,
    mut object: *mut cstring,
    mut op: *mut cstring,
    mut tout: libc::c_int,
) -> libc::c_short {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut oper: libc::c_int = 0;
    let mut ford: libc::c_int = 0;
    let mut obj: libc::c_int = 0;
    let mut oid: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if chan == 0 as libc::c_int {
        if tout > -(1 as libc::c_int) {
            (*partab.jobtab).test = 1 as libc::c_int as u_char;
        }
        return 0 as libc::c_int as libc::c_short;
    }
    if isChan(chan) == 0 as libc::c_int {
        return getError(1 as libc::c_int, 25 as libc::c_int) as libc::c_short;
    }
    if isChanFree(chan) == 0 as libc::c_int {
        return getError(1 as libc::c_int, 26 as libc::c_int) as libc::c_short;
    }
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    ret = checkCstring(object);
    if ret < 0 as libc::c_int {
        return ret as libc::c_short;
    }
    ret = checkCstring(op);
    if ret < 0 as libc::c_int {
        return ret as libc::c_short;
    }
    if (*object).len as libc::c_int == 0 as libc::c_int {
        return getError(1 as libc::c_int, 28 as libc::c_int) as libc::c_short;
    }
    if tout < -(1 as libc::c_int) {
        return getError(1 as libc::c_int, 22 as libc::c_int) as libc::c_short;
    }
    if tout > -(1 as libc::c_int) {
        (*partab.jobtab).test = 1 as libc::c_int as u_char;
    }
    oper = getOperation(op);
    if oper < 0 as libc::c_int {
        return oper as libc::c_short
    } else if oper == 2 as libc::c_int || oper == 1 as libc::c_int
        || oper == 4 as libc::c_int
    {
        ford = getObjectType(((*object).buf).as_mut_ptr() as *mut libc::c_char);
        if ford < 0 as libc::c_int {
            return ford as libc::c_short
        } else if ford == 0 as libc::c_int && oper == 1 as libc::c_int {
            obj = 1 as libc::c_int;
        } else if ford == 2 as libc::c_int {
            obj = 4 as libc::c_int;
        } else if ford == 4 as libc::c_int {
            obj = 1 as libc::c_int;
        } else {
            return getError(1 as libc::c_int, 29 as libc::c_int) as libc::c_short
        }
    } else if oper == 3 as libc::c_int {
        obj = 1 as libc::c_int;
    } else if oper == 5 as libc::c_int {
        obj = 2 as libc::c_int;
    } else if oper == 6 as libc::c_int {
        obj = 2 as libc::c_int;
    } else if oper == 9 as libc::c_int {
        obj = 3 as libc::c_int;
    } else if oper == 10 as libc::c_int {
        obj = 3 as libc::c_int;
    } else {
        return getError(1 as libc::c_int, 29 as libc::c_int) as libc::c_short
    }
    if tout > 0 as libc::c_int && obj != 1 as libc::c_int {
        alarm(tout as libc::c_uint);
    }
    match obj {
        1 => {
            oid = SQ_File_Open(((*object).buf).as_mut_ptr() as *mut libc::c_char, oper);
        }
        4 => {
            oid = SQ_Device_Open(
                ((*object).buf).as_mut_ptr() as *mut libc::c_char,
                oper,
            );
        }
        3 => {
            oid = SQ_Pipe_Open(((*object).buf).as_mut_ptr() as *mut libc::c_char, oper);
        }
        2 => {
            oid = SQ_Tcpip_Open(((*object).buf).as_mut_ptr() as *mut libc::c_char, oper);
        }
        _ => return getError(1 as libc::c_int, 30 as libc::c_int) as libc::c_short,
    }
    alarm(0 as libc::c_int as libc::c_uint);
    if oid < 0 as libc::c_int {
        if tout == 0 as libc::c_int {
            (*partab.jobtab).test = 0 as libc::c_int as u_char;
        }
        if (*partab.jobtab).trap as u_int64 & MASK[14 as libc::c_int as usize] != 0 {
            (*partab.jobtab)
                .trap = ((*partab.jobtab).trap as u_int64
                & !MASK[14 as libc::c_int as usize]) as u_int;
            (*partab.jobtab).test = 0 as libc::c_int as u_char;
            return 0 as libc::c_int as libc::c_short;
        } else if (*partab.jobtab).trap as u_int64 & MASK[2 as libc::c_int as usize] != 0
        {
            return 0 as libc::c_int as libc::c_short
        } else {
            return oid as libc::c_short
        }
    }
    ret = initObject(chan, obj);
    if ret < 0 as libc::c_int {
        return ret as libc::c_short;
    }
    (*c).mode = oper as u_char;
    (*c).fid = oid;
    i = snprintf(
        ((*c).name).as_mut_ptr() as *mut libc::c_char,
        256 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        ((*object).buf).as_mut_ptr(),
    );
    if i < 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
    }
    if oper == 6 as libc::c_int {
        ret = openSERVER(chan, ((*op).buf).as_mut_ptr() as *mut libc::c_char);
        if ret < 0 as libc::c_int {
            return ret as libc::c_short;
        }
    }
    return oid as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Use(
    mut chan: libc::c_int,
    mut interm: *mut cstring,
    mut outerm: *mut cstring,
    mut par: libc::c_int,
) -> libc::c_short {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut ret: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    if isChan(chan) == 0 as libc::c_int {
        return getError(1 as libc::c_int, 25 as libc::c_int) as libc::c_short;
    }
    if isChanFree(chan) == 1 as libc::c_int {
        if chan == 0 as libc::c_int {
            return 0 as libc::c_int as libc::c_short
        } else {
            return getError(1 as libc::c_int, 27 as libc::c_int) as libc::c_short
        }
    }
    if !interm.is_null() {
        ret = checkAsciiChars(interm);
        if ret < 0 as libc::c_int {
            return ret as libc::c_short;
        }
    }
    if !outerm.is_null() {
        ret = checkCstring(outerm);
        if ret < 0 as libc::c_int {
            return ret as libc::c_short;
        }
        if (*outerm).len as libc::c_int > 6 as libc::c_int {
            return getError(1 as libc::c_int, 33 as libc::c_int) as libc::c_short;
        }
    }
    if par < 0 as libc::c_int {
        return getError(1 as libc::c_int, 45 as libc::c_int) as libc::c_short;
    }
    (*partab.jobtab).io = chan as u_char;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    if !interm.is_null() {
        if (*interm).len as libc::c_int != 0 as libc::c_int {
            (*c).in_term = getBitMask(interm, (*c).in_term);
            flag = 1 as libc::c_int;
        } else {
            (*c).in_term.interm[0 as libc::c_int as usize] = 0 as libc::c_int as u_int64;
            (*c).in_term.interm[1 as libc::c_int as usize] = 0 as libc::c_int as u_int64;
            flag = -(1 as libc::c_int);
        }
    } else {
        flag = 0 as libc::c_int;
    }
    (*c)
        .options = setOptionsBitMask((*c).options as libc::c_int, 0 as libc::c_int, flag)
        as u_char;
    if !outerm.is_null() {
        if (*outerm).len as libc::c_int != 0 as libc::c_int {
            (*c).out_len = (*outerm).len as libc::c_short;
            memcpy(
                ((*c).out_term).as_mut_ptr() as *mut libc::c_void,
                ((*outerm).buf).as_mut_ptr() as *const libc::c_void,
                (*outerm).len as libc::c_ulong,
            );
            flag = 1 as libc::c_int;
        } else {
            (*c).out_len = 0 as libc::c_int as libc::c_short;
            (*c).out_term[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            flag = -(1 as libc::c_int);
        }
    } else {
        flag = 0 as libc::c_int;
    }
    (*c)
        .options = setOptionsBitMask((*c).options as libc::c_int, 1 as libc::c_int, flag)
        as u_char;
    if chan == 0
        && par
            & (4096 as libc::c_int | 8192 as libc::c_int | 16384 as libc::c_int
                | 32768 as libc::c_int) != 0
    {
        let mut settings: termios = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_cc: [0; 20],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        ret = tcgetattr(0 as libc::c_int, &mut settings);
        if ret == -(1 as libc::c_int) {
            return getError(0 as libc::c_int, *__error()) as libc::c_short;
        }
        if par & 4096 as libc::c_int != 0 {
            settings.c_cc[8 as libc::c_int as usize] = '\u{3}' as i32 as cc_t;
        }
        if par & 8192 as libc::c_int != 0 {
            settings.c_cc[8 as libc::c_int as usize] = -1i32 as libc::c_uchar;
        }
        if par & 16384 as libc::c_int != 0 {
            settings.c_cc[9 as libc::c_int as usize] = '\u{14}' as i32 as cc_t;
        }
        if par & 32768 as libc::c_int != 0 {
            settings.c_cc[9 as libc::c_int as usize] = -1i32 as libc::c_uchar;
        }
        ret = tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut settings);
        if ret == -(1 as libc::c_int) {
            return getError(0 as libc::c_int, *__error()) as libc::c_short;
        }
    }
    if par & 4 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
        ) as u_char;
    } else if par & 16 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            2 as libc::c_int,
            -(1 as libc::c_int),
        ) as u_char;
    } else {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            2 as libc::c_int,
            0 as libc::c_int,
        ) as u_char;
    }
    if par & 1 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            3 as libc::c_int,
            1 as libc::c_int,
        ) as u_char;
    } else if par & 2 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            3 as libc::c_int,
            -(1 as libc::c_int),
        ) as u_char;
    } else {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            3 as libc::c_int,
            0 as libc::c_int,
        ) as u_char;
    }
    if par & 512 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            4 as libc::c_int,
            1 as libc::c_int,
        ) as u_char;
    } else if par & 2048 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            4 as libc::c_int,
            1 as libc::c_int,
        ) as u_char;
    } else if par & 256 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            4 as libc::c_int,
            -(1 as libc::c_int),
        ) as u_char;
    } else {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            4 as libc::c_int,
            0 as libc::c_int,
        ) as u_char;
    }
    if par & 1024 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            5 as libc::c_int,
            1 as libc::c_int,
        ) as u_char;
    } else if par & 2048 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            5 as libc::c_int,
            1 as libc::c_int,
        ) as u_char;
    } else if par & 256 as libc::c_int != 0 {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            5 as libc::c_int,
            -(1 as libc::c_int),
        ) as u_char;
    } else {
        (*c)
            .options = setOptionsBitMask(
            (*c).options as libc::c_int,
            5 as libc::c_int,
            0 as libc::c_int,
        ) as u_char;
    }
    if par & 128 as libc::c_int != 0 {
        ret = closeSERVERClient(chan);
        if ret < 0 as libc::c_int {
            return ret as libc::c_short;
        }
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Close(mut chan: libc::c_int) -> libc::c_short {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut sb: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_birthtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    if isChan(chan) == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    if isChanFree(chan) == 1 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    if chan == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    if chan == (*partab.jobtab).io as libc::c_int {
        (*partab.jobtab).io = 0 as libc::c_int as u_char;
    }
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    match (*c).type_0 as libc::c_int {
        1 => {
            if (*c).mode as libc::c_int == 1 as libc::c_int
                || (*c).mode as libc::c_int == 3 as libc::c_int
            {
                let mut ret: libc::c_int = fstat((*c).fid, &mut sb);
                if ret == 0 as libc::c_int && sb.st_size == 0 as libc::c_int as off_t {
                    close((*c).fid);
                    unlink(((*c).name).as_mut_ptr() as *mut libc::c_char);
                } else {
                    close((*c).fid);
                }
            } else {
                close((*c).fid);
            }
            (*c).type_0 = 0 as libc::c_int as u_char;
        }
        2 => {
            closeSERVER(chan);
        }
        3 => {
            if (*c).mode as libc::c_int == 10 as libc::c_int {
                SQ_Pipe_Close((*c).fid, ((*c).name).as_mut_ptr() as *mut libc::c_char);
            } else {
                close((*c).fid);
            }
            (*c).type_0 = 0 as libc::c_int as u_char;
        }
        4 => {
            close((*c).fid);
            (*c).type_0 = 0 as libc::c_int as u_char;
        }
        _ => {}
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Write(mut writebuf: *mut cstring) -> libc::c_int {
    let mut chan: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    ret = checkCstring(writebuf);
    if ret < 0 as libc::c_int {
        return ret;
    }
    chan = (*partab.jobtab).io as libc::c_int;
    if isChan(chan) == 0 as libc::c_int {
        return getError(1 as libc::c_int, 25 as libc::c_int);
    }
    if isChanFree(chan) == 1 as libc::c_int {
        return getError(1 as libc::c_int, 27 as libc::c_int);
    }
    if (*writebuf).len as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ret = objectWrite(
        chan,
        ((*writebuf).buf).as_mut_ptr() as *mut libc::c_char,
        (*writebuf).len as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    (*partab.jobtab)
        .seqio[chan as usize]
        .dx = ((*partab.jobtab).seqio[chan as usize].dx as libc::c_int + ret) as u_short;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_WriteStar(mut c: u_char) -> libc::c_short {
    let mut chan: libc::c_int = 0;
    chan = (*partab.jobtab).io as libc::c_int;
    if isChan(chan) == 0 as libc::c_int {
        return getError(1 as libc::c_int, 25 as libc::c_int) as libc::c_short;
    }
    if isChanFree(chan) == 1 as libc::c_int {
        return getError(1 as libc::c_int, 27 as libc::c_int) as libc::c_short;
    }
    return objectWrite(
        chan,
        &mut c as *mut u_char as *mut libc::c_char,
        1 as libc::c_int,
    ) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_WriteFormat(mut count: libc::c_int) -> libc::c_short {
    let mut chan: libc::c_int = 0;
    let mut IOptr: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut writebuf: [libc::c_char; 132] = [0; 132];
    let mut ret: libc::c_int = 0;
    let mut byteswritten: libc::c_int = 0;
    let mut numspaces: libc::c_int = 0;
    let mut bytestowrite: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    if count < -(2 as libc::c_int) {
        return getError(1 as libc::c_int, 41 as libc::c_int) as libc::c_short;
    }
    chan = (*partab.jobtab).io as libc::c_int;
    if isChan(chan) == 0 as libc::c_int {
        return getError(1 as libc::c_int, 25 as libc::c_int) as libc::c_short;
    }
    if isChanFree(chan) == 1 as libc::c_int {
        return getError(1 as libc::c_int, 27 as libc::c_int) as libc::c_short;
    }
    IOptr = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    match count {
        -2 => {
            writebuf[0 as libc::c_int as usize] = 27 as libc::c_int as libc::c_char;
            writebuf[1 as libc::c_int as usize] = '[' as i32 as libc::c_char;
            writebuf[2 as libc::c_int as usize] = '2' as i32 as libc::c_char;
            writebuf[3 as libc::c_int as usize] = 'J' as i32 as libc::c_char;
            writebuf[4 as libc::c_int as usize] = 27 as libc::c_int as libc::c_char;
            writebuf[5 as libc::c_int as usize] = '[' as i32 as libc::c_char;
            writebuf[6 as libc::c_int as usize] = 'H' as i32 as libc::c_char;
            ret = objectWrite(chan, writebuf.as_mut_ptr(), 7 as libc::c_int);
            if ret < 0 as libc::c_int {
                return ret as libc::c_short;
            }
            (*IOptr).dx = 0 as libc::c_int as u_short;
            (*IOptr).dy = 0 as libc::c_int as u_short;
            return ret as libc::c_short;
        }
        -1 => {
            ret = 0 as libc::c_int;
            if (*IOptr).options as u_int64 & MASK[1 as libc::c_int as usize] != 0 {
                ret = objectWrite(
                    chan,
                    ((*IOptr).out_term).as_mut_ptr() as *mut libc::c_char,
                    (*IOptr).out_len as libc::c_int,
                );
                if ret < 0 as libc::c_int {
                    return ret as libc::c_short;
                }
            }
            (*IOptr).dx = 0 as libc::c_int as u_short;
            (*IOptr).dy = ((*IOptr).dy).wrapping_add(1);
            (*IOptr).dy;
            return ret as libc::c_short;
        }
        _ => {
            byteswritten = 0 as libc::c_int;
            numspaces = count - (*IOptr).dx as libc::c_int;
            while numspaces > 0 as libc::c_int {
                if numspaces <= 132 as libc::c_int {
                    bytestowrite = numspaces;
                } else {
                    bytestowrite = 132 as libc::c_int;
                }
                index = 0 as libc::c_int;
                while index < bytestowrite {
                    writebuf[index as usize] = ' ' as i32 as libc::c_char;
                    index += 1;
                    index;
                }
                ret = objectWrite(chan, writebuf.as_mut_ptr(), bytestowrite);
                if ret < 0 as libc::c_int {
                    return ret as libc::c_short
                } else if ret == 0 as libc::c_int {
                    if (*partab.jobtab).trap as u_int64 & MASK[2 as libc::c_int as usize]
                        != 0
                    {
                        return byteswritten as libc::c_short;
                    }
                }
                (*IOptr).dx = ((*IOptr).dx as libc::c_int + ret) as u_short;
                byteswritten = byteswritten + ret;
                numspaces = numspaces - bytestowrite;
            }
            return byteswritten as libc::c_short;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Read(
    mut buf: *mut u_char,
    mut tout: libc::c_int,
    mut maxbyt: libc::c_int,
) -> libc::c_int {
    let mut chan: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if buf.is_null() {
        return getError(1 as libc::c_int, 28 as libc::c_int);
    }
    chan = (*partab.jobtab).io as libc::c_int;
    if isChan(chan) == 0 as libc::c_int {
        return getError(1 as libc::c_int, 25 as libc::c_int);
    }
    if isChanFree(chan) == 1 as libc::c_int {
        return getError(1 as libc::c_int, 27 as libc::c_int);
    }
    if tout < -(1 as libc::c_int) {
        return getError(1 as libc::c_int, 22 as libc::c_int);
    }
    if maxbyt < -(1 as libc::c_int) {
        return getError(1 as libc::c_int, 36 as libc::c_int);
    }
    (*partab.jobtab).seqio[chan as usize].dkey_len = 0 as libc::c_int as libc::c_short;
    (*partab.jobtab)
        .seqio[chan as usize]
        .dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
    if maxbyt == -(1 as libc::c_int) {
        maxbyt = 65534 as libc::c_int;
    }
    type_0 = (*partab.jobtab).seqio[chan as usize].type_0 as libc::c_int;
    if tout > 0 as libc::c_int && type_0 != 1 as libc::c_int {
        alarm(tout as libc::c_uint);
    }
    if tout > -(1 as libc::c_int) && type_0 != 1 as libc::c_int {
        (*partab.jobtab).test = 1 as libc::c_int as u_char;
    }
    match type_0 {
        1 => {
            ret = readFILE(chan, buf, maxbyt);
        }
        2 => {
            ret = readTCP(chan, buf, maxbyt, tout);
        }
        3 => {
            ret = readPIPE(chan, buf, maxbyt, tout);
        }
        4 => {
            ret = readTERM(chan, buf, maxbyt, tout);
        }
        _ => return getError(1 as libc::c_int, 30 as libc::c_int),
    }
    alarm(0 as libc::c_int as libc::c_uint);
    if ret >= 0 as libc::c_int {
        if ret == 0 as libc::c_int && tout == 0 as libc::c_int {
            (*partab.jobtab).test = 0 as libc::c_int as u_char;
        }
        *buf.offset(ret as isize) = '\0' as i32 as u_char;
        return ret;
    } else if (*partab.jobtab).trap as u_int64 & MASK[2 as libc::c_int as usize] != 0 {
        *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
        return 0 as libc::c_int;
    } else {
        if tout == 0 as libc::c_int {
            (*partab.jobtab).test = 0 as libc::c_int as u_char;
        }
        return ret;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SQ_ReadStar(
    mut result: *mut libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_short {
    let mut chan: libc::c_int = 0;
    let mut origopt: libc::c_char = 0;
    let mut buf: [u_char; 2] = [0; 2];
    let mut ret: libc::c_int = 0;
    if result.is_null() {
        return getError(1 as libc::c_int, 28 as libc::c_int) as libc::c_short;
    }
    if timeout < -(1 as libc::c_int) {
        return getError(1 as libc::c_int, 36 as libc::c_int) as libc::c_short;
    }
    chan = (*partab.jobtab).io as libc::c_int;
    if isChan(chan) == 0 as libc::c_int {
        return getError(1 as libc::c_int, 25 as libc::c_int) as libc::c_short;
    }
    if isChanFree(chan) == 1 as libc::c_int {
        return getError(1 as libc::c_int, 27 as libc::c_int) as libc::c_short;
    }
    origopt = (*partab.jobtab).seqio[chan as usize].options as libc::c_char;
    (*partab.jobtab)
        .seqio[chan as usize]
        .options = ((*partab.jobtab).seqio[chan as usize].options as u_int64
        & !MASK[3 as libc::c_int as usize]) as u_char;
    (*partab.jobtab)
        .seqio[chan as usize]
        .options = ((*partab.jobtab).seqio[chan as usize].options as u_int64
        & !MASK[4 as libc::c_int as usize]) as u_char;
    (*partab.jobtab)
        .seqio[chan as usize]
        .options = ((*partab.jobtab).seqio[chan as usize].options as u_int64
        & !MASK[5 as libc::c_int as usize]) as u_char;
    ret = SQ_Read(buf.as_mut_ptr(), timeout, 1 as libc::c_int);
    (*partab.jobtab).seqio[chan as usize].options = origopt as u_char;
    if ret < 0 as libc::c_int {
        return ret as libc::c_short
    } else if ret == 0 as libc::c_int {
        if (*partab.jobtab).seqio[chan as usize].dkey_len as libc::c_int
            > 0 as libc::c_int
        {
            *result = (*partab.jobtab)
                .seqio[chan as usize]
                .dkey[0 as libc::c_int as usize] as libc::c_int;
            return 0 as libc::c_int as libc::c_short;
        } else {
            *result = -(1 as libc::c_int);
            return 0 as libc::c_int as libc::c_short;
        }
    } else {
        *result = buf[0 as libc::c_int as usize] as libc::c_int;
        return 1 as libc::c_int as libc::c_short;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Flush() -> libc::c_short {
    let mut chan: libc::c_int = 0;
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut what: libc::c_int = 0;
    chan = (*partab.jobtab).io as libc::c_int;
    if isChan(chan) == 0 as libc::c_int {
        return getError(1 as libc::c_int, 25 as libc::c_int) as libc::c_short;
    }
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    what = 1 as libc::c_int;
    if (*c).type_0 as libc::c_int == 4 as libc::c_int {
        let mut oid: libc::c_int = 0;
        if chan == 0 as libc::c_int {
            oid = 0 as libc::c_int;
        } else {
            oid = (*c).fid;
        }
        if isatty(oid) != 0 {
            let mut ret: libc::c_int = tcflush(oid, what);
            if ret == -(1 as libc::c_int) {
                return getError(0 as libc::c_int, *__error()) as libc::c_short;
            }
        } else {
            return getError(1 as libc::c_int, 24 as libc::c_int) as libc::c_short
        }
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Device(mut buf: *mut u_char) -> libc::c_int {
    let mut chan: libc::c_int = 0;
    let mut errmsg: [libc::c_char; 65534] = [0; 65534];
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if buf.is_null() {
        return getError(1 as libc::c_int, 28 as libc::c_int);
    }
    chan = (*partab.jobtab).io as libc::c_int;
    if isChan(chan) == 0 as libc::c_int {
        getErrorMsg(25 as libc::c_int, errmsg.as_mut_ptr());
        sprintf(
            buf as *mut libc::c_char,
            b"1,%d,%s\0" as *const u8 as *const libc::c_char,
            25 as libc::c_int,
            errmsg.as_mut_ptr(),
        );
        return strlen(buf as *mut libc::c_char) as libc::c_int;
    }
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    if isChanFree(chan) == 1 as libc::c_int {
        sprintf(buf as *mut libc::c_char, b"1,0,\0" as *const u8 as *const libc::c_char);
        return strlen(buf as *mut libc::c_char) as libc::c_int;
    }
    if (*c).type_0 as libc::c_int == 2 as libc::c_int {
        match (*c).mode as libc::c_int {
            5 => {
                name = ((*c).name).as_mut_ptr() as *mut libc::c_char;
            }
            6 => {
                if (*c).s.cid != -(1 as libc::c_int) {
                    name = ((*c).s.name).as_mut_ptr() as *mut libc::c_char;
                } else {
                    name = ((*c).name).as_mut_ptr() as *mut libc::c_char;
                }
            }
            7 => {
                if (*c).s.cid != -(1 as libc::c_int) {
                    name = ((*c).s.name).as_mut_ptr() as *mut libc::c_char;
                } else {
                    name = ((*c).name).as_mut_ptr() as *mut libc::c_char;
                }
            }
            8 => {
                name = ((*c).name).as_mut_ptr() as *mut libc::c_char;
            }
            _ => {
                name = 0 as *mut libc::c_char;
            }
        }
    } else {
        name = ((*c).name).as_mut_ptr() as *mut libc::c_char;
    }
    if name.is_null() {
        getErrorMsg(28 as libc::c_int + 200 as libc::c_int, errmsg.as_mut_ptr());
        sprintf(
            buf as *mut libc::c_char,
            b"1,%d,%s\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int + 200 as libc::c_int,
            errmsg.as_mut_ptr(),
        );
        return strlen(buf as *mut libc::c_char) as libc::c_int;
    }
    sprintf(
        buf as *mut libc::c_char,
        b"0,%d,%s\0" as *const u8 as *const libc::c_char,
        (*c).type_0 as libc::c_int,
        name,
    );
    return strlen(buf as *mut libc::c_char) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Force(
    mut device: *mut cstring,
    mut msg: *mut cstring,
) -> libc::c_short {
    let mut ret: libc::c_int = 0;
    ret = checkCstring(device);
    if ret < 0 as libc::c_int {
        return ret as libc::c_short;
    }
    ret = checkCstring(msg);
    if ret < 0 as libc::c_int {
        return ret as libc::c_short;
    }
    if (*msg).len as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn checkBytestr(mut bytestr: *const libc::c_char) -> libc::c_int {
    if bytestr.is_null() {
        return getError(1 as libc::c_int, 28 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkNbytes(mut nbytes: libc::c_int) -> libc::c_int {
    if nbytes < 0 as libc::c_int || nbytes > 65534 as libc::c_int {
        return getError(1 as libc::c_int, 33 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkCstring(mut cstr: *mut cstring) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if cstr.is_null() {
        return getError(1 as libc::c_int, 28 as libc::c_int);
    }
    ret = checkBytestr(((*cstr).buf).as_mut_ptr() as *mut libc::c_char);
    if ret < 0 as libc::c_int {
        return ret;
    }
    ret = checkNbytes((*cstr).len as libc::c_int);
    if ret < 0 as libc::c_int {
        return ret;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getObjectType(mut object: *mut libc::c_char) -> libc::c_int {
    let mut sb: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_birthtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    let mut ret: libc::c_int = 0;
    ret = stat(object, &mut sb);
    if ret == -(1 as libc::c_int) {
        if *__error() == 2 as libc::c_int {
            return 0 as libc::c_int
        } else {
            return getError(0 as libc::c_int, *__error())
        }
    }
    if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int == 0o40000 as libc::c_int)
        as libc::c_int == 1 as libc::c_int
    {
        return 1 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o20000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 2 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o60000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 3 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o100000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 4 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o10000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 5 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o120000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 6 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o140000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 7 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o160000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 8 as libc::c_int
    } else {
        return getError(1 as libc::c_int, 30 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn getObjectMode(mut fd: libc::c_int) -> libc::c_int {
    let mut sb: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_birthtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    let mut ret: libc::c_int = 0;
    ret = fstat(fd, &mut sb);
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int == 0o40000 as libc::c_int)
        as libc::c_int == 1 as libc::c_int
    {
        return 1 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o20000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 2 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o60000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 3 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o100000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 4 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o10000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 5 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o120000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 6 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o140000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 7 as libc::c_int
    } else if (sb.st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o160000 as libc::c_int) as libc::c_int == 1 as libc::c_int
    {
        return 8 as libc::c_int
    } else {
        return getError(1 as libc::c_int, 30 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn getModeCategory(mut mode: libc::c_int) -> libc::c_int {
    if mode == 4 as libc::c_int {
        return 1 as libc::c_int;
    }
    if mode == 7 as libc::c_int {
        return 2 as libc::c_int;
    }
    if mode == 5 as libc::c_int {
        return 3 as libc::c_int;
    }
    if mode == 2 as libc::c_int {
        return 4 as libc::c_int;
    }
    return getError(1 as libc::c_int, 30 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn isChan(mut chan: libc::c_int) -> libc::c_int {
    if chan < 0 as libc::c_int || chan >= 64 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isChanFree(mut chan: libc::c_int) -> libc::c_int {
    if (*partab.jobtab).seqio[chan as usize].type_0 as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkAsciiChars(mut cstr: *mut cstring) -> libc::c_int {
    let mut index: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    ret = checkCstring(cstr);
    if ret < 0 as libc::c_int {
        return ret;
    }
    index = 0 as libc::c_int;
    while index < (*cstr).len as libc::c_int {
        if (*cstr).buf[index as usize] as libc::c_int > 127 as libc::c_int {
            return getError(1 as libc::c_int, 34 as libc::c_int);
        }
        index += 1;
        index;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getBitMask(
    mut cstr: *mut cstring,
    mut in_term: IN_Term,
) -> IN_Term {
    let mut index: libc::c_int = 0;
    let mut chmask: u_int64 = 0;
    in_term.interm[0 as libc::c_int as usize] = 0 as libc::c_int as u_int64;
    in_term.interm[1 as libc::c_int as usize] = 0 as libc::c_int as u_int64;
    index = 0 as libc::c_int;
    while index < (*cstr).len as libc::c_int {
        if !((*cstr).buf[index as usize] as libc::c_int > 127 as libc::c_int) {
            chmask = ((1 as libc::c_ulong)
                << (*cstr).buf[index as usize] as libc::c_int % 64 as libc::c_int)
                as u_int64;
            in_term
                .interm[((*cstr).buf[index as usize] as libc::c_int / 64 as libc::c_int)
                as usize] |= chmask;
        }
        index += 1;
        index;
    }
    return in_term;
}
#[no_mangle]
pub unsafe extern "C" fn setOptionsBitMask(
    mut options: libc::c_int,
    mut bit: libc::c_int,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut mask: libc::c_int = 0;
    mask = ((1 as libc::c_uint) << bit) as libc::c_int;
    if flag == 1 as libc::c_int {
        return options | mask;
    }
    if flag == -(1 as libc::c_int) {
        return options & !mask;
    }
    return options;
}
#[no_mangle]
pub unsafe extern "C" fn getOperation(mut op: *mut cstring) -> libc::c_int {
    let mut str: [libc::c_char; 30] = [0; 30];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*op).len as libc::c_int > 30 as libc::c_int {
        return getError(1 as libc::c_int, 33 as libc::c_int);
    }
    ptr = strcpy(str.as_mut_ptr(), ((*op).buf).as_mut_ptr() as *mut libc::c_char);
    ptr = strchr(ptr, '=' as i32);
    if !ptr.is_null() {
        *ptr = '\0' as i32 as libc::c_char;
    }
    if strlen(str.as_mut_ptr()) == 1 as libc::c_int as libc::c_ulong {
        let mut ch: libc::c_char = tolower(
            (*op).buf[0 as libc::c_int as usize] as libc::c_int,
        ) as libc::c_char;
        if ch as libc::c_int == 'w' as i32 {
            return 1 as libc::c_int
        } else if ch as libc::c_int == 'r' as i32 {
            return 2 as libc::c_int
        } else if ch as libc::c_int == 'a' as i32 {
            return 3 as libc::c_int
        } else if ch as libc::c_int == 'i' as i32 {
            return 4 as libc::c_int
        } else if ch as libc::c_int == 't' as i32 {
            return 5 as libc::c_int
        } else if ch as libc::c_int == 's' as i32 {
            return 6 as libc::c_int
        } else if ch as libc::c_int == 'p' as i32 {
            return 9 as libc::c_int
        } else if ch as libc::c_int == 'n' as i32 {
            return 10 as libc::c_int
        }
    } else if strlen(str.as_mut_ptr()) > 1 as libc::c_int as libc::c_ulong {
        if strcasecmp(str.as_mut_ptr(), b"write\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            return 1 as libc::c_int
        } else if strcasecmp(
            str.as_mut_ptr(),
            b"read\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 2 as libc::c_int
        } else if strcasecmp(
            str.as_mut_ptr(),
            b"append\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 3 as libc::c_int
        } else if strcasecmp(
            str.as_mut_ptr(),
            b"io\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 4 as libc::c_int
        } else if strcasecmp(
            str.as_mut_ptr(),
            b"tcpip\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 5 as libc::c_int
        } else if strcasecmp(
            str.as_mut_ptr(),
            b"server\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 6 as libc::c_int
        } else if strcasecmp(
            str.as_mut_ptr(),
            b"pipe\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 9 as libc::c_int
        } else if strcasecmp(
            str.as_mut_ptr(),
            b"newpipe\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 10 as libc::c_int
        }
    }
    return getError(1 as libc::c_int, 21 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn getErrorMsg(
    mut errnum: libc::c_int,
    mut errmsg: *mut libc::c_char,
) {
    UTIL_strerror(-errnum, errmsg as *mut u_char);
}
#[no_mangle]
pub unsafe extern "C" fn initObject(
    mut chan: libc::c_int,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut par: libc::c_int = 0;
    let mut interm: cstring = CSTRING { len: 0, buf: [0; 65535] };
    let mut outerm: cstring = CSTRING { len: 0, buf: [0; 65535] };
    let mut settings: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0; 20],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut io: libc::c_char = 0;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    par = 0 as libc::c_int;
    match type_0 {
        0 => {
            (*c).type_0 = 0 as libc::c_int as u_char;
        }
        1 => {
            (*c).type_0 = 1 as libc::c_int as u_char;
            snprintf(
                (outerm.buf).as_mut_ptr() as *mut libc::c_char,
                65534 as libc::c_int as libc::c_ulong,
                b"%c\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as libc::c_char as libc::c_int,
            );
            snprintf(
                (interm.buf).as_mut_ptr() as *mut libc::c_char,
                65534 as libc::c_int as libc::c_ulong,
                b"%c\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as libc::c_char as libc::c_int,
            );
        }
        2 => {
            (*c).type_0 = 2 as libc::c_int as u_char;
            snprintf(
                (outerm.buf).as_mut_ptr() as *mut libc::c_char,
                65534 as libc::c_int as libc::c_ulong,
                b"%c%c\0" as *const u8 as *const libc::c_char,
                13 as libc::c_int as libc::c_char as libc::c_int,
                10 as libc::c_int as libc::c_char as libc::c_int,
            );
            snprintf(
                (interm.buf).as_mut_ptr() as *mut libc::c_char,
                65534 as libc::c_int as libc::c_ulong,
                b"%c%c\0" as *const u8 as *const libc::c_char,
                13 as libc::c_int as libc::c_char as libc::c_int,
                10 as libc::c_int as libc::c_char as libc::c_int,
            );
        }
        3 => {
            (*c).type_0 = 3 as libc::c_int as u_char;
            snprintf(
                (outerm.buf).as_mut_ptr() as *mut libc::c_char,
                65534 as libc::c_int as libc::c_ulong,
                b"%c\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as libc::c_char as libc::c_int,
            );
            snprintf(
                (interm.buf).as_mut_ptr() as *mut libc::c_char,
                65534 as libc::c_int as libc::c_ulong,
                b"%c\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as libc::c_char as libc::c_int,
            );
        }
        4 => {
            (*c).type_0 = 4 as libc::c_int as u_char;
            if chan == 0 as libc::c_int && isatty(0 as libc::c_int) != 0 {
                if tcgetattr(0 as libc::c_int, &mut settings) == -(1 as libc::c_int) {
                    return getError(0 as libc::c_int, *__error());
                }
                settings.c_lflag &= !(0x100 as libc::c_int) as tcflag_t;
                settings.c_lflag &= !(0x8 as libc::c_int) as tcflag_t;
                settings.c_oflag &= !(0x2 as libc::c_int) as tcflag_t;
                settings.c_iflag &= !(0x100 as libc::c_int) as tcflag_t;
                settings.c_cc[16 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
                settings.c_cc[17 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
                settings.c_cc[10 as libc::c_int as usize] = -1i32 as libc::c_uchar;
                settings.c_cc[8 as libc::c_int as usize] = '\u{3}' as i32 as cc_t;
                settings.c_cc[9 as libc::c_int as usize] = '\u{14}' as i32 as cc_t;
                if tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut settings)
                    == -(1 as libc::c_int)
                {
                    return getError(0 as libc::c_int, *__error());
                }
            }
            par
                |= 1 as libc::c_int | 4 as libc::c_int | 1024 as libc::c_int
                    | 4096 as libc::c_int;
            snprintf(
                (outerm.buf).as_mut_ptr() as *mut libc::c_char,
                65534 as libc::c_int as libc::c_ulong,
                b"%c%c\0" as *const u8 as *const libc::c_char,
                13 as libc::c_int as libc::c_char as libc::c_int,
                10 as libc::c_int as libc::c_char as libc::c_int,
            );
            snprintf(
                (interm.buf).as_mut_ptr() as *mut libc::c_char,
                65534 as libc::c_int as libc::c_ulong,
                b"%c\0" as *const u8 as *const libc::c_char,
                13 as libc::c_int as libc::c_char as libc::c_int,
            );
        }
        _ => return getError(1 as libc::c_int, 20 as libc::c_int),
    }
    (*c).options = 0 as libc::c_int as u_char;
    (*c).mode = 0 as libc::c_int as u_char;
    (*c).fid = 0 as libc::c_int;
    initSERVER(chan, 0 as libc::c_int as u_int);
    (*c).dx = 0 as libc::c_int as u_short;
    (*c).dy = 0 as libc::c_int as u_short;
    (*c).name[0 as libc::c_int as usize] = '\0' as i32 as u_char;
    (*c).dkey_len = 0 as libc::c_int as libc::c_short;
    (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
    outerm.len = strlen((outerm.buf).as_mut_ptr() as *mut libc::c_char) as u_short;
    interm.len = strlen((interm.buf).as_mut_ptr() as *mut libc::c_char) as u_short;
    io = (*partab.jobtab).io as libc::c_char;
    SQ_Use(chan, &mut interm, &mut outerm, par);
    (*partab.jobtab).io = io as u_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn objectWrite(
    mut chan: libc::c_int,
    mut writebuf: *mut libc::c_char,
    mut nbytes: libc::c_int,
) -> libc::c_int {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut oid: libc::c_int = 0;
    let mut byteswritten: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    if chan == 0 as libc::c_int {
        oid = 1 as libc::c_int;
    } else if (*c).type_0 as libc::c_int == 2 as libc::c_int {
        if (*c).mode as libc::c_int == 5 as libc::c_int {
            oid = (*c).fid;
        } else if (*c).mode as libc::c_int == 8 as libc::c_int {
            oid = (*c).fid;
        } else if (*c).mode as libc::c_int == 7 as libc::c_int {
            if (*c).s.cid == -(1 as libc::c_int) {
                return getError(1 as libc::c_int, 47 as libc::c_int)
            } else {
                oid = (*c).s.cid;
            }
        } else if (*c).mode as libc::c_int == 6 as libc::c_int {
            if (*c).s.cid == -(1 as libc::c_int) {
                return getError(1 as libc::c_int, 47 as libc::c_int)
            } else {
                oid = (*c).s.cid;
            }
        } else {
            return getError(1 as libc::c_int, 20 as libc::c_int)
        }
    } else {
        oid = (*c).fid;
    }
    byteswritten = 0 as libc::c_int;
    while byteswritten < nbytes {
        let mut bytestowrite: libc::c_int = nbytes - byteswritten;
        match (*c).type_0 as libc::c_int {
            1 => {
                ret = SQ_File_Write(
                    oid,
                    &mut *writebuf.offset(byteswritten as isize) as *mut libc::c_char
                        as *mut u_char,
                    bytestowrite,
                );
            }
            4 => {
                ret = SQ_Device_Write(
                    oid,
                    &mut *writebuf.offset(byteswritten as isize) as *mut libc::c_char
                        as *mut u_char,
                    bytestowrite,
                );
            }
            3 => {
                ret = SQ_Pipe_Write(
                    oid,
                    &mut *writebuf.offset(byteswritten as isize) as *mut libc::c_char
                        as *mut u_char,
                    bytestowrite,
                );
            }
            2 => {
                ret = SQ_Tcpip_Write(
                    oid,
                    &mut *writebuf.offset(byteswritten as isize) as *mut libc::c_char
                        as *mut u_char,
                    bytestowrite,
                );
            }
            _ => return getError(1 as libc::c_int, 30 as libc::c_int),
        }
        if ret < 0 as libc::c_int {
            if (*partab.jobtab).trap as u_int64 & MASK[13 as libc::c_int as usize] != 0 {
                (*partab.jobtab)
                    .trap = ((*partab.jobtab).trap as u_int64
                    & !MASK[13 as libc::c_int as usize]) as u_int;
                return getError(1 as libc::c_int, 47 as libc::c_int);
            } else {
                return ret
            }
        } else {
            byteswritten = byteswritten + ret;
        }
    }
    return byteswritten;
}
#[no_mangle]
pub unsafe extern "C" fn readFILE(
    mut chan: libc::c_int,
    mut buf: *mut u_char,
    mut maxbyt: libc::c_int,
) -> libc::c_int {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut bytesread: libc::c_int = 0;
    let mut crflag: libc::c_int = 0;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    bytesread = 0 as libc::c_int;
    crflag = 0 as libc::c_int;
    loop {
        let mut ret: libc::c_int = 0;
        if bytesread >= maxbyt {
            (*c).dkey_len = 0 as libc::c_int as libc::c_short;
            (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            return bytesread;
        }
        ret = SQ_File_Read((*c).fid, &mut *buf.offset(bytesread as isize));
        if ret < 0 as libc::c_int {
            (*c).dkey_len = 0 as libc::c_int as libc::c_short;
            (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            return ret;
        } else if ret == 0 as libc::c_int {
            (*c).dkey_len = 1 as libc::c_int as libc::c_short;
            (*c)
                .dkey[0 as libc::c_int
                as usize] = 255 as libc::c_int as libc::c_char as u_char;
            (*c).dkey[1 as libc::c_int as usize] = '\0' as i32 as u_char;
            return bytesread;
        }
        if (*c).options as u_int64 & MASK[0 as libc::c_int as usize] != 0 {
            if (*c).in_term.iterm == CRLF {
                if *buf.offset(bytesread as isize) as libc::c_int == 13 as libc::c_int {
                    crflag = 1 as libc::c_int;
                } else if *buf.offset(bytesread as isize) as libc::c_int
                    == 10 as libc::c_int && crflag == 1 as libc::c_int
                {
                    (*c).dkey_len = 2 as libc::c_int as libc::c_short;
                    (*c)
                        .dkey[0 as libc::c_int
                        as usize] = 13 as libc::c_int as libc::c_char as u_char;
                    (*c)
                        .dkey[1 as libc::c_int
                        as usize] = 10 as libc::c_int as libc::c_char as u_char;
                    (*c).dkey[2 as libc::c_int as usize] = '\0' as i32 as u_char;
                    return bytesread - 1 as libc::c_int;
                }
            } else if (*buf.offset(bytesread as isize) as libc::c_int)
                < 128 as libc::c_int
            {
                if (*c)
                    .in_term
                    .interm[(*buf.offset(bytesread as isize) as libc::c_int
                    / 64 as libc::c_int) as usize]
                    & MASK[(*buf.offset(bytesread as isize) as libc::c_int
                        % 64 as libc::c_int) as usize] != 0
                {
                    (*c).dkey_len = 1 as libc::c_int as libc::c_short;
                    (*c)
                        .dkey[0 as libc::c_int
                        as usize] = *buf.offset(bytesread as isize);
                    (*c).dkey[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                    return bytesread;
                }
            }
        }
        bytesread += 1;
        bytesread;
    };
}
#[no_mangle]
pub unsafe extern "C" fn readTCP(
    mut chan: libc::c_int,
    mut buf: *mut u_char,
    mut maxbyt: libc::c_int,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut oid: libc::c_int = 0;
    let mut bytesread: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut crflag: libc::c_int = 0;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    if (*c).mode as libc::c_int == 6 as libc::c_int as u_char as libc::c_int {
        ret = 0 as libc::c_int;
        while ret == 0 as libc::c_int {
            ret = acceptSERVER(chan, tout);
        }
        oid = ret;
    } else if (*c).mode as libc::c_int == 7 as libc::c_int as u_char as libc::c_int {
        ret = acceptSERVER(chan, tout);
        oid = ret;
    } else {
        oid = (*c).fid;
    }
    if oid < 0 as libc::c_int {
        (*c).dkey_len = 0 as libc::c_int as libc::c_short;
        (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
        if (*partab.jobtab).trap as u_int64 & MASK[14 as libc::c_int as usize] != 0 {
            (*c).dkey_len = 0 as libc::c_int as libc::c_short;
            (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            (*partab.jobtab)
                .trap = ((*partab.jobtab).trap as u_int64
                & !MASK[14 as libc::c_int as usize]) as u_int;
            (*partab.jobtab).test = 0 as libc::c_int as u_char;
            return 0 as libc::c_int;
        } else {
            return oid
        }
    }
    bytesread = 0 as libc::c_int;
    crflag = 0 as libc::c_int;
    loop {
        if bytesread >= maxbyt {
            (*c).dkey_len = 0 as libc::c_int as libc::c_short;
            (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            return bytesread;
        }
        ret = SQ_Tcpip_Read(oid, &mut *buf.offset(bytesread as isize), tout);
        if (*partab.jobtab).attention != 0 {
            ret = signalCaught(c);
            if ret == 14 as libc::c_int {
                return bytesread
            } else {
                return -(1 as libc::c_int)
            }
        } else if ret < 0 as libc::c_int {
            return ret
        } else if ret == 0 as libc::c_int {
            (*c).dkey_len = 1 as libc::c_int as libc::c_short;
            (*c)
                .dkey[0 as libc::c_int
                as usize] = 255 as libc::c_int as libc::c_char as u_char;
            (*c).dkey[1 as libc::c_int as usize] = '\0' as i32 as u_char;
            closeSERVERClient(chan);
            return bytesread;
        }
        if (*c).options as u_int64 & MASK[0 as libc::c_int as usize] != 0 {
            if (*c).in_term.iterm == CRLF {
                if *buf.offset(bytesread as isize) as libc::c_int == 13 as libc::c_int {
                    crflag = 1 as libc::c_int;
                } else if *buf.offset(bytesread as isize) as libc::c_int
                    == 10 as libc::c_int && crflag == 1 as libc::c_int
                {
                    (*c).dkey_len = 2 as libc::c_int as libc::c_short;
                    (*c)
                        .dkey[0 as libc::c_int
                        as usize] = 13 as libc::c_int as libc::c_char as u_char;
                    (*c)
                        .dkey[1 as libc::c_int
                        as usize] = 10 as libc::c_int as libc::c_char as u_char;
                    (*c).dkey[2 as libc::c_int as usize] = '\0' as i32 as u_char;
                    return bytesread - 1 as libc::c_int;
                }
            } else if (*buf.offset(bytesread as isize) as libc::c_int)
                < 128 as libc::c_int
            {
                if (*c)
                    .in_term
                    .interm[(*buf.offset(bytesread as isize) as libc::c_int
                    / 64 as libc::c_int) as usize]
                    & MASK[(*buf.offset(bytesread as isize) as libc::c_int
                        % 64 as libc::c_int) as usize] != 0
                {
                    (*c).dkey_len = 1 as libc::c_int as libc::c_short;
                    (*c)
                        .dkey[0 as libc::c_int
                        as usize] = *buf.offset(bytesread as isize);
                    (*c).dkey[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                    return bytesread;
                }
            }
        }
        bytesread += 1;
        bytesread;
    };
}
#[no_mangle]
pub unsafe extern "C" fn readPIPE(
    mut chan: libc::c_int,
    mut buf: *mut u_char,
    mut maxbyt: libc::c_int,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut oid: libc::c_int = 0;
    let mut bytesread: libc::c_int = 0;
    let mut crflag: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    if chan == 0 as libc::c_int {
        oid = 0 as libc::c_int;
    } else {
        oid = (*c).fid;
    }
    bytesread = 0 as libc::c_int;
    crflag = 0 as libc::c_int;
    loop {
        let mut ret: libc::c_int = 0;
        if bytesread >= maxbyt {
            (*c).dkey_len = 0 as libc::c_int as libc::c_short;
            (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            return bytesread;
        }
        ret = SQ_Pipe_Read(oid, &mut *buf.offset(bytesread as isize), tout);
        if (*partab.jobtab).attention != 0 {
            tmp = signalCaught(c);
            if tmp == 14 as libc::c_int {
                return bytesread
            } else {
                return -(1 as libc::c_int)
            }
        } else if ret < 0 as libc::c_int {
            return ret
        } else if (*c).options as u_int64 & MASK[0 as libc::c_int as usize] != 0
            && ret != 0
        {
            if (*c).in_term.iterm == CRLF {
                if *buf.offset(bytesread as isize) as libc::c_int == 13 as libc::c_int {
                    crflag = 1 as libc::c_int;
                } else if *buf.offset(bytesread as isize) as libc::c_int
                    == 10 as libc::c_int && crflag == 1 as libc::c_int
                {
                    (*c).dkey_len = 2 as libc::c_int as libc::c_short;
                    (*c)
                        .dkey[0 as libc::c_int
                        as usize] = 13 as libc::c_int as libc::c_char as u_char;
                    (*c)
                        .dkey[1 as libc::c_int
                        as usize] = 10 as libc::c_int as libc::c_char as u_char;
                    (*c).dkey[2 as libc::c_int as usize] = '\0' as i32 as u_char;
                    return bytesread - 1 as libc::c_int;
                }
            } else if (*buf.offset(bytesread as isize) as libc::c_int)
                < 128 as libc::c_int
            {
                if (*c)
                    .in_term
                    .interm[(*buf.offset(bytesread as isize) as libc::c_int
                    / 64 as libc::c_int) as usize]
                    & MASK[(*buf.offset(bytesread as isize) as libc::c_int
                        % 64 as libc::c_int) as usize] != 0
                {
                    (*c).dkey_len = 1 as libc::c_int as libc::c_short;
                    (*c)
                        .dkey[0 as libc::c_int
                        as usize] = *buf.offset(bytesread as isize);
                    (*c).dkey[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                    return bytesread;
                }
            }
        }
        bytesread += ret;
    };
}
#[no_mangle]
pub unsafe extern "C" fn signalCaught(mut c: *mut SQ_Chan) -> libc::c_int {
    (*c).dkey_len = 0 as libc::c_int as libc::c_short;
    (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
    if (*partab.jobtab).trap as u_int64 & MASK[14 as libc::c_int as usize] != 0 {
        (*c).dkey_len = 0 as libc::c_int as libc::c_short;
        (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
        (*partab.jobtab)
            .trap = ((*partab.jobtab).trap as u_int64
            & !MASK[14 as libc::c_int as usize]) as u_int;
        (*partab.jobtab).test = 0 as libc::c_int as u_char;
        return 14 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn readTERM(
    mut chan: libc::c_int,
    mut buf: *mut u_char,
    mut maxbyt: libc::c_int,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut oid: libc::c_int = 0;
    let mut bytesread: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut crflag: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut value: libc::c_char = 0;
    let mut curr: u_char = 0;
    let mut writebuf: cstring = CSTRING { len: 0, buf: [0; 65535] };
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut cpr: [libc::c_char; 12] = [0; 12];
    let mut j: libc::c_int = 0;
    static mut editing: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut start: u_short = 0;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    if in_hist as libc::c_int > -(1 as libc::c_int) {
        start = prompt_len;
    } else {
        start = (*c).dx;
    }
    if chan == 0 as libc::c_int {
        oid = 0 as libc::c_int;
    } else {
        oid = (*c).fid;
    }
    ret = ioctl(
        oid,
        0x40000000 as libc::c_int as __uint32_t as libc::c_ulong
            | (::core::mem::size_of::<winsize>() as libc::c_ulong
                & 0x1fff as libc::c_int as libc::c_ulong) << 16 as libc::c_int
            | (('t' as i32) << 8 as libc::c_int) as libc::c_ulong
            | 104 as libc::c_int as libc::c_ulong,
        &mut w as *mut winsize,
    );
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    if w.ws_col as libc::c_int == 0 as libc::c_int {
        ret = ioctl(
            1 as libc::c_int,
            0x40000000 as libc::c_int as __uint32_t as libc::c_ulong
                | (::core::mem::size_of::<winsize>() as libc::c_ulong
                    & 0x1fff as libc::c_int as libc::c_ulong) << 16 as libc::c_int
                | (('t' as i32) << 8 as libc::c_int) as libc::c_ulong
                | 104 as libc::c_int as libc::c_ulong,
            &mut w as *mut winsize,
        );
    }
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    if isatty(1 as libc::c_int) == 0 || w.ws_col as libc::c_int == 0 as libc::c_int {
        w.ws_col = 80 as libc::c_int as libc::c_ushort;
    }
    bytesread = 0 as libc::c_int;
    crflag = 0 as libc::c_int;
    loop {
        if bytesread >= maxbyt {
            (*c).dkey_len = 0 as libc::c_int as libc::c_short;
            (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            return bytesread;
        }
        if (*c).dx as libc::c_int == start as libc::c_int + bytesread {
            editing = 0 as libc::c_int as libc::c_char;
        }
        ret = SQ_Device_Read(oid, &mut curr, tout);
        if ret < 0 as libc::c_int {
            (*c).dkey_len = 0 as libc::c_int as libc::c_short;
            (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            if (*partab.jobtab).trap as u_int64 & MASK[14 as libc::c_int as usize] != 0 {
                (*partab.jobtab)
                    .trap = ((*partab.jobtab).trap as u_int64
                    & !MASK[14 as libc::c_int as usize]) as u_int;
                (*partab.jobtab).test = 0 as libc::c_int as u_char;
                *buf.offset(bytesread as isize) = curr;
                return bytesread;
            } else {
                return ret
            }
        } else if ret == 0 as libc::c_int {
            (*c).dkey_len = 1 as libc::c_int as libc::c_short;
            (*c)
                .dkey[0 as libc::c_int
                as usize] = 255 as libc::c_int as libc::c_char as u_char;
            (*c).dkey[1 as libc::c_int as usize] = '\0' as i32 as u_char;
            *buf.offset(bytesread as isize) = curr;
            return bytesread;
        }
        if curr as libc::c_int == 8 as libc::c_int
            || curr as libc::c_int == 127 as libc::c_int
        {
            if in_hist as libc::c_int > -(1 as libc::c_int)
                || curr as libc::c_int == 8 as libc::c_int
                    && (*c).options as u_int64 & MASK[4 as libc::c_int as usize] != 0
                || curr as libc::c_int == 127 as libc::c_int
                    && (*c).options as u_int64 & MASK[5 as libc::c_int as usize] != 0
            {
                if bytesread > 0 as libc::c_int
                    && (*c).dx as libc::c_int > start as libc::c_int
                {
                    if (*c).dx as libc::c_int % w.ws_col as libc::c_int == 0 {
                        ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                            as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar('F' as i32 as u_char) as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                            as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        writebuf
                            .len = uitocstring(
                            (writebuf.buf).as_mut_ptr(),
                            w.ws_col as u_int,
                        );
                        ret = SQ_Write(&mut writebuf);
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        (*c).dx = ((*c).dx as libc::c_int - ret) as u_short;
                        ret = SQ_WriteStar('G' as i32 as u_char) as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                    } else {
                        ret = SQ_WriteStar(8 as libc::c_int as libc::c_char as u_char)
                            as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                    }
                    ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                        as libc::c_int;
                    if ret < 0 as libc::c_int {
                        return ret;
                    }
                    ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                    if ret < 0 as libc::c_int {
                        return ret;
                    }
                    ret = SQ_WriteStar('P' as i32 as u_char) as libc::c_int;
                    if ret < 0 as libc::c_int {
                        return ret;
                    }
                    if editing as libc::c_int != 0
                        && start as libc::c_int + bytesread > w.ws_col as libc::c_int
                    {
                        ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                            as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar('J' as i32 as u_char) as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                            as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar('7' as i32 as u_char) as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_Device_Write(
                            oid,
                            &mut *buf
                                .offset(
                                    ((*c).dx as libc::c_int - start as libc::c_int) as isize,
                                ) as *mut u_char,
                            start as libc::c_int + bytesread - (*c).dx as libc::c_int,
                        );
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                            as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        ret = SQ_WriteStar('8' as i32 as u_char) as libc::c_int;
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                    }
                    bytesread -= 1;
                    bytesread;
                    (*c).dx = ((*c).dx).wrapping_sub(1);
                    (*c).dx;
                    if editing != 0 {
                        i = (*c).dx as libc::c_int - start as libc::c_int;
                        while i < bytesread {
                            *buf
                                .offset(
                                    i as isize,
                                ) = *buf.offset((i + 1 as libc::c_int) as isize);
                            i += 1;
                            i;
                        }
                    }
                }
                continue;
            }
        }
        if curr as libc::c_int == 27 as libc::c_int
            && ((*c).options as u_int64 & MASK[2 as libc::c_int as usize] != 0
                || in_hist as libc::c_int > -(1 as libc::c_int))
        {
            (*c).dkey_len = 1 as libc::c_int as libc::c_short;
            (*c)
                .dkey[0 as libc::c_int
                as usize] = 27 as libc::c_int as libc::c_char as u_char;
            loop {
                if (*c).dkey_len as libc::c_int > 16 as libc::c_int {
                    return getError(1 as libc::c_int, 39 as libc::c_int);
                }
                ret = SQ_Device_Read(
                    oid,
                    &mut *((*c).dkey).as_mut_ptr().offset((*c).dkey_len as isize),
                    tout,
                );
                if ret < 0 as libc::c_int {
                    (*c).dkey_len = 0 as libc::c_int as libc::c_short;
                    (*c).dkey[0 as libc::c_int as usize] = '\0' as i32 as u_char;
                    *buf.offset(bytesread as isize) = curr;
                    if (*partab.jobtab).trap as u_int64
                        & MASK[14 as libc::c_int as usize] != 0
                    {
                        (*partab.jobtab)
                            .trap = ((*partab.jobtab).trap as u_int64
                            & !MASK[14 as libc::c_int as usize]) as u_int;
                        (*partab.jobtab).test = 0 as libc::c_int as u_char;
                        return bytesread;
                    } else {
                        return ret
                    }
                } else if ret == 0 as libc::c_int {
                    (*c).dkey_len = 1 as libc::c_int as libc::c_short;
                    (*c)
                        .dkey[0 as libc::c_int
                        as usize] = 255 as libc::c_int as libc::c_char as u_char;
                    (*c).dkey[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                    *buf.offset(bytesread as isize) = curr;
                    return bytesread;
                }
                value = (*c).dkey[(*c).dkey_len as usize] as libc::c_char;
                if value as libc::c_int != 'O' as i32 {
                    if value as libc::c_int >= 'A' as i32
                        && value as libc::c_int <= 'Z' as i32
                        || value as libc::c_int >= 'a' as i32
                            && value as libc::c_int <= 'z' as i32
                        || value as libc::c_int == '~' as i32
                    {
                        (*c).dkey_len += 1;
                        (*c).dkey_len;
                        (*c).dkey[(*c).dkey_len as usize] = '\0' as i32 as u_char;
                        if in_hist as libc::c_int > -(1 as libc::c_int)
                            && (*c).dkey_len as libc::c_int == 3 as libc::c_int
                            && (value as libc::c_int == 'A' as i32
                                || value as libc::c_int == 'B' as i32)
                        {
                            if in_hist == 0 {
                                in_hist = 1 as libc::c_int as libc::c_short;
                            }
                            if (*c).dx as libc::c_int >= w.ws_col as libc::c_int {
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                writebuf
                                    .len = uitocstring(
                                    (writebuf.buf).as_mut_ptr(),
                                    ((*c).dx as libc::c_int / w.ws_col as libc::c_int) as u_int,
                                );
                                ret = SQ_Write(&mut writebuf);
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                (*c).dx = ((*c).dx as libc::c_int - ret) as u_short;
                                ret = SQ_WriteStar('F' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                            }
                            ret = SQ_WriteStar(
                                27 as libc::c_int as libc::c_char as u_char,
                            ) as libc::c_int;
                            if ret < 0 as libc::c_int {
                                return ret;
                            }
                            ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                            if ret < 0 as libc::c_int {
                                return ret;
                            }
                            writebuf
                                .len = uitocstring(
                                (writebuf.buf).as_mut_ptr(),
                                (start as libc::c_int + 1 as libc::c_int) as u_int,
                            );
                            ret = SQ_Write(&mut writebuf);
                            if ret < 0 as libc::c_int {
                                return ret;
                            }
                            (*c).dx = ((*c).dx as libc::c_int - ret) as u_short;
                            ret = SQ_WriteStar('G' as i32 as u_char) as libc::c_int;
                            if ret < 0 as libc::c_int {
                                return ret;
                            }
                            ret = SQ_WriteStar(
                                27 as libc::c_int as libc::c_char as u_char,
                            ) as libc::c_int;
                            if ret < 0 as libc::c_int {
                                return ret;
                            }
                            ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                            if ret < 0 as libc::c_int {
                                return ret;
                            }
                            ret = SQ_WriteStar('J' as i32 as u_char) as libc::c_int;
                            if ret < 0 as libc::c_int {
                                return ret;
                            }
                            if value as libc::c_int == 'A' as i32 {
                                if hist_curr as libc::c_int == 0 as libc::c_int {
                                    hist_curr = (if history[(128 as libc::c_int
                                        - 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                                        as libc::c_int != '\0' as i32
                                    {
                                        128 as libc::c_int - 1 as libc::c_int
                                    } else {
                                        hist_next as libc::c_int
                                    }) as u_short;
                                } else {
                                    hist_curr = hist_curr.wrapping_sub(1);
                                    hist_curr;
                                }
                            } else if (hist_curr as libc::c_int)
                                < 128 as libc::c_int - 1 as libc::c_int
                                && history[hist_curr as usize][0 as libc::c_int as usize]
                                    as libc::c_int != '\0' as i32
                            {
                                hist_curr = hist_curr.wrapping_add(1);
                                hist_curr;
                            } else {
                                hist_curr = 0 as libc::c_int as u_short;
                            }
                            len = strlen((history[hist_curr as usize]).as_mut_ptr())
                                as libc::c_int;
                            (*c).dx = (start as libc::c_int + len) as u_short;
                            bytesread = len;
                            ret = SQ_Device_Write(
                                oid,
                                (history[hist_curr as usize]).as_mut_ptr() as *mut u_char,
                                len,
                            );
                            if ret < 0 as libc::c_int {
                                return ret;
                            }
                            sprintf(
                                buf as *mut libc::c_char,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                (history[hist_curr as usize]).as_mut_ptr(),
                            );
                            if (*c).dx as libc::c_int % w.ws_col as libc::c_int == 0 {
                                j = 0 as libc::c_int;
                                value = '\0' as i32 as libc::c_char;
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('6' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('n' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                loop {
                                    ret = read(
                                        oid,
                                        cpr.as_mut_ptr() as *mut libc::c_void,
                                        1 as libc::c_int as size_t,
                                    ) as libc::c_int;
                                    if !(ret != 0) {
                                        break;
                                    }
                                    if ret == -(1 as libc::c_int) {
                                        return getError(0 as libc::c_int, *__error());
                                    }
                                    if !(cpr[0 as libc::c_int as usize] as libc::c_int
                                        == 27 as libc::c_int)
                                    {
                                        continue;
                                    }
                                    ret = read(
                                        oid,
                                        cpr.as_mut_ptr() as *mut libc::c_void,
                                        12 as libc::c_int as size_t,
                                    ) as libc::c_int;
                                    if ret == -(1 as libc::c_int) {
                                        return getError(0 as libc::c_int, *__error());
                                    }
                                    ret = sscanf(
                                        cpr.as_mut_ptr(),
                                        b"[%d;%*d%c\0" as *const u8 as *const libc::c_char,
                                        &mut j as *mut libc::c_int,
                                        &mut value as *mut libc::c_char,
                                    );
                                    if ret == -(1 as libc::c_int) {
                                        return getError(0 as libc::c_int, *__error());
                                    }
                                    if value as libc::c_int == 'R' as i32 {
                                        break;
                                    }
                                    ret = read(
                                        oid,
                                        cpr.as_mut_ptr() as *mut libc::c_void,
                                        12 as libc::c_int as size_t,
                                    ) as libc::c_int;
                                    if ret == -(1 as libc::c_int) {
                                        return getError(0 as libc::c_int, *__error());
                                    }
                                    break;
                                }
                                if j == w.ws_row as libc::c_int {
                                    ret = SQ_WriteStar(
                                        27 as libc::c_int as libc::c_char as u_char,
                                    ) as libc::c_int;
                                    if ret < 0 as libc::c_int {
                                        return ret;
                                    }
                                    ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                    if ret < 0 as libc::c_int {
                                        return ret;
                                    }
                                    ret = SQ_WriteStar('S' as i32 as u_char) as libc::c_int;
                                    if ret < 0 as libc::c_int {
                                        return ret;
                                    }
                                }
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('E' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                            }
                            break;
                        } else if maxbyt > 1 as libc::c_int
                            && (*c).dkey_len as libc::c_int == 3 as libc::c_int
                            && value as libc::c_int == 'C' as i32
                        {
                            if editing == 0 {
                                editing = 1 as libc::c_int as libc::c_char;
                            }
                            if (*c).dx as libc::c_int >= start as libc::c_int + bytesread
                            {
                                break;
                            }
                            (*c).dx = ((*c).dx).wrapping_add(1);
                            (*c).dx;
                            if (*c).dx as libc::c_int % w.ws_col as libc::c_int == 0 {
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('E' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                break;
                            } else {
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('C' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                break;
                            }
                        } else if maxbyt > 1 as libc::c_int
                            && (*c).dkey_len as libc::c_int == 3 as libc::c_int
                            && value as libc::c_int == 'D' as i32
                        {
                            if editing == 0 {
                                editing = 1 as libc::c_int as libc::c_char;
                            }
                            if (*c).dx as libc::c_int <= start as libc::c_int {
                                break;
                            }
                            (*c).dx = ((*c).dx).wrapping_sub(1);
                            (*c).dx;
                            if ((*c).dx as libc::c_int + 1 as libc::c_int)
                                % w.ws_col as libc::c_int == 0
                            {
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('F' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                writebuf
                                    .len = uitocstring(
                                    (writebuf.buf).as_mut_ptr(),
                                    w.ws_col as u_int,
                                );
                                ret = SQ_Write(&mut writebuf);
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                (*c).dx = ((*c).dx as libc::c_int - ret) as u_short;
                                ret = SQ_WriteStar('G' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                break;
                            } else {
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('D' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                break;
                            }
                        } else if maxbyt > 1 as libc::c_int
                            && (*c).dkey_len as libc::c_int == 4 as libc::c_int
                            && (*c).dkey[2 as libc::c_int as usize] as libc::c_int
                                == '3' as i32 && value as libc::c_int == '~' as i32
                        {
                            if editing == 0 {
                                editing = 1 as libc::c_int as libc::c_char;
                            }
                            if (*c).dx as libc::c_int >= start as libc::c_int + bytesread
                            {
                                break;
                            }
                            if start as libc::c_int + bytesread > w.ws_col as libc::c_int
                            {
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('J' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('7' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_Device_Write(
                                    oid,
                                    &mut *buf
                                        .offset(
                                            ((*c).dx as libc::c_int - start as libc::c_int
                                                + 1 as libc::c_int) as isize,
                                        ) as *mut u_char,
                                    start as libc::c_int + bytesread - (*c).dx as libc::c_int
                                        - 1 as libc::c_int,
                                );
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('8' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                            } else {
                                ret = SQ_WriteStar(
                                    27 as libc::c_int as libc::c_char as u_char,
                                ) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                ret = SQ_WriteStar('P' as i32 as u_char) as libc::c_int;
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                            }
                            i = (*c).dx as libc::c_int - start as libc::c_int;
                            while i < bytesread {
                                *buf
                                    .offset(
                                        i as isize,
                                    ) = *buf.offset((i + 1 as libc::c_int) as isize);
                                i += 1;
                                i;
                            }
                            bytesread -= 1;
                            bytesread;
                            break;
                        } else {
                            *buf.offset(bytesread as isize) = curr;
                            return bytesread;
                        }
                    }
                }
                (*c).dkey_len += 1;
                (*c).dkey_len;
            }
            if editing as libc::c_int != 0 || in_hist as libc::c_int == 1 as libc::c_int
            {
                continue;
            }
        }
        if in_hist as libc::c_int > -(1 as libc::c_int)
            || (*c).options as u_int64 & MASK[0 as libc::c_int as usize] != 0
        {
            if in_hist as libc::c_int == -(1 as libc::c_int)
                && (*c).in_term.iterm == CRLF
            {
                if curr as libc::c_int == 13 as libc::c_int {
                    crflag = 1 as libc::c_int;
                } else if curr as libc::c_int == 10 as libc::c_int
                    && crflag == 1 as libc::c_int
                {
                    if in_hist as libc::c_int == 1 as libc::c_int {
                        in_hist = 0 as libc::c_int as libc::c_short;
                    }
                    editing = 0 as libc::c_int as libc::c_char;
                    (*c).dkey_len = 2 as libc::c_int as libc::c_short;
                    (*c)
                        .dkey[0 as libc::c_int
                        as usize] = 13 as libc::c_int as libc::c_char as u_char;
                    (*c)
                        .dkey[1 as libc::c_int
                        as usize] = 10 as libc::c_int as libc::c_char as u_char;
                    (*c).dkey[2 as libc::c_int as usize] = '\0' as i32 as u_char;
                    return bytesread - 1 as libc::c_int;
                }
            } else if (curr as libc::c_int) < 128 as libc::c_int {
                if in_hist as libc::c_int > -(1 as libc::c_int)
                    && curr as libc::c_int == 13 as libc::c_int
                    || in_hist as libc::c_int == -(1 as libc::c_int)
                        && (*c)
                            .in_term
                            .interm[(curr as libc::c_int / 64 as libc::c_int) as usize]
                            & MASK[(curr as libc::c_int % 64 as libc::c_int) as usize]
                            != 0
                {
                    *buf.offset(bytesread as isize) = curr;
                    if in_hist as libc::c_int == 1 as libc::c_int {
                        in_hist = 0 as libc::c_int as libc::c_short;
                    }
                    editing = 0 as libc::c_int as libc::c_char;
                    (*c).dkey_len = 1 as libc::c_int as libc::c_short;
                    (*c)
                        .dkey[0 as libc::c_int
                        as usize] = *buf.offset(bytesread as isize);
                    (*c).dkey[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                    return bytesread;
                }
            }
        }
        if (*c).options as u_int64 & MASK[2 as libc::c_int as usize] != 0
            && maxbyt > 1 as libc::c_int
            && ((curr as libc::c_int) < 32 as libc::c_int
                || curr as libc::c_int > 126 as libc::c_int)
        {
            continue;
        }
        if editing != 0 {
            if start as libc::c_int + bytesread < w.ws_col as libc::c_int {
                ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                    as libc::c_int;
                if ret < 0 as libc::c_int {
                    return ret;
                }
                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                if ret < 0 as libc::c_int {
                    return ret;
                }
                ret = SQ_WriteStar('@' as i32 as u_char) as libc::c_int;
                if ret < 0 as libc::c_int {
                    return ret;
                }
            }
            i = bytesread - 1 as libc::c_int;
            while i >= (*c).dx as libc::c_int - start as libc::c_int {
                *buf.offset((i + 1 as libc::c_int) as isize) = *buf.offset(i as isize);
                i -= 1;
                i;
            }
            *buf.offset(((*c).dx as libc::c_int - start as libc::c_int) as isize) = curr;
        } else {
            *buf.offset(bytesread as isize) = curr;
        }
        if (*c).options as u_int64 & MASK[3 as libc::c_int as usize] != 0
            || in_hist as libc::c_int > -(1 as libc::c_int)
        {
            writebuf.len = 1 as libc::c_int as u_short;
            sprintf(
                (writebuf.buf).as_mut_ptr() as *mut libc::c_char,
                b"%c\0" as *const u8 as *const libc::c_char,
                curr as libc::c_int,
            );
            ret = SQ_Write(&mut writebuf);
            if ret < 0 as libc::c_int {
                return ret;
            }
        }
        if (start as libc::c_int + bytesread + 1 as libc::c_int)
            % w.ws_col as libc::c_int == 0
        {
            j = 0 as libc::c_int;
            value = '\0' as i32 as libc::c_char;
            ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar('6' as i32 as u_char) as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar('n' as i32 as u_char) as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            loop {
                ret = read(
                    oid,
                    cpr.as_mut_ptr() as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                ) as libc::c_int;
                if !(ret != 0) {
                    break;
                }
                if ret == -(1 as libc::c_int) {
                    return getError(0 as libc::c_int, *__error());
                }
                if !(cpr[0 as libc::c_int as usize] as libc::c_int == 27 as libc::c_int)
                {
                    continue;
                }
                ret = read(
                    oid,
                    cpr.as_mut_ptr() as *mut libc::c_void,
                    12 as libc::c_int as size_t,
                ) as libc::c_int;
                if ret == -(1 as libc::c_int) {
                    return getError(0 as libc::c_int, *__error());
                }
                ret = sscanf(
                    cpr.as_mut_ptr(),
                    b"[%d;%*d%c\0" as *const u8 as *const libc::c_char,
                    &mut j as *mut libc::c_int,
                    &mut value as *mut libc::c_char,
                );
                if ret == -(1 as libc::c_int) {
                    return getError(0 as libc::c_int, *__error());
                }
                if value as libc::c_int == 'R' as i32 {
                    break;
                }
                ret = read(
                    oid,
                    cpr.as_mut_ptr() as *mut libc::c_void,
                    12 as libc::c_int as size_t,
                ) as libc::c_int;
                if ret == -(1 as libc::c_int) {
                    return getError(0 as libc::c_int, *__error());
                }
                break;
            }
            if (start as libc::c_int + bytesread + 1 as libc::c_int
                - (*c).dx as libc::c_int) / w.ws_col as libc::c_int + j
                == w.ws_row as libc::c_int
            {
                ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                    as libc::c_int;
                if ret < 0 as libc::c_int {
                    return ret;
                }
                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                if ret < 0 as libc::c_int {
                    return ret;
                }
                ret = SQ_WriteStar('S' as i32 as u_char) as libc::c_int;
                if ret < 0 as libc::c_int {
                    return ret;
                }
                ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                    as libc::c_int;
                if ret < 0 as libc::c_int {
                    return ret;
                }
                ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
                if ret < 0 as libc::c_int {
                    return ret;
                }
                ret = SQ_WriteStar('A' as i32 as u_char) as libc::c_int;
                if ret < 0 as libc::c_int {
                    return ret;
                }
            }
        }
        if (*c).dx as libc::c_int % w.ws_col as libc::c_int == 0 {
            ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar('E' as i32 as u_char) as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
        }
        if editing as libc::c_int != 0
            && start as libc::c_int + bytesread + 1 as libc::c_int
                >= w.ws_col as libc::c_int
        {
            ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar('[' as i32 as u_char) as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar('J' as i32 as u_char) as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar('7' as i32 as u_char) as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_Device_Write(
                oid,
                &mut *buf
                    .offset(((*c).dx as libc::c_int - start as libc::c_int) as isize)
                    as *mut u_char,
                start as libc::c_int + bytesread - (*c).dx as libc::c_int
                    + 1 as libc::c_int,
            );
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar(27 as libc::c_int as libc::c_char as u_char)
                as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
            ret = SQ_WriteStar('8' as i32 as u_char) as libc::c_int;
            if ret < 0 as libc::c_int {
                return ret;
            }
        }
        bytesread += 1;
        bytesread;
    };
}
#[no_mangle]
pub unsafe extern "C" fn initFORK(mut f: *mut forktab) {
    (*f).job_no = -(1 as libc::c_int);
    (*f).pid = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn initSERVER(
    mut chan: libc::c_int,
    mut size: u_int,
) -> libc::c_int {
    let mut s: *mut servertab = 0 as *mut servertab;
    let mut f: *mut forktab = 0 as *mut forktab;
    if size > (*systab).maxjob {
        return getError(1 as libc::c_int, 42 as libc::c_int);
    }
    s = &mut (*((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)).s;
    if size != 0 {
        f = malloc(
            (::core::mem::size_of::<forktab>() as libc::c_ulong)
                .wrapping_mul(size as libc::c_ulong),
        ) as *mut forktab;
        if f.is_null() {
            return getError(0 as libc::c_int, *__error());
        }
    }
    let mut index: u_int = 0 as libc::c_int as u_int;
    while index < size {
        initFORK(&mut *f.offset(index as isize));
        index = index.wrapping_add(1);
        index;
    }
    (*s).slots = size as libc::c_int;
    (*s).taken = 0 as libc::c_int;
    (*s).cid = -(1 as libc::c_int);
    (*s).name[0 as libc::c_int as usize] = '\0' as i32 as u_char;
    (*s).forked = f;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn openSERVER(
    mut chan: libc::c_int,
    mut oper: *mut libc::c_char,
) -> libc::c_int {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    ptr = strchr(oper, '=' as i32);
    if ptr.is_null() {
        (*c).mode = 7 as libc::c_int as u_char;
        ret = initSERVER(chan, 0 as libc::c_int as u_int);
        if ret < 0 as libc::c_int {
            return ret;
        }
    } else {
        ptr = ptr.offset(1);
        ptr;
        if *ptr as libc::c_int == '\0' as i32 {
            (*c).mode = 7 as libc::c_int as u_char;
            ret = initSERVER(chan, 0 as libc::c_int as u_int);
            if ret < 0 as libc::c_int {
                return ret;
            }
        } else {
            let mut size: libc::c_int = atoi(ptr);
            if size < 1 as libc::c_int {
                return getError(0 as libc::c_int, *__error())
            } else {
                ret = initSERVER(chan, size as u_int);
                if ret < 0 as libc::c_int {
                    return ret;
                }
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn acceptSERVER(
    mut chan: libc::c_int,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut s: *mut servertab = 0 as *mut servertab;
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut index: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut sin: sockaddr_in = sockaddr_in {
        sin_len: 0,
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    s = &mut (*c).s;
    if (*s).slots > 0 as libc::c_int && (*s).taken > 0 as libc::c_int {
        index = 0 as libc::c_int;
        while index < (*s).slots {
            if (*((*s).forked).offset(index as isize)).pid != -(1 as libc::c_int) {
                if (*((*systab).jobtab)
                    .offset(
                        ((*((*s).forked).offset(index as isize)).job_no
                            - 1 as libc::c_int) as isize,
                    ))
                    .pid != (*((*s).forked).offset(index as isize)).pid
                {
                    initFORK(&mut *((*s).forked).offset(index as isize));
                    (*s).taken -= 1;
                    (*s).taken;
                }
            }
            index += 1;
            index;
        }
    }
    if (*s).cid == -(1 as libc::c_int) {
        let mut ret: libc::c_int = 0;
        (*s).cid = SQ_Tcpip_Accept((*c).fid, tout);
        if (*s).cid < 0 as libc::c_int {
            (*s).cid = -(1 as libc::c_int);
            return (*s).cid;
        }
        len = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_int;
        ret = getpeername(
            (*s).cid,
            &mut sin as *mut sockaddr_in as *mut sockaddr,
            &mut len as *mut libc::c_int as *mut socklen_t,
        );
        if ret == -(1 as libc::c_int) {
            return getError(0 as libc::c_int, *__error());
        }
        len = ::core::mem::size_of::<in_addr>() as libc::c_ulong as libc::c_int;
        snprintf(
            ((*s).name).as_mut_ptr() as *mut libc::c_char,
            256 as libc::c_int as libc::c_ulong,
            b"%s %u\0" as *const u8 as *const libc::c_char,
            inet_ntoa(sin.sin_addr),
            (if 0 != 0 {
                ((sin.sin_port as libc::c_uint & 0xff00 as libc::c_uint)
                    >> 8 as libc::c_int
                    | (sin.sin_port as libc::c_uint & 0xff as libc::c_uint)
                        << 8 as libc::c_int) as __uint16_t as libc::c_int
            } else {
                _OSSwapInt16(sin.sin_port) as libc::c_int
            }) as __uint16_t as libc::c_int,
        );
    }
    if (*s).slots > 0 as libc::c_int && (*s).taken < (*s).slots {
        let mut slot: *mut forktab = 0 as *mut forktab;
        let mut jobno: libc::c_int = 0;
        index = 0 as libc::c_int;
        while index < (*s).slots {
            if (*((*s).forked).offset(index as isize)).pid == -(1 as libc::c_int) {
                slot = &mut *((*s).forked).offset(index as isize) as *mut forktab;
                index = (*s).slots;
            }
            index += 1;
            index;
        }
        if slot.is_null() {
            return getError(1 as libc::c_int, 20 as libc::c_int);
        }
        jobno = ForkIt(1 as libc::c_int);
        if jobno > 0 as libc::c_int {
            (*slot).job_no = jobno;
            (*slot)
                .pid = (*((*systab).jobtab).offset((jobno - 1 as libc::c_int) as isize))
                .pid;
            (*s).taken += 1;
            (*s).taken;
            close((*s).cid);
            (*s).cid = -(1 as libc::c_int);
            (*s).name[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            return 0 as libc::c_int;
        } else if jobno < 0 as libc::c_int {
            c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
                as *mut SQ_Chan;
            s = &mut (*c).s;
            (*s).slots = 0 as libc::c_int;
            (*s).taken = 0 as libc::c_int;
            free((*s).forked as *mut libc::c_void);
            close((*c).fid);
            (*c).mode = 8 as libc::c_int as u_char;
            (*s).forked = 0 as *mut forktab;
            (*c).fid = (*s).cid;
            strncpy(
                ((*c).name).as_mut_ptr() as *mut libc::c_char,
                ((*s).name).as_mut_ptr() as *mut libc::c_char,
                256 as libc::c_int as libc::c_ulong,
            );
            (*s).cid = -(1 as libc::c_int);
            (*s).name[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            return (*c).fid;
        } else {
            if *__error() != 0 {
                return getError(0 as libc::c_int, *__error());
            }
            return getError(1 as libc::c_int, 49 as libc::c_int);
        }
    }
    return (*s).cid;
}
#[no_mangle]
pub unsafe extern "C" fn closeSERVERClient(mut chan: libc::c_int) -> libc::c_int {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    if (*c).type_0 as libc::c_int == 2 as libc::c_int {
        if (*c).mode as libc::c_int == 7 as libc::c_int {
            if (*c).s.cid > -(1 as libc::c_int) {
                close((*c).s.cid);
                free((*c).s.forked as *mut libc::c_void);
                initSERVER(chan, 0 as libc::c_int as u_int);
            }
        } else if (*c).mode as libc::c_int == 6 as libc::c_int {
            if (*c).s.cid > -(1 as libc::c_int) {
                close((*c).s.cid);
                (*c).s.cid = -(1 as libc::c_int);
                (*c).s.name[0 as libc::c_int as usize] = '\0' as i32 as u_char;
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn closeSERVER(mut chan: libc::c_int) -> libc::c_int {
    let mut c: *mut SQ_Chan = 0 as *mut SQ_Chan;
    let mut s: *mut servertab = 0 as *mut servertab;
    c = &mut *((*partab.jobtab).seqio).as_mut_ptr().offset(chan as isize)
        as *mut SQ_Chan;
    s = &mut (*c).s;
    match (*c).mode as libc::c_int {
        5 => {
            close((*c).fid);
            (*c).type_0 = 0 as libc::c_int as u_char;
        }
        6 => {
            if (*s).cid != -(1 as libc::c_int) {
                close((*s).cid);
            }
            free((*s).forked as *mut libc::c_void);
            close((*c).fid);
            (*c).type_0 = 0 as libc::c_int as u_char;
        }
        7 => {
            if (*s).cid != -(1 as libc::c_int) {
                close((*s).cid);
            }
            close((*c).fid);
            (*c).type_0 = 0 as libc::c_int as u_char;
        }
        8 => {
            close((*c).fid);
            (*c).type_0 = 0 as libc::c_int as u_char;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
