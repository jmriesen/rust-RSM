#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type GBD;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut partab: partab_struct;
    static mut st_hash: [libc::c_short; 0];
    static mut symtab: [symtab_struct; 0];
    fn ST_LocateIdx(idx: libc::c_int) -> libc::c_short;
    fn ST_Create(var: var_u) -> libc::c_short;
    fn ST_SymAtt(var: var_u) -> libc::c_short;
    fn ST_SymKill(syment: libc::c_short) -> libc::c_short;
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
pub type ST_data = ST_DATA;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SYMTAB {
    pub fwd_link: libc::c_short,
    pub usage: libc::c_short,
    pub data: *mut ST_DATA,
    pub varnam: var_u,
}
pub type symtab_struct = SYMTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_LOCDATA {
    pub stindex: libc::c_short,
    pub data: *mut ST_data,
}
pub type ST_locdata = ST_LOCDATA;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_NEWTAB {
    pub fwd_link: *mut ST_NEWTAB,
    pub count_enn: libc::c_short,
    pub stindex: *mut libc::c_short,
    pub count_new: libc::c_short,
    pub locdata: *mut ST_locdata,
}
pub type ST_newtab = ST_NEWTAB;
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
pub unsafe extern "C" fn ST_New(
    mut count: libc::c_int,
    mut list: *mut var_u,
) -> libc::c_short {
    let mut newtab: *mut ST_newtab = 0 as *mut ST_newtab;
    let mut i: libc::c_int = 0;
    newtab = malloc(
        (::core::mem::size_of::<ST_newtab>() as libc::c_ulong)
            .wrapping_add(
                (count as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<ST_locdata>() as libc::c_ulong),
            ),
    ) as *mut ST_newtab;
    if newtab.is_null() {
        return -(56 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    (*newtab)
        .fwd_link = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].newtab
        as *mut ST_newtab;
    (*newtab).count_enn = 0 as libc::c_int as libc::c_short;
    (*newtab).stindex = 0 as *mut libc::c_short;
    (*newtab).count_new = count as libc::c_short;
    (*newtab)
        .locdata = (&mut (*newtab).locdata as *mut *mut ST_locdata as *mut u_char)
        .offset(::core::mem::size_of::<*mut ST_locdata>() as libc::c_ulong as isize)
        as *mut ST_locdata;
    i = count - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut s: libc::c_short = ST_SymAtt(*list.offset(i as isize));
        if (s as libc::c_int) < 0 as libc::c_int {
            free(newtab as *mut libc::c_void);
            return s;
        }
        (*((*newtab).locdata).offset(i as isize)).stindex = s;
        let ref mut fresh0 = (*((*newtab).locdata).offset(i as isize)).data;
        *fresh0 = (*symtab.as_mut_ptr().offset(s as isize)).data;
        let ref mut fresh1 = (*symtab.as_mut_ptr().offset(s as isize)).data;
        *fresh1 = 0 as *mut libc::c_void as *mut ST_data;
        i -= 1;
        i;
    }
    (*partab.jobtab)
        .dostk[(*partab.jobtab).cur_do as usize]
        .newtab = newtab as *mut u_char;
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_NewAll(
    mut count: libc::c_int,
    mut list: *mut var_u,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut new: libc::c_int = 0 as libc::c_int;
    let mut cntnew: libc::c_int = 0 as libc::c_int;
    let mut cntnon: libc::c_int = 0 as libc::c_int;
    let mut newtab: *mut ST_newtab = 0 as *mut ST_newtab;
    k = 0 as libc::c_int;
    while k < count {
        ST_Create(*list.offset(k as isize));
        k += 1;
        k;
    }
    i = 0 as libc::c_int;
    while i < (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
        if !((*symtab.as_mut_ptr().offset(i as isize))
            .varnam
            .var_cu[0 as libc::c_int as usize] as libc::c_int == '$' as i32)
        {
            if !((*symtab.as_mut_ptr().offset(i as isize))
                .varnam
                .var_cu[0 as libc::c_int as usize] as libc::c_int == '\0' as i32)
            {
                if count > 0 as libc::c_int {
                    j = 0 as libc::c_int;
                    while j < count {
                        new = 1 as libc::c_int;
                        if var_equal(
                            (*symtab.as_mut_ptr().offset(i as isize)).varnam,
                            *list.offset(j as isize),
                        ) != 0
                        {
                            new = 0 as libc::c_int;
                            break;
                        } else {
                            j += 1;
                            j;
                        }
                    }
                    if new == 1 as libc::c_int {
                        cntnew += 1 as libc::c_int;
                    } else {
                        cntnon += 1 as libc::c_int;
                    }
                } else {
                    cntnew += 1 as libc::c_int;
                }
            }
        }
        i += 1;
        i;
    }
    newtab = malloc(
        (::core::mem::size_of::<ST_newtab>() as libc::c_ulong)
            .wrapping_add(
                (cntnew as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<ST_locdata>() as libc::c_ulong),
            )
            .wrapping_add(
                (cntnon as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            ),
    ) as *mut ST_newtab;
    if newtab.is_null() {
        return -(56 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    (*newtab)
        .fwd_link = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].newtab
        as *mut ST_newtab;
    (*newtab).count_enn = count as libc::c_short;
    (*newtab).count_new = 0 as libc::c_int as libc::c_short;
    (*newtab)
        .stindex = (&mut (*newtab).locdata as *mut *mut ST_locdata as *mut u_char)
        .offset(::core::mem::size_of::<*mut ST_locdata>() as libc::c_ulong as isize)
        as *mut libc::c_short;
    (*newtab)
        .locdata = (&mut (*newtab).locdata as *mut *mut ST_locdata as *mut u_char)
        .offset(::core::mem::size_of::<*mut ST_locdata>() as libc::c_ulong as isize)
        .offset(
            (cntnon as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                as isize,
        ) as *mut ST_locdata;
    i = 0 as libc::c_int;
    while i < (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
        if !((*symtab.as_mut_ptr().offset(i as isize))
            .varnam
            .var_cu[0 as libc::c_int as usize] as libc::c_int == '$' as i32)
        {
            if !((*symtab.as_mut_ptr().offset(i as isize))
                .varnam
                .var_cu[0 as libc::c_int as usize] as libc::c_int == '\0' as i32)
            {
                if count > 0 as libc::c_int {
                    j = 0 as libc::c_int;
                    while j < count {
                        new = 1 as libc::c_int;
                        if var_equal(
                            (*symtab.as_mut_ptr().offset(i as isize)).varnam,
                            *list.offset(j as isize),
                        ) != 0
                        {
                            new = 0 as libc::c_int;
                            break;
                        } else {
                            j += 1;
                            j;
                        }
                    }
                    if new == 1 as libc::c_int {
                        (*((*newtab).locdata).offset((*newtab).count_new as isize))
                            .stindex = i as libc::c_short;
                        let ref mut fresh2 = (*((*newtab).locdata)
                            .offset((*newtab).count_new as isize))
                            .data;
                        *fresh2 = (*symtab
                            .as_mut_ptr()
                            .offset(
                                (*((*newtab).locdata).offset((*newtab).count_new as isize))
                                    .stindex as isize,
                            ))
                            .data;
                        let ref mut fresh3 = (*symtab
                            .as_mut_ptr()
                            .offset(
                                (*((*newtab).locdata).offset((*newtab).count_new as isize))
                                    .stindex as isize,
                            ))
                            .data;
                        *fresh3 = 0 as *mut libc::c_void as *mut ST_data;
                        let ref mut fresh4 = (*symtab
                            .as_mut_ptr()
                            .offset(
                                (*((*newtab).locdata).offset((*newtab).count_new as isize))
                                    .stindex as isize,
                            ))
                            .usage;
                        *fresh4 += 1;
                        *fresh4;
                        (*newtab).count_new += 1;
                        (*newtab).count_new;
                    } else {
                        *((*newtab).stindex).offset(j as isize) = i as libc::c_short;
                    }
                } else {
                    (*((*newtab).locdata).offset((*newtab).count_new as isize))
                        .stindex = i as libc::c_short;
                    let ref mut fresh5 = (*((*newtab).locdata)
                        .offset((*newtab).count_new as isize))
                        .data;
                    *fresh5 = (*symtab
                        .as_mut_ptr()
                        .offset(
                            (*((*newtab).locdata).offset((*newtab).count_new as isize))
                                .stindex as isize,
                        ))
                        .data;
                    let ref mut fresh6 = (*symtab
                        .as_mut_ptr()
                        .offset(
                            (*((*newtab).locdata).offset((*newtab).count_new as isize))
                                .stindex as isize,
                        ))
                        .data;
                    *fresh6 = 0 as *mut libc::c_void as *mut ST_data;
                    let ref mut fresh7 = (*symtab
                        .as_mut_ptr()
                        .offset(
                            (*((*newtab).locdata).offset((*newtab).count_new as isize))
                                .stindex as isize,
                        ))
                        .usage;
                    *fresh7 += 1;
                    *fresh7;
                    (*newtab).count_new += 1;
                    (*newtab).count_new;
                }
            }
        }
        i += 1;
        i;
    }
    (*partab.jobtab)
        .dostk[(*partab.jobtab).cur_do as usize]
        .newtab = newtab as *mut u_char;
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Restore(mut newtab: *mut ST_newtab) {
    let mut ptr: *mut ST_newtab = 0 as *mut ST_newtab;
    let mut dd: *mut ST_depend = 0 as *mut ST_depend;
    let mut ddf: *mut ST_depend = 0 as *mut ST_depend;
    let mut i: libc::c_int = 0;
    ptr = newtab;
    if ptr.is_null() {
        return;
    }
    if !((*ptr).stindex).is_null() {
        let mut t: libc::c_int = 0 as libc::c_int;
        while t < 1023 as libc::c_int {
            if *st_hash.as_mut_ptr().offset(t as isize) as libc::c_int
                != -(1 as libc::c_int)
            {
                let mut chk: libc::c_int = *st_hash.as_mut_ptr().offset(t as isize)
                    as libc::c_int;
                while chk != -(1 as libc::c_int) {
                    let mut kill: libc::c_int = chk;
                    if (*symtab.as_mut_ptr().offset(chk as isize))
                        .varnam
                        .var_cu[0 as libc::c_int as usize] as libc::c_int == '$' as i32
                    {
                        kill = -(1 as libc::c_int);
                    } else {
                        i = 0 as libc::c_int;
                        while i < (*ptr).count_enn as libc::c_int {
                            if var_equal(
                                (*symtab.as_mut_ptr().offset(chk as isize)).varnam,
                                (*symtab
                                    .as_mut_ptr()
                                    .offset(*((*ptr).stindex).offset(i as isize) as isize))
                                    .varnam,
                            ) != 0
                            {
                                kill = -(1 as libc::c_int);
                                break;
                            } else {
                                i += 1;
                                i;
                            }
                        }
                    }
                    chk = (*symtab.as_mut_ptr().offset(chk as isize)).fwd_link
                        as libc::c_int;
                    if kill > -(1 as libc::c_int) {
                        ST_SymKill(kill as libc::c_short);
                    }
                }
            }
            t += 1;
            t;
        }
    }
    i = 0 as libc::c_int;
    while i < (*ptr).count_new as libc::c_int {
        if !((*symtab
            .as_mut_ptr()
            .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
            .data)
            .is_null()
        {
            let ref mut fresh8 = (*(*symtab
                .as_mut_ptr()
                .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                .data)
                .attach;
            *fresh8 -= 1;
            *fresh8;
            if ((*(*symtab
                .as_mut_ptr()
                .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                .data)
                .attach as libc::c_int) < 1 as libc::c_int
            {
                dd = (*(*symtab
                    .as_mut_ptr()
                    .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                    .data)
                    .deplnk;
                while !dd.is_null() {
                    ddf = dd;
                    dd = (*dd).deplnk;
                    free(ddf as *mut libc::c_void);
                }
                free(
                    (*symtab
                        .as_mut_ptr()
                        .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                        .data as *mut libc::c_void,
                );
                let ref mut fresh9 = (*symtab
                    .as_mut_ptr()
                    .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                    .data;
                *fresh9 = 0 as *mut libc::c_void as *mut ST_data;
            }
        }
        let ref mut fresh10 = (*symtab
            .as_mut_ptr()
            .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
            .data;
        *fresh10 = (*((*ptr).locdata).offset(i as isize)).data;
        let ref mut fresh11 = (*symtab
            .as_mut_ptr()
            .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
            .usage;
        *fresh11 -= 1;
        *fresh11;
        if !((*symtab
            .as_mut_ptr()
            .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
            .data)
            .is_null()
        {
            if ((*(*symtab
                .as_mut_ptr()
                .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                .data)
                .deplnk)
                .is_null()
                && ((*(*symtab
                    .as_mut_ptr()
                    .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                    .data)
                    .attach as libc::c_int) < 2 as libc::c_int
                && (*(*symtab
                    .as_mut_ptr()
                    .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                    .data)
                    .dbc as libc::c_int == 65534 as libc::c_int + 1 as libc::c_int
            {
                free(
                    (*symtab
                        .as_mut_ptr()
                        .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                        .data as *mut libc::c_void,
                );
                let ref mut fresh12 = (*symtab
                    .as_mut_ptr()
                    .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                    .data;
                *fresh12 = 0 as *mut libc::c_void as *mut ST_data;
            }
        }
        if ((*symtab
            .as_mut_ptr()
            .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
            .usage as libc::c_int) < 1 as libc::c_int
            && ((*symtab
                .as_mut_ptr()
                .offset((*((*ptr).locdata).offset(i as isize)).stindex as isize))
                .data)
                .is_null()
        {
            ST_SymKill((*((*ptr).locdata).offset(i as isize)).stindex);
        }
        i += 1;
        i;
    }
    if !((*ptr).fwd_link).is_null() {
        ST_Restore((*ptr).fwd_link);
    }
    free(ptr as *mut libc::c_void);
    if ptr
        == (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].newtab
            as *mut ST_newtab
    {
        (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .newtab = 0 as *mut u_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ST_ConData(
    mut var: *mut mvar,
    mut data: *mut u_char,
) -> libc::c_short {
    let mut cnct: libc::c_short = 0;
    cnct = ST_LocateIdx((*var).volset as libc::c_int - 1 as libc::c_int);
    if (cnct as libc::c_int) < 0 as libc::c_int {
        return -(6 as libc::c_int) as libc::c_short;
    }
    let ref mut fresh13 = (*symtab.as_mut_ptr().offset(cnct as isize)).data;
    *fresh13 = data as *mut ST_data;
    let ref mut fresh14 = (*(*symtab.as_mut_ptr().offset(cnct as isize)).data).attach;
    *fresh14 += 1;
    *fresh14;
    return 0 as libc::c_int as libc::c_short;
}
