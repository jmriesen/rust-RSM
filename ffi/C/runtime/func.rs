#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type RBD;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn random() -> libc::c_long;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn DB_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn DB_Set(var: *mut mvar, data: *mut cstring) -> libc::c_int;
    fn DB_Data(var: *mut mvar, buf: *mut u_char) -> libc::c_short;
    fn DB_Order(var: *mut mvar, buf: *mut u_char, dir: libc::c_int) -> libc::c_short;
    fn DB_Query(var: *mut mvar, buf: *mut u_char, dir: libc::c_int) -> libc::c_short;
    fn cstringtoi(str: *mut cstring) -> libc::c_int;
    fn itocstring(buf: *mut u_char, n: libc::c_int) -> u_short;
    fn uitocstring(buf: *mut u_char, n: u_int) -> u_short;
    fn runtime_add(a: *mut libc::c_char, b: *mut libc::c_char) -> libc::c_short;
    fn ST_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn ST_Set(var: *mut mvar, data: *mut cstring) -> libc::c_int;
    fn ST_Data(var: *mut mvar, buf: *mut u_char) -> libc::c_short;
    fn ST_Order(var: *mut mvar, buf: *mut u_char, dir: libc::c_int) -> libc::c_short;
    fn ST_Query(var: *mut mvar, buf: *mut u_char, dir: libc::c_int) -> libc::c_short;
    fn SS_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn SS_Data(var: *mut mvar, buf: *mut u_char) -> libc::c_short;
    fn SS_Order(var: *mut mvar, buf: *mut u_char, dir: libc::c_int) -> libc::c_short;
    fn UTIL_Key_Build(src: *mut cstring, dest: *mut u_char) -> libc::c_short;
    fn UTIL_String_Mvar(
        var: *mut mvar,
        str: *mut u_char,
        max_subs: libc::c_int,
    ) -> libc::c_short;
    fn mcopy(src: *mut u_char, dst: *mut u_char, bytes: libc::c_int) -> libc::c_int;
    fn ncopy(src: *mut *mut u_char, dst: *mut u_char) -> libc::c_short;
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
pub unsafe extern "C" fn isdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x400 as libc::c_long as libc::c_ulong);
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
pub unsafe extern "C" fn Dascii1(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
) -> libc::c_short {
    return Dascii2(ret_buffer, expr, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Dascii2(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
    mut posn: libc::c_int,
) -> libc::c_short {
    let mut asc: libc::c_int = -(1 as libc::c_int);
    if posn > 0 as libc::c_int && posn <= (*expr).len as libc::c_int {
        asc = (*expr).buf[(posn - 1 as libc::c_int) as usize] as libc::c_int;
    }
    return itocstring(ret_buffer, asc) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Dchar(
    mut ret_buffer: *mut u_char,
    mut i: libc::c_int,
) -> libc::c_short {
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if i < 0 as libc::c_int || i > 255 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    *ret_buffer.offset(0 as libc::c_int as isize) = i as u_char;
    *ret_buffer.offset(1 as libc::c_int as isize) = '\0' as i32 as u_char;
    return 1 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Ddata(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
) -> libc::c_short {
    if (*var).uci as libc::c_int == 255 as libc::c_int {
        return ST_Data(var, ret_buffer);
    }
    if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int == '$' as i32 {
        return SS_Data(var, ret_buffer);
    }
    memcpy(
        &mut (*partab.jobtab).last_ref as *mut mvar as *mut libc::c_void,
        var as *const libc::c_void,
        (::core::mem::size_of::<var_u>() as libc::c_ulong)
            .wrapping_add(5 as libc::c_int as libc::c_ulong)
            .wrapping_add((*var).slen as libc::c_ulong),
    );
    return DB_Data(var, ret_buffer);
}
#[no_mangle]
pub unsafe extern "C" fn Dextract(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
    mut start: libc::c_int,
    mut stop: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if start < 1 as libc::c_int && stop > 0 as libc::c_int {
        start = 1 as libc::c_int;
    }
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if start < 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if stop > (*expr).len as libc::c_int {
        stop = (*expr).len as libc::c_int;
    }
    if stop < start || start > (*expr).len as libc::c_int {
        return 0 as libc::c_int;
    }
    i = stop - start + 1 as libc::c_int;
    memmove(
        ret_buffer as *mut libc::c_void,
        &mut *((*expr).buf).as_mut_ptr().offset((start - 1 as libc::c_int) as isize)
            as *mut u_char as *const libc::c_void,
        i as libc::c_ulong,
    );
    *ret_buffer.offset(i as isize) = '\0' as i32 as u_char;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn Dfind2(
    mut ret_buffer: *mut u_char,
    mut expr1: *mut cstring,
    mut expr2: *mut cstring,
) -> libc::c_int {
    return Dfind3(ret_buffer, expr1, expr2, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Dfind3(
    mut ret_buffer: *mut u_char,
    mut expr1: *mut cstring,
    mut expr2: *mut cstring,
    mut start: libc::c_int,
) -> libc::c_int {
    return itocstring(ret_buffer, Dfind3x(expr1, expr2, start)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Dfind3x(
    mut expr1: *mut cstring,
    mut expr2: *mut cstring,
    mut start: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if start < 1 as libc::c_int {
        start = 1 as libc::c_int;
    }
    if (*expr2).len as libc::c_int == 0 as libc::c_int {
        return start;
    }
    i = start - 1 as libc::c_int;
    while i < (*expr1).len as libc::c_int {
        j = 0 as libc::c_int;
        while j != (*expr2).len as libc::c_int {
            if (*expr1).buf[(i + j) as usize] as libc::c_int
                != (*expr2).buf[j as usize] as libc::c_int
            {
                break;
            }
            if j + 1 as libc::c_int == (*expr2).len as libc::c_int {
                ret = i + j + 2 as libc::c_int;
            }
            j += 1;
            j;
        }
        if ret != 0 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Dfnumber2(
    mut ret_buffer: *mut u_char,
    mut numexp: *mut cstring,
    mut code: *mut cstring,
) -> libc::c_int {
    let mut tempc: *mut cstring = 0 as *mut cstring;
    let mut dest: *mut cstring = 0 as *mut cstring;
    let mut z: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut a1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dp: *mut libc::c_char = 0 as *mut libc::c_char;
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    a1 = strchr(
        &mut *((*code).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_char,
        'p' as i32,
    );
    a2 = strchr(
        &mut *((*code).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_char,
        'P' as i32,
    );
    b1 = strchr(
        &mut *((*code).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_char,
        '+' as i32,
    );
    b2 = strchr(
        &mut *((*code).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_char,
        '-' as i32,
    );
    c1 = strchr(
        &mut *((*code).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_char,
        't' as i32,
    );
    c2 = strchr(
        &mut *((*code).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_char,
        'T' as i32,
    );
    d1 = strchr(
        &mut *((*code).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_char,
        ',' as i32,
    );
    dp = strchr(
        &mut *((*numexp).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_char as *const libc::c_char,
        '.' as i32,
    );
    if (!a1.is_null() || !a2.is_null())
        && (!b1.is_null() || !b2.is_null() || !c1.is_null() || !c2.is_null())
    {
        return -(2 as libc::c_int);
    }
    if (*numexp).len as libc::c_int > 1 as libc::c_int {
        z = 0 as libc::c_int;
        while z <= (*numexp).len as libc::c_int {
            if (*numexp).buf[z as usize] as libc::c_int != '0' as i32 {
                break;
            }
            z += 1;
            z;
        }
    } else {
        z = 0 as libc::c_int;
    }
    if z == 1 as libc::c_int
        && (*numexp).buf[1 as libc::c_int as usize] as libc::c_int == '.' as i32
    {
        z = 0 as libc::c_int;
    }
    tempc = malloc(
        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_add((*numexp).len as libc::c_ulong)
            .wrapping_add(
                ((*numexp).len as libc::c_int / 3 as libc::c_int) as libc::c_ulong,
            )
            .wrapping_add(3 as libc::c_int as libc::c_ulong),
    ) as *mut cstring;
    dest = malloc(
        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_add((*numexp).len as libc::c_ulong)
            .wrapping_add(
                ((*numexp).len as libc::c_int / 3 as libc::c_int) as libc::c_ulong,
            )
            .wrapping_add(3 as libc::c_int as libc::c_ulong),
    ) as *mut cstring;
    (*dest).len = ((*numexp).len as libc::c_int - z) as u_short;
    memcpy(
        ((*dest).buf).as_mut_ptr() as *mut libc::c_void,
        &mut *((*numexp).buf).as_mut_ptr().offset(z as isize) as *mut u_char
            as *const libc::c_void,
        (*numexp).len as libc::c_ulong,
    );
    if !d1.is_null() {
        let mut ndlen: libc::c_int = 0;
        let mut nlen: libc::c_int = 0;
        let mut nc: libc::c_int = 0;
        let mut cd: libc::c_int = 0 as libc::c_int;
        let mut ptr1: *mut u_char = 0 as *mut u_char;
        if !dp.is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i <= (*dest).len as libc::c_int - 1 as libc::c_int {
                if (*dest).buf[i as usize] as libc::c_int == '.' as i32 {
                    break;
                }
                i += 1;
                i;
            }
            ndlen = i;
            if (*numexp).buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
                ndlen -= 1 as libc::c_int;
            }
            nc = ndlen / 3 as libc::c_int;
            if ndlen % 3 as libc::c_int == 0 as libc::c_int && i > 0 as libc::c_int {
                nc -= 1 as libc::c_int;
            }
            nlen = (*dest).len as libc::c_int + nc - 1 as libc::c_int;
            (*tempc).len = (nlen + 1 as libc::c_int) as u_short;
            ptr1 = &mut *((*dest).buf)
                .as_mut_ptr()
                .offset(((*dest).len as libc::c_int - 1 as libc::c_int) as isize)
                as *mut u_char;
            while *ptr1 as libc::c_int != '.' as i32 {
                memcpy(
                    &mut *((*tempc).buf).as_mut_ptr().offset(nlen as isize)
                        as *mut u_char as *mut libc::c_void,
                    ptr1 as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                );
                nlen -= 1 as libc::c_int;
                ptr1 = ptr1.offset(-(1 as libc::c_int as isize));
            }
            memcpy(
                &mut *((*tempc).buf).as_mut_ptr().offset(nlen as isize) as *mut u_char
                    as *mut libc::c_void,
                ptr1 as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
            );
            nlen -= 1 as libc::c_int;
            ptr1 = ptr1.offset(-(1 as libc::c_int as isize));
            while nlen >= 0 as libc::c_int {
                cd += 1 as libc::c_int;
                memcpy(
                    &mut *((*tempc).buf).as_mut_ptr().offset(nlen as isize)
                        as *mut u_char as *mut libc::c_void,
                    ptr1 as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                );
                nlen -= 1 as libc::c_int;
                ptr1 = ptr1.offset(-(1 as libc::c_int as isize));
                if cd % 3 as libc::c_int == 0 as libc::c_int && nlen > 0 as libc::c_int {
                    (*tempc).buf[nlen as usize] = ',' as i32 as u_char;
                    nlen -= 1 as libc::c_int;
                }
            }
        } else {
            ndlen = (*numexp).len as libc::c_int;
            if (*numexp).buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
                ndlen -= 1 as libc::c_int;
            }
            nc = ndlen / 3 as libc::c_int;
            if ndlen % 3 as libc::c_int == 0 as libc::c_int {
                nc -= 1 as libc::c_int;
            }
            nlen = (*numexp).len as libc::c_int + nc - 1 as libc::c_int;
            (*tempc).len = (nlen + 1 as libc::c_int) as u_short;
            ptr1 = &mut *((*dest).buf)
                .as_mut_ptr()
                .offset(((*dest).len as libc::c_int - 1 as libc::c_int) as isize)
                as *mut u_char;
            while nlen >= 0 as libc::c_int {
                cd += 1 as libc::c_int;
                memcpy(
                    &mut *((*tempc).buf).as_mut_ptr().offset(nlen as isize)
                        as *mut u_char as *mut libc::c_void,
                    ptr1 as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                );
                nlen -= 1 as libc::c_int;
                ptr1 = ptr1.offset(-(1 as libc::c_int as isize));
                if cd % 3 as libc::c_int == 0 as libc::c_int && nlen > 0 as libc::c_int {
                    (*tempc).buf[nlen as usize] = ',' as i32 as u_char;
                    nlen -= 1 as libc::c_int;
                }
            }
        }
        (*dest).len = (*tempc).len;
        memcpy(
            ((*dest).buf).as_mut_ptr() as *mut libc::c_void,
            ((*tempc).buf).as_mut_ptr() as *const libc::c_void,
            (*tempc).len as libc::c_ulong,
        );
    }
    if !a1.is_null() || !a2.is_null() {
        if (*numexp).buf[0 as libc::c_int as usize] as libc::c_int != '-' as i32 {
            (*tempc).buf[0 as libc::c_int as usize] = ' ' as i32 as u_char;
            memcpy(
                &mut *((*tempc).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_void,
                ((*dest).buf).as_mut_ptr() as *const libc::c_void,
                (*dest).len as libc::c_ulong,
            );
            (*tempc)
                .buf[(1 as libc::c_int + (*dest).len as libc::c_int)
                as usize] = ' ' as i32 as u_char;
            (*tempc).len = ((*dest).len as libc::c_int + 2 as libc::c_int) as u_short;
        }
        if (*numexp).buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
            (*tempc).buf[0 as libc::c_int as usize] = '(' as i32 as u_char;
            memcpy(
                &mut *((*tempc).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_void,
                &mut *((*dest).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                ((*dest).len as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
            (*tempc).buf[(*dest).len as usize] = ')' as i32 as u_char;
            (*tempc).len = ((*dest).len as libc::c_int + 1 as libc::c_int) as u_short;
        }
        (*dest).len = (*tempc).len;
        memcpy(
            ((*dest).buf).as_mut_ptr() as *mut libc::c_void,
            ((*tempc).buf).as_mut_ptr() as *const libc::c_void,
            (*tempc).len as libc::c_ulong,
        );
    }
    if !c1.is_null() || !c2.is_null() {
        if (*numexp).buf[0 as libc::c_int as usize] as libc::c_int != '-' as i32 {
            if !b1.is_null() {
                if (*dest).buf[0 as libc::c_int as usize] as libc::c_int == '+' as i32 {
                    memcpy(
                        &mut *((*tempc).buf)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut u_char
                            as *mut libc::c_void,
                        &mut *((*dest).buf)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize) as *mut u_char
                            as *const libc::c_void,
                        ((*dest).len as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                    );
                    (*tempc)
                        .buf[((*dest).len as libc::c_int - 1 as libc::c_int)
                        as usize] = '+' as i32 as u_char;
                    (*tempc).len = (*dest).len;
                } else {
                    memcpy(
                        &mut *((*tempc).buf)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut u_char
                            as *mut libc::c_void,
                        &mut *((*dest).buf)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut u_char
                            as *const libc::c_void,
                        (*dest).len as libc::c_ulong,
                    );
                    (*tempc).buf[(*dest).len as usize] = '+' as i32 as u_char;
                    (*tempc)
                        .len = ((*dest).len as libc::c_int + 1 as libc::c_int)
                        as u_short;
                }
            } else {
                memcpy(
                    ((*tempc).buf).as_mut_ptr() as *mut libc::c_void,
                    &mut *((*dest).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    (*dest).len as libc::c_ulong,
                );
                (*tempc).buf[(*dest).len as usize] = ' ' as i32 as u_char;
                (*tempc)
                    .len = ((*dest).len as libc::c_int + 1 as libc::c_int) as u_short;
            }
        } else if !b2.is_null() {
            memcpy(
                &mut *((*tempc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_void,
                &mut *((*dest).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                ((*dest).len as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
            (*tempc)
                .buf[((*dest).len as libc::c_int - 1 as libc::c_int)
                as usize] = ' ' as i32 as u_char;
            (*tempc).len = (*dest).len;
        } else {
            memcpy(
                &mut *((*tempc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_void,
                &mut *((*dest).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                ((*dest).len as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
            (*tempc)
                .buf[((*dest).len as libc::c_int - 1 as libc::c_int)
                as usize] = '-' as i32 as u_char;
            (*tempc).len = (*dest).len;
        }
        (*dest).len = (*tempc).len;
        memcpy(
            ((*dest).buf).as_mut_ptr() as *mut libc::c_void,
            ((*tempc).buf).as_mut_ptr() as *const libc::c_void,
            (*tempc).len as libc::c_ulong,
        );
    } else {
        if (*numexp).buf[0 as libc::c_int as usize] as libc::c_int != '-' as i32 {
            if (*numexp).buf[0 as libc::c_int as usize] as libc::c_int == '0' as i32
                && (*numexp).len as libc::c_int == 1 as libc::c_int
            {
                b1 = 0 as *mut libc::c_char;
            }
            if !b1.is_null() {
                if (*dest).buf[0 as libc::c_int as usize] as libc::c_int != '+' as i32 {
                    (*tempc).buf[0 as libc::c_int as usize] = '+' as i32 as u_char;
                    memcpy(
                        &mut *((*tempc).buf)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize) as *mut u_char
                            as *mut libc::c_void,
                        ((*dest).buf).as_mut_ptr() as *const libc::c_void,
                        (*dest).len as libc::c_ulong,
                    );
                    (*tempc)
                        .len = ((*dest).len as libc::c_int + 1 as libc::c_int)
                        as u_short;
                }
            } else {
                memcpy(
                    ((*tempc).buf).as_mut_ptr() as *mut libc::c_void,
                    &mut *((*dest).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    (*dest).len as libc::c_ulong,
                );
                (*tempc).len = (*dest).len;
            }
        } else if !b2.is_null() {
            memcpy(
                ((*tempc).buf).as_mut_ptr() as *mut libc::c_void,
                &mut *((*dest).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                ((*dest).len as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
            (*tempc).len = ((*dest).len as libc::c_int - 1 as libc::c_int) as u_short;
        } else {
            memcpy(
                ((*tempc).buf).as_mut_ptr() as *mut libc::c_void,
                &mut *((*dest).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                (*dest).len as libc::c_ulong,
            );
            (*tempc).len = (*dest).len;
        }
        (*dest).len = (*tempc).len;
        memcpy(
            ((*dest).buf).as_mut_ptr() as *mut libc::c_void,
            ((*tempc).buf).as_mut_ptr() as *const libc::c_void,
            (*tempc).len as libc::c_ulong,
        );
    }
    (*dest).len = (*tempc).len;
    memcpy(
        ret_buffer as *mut libc::c_void,
        ((*dest).buf).as_mut_ptr() as *const libc::c_void,
        (*dest).len as libc::c_ulong,
    );
    *ret_buffer.offset((*dest).len as isize) = '\0' as i32 as u_char;
    dlen = (*dest).len as libc::c_int;
    free(tempc as *mut libc::c_void);
    free(dest as *mut libc::c_void);
    return dlen;
}
#[no_mangle]
pub unsafe extern "C" fn Dfnumber3(
    mut ret_buffer: *mut u_char,
    mut numexp: *mut cstring,
    mut code: *mut cstring,
    mut rnd: libc::c_int,
) -> libc::c_int {
    let mut change: *mut cstring = 0 as *mut cstring;
    let mut s: libc::c_int = 0;
    s = Djustify3(ret_buffer, numexp, 0 as libc::c_int, rnd);
    if s < 0 as libc::c_int {
        return s;
    }
    change = malloc(
        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_add(s as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut cstring;
    (*change).len = s as u_short;
    memcpy(
        ((*change).buf).as_mut_ptr() as *mut libc::c_void,
        ret_buffer as *const libc::c_void,
        (s + 1 as libc::c_int) as libc::c_ulong,
    );
    return Dfnumber2(ret_buffer, change, code);
}
#[no_mangle]
pub unsafe extern "C" fn Dget1(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
) -> libc::c_int {
    let mut tmp: [u_char; 8] = [0; 8];
    let mut cptr: *mut cstring = 0 as *mut cstring;
    cptr = tmp.as_mut_ptr() as *mut cstring;
    (*cptr).len = 0 as libc::c_int as u_short;
    (*cptr).buf[0 as libc::c_int as usize] = '\0' as i32 as u_char;
    return Dget2(ret_buffer, var, cptr);
}
#[no_mangle]
pub unsafe extern "C" fn Dget2(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
    mut expr: *mut cstring,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    if (*var).uci as libc::c_int == 255 as libc::c_int {
        s = ST_Get(var, ret_buffer);
        if s >= 0 as libc::c_int {
            return s;
        }
        if s == -(6 as libc::c_int) {
            s = 0 as libc::c_int;
        }
    } else if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int == '$' as i32
    {
        s = SS_Get(var, ret_buffer);
        if s >= 0 as libc::c_int {
            return s;
        }
        if s == -(38 as libc::c_int) || s == -(7 as libc::c_int) {
            s = 0 as libc::c_int;
        }
    } else {
        memcpy(
            &mut (*partab.jobtab).last_ref as *mut mvar as *mut libc::c_void,
            var as *const libc::c_void,
            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                .wrapping_add((*var).slen as libc::c_ulong),
        );
        s = DB_Get(var, ret_buffer);
        if s >= 0 as libc::c_int {
            return s;
        }
        if s == -(7 as libc::c_int) {
            s = 0 as libc::c_int;
        }
    }
    if s != 0 as libc::c_int {
        return s;
    }
    memmove(
        &mut *ret_buffer.offset(0 as libc::c_int as isize) as *mut u_char
            as *mut libc::c_void,
        &mut *((*expr).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_void,
        (*expr).len as libc::c_ulong,
    );
    *ret_buffer.offset((*expr).len as isize) = '\0' as i32 as u_char;
    return (*expr).len as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Dincrement1(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
) -> libc::c_short {
    let mut tmp: [u_char; 256] = [0; 256];
    let mut cptr: *mut cstring = 0 as *mut cstring;
    cptr = tmp.as_mut_ptr() as *mut cstring;
    (*cptr).len = 1 as libc::c_int as u_short;
    (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
    (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
    return Dincrement2(ret_buffer, var, cptr);
}
#[no_mangle]
pub unsafe extern "C" fn Dincrement2(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
    mut numexpr: *mut cstring,
) -> libc::c_short {
    let mut s: libc::c_int = 0;
    let mut temp: [u_char; 65534] = [0; 65534];
    let mut tmp: *mut u_char = 0 as *mut u_char;
    tmp = temp.as_mut_ptr();
    if (*var).uci as libc::c_int == 255 as libc::c_int {
        s = ST_Get(var, temp.as_mut_ptr());
    } else {
        memcpy(
            &mut (*partab.jobtab).last_ref as *mut mvar as *mut libc::c_void,
            var as *const libc::c_void,
            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                .wrapping_add((*var).slen as libc::c_ulong),
        );
        s = DB_Get(var, temp.as_mut_ptr());
    }
    if s == -(6 as libc::c_int) || s == -(7 as libc::c_int) {
        *ret_buffer.offset(0 as libc::c_int as isize) = '0' as i32 as u_char;
        s = 0 as libc::c_int;
    } else if s < 0 as libc::c_int {
        return s as libc::c_short
    } else {
        s = ncopy(&mut tmp, ret_buffer) as libc::c_int;
    }
    if s < 0 as libc::c_int {
        return s as libc::c_short;
    }
    (*numexpr)
        .len = runtime_add(
        ((*numexpr).buf).as_mut_ptr() as *mut libc::c_char,
        ret_buffer as *mut libc::c_char,
    ) as u_short;
    memmove(
        &mut *ret_buffer.offset(0 as libc::c_int as isize) as *mut u_char
            as *mut libc::c_void,
        &mut *((*numexpr).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_char as *const libc::c_void,
        (*numexpr).len as libc::c_ulong,
    );
    *ret_buffer.offset((*numexpr).len as isize) = '\0' as i32 as u_char;
    if (*var).uci as libc::c_int == 255 as libc::c_int {
        return ST_Set(var, numexpr) as libc::c_short;
    }
    return DB_Set(var, numexpr) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Djustify2(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut adj: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    adj = size - (*expr).len as libc::c_int;
    if adj < 0 as libc::c_int {
        adj = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < adj {
        *ret_buffer.offset(i as isize) = ' ' as i32 as u_char;
        i += 1;
        i;
    }
    memmove(
        &mut *ret_buffer.offset(adj as isize) as *mut u_char as *mut libc::c_void,
        &mut *((*expr).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_void,
        (*expr).len as libc::c_ulong,
    );
    i = (*expr).len as libc::c_int + adj;
    *ret_buffer.offset(i as isize) = '\0' as i32 as u_char;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn Djustify3(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
    mut size: libc::c_int,
    mut round: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut spc: libc::c_int = 0;
    let mut zer: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut ru: libc::c_int = -(2 as libc::c_int);
    let mut dp: libc::c_int = -(2 as libc::c_int);
    let mut cop: libc::c_int = 0;
    if round < 0 as libc::c_int {
        return -(28 as libc::c_int);
    }
    if size < 0 as libc::c_int {
        size = 0 as libc::c_int;
    }
    len = (*expr).len as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*expr).len as libc::c_int {
        if (*expr).buf[i as usize] as libc::c_int == '.' as i32 {
            dp = i;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if dp == 0
        || dp == 1 as libc::c_int
            && (*expr).buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32
    {
        zer = 1 as libc::c_int;
    }
    if round == 0 {
        if dp != -(2 as libc::c_int) {
            len = dp;
            if (*expr).buf[(dp + 1 as libc::c_int) as usize] as libc::c_int > '4' as i32
            {
                ru = dp - 1 as libc::c_int;
            }
        }
    } else if dp != -(2 as libc::c_int) {
        len = dp + round + 1 as libc::c_int;
        if len < (*expr).len as libc::c_int
            && (*expr).buf[len as usize] as libc::c_int > '4' as i32
        {
            ru = len - 1 as libc::c_int;
        }
    } else {
        len += round + 1 as libc::c_int;
    }
    spc = size - len - zer;
    if spc < 0 as libc::c_int {
        spc = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < spc {
        let fresh0 = i;
        i = i + 1;
        *ret_buffer.offset(fresh0 as isize) = ' ' as i32 as u_char;
    }
    if (*expr).buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
        let fresh1 = i;
        i = i + 1;
        *ret_buffer.offset(fresh1 as isize) = '-' as i32 as u_char;
        j = 1 as libc::c_int;
    }
    cop = 0 as libc::c_int;
    while cop < zer {
        let fresh2 = i;
        i = i + 1;
        *ret_buffer.offset(fresh2 as isize) = '0' as i32 as u_char;
        cop += 1;
        cop;
    }
    cop = (*expr).len as libc::c_int - j;
    if len < (*expr).len as libc::c_int {
        cop = len - j;
    }
    memmove(
        &mut *ret_buffer.offset(i as isize) as *mut u_char as *mut libc::c_void,
        &mut *((*expr).buf).as_mut_ptr().offset(j as isize) as *mut u_char
            as *const libc::c_void,
        cop as libc::c_ulong,
    );
    i += cop;
    len += zer + spc;
    if dp == -(2 as libc::c_int) && i < len {
        let fresh3 = i;
        i = i + 1;
        *ret_buffer.offset(fresh3 as isize) = '.' as i32 as u_char;
    }
    while i < len {
        let fresh4 = i;
        i = i + 1;
        *ret_buffer.offset(fresh4 as isize) = '0' as i32 as u_char;
    }
    *ret_buffer.offset(len as isize) = '\0' as i32 as u_char;
    if ru != -(2 as libc::c_int) {
        ru += zer + spc;
        loop {
            let ref mut fresh5 = *ret_buffer.offset(ru as isize);
            *fresh5 = (*fresh5).wrapping_add(1);
            *fresh5;
            if *ret_buffer.offset(ru as isize) as libc::c_int <= '9' as i32 {
                break;
            }
            *ret_buffer.offset(ru as isize) = '0' as i32 as u_char;
            ru -= 1;
            ru;
            if *ret_buffer.offset(ru as isize) as libc::c_int == '.' as i32 {
                ru -= 1;
                ru;
            }
            if *ret_buffer.offset(ru as isize) as libc::c_int == ' ' as i32 {
                *ret_buffer.offset(ru as isize) = '0' as i32 as u_char;
            }
            if *ret_buffer.offset(ru as isize) as libc::c_int == '-' as i32
                && *ret_buffer.offset(0 as libc::c_int as isize) as libc::c_int
                    == ' ' as i32
            {
                let fresh6 = ru;
                ru = ru - 1;
                *ret_buffer.offset(fresh6 as isize) = '1' as i32 as u_char;
                *ret_buffer.offset(ru as isize) = '-' as i32 as u_char;
                break;
            } else {
                if ru >= j {
                    continue;
                }
                memmove(
                    &mut *ret_buffer.offset((j + 1 as libc::c_int) as isize)
                        as *mut u_char as *mut libc::c_void,
                    &mut *ret_buffer.offset(j as isize) as *mut u_char
                        as *const libc::c_void,
                    (len - j + 1 as libc::c_int) as libc::c_ulong,
                );
                *ret_buffer.offset(j as isize) = '1' as i32 as u_char;
                len += 1;
                len;
                break;
            }
        }
    }
    i = 0 as libc::c_int;
    while i < len {
        if *ret_buffer.offset(i as isize) as libc::c_int == '-' as i32 {
            j = i + 1 as libc::c_int;
            while j < len {
                if *ret_buffer.offset(j as isize) as libc::c_int != '0' as i32
                    && *ret_buffer.offset(j as isize) as libc::c_int != '.' as i32
                {
                    break;
                }
                j += 1;
                j;
            }
            if j == len {
                if i != 0 as libc::c_int || len == size {
                    *ret_buffer.offset(i as isize) = ' ' as i32 as u_char;
                    break;
                } else {
                    let fresh7 = len;
                    len = len - 1;
                    memmove(
                        &mut *ret_buffer.offset(0 as libc::c_int as isize) as *mut u_char
                            as *mut libc::c_void,
                        &mut *ret_buffer.offset(1 as libc::c_int as isize) as *mut u_char
                            as *const libc::c_void,
                        fresh7 as libc::c_ulong,
                    );
                    break;
                }
            }
        } else if *ret_buffer.offset(i as isize) as libc::c_int != ' ' as i32 {
            break;
        }
        i += 1;
        i;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn Dlength1(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
) -> libc::c_short {
    return uitocstring(ret_buffer, (*expr).len as u_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Dlength2(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
    mut delim: *mut cstring,
) -> libc::c_short {
    return itocstring(ret_buffer, Dlength2x(expr, delim)) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Dlength2x(
    mut expr: *mut cstring,
    mut delim: *mut cstring,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pce: libc::c_int = 1 as libc::c_int;
    if (*delim).len as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*expr).len as libc::c_int {
        j = 0 as libc::c_int;
        while j != (*delim).len as libc::c_int {
            if (*expr).buf[(i + j) as usize] as libc::c_int
                != (*delim).buf[j as usize] as libc::c_int
            {
                break;
            }
            if j + 1 as libc::c_int == (*delim).len as libc::c_int {
                pce += 1;
                pce;
                i = i + j;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return pce;
}
#[no_mangle]
pub unsafe extern "C" fn Dname1(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
) -> libc::c_short {
    return Dname2(ret_buffer, var, 63 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Dname2(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
    mut sub: libc::c_int,
) -> libc::c_short {
    if sub < 0 as libc::c_int {
        return -(39 as libc::c_int) as libc::c_short;
    }
    return UTIL_String_Mvar(var, ret_buffer, sub);
}
#[no_mangle]
pub unsafe extern "C" fn Dorder1(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
) -> libc::c_short {
    return Dorder2(ret_buffer, var, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Dorder2(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
    mut dir: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = -(1 as libc::c_int);
    let mut s: libc::c_short = 0;
    let mut realdir: libc::c_int = 0;
    if dir != 1 as libc::c_int && dir != -(1 as libc::c_int)
        && (dir != 2 as libc::c_int && (*systab).historic & 4 as libc::c_int != 0)
    {
        return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    realdir = dir;
    if dir == 2 as libc::c_int {
        realdir = 1 as libc::c_int;
    }
    if dir == -(1 as libc::c_int) {
        if (*var).key[((*var).slen as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int == '\0' as i32
            && (*var).key[((*var).slen as libc::c_int - 2 as libc::c_int) as usize]
                as libc::c_int == '\0' as i32
        {
            i = (*var).slen as libc::c_int - 2 as libc::c_int;
            (*var).key[i as usize] = -1i32 as u_char;
        }
    }
    if (*var).uci as libc::c_int == 255 as libc::c_int {
        s = ST_Order(var, ret_buffer, realdir);
    } else if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int == '$' as i32
    {
        s = SS_Order(var, ret_buffer, realdir);
    } else {
        memcpy(
            &mut (*partab.jobtab).last_ref as *mut mvar as *mut libc::c_void,
            var as *const libc::c_void,
            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                .wrapping_add((*var).slen as libc::c_ulong),
        );
        if i != -(1 as libc::c_int) {
            (*partab.jobtab).last_ref.key[i as usize] = '\0' as i32 as u_char;
        }
        s = DB_Order(var, ret_buffer, realdir);
    }
    if dir == 2 as libc::c_int && s as libc::c_int == 0 as libc::c_int {
        memcpy(
            ret_buffer as *mut libc::c_void,
            b"-1\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        );
        s = 2 as libc::c_int as libc::c_short;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn Dpiece2(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
    mut delim: *mut cstring,
) -> libc::c_int {
    return Dpiece4(ret_buffer, expr, delim, 1 as libc::c_int, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Dpiece3(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
    mut delim: *mut cstring,
    mut i1: libc::c_int,
) -> libc::c_int {
    return Dpiece4(ret_buffer, expr, delim, i1, i1);
}
#[no_mangle]
pub unsafe extern "C" fn Dpiece4(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
    mut delim: *mut cstring,
    mut i1: libc::c_int,
    mut i2: libc::c_int,
) -> libc::c_int {
    let mut beg: libc::c_int = 0 as libc::c_int;
    let mut end: libc::c_int = 0;
    let mut pce: libc::c_int = 1 as libc::c_int;
    let mut f: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if (*delim).len as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if i1 < 0 as libc::c_int {
        i1 = 0 as libc::c_int;
    }
    if i2 < 0 as libc::c_int {
        i2 = 0 as libc::c_int;
    }
    if i1 == 0 as libc::c_int && i2 == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if i1 > i2 {
        return 0 as libc::c_int;
    }
    end = 0 as libc::c_int;
    while end < (*expr).len as libc::c_int {
        if (*expr).buf[end as usize] as libc::c_int
            == (*delim).buf[0 as libc::c_int as usize] as libc::c_int
        {
            f = 1 as libc::c_int;
            j = 1 as libc::c_int;
            while j < (*delim).len as libc::c_int {
                if (*expr).buf[(end + j) as usize] as libc::c_int
                    != (*delim).buf[j as usize] as libc::c_int
                {
                    f = 0 as libc::c_int;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            if f == 1 as libc::c_int {
                if pce == i2 {
                    end -= 1;
                    end;
                    break;
                } else {
                    pce += 1;
                    pce;
                    end = end + (*delim).len as libc::c_int - 1 as libc::c_int;
                    if pce == i1 {
                        beg = end + 1 as libc::c_int;
                    }
                }
            }
        }
        end += 1;
        end;
    }
    if pce < i1 {
        return 0 as libc::c_int;
    }
    if end == (*expr).len as libc::c_int {
        end -= 1;
        end;
    }
    j = end - beg + 1 as libc::c_int;
    memmove(
        ret_buffer as *mut libc::c_void,
        &mut *((*expr).buf).as_mut_ptr().offset(beg as isize) as *mut u_char
            as *const libc::c_void,
        j as libc::c_ulong,
    );
    *ret_buffer.offset(j as isize) = '\0' as i32 as u_char;
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn Dquery2(
    mut ret_buffer: *mut u_char,
    mut var: *mut mvar,
    mut dir: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = -(1 as libc::c_int);
    if dir != 1 as libc::c_int && dir != -(1 as libc::c_int) {
        return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if dir == -(1 as libc::c_int) {
        if (*var).key[((*var).slen as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int == '\0' as i32
            && (*var).key[((*var).slen as libc::c_int - 2 as libc::c_int) as usize]
                as libc::c_int == '\0' as i32
        {
            i = (*var).slen as libc::c_int - 2 as libc::c_int;
            (*var).key[i as usize] = -1i32 as u_char;
        }
    }
    if (*var).uci as libc::c_int == 255 as libc::c_int {
        return ST_Query(var, ret_buffer, dir);
    }
    if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int == '$' as i32 {
        return -(38 as libc::c_int) as libc::c_short;
    }
    memcpy(
        &mut (*partab.jobtab).last_ref as *mut mvar as *mut libc::c_void,
        var as *const libc::c_void,
        (::core::mem::size_of::<var_u>() as libc::c_ulong)
            .wrapping_add(5 as libc::c_int as libc::c_ulong)
            .wrapping_add((*var).slen as libc::c_ulong),
    );
    if i != -(1 as libc::c_int) {
        (*partab.jobtab).last_ref.key[i as usize] = '\0' as i32 as u_char;
    }
    return DB_Query(var, ret_buffer, dir);
}
#[no_mangle]
pub unsafe extern "C" fn Drandom(
    mut ret_buffer: *mut u_char,
    mut seed: libc::c_int,
) -> libc::c_short {
    if seed < 1 as libc::c_int {
        return -(3 as libc::c_int) as libc::c_short;
    }
    seed = (random() % seed as libc::c_long) as libc::c_int;
    return itocstring(ret_buffer, seed) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Dreverse(
    mut ret_buffer: *mut u_char,
    mut expr: *mut cstring,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    i = (*expr).len as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let fresh8 = j;
        j = j + 1;
        *ret_buffer.offset(fresh8 as isize) = (*expr).buf[i as usize];
        i -= 1;
        i;
    }
    *ret_buffer.offset(j as isize) = '\0' as i32 as u_char;
    return (*expr).len as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Dstack1(
    mut ret_buffer: *mut u_char,
    mut level: libc::c_int,
) -> libc::c_short {
    return Dstack1x(
        ret_buffer,
        level,
        (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Dstack1x(
    mut ret_buffer: *mut u_char,
    mut level: libc::c_int,
    mut job: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if level < -(1 as libc::c_int) {
        return 0 as libc::c_int as libc::c_short;
    }
    i = (*((*systab).jobtab).offset(job as isize)).cur_do;
    if (*((*systab).jobtab).offset(job as isize)).error_frame as libc::c_int
        > (*((*systab).jobtab).offset(job as isize)).cur_do
    {
        i = (*((*systab).jobtab).offset(job as isize)).error_frame as libc::c_int;
    }
    if level > i {
        return 0 as libc::c_int as libc::c_short;
    }
    if level == -(1 as libc::c_int) {
        return itocstring(ret_buffer, i) as libc::c_short;
    }
    if level == 0 as libc::c_int {
        if (*((*systab).jobtab).offset(job as isize))
            .dostk[0 as libc::c_int as usize]
            .type_0 as libc::c_int == 2 as libc::c_int
        {
            return mcopy(
                b"JOB\0" as *const u8 as *const libc::c_char as *mut u_char,
                ret_buffer,
                3 as libc::c_int,
            ) as libc::c_short;
        }
        return mcopy(
            b"RUN\0" as *const u8 as *const libc::c_char as *mut u_char,
            ret_buffer,
            3 as libc::c_int,
        ) as libc::c_short;
    }
    if level == (*((*systab).jobtab).offset(job as isize)).error_frame as libc::c_int {
        level = 128 as libc::c_int - 1 as libc::c_int;
    }
    i = (*((*systab).jobtab).offset(job as isize)).dostk[level as usize].type_0
        as libc::c_int & 127 as libc::c_int;
    if i == 1 as libc::c_int {
        return mcopy(
            b"BREAK\0" as *const u8 as *const libc::c_char as *mut u_char,
            ret_buffer,
            5 as libc::c_int,
        ) as libc::c_short;
    }
    if i == 3 as libc::c_int {
        return mcopy(
            b"DO\0" as *const u8 as *const libc::c_char as *mut u_char,
            ret_buffer,
            2 as libc::c_int,
        ) as libc::c_short;
    }
    if i == 4 as libc::c_int {
        return mcopy(
            b"$$\0" as *const u8 as *const libc::c_char as *mut u_char,
            ret_buffer,
            2 as libc::c_int,
        ) as libc::c_short;
    }
    if i == 5 as libc::c_int {
        return mcopy(
            b"XECUTE\0" as *const u8 as *const libc::c_char as *mut u_char,
            ret_buffer,
            6 as libc::c_int,
        ) as libc::c_short;
    }
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Dstack2(
    mut ret_buffer: *mut u_char,
    mut level: libc::c_int,
    mut code: *mut cstring,
) -> libc::c_int {
    return Dstack2x(
        ret_buffer,
        level,
        code,
        (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Dstack2x(
    mut ret_buffer: *mut u_char,
    mut level: libc::c_int,
    mut code: *mut cstring,
    mut job: libc::c_int,
) -> libc::c_int {
    let mut arg2: libc::c_int = 0 as libc::c_int;
    let mut rounam: *mut var_u = 0 as *mut var_u;
    let mut line: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut p: *mut u_char = 0 as *mut u_char;
    let mut var: *mut mvar = 0 as *mut mvar;
    let mut temp: [u_char; 36] = [0; 36];
    let mut cptr: *mut cstring = 0 as *mut cstring;
    let mut s: libc::c_int = 0;
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if level < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    i = (*((*systab).jobtab).offset(job as isize)).cur_do;
    if (*((*systab).jobtab).offset(job as isize)).error_frame as libc::c_int
        > (*((*systab).jobtab).offset(job as isize)).cur_do
    {
        i = (*((*systab).jobtab).offset(job as isize)).error_frame as libc::c_int;
    }
    if level > i {
        return 0 as libc::c_int;
    }
    if strncasecmp(
        ((*code).buf).as_mut_ptr() as *const libc::c_char,
        b"ecode\0\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        arg2 = 1 as libc::c_int;
    } else if strncasecmp(
        ((*code).buf).as_mut_ptr() as *const libc::c_char,
        b"mcode\0\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        arg2 = 2 as libc::c_int;
    } else if strncasecmp(
        ((*code).buf).as_mut_ptr() as *const libc::c_char,
        b"place\0\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        arg2 = 3 as libc::c_int;
    } else {
        return -(50 as libc::c_int + 200 as libc::c_int)
    }
    if arg2 == 1 as libc::c_int {
        *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
        if job as libc::c_long
            != (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
        {
            return 0 as libc::c_int;
        }
        var = ret_buffer as *mut mvar;
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
        (*var).volset = 0 as libc::c_int as u_char;
        (*var).uci = 255 as libc::c_int as u_char;
        cptr = temp.as_mut_ptr() as *mut cstring;
        (*cptr).len = itocstring(((*cptr).buf).as_mut_ptr(), level);
        (*var)
            .slen = UTIL_Key_Build(
            cptr,
            &mut *((*var).key).as_mut_ptr().offset(0 as libc::c_int as isize),
        ) as u_char;
        s = ST_Get(var, ret_buffer);
        if s == -(6 as libc::c_int) {
            s = 0 as libc::c_int;
        }
        return s;
    }
    if level != 0
        && level == (*((*systab).jobtab).offset(job as isize)).error_frame as libc::c_int
    {
        level = 128 as libc::c_int - 1 as libc::c_int;
    }
    if ((*((*systab).jobtab).offset(job as isize)).dostk[level as usize].type_0
        as libc::c_int & 127 as libc::c_int == 5 as libc::c_int
        || (*((*systab).jobtab).offset(job as isize)).dostk[level as usize].type_0
            as libc::c_int & 127 as libc::c_int == 1 as libc::c_int
        || (*((*systab).jobtab).offset(job as isize)).dostk[level as usize].type_0
            as libc::c_int & 127 as libc::c_int == 2 as libc::c_int)
        && var_empty(
            (*((*systab).jobtab).offset(job as isize)).dostk[level as usize].rounam,
        ) != 0
    {
        if arg2 == 2 as libc::c_int {
            *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
            if (*((*systab).jobtab).offset(job as isize)).cur_do < level {
                return 0 as libc::c_int;
            }
            if job as libc::c_long
                != (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
            {
                return 0 as libc::c_int;
            }
            p = (*((*systab).jobtab).offset(job as isize)).dostk[level as usize].routine;
            if p.is_null() {
                return 0 as libc::c_int;
            }
            i = 0 as libc::c_int;
            loop {
                let ref mut fresh9 = *ret_buffer.offset(i as isize);
                *fresh9 = *p.offset(i as isize);
                if !(*fresh9 != 0) {
                    break;
                }
                i += 1;
                i;
            }
            return i;
        }
        return mcopy(
            b"XECUTE\0" as *const u8 as *const libc::c_char as *mut u_char,
            ret_buffer,
            6 as libc::c_int,
        );
    }
    rounam = &mut (*((*((*systab).jobtab).offset(job as isize)).dostk)
        .as_mut_ptr()
        .offset(level as isize))
        .rounam;
    line = (*((*systab).jobtab).offset(job as isize)).dostk[level as usize].line_num
        as libc::c_int;
    if arg2 == 2 as libc::c_int {
        var = ret_buffer as *mut mvar;
        let mut var_i_0: u_int = 0 as libc::c_int as u_int;
        while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            (*var).name.var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
            var_i_0 = var_i_0.wrapping_add(1);
            var_i_0;
        }
        memcpy(
            &mut *((*var).name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_void,
            b"$ROUTINE\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        (*var).volset = (*((*systab).jobtab).offset(job as isize)).rvol;
        (*var).uci = (*((*systab).jobtab).offset(job as isize)).ruci;
        if (*rounam).var_cu[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
            (*var).uci = 1 as libc::c_int as u_char;
        }
        cptr = temp.as_mut_ptr() as *mut cstring;
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if (*rounam).var_cu[i as usize] as libc::c_int == 0 as libc::c_int {
                break;
            }
            (*cptr).buf[i as usize] = (*rounam).var_cu[i as usize];
            i += 1;
            i;
        }
        (*cptr).buf[i as usize] = '\0' as i32 as u_char;
        (*cptr).len = i as u_short;
        s = UTIL_Key_Build(
            cptr,
            &mut *((*var).key).as_mut_ptr().offset(0 as libc::c_int as isize),
        ) as libc::c_int;
        if s < 0 as libc::c_int {
            return s;
        }
        (*var).slen = s as u_char;
        (*cptr).len = itocstring(((*cptr).buf).as_mut_ptr(), line);
        s = UTIL_Key_Build(
            cptr,
            &mut *((*var).key).as_mut_ptr().offset((*var).slen as isize),
        ) as libc::c_int;
        if s < 0 as libc::c_int {
            return s;
        }
        (*var)
            .slen = ((*var).slen as libc::c_int + s as u_char as libc::c_int) as u_char;
        s = Dget1(ret_buffer, var);
        if s < 0 as libc::c_int {
            s = 0 as libc::c_int;
        }
        *ret_buffer.offset(s as isize) = '\0' as i32 as u_char;
        return s;
    }
    i = 0 as libc::c_int;
    let fresh10 = i;
    i = i + 1;
    *ret_buffer.offset(fresh10 as isize) = '+' as i32 as u_char;
    i += itocstring(&mut *ret_buffer.offset(i as isize), line) as libc::c_int;
    let fresh11 = i;
    i = i + 1;
    *ret_buffer.offset(fresh11 as isize) = '^' as i32 as u_char;
    arg2 = 0 as libc::c_int;
    while arg2 < 32 as libc::c_int {
        let fresh12 = i;
        i = i + 1;
        let ref mut fresh13 = *ret_buffer.offset(fresh12 as isize);
        *fresh13 = (*rounam).var_cu[arg2 as usize];
        if *fresh13 as libc::c_int == 0 as libc::c_int {
            break;
        }
        arg2 += 1;
        arg2;
    }
    if *ret_buffer.offset((i - 1 as libc::c_int) as isize) as libc::c_int == '\0' as i32
    {
        i -= 1;
        i;
    }
    *ret_buffer.offset(i as isize) = '\0' as i32 as u_char;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn Dtext(
    mut ret_buffer: *mut u_char,
    mut str: *mut cstring,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut slen: u_char = 0;
    let mut s: libc::c_int = 0;
    let mut off: libc::c_int = 1 as libc::c_int;
    let mut rou: [u_char; 36] = [0; 36];
    let mut tag: [u_char; 36] = [0; 36];
    let mut cr: *mut cstring = 0 as *mut cstring;
    let mut ct: *mut cstring = 0 as *mut cstring;
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    ct = &mut *tag.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
        as *mut cstring;
    cr = &mut *rou.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
        as *mut cstring;
    (*ct).len = 0 as libc::c_int as u_short;
    (*cr).len = 0 as libc::c_int as u_short;
    if memcmp(
        ((*str).buf).as_mut_ptr() as *const libc::c_void,
        b"+0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if (*partab.jobtab)
                .dostk[(*partab.jobtab).cur_do as usize]
                .rounam
                .var_cu[i as usize] == 0
            {
                break;
            }
            *ret_buffer
                .offset(
                    i as isize,
                ) = (*partab.jobtab)
                .dostk[(*partab.jobtab).cur_do as usize]
                .rounam
                .var_cu[i as usize];
            i += 1;
            i;
        }
        *ret_buffer.offset(i as isize) = '\0' as i32 as u_char;
        return i;
    }
    if (*str).buf[i as usize] as libc::c_int != '+' as i32
        && (*str).buf[i as usize] as libc::c_int != '^' as i32
    {
        while j < 32 as libc::c_int {
            if i == 0 as libc::c_int
                && (*str).buf[i as usize] as libc::c_int == '%' as i32
            {
                let fresh14 = i;
                i = i + 1;
                let fresh15 = j;
                j = j + 1;
                (*ct).buf[fresh15 as usize] = (*str).buf[fresh14 as usize];
            } else {
                if isalnum((*str).buf[i as usize] as libc::c_int) == 0 as libc::c_int {
                    break;
                }
                let fresh16 = i;
                i = i + 1;
                let fresh17 = j;
                j = j + 1;
                (*ct).buf[fresh17 as usize] = (*str).buf[fresh16 as usize];
            }
        }
        (*ct).buf[j as usize] = '\0' as i32 as u_char;
        (*ct).len = j as u_short;
        off = 0 as libc::c_int;
        while (*str).buf[i as usize] as libc::c_int != '+' as i32
            && (*str).buf[i as usize] as libc::c_int != '^' as i32
            && (*str).buf[i as usize] as libc::c_int != '\0' as i32
        {
            i += 1;
            i;
        }
    }
    if (*str).buf[i as usize] as libc::c_int == '+' as i32 {
        off = 0 as libc::c_int;
        i += 1;
        i;
        while isdigit((*str).buf[i as usize] as libc::c_int) != 0 as libc::c_int {
            let fresh18 = i;
            i = i + 1;
            off = off * 10 as libc::c_int
                + ((*str).buf[fresh18 as usize] as libc::c_int - '0' as i32);
        }
    }
    if (*str).buf[i as usize] as libc::c_int != '^' as i32
        && (*str).buf[i as usize] as libc::c_int != '\0' as i32
    {
        return -(12 as libc::c_int + 200 as libc::c_int);
    }
    j = 0 as libc::c_int;
    if (*str).buf[i as usize] as libc::c_int == '^' as i32 {
        i += 1;
        i;
        while j < 32 as libc::c_int {
            if j == 0 as libc::c_int
                && (*str).buf[i as usize] as libc::c_int == '%' as i32
            {
                let fresh19 = i;
                i = i + 1;
                let fresh20 = j;
                j = j + 1;
                (*cr).buf[fresh20 as usize] = (*str).buf[fresh19 as usize];
            } else {
                if isalnum((*str).buf[i as usize] as libc::c_int) == 0 as libc::c_int {
                    break;
                }
                let fresh21 = i;
                i = i + 1;
                let fresh22 = j;
                j = j + 1;
                (*cr).buf[fresh22 as usize] = (*str).buf[fresh21 as usize];
            }
        }
        (*cr).buf[j as usize] = '\0' as i32 as u_char;
        (*cr).len = j as u_short;
    } else {
        j = 0 as libc::c_int;
        while j < 32 as libc::c_int {
            (*cr)
                .buf[j
                as usize] = (*partab.jobtab)
                .dostk[(*partab.jobtab).cur_do as usize]
                .rounam
                .var_cu[j as usize];
            if (*cr).buf[j as usize] as libc::c_int == '\0' as i32 {
                break;
            }
            j += 1;
            j;
        }
        (*cr).buf[j as usize] = '\0' as i32 as u_char;
        (*cr).len = j as u_short;
    }
    if (*cr).len as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        partab.src_var.name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    memcpy(
        &mut *(partab.src_var.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_void,
        b"$ROUTINE\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    partab.src_var.volset = (*partab.jobtab).rvol;
    partab.src_var.uci = (*partab.jobtab).ruci;
    if (*cr).buf[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
        partab.src_var.uci = 1 as libc::c_int as u_char;
    }
    partab.src_var.slen = 0 as libc::c_int as u_char;
    s = UTIL_Key_Build(
        cr,
        &mut *(partab.src_var.key).as_mut_ptr().offset(0 as libc::c_int as isize),
    ) as libc::c_int;
    if s < 0 as libc::c_int {
        return s;
    }
    slen = s as u_char;
    if (*ct).len as libc::c_int == 0 as libc::c_int {
        (*ct).len = itocstring(((*ct).buf).as_mut_ptr(), off);
        s = UTIL_Key_Build(
            ct,
            &mut *(partab.src_var.key).as_mut_ptr().offset(slen as isize),
        ) as libc::c_int;
        if s < 0 as libc::c_int {
            return s;
        }
        partab.src_var.slen = (s + slen as libc::c_int) as u_char;
        s = DB_Get(&mut partab.src_var, ret_buffer);
        if s < 0 as libc::c_int {
            *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
            s = 0 as libc::c_int;
        } else if off == 0 {
            return mcopy(((*cr).buf).as_mut_ptr(), ret_buffer, (*cr).len as libc::c_int)
        }
        return s;
    }
    j = 1 as libc::c_int;
    loop {
        (*cr).len = itocstring(((*cr).buf).as_mut_ptr(), j);
        s = UTIL_Key_Build(
            cr,
            &mut *(partab.src_var.key).as_mut_ptr().offset(slen as isize),
        ) as libc::c_int;
        if s < 0 as libc::c_int {
            return s;
        }
        partab.src_var.slen = (s + slen as libc::c_int) as u_char;
        s = DB_Get(&mut partab.src_var, ret_buffer);
        if s < 0 as libc::c_int {
            *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < (*ct).len as libc::c_int {
            if *ret_buffer.offset(i as isize) as libc::c_int
                != (*ct).buf[i as usize] as libc::c_int
            {
                break;
            }
            i += 1;
            i;
        }
        if !(i < (*ct).len as libc::c_int) {
            if !(*ret_buffer.offset(i as isize) as libc::c_int != ' ' as i32
                && *ret_buffer.offset(i as isize) as libc::c_int != '(' as i32
                && *ret_buffer.offset(i as isize) as libc::c_int != '\0' as i32)
            {
                if off == 0 as libc::c_int {
                    return s;
                }
                j = j + off;
                (*cr).len = itocstring(((*cr).buf).as_mut_ptr(), j);
                s = UTIL_Key_Build(
                    cr,
                    &mut *(partab.src_var.key).as_mut_ptr().offset(slen as isize),
                ) as libc::c_int;
                if s < 0 as libc::c_int {
                    return s;
                }
                partab.src_var.slen = (s + slen as libc::c_int) as u_char;
                s = DB_Get(&mut partab.src_var, ret_buffer);
                if s < 0 as libc::c_int {
                    *ret_buffer
                        .offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
                    s = 0 as libc::c_int;
                }
                return s;
            }
        }
        j += 1;
        j;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Dtranslate2(
    mut ret_buffer: *mut u_char,
    mut expr1: *mut cstring,
    mut expr2: *mut cstring,
) -> libc::c_int {
    let mut s: u_short = 0 as libc::c_int as u_short;
    return Dtranslate3(ret_buffer, expr1, expr2, &mut s as *mut u_short as *mut cstring);
}
#[no_mangle]
pub unsafe extern "C" fn Dtranslate3(
    mut ret_buffer: *mut u_char,
    mut expr1: *mut cstring,
    mut expr2: *mut cstring,
    mut expr3: *mut cstring,
) -> libc::c_int {
    let mut i1: u_int = 0;
    let mut i2: u_int = 0;
    let mut p: libc::c_int = 0 as libc::c_int;
    i1 = 0 as libc::c_int as u_int;
    while i1 != (*expr1).len as u_int {
        let mut found: libc::c_short = 0 as libc::c_int as libc::c_short;
        i2 = 0 as libc::c_int as u_int;
        while i2 != (*expr2).len as u_int {
            if (*expr1).buf[i1 as usize] as libc::c_int
                == (*expr2).buf[i2 as usize] as libc::c_int
            {
                found = 1 as libc::c_int as libc::c_short;
                if i2 < (*expr3).len as u_int {
                    let fresh23 = p;
                    p = p + 1;
                    *ret_buffer.offset(fresh23 as isize) = (*expr3).buf[i2 as usize];
                }
                break;
            } else {
                i2 = i2.wrapping_add(1);
                i2;
            }
        }
        if found == 0 {
            let fresh24 = p;
            p = p + 1;
            *ret_buffer.offset(fresh24 as isize) = (*expr1).buf[i1 as usize];
        }
        i1 = i1.wrapping_add(1);
        i1;
    }
    *ret_buffer.offset(p as isize) = '\0' as i32 as u_char;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn Dview(
    mut ret_buffer: *mut u_char,
    mut chan: libc::c_int,
    mut loc: libc::c_int,
    mut size: libc::c_int,
    mut value: *mut cstring,
) -> libc::c_int {
    let mut vb: *mut u_char = 0 as *mut u_char;
    if chan > -(1 as libc::c_int) || chan < -(1 as libc::c_int) {
        return -(63 as libc::c_int + 200 as libc::c_int);
    }
    chan = -chan - 1 as libc::c_int;
    if (*((*partab.jobtab).view).as_mut_ptr().offset(chan as isize)).is_null() {
        return -(63 as libc::c_int + 200 as libc::c_int);
    }
    vb = (**((*partab.jobtab).view).as_mut_ptr().offset(chan as isize)).mem
        as *mut u_char;
    if loc < 0 as libc::c_int || size < 1 as libc::c_int
        || loc + size
            > (*(*(*systab).vol[chan as usize]).vollab).block_size as libc::c_int
    {
        return -(63 as libc::c_int + 200 as libc::c_int);
    }
    vb = vb.offset(loc as isize);
    if value.is_null() {
        if size == 1 as libc::c_int {
            return uitocstring(ret_buffer, *vb as u_int) as libc::c_int;
        }
        if size == 2 as libc::c_int {
            return uitocstring(ret_buffer, *(vb as *mut u_short) as u_int)
                as libc::c_int;
        }
        if size == 4 as libc::c_int {
            return uitocstring(ret_buffer, *(vb as *mut u_int)) as libc::c_int;
        }
        return mcopy(vb, ret_buffer, size);
    }
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if size == 1 as libc::c_int || size == 2 as libc::c_int || size == 4 as libc::c_int {
        let mut i: libc::c_int = cstringtoi(value);
        if size == 1 as libc::c_int {
            *vb = i as u_char;
        } else if size == 2 as libc::c_int {
            *(vb as *mut u_short) = i as u_short;
        } else {
            *(vb as *mut u_int) = i as u_int;
        }
    } else {
        if size != (*value).len as libc::c_int {
            return -(63 as libc::c_int + 200 as libc::c_int);
        }
        memcpy(
            vb as *mut libc::c_void,
            ((*value).buf).as_mut_ptr() as *const libc::c_void,
            size as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DSetextract(
    mut tmp: *mut u_char,
    mut cptr: *mut cstring,
    mut var: *mut mvar,
    mut i1: libc::c_int,
    mut i2: libc::c_int,
) -> libc::c_int {
    let mut vptr: *mut cstring = 0 as *mut cstring;
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if i1 < 1 as libc::c_int {
        i1 = 1 as libc::c_int;
    }
    if i1 > i2 {
        return 0 as libc::c_int;
    }
    if i2 > 65534 as libc::c_int {
        return -(75 as libc::c_int);
    }
    vptr = tmp as *mut cstring;
    s = Dget1(((*vptr).buf).as_mut_ptr(), var);
    if s < 0 as libc::c_int {
        return s;
    }
    (*vptr).len = s as u_short;
    i = s;
    while i < i1 {
        let fresh25 = i;
        i = i + 1;
        (*vptr).buf[fresh25 as usize] = ' ' as i32 as u_char;
    }
    if s <= i2 {
        s = mcopy(
            ((*cptr).buf).as_mut_ptr(),
            &mut *((*vptr).buf).as_mut_ptr().offset((i1 - 1 as libc::c_int) as isize),
            (*cptr).len as libc::c_int,
        );
        if s < 0 as libc::c_int {
            return s;
        }
        (*vptr).len = (i1 - 1 as libc::c_int + (*cptr).len as libc::c_int) as u_short;
        if (*var).uci as libc::c_int == 255 as libc::c_int {
            return ST_Set(var, vptr);
        }
        return DB_Set(var, vptr);
    }
    if i2 - i1 + 1 as libc::c_int != (*cptr).len as libc::c_int {
        s = mcopy(
            &mut *((*vptr).buf).as_mut_ptr().offset(i2 as isize),
            &mut *((*vptr).buf)
                .as_mut_ptr()
                .offset((i1 - 1 as libc::c_int + (*cptr).len as libc::c_int) as isize),
            (*vptr).len as libc::c_int - i2 + 2 as libc::c_int,
        );
        if s < 0 as libc::c_int {
            return s;
        }
    }
    memmove(
        &mut *((*vptr).buf).as_mut_ptr().offset((i1 - 1 as libc::c_int) as isize)
            as *mut u_char as *mut libc::c_void,
        ((*cptr).buf).as_mut_ptr() as *const libc::c_void,
        (*cptr).len as libc::c_ulong,
    );
    (*vptr)
        .len = ((*vptr).len as libc::c_int - (i2 - i1 + 1 as libc::c_int)
        + (*cptr).len as libc::c_int) as u_short;
    if (*var).uci as libc::c_int == 255 as libc::c_int {
        return ST_Set(var, vptr);
    }
    return DB_Set(var, vptr);
}
#[no_mangle]
pub unsafe extern "C" fn DSetpiece(
    mut tmp: *mut u_char,
    mut cptr: *mut cstring,
    mut var: *mut mvar,
    mut dptr: *mut cstring,
    mut i1: libc::c_int,
    mut i2: libc::c_int,
) -> libc::c_int {
    let mut vptr: *mut cstring = 0 as *mut cstring;
    let mut s: libc::c_int = 0;
    let mut beg: libc::c_int = 0 as libc::c_int;
    let mut end: libc::c_int = 0;
    let mut pce: libc::c_int = 1 as libc::c_int;
    let mut f: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut np: libc::c_int = 0;
    if i1 < 1 as libc::c_int {
        i1 = 1 as libc::c_int;
    }
    if i1 > i2 {
        return 0 as libc::c_int;
    }
    vptr = tmp as *mut cstring;
    s = Dget1(((*vptr).buf).as_mut_ptr(), var);
    if s < 0 as libc::c_int {
        return s;
    }
    (*vptr).len = s as u_short;
    if (*dptr).len as libc::c_int == 0 as libc::c_int {
        s = mcopy(
            ((*cptr).buf).as_mut_ptr(),
            &mut *((*vptr).buf).as_mut_ptr().offset((*vptr).len as isize),
            (*cptr).len as libc::c_int,
        );
        if s < 0 as libc::c_int {
            return s;
        }
        (*vptr)
            .len = ((*vptr).len as libc::c_int + (*cptr).len as libc::c_int) as u_short;
        if (*var).uci as libc::c_int == 255 as libc::c_int {
            return ST_Set(var, vptr);
        }
        return DB_Set(var, vptr);
    }
    np = Dlength2x(vptr, dptr);
    if np < i1 {
        f = i1 - np;
        j = 0 as libc::c_int;
        while j < f {
            s = mcopy(
                ((*dptr).buf).as_mut_ptr(),
                &mut *((*vptr).buf).as_mut_ptr().offset((*vptr).len as isize),
                (*dptr).len as libc::c_int,
            );
            if s < 0 as libc::c_int {
                return s;
            }
            if (*vptr).len as libc::c_int + s > 65534 as libc::c_int {
                return -(75 as libc::c_int);
            }
            (*vptr).len = ((*vptr).len as libc::c_int + s) as u_short;
            j += 1;
            j;
        }
        s = mcopy(
            ((*cptr).buf).as_mut_ptr(),
            &mut *((*vptr).buf).as_mut_ptr().offset((*vptr).len as isize),
            (*cptr).len as libc::c_int,
        );
        if (*vptr).len as libc::c_int + s > 65534 as libc::c_int {
            return -(75 as libc::c_int);
        }
        (*vptr).len = ((*vptr).len as libc::c_int + s) as u_short;
        if (*var).uci as libc::c_int == 255 as libc::c_int {
            return ST_Set(var, vptr);
        }
        return DB_Set(var, vptr);
    }
    end = 0 as libc::c_int;
    while end < (*vptr).len as libc::c_int {
        if (*vptr).buf[end as usize] as libc::c_int
            == (*dptr).buf[0 as libc::c_int as usize] as libc::c_int
        {
            f = 1 as libc::c_int;
            j = 1 as libc::c_int;
            while j < (*dptr).len as libc::c_int {
                if (*vptr).buf[(end + j) as usize] as libc::c_int
                    != (*dptr).buf[j as usize] as libc::c_int
                {
                    f = 0 as libc::c_int;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            if f == 1 as libc::c_int {
                if pce == i2 {
                    end -= 1;
                    end;
                    break;
                } else {
                    pce += 1;
                    pce;
                    end = end + (*dptr).len as libc::c_int - 1 as libc::c_int;
                    if pce == i1 {
                        beg = end + 1 as libc::c_int;
                    }
                }
            }
        }
        end += 1;
        end;
    }
    if np == i1 {
        s = mcopy(
            ((*cptr).buf).as_mut_ptr(),
            &mut *((*vptr).buf).as_mut_ptr().offset(beg as isize),
            (*cptr).len as libc::c_int,
        );
        if s < 0 as libc::c_int {
            return s;
        }
        (*vptr).len = (beg + (*cptr).len as libc::c_int) as u_short;
        if (*var).uci as libc::c_int == 255 as libc::c_int {
            return ST_Set(var, vptr);
        }
        return DB_Set(var, vptr);
    }
    if end >= (*vptr).len as libc::c_int {
        end = (*vptr).len as libc::c_int - 1 as libc::c_int;
    }
    i1 = beg;
    i2 = end;
    if i2 - i1 + 1 as libc::c_int != (*cptr).len as libc::c_int {
        s = mcopy(
            &mut *((*vptr).buf).as_mut_ptr().offset((i2 + 1 as libc::c_int) as isize),
            &mut *((*vptr).buf)
                .as_mut_ptr()
                .offset((i1 + (*cptr).len as libc::c_int) as isize),
            (*vptr).len as libc::c_int - i2 + 2 as libc::c_int,
        );
        if s < 0 as libc::c_int {
            return s;
        }
    }
    if (*cptr).len != 0 {
        memmove(
            &mut *((*vptr).buf).as_mut_ptr().offset(i1 as isize) as *mut u_char
                as *mut libc::c_void,
            ((*cptr).buf).as_mut_ptr() as *const libc::c_void,
            (*cptr).len as libc::c_ulong,
        );
    }
    (*vptr)
        .len = ((*vptr).len as libc::c_int - (i2 - i1 + 1 as libc::c_int)
        + (*cptr).len as libc::c_int) as u_short;
    if (*var).uci as libc::c_int == 255 as libc::c_int {
        return ST_Set(var, vptr);
    }
    return DB_Set(var, vptr);
}
