#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type GBD;
    pub type RBD;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn localtime(_: *const time_t) -> *mut tm;
    fn time(_: *mut time_t) -> time_t;
    fn uname(_: *mut utsname) -> libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn ST_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn ST_Set(var: *mut mvar, data: *mut cstring) -> libc::c_int;
    fn UTIL_Key_Build(src: *mut cstring, dest: *mut u_char) -> libc::c_short;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type off_t = __darwin_off_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 256],
    pub nodename: [libc::c_char; 256],
    pub release: [libc::c_char; 256],
    pub version: [libc::c_char; 256],
    pub machine: [libc::c_char; 256],
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
pub unsafe extern "C" fn cstringtoi(mut str: *mut cstring) -> libc::c_int {
    let mut ret: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut i: libc::c_int = 0;
    let mut minus: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*str).len as libc::c_int
        && ((*str).buf[i as usize] as libc::c_int == '-' as i32
            || (*str).buf[i as usize] as libc::c_int == '+' as i32)
    {
        if (*str).buf[i as usize] as libc::c_int == '-' as i32 {
            minus = (minus == 0) as libc::c_int;
        }
        i += 1;
        i;
    }
    while i < (*str).len as libc::c_int {
        if (*str).buf[i as usize] as libc::c_int > '9' as i32
            || ((*str).buf[i as usize] as libc::c_int) < '0' as i32
        {
            break;
        }
        ret = ret * 10 as libc::c_int as libc::c_long
            + ((*str).buf[i as usize] as libc::c_int - '0' as i32) as libc::c_long;
        if ret > 2147483647 as libc::c_int as libc::c_long {
            if minus != 0 {
                return -(2147483647 as libc::c_int) - 1 as libc::c_int;
            }
            return 2147483647 as libc::c_int;
        }
        i += 1;
        i;
    }
    if (*systab).historic & 1 as libc::c_int != 0
        && i < (*str).len as libc::c_int - 1 as libc::c_int
        && (*str).buf[i as usize] as libc::c_int == 'E' as i32
    {
        let mut exp: libc::c_int = 0 as libc::c_int;
        let mut expsgn: libc::c_int = 1 as libc::c_int;
        i += 1;
        i;
        if (*str).buf[i as usize] as libc::c_int == '-' as i32 {
            expsgn = -(1 as libc::c_int);
            i += 1;
            i;
        } else if (*str).buf[i as usize] as libc::c_int == '+' as i32 {
            i += 1;
            i;
        }
        while i < (*str).len as libc::c_int {
            if ((*str).buf[i as usize] as libc::c_int) < '0' as i32
                || (*str).buf[i as usize] as libc::c_int > '9' as i32
            {
                break;
            }
            exp = exp * 10 as libc::c_int
                + ((*str).buf[i as usize] as libc::c_int - '0' as i32);
            i += 1;
            i;
        }
        if exp != 0 {
            let mut j: libc::c_long = 10 as libc::c_int as libc::c_long;
            while exp > 1 as libc::c_int {
                j *= 10 as libc::c_int as libc::c_long;
                exp -= 1;
                exp;
                if j > 2147483647 as libc::c_int as libc::c_long {
                    if expsgn > 0 as libc::c_int {
                        return 2147483647 as libc::c_int;
                    }
                    return -(2147483647 as libc::c_int) - 1 as libc::c_int;
                }
            }
            if expsgn > 0 as libc::c_int {
                ret *= j;
            } else {
                ret /= j;
            }
            if ret > 2147483647 as libc::c_int as libc::c_long {
                if minus != 0 {
                    return -(2147483647 as libc::c_int) - 1 as libc::c_int;
                }
                return 2147483647 as libc::c_int;
            }
        }
    }
    if minus != 0 {
        ret = -ret;
    }
    return ret as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cstringtob(mut str: *mut cstring) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut dp: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*str).len as libc::c_int
        && ((*str).buf[i as usize] as libc::c_int == '-' as i32
            || (*str).buf[i as usize] as libc::c_int == '+' as i32)
    {
        i += 1;
        i;
    }
    while i < (*str).len as libc::c_int {
        if !((*str).buf[i as usize] as libc::c_int == '0' as i32) {
            if (*str).buf[i as usize] as libc::c_int == '.' as i32 {
                if dp != 0 {
                    break;
                }
                dp = 1 as libc::c_int;
            } else {
                if (*str).buf[i as usize] as libc::c_int >= '1' as i32
                    && (*str).buf[i as usize] as libc::c_int <= '9' as i32
                {
                    ret = 1 as libc::c_int;
                }
                break;
            }
        }
        i += 1;
        i;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn itocstring(
    mut buf: *mut u_char,
    mut n: libc::c_int,
) -> u_short {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut p: libc::c_int = 0 as libc::c_int;
    let mut a: [libc::c_int; 12] = [0; 12];
    a[0 as libc::c_int as usize] = 0 as libc::c_int;
    if n < 0 as libc::c_int {
        let fresh0 = p;
        p = p + 1;
        *buf.offset(fresh0 as isize) = '-' as i32 as u_char;
        n = -n;
    }
    while n != 0 {
        let fresh1 = i;
        i = i + 1;
        a[fresh1 as usize] = n % 10 as libc::c_int;
        n = n / 10 as libc::c_int;
    }
    while i != 0 {
        i -= 1;
        let fresh2 = p;
        p = p + 1;
        *buf.offset(fresh2 as isize) = (a[i as usize] + 48 as libc::c_int) as u_char;
    }
    if p == 0 {
        let fresh3 = p;
        p = p + 1;
        *buf.offset(fresh3 as isize) = '0' as i32 as u_char;
    }
    *buf.offset(p as isize) = '\0' as i32 as u_char;
    return p as u_short;
}
#[no_mangle]
pub unsafe extern "C" fn uitocstring(mut buf: *mut u_char, mut n: u_int) -> u_short {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut p: libc::c_int = 0 as libc::c_int;
    let mut a: [libc::c_int; 12] = [0; 12];
    a[0 as libc::c_int as usize] = 0 as libc::c_int;
    while n != 0 {
        let fresh4 = i;
        i = i + 1;
        a[fresh4 as usize] = (n % 10 as libc::c_int as u_int) as libc::c_int;
        n = n / 10 as libc::c_int as u_int;
    }
    while i != 0 {
        i -= 1;
        let fresh5 = p;
        p = p + 1;
        *buf.offset(fresh5 as isize) = (a[i as usize] + 48 as libc::c_int) as u_char;
    }
    if p == 0 {
        let fresh6 = p;
        p = p + 1;
        *buf.offset(fresh6 as isize) = '0' as i32 as u_char;
    }
    *buf.offset(p as isize) = '\0' as i32 as u_char;
    return p as u_short;
}
#[no_mangle]
pub unsafe extern "C" fn Set_Error(
    mut err: libc::c_int,
    mut user: *mut cstring,
    mut space: *mut cstring,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut var: *mut mvar = 0 as *mut mvar;
    var = &mut partab.src_var;
    (*var).slen = 0 as libc::c_int as u_char;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        (*var).name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    memcpy(
        &mut *((*var).name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_void,
        b"$ECODE\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    t = ST_Get(var, ((*space).buf).as_mut_ptr());
    if t < 0 as libc::c_int {
        t = 0 as libc::c_int;
    }
    flag = t;
    if t < 1024 as libc::c_int {
        let mut j: libc::c_int = 0;
        let mut tmp: *mut cstring = 0 as *mut cstring;
        let mut temp: [libc::c_char; 16] = [0; 16];
        if t == 0 as libc::c_int
            || (*space).buf[(t - 1 as libc::c_int) as usize] as libc::c_int != ',' as i32
        {
            let fresh7 = t;
            t = t + 1;
            (*space).buf[fresh7 as usize] = ',' as i32 as u_char;
        }
        j = -err;
        if err == -(9999 as libc::c_int) {
            memmove(
                &mut *((*space).buf).as_mut_ptr().offset(t as isize) as *mut u_char
                    as *mut libc::c_void,
                ((*user).buf).as_mut_ptr() as *const libc::c_void,
                (*user).len as libc::c_ulong,
            );
            t += (*user).len as libc::c_int;
        } else {
            if j > 200 as libc::c_int {
                let fresh8 = t;
                t = t + 1;
                (*space).buf[fresh8 as usize] = 'Z' as i32 as u_char;
                j -= 200 as libc::c_int;
            } else {
                let fresh9 = t;
                t = t + 1;
                (*space).buf[fresh9 as usize] = 'M' as i32 as u_char;
            }
            t
                += itocstring(&mut *((*space).buf).as_mut_ptr().offset(t as isize), j)
                    as libc::c_int;
        }
        let fresh10 = t;
        t = t + 1;
        (*space).buf[fresh10 as usize] = ',' as i32 as u_char;
        (*space).buf[t as usize] = '\0' as i32 as u_char;
        (*space).len = t as u_short;
        ST_Set(var, space);
        tmp = temp.as_mut_ptr() as *mut cstring;
        (*tmp).len = itocstring(((*tmp).buf).as_mut_ptr(), (*partab.jobtab).cur_do);
        (*var).slen = UTIL_Key_Build(tmp, ((*var).key).as_mut_ptr()) as u_char;
        if flag != 0 {
            t = ST_Get(var, ((*space).buf).as_mut_ptr());
            if t < 0 as libc::c_int {
                t = 0 as libc::c_int;
            }
            flag = t;
            if t == 0 as libc::c_int
                || (*space).buf[(t - 1 as libc::c_int) as usize] as libc::c_int
                    != ',' as i32
            {
                let fresh11 = t;
                t = t + 1;
                (*space).buf[fresh11 as usize] = ',' as i32 as u_char;
            }
            j = -err;
            if err == -(9999 as libc::c_int) {
                memmove(
                    &mut *((*space).buf).as_mut_ptr().offset(t as isize) as *mut u_char
                        as *mut libc::c_void,
                    ((*user).buf).as_mut_ptr() as *const libc::c_void,
                    (*user).len as libc::c_ulong,
                );
                t += (*user).len as libc::c_int;
            } else {
                if j > 200 as libc::c_int {
                    let fresh12 = t;
                    t = t + 1;
                    (*space).buf[fresh12 as usize] = 'Z' as i32 as u_char;
                    j -= 200 as libc::c_int;
                } else {
                    let fresh13 = t;
                    t = t + 1;
                    (*space).buf[fresh13 as usize] = 'M' as i32 as u_char;
                }
                t
                    += itocstring(
                        &mut *((*space).buf).as_mut_ptr().offset(t as isize),
                        j,
                    ) as libc::c_int;
            }
            let fresh14 = t;
            t = t + 1;
            (*space).buf[fresh14 as usize] = ',' as i32 as u_char;
            (*space).buf[t as usize] = '\0' as i32 as u_char;
            (*space).len = t as u_short;
        }
        ST_Set(var, space);
    }
    return flag;
}
#[no_mangle]
pub unsafe extern "C" fn short_version(
    mut ret_buffer: *mut u_char,
    mut i: libc::c_int,
) -> libc::c_int {
    i
        += sprintf(
            &mut *ret_buffer.offset(i as isize) as *mut u_char as *mut libc::c_char,
            b"%d.%d.%d\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            77 as libc::c_int,
            0 as libc::c_int,
        );
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn rsm_version(mut ret_buffer: *mut u_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut uts: utsname = utsname {
        sysname: [0; 256],
        nodename: [0; 256],
        release: [0; 256],
        version: [0; 256],
        machine: [0; 256],
    };
    i = uname(&mut uts);
    if i == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    memcpy(
        ret_buffer as *mut libc::c_void,
        b"Reference Standard M V\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        22 as libc::c_int as libc::c_ulong,
    );
    i = 22 as libc::c_int;
    i = short_version(ret_buffer, i);
    memcpy(
        &mut *ret_buffer.offset(i as isize) as *mut u_char as *mut libc::c_void,
        b" for \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        5 as libc::c_int as libc::c_ulong,
    );
    i += 5 as libc::c_int;
    j = 0 as libc::c_int;
    loop {
        let fresh15 = j;
        j = j + 1;
        let fresh16 = i;
        i = i + 1;
        let ref mut fresh17 = *ret_buffer.offset(fresh16 as isize);
        *fresh17 = uts.sysname[fresh15 as usize] as u_char;
        if !(*fresh17 != 0) {
            break;
        }
    }
    *ret_buffer.offset((i - 1 as libc::c_int) as isize) = ' ' as i32 as u_char;
    j = 0 as libc::c_int;
    loop {
        let fresh18 = j;
        j = j + 1;
        let fresh19 = i;
        i = i + 1;
        let ref mut fresh20 = *ret_buffer.offset(fresh19 as isize);
        *fresh20 = uts.machine[fresh18 as usize] as u_char;
        if !(*fresh20 != 0) {
            break;
        }
    }
    *ret_buffer.offset((i - 1 as libc::c_int) as isize) = ' ' as i32 as u_char;
    i
        += sprintf(
            &mut *ret_buffer.offset(i as isize) as *mut u_char as *mut libc::c_char,
            b"Built %s at %s\0" as *const u8 as *const libc::c_char,
            b"May 14 2024\0" as *const u8 as *const libc::c_char,
            b"11:04:17\0" as *const u8 as *const libc::c_char,
        );
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn current_time(mut local: libc::c_short) -> time_t {
    let mut sec: time_t = time(0 as *mut time_t);
    if local != 0 {
        let mut buf: *mut tm = localtime(&mut sec);
        sec += (*buf).tm_gmtoff;
    }
    return sec;
}
