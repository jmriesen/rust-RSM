#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type GBD;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn DB_ViewRel(vol: u_int, ptr: *mut GBD);
    fn SQ_Close(chan: libc::c_int) -> libc::c_short;
    fn ST_Kill(var: *mut mvar) -> libc::c_short;
    fn ST_KillAll(count: libc::c_int, keep: *mut var_u) -> libc::c_short;
    fn ST_SymDet(count: libc::c_int, list: *mut libc::c_short);
    fn Routine_Detach(pointer: *mut RBD);
    fn LCK_Remove(job: libc::c_int);
    fn ST_Restore(newtab: *mut ST_newtab);
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
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
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            __darwin_size_t,
            *mut *const libc::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option::<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut libc::c_char,
            __darwin_size_t,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
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
pub type ST_newtab = ST_NEWTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_NEWTAB {
    pub fwd_link: *mut ST_NEWTAB,
    pub count_enn: libc::c_short,
    pub stindex: *mut libc::c_short,
    pub count_new: libc::c_short,
    pub locdata: *mut ST_locdata,
}
pub type ST_locdata = ST_LOCDATA;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_LOCDATA {
    pub stindex: libc::c_short,
    pub data: *mut ST_data,
}
pub type ST_data = ST_DATA;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_DATA {
    pub deplnk: *mut ST_depend,
    pub attach: libc::c_short,
    pub dbc: u_short,
    pub data: [u_char; 65535],
}
pub type ST_depend = ST_DEPEND;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_DEPEND {
    pub deplnk: *mut ST_DEPEND,
    pub keylen: u_char,
    pub bytes: [u_char; 65794],
}
#[inline]
unsafe extern "C" fn __isctype(
    mut _c: __darwin_ct_rune_t,
    mut _f: libc::c_ulong,
) -> __darwin_ct_rune_t {
    return if _c < 0 as libc::c_int || _c >= (1 as libc::c_int) << 8 as libc::c_int {
        0 as libc::c_int
    } else {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0)
            as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x400 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn mcopy(
    mut src: *mut u_char,
    mut dst: *mut u_char,
    mut bytes: libc::c_int,
) -> libc::c_int {
    if dst >= partab.strstk_start && dst < partab.strstk_last
        && &mut *dst.offset(bytes as isize) as *mut u_char > partab.strstk_last
    {
        return -(8 as libc::c_int + 200 as libc::c_int);
    }
    if bytes > 65534 as libc::c_int {
        return -(75 as libc::c_int);
    }
    memmove(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        bytes as libc::c_ulong,
    );
    *dst.offset(bytes as isize) = '\0' as i32 as u_char;
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn ncopy(
    mut src: *mut *mut u_char,
    mut dst: *mut u_char,
) -> libc::c_short {
    let mut current_block: u64;
    let mut c: u_char = 0;
    let mut p: *mut u_char = 0 as *mut u_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut dp: libc::c_int = 0 as libc::c_int;
    let mut minus: libc::c_int = 0 as libc::c_int;
    let mut exp: libc::c_int = 0 as libc::c_int;
    let mut expsgn: libc::c_int = 1 as libc::c_int;
    if dst >= partab.strstk_start && dst < partab.strstk_last
        && &mut *dst.offset(256 as libc::c_int as isize) as *mut u_char
            > partab.strstk_last
    {
        return -(8 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    p = *src;
    loop {
        if i > 256 as libc::c_int {
            return -(92 as libc::c_int) as libc::c_short;
        }
        let fresh0 = p;
        p = p.offset(1);
        c = *fresh0;
        if i == 0 as libc::c_int && k == 0 as libc::c_int {
            if c as libc::c_int == '+' as i32 {
                continue;
            }
            if c as libc::c_int == '-' as i32 {
                minus = (minus == 0) as libc::c_int;
                continue;
            } else if minus != 0 {
                let fresh1 = i;
                i = i + 1;
                *dst.offset(fresh1 as isize) = '-' as i32 as u_char;
            }
        }
        if i == minus && c as libc::c_int == '0' as i32 {
            k = 1 as libc::c_int;
        } else if c as libc::c_int == '.' as i32 {
            if dp != 0 as libc::c_int {
                break;
            }
            dp = 1 as libc::c_int;
            let fresh2 = i;
            i = i + 1;
            *dst.offset(fresh2 as isize) = c;
        } else if (*systab).historic & 1 as libc::c_int != 0
            && c as libc::c_int == 'E' as i32
        {
            let fresh3 = p;
            p = p.offset(1);
            c = *fresh3;
            if c as libc::c_int == '-' as i32 {
                expsgn = -(1 as libc::c_int);
            } else if isdigit(c as libc::c_int) != 0 {
                exp = c as libc::c_int - '0' as i32;
            } else if c as libc::c_int != '+' as i32 {
                break;
            }
            loop {
                let fresh4 = p;
                p = p.offset(1);
                c = *fresh4;
                if isdigit(c as libc::c_int) == 0 as libc::c_int {
                    break;
                }
                exp = exp * 10 as libc::c_int + (c as libc::c_int - '0' as i32);
            }
            break;
        } else {
            if isdigit(c as libc::c_int) == 0 as libc::c_int {
                break;
            }
            let fresh5 = i;
            i = i + 1;
            *dst.offset(fresh5 as isize) = c;
        }
    }
    if dp != 0 {
        k = 0 as libc::c_int;
        while *dst.offset((i - k - 1 as libc::c_int) as isize) as libc::c_int
            == '0' as i32
        {
            k += 1;
            k;
        }
        i -= k;
        if *dst.offset((i - 1 as libc::c_int) as isize) as libc::c_int == '.' as i32 {
            i -= 1;
            i;
        }
    }
    if i != 0
        && *dst.offset((i - 1 as libc::c_int) as isize) as libc::c_int == '-' as i32
    {
        i -= 1;
        i;
    }
    if i == 0 as libc::c_int {
        let fresh6 = i;
        i = i + 1;
        *dst.offset(fresh6 as isize) = '0' as i32 as u_char;
    }
    *dst.offset(i as isize) = '\0' as i32 as u_char;
    p = p.offset(-1);
    p;
    *src = p;
    if exp == 0 {
        return i as libc::c_short;
    }
    *dst.offset(i as isize) = '0' as i32 as u_char;
    dp = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < i {
        if *dst.offset(k as isize) as libc::c_int == '.' as i32 {
            dp = 1 as libc::c_int;
            break;
        } else {
            k += 1;
            k;
        }
    }
    if expsgn > 0 as libc::c_int {
        if dp != 0 {
            loop {
                if !(k < i) {
                    current_block = 6545907279487748450;
                    break;
                }
                *dst.offset(k as isize) = *dst.offset((k + 1 as libc::c_int) as isize);
                *dst.offset((k + 1 as libc::c_int) as isize) = '.' as i32 as u_char;
                exp -= 1;
                exp;
                if exp == 0 {
                    current_block = 13282675481861905020;
                    break;
                }
                k += 1;
                k;
            }
        } else {
            current_block = 6545907279487748450;
        }
        match current_block {
            13282675481861905020 => {}
            _ => {
                if exp + i > 256 as libc::c_int {
                    return -(92 as libc::c_int) as libc::c_short;
                }
                while exp != 0 {
                    let fresh7 = i;
                    i = i + 1;
                    *dst.offset(fresh7 as isize) = '0' as i32 as u_char;
                    exp -= 1;
                    exp;
                }
                *dst.offset(i as isize) = '\0' as i32 as u_char;
            }
        }
    } else {
        if dp == 0 {
            k = i;
            i += 1;
            i;
        }
        if k > 0 as libc::c_int {
            dp = k;
            loop {
                if !(dp > minus) {
                    current_block = 11739054925370445424;
                    break;
                }
                *dst.offset(dp as isize) = *dst.offset((dp - 1 as libc::c_int) as isize);
                *dst.offset((dp - 1 as libc::c_int) as isize) = '.' as i32 as u_char;
                exp -= 1;
                exp;
                if exp == 0 {
                    current_block = 13282675481861905020;
                    break;
                }
                dp -= 1;
                dp;
            }
        } else {
            current_block = 11739054925370445424;
        }
        match current_block {
            13282675481861905020 => {}
            _ => {
                if exp + i > 256 as libc::c_int {
                    return -(92 as libc::c_int) as libc::c_short;
                }
                memmove(
                    &mut *dst.offset((minus + exp + 1 as libc::c_int) as isize)
                        as *mut u_char as *mut libc::c_void,
                    &mut *dst.offset((minus + 1 as libc::c_int) as isize) as *mut u_char
                        as *const libc::c_void,
                    i as libc::c_ulong,
                );
                k = minus + 1 as libc::c_int;
                while k <= minus + exp {
                    let fresh8 = k;
                    k = k + 1;
                    *dst.offset(fresh8 as isize) = '0' as i32 as u_char;
                }
                i += exp;
            }
        }
    }
    dp = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < i {
        if *dst.offset(k as isize) as libc::c_int == '.' as i32 {
            dp = 1 as libc::c_int;
            break;
        } else {
            k += 1;
            k;
        }
    }
    if dp != 0 {
        while *dst.offset((i - 1 as libc::c_int) as isize) as libc::c_int == '0' as i32 {
            i -= 1;
            i;
        }
        if *dst.offset((i - 1 as libc::c_int) as isize) as libc::c_int == '.' as i32 {
            i -= 1;
            i;
        }
    }
    if i == 0 {
        let fresh9 = i;
        i = i + 1;
        *dst.offset(fresh9 as isize) = '0' as i32 as u_char;
    }
    *dst.offset(i as isize) = '\0' as i32 as u_char;
    dp = (*dst.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
        as libc::c_int;
    if *dst.offset(dp as isize) as libc::c_int == '0' as i32 {
        k = dp;
        while k < i && *dst.offset(k as isize) as libc::c_int == '0' as i32 {
            k += 1;
            k;
        }
        memmove(
            &mut *dst.offset(dp as isize) as *mut u_char as *mut libc::c_void,
            &mut *dst.offset(k as isize) as *mut u_char as *const libc::c_void,
            (i - k) as libc::c_ulong,
        );
        i -= k - dp;
        if i == dp {
            if dp != 0 {
                i -= 1;
                i;
            }
            let fresh10 = i;
            i = i + 1;
            *dst.offset(fresh10 as isize) = '0' as i32 as u_char;
        }
        *dst.offset(i as isize) = '\0' as i32 as u_char;
    }
    return i as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn CleanJob(mut job: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    j = job - 1 as libc::c_int;
    if job == 0 {
        j = (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long as libc::c_int;
    }
    LCK_Remove(j + 1 as libc::c_int);
    i = (*((*systab).jobtab).offset(j as isize)).cur_do;
    while i != 0 {
        if job == 0 {
            if !((*((*systab).jobtab).offset(j as isize)).dostk[i as usize].newtab)
                .is_null()
            {
                ST_Restore(
                    (*((*systab).jobtab).offset(j as isize)).dostk[i as usize].newtab
                        as *mut ST_newtab,
                );
            }
            if (*((*systab).jobtab).offset(j as isize)).dostk[i as usize].flags
                as libc::c_int & 2 as libc::c_int != 0
                && !((*((*systab).jobtab).offset(j as isize)).dostk[i as usize].symbol)
                    .is_null()
            {
                ST_SymDet(
                    (*((*((*systab).jobtab).offset(j as isize)).dostk[i as usize].routine
                        as *mut rbd))
                        .num_vars as libc::c_int,
                    (*((*systab).jobtab).offset(j as isize)).dostk[i as usize].symbol,
                );
            }
        }
        if (*((*systab).jobtab).offset(j as isize)).dostk[i as usize].flags
            as libc::c_int & 2 as libc::c_int != 0
        {
            Routine_Detach(
                (*((*systab).jobtab).offset(j as isize)).dostk[i as usize].routine
                    as *mut rbd,
            );
        }
        i -= 1;
        i;
    }
    if job == 0 {
        ST_KillAll(0 as libc::c_int, 0 as *mut var_u);
        partab.src_var.volset = 0 as libc::c_int as u_char;
        partab.src_var.slen = 0 as libc::c_int as u_char;
        partab.src_var.uci = 255 as libc::c_int as u_char;
        let mut var_i: u_int = 0 as libc::c_int as u_int;
        while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            partab.src_var.name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
            var_i = var_i.wrapping_add(1);
            var_i;
        }
        memcpy(
            (partab.src_var.name.var_cu).as_mut_ptr() as *mut libc::c_void,
            b"$ETRAP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        );
        ST_Kill(&mut partab.src_var);
        let mut var_i_0: u_int = 0 as libc::c_int as u_int;
        while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            partab.src_var.name.var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
            var_i_0 = var_i_0.wrapping_add(1);
            var_i_0;
        }
        memcpy(
            (partab.src_var.name.var_cu).as_mut_ptr() as *mut libc::c_void,
            b"$ECODE\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        );
        ST_Kill(&mut partab.src_var);
    }
    i = 0 as libc::c_int;
    while i < 1 as libc::c_int {
        if !(*((*((*systab).jobtab).offset(j as isize)).view)
            .as_mut_ptr()
            .offset(i as isize))
            .is_null()
        {
            DB_ViewRel(
                (i + 1 as libc::c_int) as u_int,
                *((*((*systab).jobtab).offset(j as isize)).view)
                    .as_mut_ptr()
                    .offset(i as isize),
            );
            let ref mut fresh11 = *((*((*systab).jobtab).offset(j as isize)).view)
                .as_mut_ptr()
                .offset(i as isize);
            *fresh11 = 0 as *mut GBD;
        }
        i += 1;
        i;
    }
    (*((*systab).jobtab).offset(j as isize)).cur_do = 0 as libc::c_int;
    if job == 0 {
        i = 1 as libc::c_int;
        while i < 64 as libc::c_int {
            let fresh12 = i;
            i = i + 1;
            SQ_Close(fresh12);
        }
        partab.jobtab = 0 as *mut jobtab;
    }
    memset(
        &mut *((*systab).jobtab).offset(j as isize) as *mut jobtab as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<jobtab>() as libc::c_ulong,
    );
}
