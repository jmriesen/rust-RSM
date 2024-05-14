#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type GBD;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut partab: partab_struct;
    fn eval();
    static mut source_ptr: *mut u_char;
    static mut comp_ptr: *mut u_char;
    fn atom();
}
pub type __uint32_t = libc::c_uint;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
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
pub unsafe extern "C" fn toupper(mut _c: libc::c_int) -> libc::c_int {
    return __toupper(_c);
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
pub unsafe extern "C" fn localvar() -> libc::c_short {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut idx: u_char = 0 as libc::c_int as u_char;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut var: var_u = VAR_U { var_q: 0 };
    let mut sptr: *mut u_char = 0 as *mut u_char;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut type_0: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut ret: libc::c_short = 0;
    ptr = comp_ptr;
    let fresh0 = source_ptr;
    source_ptr = source_ptr.offset(1);
    c = *fresh0 as libc::c_char;
    if c as libc::c_int == '@' as i32 {
        if *source_ptr as libc::c_int == '(' as i32 {
            type_0 = 255 as libc::c_int as libc::c_short;
            current_block = 16449232240049484035;
        } else {
            sptr = source_ptr;
            atom();
            if *source_ptr as libc::c_int == '@' as i32
                && *source_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    == '(' as i32
            {
                let fresh1 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh1 = 66 as libc::c_int as u_char;
                type_0 = 255 as libc::c_int as libc::c_short;
                source_ptr = source_ptr.offset(1);
                source_ptr;
                current_block = 16449232240049484035;
            } else {
                if *source_ptr as libc::c_int == '(' as i32
                    || *source_ptr as libc::c_int == '\0' as i32
                {
                    source_ptr = sptr;
                    comp_ptr = ptr;
                    let fresh2 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh2 as libc::c_char;
                } else {
                    return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short
                }
                current_block = 8457315219000651999;
            }
        }
    } else {
        current_block = 8457315219000651999;
    }
    match current_block {
        8457315219000651999 => {
            if c as libc::c_int == '^' as i32 {
                type_0 = 128 as libc::c_int as libc::c_short;
                let fresh3 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh3 as libc::c_char;
                v = (c as libc::c_int == '|' as i32) as libc::c_int;
                if v != 0 || c as libc::c_int == '[' as i32 {
                    type_0 = 253 as libc::c_int as libc::c_short;
                    atom();
                    let fresh4 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh4 as libc::c_char;
                    if v == 0 && c as libc::c_int == ',' as i32 {
                        type_0 = 254 as libc::c_int as libc::c_short;
                        atom();
                        let fresh5 = source_ptr;
                        source_ptr = source_ptr.offset(1);
                        c = *fresh5 as libc::c_char;
                    }
                    if v != 0 && c as libc::c_int != '|' as i32
                        || v == 0 && c as libc::c_int != ']' as i32
                    {
                        return -(12 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                    }
                    let fresh6 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh6 as libc::c_char;
                    current_block = 1538046216550696469;
                } else if c as libc::c_int == '(' as i32 {
                    type_0 = 252 as libc::c_int as libc::c_short;
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    current_block = 16449232240049484035;
                } else {
                    current_block = 1538046216550696469;
                }
            } else {
                current_block = 1538046216550696469;
            }
            match current_block {
                16449232240049484035 => {}
                _ => {
                    if isalpha(c as libc::c_int) == 0 as libc::c_int
                        && c as libc::c_int != '%' as i32
                        && c as libc::c_int != '$' as i32
                    {
                        return -(12 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                    }
                    if c as libc::c_int == '$' as i32
                        && type_0 as libc::c_int == 0 as libc::c_int
                    {
                        if isalpha(*source_ptr as libc::c_int) == 0 as libc::c_int {
                            return -(12 as libc::c_int + 200 as libc::c_int)
                                as libc::c_short;
                        }
                        i = toupper(*source_ptr as libc::c_int);
                        if (strchr(
                            b"DEHIJKPQRSTXYZ\0" as *const u8 as *const libc::c_char,
                            i,
                        ))
                            .is_null()
                        {
                            return -(8 as libc::c_int) as libc::c_short;
                        }
                    }
                    let mut var_i: u_int = 0 as libc::c_int as u_int;
                    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        var.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
                        var_i = var_i.wrapping_add(1);
                        var_i;
                    }
                    var.var_cu[0 as libc::c_int as usize] = c as u_char;
                    i = 1 as libc::c_int;
                    while i < 32 as libc::c_int {
                        let fresh7 = source_ptr;
                        source_ptr = source_ptr.offset(1);
                        c = *fresh7 as libc::c_char;
                        if isalnum(c as libc::c_int) == 0 as libc::c_int {
                            source_ptr = source_ptr.offset(-1);
                            source_ptr;
                            break;
                        } else {
                            var.var_cu[i as usize] = c as u_char;
                            i += 1;
                            i;
                        }
                    }
                    if isalnum(*source_ptr as libc::c_int) != 0 as libc::c_int {
                        return -(56 as libc::c_int) as libc::c_short;
                    }
                }
            }
        }
        _ => {}
    }
    if *source_ptr as libc::c_int == '(' as i32 {
        source_ptr = source_ptr.offset(1);
        source_ptr;
        loop {
            eval();
            count += 1;
            count;
            let fresh8 = source_ptr;
            source_ptr = source_ptr.offset(1);
            c = *fresh8 as libc::c_char;
            if c as libc::c_int == ')' as i32 {
                break;
            }
            if c as libc::c_int != ',' as i32 {
                return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
        }
    }
    if count > 63 as libc::c_int {
        return -(15 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    ret = comp_ptr.offset_from(ptr) as libc::c_long as libc::c_short;
    let fresh9 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh9 = 61 as libc::c_int as u_char;
    if (type_0 as libc::c_int) < 128 as libc::c_int && !(partab.varlst).is_null()
        && var.var_cu[0 as libc::c_int as usize] as libc::c_int != '$' as i32
    {
        i = 0 as libc::c_int;
        while !(i == 255 as libc::c_int + 1 as libc::c_int) {
            if var_equal(*(partab.varlst).offset(i as isize), var) != 0 {
                break;
            }
            if var_empty(*(partab.varlst).offset(i as isize)) != 0 {
                let mut var_i_0: u_int = 0 as libc::c_int as u_int;
                while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*(partab.varlst).offset(i as isize))
                        .var_qu[var_i_0 as usize] = var.var_qu[var_i_0 as usize];
                    var_i_0 = var_i_0.wrapping_add(1);
                    var_i_0;
                }
                break;
            } else {
                i += 1;
                i;
            }
        }
        if i != 255 as libc::c_int + 1 as libc::c_int {
            type_0 = (type_0 as libc::c_int | 64 as libc::c_int) as libc::c_short;
            idx = i as u_char;
        }
    }
    if (type_0 as libc::c_int) < 252 as libc::c_int {
        type_0 = (type_0 as libc::c_int + count) as libc::c_short;
        let fresh10 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh10 = type_0 as u_char;
        if type_0 as libc::c_int & 64 as libc::c_int != 0 {
            let fresh11 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh11 = idx;
        }
    } else {
        let fresh12 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh12 = type_0 as u_char;
        let fresh13 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh13 = count as u_char;
    }
    if ((type_0 as libc::c_int) < 64 as libc::c_int
        || type_0 as libc::c_int >= 128 as libc::c_int)
        && type_0 as libc::c_int != 252 as libc::c_int
        && type_0 as libc::c_int != 255 as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            let fresh14 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh14 = var.var_cu[i as usize];
            i += 1;
            i;
        }
    }
    return ret;
}
