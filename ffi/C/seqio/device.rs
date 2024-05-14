#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type GBD;
    fn __error() -> *mut libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn raise(_: libc::c_int) -> libc::c_int;
    fn tcgetattr(_: libc::c_int, _: *mut termios) -> libc::c_int;
    fn tcsetattr(_: libc::c_int, _: libc::c_int, _: *const termios) -> libc::c_int;
    fn isatty(_: libc::c_int) -> libc::c_int;
    fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    static mut partab: partab_struct;
    fn getError(type_0: libc::c_int, errnum: libc::c_int) -> libc::c_int;
}
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
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
pub unsafe extern "C" fn SQ_Device_Open(
    mut device: *mut libc::c_char,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut flag: libc::c_int = 0;
    match op {
        1 => {
            flag = 0x1 as libc::c_int;
        }
        2 => {
            flag = 0 as libc::c_int;
        }
        4 => {
            flag = 0x2 as libc::c_int;
        }
        _ => return getError(1 as libc::c_int, 21 as libc::c_int),
    }
    loop {
        let mut did: libc::c_int = open(device, flag, 0 as libc::c_int);
        if did == -(1 as libc::c_int) {
            if *__error() != 16 as libc::c_int {
                return getError(0 as libc::c_int, *__error())
            } else if (*partab.jobtab).trap & 16384 as libc::c_int as u_int != 0 {
                return -(1 as libc::c_int)
            }
        } else {
            return did
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Device_Write(
    mut did: libc::c_int,
    mut writebuf: *mut u_char,
    mut nbytes: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = write(did, writebuf as *const libc::c_void, nbytes as size_t) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Device_Read(
    mut did: libc::c_int,
    mut readbuf: *mut u_char,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = isatty(did);
    if ret == 1 as libc::c_int {
        return SQ_Device_Read_TTY(did, readbuf, tout)
    } else {
        return getError(1 as libc::c_int, 24 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Device_Read_TTY(
    mut did: libc::c_int,
    mut readbuf: *mut u_char,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut settings: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0; 20],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut ret: libc::c_int = 0;
    let mut rret: libc::c_int = 0;
    if tout == 0 as libc::c_int {
        ret = tcgetattr(did, &mut settings);
        if ret == -(1 as libc::c_int) {
            return getError(0 as libc::c_int, *__error());
        }
        settings.c_cc[16 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
        ret = tcsetattr(did, 0 as libc::c_int, &mut settings);
        if ret == -(1 as libc::c_int) {
            return getError(0 as libc::c_int, *__error());
        }
    }
    rret = read(did, readbuf as *mut libc::c_void, 1 as libc::c_int as size_t)
        as libc::c_int;
    if tout == 0 as libc::c_int {
        ret = tcgetattr(did, &mut settings);
        if ret == -(1 as libc::c_int) {
            return getError(0 as libc::c_int, *__error());
        }
        settings.c_cc[16 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
        ret = tcsetattr(did, 0 as libc::c_int, &mut settings);
        if ret == -(1 as libc::c_int) {
            return getError(0 as libc::c_int, *__error());
        }
        if rret == 0 as libc::c_int {
            (*partab.jobtab).trap |= 16384 as libc::c_int as u_int;
            return -(1 as libc::c_int);
        }
    }
    if rret == -(1 as libc::c_int) {
        if *__error() == 35 as libc::c_int {
            if raise(14 as libc::c_int) != 0 {
                return getError(0 as libc::c_int, *__error());
            }
        }
        return getError(0 as libc::c_int, *__error());
    } else {
        return rret
    };
}
