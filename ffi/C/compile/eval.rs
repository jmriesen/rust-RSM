#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type GBD;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut partab: partab_struct;
    fn SQ_Write(buf: *mut cstring) -> libc::c_int;
    fn SQ_WriteFormat(count: libc::c_int) -> libc::c_short;
    fn dodollar();
    fn itocstring(buf: *mut u_char, n: libc::c_int) -> u_short;
    fn UTIL_strerror(err: libc::c_int, buf: *mut u_char) -> u_short;
    fn ncopy(src: *mut *mut u_char, dst: *mut u_char) -> libc::c_short;
    fn localvar() -> libc::c_short;
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
#[no_mangle]
pub static mut source_ptr: *mut u_char = 0 as *const u_char as *mut u_char;
#[no_mangle]
pub static mut comp_ptr: *mut u_char = 0 as *const u_char as *mut u_char;
#[no_mangle]
pub unsafe extern "C" fn comperror(mut err: libc::c_short) {
    let mut current_block: u64;
    let mut s: libc::c_int = 0;
    let mut us: u_short = 0;
    let mut line: *mut cstring = 0 as *mut cstring;
    let mut src: *mut u_char = 0 as *mut u_char;
    let mut i: libc::c_int = 0;
    let mut tmp: [u_char; 128] = [0; 128];
    let fresh0 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh0 = 2 as libc::c_int as u_char;
    memcpy(
        comp_ptr as *mut libc::c_void,
        &mut err as *mut libc::c_short as *const libc::c_void,
        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
    );
    comp_ptr = comp_ptr
        .offset(::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize);
    let fresh1 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh1 = 179 as libc::c_int as u_char;
    let fresh2 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh2 = 179 as libc::c_int as u_char;
    if !(partab.checkonly == 0) {
        if partab.checkonly == *partab.ln {
            return;
        }
        partab.checkonly = *partab.ln;
        line = *partab.lp;
        src = *partab.sp;
        s = SQ_Write(line);
        if !(s < 0 as libc::c_int) {
            s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
            if !(s < 0 as libc::c_int) {
                i = (src.offset_from(((*line).buf).as_mut_ptr()) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as libc::c_int;
                if i > 0 as libc::c_int {
                    s = SQ_WriteFormat(i) as libc::c_int;
                    if s < 0 as libc::c_int {
                        current_block = 3644257603679614882;
                    } else {
                        current_block = 5143058163439228106;
                    }
                } else {
                    current_block = 5143058163439228106;
                }
                match current_block {
                    3644257603679614882 => {}
                    _ => {
                        line = tmp.as_mut_ptr() as *mut cstring;
                        (*line).buf[0 as libc::c_int as usize] = '^' as i32 as u_char;
                        (*line).buf[1 as libc::c_int as usize] = ' ' as i32 as u_char;
                        us = UTIL_strerror(
                            err as libc::c_int,
                            &mut *((*line).buf)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize),
                        );
                        (*line).len = (us as libc::c_int + 2 as libc::c_int) as u_short;
                        memcpy(
                            &mut *((*line).buf).as_mut_ptr().offset((*line).len as isize)
                                as *mut u_char as *mut libc::c_void,
                            b" - At line \0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            11 as libc::c_int as libc::c_ulong,
                        );
                        us = itocstring(
                            &mut *((*line).buf)
                                .as_mut_ptr()
                                .offset(
                                    ((*line).len as libc::c_int + 11 as libc::c_int) as isize,
                                ),
                            *partab.ln,
                        );
                        (*line)
                            .len = ((*line).len as libc::c_int
                            + (us as libc::c_int + 11 as libc::c_int)) as u_short;
                        s = SQ_Write(line);
                        if s >= 0 as libc::c_int {
                            SQ_WriteFormat(-(1 as libc::c_int));
                        }
                        if partab.checkonly != 0 {
                            partab.errors = (partab.errors).wrapping_add(1);
                            partab.errors;
                        }
                    }
                }
            }
        }
    }
    while *source_ptr != 0 {
        source_ptr = source_ptr.offset(1);
        source_ptr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn atom() {
    let mut c: libc::c_char = 0;
    let mut s: libc::c_short = 0;
    let fresh3 = source_ptr;
    source_ptr = source_ptr.offset(1);
    c = *fresh3 as libc::c_char;
    if c as libc::c_int == '@' as i32 {
        atom();
        if *source_ptr as libc::c_int != '@' as i32 {
            let fresh4 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh4 = 65 as libc::c_int as u_char;
            return;
        }
        let fresh5 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh5 = 66 as libc::c_int as u_char;
        s = localvar();
        if (s as libc::c_int) < 0 as libc::c_int {
            comperror(s);
            return;
        }
        return;
    }
    if isalpha(c as libc::c_int) != 0 as libc::c_int || c as libc::c_int == '%' as i32
        || c as libc::c_int == '^' as i32
    {
        source_ptr = source_ptr.offset(-1);
        source_ptr;
        s = localvar();
        if (s as libc::c_int) < 0 as libc::c_int {
            comperror(s);
            return;
        }
        return;
    }
    if c as libc::c_int == '$' as i32 {
        dodollar();
        return;
    }
    if isdigit(c as libc::c_int) != 0 as libc::c_int || c as libc::c_int == '.' as i32 {
        source_ptr = source_ptr.offset(-1);
        source_ptr;
        let fresh6 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh6 = 60 as libc::c_int as u_char;
        s = ncopy(
            &mut source_ptr,
            comp_ptr.offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize),
        );
        if (s as libc::c_int) < 0 as libc::c_int {
            comp_ptr = comp_ptr.offset(-1);
            comp_ptr;
            comperror(s);
            return;
        }
        *(comp_ptr as *mut u_short) = s as u_short;
        comp_ptr = comp_ptr
            .offset(
                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                    .wrapping_add(s as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        return;
    }
    if c as libc::c_int == '"' as i32 {
        let mut j: libc::c_int = ::core::mem::size_of::<u_short>() as libc::c_ulong
            as libc::c_int;
        let mut p: *mut u_char = 0 as *mut u_char;
        let fresh7 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh7 = 60 as libc::c_int as u_char;
        p = comp_ptr;
        loop {
            if *source_ptr as libc::c_int == '\0' as i32 {
                comp_ptr = comp_ptr.offset(-1);
                comp_ptr;
                comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            if *source_ptr as libc::c_int == '"' as i32
                && *source_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    != '"' as i32
            {
                *p.offset(j as isize) = '\0' as i32 as u_char;
                source_ptr = source_ptr.offset(1);
                source_ptr;
                break;
            } else {
                let fresh8 = source_ptr;
                source_ptr = source_ptr.offset(1);
                let fresh9 = j;
                j = j + 1;
                *p.offset(fresh9 as isize) = *fresh8;
                if *source_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == '"' as i32 && *source_ptr as libc::c_int == '"' as i32
                {
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                }
            }
        }
        *(p
            as *mut u_short) = (j as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<u_short>() as libc::c_ulong) as u_short;
        comp_ptr = comp_ptr.offset((j + 1 as libc::c_int) as isize);
        return;
    }
    if c as libc::c_int == '\'' as i32 {
        atom();
        let fresh10 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh10 = 3 as libc::c_int as u_char;
        return;
    }
    if c as libc::c_int == '+' as i32 {
        atom();
        let fresh11 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh11 = 18 as libc::c_int as u_char;
        return;
    }
    if c as libc::c_int == '-' as i32 {
        atom();
        let fresh12 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh12 = 19 as libc::c_int as u_char;
        return;
    }
    if c as libc::c_int == '(' as i32 {
        eval();
        let fresh13 = source_ptr;
        source_ptr = source_ptr.offset(1);
        if *fresh13 as libc::c_int != ')' as i32 {
            comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
            return;
        }
        return;
    }
    comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
}
#[no_mangle]
pub unsafe extern "C" fn operator() -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut not: libc::c_int = 0 as libc::c_int;
    let fresh14 = source_ptr;
    source_ptr = source_ptr.offset(1);
    c = *fresh14 as libc::c_char;
    if c as libc::c_int == '\'' as i32 {
        not = 1 as libc::c_int;
        let fresh15 = source_ptr;
        source_ptr = source_ptr.offset(1);
        c = *fresh15 as libc::c_char;
    }
    match c as libc::c_int {
        43 => {
            if not != 0 {
                return 0 as libc::c_int;
            }
            return 10 as libc::c_int;
        }
        45 => {
            if not != 0 {
                return 0 as libc::c_int;
            }
            return 11 as libc::c_int;
        }
        42 => {
            if not != 0 {
                return 0 as libc::c_int;
            }
            if *source_ptr as libc::c_int == '*' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                return 16 as libc::c_int;
            }
            return 12 as libc::c_int;
        }
        47 => {
            if not != 0 {
                return 0 as libc::c_int;
            }
            return 13 as libc::c_int;
        }
        92 => {
            if not != 0 {
                return 0 as libc::c_int;
            }
            return 14 as libc::c_int;
        }
        35 => {
            if not != 0 {
                return 0 as libc::c_int;
            }
            return 15 as libc::c_int;
        }
        95 => {
            if not != 0 {
                return 0 as libc::c_int;
            }
            return 17 as libc::c_int;
        }
        61 => return if not != 0 { 30 as libc::c_int } else { 20 as libc::c_int },
        60 => return if not != 0 { 31 as libc::c_int } else { 21 as libc::c_int },
        62 => return if not != 0 { 32 as libc::c_int } else { 22 as libc::c_int },
        38 => return if not != 0 { 33 as libc::c_int } else { 23 as libc::c_int },
        33 => return if not != 0 { 34 as libc::c_int } else { 24 as libc::c_int },
        91 => return if not != 0 { 35 as libc::c_int } else { 25 as libc::c_int },
        93 => {
            if *source_ptr as libc::c_int == ']' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                return if not != 0 { 37 as libc::c_int } else { 27 as libc::c_int };
            }
            return if not != 0 { 36 as libc::c_int } else { 26 as libc::c_int };
        }
        63 => return if not != 0 { 38 as libc::c_int } else { 28 as libc::c_int },
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn eval() {
    let mut q: libc::c_int = 0;
    let mut ptr: *mut cstring = 0 as *mut cstring;
    let mut c: u_char = 0;
    atom();
    if *source_ptr as libc::c_int == ')' as i32
        || *source_ptr as libc::c_int == ',' as i32
        || *source_ptr as libc::c_int == ':' as i32
        || *source_ptr as libc::c_int == '\0' as i32
        || *source_ptr as libc::c_int == '^' as i32
        || *source_ptr as libc::c_int == '@' as i32
        || *source_ptr as libc::c_int == ' ' as i32
    {
        return;
    }
    loop {
        let mut op: libc::c_int = operator();
        let mut pattern: libc::c_int = 0 as libc::c_int;
        if op == 0 as libc::c_int {
            comperror(-(12 as libc::c_int + 200 as libc::c_int) as libc::c_short);
            return;
        }
        pattern = (op == 28 as libc::c_int || op == 38 as libc::c_int) as libc::c_int;
        if pattern != 0 && *source_ptr as libc::c_int == '@' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
            pattern = 0 as libc::c_int;
        }
        if pattern != 0 {
            q = 0 as libc::c_int;
            let fresh16 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh16 = 60 as libc::c_int as u_char;
            ptr = comp_ptr as *mut cstring;
            comp_ptr = comp_ptr
                .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
            loop {
                let fresh17 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh17;
                if q != 0 {
                    let fresh18 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh18 = c;
                    if c as libc::c_int == '"' as i32 {
                        q = 0 as libc::c_int;
                    }
                } else if c as libc::c_int == '"' as i32 {
                    q = 1 as libc::c_int;
                    let fresh19 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh19 = c;
                } else if isalnum(c as libc::c_int) != 0 as libc::c_int
                    || c as libc::c_int == '.' as i32
                {
                    let fresh20 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh20 = c;
                } else if c as libc::c_int == '(' as i32 {
                    pattern += 1;
                    pattern;
                    let fresh21 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh21 = c;
                } else if c as libc::c_int == ')' as i32 && pattern > 1 as libc::c_int {
                    pattern -= 1;
                    pattern;
                    let fresh22 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh22 = c;
                } else if pattern > 1 as libc::c_int && c as libc::c_int == ',' as i32 {
                    let fresh23 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh23 = c;
                } else {
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    break;
                }
            }
            (*ptr)
                .len = comp_ptr.offset_from(((*ptr).buf).as_mut_ptr()) as libc::c_long
                as u_short;
            let fresh24 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh24 = '\0' as i32 as u_char;
        } else {
            atom();
        }
        let fresh25 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh25 = op as u_char;
        if *source_ptr as libc::c_int == ')' as i32
            || *source_ptr as libc::c_int == ',' as i32
            || *source_ptr as libc::c_int == ':' as i32
            || *source_ptr as libc::c_int == '\0' as i32
            || *source_ptr as libc::c_int == '^' as i32
            || *source_ptr as libc::c_int == ' ' as i32
        {
            return;
        }
    };
}
