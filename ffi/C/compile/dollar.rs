#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type GBD;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut systab: *mut systab_struct;
    fn eval();
    fn routine(runtime: libc::c_int) -> libc::c_short;
    static mut source_ptr: *mut u_char;
    static mut comp_ptr: *mut u_char;
    fn localvar() -> libc::c_short;
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
pub unsafe extern "C" fn isalpha(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x100 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x400 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn toupper(mut _c: libc::c_int) -> libc::c_int {
    return __toupper(_c);
}
#[no_mangle]
pub unsafe extern "C" fn dodollar() {
    let mut len: libc::c_int = 0;
    let mut s: libc::c_short = 0;
    let mut us: u_short = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut sel: libc::c_int = 0;
    let mut args: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut p: *mut u_char = 0 as *mut u_char;
    let mut selj: [*mut u_char; 256] = [0 as *mut u_char; 256];
    let mut name: [libc::c_char; 20] = [0; 20];
    let mut c: libc::c_char = 0;
    let mut save: [u_char; 1024] = [0; 1024];
    let mut savecount: libc::c_int = 0;
    let mut errm4: libc::c_short = -(4 as libc::c_int) as libc::c_short;
    let fresh0 = source_ptr;
    source_ptr = source_ptr.offset(1);
    c = toupper(*fresh0 as libc::c_int) as libc::c_char;
    if c as libc::c_int == '$' as i32 {
        ptr = comp_ptr;
        let fresh1 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh1 = 141 as libc::c_int as u_char;
        i = routine(-(1 as libc::c_int)) as libc::c_int;
        if i > -(1 as libc::c_int) || i == -(4 as libc::c_int) {
            comp_ptr = ptr;
            comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
            return;
        }
        if i < -(4 as libc::c_int) {
            comperror(i as libc::c_short);
            return;
        }
        args = 129 as libc::c_int;
        if i == -(2 as libc::c_int) {
            *ptr = 143 as libc::c_int as u_char;
        } else if i == -(3 as libc::c_int) {
            *ptr = 142 as libc::c_int as u_char;
        }
        if *source_ptr as libc::c_int == '(' as i32 {
            args -= 1;
            args;
            savecount = comp_ptr.offset_from(ptr) as libc::c_long as libc::c_int;
            memcpy(
                save.as_mut_ptr() as *mut libc::c_void,
                ptr as *const libc::c_void,
                savecount as libc::c_ulong,
            );
            comp_ptr = ptr;
            source_ptr = source_ptr.offset(1);
            source_ptr;
            loop {
                if args > 127 as libc::c_int - 1 as libc::c_int | 128 as libc::c_int {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                args += 1;
                args;
                if *source_ptr as libc::c_int == ')' as i32 {
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                    break;
                } else {
                    if *source_ptr as libc::c_int == ',' as i32
                        || *source_ptr as libc::c_int == ')' as i32
                    {
                        let fresh2 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh2 = 169 as libc::c_int as u_char;
                    } else if *source_ptr as libc::c_int == '.' as i32
                        && isdigit(
                            *source_ptr.offset(1 as libc::c_int as isize) as libc::c_int,
                        ) == 0 as libc::c_int
                    {
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                        if *source_ptr as libc::c_int == '@' as i32 {
                            source_ptr = source_ptr.offset(1);
                            source_ptr;
                            atom();
                            let fresh3 = comp_ptr;
                            comp_ptr = comp_ptr.offset(1);
                            *fresh3 = 66 as libc::c_int as u_char;
                        } else {
                            p = comp_ptr;
                            s = localvar();
                            if (s as libc::c_int) < 0 as libc::c_int {
                                comperror(s);
                                return;
                            }
                            p = p.offset(s as libc::c_int as isize);
                            *p = 62 as libc::c_int as u_char;
                        }
                        let fresh4 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh4 = 168 as libc::c_int as u_char;
                    } else {
                        eval();
                    }
                    if *source_ptr as libc::c_int == ')' as i32 {
                        continue;
                    }
                    if *source_ptr as libc::c_int == ',' as i32 {
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                    } else {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
            }
            memcpy(
                comp_ptr as *mut libc::c_void,
                save.as_mut_ptr() as *const libc::c_void,
                savecount as libc::c_ulong,
            );
            comp_ptr = comp_ptr.offset(savecount as isize);
        }
        let fresh5 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh5 = args as u_char;
        return;
    }
    if c as libc::c_int == '&' as i32 {
        let fresh6 = source_ptr;
        source_ptr = source_ptr.offset(1);
        c = toupper(*fresh6 as libc::c_int) as libc::c_char;
        if c as libc::c_int == '%' as i32 {
            let fresh7 = i;
            i = i + 1;
            name[fresh7 as usize] = c;
            let fresh8 = source_ptr;
            source_ptr = source_ptr.offset(1);
            c = toupper(*fresh8 as libc::c_int) as libc::c_char;
        }
        while isalpha(c as libc::c_int) != 0 as libc::c_int {
            let fresh9 = i;
            i = i + 1;
            name[fresh9 as usize] = c;
            let fresh10 = source_ptr;
            source_ptr = source_ptr.offset(1);
            c = toupper(*fresh10 as libc::c_int) as libc::c_char;
        }
        name[i as usize] = '\0' as i32 as libc::c_char;
        if c as libc::c_int == '(' as i32 {
            loop {
                eval();
                args += 1;
                args;
                let fresh11 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh11 as libc::c_char;
                if c as libc::c_int == ')' as i32 {
                    break;
                }
                if c as libc::c_int != ',' as i32 {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
            }
        } else {
            source_ptr = source_ptr.offset(-1);
            source_ptr;
        }
        if args > 2 as libc::c_int {
            comperror(-(18 as libc::c_int + 200 as libc::c_int) as libc::c_short);
            return;
        }
        i = args;
        while i < 2 as libc::c_int {
            let fresh12 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh12 = 60 as libc::c_int as u_char;
            let fresh13 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh13 = 0 as libc::c_int as u_char;
            let fresh14 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh14 = 0 as libc::c_int as u_char;
            let fresh15 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh15 = '\0' as i32 as u_char;
            i += 1;
            i;
        }
        if strcmp(name.as_mut_ptr(), b"%DIRECTORY\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let fresh16 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh16 = 240 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%HOST\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh17 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh17 = 237 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%FILE\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh18 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh18 = 238 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%ERRMSG\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh19 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh19 = 241 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%OPCOM\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh20 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh20 = 242 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%SIGNAL\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh21 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh21 = 236 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%SPAWN\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh22 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh22 = 243 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%VERSION\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh23 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh23 = 244 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%ZWRITE\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh24 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh24 = 245 as libc::c_int as u_char;
        } else if strcmp(name.as_mut_ptr(), b"E\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let fresh25 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh25 = 246 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"PASCHK\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh26 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh26 = 247 as libc::c_int as u_char;
        } else if strcmp(name.as_mut_ptr(), b"V\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let fresh27 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh27 = 248 as libc::c_int as u_char;
        } else if strcmp(name.as_mut_ptr(), b"X\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let fresh28 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh28 = 249 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"XRSM\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh29 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh29 = 250 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%SETENV\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh30 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh30 = 251 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%GETENV\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh31 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh31 = 252 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%ROUCHK\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh32 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh32 = 253 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%FORK\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh33 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh33 = 254 as libc::c_int as u_char;
        } else if strcmp(name.as_mut_ptr(), b"%IC\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let fresh34 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh34 = 255 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%WAIT\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh35 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh35 = 234 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"DEBUG\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh36 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh36 = 239 as libc::c_int as u_char;
        } else if strcmp(
            name.as_mut_ptr(),
            b"%COMPRESS\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh37 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh37 = 235 as libc::c_int as u_char;
        } else {
            comperror(-(18 as libc::c_int + 200 as libc::c_int) as libc::c_short);
        }
        return;
    }
    name[0 as libc::c_int as usize] = c;
    len = 0 as libc::c_int;
    while isalpha(*source_ptr.offset(len as isize) as libc::c_int) != 0 as libc::c_int {
        name[(len + 1 as libc::c_int)
            as usize] = *source_ptr.offset(len as isize) as libc::c_char;
        len += 1;
        len;
    }
    source_ptr = source_ptr.offset(len as isize);
    len += 1;
    len;
    name[len as usize] = '\0' as i32 as libc::c_char;
    if *source_ptr as libc::c_int == '(' as i32 {
        source_ptr = source_ptr.offset(1);
        source_ptr;
        ptr = comp_ptr;
        sel = (name[0 as libc::c_int as usize] as libc::c_int == 'S' as i32
            && toupper(name[1 as libc::c_int as usize] as libc::c_int) != 'T' as i32)
            as libc::c_int;
        if name[0 as libc::c_int as usize] as libc::c_int == 'D' as i32
            || name[0 as libc::c_int as usize] as libc::c_int == 'G' as i32
            || name[0 as libc::c_int as usize] as libc::c_int == 'I' as i32
            || name[0 as libc::c_int as usize] as libc::c_int == 'N' as i32
            || name[0 as libc::c_int as usize] as libc::c_int == 'O' as i32
            || name[0 as libc::c_int as usize] as libc::c_int == 'Q' as i32
                && toupper(name[1 as libc::c_int as usize] as libc::c_int) != 'S' as i32
                && toupper(name[1 as libc::c_int as usize] as libc::c_int) != 'L' as i32
        {
            if *source_ptr as libc::c_int == '@' as i32 {
                atom();
                ptr = comp_ptr.offset(-(1 as libc::c_int as isize));
                if *ptr as libc::c_int == 65 as libc::c_int {
                    if name[0 as libc::c_int as usize] as libc::c_int == 'N' as i32
                        || name[0 as libc::c_int as usize] as libc::c_int == 'O' as i32
                        || name[0 as libc::c_int as usize] as libc::c_int == 'Q' as i32
                    {
                        *ptr = 67 as libc::c_int as u_char;
                    } else {
                        *ptr = 66 as libc::c_int as u_char;
                    }
                } else {
                    ptr = ptr.offset(-(2 as libc::c_int as isize));
                    if *ptr as libc::c_int == 61 as libc::c_int {
                        if name[0 as libc::c_int as usize] as libc::c_int == 'N' as i32
                            || name[0 as libc::c_int as usize] as libc::c_int
                                == 'O' as i32
                            || name[0 as libc::c_int as usize] as libc::c_int
                                == 'Q' as i32
                        {
                            *ptr = 63 as libc::c_int as u_char;
                        } else {
                            *ptr = 62 as libc::c_int as u_char;
                        }
                    }
                }
            } else {
                s = localvar();
                if (s as libc::c_int) < 0 as libc::c_int {
                    comperror(s);
                    return;
                }
                ptr = &mut *ptr.offset(s as isize) as *mut u_char;
                if name[0 as libc::c_int as usize] as libc::c_int == 'N' as i32
                    || name[0 as libc::c_int as usize] as libc::c_int == 'O' as i32
                    || name[0 as libc::c_int as usize] as libc::c_int == 'Q' as i32
                {
                    *ptr = 63 as libc::c_int as u_char;
                } else {
                    *ptr = 62 as libc::c_int as u_char;
                }
            }
        } else if name[0 as libc::c_int as usize] as libc::c_int == 'T' as i32
            && toupper(name[1 as libc::c_int as usize] as libc::c_int) != 'R' as i32
        {
            i = routine(-(2 as libc::c_int)) as libc::c_int;
            if i < -(4 as libc::c_int) {
                comperror(i as libc::c_short);
                return;
            }
        } else {
            eval();
        }
        loop {
            args += 1;
            args;
            if args > 255 as libc::c_int {
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            let fresh55 = source_ptr;
            source_ptr = source_ptr.offset(1);
            c = *fresh55 as libc::c_char;
            if c as libc::c_int == ')' as i32 {
                break;
            }
            if sel != 0 {
                if c as libc::c_int
                    != (if args & 1 as libc::c_int != 0 {
                        ':' as i32
                    } else {
                        ',' as i32
                    })
                {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                let fresh56 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh56 = (if args & 1 as libc::c_int != 0 {
                    5 as libc::c_int
                } else {
                    172 as libc::c_int
                }) as u_char;
                selj[args as usize] = comp_ptr;
                comp_ptr = comp_ptr
                    .offset(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                    );
            } else if c as libc::c_int != ',' as i32 {
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            eval();
        }
        match name[0 as libc::c_int as usize] as libc::c_int {
            65 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"ascii\0\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 1 as libc::c_int {
                    let fresh57 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh57 = 100 as libc::c_int as u_char;
                    return;
                }
                if args == 2 as libc::c_int {
                    let fresh58 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh58 = 101 as libc::c_int as u_char;
                    return;
                }
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            67 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"char\0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args > 255 as libc::c_int {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                let fresh59 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh59 = 102 as libc::c_int as u_char;
                let fresh60 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh60 = args as u_char;
                return;
            }
            68 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"data\0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args > 1 as libc::c_int {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                let fresh61 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh61 = 103 as libc::c_int as u_char;
                return;
            }
            69 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"extract\0\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 1 as libc::c_int {
                    let fresh62 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh62 = 104 as libc::c_int as u_char;
                    return;
                }
                if args == 2 as libc::c_int {
                    let fresh63 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh63 = 105 as libc::c_int as u_char;
                    return;
                }
                if args == 3 as libc::c_int {
                    let fresh64 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh64 = 106 as libc::c_int as u_char;
                    return;
                }
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            70 => {
                if len == 1 as libc::c_int
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"find\0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 2 as libc::c_int {
                        let fresh65 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh65 = 107 as libc::c_int as u_char;
                        return;
                    }
                    if args == 3 as libc::c_int {
                        let fresh66 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh66 = 108 as libc::c_int as u_char;
                        return;
                    }
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if len == 2 as libc::c_int
                    && toupper(name[1 as libc::c_int as usize] as libc::c_int)
                        == 'N' as i32
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"fnumber\0\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 2 as libc::c_int {
                        let fresh67 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh67 = 109 as libc::c_int as u_char;
                        return;
                    }
                    if args == 3 as libc::c_int {
                        let fresh68 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh68 = 110 as libc::c_int as u_char;
                        return;
                    }
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            71 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"get\0\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 1 as libc::c_int {
                    let fresh69 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh69 = 111 as libc::c_int as u_char;
                } else if args == 2 as libc::c_int {
                    let fresh70 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh70 = 112 as libc::c_int as u_char;
                } else {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                return;
            }
            73 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"increment\0\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 1 as libc::c_int {
                    let fresh71 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh71 = 98 as libc::c_int as u_char;
                } else if args == 2 as libc::c_int {
                    let fresh72 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh72 = 99 as libc::c_int as u_char;
                } else {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                return;
            }
            74 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"justify\0\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 2 as libc::c_int {
                    let fresh73 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh73 = 113 as libc::c_int as u_char;
                } else if args == 3 as libc::c_int {
                    let fresh74 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh74 = 114 as libc::c_int as u_char;
                } else {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                return;
            }
            76 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"length\0\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 1 as libc::c_int {
                    let fresh75 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh75 = 115 as libc::c_int as u_char;
                } else if args == 2 as libc::c_int {
                    let fresh76 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh76 = 116 as libc::c_int as u_char;
                } else {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                return;
            }
            78 => {
                if toupper(name[1 as libc::c_int as usize] as libc::c_int) != 'A' as i32
                {
                    if len > 1 as libc::c_int {
                        if strncasecmp(
                            name.as_mut_ptr(),
                            b"next\0\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int as libc::c_ulong,
                        ) != 0 as libc::c_int
                        {
                            comperror(
                                -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                            );
                            return;
                        }
                    }
                    if (*systab).historic & 4 as libc::c_int == 0 {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    if args != 1 as libc::c_int {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    let fresh77 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh77 = 60 as libc::c_int as u_char;
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
                    let fresh78 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh78 = '2' as i32 as u_char;
                    let fresh79 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh79 = '\0' as i32 as u_char;
                    let fresh80 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh80 = 120 as libc::c_int as u_char;
                    return;
                }
                if len > 2 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"name\0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 1 as libc::c_int {
                    let fresh81 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh81 = 117 as libc::c_int as u_char;
                } else if args == 2 as libc::c_int {
                    let fresh82 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh82 = 118 as libc::c_int as u_char;
                } else {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                return;
            }
            79 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"order\0\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 1 as libc::c_int {
                    let fresh83 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh83 = 119 as libc::c_int as u_char;
                } else if args == 2 as libc::c_int {
                    let fresh84 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh84 = 120 as libc::c_int as u_char;
                } else {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                return;
            }
            80 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"piece\0\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 2 as libc::c_int {
                    let fresh85 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh85 = 121 as libc::c_int as u_char;
                } else if args == 3 as libc::c_int {
                    let fresh86 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh86 = 122 as libc::c_int as u_char;
                } else if args == 4 as libc::c_int {
                    let fresh87 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh87 = 123 as libc::c_int as u_char;
                } else {
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                return;
            }
            81 => {
                if len == 1 as libc::c_int
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"query\0\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 1 as libc::c_int {
                        let fresh88 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh88 = 126 as libc::c_int as u_char;
                    } else if args == 2 as libc::c_int {
                        let fresh89 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh89 = 127 as libc::c_int as u_char;
                    } else {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    return;
                }
                if len == 2 as libc::c_int
                    && toupper(name[1 as libc::c_int as usize] as libc::c_int)
                        == 'L' as i32
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"qlength\0\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 1 as libc::c_int {
                        let fresh90 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh90 = 124 as libc::c_int as u_char;
                        return;
                    }
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if len == 2 as libc::c_int
                    && toupper(name[1 as libc::c_int as usize] as libc::c_int)
                        == 'S' as i32
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"qsubscript\0\0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 2 as libc::c_int {
                        let fresh91 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh91 = 125 as libc::c_int as u_char;
                        return;
                    }
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            82 => {
                if len == 1 as libc::c_int
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"random\0\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 1 as libc::c_int {
                        let fresh92 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh92 = 128 as libc::c_int as u_char;
                        return;
                    }
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if len == 2 as libc::c_int
                    && toupper(name[1 as libc::c_int as usize] as libc::c_int)
                        == 'E' as i32
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"reverse\0\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 1 as libc::c_int {
                        let fresh93 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh93 = 129 as libc::c_int as u_char;
                        return;
                    }
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            83 => {
                if len == 1 as libc::c_int
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"select\0\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args & 1 as libc::c_int != 0 {
                        comp_ptr = ptr;
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    let fresh94 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh94 = 172 as libc::c_int as u_char;
                    selj[args as usize] = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    selj[(args + 1 as libc::c_int) as usize] = comp_ptr;
                    let fresh95 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh95 = 2 as libc::c_int as u_char;
                    memcpy(
                        comp_ptr as *mut libc::c_void,
                        &mut errm4 as *mut libc::c_short as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    );
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    i = 1 as libc::c_int;
                    while i <= args {
                        if i & 1 as libc::c_int != 0 {
                            *(selj[i as usize]
                                as *mut libc::c_short) = (selj[(i + 1 as libc::c_int)
                                as usize])
                                .offset_from(selj[i as usize]) as libc::c_long
                                as libc::c_short;
                        } else {
                            *(selj[i as usize]
                                as *mut libc::c_short) = (comp_ptr
                                .offset_from(selj[i as usize]) as libc::c_long
                                as libc::c_short as libc::c_ulong)
                                .wrapping_sub(
                                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                                ) as libc::c_short;
                        }
                        i += 1;
                        i;
                    }
                    return;
                }
                if len == 2 as libc::c_int
                    && toupper(name[1 as libc::c_int as usize] as libc::c_int)
                        == 'T' as i32
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"stack\0\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 1 as libc::c_int {
                        let fresh96 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh96 = 130 as libc::c_int as u_char;
                        return;
                    }
                    if args == 2 as libc::c_int {
                        let fresh97 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh97 = 131 as libc::c_int as u_char;
                        return;
                    }
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            84 => {
                if len == 1 as libc::c_int
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"text\0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 1 as libc::c_int {
                        let fresh98 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh98 = 132 as libc::c_int as u_char;
                        return;
                    }
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if len == 2 as libc::c_int
                    && toupper(name[1 as libc::c_int as usize] as libc::c_int)
                        == 'R' as i32
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"translate\0\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if args == 2 as libc::c_int {
                        let fresh99 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh99 = 133 as libc::c_int as u_char;
                        return;
                    }
                    if args == 3 as libc::c_int {
                        let fresh100 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh100 = 134 as libc::c_int as u_char;
                        return;
                    }
                    comperror(
                        -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            86 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"view\0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
                if args == 2 as libc::c_int {
                    let fresh101 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh101 = 135 as libc::c_int as u_char;
                    return;
                }
                if args == 3 as libc::c_int {
                    let fresh102 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh102 = 136 as libc::c_int as u_char;
                    return;
                }
                if args == 4 as libc::c_int {
                    let fresh103 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh103 = 137 as libc::c_int as u_char;
                    return;
                }
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            _ => {
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
        }
    } else {
        match name[0 as libc::c_int as usize] as libc::c_int {
            68 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"device\0\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
                let fresh38 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh38 = 80 as libc::c_int as u_char;
                return;
            }
            69 => {
                if len < 2 as libc::c_int {
                    comperror(-(8 as libc::c_int) as libc::c_short);
                    return;
                }
                match toupper(name[1 as libc::c_int as usize] as libc::c_int) {
                    67 => {
                        if len > 2 as libc::c_int {
                            if strncasecmp(
                                name.as_mut_ptr(),
                                b"ecode\0\0" as *const u8 as *const libc::c_char,
                                6 as libc::c_int as libc::c_ulong,
                            ) != 0 as libc::c_int
                            {
                                comperror(-(8 as libc::c_int) as libc::c_short);
                                return;
                            }
                        }
                        let fresh39 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh39 = 81 as libc::c_int as u_char;
                        return;
                    }
                    83 => {
                        if len > 2 as libc::c_int {
                            if strncasecmp(
                                name.as_mut_ptr(),
                                b"estack\0\0" as *const u8 as *const libc::c_char,
                                7 as libc::c_int as libc::c_ulong,
                            ) != 0 as libc::c_int
                            {
                                comperror(-(8 as libc::c_int) as libc::c_short);
                                return;
                            }
                        }
                        let fresh40 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh40 = 82 as libc::c_int as u_char;
                        return;
                    }
                    84 => {
                        if len > 2 as libc::c_int {
                            if strncasecmp(
                                name.as_mut_ptr(),
                                b"etrap\0\0" as *const u8 as *const libc::c_char,
                                6 as libc::c_int as libc::c_ulong,
                            ) != 0 as libc::c_int
                            {
                                comperror(-(8 as libc::c_int) as libc::c_short);
                                return;
                            }
                        }
                        let fresh41 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh41 = 83 as libc::c_int as u_char;
                        return;
                    }
                    _ => {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
            }
            72 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"horolog\0\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
                let fresh42 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh42 = 84 as libc::c_int as u_char;
                return;
            }
            73 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"io\0\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
                let fresh43 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh43 = 85 as libc::c_int as u_char;
                return;
            }
            74 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"job\0\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
                let fresh44 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh44 = 86 as libc::c_int as u_char;
                return;
            }
            75 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"key\0\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
                let fresh45 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh45 = 87 as libc::c_int as u_char;
                return;
            }
            80 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"principal\0\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
                let fresh46 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh46 = 88 as libc::c_int as u_char;
                return;
            }
            81 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"quit\0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
                let fresh47 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh47 = 89 as libc::c_int as u_char;
                return;
            }
            82 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"reference\0\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
                let fresh48 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh48 = 90 as libc::c_int as u_char;
                return;
            }
            83 => {
                if len == 1 as libc::c_int
                    || strncasecmp(
                        name.as_mut_ptr(),
                        b"storage\0\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    let fresh49 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh49 = 91 as libc::c_int as u_char;
                    return;
                }
                match toupper(name[1 as libc::c_int as usize] as libc::c_int) {
                    84 => {
                        if len > 2 as libc::c_int {
                            if strncasecmp(
                                name.as_mut_ptr(),
                                b"stack\0\0" as *const u8 as *const libc::c_char,
                                6 as libc::c_int as libc::c_ulong,
                            ) != 0 as libc::c_int
                            {
                                comperror(-(8 as libc::c_int) as libc::c_short);
                                return;
                            }
                        }
                        let fresh50 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh50 = 92 as libc::c_int as u_char;
                        return;
                    }
                    89 => {
                        if len > 2 as libc::c_int {
                            if strncasecmp(
                                name.as_mut_ptr(),
                                b"system\0\0" as *const u8 as *const libc::c_char,
                                7 as libc::c_int as libc::c_ulong,
                            ) != 0 as libc::c_int
                            {
                                comperror(-(8 as libc::c_int) as libc::c_short);
                                return;
                            }
                        }
                        let fresh51 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh51 = 93 as libc::c_int as u_char;
                        return;
                    }
                    _ => {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
            }
            84 => {
                if len > 1 as libc::c_int {
                    if strncasecmp(
                        name.as_mut_ptr(),
                        b"test\0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(-(8 as libc::c_int) as libc::c_short);
                        return;
                    }
                }
                let fresh52 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh52 = 94 as libc::c_int as u_char;
                return;
            }
            88 => {
                if len > 1 as libc::c_int {
                    comperror(-(8 as libc::c_int) as libc::c_short);
                    return;
                }
                let fresh53 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh53 = 95 as libc::c_int as u_char;
                return;
            }
            89 => {
                if len > 1 as libc::c_int {
                    comperror(-(8 as libc::c_int) as libc::c_short);
                    return;
                }
                let fresh54 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh54 = 96 as libc::c_int as u_char;
                return;
            }
            _ => {
                comperror(-(8 as libc::c_int) as libc::c_short);
                return;
            }
        }
    };
}
