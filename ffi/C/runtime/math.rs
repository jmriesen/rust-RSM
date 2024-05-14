#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type GBD;
    fn atoi(_: *const libc::c_char) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut partab: partab_struct;
    fn itocstring(buf: *mut u_char, n: libc::c_int) -> u_short;
}
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
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
#[no_mangle]
pub unsafe extern "C" fn runtime_add(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
) -> libc::c_short {
    let mut dpa: libc::c_short = 0;
    let mut dpb: libc::c_short = 0;
    let mut lena: libc::c_short = 0;
    let mut lenb: libc::c_short = 0;
    let mut mi: libc::c_int = 0;
    let mut sign: libc::c_short = 0;
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut carry: libc::c_int = 0;
    if *b.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        return strlen(a) as libc::c_short;
    }
    if *a.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        strcpy(a, b);
        return strlen(a) as libc::c_short;
    }
    mi = 0 as libc::c_int;
    sign = 0 as libc::c_int as libc::c_short;
    if *a.offset(0 as libc::c_int as isize) as libc::c_int
        == *b.offset(0 as libc::c_int as isize) as libc::c_int
        && *a.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        mi += 1;
        mi;
        let ref mut fresh0 = *b.offset(0 as libc::c_int as isize);
        *fresh0 = '0' as i32 as libc::c_char;
        *a.offset(0 as libc::c_int as isize) = *fresh0;
    } else if *a.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        sign -= 1;
        sign;
        *a
            .offset(
                0 as libc::c_int as isize,
            ) = ('0' as i32 + 10 as libc::c_int - 1 as libc::c_int) as libc::c_char;
        i = 0 as libc::c_int;
        loop {
            i += 1;
            ch = *a.offset(i as isize) as libc::c_int;
            if !(ch != '\0' as i32 as libc::c_char as libc::c_int) {
                break;
            }
            if ch != '.' as i32 {
                *a
                    .offset(
                        i as isize,
                    ) = ('0' as i32 + ('0' as i32 + 10 as libc::c_int - 1 as libc::c_int)
                    - ch) as libc::c_char;
            }
        }
        i -= 1;
        let ref mut fresh1 = *a.offset(i as isize);
        *fresh1 += 1;
        *fresh1;
    } else if *b.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        sign += 1;
        sign;
        *b
            .offset(
                0 as libc::c_int as isize,
            ) = ('0' as i32 + 10 as libc::c_int - 1 as libc::c_int) as libc::c_char;
        i = 0 as libc::c_int;
        loop {
            i += 1;
            ch = *b.offset(i as isize) as libc::c_int;
            if !(ch != '\0' as i32 as libc::c_char as libc::c_int) {
                break;
            }
            if ch != '.' as i32 {
                *b
                    .offset(
                        i as isize,
                    ) = ('0' as i32 + ('0' as i32 + 10 as libc::c_int - 1 as libc::c_int)
                    - ch) as libc::c_char;
            }
        }
        i -= 1;
        let ref mut fresh2 = *b.offset(i as isize);
        *fresh2 += 1;
        *fresh2;
    }
    dpb = -(1 as libc::c_int) as libc::c_short;
    dpa = dpb;
    i = 0 as libc::c_int;
    while *a.offset(i as isize) as libc::c_int
        != '\0' as i32 as libc::c_char as libc::c_int
    {
        if *a.offset(i as isize) as libc::c_int == '.' as i32 {
            dpa = i as libc::c_short;
        }
        i += 1;
        i;
    }
    lena = i as libc::c_short;
    if (dpa as libc::c_int) < 0 as libc::c_int {
        dpa = i as libc::c_short;
    }
    loop {
        i = 0 as libc::c_int;
        while *b.offset(i as isize) as libc::c_int
            != '\0' as i32 as libc::c_char as libc::c_int
        {
            if *b.offset(i as isize) as libc::c_int == '.' as i32 {
                dpb = i as libc::c_short;
            }
            i += 1;
            i;
        }
        lenb = i as libc::c_short;
        if (dpb as libc::c_int) < 0 as libc::c_int {
            dpb = i as libc::c_short;
        }
        if i == 1 as libc::c_int {
            if *b.offset(0 as libc::c_int as isize) as libc::c_int
                == '0' as i32 + 1 as libc::c_int
                && sign as libc::c_int == 0 as libc::c_int
                && dpa as libc::c_int > 0 as libc::c_int
            {
                i = dpa as libc::c_int - 1 as libc::c_int;
                loop {
                    let ref mut fresh3 = *a.offset(i as isize);
                    *fresh3 += 1;
                    if !(*fresh3 as libc::c_int
                        > '0' as i32 + 10 as libc::c_int - 1 as libc::c_int)
                    {
                        break;
                    }
                    let fresh4 = i;
                    i = i - 1;
                    *a.offset(fresh4 as isize) = '0' as i32 as libc::c_char;
                    if i < 0 as libc::c_int {
                        i = lena as libc::c_int;
                        while i >= 0 as libc::c_int {
                            *a
                                .offset(
                                    (i + 1 as libc::c_int) as isize,
                                ) = *a.offset(i as isize);
                            i -= 1;
                            i;
                        }
                        *a
                            .offset(
                                0 as libc::c_int as isize,
                            ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
                        return strlen(a) as libc::c_short;
                    }
                }
                return strlen(a) as libc::c_short;
            }
        }
        if !(lenb as libc::c_int - dpb as libc::c_int
            > lena as libc::c_int - dpa as libc::c_int)
        {
            break;
        }
        j = dpa as libc::c_int - dpb as libc::c_int;
        if lenb as libc::c_int + j > 256 as libc::c_int {
            i = 256 as libc::c_int - j;
            if (*b.offset(i as isize) as libc::c_int)
                < '0' as i32 + 10 as libc::c_int / 2 as libc::c_int
            {
                *b.offset(i as isize) = '\0' as i32 as libc::c_char;
                lenb -= 1;
                lenb;
                loop {
                    i -= 1;
                    if !(*b.offset(i as isize) as libc::c_int == '0' as i32) {
                        break;
                    }
                    *b.offset(i as isize) = '\0' as i32 as libc::c_char;
                    lenb -= 1;
                    lenb;
                }
            } else {
                loop {
                    if i >= dpb as libc::c_int {
                        *b.offset(i as isize) = '\0' as i32 as libc::c_char;
                        lenb -= 1;
                        lenb;
                    } else {
                        *b.offset(i as isize) = '0' as i32 as libc::c_char;
                    }
                    i -= 1;
                    if i < 0 as libc::c_int {
                        i = lenb as libc::c_int;
                        while i >= 0 as libc::c_int {
                            *b
                                .offset(
                                    (i + 1 as libc::c_int) as isize,
                                ) = *b.offset(i as isize);
                            i -= 1;
                            i;
                        }
                        *b
                            .offset(
                                0 as libc::c_int as isize,
                            ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
                        lenb += 1;
                        dpb = lenb;
                        break;
                    } else {
                        ch = *b.offset(i as isize) as libc::c_int;
                        if ch == '.' as i32 {
                            dpb = i as libc::c_short;
                        } else {
                            if !(ch < '0' as i32 + 10 as libc::c_int - 1 as libc::c_int
                                && ch >= '0' as i32)
                            {
                                continue;
                            }
                            ch += 1;
                            *b.offset(i as isize) = ch as libc::c_char;
                            break;
                        }
                    }
                }
            }
        } else {
            i = lena as libc::c_int - dpa as libc::c_int + dpb as libc::c_int;
            lenb = i as libc::c_short;
            j = lena as libc::c_int;
            loop {
                let fresh5 = i;
                i = i + 1;
                let fresh6 = j;
                j = j + 1;
                let ref mut fresh7 = *a.offset(fresh6 as isize);
                *fresh7 = *b.offset(fresh5 as isize);
                if !(*fresh7 as libc::c_int
                    != '\0' as i32 as libc::c_char as libc::c_int)
                {
                    break;
                }
            }
            j -= 1;
            lena = j as libc::c_short;
            *b.offset(lenb as isize) = '\0' as i32 as libc::c_char;
            break;
        }
    }
    i = dpa as libc::c_int - dpb as libc::c_int;
    if i < 0 as libc::c_int {
        j = lena as libc::c_int;
        lena = (lena as libc::c_int - i) as libc::c_short;
        i = lena as libc::c_int;
        if i > 256 as libc::c_int - 2 as libc::c_int {
            return -(92 as libc::c_int) as libc::c_short;
        }
        ch = if sign as libc::c_int >= 0 as libc::c_int {
            '0' as i32
        } else {
            '0' as i32 + 10 as libc::c_int - 1 as libc::c_int
        };
        while j >= 0 as libc::c_int {
            let fresh8 = j;
            j = j - 1;
            let fresh9 = i;
            i = i - 1;
            *a.offset(fresh9 as isize) = *a.offset(fresh8 as isize);
        }
        while i >= 0 as libc::c_int {
            let fresh10 = i;
            i = i - 1;
            *a.offset(fresh10 as isize) = ch as libc::c_char;
        }
        dpa = dpb;
    } else if i > 0 as libc::c_int {
        j = lenb as libc::c_int;
        i += lenb as libc::c_int;
        lenb = i as libc::c_short;
        if lenb as libc::c_int > 256 as libc::c_int - 2 as libc::c_int {
            return -(92 as libc::c_int) as libc::c_short;
        }
        ch = if sign as libc::c_int <= 0 as libc::c_int {
            '0' as i32
        } else {
            '0' as i32 + 10 as libc::c_int - 1 as libc::c_int
        };
        while j >= 0 as libc::c_int {
            let fresh11 = j;
            j = j - 1;
            let fresh12 = i;
            i = i - 1;
            *b.offset(fresh12 as isize) = *b.offset(fresh11 as isize);
        }
        while i >= 0 as libc::c_int {
            let fresh13 = i;
            i = i - 1;
            *b.offset(fresh13 as isize) = ch as libc::c_char;
        }
        dpb = dpa;
    }
    carry = 0 as libc::c_int;
    i = lenb as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        ch = *a.offset(i as isize) as libc::c_int;
        if !(ch == '.' as i32) {
            ch += *b.offset(i as isize) as libc::c_int - '0' as i32 + carry;
            carry = (ch > '0' as i32 + 10 as libc::c_int - 1 as libc::c_int)
                as libc::c_int;
            if carry != 0 {
                ch -= 10 as libc::c_int;
            }
            *a.offset(i as isize) = ch as libc::c_char;
        }
        i -= 1;
        i;
    }
    while *a.offset(lena as isize) as libc::c_int
        != '\0' as i32 as libc::c_char as libc::c_int
    {
        lena += 1;
        lena;
    }
    if carry != 0 {
        lena += 1;
        i = lena as libc::c_int;
        if i > 256 as libc::c_int - 2 as libc::c_int {
            return -(92 as libc::c_int) as libc::c_short;
        }
        while i > 0 as libc::c_int {
            *a.offset(i as isize) = *a.offset((i - 1 as libc::c_int) as isize);
            i -= 1;
            i;
        }
        *a
            .offset(
                0 as libc::c_int as isize,
            ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
    }
    if sign != 0 {
        if *a.offset(0 as libc::c_int as isize) as libc::c_int
            == '0' as i32 + 1 as libc::c_int
        {
            *a.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        } else {
            i = 0 as libc::c_int;
            carry = 0 as libc::c_int;
            loop {
                i += 1;
                ch = *a.offset(i as isize) as libc::c_int;
                if !(ch != '\0' as i32 as libc::c_char as libc::c_int) {
                    break;
                }
                if ch != '.' as i32 {
                    *a
                        .offset(
                            i as isize,
                        ) = ('0' as i32
                        + ('0' as i32 + 10 as libc::c_int - 1 as libc::c_int) - ch)
                        as libc::c_char;
                }
            }
            loop {
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
                if !(*a.offset(i as isize) as libc::c_int != '.' as i32) {
                    continue;
                }
                let ref mut fresh14 = *a.offset(i as isize);
                *fresh14 += 1;
                if *fresh14 as libc::c_int
                    <= '0' as i32 + 10 as libc::c_int - 1 as libc::c_int
                {
                    break;
                }
                *a.offset(i as isize) = '0' as i32 as libc::c_char;
            }
            mi = 1 as libc::c_int;
            *a.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        }
        while *a.offset(mi as isize) as libc::c_int == '0' as i32 {
            memmove(
                &mut *a.offset(mi as isize) as *mut libc::c_char as *mut libc::c_void,
                &mut *a.offset((mi + 1 as libc::c_int) as isize) as *mut libc::c_char
                    as *const libc::c_void,
                strlen(&mut *a.offset(mi as isize)),
            );
            dpa -= 1;
            dpa;
        }
        if (dpa as libc::c_int) < 0 as libc::c_int {
            dpa = 0 as libc::c_int as libc::c_short;
        }
    }
    i = dpa as libc::c_int;
    while *a.offset(i as isize) as libc::c_int
        != '\0' as i32 as libc::c_char as libc::c_int
    {
        i += 1;
        i;
    }
    i -= 1;
    if i > dpa as libc::c_int {
        while *a.offset(i as isize) as libc::c_int == '0' as i32 {
            let fresh15 = i;
            i = i - 1;
            *a.offset(fresh15 as isize) = '\0' as i32 as libc::c_char;
        }
    }
    if *a.offset(i as isize) as libc::c_int == '.' as i32 {
        *a.offset(i as isize) = '\0' as i32 as libc::c_char;
    }
    if mi != 0 {
        if *a.offset(0 as libc::c_int as isize) as libc::c_int != '0' as i32 {
            i = 0 as libc::c_int;
            loop {
                let fresh16 = i;
                i = i + 1;
                if !(*a.offset(fresh16 as isize) as libc::c_int
                    != '\0' as i32 as libc::c_char as libc::c_int)
                {
                    break;
                }
            }
            while i > 0 as libc::c_int {
                *a.offset(i as isize) = *a.offset((i - 1 as libc::c_int) as isize);
                i -= 1;
                i;
            }
        }
        *a.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
    }
    if *a.offset(mi as isize) as libc::c_int
        == '\0' as i32 as libc::c_char as libc::c_int
    {
        *a.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        *a.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    return strlen(a) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn runtime_mul(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
) -> libc::c_short {
    let mut current_block: u64;
    let mut c: [libc::c_char; 514] = [0; 514];
    let mut alen: libc::c_short = 0;
    let mut blen: libc::c_short = 0;
    let mut clen: libc::c_short = 0;
    let mut mi: libc::c_short = 0;
    let mut tmpx: libc::c_short = 0;
    let mut acur: libc::c_int = 0;
    let mut bcur: libc::c_int = 0;
    let mut ccur: libc::c_int = 0;
    let mut carry: libc::c_int = 0;
    if *b.offset(1 as libc::c_int as isize) as libc::c_int
        == '\0' as i32 as libc::c_char as libc::c_int
    {
        if *b.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
            *a.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
            *a.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            return 1 as libc::c_int as libc::c_short;
        }
        if *b.offset(0 as libc::c_int as isize) as libc::c_int
            == '0' as i32 + 1 as libc::c_int
        {
            return strlen(a) as libc::c_short;
        }
        if *b.offset(0 as libc::c_int as isize) as libc::c_int
            == '0' as i32 + 2 as libc::c_int
        {
            current_block = 970026022263154767;
        } else {
            current_block = 12199444798915819164;
        }
    } else {
        current_block = 12199444798915819164;
    }
    match current_block {
        12199444798915819164 => {
            if *a.offset(1 as libc::c_int as isize) as libc::c_int
                == '\0' as i32 as libc::c_char as libc::c_int
            {
                if *a.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
                    return strlen(a) as libc::c_short;
                }
                if *a.offset(0 as libc::c_int as isize) as libc::c_int
                    == '0' as i32 + 1 as libc::c_int
                {
                    strcpy(a, b);
                    return strlen(a) as libc::c_short;
                }
                if *a.offset(0 as libc::c_int as isize) as libc::c_int
                    == '0' as i32 + 2 as libc::c_int
                {
                    strcpy(a, b);
                    current_block = 970026022263154767;
                } else {
                    current_block = 13460095289871124136;
                }
            } else {
                current_block = 13460095289871124136;
            }
            match current_block {
                970026022263154767 => {}
                _ => {
                    mi = (*a.offset(0 as libc::c_int as isize) as libc::c_int
                        == '-' as i32) as libc::c_int as libc::c_short;
                    if mi != 0 {
                        *a
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '0' as i32 as libc::c_char;
                    }
                    if *b.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                    {
                        *b
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '0' as i32 as libc::c_char;
                        mi = (mi as libc::c_int ^ 0o1 as libc::c_int) as libc::c_short;
                    }
                    carry = 0 as libc::c_int;
                    alen = 0 as libc::c_int as libc::c_short;
                    while *a.offset(alen as isize) as libc::c_int
                        != '\0' as i32 as libc::c_char as libc::c_int
                    {
                        let ref mut fresh18 = *a.offset(alen as isize);
                        *fresh18 = (*fresh18 as libc::c_int - '0' as i32)
                            as libc::c_char;
                        let fresh19 = alen;
                        alen = alen + 1;
                        if *a.offset(fresh19 as isize) as libc::c_int
                            == '.' as i32 - '0' as i32
                        {
                            carry = alen as libc::c_int;
                        }
                    }
                    carry -= 1;
                    if carry < 0 as libc::c_int {
                        carry = alen as libc::c_int;
                        let fresh20 = alen;
                        alen = alen + 1;
                        *a
                            .offset(
                                fresh20 as isize,
                            ) = ('.' as i32 - '0' as i32) as libc::c_char;
                        *a.offset(alen as isize) = 0 as libc::c_int as libc::c_char;
                    }
                    ccur = 0 as libc::c_int;
                    blen = 0 as libc::c_int as libc::c_short;
                    while *b.offset(blen as isize) as libc::c_int
                        != '\0' as i32 as libc::c_char as libc::c_int
                    {
                        let ref mut fresh21 = *b.offset(blen as isize);
                        *fresh21 = (*fresh21 as libc::c_int - '0' as i32)
                            as libc::c_char;
                        let fresh22 = blen;
                        blen = blen + 1;
                        if *b.offset(fresh22 as isize) as libc::c_int
                            == '.' as i32 - '0' as i32
                        {
                            ccur = blen as libc::c_int;
                        }
                    }
                    ccur -= 1;
                    if ccur < 0 as libc::c_int {
                        ccur = blen as libc::c_int;
                        let fresh23 = blen;
                        blen = blen + 1;
                        *b
                            .offset(
                                fresh23 as isize,
                            ) = ('.' as i32 - '0' as i32) as libc::c_char;
                        *b.offset(blen as isize) = 0 as libc::c_int as libc::c_char;
                    }
                    carry += ccur;
                    if carry > 256 as libc::c_int - 3 as libc::c_int {
                        *a
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                        return -(92 as libc::c_int) as libc::c_short;
                    }
                    clen = (alen as libc::c_int + blen as libc::c_int) as libc::c_short;
                    ccur = clen as libc::c_int;
                    while ccur >= 0 as libc::c_int {
                        let fresh24 = ccur;
                        ccur = ccur - 1;
                        c[fresh24 as usize] = 0 as libc::c_int as libc::c_char;
                    }
                    c[carry as usize] = ('.' as i32 - '0' as i32) as libc::c_char;
                    bcur = blen as libc::c_int;
                    clen = (alen as libc::c_int + blen as libc::c_int - 1 as libc::c_int)
                        as libc::c_short;
                    carry = 0 as libc::c_int;
                    while bcur > 0 as libc::c_int {
                        bcur -= 1;
                        if *b.offset(bcur as isize) as libc::c_int
                            == '.' as i32 - '0' as i32
                        {
                            continue;
                        }
                        if c[clen as usize] as libc::c_int == '.' as i32 - '0' as i32 {
                            clen -= 1;
                            clen;
                        }
                        acur = alen as libc::c_int;
                        let fresh25 = clen;
                        clen = clen - 1;
                        ccur = fresh25 as libc::c_int;
                        while acur > 0 as libc::c_int {
                            acur -= 1;
                            if *a.offset(acur as isize) as libc::c_int
                                == '.' as i32 - '0' as i32
                            {
                                continue;
                            }
                            ccur -= 1;
                            if c[ccur as usize] as libc::c_int == '.' as i32 - '0' as i32
                            {
                                ccur -= 1;
                                ccur;
                            }
                            tmpx = (*a.offset(acur as isize) as libc::c_int
                                * *b.offset(bcur as isize) as libc::c_int
                                + c[ccur as usize] as libc::c_int + carry) as libc::c_short;
                            carry = tmpx as libc::c_int / 10 as libc::c_int;
                            c[ccur
                                as usize] = (tmpx as libc::c_int % 10 as libc::c_int)
                                as libc::c_char;
                        }
                        while carry != 0 {
                            ccur -= 1;
                            if c[ccur as usize] as libc::c_int == '.' as i32 - '0' as i32
                            {
                                ccur -= 1;
                                ccur;
                            }
                            c[ccur
                                as usize] = (c[ccur as usize] as libc::c_int + carry)
                                as libc::c_char;
                            if c[ccur as usize] as libc::c_int >= 10 as libc::c_int {
                                c[ccur
                                    as usize] = (c[ccur as usize] as libc::c_int
                                    - 10 as libc::c_int) as libc::c_char;
                                carry = 1 as libc::c_int;
                            } else {
                                carry = 0 as libc::c_int;
                            }
                        }
                    }
                    alen = (alen as libc::c_int + blen as libc::c_int) as libc::c_short;
                    acur = alen as libc::c_int;
                    clen = acur as libc::c_short;
                    ccur = clen as libc::c_int;
                    *a.offset(ccur as isize) = '\0' as i32 as libc::c_char;
                    loop {
                        ccur -= 1;
                        if !(ccur >= 0 as libc::c_int) {
                            break;
                        }
                        if (c[ccur as usize] as libc::c_int) < 10 as libc::c_int {
                            *a
                                .offset(
                                    ccur as isize,
                                ) = (c[ccur as usize] as libc::c_int + '0' as i32)
                                as libc::c_char;
                        } else {
                            alen = ccur as libc::c_short;
                            *a.offset(alen as isize) = '.' as i32 as libc::c_char;
                        }
                    }
                    if acur > 256 as libc::c_int {
                        acur = 256 as libc::c_int;
                        alen = acur as libc::c_short;
                        if *a.offset(alen as isize) as libc::c_int
                            >= '0' as i32 + 10 as libc::c_int / 2 as libc::c_int
                        {
                            let mut l1: libc::c_int = 256 as libc::c_int;
                            if *a.offset(l1 as isize) as libc::c_int
                                >= '0' as i32 + 10 as libc::c_int / 2 as libc::c_int
                            {
                                loop {
                                    l1 -= 1;
                                    if *a.offset(l1 as isize) as libc::c_int == '.' as i32 {
                                        l1 -= 1;
                                        l1;
                                    }
                                    if l1
                                        < (*a.offset(0 as libc::c_int as isize) as libc::c_int
                                            == '-' as i32) as libc::c_int
                                    {
                                        l1 = 256 as libc::c_int;
                                        while l1 > 0 as libc::c_int {
                                            *a
                                                .offset(
                                                    l1 as isize,
                                                ) = *a.offset((l1 - 1 as libc::c_int) as isize);
                                            l1 -= 1;
                                            l1;
                                        }
                                        *a
                                            .offset(
                                                (*a.offset(0 as libc::c_int as isize) as libc::c_int
                                                    == '-' as i32) as libc::c_int as isize,
                                            ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
                                        break;
                                    } else {
                                        let ref mut fresh26 = *a.offset(l1 as isize);
                                        *fresh26 += 1;
                                        if !(*fresh26 as libc::c_int
                                            == '0' as i32 + 10 as libc::c_int - 1 as libc::c_int
                                                + 1 as libc::c_int)
                                        {
                                            break;
                                        }
                                        *a.offset(l1 as isize) = '0' as i32 as libc::c_char;
                                    }
                                }
                            }
                        }
                        *a.offset(acur as isize) = '\0' as i32 as libc::c_char;
                    }
                    if acur >= alen as libc::c_int {
                        loop {
                            acur -= 1;
                            if !(*a.offset(acur as isize) as libc::c_int == '0' as i32) {
                                break;
                            }
                            *a.offset(acur as isize) = '\0' as i32 as libc::c_char;
                        }
                    }
                    if *a.offset(acur as isize) as libc::c_int == '.' as i32 {
                        *a.offset(acur as isize) = '\0' as i32 as libc::c_char;
                    }
                    while *a.offset(mi as isize) as libc::c_int == '0' as i32 {
                        acur = mi as libc::c_int;
                        loop {
                            let ref mut fresh27 = *a.offset(acur as isize);
                            *fresh27 = *a.offset((acur + 1 as libc::c_int) as isize);
                            if !(*fresh27 as libc::c_int
                                != '\0' as i32 as libc::c_char as libc::c_int)
                            {
                                break;
                            }
                            acur += 1;
                            acur;
                        }
                    }
                    if *a.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32 as libc::c_char as libc::c_int
                    {
                        *a
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '0' as i32 as libc::c_char;
                        *a
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                        mi = 0 as libc::c_int as libc::c_short;
                    }
                    if mi != 0 {
                        if *a.offset(0 as libc::c_int as isize) as libc::c_int
                            != '0' as i32
                        {
                            acur = clen as libc::c_int;
                            while acur > 0 as libc::c_int {
                                *a
                                    .offset(
                                        acur as isize,
                                    ) = *a.offset((acur - 1 as libc::c_int) as isize);
                                acur -= 1;
                                acur;
                            }
                        }
                        *a
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '-' as i32 as libc::c_char;
                    }
                    return strlen(a) as libc::c_short;
                }
            }
        }
        _ => {}
    }
    acur = 0 as libc::c_int;
    loop {
        acur += 1;
        if !(*a.offset(acur as isize) as libc::c_int
            != '\0' as i32 as libc::c_char as libc::c_int)
        {
            break;
        }
    }
    mi = (*a.offset((acur - 1 as libc::c_int) as isize) as libc::c_int
        == '0' as i32 + 10 as libc::c_int / 2 as libc::c_int) as libc::c_int
        as libc::c_short;
    carry = 0 as libc::c_int;
    ccur = acur;
    while acur > 0 as libc::c_int {
        acur -= 1;
        bcur = *a.offset(acur as isize) as libc::c_int;
        if bcur < '0' as i32 {
            continue;
        }
        bcur = bcur * 2 as libc::c_int - '0' as i32 + carry;
        carry = 0 as libc::c_int;
        if bcur > '0' as i32 + 10 as libc::c_int - 1 as libc::c_int {
            carry = 1 as libc::c_int;
            bcur -= 10 as libc::c_int;
        }
        *a.offset(acur as isize) = bcur as libc::c_char;
    }
    if carry != 0 {
        acur = ccur;
        if acur > 256 as libc::c_int - 1 as libc::c_int {
            return -(92 as libc::c_int) as libc::c_short;
        }
        while acur >= 0 as libc::c_int {
            *a.offset((acur + 1 as libc::c_int) as isize) = *a.offset(acur as isize);
            acur -= 1;
            acur;
        }
        *a
            .offset(
                (*a.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
                    as libc::c_int as isize,
            ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
    }
    if mi != 0 {
        if carry != 0 {
            ccur += 1;
            ccur;
        }
        acur = 0 as libc::c_int;
        while acur < ccur {
            let fresh17 = acur;
            acur = acur + 1;
            if *a.offset(fresh17 as isize) as libc::c_int == '.' as i32 {
                ccur -= 1;
                *a.offset(ccur as isize) = '\0' as i32 as libc::c_char;
                if acur == ccur {
                    ccur -= 1;
                    *a.offset(ccur as isize) = '\0' as i32 as libc::c_char;
                    return strlen(a) as libc::c_short;
                }
            }
        }
    }
    return strlen(a) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn runtime_div(
    mut uu: *mut libc::c_char,
    mut v: *mut libc::c_char,
    mut typ: libc::c_short,
) -> libc::c_short {
    let mut q: [libc::c_char; 258] = [0; 258];
    let mut u: [libc::c_char; 514] = [0; 514];
    let mut vv: [libc::c_char; 257] = [0; 257];
    let mut d: libc::c_short = 0;
    let mut d1: libc::c_short = 0;
    let mut k1: libc::c_short = 0;
    let mut m: libc::c_short = 0;
    let mut ulen: libc::c_short = 0;
    let mut vlen: libc::c_short = 0;
    let mut dpu: libc::c_short = 0;
    let mut dpv: libc::c_short = 0;
    let mut guess: libc::c_short = 0;
    let mut mi: libc::c_short = 0;
    let mut plus: libc::c_short = 0;
    let mut v1: libc::c_short = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut carry: libc::c_int = 0 as libc::c_int;
    if *v.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && *v.offset(1 as libc::c_int as isize) == 0
    {
        return -(9 as libc::c_int) as libc::c_short;
    }
    if *uu.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        return 1 as libc::c_int as libc::c_short;
    }
    strcpy(u.as_mut_ptr(), uu);
    mi = 0 as libc::c_int as libc::c_short;
    plus = 0 as libc::c_int as libc::c_short;
    if typ as libc::c_int != 15 as libc::c_int {
        if u[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
            u[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            mi = 1 as libc::c_int as libc::c_short;
        }
        if *v.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            *v.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
            mi = (mi as libc::c_int ^ 0o1 as libc::c_int) as libc::c_short;
        }
    } else {
        strcpy(vv.as_mut_ptr(), v);
        if u[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
            u[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            plus = 1 as libc::c_int as libc::c_short;
        }
        if *v.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            *v.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
            mi = 1 as libc::c_int as libc::c_short;
            plus = (plus as libc::c_int ^ 0o1 as libc::c_int) as libc::c_short;
        }
    }
    i = 0 as libc::c_int;
    dpv = -(1 as libc::c_int) as libc::c_short;
    k = 0 as libc::c_int;
    loop {
        j = *v.offset(i as isize) as libc::c_int;
        if !(j != '\0' as i32 as libc::c_char as libc::c_int) {
            break;
        }
        j -= '0' as i32;
        if j == '.' as i32 - '0' as i32 {
            dpv = i as libc::c_short;
        }
        if j == 0 as libc::c_int {
            k += 1;
            k;
        }
        let fresh28 = i;
        i = i + 1;
        *v.offset(fresh28 as isize) = j as libc::c_char;
    }
    vlen = i as libc::c_short;
    *v.offset(vlen as isize) = 0 as libc::c_int as libc::c_char;
    *v.offset((i + 1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
    *v.offset((i + 2 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
    if *v.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
        while i >= 0 as libc::c_int {
            *v.offset((i + 1 as libc::c_int) as isize) = *v.offset(i as isize);
            i -= 1;
            i;
        }
        *v.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        dpv += 1;
        dpv;
    } else {
        vlen -= 1;
        vlen;
    }
    d1 = 0 as libc::c_int as libc::c_short;
    i = 0 as libc::c_int;
    dpu = -(1 as libc::c_int) as libc::c_short;
    while u[i as usize] as libc::c_int != '\0' as i32 as libc::c_char as libc::c_int {
        u[i as usize] = (u[i as usize] as libc::c_int - '0' as i32) as libc::c_char;
        if u[i as usize] as libc::c_int == '.' as i32 - '0' as i32 {
            dpu = i as libc::c_short;
        }
        i += 1;
        i;
    }
    if (dpu as libc::c_int) < 0 as libc::c_int {
        let fresh29 = i;
        i = i + 1;
        dpu = fresh29 as libc::c_short;
        u[dpu as usize] = ('.' as i32 - '0' as i32) as libc::c_char;
    }
    ulen = i as libc::c_short;
    while i < 256 as libc::c_int * 2 as libc::c_int {
        let fresh30 = i;
        i = i + 1;
        u[fresh30 as usize] = 0 as libc::c_int as libc::c_char;
    }
    i = ulen as libc::c_int;
    if u[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {
        while i >= 0 as libc::c_int {
            u[(i + 1 as libc::c_int) as usize] = u[i as usize];
            i -= 1;
            i;
        }
        u[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        dpu += 1;
        dpu;
    } else {
        ulen -= 1;
        ulen;
    }
    if vlen as libc::c_int + (*partab.jobtab).precision as libc::c_int
        > 256 as libc::c_int
        && (dpv as libc::c_int + (*partab.jobtab).precision as libc::c_int)
            < vlen as libc::c_int
    {
        vlen = (vlen as libc::c_int - (*partab.jobtab).precision as libc::c_int)
            as libc::c_short;
    }
    if dpv as libc::c_int > 0 as libc::c_int {
        d1 = (vlen as libc::c_int - dpv as libc::c_int) as libc::c_short;
        i = dpv as libc::c_int;
        while i < vlen as libc::c_int {
            *v.offset(i as isize) = *v.offset((i + 1 as libc::c_int) as isize);
            i += 1;
            i;
        }
        vlen -= 1;
        vlen;
        while *v.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            i = 1 as libc::c_int;
            while i <= vlen as libc::c_int {
                *v.offset(i as isize) = *v.offset((i + 1 as libc::c_int) as isize);
                i += 1;
                i;
            }
            vlen -= 1;
            vlen;
        }
        *v
            .offset(
                (vlen as libc::c_int + 1 as libc::c_int) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        *v
            .offset(
                (vlen as libc::c_int + 2 as libc::c_int) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        i = dpu as libc::c_int;
        j = 0 as libc::c_int;
        while j < d1 as libc::c_int {
            if i >= ulen as libc::c_int {
                u[(i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
                ulen += 1;
                ulen;
            }
            u[i as usize] = u[(i + 1 as libc::c_int) as usize];
            i += 1;
            i;
            j += 1;
            j;
        }
        u[i as usize] = ('.' as i32 - '0' as i32) as libc::c_char;
        dpu = i as libc::c_short;
    }
    d = (dpu as libc::c_int + 1 as libc::c_int - ulen as libc::c_int) as libc::c_short;
    if dpv as libc::c_int > dpu as libc::c_int {
        d = (d as libc::c_int + (dpv as libc::c_int - dpu as libc::c_int))
            as libc::c_short;
    }
    if typ as libc::c_int == 13 as libc::c_int {
        d = (d as libc::c_int + (*partab.jobtab).precision as libc::c_int)
            as libc::c_short;
    }
    if d as libc::c_int + ulen as libc::c_int > 256 as libc::c_int {
        return -(92 as libc::c_int) as libc::c_short;
    }
    while d as libc::c_int > 0 as libc::c_int {
        ulen += 1;
        u[ulen as usize] = 0 as libc::c_int as libc::c_char;
        d -= 1;
        d;
    }
    d = (10 as libc::c_int
        / (*v.offset(1 as libc::c_int as isize) as libc::c_int + 1 as libc::c_int))
        as libc::c_short;
    if d as libc::c_int > 1 as libc::c_int {
        i = ulen as libc::c_int;
        carry = 0 as libc::c_int;
        while i > 0 as libc::c_int {
            if u[i as usize] as libc::c_int != '.' as i32 - '0' as i32 {
                carry += u[i as usize] as libc::c_int * d as libc::c_int;
                u[i as usize] = (carry % 10 as libc::c_int) as libc::c_char;
                carry = carry / 10 as libc::c_int;
            }
            i -= 1;
            i;
        }
        u[0 as libc::c_int as usize] = carry as libc::c_char;
        i = vlen as libc::c_int;
        carry = 0 as libc::c_int;
        while i > 0 as libc::c_int {
            carry += *v.offset(i as isize) as libc::c_int * d as libc::c_int;
            *v.offset(i as isize) = (carry % 10 as libc::c_int) as libc::c_char;
            carry = carry / 10 as libc::c_int;
            i -= 1;
            i;
        }
        *v.offset(0 as libc::c_int as isize) = carry as libc::c_char;
    }
    j = 0 as libc::c_int;
    m = (ulen as libc::c_int - vlen as libc::c_int + 1 as libc::c_int) as libc::c_short;
    if m as libc::c_int <= dpu as libc::c_int {
        m = (dpu as libc::c_int + 1 as libc::c_int) as libc::c_short;
    }
    i = 0 as libc::c_int;
    while i <= m as libc::c_int {
        let fresh31 = i;
        i = i + 1;
        q[fresh31 as usize] = '0' as i32 as libc::c_char;
    }
    if typ as libc::c_int == 15 as libc::c_int {
        m = (dpu as libc::c_int - vlen as libc::c_int) as libc::c_short;
    }
    v1 = *v.offset(1 as libc::c_int as isize) as libc::c_short;
    while j < m as libc::c_int {
        if u[j as usize] as libc::c_int != '.' as i32 - '0' as i32 {
            k = u[j as usize] as libc::c_int * 10 as libc::c_int
                + (if u[(j + 1 as libc::c_int) as usize] as libc::c_int
                    == '.' as i32 - '0' as i32
                {
                    u[(j + 2 as libc::c_int) as usize] as libc::c_int
                } else {
                    u[(j + 1 as libc::c_int) as usize] as libc::c_int
                });
            if k == 0 as libc::c_int {
                j += 1;
                j;
            } else {
                k1 = (if u[(j + 1 as libc::c_int) as usize] as libc::c_int
                    == '.' as i32 - '0' as i32
                    || u[(j + 2 as libc::c_int) as usize] as libc::c_int
                        == '.' as i32 - '0' as i32
                {
                    u[(j + 3 as libc::c_int) as usize] as libc::c_int
                } else {
                    u[(j + 2 as libc::c_int) as usize] as libc::c_int
                }) as libc::c_short;
                guess = (if u[j as usize] as libc::c_int == v1 as libc::c_int {
                    10 as libc::c_int - 1 as libc::c_int
                } else {
                    k / v1 as libc::c_int
                }) as libc::c_short;
                if *v.offset(2 as libc::c_int as isize) as libc::c_int
                    * guess as libc::c_int
                    > (k - guess as libc::c_int * v1 as libc::c_int) * 10 as libc::c_int
                        + k1 as libc::c_int
                {
                    guess -= 1;
                    guess;
                    if *v.offset(2 as libc::c_int as isize) as libc::c_int
                        * guess as libc::c_int
                        > (k - guess as libc::c_int * v1 as libc::c_int)
                            * 10 as libc::c_int + k1 as libc::c_int
                    {
                        guess -= 1;
                        guess;
                    }
                }
                i = vlen as libc::c_int;
                carry = 0 as libc::c_int;
                k = j + i;
                if j < dpu as libc::c_int && k >= dpu as libc::c_int {
                    k += 1;
                    k;
                }
                while k >= 0 as libc::c_int {
                    if u[k as usize] as libc::c_int == '.' as i32 - '0' as i32 {
                        k -= 1;
                        k;
                    }
                    if i >= 0 as libc::c_int {
                        let fresh32 = i;
                        i = i - 1;
                        u[k
                            as usize] = (u[k as usize] as libc::c_int
                            - (*v.offset(fresh32 as isize) as libc::c_int
                                * guess as libc::c_int + carry)) as libc::c_char;
                    } else {
                        if carry == 0 as libc::c_int {
                            break;
                        }
                        u[k
                            as usize] = (u[k as usize] as libc::c_int - carry)
                            as libc::c_char;
                    }
                    carry = 0 as libc::c_int;
                    while (u[k as usize] as libc::c_int) < 0 as libc::c_int {
                        u[k
                            as usize] = (u[k as usize] as libc::c_int
                            + 10 as libc::c_int) as libc::c_char;
                        carry += 1;
                        carry;
                    }
                    k -= 1;
                    k;
                }
                if carry != 0 {
                    guess -= 1;
                    guess;
                    i = vlen as libc::c_int;
                    carry = 0 as libc::c_int;
                    k = j + i;
                    if j < dpu as libc::c_int && k >= dpu as libc::c_int {
                        k += 1;
                        k;
                    }
                    while k >= 0 as libc::c_int {
                        if u[k as usize] as libc::c_int == '.' as i32 - '0' as i32 {
                            k -= 1;
                            k;
                        }
                        if i >= 0 as libc::c_int {
                            let fresh33 = i;
                            i = i - 1;
                            u[k
                                as usize] = (u[k as usize] as libc::c_int
                                + (*v.offset(fresh33 as isize) as libc::c_int + carry))
                                as libc::c_char;
                        } else {
                            if carry == 0 as libc::c_int {
                                break;
                            }
                            u[k
                                as usize] = (u[k as usize] as libc::c_int + carry)
                                as libc::c_char;
                        }
                        carry = u[k as usize] as libc::c_int / 10 as libc::c_int;
                        u[k
                            as usize] = (u[k as usize] as libc::c_int
                            % 10 as libc::c_int) as libc::c_char;
                        k -= 1;
                        k;
                    }
                }
                let fresh34 = j;
                j = j + 1;
                q[fresh34
                    as usize] = (guess as libc::c_int + '0' as i32) as libc::c_char;
                u[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            }
        } else {
            let fresh35 = j;
            j = j + 1;
            q[fresh35 as usize] = '.' as i32 as libc::c_char;
        }
    }
    if typ as libc::c_int != 15 as libc::c_int {
        i = 0 as libc::c_int;
        while i <= m as libc::c_int {
            u[i as usize] = q[i as usize];
            if u[i as usize] as libc::c_int == '.' as i32 {
                dpv = i as libc::c_short;
            }
            i += 1;
            i;
        }
        k = vlen as libc::c_int;
        k1 = dpv;
        loop {
            let fresh36 = k;
            k = k - 1;
            if !(fresh36 > 0 as libc::c_int) {
                break;
            }
            while k1 as libc::c_int <= 0 as libc::c_int {
                m += 1;
                i = m as libc::c_int;
                while i > 0 as libc::c_int {
                    u[i as usize] = u[(i - 1 as libc::c_int) as usize];
                    i -= 1;
                    i;
                }
                k1 += 1;
                k1;
                u[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            }
            u[k1 as usize] = u[(k1 as libc::c_int - 1 as libc::c_int) as usize];
            k1 -= 1;
            u[k1 as usize] = '.' as i32 as libc::c_char;
            dpv = k1;
        }
        u[m as usize] = '\0' as i32 as libc::c_char;
        if typ as libc::c_int != 13 as libc::c_int {
            u[(dpv as libc::c_int + 1 as libc::c_int)
                as usize] = '\0' as i32 as libc::c_char;
        } else {
            k = dpv as libc::c_int + (*partab.jobtab).precision as libc::c_int;
            k1 = (u[(k + 1 as libc::c_int) as usize] as libc::c_int
                >= '0' as i32 + 10 as libc::c_int / 2 as libc::c_int) as libc::c_int
                as libc::c_short;
            u[(k + 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
            if k1 != 0 {
                loop {
                    if u[k as usize] as libc::c_int != '.' as i32 {
                        carry = (u[k as usize] as libc::c_int
                            == '0' as i32 + 10 as libc::c_int - 1 as libc::c_int)
                            as libc::c_int;
                        if carry != 0 {
                            u[k as usize] = '0' as i32 as libc::c_char;
                        } else {
                            u[k as usize] += 1;
                            u[k as usize];
                        }
                    }
                    k -= 1;
                    k;
                    if !(carry != 0) {
                        break;
                    }
                }
            }
        }
    } else {
        carry = 0 as libc::c_int;
        if d as libc::c_int > 1 as libc::c_int {
            i = 0 as libc::c_int;
            while i <= ulen as libc::c_int {
                if u[i as usize] as libc::c_int == '.' as i32 - '0' as i32 {
                    u[i as usize] = '.' as i32 as libc::c_char;
                    dpu = i as libc::c_short;
                } else {
                    j = carry + u[i as usize] as libc::c_int;
                    u[i as usize] = (j / d as libc::c_int + '0' as i32) as libc::c_char;
                    carry = j % d as libc::c_int * 10 as libc::c_int;
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i <= ulen as libc::c_int {
                if u[i as usize] as libc::c_int == '.' as i32 - '0' as i32 {
                    dpu = i as libc::c_short;
                    u[dpu as usize] = '.' as i32 as libc::c_char;
                } else {
                    u[i
                        as usize] = (u[i as usize] as libc::c_int + '0' as i32)
                        as libc::c_char;
                }
                i += 1;
                i;
            }
        }
        u[i as usize] = '\0' as i32 as libc::c_char;
        if d1 as libc::c_int > 0 as libc::c_int {
            u[(i + 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
            u[(i + 2 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
            i = dpu as libc::c_int;
            j = 0 as libc::c_int;
            while j < d1 as libc::c_int {
                u[i as usize] = u[(i - 1 as libc::c_int) as usize];
                i -= 1;
                i;
                j += 1;
                j;
            }
            u[i as usize] = '.' as i32 as libc::c_char;
        }
    }
    i = 0 as libc::c_int;
    k = -(1 as libc::c_int);
    while u[i as usize] as libc::c_int != '\0' as i32 as libc::c_char as libc::c_int {
        if u[i as usize] as libc::c_int == '.' as i32 {
            k = i;
        }
        i += 1;
        i;
    }
    i -= 1;
    i;
    if k >= 0 as libc::c_int {
        while u[i as usize] as libc::c_int == '0' as i32 && i > k {
            let fresh37 = i;
            i = i - 1;
            u[fresh37 as usize] = '\0' as i32 as libc::c_char;
        }
    }
    if u[i as usize] as libc::c_int == '.' as i32 {
        u[i as usize] = '\0' as i32 as libc::c_char;
    }
    while u[0 as libc::c_int as usize] as libc::c_int == '0' as i32 {
        i = 0 as libc::c_int;
        loop {
            u[i as usize] = u[(i + 1 as libc::c_int) as usize];
            if !(u[i as usize] as libc::c_int
                != '\0' as i32 as libc::c_char as libc::c_int)
            {
                break;
            }
            i += 1;
            i;
        }
    }
    if u[0 as libc::c_int as usize] as libc::c_int
        == '\0' as i32 as libc::c_char as libc::c_int
    {
        u[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
        u[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        mi = 0 as libc::c_int as libc::c_short;
    }
    if (mi as libc::c_int != 0 || plus as libc::c_int != 0)
        && u[0 as libc::c_int as usize] as libc::c_int != '0' as i32
    {
        if mi as libc::c_int != plus as libc::c_int {
            i = strlen(u.as_mut_ptr()) as libc::c_int + 1 as libc::c_int;
            loop {
                u[i as usize] = u[(i - 1 as libc::c_int) as usize];
                i -= 1;
                i;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
            u[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
        }
        if plus != 0 {
            let mut s: libc::c_short = runtime_add(u.as_mut_ptr(), vv.as_mut_ptr());
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
        }
    }
    strcpy(uu, u.as_mut_ptr());
    return strlen(uu) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn runtime_power(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
) -> libc::c_short {
    let mut s: libc::c_short = 0;
    let mut c: [libc::c_char; 258] = [0; 258];
    let mut d: [libc::c_char; 1028] = [0; 1028];
    let mut e: [libc::c_char; 258] = [0; 258];
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut Z: [libc::c_char; 258] = [0; 258];
    if *a.offset(1 as libc::c_int as isize) as libc::c_int
        == '\0' as i32 as libc::c_char as libc::c_int
    {
        if *a.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
            if *b.offset(1 as libc::c_int as isize) as libc::c_int
                == '\0' as i32 as libc::c_char as libc::c_int
                && *b.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
            {
                return -(9 as libc::c_int) as libc::c_short;
            }
            return 1 as libc::c_int as libc::c_short;
        }
        if *a.offset(0 as libc::c_int as isize) as libc::c_int
            == '0' as i32 + 1 as libc::c_int
        {
            return 1 as libc::c_int as libc::c_short;
        }
    }
    if *b.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        s = runtime_power(a, &mut *b.offset(1 as libc::c_int as isize));
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        strcpy(c.as_mut_ptr(), a);
        *a
            .offset(
                0 as libc::c_int as isize,
            ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
        *a.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        s = runtime_div(a, c.as_mut_ptr(), 13 as libc::c_int as libc::c_short);
        return s;
    }
    if *b.offset(1 as libc::c_int as isize) as libc::c_int
        == '\0' as i32 as libc::c_char as libc::c_int
    {
        's_107: {
            match *b.offset(0 as libc::c_int as isize) as libc::c_int {
                48 => {
                    *a
                        .offset(
                            0 as libc::c_int as isize,
                        ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
                    *a.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                }
                49 => {}
                50 => {
                    strcpy(c.as_mut_ptr(), a);
                    return runtime_mul(a, c.as_mut_ptr());
                }
                _ => {
                    break 's_107;
                }
            }
            return strlen(a) as libc::c_short;
        }
    }
    e[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    i = 0 as libc::c_int as libc::c_long;
    while *b.offset(i as isize) as libc::c_int
        != '\0' as i32 as libc::c_char as libc::c_int
    {
        if *b.offset(i as isize) as libc::c_int == '.' as i32 {
            if *a.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                return -(9 as libc::c_int) as libc::c_short;
            }
            if *b.offset((i + 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
                == '0' as i32 + 10 as libc::c_int / 2 as libc::c_int
                && *b.offset((i + 2 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int == '\0' as i32 as libc::c_char as libc::c_int
            {
                if i != 0 {
                    strcpy(c.as_mut_ptr(), b);
                    s = runtime_add(b, c.as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        return s;
                    }
                    s = runtime_power(a, b);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        return s;
                    }
                }
                s = g_sqrt(a) as libc::c_short;
                return s;
            }
            strcpy(e.as_mut_ptr(), &mut *b.offset(i as isize));
            *b.offset(i as isize) = '\0' as i32 as libc::c_char;
            break;
        } else {
            i += 1;
            i;
        }
    }
    strcpy(d.as_mut_ptr(), a);
    i = atoi(b) as libc::c_long;
    if i == 2147483647 as libc::c_int as libc::c_long {
        return -(92 as libc::c_int) as libc::c_short;
    }
    if i == 0 as libc::c_int as libc::c_long {
        *a
            .offset(
                0 as libc::c_int as isize,
            ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
        *a.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        j = 1 as libc::c_int as libc::c_long;
        while j < i {
            j = j * 2 as libc::c_int as libc::c_long;
            if j < 0 as libc::c_int as libc::c_long {
                return -(92 as libc::c_int) as libc::c_short;
            }
        }
        if i != j {
            j = j / 2 as libc::c_int as libc::c_long;
        }
        j = j / 2 as libc::c_int as libc::c_long;
        while j != 0 {
            strcpy(c.as_mut_ptr(), a);
            s = runtime_mul(a, c.as_mut_ptr());
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            if i & j != 0 {
                strcpy(c.as_mut_ptr(), d.as_mut_ptr());
                s = runtime_mul(a, c.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    return s;
                }
            }
            j = j / 2 as libc::c_int as libc::c_long;
        }
        if e[0 as libc::c_int as usize] as libc::c_int
            == '\0' as i32 as libc::c_char as libc::c_int
        {
            return strlen(a) as libc::c_short;
        }
    }
    Z[0 as libc::c_int as usize] = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
    Z[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    strcpy(c.as_mut_ptr(), e.as_mut_ptr());
    s = runtime_div(Z.as_mut_ptr(), c.as_mut_ptr(), 13 as libc::c_int as libc::c_short);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    i = 0 as libc::c_int as libc::c_long;
    loop {
        let fresh38 = i;
        i = i + 1;
        j = Z[fresh38 as usize] as libc::c_long;
        if j == '\0' as i32 as libc::c_char as libc::c_long {
            j = atoi(Z.as_mut_ptr()) as libc::c_long;
            if j == 2147483647 as libc::c_int as libc::c_long {
                return -(92 as libc::c_int) as libc::c_short;
            }
            break;
        } else {
            if j != '.' as i32 as libc::c_long {
                continue;
            }
            j = atoi(Z.as_mut_ptr()) as libc::c_long;
            if j == 2147483647 as libc::c_int as libc::c_long {
                return -(92 as libc::c_int) as libc::c_short;
            }
            if Z[i as usize] as libc::c_int
                == '0' as i32 + 10 as libc::c_int - 1 as libc::c_int
            {
                j += 1;
                j;
            }
            break;
        }
    }
    Z[0 as libc::c_int as usize] = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
    Z[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    s = itocstring(c.as_mut_ptr() as *mut u_char, j as libc::c_int) as libc::c_short;
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    s = runtime_div(Z.as_mut_ptr(), c.as_mut_ptr(), 13 as libc::c_int as libc::c_short);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    if strcmp(Z.as_mut_ptr(), e.as_mut_ptr()) == 0 as libc::c_int {
        strcpy(Z.as_mut_ptr(), d.as_mut_ptr());
        s = root(Z.as_mut_ptr(), j as libc::c_int) as libc::c_short;
        if (s as libc::c_int) < 0 as libc::c_int {
            s = runtime_mul(a, Z.as_mut_ptr());
            return s;
        }
    }
    Z[0 as libc::c_int as usize] = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
    Z[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    (*partab.jobtab)
        .precision = ((*partab.jobtab).precision as libc::c_int + 2 as libc::c_int)
        as libc::c_short;
    loop {
        c[0 as libc::c_int as usize] = ('0' as i32 + 2 as libc::c_int) as libc::c_char;
        c[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        s = runtime_mul(e.as_mut_ptr(), c.as_mut_ptr());
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        s = g_sqrt(d.as_mut_ptr()) as libc::c_short;
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        if e[0 as libc::c_int as usize] as libc::c_int == '0' as i32 + 1 as libc::c_int {
            e[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            strcpy(c.as_mut_ptr(), d.as_mut_ptr());
            s = runtime_mul(Z.as_mut_ptr(), c.as_mut_ptr());
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            roundit(Z.as_mut_ptr(), (*partab.jobtab).precision as libc::c_int);
        }
        i = 0 as libc::c_int as libc::c_long;
        j = (if d[0 as libc::c_int as usize] as libc::c_int
            == '0' as i32 + 1 as libc::c_int
        {
            '0' as i32
        } else {
            '0' as i32 + 10 as libc::c_int - 1 as libc::c_int
        }) as libc::c_long;
        loop {
            i += 1;
            i;
            if d[i as usize] as libc::c_long != j
                && d[i as usize] as libc::c_int != '.' as i32
            {
                break;
            }
        }
        if d[i as usize] as libc::c_int == '\0' as i32 as libc::c_char as libc::c_int
            || i > (*partab.jobtab).precision as libc::c_long
        {
            break;
        }
    }
    (*partab.jobtab)
        .precision = ((*partab.jobtab).precision as libc::c_int - 2 as libc::c_int)
        as libc::c_short;
    s = runtime_mul(a, Z.as_mut_ptr());
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    roundit(a, (*partab.jobtab).precision as libc::c_int + 1 as libc::c_int);
    return strlen(a) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn runtime_comp(
    mut s: *mut libc::c_char,
    mut t: *mut libc::c_char,
) -> libc::c_short {
    let mut s1: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    s1 = *s.offset(0 as libc::c_int as isize) as libc::c_int;
    t1 = *t.offset(0 as libc::c_int as isize) as libc::c_int;
    if s1 != t1 {
        if s1 == '-' as i32 {
            return 1 as libc::c_int as libc::c_short;
        }
        if t1 == '-' as i32 {
            return 0 as libc::c_int as libc::c_short;
        }
        if s1 == '.' as i32 && t1 == '0' as i32 {
            return 0 as libc::c_int as libc::c_short;
        }
        if t1 == '.' as i32 && s1 == '0' as i32 {
            return 1 as libc::c_int as libc::c_short;
        }
    }
    if t1 == '-' as i32 {
        let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
        a = &mut *t.offset(1 as libc::c_int as isize) as *mut libc::c_char;
        t = &mut *s.offset(1 as libc::c_int as isize) as *mut libc::c_char;
        s = a;
    }
    s1 = 0 as libc::c_int;
    while *s.offset(s1 as isize) as libc::c_int > '.' as i32 {
        s1 += 1;
        s1;
    }
    t1 = 0 as libc::c_int;
    while *t.offset(t1 as isize) as libc::c_int > '.' as i32 {
        t1 += 1;
        t1;
    }
    if t1 > s1 {
        return 1 as libc::c_int as libc::c_short;
    }
    if t1 < s1 {
        return 0 as libc::c_int as libc::c_short;
    }
    while *t as libc::c_int == *s as libc::c_int {
        if *t as libc::c_int == '\0' as i32 as libc::c_char as libc::c_int {
            return 0 as libc::c_int as libc::c_short;
        }
        t = t.offset(1);
        t;
        s = s.offset(1);
        s;
    }
    if *t as libc::c_int > *s as libc::c_int {
        return 1 as libc::c_int as libc::c_short;
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn g_sqrt(mut a: *mut libc::c_char) -> libc::c_int {
    let mut tmp1: [libc::c_char; 258] = [0; 258];
    let mut tmp2: [libc::c_char; 258] = [0; 258];
    let mut XX: [libc::c_char; 258] = [0; 258];
    let mut XXX: [libc::c_char; 258] = [0; 258];
    if *a.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        return 1 as libc::c_int;
    }
    if *a.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        return -(9 as libc::c_int);
    }
    strcpy(XX.as_mut_ptr(), a);
    if *a.offset(0 as libc::c_int as isize) as libc::c_int
        > '0' as i32 + 1 as libc::c_int
        || *a.offset(0 as libc::c_int as isize) as libc::c_int
            == '0' as i32 + 1 as libc::c_int
            && *a.offset(1 as libc::c_int as isize) as libc::c_int != '.' as i32
    {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut ch: libc::c_int = 0;
        loop {
            let fresh39 = i;
            i = i + 1;
            ch = *a.offset(fresh39 as isize) as libc::c_int;
            if !(ch != '\0' as i32 as libc::c_char as libc::c_int) {
                break;
            }
            if ch == '.' as i32 {
                break;
            }
        }
        i = (i + 1 as libc::c_int) / 2 as libc::c_int;
        if i != 0 {
            *a.offset(i as isize) = '\0' as i32 as libc::c_char;
        }
    } else if *a.offset(0 as libc::c_int as isize) as libc::c_int
        != '0' as i32 + 1 as libc::c_int
    {
        *a
            .offset(
                0 as libc::c_int as isize,
            ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
        *a.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    (*partab.jobtab).precision += 1;
    (*partab.jobtab).precision;
    loop {
        let mut s: libc::c_short = 0;
        strcpy(XXX.as_mut_ptr(), a);
        strcpy(tmp1.as_mut_ptr(), XX.as_mut_ptr());
        strcpy(tmp2.as_mut_ptr(), a);
        s = runtime_div(
            tmp1.as_mut_ptr(),
            tmp2.as_mut_ptr(),
            13 as libc::c_int as libc::c_short,
        );
        if (s as libc::c_int) < 0 as libc::c_int {
            return s as libc::c_int;
        }
        s = runtime_add(a, tmp1.as_mut_ptr());
        if (s as libc::c_int) < 0 as libc::c_int {
            return s as libc::c_int;
        }
        tmp2[0 as libc::c_int
            as usize] = ('0' as i32 + 2 as libc::c_int) as libc::c_char;
        tmp2[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        s = runtime_div(a, tmp2.as_mut_ptr(), 13 as libc::c_int as libc::c_short);
        if (s as libc::c_int) < 0 as libc::c_int {
            return s as libc::c_int;
        }
        if !(runtime_comp(a, XXX.as_mut_ptr()) != 0) {
            break;
        }
    }
    (*partab.jobtab).precision -= 1;
    (*partab.jobtab).precision;
    return strlen(a) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn root(
    mut a: *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut s: libc::c_short = 0;
    let mut tmp1: [libc::c_char; 258] = [0; 258];
    let mut tmp2: [libc::c_char; 258] = [0; 258];
    let mut XX: [libc::c_char; 258] = [0; 258];
    let mut XXX: [libc::c_char; 258] = [0; 258];
    let mut again: libc::c_short = 0;
    if *a.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        return 1 as libc::c_int;
    }
    if *a.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        || n == 0 as libc::c_int
    {
        return -(9 as libc::c_int);
    }
    strcpy(XX.as_mut_ptr(), a);
    if *a.offset(0 as libc::c_int as isize) as libc::c_int
        > '0' as i32 + 1 as libc::c_int
        || *a.offset(0 as libc::c_int as isize) as libc::c_int
            == '0' as i32 + 1 as libc::c_int
            && *a.offset(1 as libc::c_int as isize) as libc::c_int != '.' as i32
    {
        i = 0 as libc::c_int;
        loop {
            let fresh40 = i;
            i = i + 1;
            ch = *a.offset(fresh40 as isize) as libc::c_int;
            if !(ch != '\0' as i32 as libc::c_char as libc::c_int && ch != '.' as i32) {
                break;
            }
        }
        i = (i + n - 2 as libc::c_int) / n;
        if i > 0 as libc::c_int {
            *a
                .offset(
                    0 as libc::c_int as isize,
                ) = ('0' as i32 + 3 as libc::c_int) as libc::c_char;
            *a.offset(i as isize) = '\0' as i32 as libc::c_char;
        }
    } else if *a.offset(0 as libc::c_int as isize) as libc::c_int
        != '0' as i32 + 1 as libc::c_int
    {
        *a
            .offset(
                0 as libc::c_int as isize,
            ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
        *a.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    if (*partab.jobtab).precision as libc::c_int <= 3 as libc::c_int {
        again = 0 as libc::c_int as libc::c_short;
    } else {
        again = (*partab.jobtab).precision;
        (*partab.jobtab).precision = 2 as libc::c_int as libc::c_short;
    }
    loop {
        (*partab.jobtab).precision += 1;
        (*partab.jobtab).precision;
        loop {
            strcpy(XXX.as_mut_ptr(), a);
            s = itocstring(tmp1.as_mut_ptr() as *mut u_char, n - 1 as libc::c_int)
                as libc::c_short;
            if (s as libc::c_int) < 0 as libc::c_int {
                return s as libc::c_int;
            }
            strcpy(tmp2.as_mut_ptr(), a);
            s = runtime_power(tmp2.as_mut_ptr(), tmp1.as_mut_ptr());
            if (s as libc::c_int) < 0 as libc::c_int {
                return s as libc::c_int;
            }
            strcpy(tmp1.as_mut_ptr(), XX.as_mut_ptr());
            s = runtime_div(
                tmp1.as_mut_ptr(),
                tmp2.as_mut_ptr(),
                13 as libc::c_int as libc::c_short,
            );
            if (s as libc::c_int) < 0 as libc::c_int {
                break;
            }
            s = itocstring(tmp2.as_mut_ptr() as *mut u_char, n - 1 as libc::c_int)
                as libc::c_short;
            if (s as libc::c_int) < 0 as libc::c_int {
                return s as libc::c_int;
            }
            s = runtime_mul(a, tmp2.as_mut_ptr());
            if (s as libc::c_int) < 0 as libc::c_int {
                return s as libc::c_int;
            }
            s = runtime_add(a, tmp1.as_mut_ptr());
            if (s as libc::c_int) < 0 as libc::c_int {
                return s as libc::c_int;
            }
            s = itocstring(tmp2.as_mut_ptr() as *mut u_char, n) as libc::c_short;
            if (s as libc::c_int) < 0 as libc::c_int {
                return s as libc::c_int;
            }
            s = runtime_div(a, tmp2.as_mut_ptr(), 13 as libc::c_int as libc::c_short);
            if (s as libc::c_int) < 0 as libc::c_int {
                return s as libc::c_int;
            }
            strcpy(tmp2.as_mut_ptr(), a);
            s = runtime_div(
                XXX.as_mut_ptr(),
                tmp2.as_mut_ptr(),
                13 as libc::c_int as libc::c_short,
            );
            if (s as libc::c_int) < 0 as libc::c_int {
                return s as libc::c_int;
            }
            tmp2[0 as libc::c_int
                as usize] = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
            if (*partab.jobtab).precision as libc::c_int <= 0 as libc::c_int {
                tmp2[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            } else {
                tmp2[1 as libc::c_int as usize] = '.' as i32 as libc::c_char;
                i = 2 as libc::c_int;
                while i < (*partab.jobtab).precision as libc::c_int {
                    tmp2[i as usize] = '0' as i32 as libc::c_char;
                    i += 1;
                    i;
                }
                let fresh41 = i;
                i = i + 1;
                tmp2[fresh41
                    as usize] = ('0' as i32 + 10 as libc::c_int / 2 as libc::c_int)
                    as libc::c_char;
                tmp2[i as usize] = '\0' as i32 as libc::c_char;
            }
            if runtime_comp(XXX.as_mut_ptr(), tmp2.as_mut_ptr()) == 0 {
                continue;
            }
            if (*partab.jobtab).precision as libc::c_int <= 0 as libc::c_int {
                break;
            }
            tmp2[0 as libc::c_int as usize] = '.' as i32 as libc::c_char;
            i = 1 as libc::c_int;
            while i < (*partab.jobtab).precision as libc::c_int {
                tmp2[i
                    as usize] = ('0' as i32 + 10 as libc::c_int - 1 as libc::c_int)
                    as libc::c_char;
                i += 1;
                i;
            }
            tmp2[(i - 1 as libc::c_int)
                as usize] = ('0' as i32 + 10 as libc::c_int / 2 as libc::c_int)
                as libc::c_char;
            tmp2[i as usize] = '\0' as i32 as libc::c_char;
            if runtime_comp(tmp2.as_mut_ptr(), XXX.as_mut_ptr()) != 0 {
                break;
            }
        }
        (*partab.jobtab).precision -= 1;
        (*partab.jobtab).precision;
        if !(again != 0) {
            break;
        }
        (*partab.jobtab).precision = again;
        again = 0 as libc::c_int as libc::c_short;
    }
    return strlen(a) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn roundit(mut a: *mut libc::c_char, mut digits: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut pointpos: libc::c_int = 0;
    let mut lena: libc::c_int = 0;
    pointpos = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *a.offset(i as isize) as libc::c_int
        != '\0' as i32 as libc::c_char as libc::c_int
    {
        if *a.offset(i as isize) as libc::c_int == '.' as i32 {
            pointpos = i;
        }
        i += 1;
        i;
    }
    lena = i;
    if pointpos < 0 as libc::c_int {
        pointpos = i;
    }
    if pointpos + digits + 1 as libc::c_int >= i {
        return;
    }
    i = pointpos + digits + 1 as libc::c_int;
    if (*a.offset(i as isize) as libc::c_int)
        < '0' as i32 + 10 as libc::c_int / 2 as libc::c_int
    {
        *a.offset(i as isize) = '\0' as i32 as libc::c_char;
        loop {
            i -= 1;
            if !(*a.offset(i as isize) as libc::c_int == '0' as i32) {
                break;
            }
            *a.offset(i as isize) = '\0' as i32 as libc::c_char;
        }
        if *a.offset(i as isize) as libc::c_int == '.' as i32 {
            *a.offset(i as isize) = '\0' as i32 as libc::c_char;
            if i == 0 as libc::c_int
                || i == 1 as libc::c_int
                    && *a.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            {
                *a.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
            }
        }
        return;
    }
    loop {
        let mut ch: libc::c_int = 0;
        if i >= pointpos {
            *a.offset(i as isize) = '\0' as i32 as libc::c_char;
        } else {
            *a.offset(i as isize) = '0' as i32 as libc::c_char;
        }
        i -= 1;
        if i
            < (*a.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
                as libc::c_int
        {
            i = lena;
            while i >= 0 as libc::c_int {
                *a.offset((i + 1 as libc::c_int) as isize) = *a.offset(i as isize);
                i -= 1;
                i;
            }
            *a
                .offset(
                    (*a.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
                        as libc::c_int as isize,
                ) = ('0' as i32 + 1 as libc::c_int) as libc::c_char;
            break;
        } else {
            ch = *a.offset(i as isize) as libc::c_int;
            if ch == '.' as i32 {
                continue;
            }
            if !((*a.offset(i as isize) as libc::c_int)
                < '0' as i32 + 10 as libc::c_int - 1 as libc::c_int && ch >= '0' as i32)
            {
                continue;
            }
            ch += 1;
            *a.offset(i as isize) = ch as libc::c_char;
            break;
        }
    };
}
