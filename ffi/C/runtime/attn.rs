#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    pub type GBD;
    static mut __stdinp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn freopen(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn signal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn ioctl(_: libc::c_int, _: libc::c_ulong, _: ...) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn kill(_: pid_t, _: libc::c_int) -> libc::c_int;
    fn fork() -> pid_t;
    fn getpid() -> pid_t;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    fn sched_yield() -> libc::c_int;
    fn __error() -> *mut libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn SQ_Close(chan: libc::c_int) -> libc::c_short;
    fn UTIL_String_Mvar(
        var: *mut mvar,
        str: *mut u_char,
        max_subs: libc::c_int,
    ) -> libc::c_short;
    fn CleanJob(job: libc::c_int);
    fn panic(msg: *mut libc::c_char);
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
    fn setSignal(sig: libc::c_int, flag: libc::c_int) -> libc::c_int;
    static mut failed_tty: libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
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
pub type rbd = RBD;
#[no_mangle]
pub unsafe extern "C" fn attention() -> libc::c_short {
    let mut s: libc::c_short = 0 as libc::c_int as libc::c_short;
    if (*partab.jobtab).trap & (1 as libc::c_uint) << 2 as libc::c_int != 0 {
        (*partab.jobtab)
            .trap = (*partab.jobtab).trap & !((1 as libc::c_uint) << 2 as libc::c_int);
        (*partab.jobtab)
            .async_error = -(51 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if (*partab.jobtab).trap & 1 as libc::c_int as u_int != 0 {
        (*partab.jobtab).trap = (*partab.jobtab).trap & !(1 as libc::c_int) as u_int;
        (*partab.jobtab)
            .async_error = -(66 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if (*partab.jobtab).trap & (1 as libc::c_uint) << 30 as libc::c_int != 0 {
        (*partab.jobtab)
            .trap = (*partab.jobtab).trap & !((1 as libc::c_uint) << 30 as libc::c_int);
        (*partab.jobtab)
            .async_error = -(67 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if (*partab.jobtab).trap & (1 as libc::c_uint) << 31 as libc::c_int != 0 {
        (*partab.jobtab)
            .trap = (*partab.jobtab).trap & !((1 as libc::c_uint) << 31 as libc::c_int);
        (*partab.jobtab)
            .async_error = -(68 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if (*partab.jobtab).trap
        & ((1 as libc::c_uint) << 3 as libc::c_int
            | (1 as libc::c_uint) << 15 as libc::c_int
            | (1 as libc::c_uint) << 17 as libc::c_int) != 0
    {
        (*partab.jobtab).trap = 0 as libc::c_int as u_int;
        (*partab.jobtab).async_error = 0 as libc::c_int as libc::c_short;
        (*partab.jobtab).attention = 0 as libc::c_int;
        return 1 as libc::c_int as libc::c_short;
    }
    (*partab.jobtab).trap = 0 as libc::c_int as u_int;
    (*partab.jobtab).attention = 0 as libc::c_int;
    s = (*partab.jobtab).async_error;
    (*partab.jobtab).async_error = 0 as libc::c_int as libc::c_short;
    if s as libc::c_int == 0 as libc::c_int && partab.debug > 0 as libc::c_int {
        if partab.debug <= (*partab.jobtab).commands as libc::c_int {
            s = 256 as libc::c_int as libc::c_short;
        } else {
            (*partab.jobtab).attention = 1 as libc::c_int;
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn DoInfo() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ct: [libc::c_char; 400] = [0; 400];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut mvar = 0 as *mut mvar;
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    memcpy(
        ct.as_mut_ptr() as *mut libc::c_void,
        b"\x1B7\x1B[99;1H\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        9 as libc::c_int as libc::c_ulong,
    );
    i = 9 as libc::c_int;
    i
        += sprintf(
            &mut *ct.as_mut_ptr().offset(i as isize) as *mut libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long as libc::c_int
                + 1 as libc::c_int,
        );
    i
        += sprintf(
            &mut *ct.as_mut_ptr().offset(i as isize) as *mut libc::c_char,
            b" (%d) \0" as *const u8 as *const libc::c_char,
            (*partab.jobtab).pid,
        );
    p = &mut (*((*partab.jobtab).dostk)
        .as_mut_ptr()
        .offset((*partab.jobtab).cur_do as isize))
        .rounam as *mut var_u as *mut libc::c_char;
    j = 0 as libc::c_int;
    while j < 32 as libc::c_int && *p.offset(j as isize) as libc::c_int != 0 {
        let fresh0 = j;
        j = j + 1;
        let fresh1 = i;
        i = i + 1;
        ct[fresh1 as usize] = *p.offset(fresh0 as isize);
    }
    i
        += sprintf(
            &mut *ct.as_mut_ptr().offset(i as isize) as *mut libc::c_char,
            b" Cmds: %u \0" as *const u8 as *const libc::c_char,
            (*partab.jobtab).commands,
        );
    i
        += sprintf(
            &mut *ct.as_mut_ptr().offset(i as isize) as *mut libc::c_char,
            b"Grefs: %u \0" as *const u8 as *const libc::c_char,
            (*partab.jobtab).grefs,
        );
    var = &mut (*partab.jobtab).last_ref;
    if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
        i
            += UTIL_String_Mvar(
                var,
                &mut *ct.as_mut_ptr().offset(i as isize) as *mut libc::c_char
                    as *mut u_char,
                63 as libc::c_int,
            ) as libc::c_int;
    }
    if ioctl(
        1 as libc::c_int,
        0x40000000 as libc::c_int as __uint32_t as libc::c_ulong
            | (::core::mem::size_of::<winsize>() as libc::c_ulong
                & 0x1fff as libc::c_int as libc::c_ulong) << 16 as libc::c_int
            | (('t' as i32) << 8 as libc::c_int) as libc::c_ulong
            | 104 as libc::c_int as libc::c_ulong,
        &mut w as *mut winsize,
    ) != -(1 as libc::c_int) && i > w.ws_col as libc::c_int + 9 as libc::c_int
    {
        i = w.ws_col as libc::c_int + 9 as libc::c_int;
    } else if i > 89 as libc::c_int {
        i = 89 as libc::c_int;
    }
    memcpy(
        &mut *ct.as_mut_ptr().offset(i as isize) as *mut libc::c_char
            as *mut libc::c_void,
        b"\x1B[K\x1B8\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    fprintf(__stderrp, b"%s\0" as *const u8 as *const libc::c_char, ct.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn ForkIt(mut cft: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut mid: libc::c_int = -(1 as libc::c_int);
    let mut j: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut k: u_int = 0 as libc::c_int as u_int;
    while k < (*systab).maxjob {
        ret = (*((*systab).jobtab).offset(k as isize)).pid;
        if !(ret != 0) {
            break;
        }
        if kill(ret, 0 as libc::c_int) != 0 {
            if *__error() == 3 as libc::c_int {
                CleanJob(k.wrapping_add(1 as libc::c_int as u_int) as libc::c_int);
                break;
            }
        }
        k = k.wrapping_add(1);
        k;
    }
    if cft > -(1 as libc::c_int) {
        i = SemOp(0 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int)
            as libc::c_int;
        if i < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        let mut k_0: u_int = 0 as libc::c_int as u_int;
        while k_0 < (*systab).maxjob {
            if (*((*systab).jobtab).offset(k_0 as isize)).pid == 0 as libc::c_int {
                mid = k_0 as libc::c_int;
                break;
            } else {
                k_0 = k_0.wrapping_add(1);
                k_0;
            }
        }
        if mid == -(1 as libc::c_int) {
            SemOp(0 as libc::c_int, (*systab).maxjob as libc::c_int);
            return 0 as libc::c_int;
        }
    }
    signal(
        20 as libc::c_int,
        ::core::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t),
    );
    i = fork();
    if i == 0 {
        failed_tty = -(1 as libc::c_int);
        setSignal(2 as libc::c_int, 1 as libc::c_int);
    }
    if cft == -(1 as libc::c_int) {
        if i == 0 {
            j = freopen(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                b"r\0" as *const u8 as *const libc::c_char,
                __stdinp,
            ) as *mut libc::c_void;
            if j.is_null() {
                fprintf(
                    __stderrp,
                    b"freopen() errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
                    *__error(),
                    strerror(*__error()),
                );
            }
            j = freopen(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
                __stdoutp,
            ) as *mut libc::c_void;
            if j.is_null() {
                fprintf(
                    __stderrp,
                    b"freopen() errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
                    *__error(),
                    strerror(*__error()),
                );
            }
            j = freopen(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
                __stderrp,
            ) as *mut libc::c_void;
            if j.is_null() {
                fprintf(
                    __stderrp,
                    b"freopen() errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
                    *__error(),
                    strerror(*__error()),
                );
            }
        }
        return i;
    }
    if i == -(1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"fork() errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
        SemOp(0 as libc::c_int, (*systab).maxjob as libc::c_int);
        return 0 as libc::c_int;
    } else if i > 0 as libc::c_int {
        memcpy(
            &mut *((*systab).jobtab).offset(mid as isize) as *mut jobtab
                as *mut libc::c_void,
            partab.jobtab as *const libc::c_void,
            ::core::mem::size_of::<jobtab>() as libc::c_ulong,
        );
        (*((*systab).jobtab).offset(mid as isize)).pid = i;
        SemOp(0 as libc::c_int, (*systab).maxjob as libc::c_int);
        return mid + 1 as libc::c_int;
    }
    ret = -((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
        + 1 as libc::c_int as libc::c_long) as libc::c_int;
    partab.jobtab = &mut *((*systab).jobtab).offset(mid as isize) as *mut jobtab;
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        if getpid() == (*partab.jobtab).pid {
            break;
        }
        SchedYield();
        i += 1;
        i;
    }
    if i > 9999 as libc::c_int {
        i = 0 as libc::c_int;
        while !(getpid() == (*partab.jobtab).pid) {
            if i > 120 as libc::c_int {
                panic(
                    b"ForkIt: Child job never got setup\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            sleep(1 as libc::c_int as libc::c_uint);
            i += 1;
            i;
        }
    }
    if cft != 0 {
        i = SemOp(3 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int)
            as libc::c_int;
        if i < 0 as libc::c_int {
            panic(
                b"Can't get SEM_ROU in ForkIt()\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        i = (*partab.jobtab).cur_do;
        while i > 0 as libc::c_int {
            if (*partab.jobtab).dostk[i as usize].flags as libc::c_int & 2 as libc::c_int
                != 0
            {
                let ref mut fresh2 = (*((*partab.jobtab).dostk[i as usize].routine
                    as *mut rbd))
                    .attached;
                *fresh2 = (*fresh2).wrapping_add(1);
                *fresh2;
            }
            i -= 1;
            i;
        }
        SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
        return ret;
    }
    i = 1 as libc::c_int;
    while i < 64 as libc::c_int {
        let fresh3 = i;
        i = i + 1;
        SQ_Close(fresh3);
    }
    j = freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        __stdinp,
    ) as *mut libc::c_void;
    if j.is_null() {
        fprintf(
            __stderrp,
            b"freopen() errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
    }
    j = freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
        __stdoutp,
    ) as *mut libc::c_void;
    if j.is_null() {
        fprintf(
            __stderrp,
            b"freopen() errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
    }
    j = freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
        __stderrp,
    ) as *mut libc::c_void;
    if j.is_null() {
        fprintf(
            __stderrp,
            b"freopen() errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SchedYield() {
    sched_yield();
}
