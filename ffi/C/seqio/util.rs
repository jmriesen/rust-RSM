#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type GBD;
    fn __error() -> *mut libc::c_int;
    fn __darwin_check_fd_set_overflow(
        _: libc::c_int,
        _: *const libc::c_void,
        _: libc::c_int,
    ) -> libc::c_int;
    fn select(
        _: libc::c_int,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut timeval,
    ) -> libc::c_int;
    fn raise(_: libc::c_int) -> libc::c_int;
    static mut partab: partab_struct;
    fn DoInfo();
}
pub type __int32_t = libc::c_int;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_suseconds_t = __int32_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__int32_t; 32],
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
#[inline(always)]
unsafe extern "C" fn __darwin_check_fd_set(
    mut _a: libc::c_int,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    if ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_void,
                libc::c_int,
            ) -> libc::c_int,
        >,
        uintptr_t,
    >(
        Some(
            __darwin_check_fd_set_overflow
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    ) != 0 as libc::c_int as uintptr_t
    {
        return __darwin_check_fd_set_overflow(_a, _b, 0 as libc::c_int)
    } else {
        return 1 as libc::c_int
    };
}
#[inline(always)]
unsafe extern "C" fn __darwin_fd_set(mut _fd: libc::c_int, _p: *mut fd_set) {
    if __darwin_check_fd_set(_fd, _p as *const libc::c_void) != 0 {
        (*_p)
            .fds_bits[(_fd as libc::c_ulong)
            .wrapping_div(
                (::core::mem::size_of::<__int32_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            |= ((1 as libc::c_int as libc::c_ulong)
                << (_fd as libc::c_ulong)
                    .wrapping_rem(
                        (::core::mem::size_of::<__int32_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )) as __int32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getError(
    mut type_0: libc::c_int,
    mut errnum: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    match type_0 {
        0 => {
            err = 200 as libc::c_int + 200 as libc::c_int + errnum;
        }
        1 => {
            err = 200 as libc::c_int + errnum;
        }
        _ => {
            err = 20 as libc::c_int + 200 as libc::c_int;
        }
    }
    *__error() = 0 as libc::c_int;
    return -err;
}
#[no_mangle]
pub unsafe extern "C" fn setSignalBitMask(mut sig: libc::c_int) {
    if sig == 3 as libc::c_int {
        DoInfo();
    } else {
        let mut mask: libc::c_int = 0;
        (*partab.jobtab).attention = 1 as libc::c_int;
        mask = ((1 as libc::c_uint) << sig) as libc::c_int;
        (*partab.jobtab).trap |= mask as u_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn seqioSelect(
    mut sid: libc::c_int,
    mut type_0: libc::c_int,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut nfds: libc::c_int = 0;
    let mut fds: fd_set = fd_set { fds_bits: [0; 32] };
    let mut ret: libc::c_int = 0;
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    nfds = sid + 1 as libc::c_int;
    __darwin_fd_set(sid, &mut fds);
    if tout == 0 as libc::c_int {
        timeout.tv_sec = 0 as libc::c_int as __darwin_time_t;
        timeout.tv_usec = 0 as libc::c_int;
        if type_0 == 0 as libc::c_int {
            ret = select(
                nfds,
                &mut fds,
                0 as *mut fd_set,
                0 as *mut fd_set,
                &mut timeout,
            );
        } else {
            ret = select(
                nfds,
                0 as *mut fd_set,
                &mut fds,
                0 as *mut fd_set,
                &mut timeout,
            );
        }
    } else if type_0 == 0 as libc::c_int {
        ret = select(
            nfds,
            &mut fds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
    } else {
        ret = select(
            nfds,
            0 as *mut fd_set,
            &mut fds,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
    }
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error())
    } else if ret == 0 as libc::c_int {
        if raise(14 as libc::c_int) != 0 {
            return getError(0 as libc::c_int, *__error());
        }
        return -(1 as libc::c_int);
    } else {
        return 0 as libc::c_int
    };
}
