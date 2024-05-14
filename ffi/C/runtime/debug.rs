#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type GBD;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut partab: partab_struct;
    static mut strstk: [u_char; 0];
    static mut rsmpc: *mut u_char;
    fn SQ_Write(buf: *mut cstring) -> libc::c_int;
    fn SQ_WriteFormat(count: libc::c_int) -> libc::c_short;
    fn SQ_Read(buf: *mut u_char, tout: libc::c_int, maxbyt: libc::c_int) -> libc::c_int;
    fn parse();
    fn itocstring(buf: *mut u_char, n: libc::c_int) -> u_short;
    fn uitocstring(buf: *mut u_char, n: u_int) -> u_short;
    fn run(asp: libc::c_int, ssp: libc::c_int) -> libc::c_short;
    fn ST_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn ST_Set(var: *mut mvar, data: *mut cstring) -> libc::c_int;
    fn ST_Kill(var: *mut mvar) -> libc::c_short;
    static mut source_ptr: *mut u_char;
    static mut comp_ptr: *mut u_char;
    static mut isp: libc::c_long;
    static mut history: [[libc::c_char; 65534]; 128];
    static mut hist_next: u_short;
    static mut hist_curr: u_short;
    static mut in_hist: libc::c_short;
    static mut prompt_len: u_short;
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
pub static mut dvar: mvar = MVAR {
    name: VAR_U { var_q: 0 },
    volset: 0,
    uci: 0,
    slen: 0,
    key: [0; 256],
};
#[no_mangle]
pub static mut src: [u_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut cmp: [u_char; 1024] = [0; 1024];
#[no_mangle]
pub unsafe extern "C" fn Debug_off() {
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        dvar.name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    memcpy(
        &mut *(dvar.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_void,
        b"$ZBP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    dvar.volset = 0 as libc::c_int as u_char;
    dvar.uci = 255 as libc::c_int as u_char;
    dvar.slen = 0 as libc::c_int as u_char;
    ST_Kill(&mut dvar);
    partab.debug = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Debug_on(mut param: *mut cstring) -> libc::c_short {
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut cstring = 0 as *mut cstring;
    let mut temp_src: [u_char; 256] = [0; 256];
    let mut temp_cmp: [u_char; 1024] = [0; 1024];
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        dvar.name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    memcpy(
        &mut *(dvar.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_void,
        b"$ZBP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    dvar.volset = 0 as libc::c_int as u_char;
    dvar.uci = 255 as libc::c_int as u_char;
    dvar.slen = 0 as libc::c_int as u_char;
    dvar.key[0 as libc::c_int as usize] = 128 as libc::c_int as u_char;
    if (*param).buf[0 as libc::c_int as usize] as libc::c_int != ':' as i32 {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut s: libc::c_int = 0;
        let mut off: libc::c_int = 1 as libc::c_int;
        if (*param).buf[i as usize] as libc::c_int == '+' as i32 {
            i += 1;
            i;
            off = 0 as libc::c_int;
            while isdigit((*param).buf[i as usize] as libc::c_int) != 0 {
                let fresh0 = i;
                i = i + 1;
                off = off * 10 as libc::c_int
                    + ((*param).buf[fresh0 as usize] as libc::c_int - '0' as i32);
            }
        }
        if off < 1 as libc::c_int || off > 65534 as libc::c_int {
            return -(9 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        let fresh1 = i;
        i = i + 1;
        if (*param).buf[fresh1 as usize] as libc::c_int != '^' as i32 {
            return -(9 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        j = 0 as libc::c_int;
        while j < 32 as libc::c_int {
            if isalnum((*param).buf[(j + i) as usize] as libc::c_int) == 0 as libc::c_int
                && ((*param).buf[(j + i) as usize] as libc::c_int != '%' as i32
                    || j != 0 as libc::c_int)
            {
                break;
            }
            dvar.key[(j + 1 as libc::c_int) as usize] = (*param).buf[(j + i) as usize];
            j += 1;
            j;
        }
        dvar.key[(j + 1 as libc::c_int) as usize] = '\0' as i32 as u_char;
        dvar.slen = (j + 2 as libc::c_int) as u_char;
        j += i;
        if isalnum((*param).buf[j as usize] as libc::c_int) != 0 {
            return -(56 as libc::c_int) as libc::c_short;
        }
        if (*param).buf[j as usize] as libc::c_int != ':' as i32
            && (*param).buf[j as usize] as libc::c_int != '\0' as i32
        {
            return -(9 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        let fresh2 = dvar.slen;
        dvar.slen = (dvar.slen).wrapping_add(1);
        dvar.key[fresh2 as usize] = 64 as libc::c_int as u_char;
        s = itocstring(&mut *(dvar.key).as_mut_ptr().offset(dvar.slen as isize), off)
            as libc::c_int;
        dvar
            .key[(dvar.slen as libc::c_int - 1 as libc::c_int)
            as usize] = (dvar.key[(dvar.slen as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int | s) as u_char;
        dvar.slen = (dvar.slen as libc::c_int + s) as u_char;
        dvar.slen = (dvar.slen).wrapping_add(1);
        dvar.slen;
    }
    let fresh3 = j;
    j = j + 1;
    if (*param).buf[fresh3 as usize] as libc::c_int == '\0' as i32 {
        return ST_Kill(&mut dvar);
    }
    if partab.debug == 0 {
        partab.debug = -(1 as libc::c_int);
    }
    if (*param).buf[j as usize] as libc::c_int == '\0' as i32 {
        ptr = temp_src.as_mut_ptr() as *mut cstring;
        (*ptr).len = 0 as libc::c_int as u_short;
        (*ptr).buf[0 as libc::c_int as usize] = '\0' as i32 as u_char;
        return ST_Set(&mut dvar, ptr) as libc::c_short;
    }
    if (*param).len as libc::c_int - j > 255 as libc::c_int {
        return -(9 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    source_ptr = &mut *((*param).buf).as_mut_ptr().offset(j as isize) as *mut u_char;
    ptr = temp_cmp.as_mut_ptr() as *mut cstring;
    comp_ptr = ((*ptr).buf).as_mut_ptr();
    parse();
    let fresh4 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh4 = 0 as libc::c_int as u_char;
    let fresh5 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh5 = 0 as libc::c_int as u_char;
    (*ptr)
        .len = comp_ptr.offset_from(((*ptr).buf).as_mut_ptr()) as libc::c_long
        as u_short;
    return ST_Set(&mut dvar, ptr) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Debug(
    mut savasp: libc::c_int,
    mut savssp: libc::c_int,
    mut dot: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut io: libc::c_int = 0;
    let mut s: libc::c_int = 0 as libc::c_int;
    let mut ts: libc::c_short = 0;
    let mut curframe: *mut do_frame = 0 as *mut do_frame;
    let mut ptr: *mut cstring = 0 as *mut cstring;
    let mut var: *mut mvar = 0 as *mut mvar;
    if partab.debug == 0 {
        partab.debug = -(1 as libc::c_int);
    }
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        dvar.name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    memcpy(
        &mut *(dvar.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_void,
        b"$ZBP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    dvar.volset = 0 as libc::c_int as u_char;
    dvar.uci = 255 as libc::c_int as u_char;
    dvar.slen = 0 as libc::c_int as u_char;
    curframe = &mut *((*partab.jobtab).dostk)
        .as_mut_ptr()
        .offset((*partab.jobtab).cur_do as isize) as *mut do_frame;
    if dot == 0 as libc::c_int {
        if (*curframe).type_0 as libc::c_int != 3 as libc::c_int
            && (*curframe).type_0 as libc::c_int != 4 as libc::c_int
        {
            return 0 as libc::c_int as libc::c_short;
        }
        dvar.key[0 as libc::c_int as usize] = 128 as libc::c_int as u_char;
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            dvar
                .key[(i + 1 as libc::c_int)
                as usize] = (*curframe).rounam.var_cu[i as usize];
            if dvar.key[(i + 1 as libc::c_int) as usize] == 0 {
                break;
            }
            i += 1;
            i;
        }
        dvar.slen = (i + 1 as libc::c_int) as u_char;
        let fresh6 = dvar.slen;
        dvar.slen = (dvar.slen).wrapping_add(1);
        dvar.key[fresh6 as usize] = '\0' as i32 as u_char;
        let fresh7 = dvar.slen;
        dvar.slen = (dvar.slen).wrapping_add(1);
        dvar.key[fresh7 as usize] = 64 as libc::c_int as u_char;
        s = uitocstring(
            &mut *(dvar.key).as_mut_ptr().offset(dvar.slen as isize),
            (*curframe).line_num as u_int,
        ) as libc::c_int;
        dvar
            .key[(dvar.slen as libc::c_int - 1 as libc::c_int)
            as usize] = (dvar.key[(dvar.slen as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int | s) as u_char;
        dvar.slen = (dvar.slen as libc::c_int + s) as u_char;
        dvar.slen = (dvar.slen).wrapping_add(1);
        dvar.slen;
        s = ST_Get(
            &mut dvar,
            &mut *cmp
                .as_mut_ptr()
                .offset(
                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                ),
        );
        if s < 0 as libc::c_int {
            return 0 as libc::c_int as libc::c_short;
        }
        ts = s as libc::c_short;
        memcpy(
            cmp.as_mut_ptr() as *mut libc::c_void,
            &mut ts as *mut libc::c_short as *const libc::c_void,
            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
        );
    } else if dot == -(1 as libc::c_int) {
        s = ST_Get(
            &mut dvar,
            &mut *cmp
                .as_mut_ptr()
                .offset(
                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                ),
        );
        if s < 0 as libc::c_int {
            s = 0 as libc::c_int;
        }
        ts = s as libc::c_short;
        memcpy(
            cmp.as_mut_ptr() as *mut libc::c_void,
            &mut ts as *mut libc::c_short as *const libc::c_void,
            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
        );
    }
    if (*partab.jobtab).cur_do >= 128 as libc::c_int {
        return -(8 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].pc = rsmpc;
    if s > 0 as libc::c_int {
        (*partab.jobtab).cur_do += 1;
        (*partab.jobtab).cur_do;
        rsmpc = &mut *cmp
            .as_mut_ptr()
            .offset(::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize)
            as *mut u_char;
        src[0 as libc::c_int as usize] = '\0' as i32 as u_char;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .routine = src.as_mut_ptr();
        (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].pc = rsmpc;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .symbol = 0 as *mut libc::c_short;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .newtab = 0 as *mut u_char;
        memcpy(
            &mut ts as *mut libc::c_short as *mut libc::c_void,
            &mut cmp as *mut [u_char; 1024] as *const libc::c_void,
            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
        );
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .endlin = &mut *cmp
            .as_mut_ptr()
            .offset(
                ((ts as libc::c_int - 3 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ) as isize,
            ) as *mut u_char;
        let mut var_i_0: u_int = 0 as libc::c_int as u_int;
        while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            (*partab.jobtab)
                .dostk[(*partab.jobtab).cur_do as usize]
                .rounam
                .var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
            var_i_0 = var_i_0.wrapping_add(1);
            var_i_0;
        }
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .vol = (*partab.jobtab).vol;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .uci = (*partab.jobtab).uci;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .line_num = 0 as libc::c_int as u_short;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .type_0 = 1 as libc::c_int as u_char;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .level = 0 as libc::c_int as u_char;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .flags = 0 as libc::c_int as u_char;
        (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].isp = isp;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .asp = savasp as libc::c_long;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .ssp = savssp as libc::c_long;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .savasp = savasp as libc::c_long;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .savssp = savssp as libc::c_long;
        s = run(savasp, savssp) as libc::c_int;
        if s == 1 as libc::c_int {
            return s as libc::c_short;
        }
        (*partab.jobtab).cur_do -= 1;
        (*partab.jobtab).cur_do;
        rsmpc = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].pc;
        if s & 16384 as libc::c_int != 0 {
            s &= !(16384 as libc::c_int);
            if s > 0 as libc::c_int {
                partab
                    .debug = (s as u_int).wrapping_add((*partab.jobtab).commands)
                    as libc::c_int;
                (*partab.jobtab).attention = 1 as libc::c_int;
                s = 0 as libc::c_int;
            }
        } else if s > 0 as libc::c_int {
            return s as libc::c_short
        }
    }
    io = (*partab.jobtab).io as libc::c_int;
    (*partab.jobtab).io = 0 as libc::c_int as u_char;
    (*partab.jobtab)
        .seqio[0 as libc::c_int as usize]
        .options = ((*partab.jobtab).seqio[0 as libc::c_int as usize].options
        as libc::c_int | 8 as libc::c_int) as u_char;
    ptr = src.as_mut_ptr() as *mut cstring;
    loop {
        if in_hist as libc::c_int == 0 as libc::c_int {
            if (*partab.jobtab).seqio[0 as libc::c_int as usize].dx != 0 {
                s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                if s < 0 as libc::c_int {
                    return s as libc::c_short;
                }
            }
            if var_empty((*curframe).rounam) != 0 {
                memcpy(
                    ((*ptr).buf).as_mut_ptr() as *mut libc::c_void,
                    b"Debug\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    5 as libc::c_int as libc::c_ulong,
                );
                (*ptr).len = 5 as libc::c_int as u_short;
                partab.debug = -(1 as libc::c_int);
            } else {
                (*ptr).len = 0 as libc::c_int as u_short;
                let fresh8 = (*ptr).len;
                (*ptr).len = ((*ptr).len).wrapping_add(1);
                (*ptr).buf[fresh8 as usize] = '+' as i32 as u_char;
                (*ptr)
                    .len = ((*ptr).len as libc::c_int
                    + uitocstring(
                        &mut *((*ptr).buf).as_mut_ptr().offset((*ptr).len as isize),
                        (*curframe).line_num as u_int,
                    ) as libc::c_int) as u_short;
                let fresh9 = (*ptr).len;
                (*ptr).len = ((*ptr).len).wrapping_add(1);
                (*ptr).buf[fresh9 as usize] = '^' as i32 as u_char;
                i = 0 as libc::c_int;
                while i < 32 as libc::c_int {
                    (*ptr)
                        .buf[(i + (*ptr).len as libc::c_int)
                        as usize] = (*curframe).rounam.var_cu[i as usize];
                    if (*ptr).buf[(i + (*ptr).len as libc::c_int) as usize] == 0 {
                        break;
                    }
                    i += 1;
                    i;
                }
                (*ptr).len = ((*ptr).len as libc::c_int + i) as u_short;
            }
            let fresh10 = (*ptr).len;
            (*ptr).len = ((*ptr).len).wrapping_add(1);
            (*ptr).buf[fresh10 as usize] = '>' as i32 as u_char;
            let fresh11 = (*ptr).len;
            (*ptr).len = ((*ptr).len).wrapping_add(1);
            (*ptr).buf[fresh11 as usize] = ' ' as i32 as u_char;
            (*ptr).buf[(*ptr).len as usize] = '\0' as i32 as u_char;
            prompt_len = (*ptr).len;
            s = SQ_Write(ptr);
            if s < 0 as libc::c_int {
                return s as libc::c_short;
            }
        }
        s = SQ_Read(((*ptr).buf).as_mut_ptr(), -(1 as libc::c_int), 256 as libc::c_int);
        if s < 1 as libc::c_int {
            continue;
        }
        if hist_next == 0
            || strcmp(
                (history[(hist_next as libc::c_int - 1 as libc::c_int) as usize])
                    .as_mut_ptr(),
                ((*ptr).buf).as_mut_ptr() as *mut libc::c_char,
            ) != 0
        {
            strcpy(
                (history[hist_next as usize]).as_mut_ptr(),
                ((*ptr).buf).as_mut_ptr() as *mut libc::c_char,
            );
            if hist_next as libc::c_int == 128 as libc::c_int - 1 as libc::c_int {
                hist_next = 0 as libc::c_int as u_short;
            } else {
                hist_next = hist_next.wrapping_add(1);
                hist_next;
            }
        }
        hist_curr = hist_next;
        s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
        if s < 0 as libc::c_int {
            return s as libc::c_short;
        }
        source_ptr = ((*ptr).buf).as_mut_ptr();
        comp_ptr = cmp.as_mut_ptr();
        parse();
        let fresh12 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh12 = 0 as libc::c_int as u_char;
        let fresh13 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh13 = 0 as libc::c_int as u_char;
        (*partab.jobtab).cur_do += 1;
        (*partab.jobtab).cur_do;
        rsmpc = cmp.as_mut_ptr();
        src[0 as libc::c_int as usize] = '\0' as i32 as u_char;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .routine = src.as_mut_ptr();
        (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].pc = rsmpc;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .symbol = 0 as *mut libc::c_short;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .newtab = 0 as *mut u_char;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .endlin = comp_ptr.offset(-(3 as libc::c_int as isize));
        let mut var_i_1: u_int = 0 as libc::c_int as u_int;
        while var_i_1 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            (*partab.jobtab)
                .dostk[(*partab.jobtab).cur_do as usize]
                .rounam
                .var_qu[var_i_1 as usize] = 0 as libc::c_int as u_int64;
            var_i_1 = var_i_1.wrapping_add(1);
            var_i_1;
        }
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .vol = (*partab.jobtab).vol;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .uci = (*partab.jobtab).uci;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .line_num = 0 as libc::c_int as u_short;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .type_0 = 1 as libc::c_int as u_char;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .level = 0 as libc::c_int as u_char;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .flags = 0 as libc::c_int as u_char;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .savasp = savasp as libc::c_long;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .savssp = savssp as libc::c_long;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .asp = savasp as libc::c_long;
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .ssp = savssp as libc::c_long;
        (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].isp = isp;
        s = run(savasp, savssp) as libc::c_int;
        if s == 1 as libc::c_int {
            return s as libc::c_short;
        }
        (*partab.jobtab).cur_do -= 1;
        (*partab.jobtab).cur_do;
        if partab.debug == 0 {
            break;
        }
        if s & 16384 as libc::c_int != 0 {
            s &= !(16384 as libc::c_int);
            if s > 0 as libc::c_int {
                partab
                    .debug = (s as u_int).wrapping_add((*partab.jobtab).commands)
                    as libc::c_int;
                (*partab.jobtab).attention = 1 as libc::c_int;
                s = 0 as libc::c_int;
                break;
            }
        }
        if s == 157 as libc::c_int {
            partab.debug = -(1 as libc::c_int);
            break;
        } else {
            if s == 1 as libc::c_int {
                (*partab.jobtab).io = io as u_char;
                rsmpc = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].pc;
                return s as libc::c_short;
            }
            var = &mut *strstk.as_mut_ptr().offset(savssp as isize) as *mut u_char
                as *mut mvar;
            let mut var_i_2: u_int = 0 as libc::c_int as u_int;
            while var_i_2 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                (*var).name.var_qu[var_i_2 as usize] = 0 as libc::c_int as u_int64;
                var_i_2 = var_i_2.wrapping_add(1);
                var_i_2;
            }
            memcpy(
                &mut *((*var).name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_void,
                b"$ECODE\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                6 as libc::c_int as libc::c_ulong,
            );
            (*var).volset = 0 as libc::c_int as u_char;
            (*var).uci = 255 as libc::c_int as u_char;
            (*var).slen = 0 as libc::c_int as u_char;
            ptr = (&mut *strstk.as_mut_ptr().offset(savssp as isize) as *mut u_char)
                .offset(::core::mem::size_of::<mvar>() as libc::c_ulong as isize)
                as *mut cstring;
            memcpy(
                ((*ptr).buf).as_mut_ptr() as *mut libc::c_void,
                b"$ECODE=\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                7 as libc::c_int as libc::c_ulong,
            );
            s = ST_Get(
                var,
                &mut *((*ptr).buf).as_mut_ptr().offset(7 as libc::c_int as isize),
            );
            if s < 1 as libc::c_int {
                continue;
            }
            (*ptr).len = (s + 7 as libc::c_int) as u_short;
            if (*partab.jobtab).seqio[0 as libc::c_int as usize].dx != 0 {
                s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                if s < 0 as libc::c_int {
                    return s as libc::c_short;
                }
            }
            s = SQ_Write(ptr);
            if s < 0 as libc::c_int {
                return s as libc::c_short;
            }
            s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
            if s < 0 as libc::c_int {
                return s as libc::c_short;
            }
        }
    }
    (*partab.jobtab).io = io as u_char;
    rsmpc = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].pc;
    return 0 as libc::c_int as libc::c_short;
}
