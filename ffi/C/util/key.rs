#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type GBD;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
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
pub unsafe extern "C" fn UTIL_Key_Build(
    mut src: *mut cstring,
    mut dest: *mut u_char,
) -> libc::c_short {
    let mut current_block: u64;
    let mut minus: libc::c_int = 0 as libc::c_int;
    let mut dp: libc::c_int = -(1 as libc::c_int);
    let mut to: libc::c_int = 0 as libc::c_int;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if (*src).len as libc::c_int > 127 as libc::c_int {
        return -(1 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if (*src).len as libc::c_int == 0 as libc::c_int {
        let fresh0 = to;
        to = to + 1;
        *dest.offset(fresh0 as isize) = '\0' as i32 as u_char;
        let fresh1 = to;
        to = to + 1;
        *dest.offset(fresh1 as isize) = '\0' as i32 as u_char;
        *dest.offset(to as isize) = '\0' as i32 as u_char;
        return to as libc::c_short;
    }
    if (*src).len as libc::c_int == 1 as libc::c_int
        && (*src).buf[0 as libc::c_int as usize] as libc::c_int == '0' as i32
    {
        let fresh2 = to;
        to = to + 1;
        *dest.offset(fresh2 as isize) = 64 as libc::c_int as u_char;
        let fresh3 = to;
        to = to + 1;
        *dest.offset(fresh3 as isize) = '\0' as i32 as u_char;
        *dest.offset(to as isize) = '\0' as i32 as u_char;
        return to as libc::c_short;
    }
    if (*src).buf[idx as usize] as libc::c_int == '-' as i32 {
        idx += 1;
        idx;
        minus = 1 as libc::c_int;
    }
    if (*src).buf[idx as usize] as libc::c_int == '.' as i32 {
        dp = 0 as libc::c_int;
        current_block = 8457315219000651999;
    } else if ((*src).buf[idx as usize] as libc::c_int) < '1' as i32
        || (*src).buf[idx as usize] as libc::c_int > '9' as i32
    {
        current_block = 607274259946404142;
    } else {
        current_block = 8457315219000651999;
    }
    match current_block {
        8457315219000651999 => {
            i = idx + 1 as libc::c_int;
            loop {
                if !(i < (*src).len as libc::c_int) {
                    current_block = 2719512138335094285;
                    break;
                }
                if (*src).buf[i as usize] as libc::c_int == '.' as i32 {
                    if dp != -(1 as libc::c_int) {
                        current_block = 607274259946404142;
                        break;
                    }
                    dp = i - minus;
                } else if ((*src).buf[i as usize] as libc::c_int) < '0' as i32
                    || (*src).buf[i as usize] as libc::c_int > '9' as i32
                {
                    current_block = 607274259946404142;
                    break;
                }
                i += 1;
                i;
            }
            match current_block {
                607274259946404142 => {}
                _ => {
                    if !(dp != -(1 as libc::c_int)
                        && (*src)
                            .buf[((*src).len as libc::c_int - 1 as libc::c_int) as usize]
                            as libc::c_int == '0' as i32)
                    {
                        if !(dp == (*src).len as libc::c_int - 1 as libc::c_int) {
                            if dp == -(1 as libc::c_int) {
                                dp = (*src).len as libc::c_int - minus;
                            }
                            if !(dp > 63 as libc::c_int) {
                                if minus == 0 {
                                    let fresh4 = to;
                                    to = to + 1;
                                    *dest
                                        .offset(
                                            fresh4 as isize,
                                        ) = (dp + 64 as libc::c_int) as u_char;
                                    i = 0 as libc::c_int;
                                    while i < dp {
                                        let fresh5 = to;
                                        to = to + 1;
                                        *dest.offset(fresh5 as isize) = (*src).buf[i as usize];
                                        i += 1;
                                        i;
                                    }
                                    i = dp + 1 as libc::c_int;
                                    while i < (*src).len as libc::c_int {
                                        let fresh6 = to;
                                        to = to + 1;
                                        *dest.offset(fresh6 as isize) = (*src).buf[i as usize];
                                        i += 1;
                                        i;
                                    }
                                    let fresh7 = to;
                                    to = to + 1;
                                    *dest.offset(fresh7 as isize) = '\0' as i32 as u_char;
                                    *dest.offset(to as isize) = '\0' as i32 as u_char;
                                    return to as libc::c_short;
                                }
                                let fresh8 = to;
                                to = to + 1;
                                *dest
                                    .offset(
                                        fresh8 as isize,
                                    ) = (63 as libc::c_int - dp) as u_char;
                                i = idx;
                                while i < (*src).len as libc::c_int {
                                    if (*src).buf[i as usize] as libc::c_int != '.' as i32 {
                                        let fresh9 = to;
                                        to = to + 1;
                                        *dest
                                            .offset(
                                                fresh9 as isize,
                                            ) = (57 as libc::c_int
                                            - (*src).buf[i as usize] as libc::c_int + 48 as libc::c_int)
                                            as u_char;
                                    }
                                    i += 1;
                                    i;
                                }
                                let fresh10 = to;
                                to = to + 1;
                                *dest
                                    .offset(fresh10 as isize) = 255 as libc::c_int as u_char;
                                *dest.offset(to as isize) = '\0' as i32 as u_char;
                                return to as libc::c_short;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    let fresh11 = to;
    to = to + 1;
    *dest.offset(fresh11 as isize) = 128 as libc::c_int as u_char;
    i = 0 as libc::c_int;
    while i < (*src).len as libc::c_int {
        let fresh12 = to;
        to = to + 1;
        let ref mut fresh13 = *dest.offset(fresh12 as isize);
        *fresh13 = (*src).buf[i as usize];
        if *fresh13 as libc::c_int == 0 as libc::c_int {
            return -(5 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        i += 1;
        i;
    }
    let fresh14 = to;
    to = to + 1;
    *dest.offset(fresh14 as isize) = '\0' as i32 as u_char;
    *dest.offset(to as isize) = '\0' as i32 as u_char;
    return to as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_Key_Extract(
    mut key: *mut u_char,
    mut str: *mut u_char,
    mut cnt: *mut libc::c_int,
) -> libc::c_short {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut flg: libc::c_int = 0;
    flg = *cnt;
    let fresh15 = key;
    key = key.offset(1);
    s = *fresh15 as libc::c_int;
    if (s == 0 as libc::c_int || s == 255 as libc::c_int)
        && *key as libc::c_int == 0 as libc::c_int
    {
        *cnt = 2 as libc::c_int;
        *str.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
        return 0 as libc::c_int as libc::c_short;
    }
    if s & 128 as libc::c_int != 0 {
        let mut j: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while *key.offset(i as isize) as libc::c_int != 0 as libc::c_int {
            let fresh16 = j;
            j = j + 1;
            *str.offset(fresh16 as isize) = *key.offset(i as isize);
            if *key.offset(i as isize) as libc::c_int == '"' as i32 && flg != 0 {
                let fresh17 = j;
                j = j + 1;
                *str.offset(fresh17 as isize) = '"' as i32 as u_char;
            }
            if i > 127 as libc::c_int {
                return -(1 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
            i += 1;
            i;
        }
        *str.offset(j as isize) = 0 as libc::c_int as u_char;
        *cnt = i + 2 as libc::c_int;
        return j as libc::c_short;
    }
    if s & 64 as libc::c_int != 0 {
        s &= 63 as libc::c_int;
        if *key as libc::c_int == '\0' as i32 && s == 0 as libc::c_int {
            let fresh18 = idx;
            idx = idx + 1;
            *str.offset(fresh18 as isize) = '0' as i32 as u_char;
            *str.offset(idx as isize) = '\0' as i32 as u_char;
            *cnt = 2 as libc::c_int;
            return idx as libc::c_short;
        }
        i = 0 as libc::c_int;
        while i < s {
            let fresh19 = key;
            key = key.offset(1);
            let fresh20 = idx;
            idx = idx + 1;
            *str.offset(fresh20 as isize) = *fresh19;
            i += 1;
            i;
        }
        *str.offset(idx as isize) = 0 as libc::c_int as u_char;
        *cnt = s + 2 as libc::c_int;
        if *key as libc::c_int == '\0' as i32 {
            return idx as libc::c_short;
        }
        let fresh21 = idx;
        idx = idx + 1;
        *str.offset(fresh21 as isize) = '.' as i32 as u_char;
        loop {
            let fresh22 = key;
            key = key.offset(1);
            let fresh23 = idx;
            idx = idx + 1;
            let ref mut fresh24 = *str.offset(fresh23 as isize);
            *fresh24 = *fresh22;
            if !(*fresh24 != 0) {
                break;
            }
            s += 1;
            s;
        }
        idx -= 1;
        idx;
        if s > 127 as libc::c_int {
            return -(1 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        *cnt = s + 2 as libc::c_int;
        return idx as libc::c_short;
    }
    s = 63 as libc::c_int - s;
    let fresh25 = idx;
    idx = idx + 1;
    *str.offset(fresh25 as isize) = '-' as i32 as u_char;
    i = 0 as libc::c_int;
    while i < s {
        let fresh26 = key;
        key = key.offset(1);
        let fresh27 = idx;
        idx = idx + 1;
        *str
            .offset(
                fresh27 as isize,
            ) = ('9' as i32 + '0' as i32 - *fresh26 as libc::c_int) as u_char;
        i += 1;
        i;
    }
    *str.offset(idx as isize) = 0 as libc::c_int as u_char;
    *cnt = s + 2 as libc::c_int;
    if *key as libc::c_int == 255 as libc::c_int {
        return idx as libc::c_short;
    }
    let fresh28 = idx;
    idx = idx + 1;
    *str.offset(fresh28 as isize) = '.' as i32 as u_char;
    while !(*key as libc::c_int == 255 as libc::c_int) {
        s += 1;
        s;
        let fresh29 = key;
        key = key.offset(1);
        let fresh30 = idx;
        idx = idx + 1;
        *str
            .offset(
                fresh30 as isize,
            ) = ('9' as i32 + '0' as i32 - *fresh29 as libc::c_int) as u_char;
    }
    if s > 127 as libc::c_int {
        return -(1 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    *str.offset(idx as isize) = 0 as libc::c_int as u_char;
    *cnt = s + 2 as libc::c_int;
    return idx as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_String_Key(
    mut key: *mut u_char,
    mut str: *mut u_char,
    mut max_subs: libc::c_int,
) -> libc::c_short {
    let mut count: libc::c_int = 1 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut clen: libc::c_int = 0 as libc::c_int;
    let fresh31 = idx;
    idx = idx + 1;
    len = *key.offset(fresh31 as isize) as libc::c_int;
    let fresh32 = clen;
    clen = clen + 1;
    *str.offset(fresh32 as isize) = '(' as i32 as u_char;
    while len > 1 as libc::c_int {
        let mut string: libc::c_int = 0 as libc::c_int;
        let mut ret: libc::c_short = 0;
        if *key.offset(idx as isize) as libc::c_int & 128 as libc::c_int != 0 {
            string = 1 as libc::c_int;
            let fresh33 = clen;
            clen = clen + 1;
            *str.offset(fresh33 as isize) = '"' as i32 as u_char;
        }
        ret = UTIL_Key_Extract(
            &mut *key.offset(idx as isize),
            &mut *str.offset(clen as isize),
            &mut count,
        );
        if (ret as libc::c_int) < 0 as libc::c_int {
            return ret;
        }
        if ret as libc::c_int == 0 as libc::c_int {
            string = 1 as libc::c_int;
            let fresh34 = clen;
            clen = clen + 1;
            *str.offset(fresh34 as isize) = '"' as i32 as u_char;
        }
        clen = clen + ret as libc::c_int;
        if string == 1 as libc::c_int {
            let fresh35 = clen;
            clen = clen + 1;
            *str.offset(fresh35 as isize) = '"' as i32 as u_char;
        }
        len = len - count;
        idx = idx + count;
        let fresh36 = clen;
        clen = clen + 1;
        *str.offset(fresh36 as isize) = ',' as i32 as u_char;
        max_subs -= 1;
        max_subs;
        if max_subs < 1 as libc::c_int {
            break;
        }
    }
    clen -= 1;
    clen;
    let fresh37 = clen;
    clen = clen + 1;
    *str.offset(fresh37 as isize) = ')' as i32 as u_char;
    *str.offset(clen as isize) = '\0' as i32 as u_char;
    return clen as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_Key_Last(mut var: *mut mvar) -> libc::c_int {
    let mut last: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*var).slen as libc::c_int {
        last = i;
        let fresh38 = i;
        i = i + 1;
        if ((*var).key[fresh38 as usize] as libc::c_int) < 64 as libc::c_int {
            loop {
                let fresh39 = i;
                i = i + 1;
                if !((*var).key[fresh39 as usize] as libc::c_int != 255 as libc::c_int
                    && i < (*var).slen as libc::c_int)
                {
                    break;
                }
            }
        } else {
            loop {
                let fresh40 = i;
                i = i + 1;
                if !((*var).key[fresh40 as usize] as libc::c_int != 0 as libc::c_int
                    && i < (*var).slen as libc::c_int)
                {
                    break;
                }
            }
        }
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_String_Mvar(
    mut var: *mut mvar,
    mut str: *mut u_char,
    mut max_subs: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0 as libc::c_int;
    let mut up: uci_tab = UCI_TAB {
        name: VAR_U { var_q: 0 },
        global: 0,
    };
    let mut vt: *mut var_u = 0 as *mut var_u;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    if (*var).uci as libc::c_int != 255 as libc::c_int {
        let fresh41 = p;
        p = p + 1;
        *str.offset(fresh41 as isize) = '^' as i32 as u_char;
        if (*var).uci as libc::c_int != 0 as libc::c_int {
            let mut vol: libc::c_int = (*var).volset as libc::c_int;
            let fresh42 = p;
            p = p + 1;
            *str.offset(fresh42 as isize) = '[' as i32 as u_char;
            let fresh43 = p;
            p = p + 1;
            *str.offset(fresh43 as isize) = '"' as i32 as u_char;
            if vol == 0 as libc::c_int {
                vol = (*partab.jobtab).vol as libc::c_int;
            }
            up = (*(*(*systab).vol[(vol - 1 as libc::c_int) as usize]).vollab)
                .uci[((*var).uci as libc::c_int - 1 as libc::c_int) as usize];
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                if up.name.var_cu[i as usize] as libc::c_int == '\0' as i32 {
                    break;
                }
                let fresh44 = p;
                p = p + 1;
                *str.offset(fresh44 as isize) = up.name.var_cu[i as usize];
                i += 1;
                i;
            }
            let fresh45 = p;
            p = p + 1;
            *str.offset(fresh45 as isize) = '"' as i32 as u_char;
            if (*var).volset as libc::c_int != 0 as libc::c_int {
                let fresh46 = p;
                p = p + 1;
                *str.offset(fresh46 as isize) = ',' as i32 as u_char;
                let fresh47 = p;
                p = p + 1;
                *str.offset(fresh47 as isize) = '"' as i32 as u_char;
                ptr = ((*(*(*systab)
                    .vol[((*var).volset as libc::c_int - 1 as libc::c_int) as usize])
                    .vollab)
                    .volnam
                    .var_cu)
                    .as_mut_ptr();
                i = 0 as libc::c_int;
                while i < 32 as libc::c_int {
                    if *ptr.offset(i as isize) as libc::c_int == '\0' as i32 {
                        break;
                    }
                    let fresh48 = p;
                    p = p + 1;
                    *str.offset(fresh48 as isize) = *ptr.offset(i as isize);
                    i += 1;
                    i;
                }
                let fresh49 = p;
                p = p + 1;
                *str.offset(fresh49 as isize) = '"' as i32 as u_char;
            }
            let fresh50 = p;
            p = p + 1;
            *str.offset(fresh50 as isize) = ']' as i32 as u_char;
        }
    }
    if (*var).uci as libc::c_int == 255 as libc::c_int
        && (*var).volset as libc::c_int != 0
    {
        let mut r: *mut rbd = (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .routine as *mut rbd;
        vt = (r as *mut u_char).offset((*r).var_tbl as libc::c_int as isize)
            as *mut var_u;
        let mut var_i: u_int = 0 as libc::c_int as u_int;
        while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            (*var)
                .name
                .var_qu[var_i
                as usize] = (*vt
                .offset(((*var).volset as libc::c_int - 1 as libc::c_int) as isize))
                .var_qu[var_i as usize];
            var_i = var_i.wrapping_add(1);
            var_i;
        }
        (*var).volset = 0 as libc::c_int as u_char;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*var).name.var_cu[i as usize] as libc::c_int == '\0' as i32 {
            break;
        }
        let fresh51 = p;
        p = p + 1;
        *str.offset(fresh51 as isize) = (*var).name.var_cu[i as usize];
        i += 1;
        i;
    }
    if (*var).slen as libc::c_int != 0 as libc::c_int && max_subs > 0 as libc::c_int {
        i = UTIL_String_Key(&mut (*var).slen, &mut *str.offset(p as isize), max_subs)
            as libc::c_int;
        if i < 0 as libc::c_int {
            return i as libc::c_short;
        }
        p = p + i;
    }
    *str.offset(p as isize) = '\0' as i32 as u_char;
    return p as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_MvarFromCStr(
    mut src: *mut cstring,
    mut var: *mut mvar,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut subs: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut kb: *mut cstring = 0 as *mut cstring;
    let mut nam: var_u = VAR_U { var_q: 0 };
    let mut vol: var_u = VAR_U { var_q: 0 };
    let mut tmp: [u_char; 260] = [0; 260];
    kb = tmp.as_mut_ptr() as *mut cstring;
    (*var).volset = 0 as libc::c_int as u_char;
    (*var).uci = 255 as libc::c_int as u_char;
    (*var).slen = 0 as libc::c_int as u_char;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        (*var).name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    ptr = ((*src).buf).as_mut_ptr();
    if *ptr as libc::c_int == '^' as i32 {
        let mut v: libc::c_int = 0;
        ptr = ptr.offset(1);
        ptr;
        (*var).uci = 0 as libc::c_int as u_char;
        v = (*ptr as libc::c_int == '|' as i32) as libc::c_int;
        if v != 0 || *ptr as libc::c_int == '[' as i32 {
            ptr = ptr.offset(1);
            ptr;
            let mut var_i_0: u_int = 0 as libc::c_int as u_int;
            while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                nam.var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
                var_i_0 = var_i_0.wrapping_add(1);
                var_i_0;
            }
            let fresh52 = ptr;
            ptr = ptr.offset(1);
            if *fresh52 as libc::c_int != '"' as i32 {
                return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
            i = 0 as libc::c_int;
            while *ptr as libc::c_int != '"' as i32 {
                if i == 32 as libc::c_int {
                    return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
                }
                let fresh53 = ptr;
                ptr = ptr.offset(1);
                let fresh54 = i;
                i = i + 1;
                nam.var_cu[fresh54 as usize] = *fresh53;
            }
            ptr = ptr.offset(1);
            ptr;
            if v == 0 && *ptr as libc::c_int == ',' as i32 {
                ptr = ptr.offset(1);
                ptr;
                let mut var_i_1: u_int = 0 as libc::c_int as u_int;
                while var_i_1 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    vol.var_qu[var_i_1 as usize] = 0 as libc::c_int as u_int64;
                    var_i_1 = var_i_1.wrapping_add(1);
                    var_i_1;
                }
                let fresh55 = ptr;
                ptr = ptr.offset(1);
                if *fresh55 as libc::c_int != '"' as i32 {
                    return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
                }
                i = 0 as libc::c_int;
                while *ptr as libc::c_int != '"' as i32 {
                    if i == 32 as libc::c_int {
                        return -(12 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                    }
                    let fresh56 = ptr;
                    ptr = ptr.offset(1);
                    let fresh57 = i;
                    i = i + 1;
                    vol.var_cu[fresh57 as usize] = *fresh56;
                }
                ptr = ptr.offset(1);
                ptr;
                i = 0 as libc::c_int;
                while i < 1 as libc::c_int {
                    if !((*systab).vol[i as usize]).is_null() {
                        if var_equal((*(*(*systab).vol[i as usize]).vollab).volnam, vol)
                            != 0
                        {
                            break;
                        }
                    }
                    i += 1;
                    i;
                }
                if i == 1 as libc::c_int {
                    return -(26 as libc::c_int) as libc::c_short;
                }
                (*var).volset = (i + 1 as libc::c_int) as u_char;
            }
            if (*var).volset as libc::c_int == 0 as libc::c_int {
                (*var).volset = (*partab.jobtab).vol;
            }
            i = 0 as libc::c_int;
            while i < 64 as libc::c_int {
                if var_equal(
                    (*(*(*systab)
                        .vol[((*var).volset as libc::c_int - 1 as libc::c_int) as usize])
                        .vollab)
                        .uci[i as usize]
                        .name,
                    nam,
                ) != 0
                {
                    break;
                }
                i += 1;
                i;
            }
            if i == 64 as libc::c_int {
                return -(26 as libc::c_int) as libc::c_short;
            }
            (*var).uci = (i + 1 as libc::c_int) as u_char;
            if v != 0 && *ptr as libc::c_int != '|' as i32
                || v == 0 && *ptr as libc::c_int != ']' as i32
            {
                return -(26 as libc::c_int) as libc::c_short;
            }
            ptr = ptr.offset(1);
            ptr;
        }
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if *ptr as libc::c_int == '(' as i32 || *ptr as libc::c_int == '\0' as i32 {
            break;
        }
        let fresh58 = ptr;
        ptr = ptr.offset(1);
        (*var).name.var_cu[i as usize] = *fresh58;
        i += 1;
        i;
    }
    if *ptr as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int as libc::c_short;
    }
    let fresh59 = ptr;
    ptr = ptr.offset(1);
    if *fresh59 as libc::c_int != '(' as i32 {
        return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    loop {
        let mut q: libc::c_int = 0;
        let mut s: libc::c_short = 0;
        if *ptr as libc::c_int == '\0' as i32 {
            return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        q = (*ptr as libc::c_int == '"' as i32) as libc::c_int;
        if q != 0 {
            ptr = ptr.offset(1);
            ptr;
        }
        i = 0 as libc::c_int;
        loop {
            if *ptr as libc::c_int == '\0' as i32 {
                return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
            if *ptr as libc::c_int == '"' as i32 && q != 0 {
                ptr = ptr.offset(1);
                ptr;
                if *ptr as libc::c_int != '"' as i32 {
                    if *ptr as libc::c_int == ',' as i32 {
                        break;
                    }
                    if *ptr as libc::c_int == ')' as i32 {
                        break;
                    }
                    return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
                }
            }
            if i == 255 as libc::c_int {
                return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
            if q == 0
                && (*ptr as libc::c_int == ',' as i32
                    || *ptr as libc::c_int == ')' as i32)
            {
                break;
            }
            let fresh60 = ptr;
            ptr = ptr.offset(1);
            let fresh61 = i;
            i = i + 1;
            (*kb).buf[fresh61 as usize] = *fresh60;
        }
        (*kb).buf[i as usize] = '\0' as i32 as u_char;
        (*kb).len = i as u_short;
        s = UTIL_Key_Build(
            kb,
            &mut *((*var).key).as_mut_ptr().offset((*var).slen as isize),
        );
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        if s as libc::c_int + (*var).slen as libc::c_int > 255 as libc::c_int {
            return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        if (*var).key[(*var).slen as usize] as libc::c_int == 128 as libc::c_int
            && q == 0
        {
            return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        subs += 1;
        subs;
        (*var).slen = (s as libc::c_int + (*var).slen as libc::c_int) as u_char;
        if *ptr as libc::c_int == ',' as i32 {
            ptr = ptr.offset(1);
            ptr;
        } else if *ptr as libc::c_int == ')' as i32 {
            ptr = ptr.offset(1);
            ptr;
            break;
        } else {
            return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short
        }
    }
    if *ptr as libc::c_int != '\0' as i32 {
        return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    return subs as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_Key_KeyCmp(
    mut key1: *mut u_char,
    mut key2: *mut u_char,
    mut kleng1: libc::c_int,
    mut kleng2: libc::c_int,
) -> libc::c_int {
    let mut cmpvar: libc::c_int = 0;
    cmpvar = memcmp(
        key1 as *const libc::c_void,
        key2 as *const libc::c_void,
        (if kleng1 < kleng2 { kleng1 } else { kleng2 }) as libc::c_ulong,
    );
    if cmpvar == 0 {
        if kleng1 == kleng2 {
            return 0 as libc::c_int;
        }
        if kleng1 > kleng2 {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if cmpvar > 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_Key_Chars_In_Subs(
    mut Key: *mut libc::c_char,
    mut keylen: libc::c_int,
    mut maxsubs: libc::c_int,
    mut subs: *mut libc::c_int,
    mut KeyBuffer: *mut libc::c_char,
) -> libc::c_int {
    let mut cnt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    cnt = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < keylen && cnt < maxsubs {
        if *Key.offset(i as isize) as libc::c_int & 128 as libc::c_int != 0
            || *Key.offset(i as isize) as libc::c_int & 64 as libc::c_int != 0
        {
            i += 1;
            i;
            while *Key.offset(i as isize) != 0 {
                i += 1;
                i;
            }
            i += 1;
            i;
        } else {
            i += 1;
            i;
            while *Key.offset(i as isize) as libc::c_int != -(1 as libc::c_int) {
                i += 1;
                i;
            }
            i += 1;
            i;
        }
        cnt += 1;
        cnt;
    }
    if !subs.is_null() {
        *subs = cnt;
    }
    if !KeyBuffer.is_null() {
        memcpy(
            KeyBuffer as *mut libc::c_void,
            Key as *const libc::c_void,
            i as libc::c_ulong,
        );
    }
    return i;
}
