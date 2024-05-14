#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type GBD;
    fn atoi(_: *const libc::c_char) -> libc::c_int;
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
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn DB_Set(var: *mut mvar, data: *mut cstring) -> libc::c_int;
    fn DB_Kill(var: *mut mvar) -> libc::c_short;
    fn eval();
    fn parse();
    fn cstringtoi(str: *mut cstring) -> libc::c_int;
    fn itocstring(buf: *mut u_char, n: libc::c_int) -> u_short;
    fn Dget1(ret_buffer: *mut u_char, var: *mut mvar) -> libc::c_int;
    fn Dorder1(ret_buffer: *mut u_char, var: *mut mvar) -> libc::c_short;
    fn Vhorolog(ret_buffer: *mut u_char) -> libc::c_short;
    fn SS_Norm(var: *mut mvar) -> libc::c_short;
    fn UTIL_Key_Build(src: *mut cstring, dest: *mut u_char) -> libc::c_short;
    fn UTIL_Key_Extract(
        key: *mut u_char,
        str: *mut u_char,
        cnt: *mut libc::c_int,
    ) -> libc::c_short;
    fn Routine_Delete(routine_0: var_u, uci: libc::c_int);
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
    static mut source_ptr: *mut u_char;
    static mut comp_ptr: *mut u_char;
    fn atom();
    fn comperror(err: libc::c_short);
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
pub type tags = TAGS;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct TAGS {
    pub name: var_u,
    pub code: u_short,
}
#[inline]
unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
    return (_c & !(0x7f as libc::c_int) == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __istype(
    mut _c: __darwin_ct_rune_t,
    mut _f: libc::c_ulong,
) -> libc::c_int {
    return if isascii(_c) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0)
            as libc::c_int
    } else {
        (__maskrune(_c, _f) != 0) as libc::c_int
    };
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
pub unsafe extern "C" fn isalnum(mut _c: libc::c_int) -> libc::c_int {
    return __istype(
        _c,
        (0x100 as libc::c_long | 0x400 as libc::c_long) as libc::c_ulong,
    );
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isalpha(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x100 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x400 as libc::c_long as libc::c_ulong);
}
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
#[inline]
unsafe extern "C" fn var_empty(mut var: var_u) -> u_int {
    if var.var_q == 0 as libc::c_int as u_int64 {
        return 1 as libc::c_int as u_int
    } else {
        return 0 as libc::c_int as u_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn routine(mut runtime: libc::c_int) -> libc::c_short {
    let mut c: libc::c_char = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut us: u_short = 0;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut j: libc::c_int = 0;
    let mut dp: libc::c_int = 0 as libc::c_int;
    let mut tag: var_u = VAR_U { var_q: 0 };
    let mut rou: var_u = VAR_U { var_q: 0 };
    let mut ntag: libc::c_int = 0;
    let mut isinder: libc::c_int = 0 as libc::c_int;
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut gotplus: libc::c_int = 0 as libc::c_int;
    let mut p1indirect: libc::c_int = 0 as libc::c_int;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        tag.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    let mut var_i_0: u_int = 0 as libc::c_int as u_int;
    while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        rou.var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
        var_i_0 = var_i_0.wrapping_add(1);
        var_i_0;
    }
    let fresh0 = source_ptr;
    source_ptr = source_ptr.offset(1);
    c = *fresh0 as libc::c_char;
    if c as libc::c_int == '@' as i32 {
        isinder = 1 as libc::c_int;
        if runtime == -(2 as libc::c_int) {
            ptr = comp_ptr;
        }
        atom();
        if runtime == -(2 as libc::c_int) && *ptr as libc::c_int == 60 as libc::c_int
            && *(ptr.offset(1 as libc::c_int as isize) as *mut u_short) as libc::c_int
                == 0 as libc::c_int
        {
            return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        p1indirect = 1 as libc::c_int;
        let fresh1 = source_ptr;
        source_ptr = source_ptr.offset(1);
        c = *fresh1 as libc::c_char;
        if c as libc::c_int == ')' as i32 || c as libc::c_int == ',' as i32
            || c as libc::c_int == ' ' as i32 || c as libc::c_int == '\0' as i32
        {
            source_ptr = source_ptr.offset(-1);
            source_ptr;
            return 0 as libc::c_int as libc::c_short;
        }
    } else {
        ntag = isdigit(c as libc::c_int);
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if ntag != 0 && isdigit(c as libc::c_int) == 0 {
                break;
            }
            if i != 0 as libc::c_int && c as libc::c_int == '%' as i32 {
                break;
            }
            if isalnum(c as libc::c_int) == 0 && c as libc::c_int != '%' as i32 {
                break;
            }
            tag.var_cu[i as usize] = c as u_char;
            let fresh2 = source_ptr;
            source_ptr = source_ptr.offset(1);
            c = *fresh2 as libc::c_char;
            i += 1;
            i;
        }
        if isalnum(c as libc::c_int) != 0 {
            comperror(-(56 as libc::c_int) as libc::c_short);
        }
    }
    if runtime == -(2 as libc::c_int) {
        if isinder == 0 && var_empty(tag) == 0 {
            let fresh3 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh3 = 60 as libc::c_int as u_char;
            us = i as u_short;
            memcpy(
                comp_ptr as *mut libc::c_void,
                &mut us as *mut u_short as *const libc::c_void,
                ::core::mem::size_of::<u_short>() as libc::c_ulong,
            );
            comp_ptr = comp_ptr
                .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
            j = 0 as libc::c_int;
            while j < i {
                let fresh4 = j;
                j = j + 1;
                let fresh5 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh5 = tag.var_cu[fresh4 as usize];
            }
            let fresh6 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh6 = '\0' as i32 as u_char;
        }
        runtime = 0 as libc::c_int;
        isinder = 1 as libc::c_int;
    }
    if c as libc::c_int == ')' as i32 || c as libc::c_int == ',' as i32
        || c as libc::c_int == ' ' as i32 || c as libc::c_int == '\0' as i32
    {
        source_ptr = source_ptr.offset(-1);
        source_ptr;
    } else {
        if c as libc::c_int == '+' as i32 && runtime != -(1 as libc::c_int) {
            gotplus = 1 as libc::c_int;
            if runtime == 0 {
                if isinder == 0 && var_empty(tag) == 0 {
                    let fresh7 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh7 = 60 as libc::c_int as u_char;
                    us = i as u_short;
                    memcpy(
                        comp_ptr as *mut libc::c_void,
                        &mut us as *mut u_short as *const libc::c_void,
                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                    );
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                        );
                    j = 0 as libc::c_int;
                    while j < i {
                        let fresh8 = j;
                        j = j + 1;
                        let fresh9 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh9 = tag.var_cu[fresh8 as usize];
                    }
                    let fresh10 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh10 = '\0' as i32 as u_char;
                }
                isinder = 1 as libc::c_int;
                let fresh11 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh11 = 60 as libc::c_int as u_char;
                us = 1 as libc::c_int as u_short;
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    &mut us as *mut u_short as *const libc::c_void,
                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                );
                comp_ptr = comp_ptr
                    .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
                let fresh12 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh12 = '+' as i32 as u_char;
                let fresh13 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh13 = '\0' as i32 as u_char;
                if var_empty(tag) == 0 || p1indirect != 0 {
                    let fresh14 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh14 = 17 as libc::c_int as u_char;
                }
                eval();
                let fresh15 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh15 = 18 as libc::c_int as u_char;
                let fresh16 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh16 = 17 as libc::c_int as u_char;
            } else {
                if isdigit(*source_ptr as libc::c_int) == 0 {
                    comperror(-(5 as libc::c_int) as libc::c_short);
                    return -(1 as libc::c_int) as libc::c_short;
                }
                loop {
                    if *source_ptr as libc::c_int == '.' as i32 {
                        if dp != 0 {
                            break;
                        }
                        dp = 1 as libc::c_int;
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                    } else {
                        if isdigit(*source_ptr as libc::c_int) == 0 {
                            break;
                        }
                        if dp == 0 {
                            let fresh17 = source_ptr;
                            source_ptr = source_ptr.offset(1);
                            offset = offset * 10 as libc::c_int + *fresh17 as libc::c_int
                                - '0' as i32;
                        } else {
                            source_ptr = source_ptr.offset(1);
                            source_ptr;
                        }
                    }
                }
                if runtime == 1 as libc::c_int {
                    if (*systab).historic & 2 as libc::c_int == 0 {
                        comperror(
                            -(70 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return -(1 as libc::c_int) as libc::c_short;
                    }
                }
            }
            let fresh18 = source_ptr;
            source_ptr = source_ptr.offset(1);
            c = *fresh18 as libc::c_char;
        }
        if c as libc::c_int == ')' as i32 || c as libc::c_int == ',' as i32
            || c as libc::c_int == ' ' as i32 || c as libc::c_int == '\0' as i32
        {
            source_ptr = source_ptr.offset(-1);
            source_ptr;
        } else if c as libc::c_int == '^' as i32 {
            let fresh19 = source_ptr;
            source_ptr = source_ptr.offset(1);
            c = *fresh19 as libc::c_char;
            if c as libc::c_int != '@' as i32 {
                i = 0 as libc::c_int;
                while i < 32 as libc::c_int {
                    if i == 0 as libc::c_int
                        && isdigit(c as libc::c_int) != 0 as libc::c_int
                    {
                        break;
                    }
                    if i != 0 as libc::c_int && c as libc::c_int == '%' as i32 {
                        break;
                    }
                    if isalnum(c as libc::c_int) == 0 as libc::c_int
                        && c as libc::c_int != '%' as i32
                    {
                        break;
                    }
                    rou.var_cu[i as usize] = c as u_char;
                    let fresh20 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh20 as libc::c_char;
                    i += 1;
                    i;
                }
                if isalnum(c as libc::c_int) != 0 as libc::c_int {
                    comperror(-(56 as libc::c_int) as libc::c_short);
                }
                if isinder != 0 {
                    let fresh21 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh21 = 60 as libc::c_int as u_char;
                    us = (i + 1 as libc::c_int) as u_short;
                    memcpy(
                        comp_ptr as *mut libc::c_void,
                        &mut us as *mut u_short as *const libc::c_void,
                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                    );
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                        );
                    let fresh22 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh22 = '^' as i32 as u_char;
                    j = 0 as libc::c_int;
                    while j < i {
                        let fresh23 = j;
                        j = j + 1;
                        let fresh24 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh24 = rou.var_cu[fresh23 as usize];
                    }
                    let fresh25 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh25 = '\0' as i32 as u_char;
                    if var_empty(tag) == 0 || gotplus != 0 || p1indirect != 0 {
                        let fresh26 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh26 = 17 as libc::c_int as u_char;
                    }
                }
                source_ptr = source_ptr.offset(-1);
                source_ptr;
            } else {
                if isinder == 0 && var_empty(tag) == 0 {
                    let fresh27 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh27 = 60 as libc::c_int as u_char;
                    us = i as u_short;
                    memcpy(
                        comp_ptr as *mut libc::c_void,
                        &mut us as *mut u_short as *const libc::c_void,
                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                    );
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                        );
                    j = 0 as libc::c_int;
                    while j < i {
                        let fresh28 = j;
                        j = j + 1;
                        let fresh29 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh29 = tag.var_cu[fresh28 as usize];
                    }
                    let fresh30 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh30 = '\0' as i32 as u_char;
                }
                isinder = 1 as libc::c_int;
                if offset != 0 {
                    let fresh31 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh31 = 60 as libc::c_int as u_char;
                    us = 1 as libc::c_int as u_short;
                    memcpy(
                        comp_ptr as *mut libc::c_void,
                        &mut us as *mut u_short as *const libc::c_void,
                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                    );
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                        );
                    let fresh32 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh32 = '+' as i32 as u_char;
                    let fresh33 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh33 = '\0' as i32 as u_char;
                    if var_empty(tag) == 0 {
                        let fresh34 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh34 = 17 as libc::c_int as u_char;
                    }
                    eval();
                    let fresh35 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh35 = 18 as libc::c_int as u_char;
                    let fresh36 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh36 = 17 as libc::c_int as u_char;
                }
                let fresh37 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh37 = 60 as libc::c_int as u_char;
                us = 1 as libc::c_int as u_short;
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    &mut us as *mut u_short as *const libc::c_void,
                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                );
                comp_ptr = comp_ptr
                    .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
                let fresh38 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh38 = '^' as i32 as u_char;
                let fresh39 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh39 = '\0' as i32 as u_char;
                if var_empty(tag) == 0 || gotplus != 0 || p1indirect != 0 {
                    let fresh40 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh40 = 17 as libc::c_int as u_char;
                }
                atom();
                let fresh41 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh41 = 17 as libc::c_int as u_char;
            }
        } else {
            source_ptr = source_ptr.offset(-1);
            source_ptr;
        }
    }
    if isinder != 0 {
        return 0 as libc::c_int as libc::c_short;
    }
    if var_empty(tag) == 0 || offset != 0 {
        ntag = 1 as libc::c_int;
        if var_empty(rou) == 0 || offset != 0 {
            ntag = 2 as libc::c_int;
            memcpy(
                comp_ptr as *mut libc::c_void,
                &mut rou.var_qu as *mut [u_int64; 4] as *const libc::c_void,
                32 as libc::c_int as libc::c_ulong,
            );
            comp_ptr = comp_ptr.offset(32 as libc::c_int as isize);
        }
        memcpy(
            comp_ptr as *mut libc::c_void,
            &mut tag.var_qu as *mut [u_int64; 4] as *const libc::c_void,
            32 as libc::c_int as libc::c_ulong,
        );
        comp_ptr = comp_ptr.offset(32 as libc::c_int as isize);
        if offset != 0 {
            if offset > 65534 as libc::c_int - 1 as libc::c_int {
                comperror(-(70 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return -(4 as libc::c_int) as libc::c_short;
            }
            us = offset as u_short;
            memcpy(
                comp_ptr as *mut libc::c_void,
                &mut us as *mut u_short as *const libc::c_void,
                ::core::mem::size_of::<u_short>() as libc::c_ulong,
            );
            comp_ptr = comp_ptr
                .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
            return -(4 as libc::c_int) as libc::c_short;
        }
        return -ntag as libc::c_short;
    }
    memcpy(
        comp_ptr as *mut libc::c_void,
        &mut rou.var_qu as *mut [u_int64; 4] as *const libc::c_void,
        32 as libc::c_int as libc::c_ulong,
    );
    comp_ptr = comp_ptr.offset(32 as libc::c_int as isize);
    return -(3 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Compile_Routine(
    mut rou: *mut mvar,
    mut src: *mut mvar,
    mut stack: *mut u_char,
) -> libc::c_int {
    let mut line: *mut cstring = 0 as *mut cstring;
    let mut code: *mut u_char = 0 as *mut u_char;
    let mut s: libc::c_int = 0;
    let mut ss: libc::c_short = 0;
    let mut us: u_short = 0;
    let mut uss: u_short = 0;
    let mut cnt: libc::c_int = 0;
    let mut src_slen: u_char = 0;
    let mut rou_slen: u_char = 0 as libc::c_int as u_char;
    let mut temp: [u_char; 100] = [0; 100];
    let mut tag_tbl: [tags; 256] = [TAGS {
        name: VAR_U { var_q: 0 },
        code: 0,
    }; 256];
    let mut var_tbl: [var_u; 256] = [VAR_U { var_q: 0 }; 256];
    let mut var: var_u = VAR_U { var_q: 0 };
    let mut num_tags: libc::c_int = 0 as libc::c_int;
    let mut num_vars: libc::c_int = 0 as libc::c_int;
    let mut lino: libc::c_int = 0 as libc::c_int;
    let mut last_dots: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cptr: *mut cstring = 0 as *mut cstring;
    let mut p: *mut u_char = 0 as *mut u_char;
    let mut rounam: var_u = VAR_U { var_q: 0 };
    let mut same: libc::c_int = 0 as libc::c_int;
    partab.checkonly = 0 as libc::c_int;
    partab.errors = 0 as libc::c_int as u_int;
    partab.ln = &mut lino;
    line = stack as *mut cstring;
    code = stack.offset(::core::mem::size_of::<cstring>() as libc::c_ulong as isize);
    cptr = temp.as_mut_ptr() as *mut cstring;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut var_i: u_int = 0 as libc::c_int as u_int;
        while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            var_tbl[i as usize].var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
            var_i = var_i.wrapping_add(1);
            var_i;
        }
        i += 1;
        i;
    }
    if code.offset(65534 as libc::c_int as isize) > partab.strstk_last {
        return -(8 as libc::c_int + 200 as libc::c_int);
    }
    if !rou.is_null() {
        s = SS_Norm(rou) as libc::c_int;
        if s < 0 as libc::c_int {
            return s;
        }
    }
    i = 0 as libc::c_int;
    if !rou.is_null() {
        let mut nsubs: libc::c_int = 0 as libc::c_int;
        while i < (*rou).slen as libc::c_int {
            cnt = 0 as libc::c_int;
            if nsubs > 0 as libc::c_int {
                return -(38 as libc::c_int);
            }
            if (*rou).slen as libc::c_int > 32 as libc::c_int + 2 as libc::c_int {
                return -(38 as libc::c_int);
            }
            ss = UTIL_Key_Extract(
                &mut *((*rou).key).as_mut_ptr().offset(i as isize),
                temp.as_mut_ptr(),
                &mut cnt,
            );
            if (ss as libc::c_int) < 0 as libc::c_int {
                return ss as libc::c_int;
            }
            if ss as libc::c_int > 32 as libc::c_int
                || (ss as libc::c_int) < 1 as libc::c_int
            {
                return -(38 as libc::c_int);
            }
            i += cnt;
            nsubs += 1;
            nsubs;
        }
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if i > 0 as libc::c_int && temp[i as usize] == 0 {
                j = i;
                while j < 32 as libc::c_int {
                    rounam.var_cu[j as usize] = '\0' as i32 as u_char;
                    j += 1;
                    j;
                }
                break;
            } else {
                if isalpha(temp[i as usize] as libc::c_int) != 0 {
                    rounam.var_cu[i as usize] = temp[i as usize];
                } else if isdigit(temp[i as usize] as libc::c_int) != 0 && i != 0 {
                    rounam.var_cu[i as usize] = temp[i as usize];
                } else if temp[i as usize] as libc::c_int == '%' as i32 && i == 0 {
                    rounam.var_cu[i as usize] = temp[i as usize];
                } else {
                    return -(38 as libc::c_int)
                }
                i += 1;
                i;
            }
        }
    } else {
        partab.checkonly = 1 as libc::c_int;
        same = 1 as libc::c_int;
        partab.sp = &mut source_ptr;
        partab.lp = &mut line;
    }
    if (*src).name.var_cu[0 as libc::c_int as usize] as libc::c_int == '$' as i32 {
        s = SS_Norm(src) as libc::c_int;
        if s < 0 as libc::c_int {
            partab.checkonly = 0 as libc::c_int;
            return s;
        }
        if memcmp(
            ((*src).name.var_cu).as_mut_ptr() as *const libc::c_void,
            b"$ROUTINE\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            9 as libc::c_int as libc::c_ulong,
        ) != 0
        {
            partab.checkonly = 0 as libc::c_int;
            return -(38 as libc::c_int);
        }
        if partab.checkonly == 0 {
            if (*src).volset as libc::c_int == (*rou).volset as libc::c_int {
                if (*src).uci as libc::c_int == (*rou).uci as libc::c_int {
                    if (*src).slen as libc::c_int == (*rou).slen as libc::c_int {
                        if memcmp(
                            ((*src).key).as_mut_ptr() as *const libc::c_void,
                            ((*rou).key).as_mut_ptr() as *const libc::c_void,
                            (*rou).slen as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            same = 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    if partab.checkonly == 0 {
        ss = SemOp(3 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int);
        if (ss as libc::c_int) < 0 as libc::c_int {
            return ss as libc::c_int;
        }
        rou_slen = (*rou).slen;
    }
    if same == 0 {
        ss = DB_Kill(rou);
        if (ss as libc::c_int) < 0 as libc::c_int {
            SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
            partab.checkonly = 0 as libc::c_int;
            return ss as libc::c_int;
        }
        Routine_Delete(rounam, (*rou).uci as libc::c_int);
    }
    src_slen = (*src).slen;
    (*line).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
    (*line).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
    (*line).len = 1 as libc::c_int as u_short;
    ss = UTIL_Key_Build(line, &mut *((*src).key).as_mut_ptr().offset(src_slen as isize));
    (*src).slen = (src_slen as libc::c_int + ss as libc::c_int) as u_char;
    comp_ptr = code;
    partab.varlst = var_tbl.as_mut_ptr();
    loop {
        s = Dorder1(((*line).buf).as_mut_ptr(), src) as libc::c_int;
        if s == 0 {
            break;
        }
        if s < 0 as libc::c_int {
            partab.checkonly = 0 as libc::c_int;
            return s;
        }
        (*line).len = s as u_short;
        s = UTIL_Key_Build(
            line,
            &mut *((*src).key).as_mut_ptr().offset(src_slen as isize),
        ) as libc::c_int;
        (*src).slen = (src_slen as libc::c_int + s) as u_char;
        s = Dget1(((*line).buf).as_mut_ptr(), src);
        if s < 1 as libc::c_int {
            continue;
        }
        (*line).len = s as u_short;
        source_ptr = ((*line).buf).as_mut_ptr();
        if isalnum(*source_ptr as libc::c_int) != 0
            || *source_ptr as libc::c_int == '%' as i32
        {
            j = isdigit(*source_ptr as libc::c_int);
            if num_tags == 256 as libc::c_int {
                comperror(-(53 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                continue;
            } else {
                tag_tbl[num_tags as usize]
                    .code = comp_ptr.offset_from(code) as libc::c_long as u_short;
                let mut var_i_0: u_int = 0 as libc::c_int as u_int;
                while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    tag_tbl[num_tags as usize]
                        .name
                        .var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
                    var_i_0 = var_i_0.wrapping_add(1);
                    var_i_0;
                }
                let fresh42 = source_ptr;
                source_ptr = source_ptr.offset(1);
                tag_tbl[num_tags as usize]
                    .name
                    .var_cu[0 as libc::c_int as usize] = *fresh42;
                i = 1 as libc::c_int;
                while !(isalnum(*source_ptr as libc::c_int) == 0) {
                    if j != 0 && isdigit(*source_ptr as libc::c_int) == 0 {
                        break;
                    }
                    if i < 32 as libc::c_int {
                        let fresh43 = i;
                        i = i + 1;
                        tag_tbl[num_tags as usize]
                            .name
                            .var_cu[fresh43 as usize] = *source_ptr;
                    }
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                }
                i = 0 as libc::c_int;
                while i < num_tags {
                    if var_equal(
                        tag_tbl[num_tags as usize].name,
                        tag_tbl[i as usize].name,
                    ) != 0
                    {
                        comperror(
                            -(65 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        p = comp_ptr;
                        comp_ptr = code
                            .offset(tag_tbl[i as usize].code as libc::c_int as isize);
                        comperror(
                            -(65 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        comp_ptr = p;
                    }
                    i += 1;
                    i;
                }
                num_tags += 1;
                num_tags;
                if *source_ptr as libc::c_int == '(' as i32 {
                    cnt = 0 as libc::c_int;
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                    let fresh44 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh44 = 171 as libc::c_int as u_char;
                    p = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    comp_ptr;
                    loop {
                        if *source_ptr as libc::c_int == ')' as i32 {
                            source_ptr = source_ptr.offset(1);
                            source_ptr;
                            break;
                        } else {
                            let mut var_i_1: u_int = 0 as libc::c_int as u_int;
                            while var_i_1
                                < (32 as libc::c_int / 8 as libc::c_int) as u_int
                            {
                                var.var_qu[var_i_1 as usize] = 0 as libc::c_int as u_int64;
                                var_i_1 = var_i_1.wrapping_add(1);
                                var_i_1;
                            }
                            i = 0 as libc::c_int;
                            while i < 32 as libc::c_int {
                                if isalpha(*source_ptr as libc::c_int) != 0
                                    || *source_ptr as libc::c_int == '%' as i32 && i == 0
                                {
                                    let fresh45 = source_ptr;
                                    source_ptr = source_ptr.offset(1);
                                    var.var_cu[i as usize] = *fresh45;
                                } else if isalnum(*source_ptr as libc::c_int) != 0 && i != 0
                                {
                                    let fresh46 = source_ptr;
                                    source_ptr = source_ptr.offset(1);
                                    var.var_cu[i as usize] = *fresh46;
                                } else {
                                    if *source_ptr as libc::c_int == ',' as i32 && i != 0 {
                                        break;
                                    }
                                    if *source_ptr as libc::c_int == ')' as i32 {
                                        break;
                                    }
                                    cnt = -(13 as libc::c_int + 200 as libc::c_int);
                                    break;
                                }
                                i += 1;
                                i;
                            }
                            if cnt < 0 as libc::c_int {
                                break;
                            }
                            if isalnum(*source_ptr as libc::c_int) != 0 {
                                cnt = -(56 as libc::c_int);
                                break;
                            } else {
                                if *source_ptr as libc::c_int == ',' as i32 {
                                    source_ptr = source_ptr.offset(1);
                                    source_ptr;
                                } else if *source_ptr as libc::c_int != ')' as i32 {
                                    cnt = -(13 as libc::c_int + 200 as libc::c_int);
                                    break;
                                }
                                i = 0 as libc::c_int;
                                while i < 256 as libc::c_int {
                                    if var_equal(var_tbl[i as usize], var) != 0 {
                                        break;
                                    }
                                    if var_empty(var_tbl[i as usize]) != 0 {
                                        let mut var_i_2: u_int = 0 as libc::c_int as u_int;
                                        while var_i_2
                                            < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                        {
                                            var_tbl[i as usize]
                                                .var_qu[var_i_2 as usize] = var.var_qu[var_i_2 as usize];
                                            var_i_2 = var_i_2.wrapping_add(1);
                                            var_i_2;
                                        }
                                        break;
                                    } else {
                                        i += 1;
                                        i;
                                    }
                                }
                                if i == 256 as libc::c_int {
                                    cnt = -(74 as libc::c_int + 200 as libc::c_int);
                                    break;
                                } else {
                                    let fresh47 = comp_ptr;
                                    comp_ptr = comp_ptr.offset(1);
                                    *fresh47 = i as u_char;
                                    cnt += 1;
                                    cnt;
                                    j = 1 as libc::c_int;
                                    while j < cnt {
                                        if *p.offset(j as isize) as libc::c_int == i {
                                            cnt = -(21 as libc::c_int);
                                            break;
                                        } else {
                                            j += 1;
                                            j;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if cnt > 127 as libc::c_int - 1 as libc::c_int {
                        cnt = -(75 as libc::c_int + 200 as libc::c_int);
                    }
                    if cnt < 0 as libc::c_int {
                        p = p.offset(-1);
                        p;
                        comp_ptr = p;
                        comperror(cnt as libc::c_short);
                        continue;
                    } else {
                        *p = cnt as u_char;
                    }
                }
            }
        }
        lino += 1;
        lino;
        if same == 0 {
            i = 0 as libc::c_int;
            while *source_ptr.offset(i as isize) as libc::c_int == '\t' as i32 {
                let fresh48 = i;
                i = i + 1;
                *source_ptr.offset(fresh48 as isize) = ' ' as i32 as u_char;
            }
            (*cptr).len = itocstring(((*cptr).buf).as_mut_ptr(), lino);
            s = UTIL_Key_Build(
                cptr,
                &mut *((*rou).key).as_mut_ptr().offset(rou_slen as isize),
            ) as libc::c_int;
            (*rou).slen = (rou_slen as libc::c_int + s) as u_char;
            s = DB_Set(rou, line);
            if s < 0 as libc::c_int {
                if partab.checkonly == 0 {
                    SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
                }
                partab.varlst = 0 as *mut var_u;
                partab.checkonly = 0 as libc::c_int;
                return s;
            }
        }
        if lino > 65534 as libc::c_int {
            comperror(-(54 as libc::c_int + 200 as libc::c_int) as libc::c_short);
        } else {
            if *source_ptr as libc::c_int == ';' as i32 {
                continue;
            }
            let fresh49 = source_ptr;
            source_ptr = source_ptr.offset(1);
            if *fresh49 as libc::c_int != ' ' as i32 {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
            } else {
                while *source_ptr as libc::c_int == ' ' as i32 {
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                }
                let fresh50 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh50 = 170 as libc::c_int as u_char;
                us = lino as u_short;
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    &mut us as *mut u_short as *const libc::c_void,
                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                );
                comp_ptr = comp_ptr
                    .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
                p = comp_ptr;
                let fresh51 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh51 = 0 as libc::c_int as u_char;
                let fresh52 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh52 = 0 as libc::c_int as u_char;
                j = 0 as libc::c_int;
                if *source_ptr as libc::c_int == '.' as i32 {
                    i = 0 as libc::c_int;
                    loop {
                        if *source_ptr as libc::c_int == '.' as i32 {
                            j += 1;
                            j;
                            source_ptr = source_ptr.offset(1);
                            source_ptr;
                        } else {
                            if !(*source_ptr as libc::c_int == ' ' as i32) {
                                break;
                            }
                            source_ptr = source_ptr.offset(1);
                            source_ptr;
                        }
                        i += 1;
                        i;
                    }
                }
                if j > 255 as libc::c_int {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                } else {
                    if j != 0 || last_dots != 0 {
                        let fresh53 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh53 = 156 as libc::c_int as u_char;
                        let fresh54 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh54 = j as u_char;
                        last_dots = j;
                    }
                    if *source_ptr as libc::c_int != ';' as i32
                        && *source_ptr as libc::c_int != '\0' as i32
                    {
                        parse();
                    } else {
                        let fresh55 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh55 = 0 as libc::c_int as u_char;
                    }
                    *(p
                        as *mut u_short) = (comp_ptr.offset_from(p) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as u_short;
                }
            }
        }
    }
    let fresh56 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh56 = 157 as libc::c_int as u_char;
    let fresh57 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh57 = 0 as libc::c_int as u_char;
    let fresh58 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh58 = 0 as libc::c_int as u_char;
    partab.varlst = 0 as *mut var_u;
    num_vars = 0 as libc::c_int;
    while var_empty(var_tbl[num_vars as usize]) == 0 {
        num_vars += 1;
        num_vars;
    }
    p = ((*line).buf).as_mut_ptr();
    (*cptr).len = Vhorolog(((*cptr).buf).as_mut_ptr()) as u_short;
    us = 8 as libc::c_int as u_short;
    memcpy(
        p as *mut libc::c_void,
        &mut us as *mut u_short as *const libc::c_void,
        ::core::mem::size_of::<u_short>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
    us = (*partab.jobtab).user as u_short;
    memcpy(
        p as *mut libc::c_void,
        &mut us as *mut u_short as *const libc::c_void,
        ::core::mem::size_of::<u_short>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
    i = cstringtoi(cptr);
    memcpy(
        p as *mut libc::c_void,
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize);
    i = atoi(
        &mut *((*cptr).buf).as_mut_ptr().offset(6 as libc::c_int as isize) as *mut u_char
            as *mut libc::c_char,
    );
    memcpy(
        p as *mut libc::c_void,
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize);
    i = (::core::mem::size_of::<tags>() as libc::c_ulong)
        .wrapping_mul(num_tags as libc::c_ulong) as libc::c_int;
    j = (::core::mem::size_of::<var_u>() as libc::c_ulong)
        .wrapping_mul(num_vars as libc::c_ulong) as libc::c_int;
    uss = (p.offset_from(((*line).buf).as_mut_ptr()) as libc::c_long as libc::c_ulong)
        .wrapping_add(
            (6 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<u_short>() as libc::c_ulong),
        ) as u_short;
    memcpy(
        &mut *((*line).buf).as_mut_ptr().offset(uss as isize) as *mut u_char
            as *mut libc::c_void,
        tag_tbl.as_mut_ptr() as *const libc::c_void,
        i as libc::c_ulong,
    );
    memcpy(
        p as *mut libc::c_void,
        &mut uss as *mut u_short as *const libc::c_void,
        ::core::mem::size_of::<u_short>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
    us = num_tags as u_short;
    memcpy(
        p as *mut libc::c_void,
        &mut us as *mut u_short as *const libc::c_void,
        ::core::mem::size_of::<u_short>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
    uss = (uss as libc::c_int + i) as u_short;
    memcpy(
        &mut *((*line).buf).as_mut_ptr().offset(uss as isize) as *mut u_char
            as *mut libc::c_void,
        var_tbl.as_mut_ptr() as *const libc::c_void,
        j as libc::c_ulong,
    );
    memcpy(
        p as *mut libc::c_void,
        &mut uss as *mut u_short as *const libc::c_void,
        ::core::mem::size_of::<u_short>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
    us = num_vars as u_short;
    memcpy(
        p as *mut libc::c_void,
        &mut us as *mut u_short as *const libc::c_void,
        ::core::mem::size_of::<u_short>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
    uss = (uss as libc::c_int + j) as u_short;
    memmove(
        &mut *((*line).buf).as_mut_ptr().offset(uss as isize) as *mut u_char
            as *mut libc::c_void,
        code as *const libc::c_void,
        comp_ptr.offset_from(code) as libc::c_long as libc::c_ulong,
    );
    memcpy(
        p as *mut libc::c_void,
        &mut uss as *mut u_short as *const libc::c_void,
        ::core::mem::size_of::<u_short>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
    us = comp_ptr.offset_from(code) as libc::c_long as u_short;
    memcpy(
        p as *mut libc::c_void,
        &mut us as *mut u_short as *const libc::c_void,
        ::core::mem::size_of::<u_short>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
    i = (p.offset_from(((*line).buf).as_mut_ptr()) as libc::c_long
        + comp_ptr.offset_from(code) as libc::c_long + i as libc::c_long
        + j as libc::c_long) as libc::c_int;
    if i > 65534 as libc::c_int {
        comperror(-(54 as libc::c_int + 200 as libc::c_int) as libc::c_short);
        if partab.checkonly == 0 {
            SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
        }
        partab.checkonly = 0 as libc::c_int;
        return -(75 as libc::c_int);
    }
    (*line).len = i as u_short;
    (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
    (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
    (*cptr).len = 1 as libc::c_int as u_short;
    if partab.checkonly != 0 {
        partab.checkonly = 0 as libc::c_int;
        return partab.errors as libc::c_int;
    }
    s = UTIL_Key_Build(cptr, &mut *((*rou).key).as_mut_ptr().offset(rou_slen as isize))
        as libc::c_int;
    (*rou).slen = (rou_slen as libc::c_int + s) as u_char;
    s = DB_Set(rou, line);
    if same != 0 {
        Routine_Delete(rounam, (*rou).uci as libc::c_int);
    }
    i = SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int) as libc::c_int;
    partab.checkonly = 0 as libc::c_int;
    return s;
}
