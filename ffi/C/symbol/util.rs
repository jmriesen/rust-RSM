#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type GBD;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut partab: partab_struct;
    fn DB_Set(var: *mut mvar, data: *mut cstring) -> libc::c_int;
    fn SQ_Write(buf: *mut cstring) -> libc::c_int;
    fn SQ_WriteFormat(count: libc::c_int) -> libc::c_short;
    fn UTIL_Key_Build(src: *mut cstring, dest: *mut u_char) -> libc::c_short;
    fn UTIL_Key_Extract(
        key: *mut u_char,
        str: *mut u_char,
        cnt: *mut libc::c_int,
    ) -> libc::c_short;
    fn UTIL_String_Mvar(
        var: *mut mvar,
        str: *mut u_char,
        max_subs: libc::c_int,
    ) -> libc::c_short;
    fn UTIL_Key_KeyCmp(
        key1: *mut u_char,
        key2: *mut u_char,
        kleng1: libc::c_int,
        kleng2: libc::c_int,
    ) -> libc::c_int;
    fn UTIL_Key_Chars_In_Subs(
        Key: *mut libc::c_char,
        keylen: libc::c_int,
        maxsubs: libc::c_int,
        subs: *mut libc::c_int,
        KeyBuffer: *mut libc::c_char,
    ) -> libc::c_int;
    fn mcopy(src: *mut u_char, dst: *mut u_char, bytes: libc::c_int) -> libc::c_int;
    fn panic(msg: *mut libc::c_char);
}
pub type __darwin_time_t = libc::c_long;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
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
#[inline]
unsafe extern "C" fn var_empty(mut var: var_u) -> u_int {
    if var.var_q == 0 as libc::c_int as u_int64 {
        return 1 as libc::c_int as u_int
    } else {
        return 0 as libc::c_int as u_int
    };
}
#[no_mangle]
pub static mut st_hash: [libc::c_short; 1024] = [0; 1024];
#[no_mangle]
pub static mut symtab: [symtab_struct; 3073] = [SYMTAB {
    fwd_link: 0,
    usage: 0,
    data: 0 as *const ST_DATA as *mut ST_DATA,
    varnam: VAR_U { var_q: 0 },
}; 3073];
#[no_mangle]
pub unsafe extern "C" fn ST_Hash(mut var: var_u) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let p: [libc::c_int; 32] = [
        3 as libc::c_int,
        5 as libc::c_int,
        7 as libc::c_int,
        11 as libc::c_int,
        13 as libc::c_int,
        17 as libc::c_int,
        19 as libc::c_int,
        23 as libc::c_int,
        29 as libc::c_int,
        31 as libc::c_int,
        37 as libc::c_int,
        41 as libc::c_int,
        43 as libc::c_int,
        47 as libc::c_int,
        53 as libc::c_int,
        59 as libc::c_int,
        61 as libc::c_int,
        67 as libc::c_int,
        71 as libc::c_int,
        73 as libc::c_int,
        79 as libc::c_int,
        83 as libc::c_int,
        89 as libc::c_int,
        97 as libc::c_int,
        101 as libc::c_int,
        103 as libc::c_int,
        107 as libc::c_int,
        109 as libc::c_int,
        113 as libc::c_int,
        127 as libc::c_int,
        131 as libc::c_int,
        137 as libc::c_int,
    ];
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if var.var_cu[i as usize] as libc::c_int == 0 as libc::c_int {
            break;
        }
        ret = var.var_cu[i as usize] as libc::c_int * p[i as usize] + ret;
        i += 1;
        i;
    }
    return (ret % 1023 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Init() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 1023 as libc::c_int {
        st_hash[i as usize] = -(1 as libc::c_int) as libc::c_short;
        i += 1;
        i;
    }
    st_hash[1023 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    i = 0 as libc::c_int;
    while i < (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
        symtab[i as usize].fwd_link = (i + 1 as libc::c_int) as libc::c_short;
        symtab[i as usize].usage = 0 as libc::c_int as libc::c_short;
        symtab[i as usize].data = 0 as *mut libc::c_void as *mut ST_data;
        let mut var_i: u_int = 0 as libc::c_int as u_int;
        while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            symtab[i as usize]
                .varnam
                .var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
            var_i = var_i.wrapping_add(1);
            var_i;
        }
        i += 1;
        i;
    }
    symtab[((1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int) as usize]
        .fwd_link = -(1 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Locate(mut var: var_u) -> libc::c_short {
    let mut hash: libc::c_int = 0;
    let mut fwd: libc::c_short = 0;
    hash = ST_Hash(var) as libc::c_int;
    fwd = st_hash[hash as usize];
    while fwd as libc::c_int != -(1 as libc::c_int) {
        if var_equal(symtab[fwd as usize].varnam, var) != 0 {
            return fwd;
        }
        fwd = symtab[fwd as usize].fwd_link;
    }
    return -(1 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_LocateIdx(mut idx: libc::c_int) -> libc::c_short {
    let mut fwd: libc::c_short = 0;
    let mut var: var_u = VAR_U { var_q: 0 };
    let mut p: *mut rbd = 0 as *mut rbd;
    let mut vt: *mut var_u = 0 as *mut var_u;
    fwd = *((*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].symbol)
        .offset(idx as isize);
    if fwd as libc::c_int > -(1 as libc::c_int) {
        return fwd;
    }
    p = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].routine as *mut rbd;
    vt = (p as *mut u_char).offset((*p).var_tbl as libc::c_int as isize) as *mut var_u;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        var.var_qu[var_i as usize] = (*vt.offset(idx as isize)).var_qu[var_i as usize];
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    fwd = ST_SymAtt(var);
    if (fwd as libc::c_int) < 0 as libc::c_int {
        return fwd;
    }
    *((*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].symbol)
        .offset(idx as isize) = fwd;
    return fwd;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Free(mut var: var_u) {
    let mut hash: libc::c_short = 0;
    let mut fwd: libc::c_short = 0;
    let mut last: libc::c_short = 0;
    hash = ST_Hash(var);
    last = -(hash as libc::c_int) as libc::c_short;
    fwd = st_hash[hash as usize];
    while fwd as libc::c_int != -(1 as libc::c_int) {
        if var_equal(symtab[fwd as usize].varnam, var) != 0 {
            break;
        }
        last = fwd;
        fwd = symtab[fwd as usize].fwd_link;
    }
    if fwd as libc::c_int == -(1 as libc::c_int) {
        return;
    }
    if last as libc::c_int == -(hash as libc::c_int) {
        st_hash[hash as usize] = symtab[fwd as usize].fwd_link;
    } else {
        symtab[last as usize].fwd_link = symtab[fwd as usize].fwd_link;
    }
    symtab[fwd as usize].data = 0 as *mut libc::c_void as *mut ST_data;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        symtab[fwd as usize].varnam.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    symtab[fwd as usize].fwd_link = st_hash[1023 as libc::c_int as usize];
    st_hash[1023 as libc::c_int as usize] = fwd;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Create(mut var: var_u) -> libc::c_short {
    let mut hash: libc::c_int = 0;
    let mut fwd: libc::c_short = 0;
    hash = ST_Hash(var) as libc::c_int;
    fwd = st_hash[hash as usize];
    while fwd as libc::c_int != -(1 as libc::c_int) {
        if var_equal(symtab[fwd as usize].varnam, var) != 0 {
            return fwd;
        }
        fwd = symtab[fwd as usize].fwd_link;
    }
    fwd = st_hash[1023 as libc::c_int as usize];
    if fwd as libc::c_int == -(1 as libc::c_int)
        || fwd as libc::c_int
            == (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int
            && var.var_q != 76159689901348 as libc::c_long as u_int64
    {
        return -(56 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    st_hash[1023 as libc::c_int as usize] = symtab[fwd as usize].fwd_link;
    symtab[fwd as usize].fwd_link = st_hash[hash as usize];
    st_hash[hash as usize] = fwd;
    symtab[fwd as usize].usage = 0 as libc::c_int as libc::c_short;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        symtab[fwd as usize].varnam.var_qu[var_i as usize] = var.var_qu[var_i as usize];
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    symtab[fwd as usize].data = 0 as *mut libc::c_void as *mut ST_data;
    return fwd;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Kill(mut var: *mut mvar) -> libc::c_short {
    let mut ptr: libc::c_short = 0;
    let mut data: *mut ST_data = 0 as *mut ST_data;
    let mut check: *mut ST_depend = 0 as *mut ST_depend;
    let mut checkprev: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    if (*var).volset != 0 {
        ptr = ST_LocateIdx((*var).volset as libc::c_int - 1 as libc::c_int);
    } else {
        ptr = ST_Locate((*var).name);
    }
    if (ptr as libc::c_int) < 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    data = symtab[ptr as usize].data;
    if !data.is_null() {
        if (*var).slen as libc::c_int == 0 as libc::c_int {
            check = (*data).deplnk;
            while !check.is_null() {
                checkprev = check;
                check = (*check).deplnk;
                free(checkprev as *mut libc::c_void);
            }
            (*data).deplnk = 0 as *mut libc::c_void as *mut ST_depend;
            (*data).dbc = (65534 as libc::c_int + 1 as libc::c_int) as u_short;
        } else {
            check = (*data).deplnk;
            if !check.is_null() {
                while !check.is_null()
                    && UTIL_Key_KeyCmp(
                        ((*check).bytes).as_mut_ptr(),
                        ((*var).key).as_mut_ptr(),
                        (*check).keylen as libc::c_int,
                        (*var).slen as libc::c_int,
                    ) < 0 as libc::c_int
                {
                    checkprev = check;
                    check = (*check).deplnk;
                }
                if !check.is_null()
                    && memcmp(
                        ((*check).bytes).as_mut_ptr() as *const libc::c_void,
                        ((*var).key).as_mut_ptr() as *const libc::c_void,
                        (*var).slen as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    ST_RemDp(data, checkprev, check, var);
                }
            }
        }
        if ((*data).deplnk).is_null()
            && ((*data).attach as libc::c_int) < 2 as libc::c_int
            && (*data).dbc as libc::c_int == 65534 as libc::c_int + 1 as libc::c_int
        {
            free(data as *mut libc::c_void);
            symtab[ptr as usize].data = 0 as *mut libc::c_void as *mut ST_data;
        }
    }
    if (symtab[ptr as usize].data).is_null()
        && symtab[ptr as usize].usage as libc::c_int == 0 as libc::c_int
    {
        ST_Free(symtab[ptr as usize].varnam);
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_RemDp(
    mut dblk: *mut ST_data,
    mut prev: *mut ST_depend,
    mut dp: *mut ST_depend,
    mut mvardr: *mut mvar,
) {
    if dp.is_null() {
        return;
    }
    if !((*dp).deplnk).is_null() && (*mvardr).slen as libc::c_int == 0 as libc::c_int {
        ST_RemDp(dblk, dp, (*dp).deplnk, mvardr);
    } else if !((*dp).deplnk).is_null()
        && memcmp(
            ((*dp).bytes).as_mut_ptr() as *const libc::c_void,
            ((*mvardr).key).as_mut_ptr() as *const libc::c_void,
            (*mvardr).slen as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        ST_RemDp(dblk, dp, (*dp).deplnk, mvardr);
    }
    if (*mvardr).slen as libc::c_int == 0 as libc::c_int {
        if !prev.is_null() {
            (*prev).deplnk = 0 as *mut libc::c_void as *mut ST_depend;
            free(dp as *mut libc::c_void);
        } else {
            (*dblk).deplnk = 0 as *mut libc::c_void as *mut ST_depend;
            free(dp as *mut libc::c_void);
        }
    } else if memcmp(
        ((*dp).bytes).as_mut_ptr() as *const libc::c_void,
        ((*mvardr).key).as_mut_ptr() as *const libc::c_void,
        (*mvardr).slen as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        if !prev.is_null() {
            (*prev).deplnk = (*dp).deplnk;
        } else {
            (*dblk).deplnk = (*dp).deplnk;
        }
        free(dp as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ST_Get(
    mut var: *mut mvar,
    mut buf: *mut u_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut data: *mut cstring = 0 as *mut cstring;
    s = ST_GetAdd(var, &mut data);
    if s < 0 as libc::c_int {
        return s;
    }
    s = mcopy(
        &mut *((*data).buf).as_mut_ptr().offset(0 as libc::c_int as isize),
        buf,
        s,
    );
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn FixData(
    mut old: *const ST_data,
    mut new: *mut ST_data,
    mut count: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut newtab: *mut ST_newtab = 0 as *mut ST_newtab;
    i = 0 as libc::c_int;
    while i < (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
        if symtab[i as usize].data == old as *mut ST_DATA {
            symtab[i as usize].data = new;
            count -= 1;
            count;
            if count == 0 as libc::c_int {
                return;
            }
        }
        i += 1;
        i;
    }
    i = (*partab.jobtab).cur_do;
    while i != 0 {
        let fresh0 = i;
        i = i - 1;
        newtab = (*partab.jobtab).dostk[fresh0 as usize].newtab as *mut ST_newtab;
        while !newtab.is_null() {
            c = 0 as libc::c_int;
            while c < (*newtab).count_new as libc::c_int {
                if (*((*newtab).locdata).offset(c as isize)).data == old as *mut ST_data
                {
                    let ref mut fresh1 = (*((*newtab).locdata).offset(c as isize)).data;
                    *fresh1 = new;
                    count -= 1;
                    count;
                    if count == 0 as libc::c_int {
                        return;
                    }
                }
                c += 1;
                c;
            }
            newtab = (*newtab).fwd_link;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ST_Set(
    mut var: *mut mvar,
    mut data: *mut cstring,
) -> libc::c_int {
    let mut ptr1: *mut ST_depend = 0 as *mut ST_depend;
    let mut newPtrDp: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    let mut newPtrDt: *mut ST_data = 0 as *mut libc::c_void as *mut ST_data;
    let mut prevPtr: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    let mut n: libc::c_int = 0;
    let mut pad: libc::c_int = 0 as libc::c_int;
    let mut ptr2ushort: *mut u_short = 0 as *mut u_short;
    let mut fwd: libc::c_short = 0;
    if (*var).slen as libc::c_int & 1 as libc::c_int != 0 as libc::c_int {
        pad = 1 as libc::c_int;
    }
    if (*var).volset != 0 {
        fwd = ST_LocateIdx((*var).volset as libc::c_int - 1 as libc::c_int);
    } else {
        fwd = ST_Create((*var).name);
    }
    if (fwd as libc::c_int) < 0 as libc::c_int {
        return fwd as libc::c_int;
    }
    if (symtab[fwd as usize].data).is_null() {
        let mut i: u_int = (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<u_char>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong)
            .wrapping_add((*data).len as libc::c_ulong) as u_int;
        if (*var).slen as libc::c_int != 0 as libc::c_int
            || (i as libc::c_ulong)
                < (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_char>() as libc::c_ulong)
                            .wrapping_mul(20 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        ::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong,
                    )
        {
            i = (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<u_char>() as libc::c_ulong)
                        .wrapping_mul(20 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong)
                as u_int;
        }
        newPtrDt = malloc(i as libc::c_ulong) as *mut ST_data;
        if newPtrDt.is_null() {
            return -(56 as libc::c_int + 200 as libc::c_int);
        }
        if (*var).slen as libc::c_int == 0 as libc::c_int {
            (*newPtrDt).deplnk = 0 as *mut libc::c_void as *mut ST_depend;
            (*newPtrDt).attach = 1 as libc::c_int as libc::c_short;
            (*newPtrDt).dbc = (*data).len;
            memcpy(
                &mut (*newPtrDt).data as *mut [u_char; 65535] as *mut libc::c_void,
                &mut *((*data).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                ((*data).len as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
            );
        } else {
            newPtrDp = malloc(
                (::core::mem::size_of::<u_char>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong,
                    )
                    .wrapping_add((*data).len as libc::c_ulong)
                    .wrapping_add((*var).slen as libc::c_ulong)
                    .wrapping_add(pad as libc::c_ulong),
            ) as *mut ST_depend;
            if newPtrDp.is_null() {
                free(newPtrDt as *mut libc::c_void);
                return -(56 as libc::c_int + 200 as libc::c_int);
            }
            (*newPtrDt).dbc = (65534 as libc::c_int + 1 as libc::c_int) as u_short;
            (*newPtrDt).deplnk = newPtrDp;
            (*newPtrDt).attach = 1 as libc::c_int as libc::c_short;
            (*newPtrDp).deplnk = 0 as *mut libc::c_void as *mut ST_depend;
            (*newPtrDp).keylen = (*var).slen;
            n = (*var).slen as libc::c_int;
            memcpy(
                &mut *((*newPtrDp).bytes).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_void,
                &mut *((*var).key).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                n as libc::c_ulong,
            );
            if n & 1 as libc::c_int != 0 {
                n += 1;
                n;
            }
            ptr2ushort = &mut *((*newPtrDp).bytes).as_mut_ptr().offset(n as isize)
                as *mut u_char as *mut u_short;
            *ptr2ushort = (*data).len;
            n += 2 as libc::c_int;
            memcpy(
                &mut *((*newPtrDp).bytes).as_mut_ptr().offset(n as isize) as *mut u_char
                    as *mut libc::c_void,
                &mut *((*data).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                ((*data).len as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
            );
        }
        symtab[fwd as usize].data = newPtrDt;
    } else if (*var).slen as libc::c_int == 0 as libc::c_int {
        newPtrDt = realloc(
            symtab[fwd as usize].data as *mut libc::c_void,
            (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<u_char>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong)
                .wrapping_add((*data).len as libc::c_ulong),
        ) as *mut ST_data;
        if newPtrDt.is_null() {
            return -(56 as libc::c_int + 200 as libc::c_int);
        }
        if newPtrDt != symtab[fwd as usize].data
            && (*newPtrDt).attach as libc::c_int > 1 as libc::c_int
        {
            FixData(
                symtab[fwd as usize].data,
                newPtrDt,
                (*newPtrDt).attach as libc::c_int,
            );
        } else {
            symtab[fwd as usize].data = newPtrDt;
        }
        (*newPtrDt).dbc = (*data).len;
        memcpy(
            &mut (*newPtrDt).data as *mut [u_char; 65535] as *mut libc::c_void,
            &mut *((*data).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            ((*data).len as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
    } else {
        newPtrDp = malloc(
            (::core::mem::size_of::<u_char>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong)
                .wrapping_add((*data).len as libc::c_ulong)
                .wrapping_add((*var).slen as libc::c_ulong)
                .wrapping_add(pad as libc::c_ulong),
        ) as *mut ST_depend;
        if newPtrDp.is_null() {
            return -(56 as libc::c_int + 200 as libc::c_int);
        }
        (*newPtrDp).deplnk = 0 as *mut libc::c_void as *mut ST_depend;
        (*newPtrDp).keylen = (*var).slen;
        n = (*var).slen as libc::c_int;
        memcpy(
            &mut *((*newPtrDp).bytes).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_void,
            &mut *((*var).key).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            n as libc::c_ulong,
        );
        if n & 1 as libc::c_int != 0 {
            n += 1;
            n;
        }
        ptr2ushort = &mut *((*newPtrDp).bytes).as_mut_ptr().offset(n as isize)
            as *mut u_char as *mut u_short;
        *ptr2ushort = (*data).len;
        n += 2 as libc::c_int;
        memcpy(
            &mut *((*newPtrDp).bytes).as_mut_ptr().offset(n as isize) as *mut u_char
                as *mut libc::c_void,
            &mut *((*data).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            ((*data).len as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
        ptr1 = (*symtab[fwd as usize].data).deplnk;
        if !ptr1.is_null() {
            while !ptr1.is_null()
                && UTIL_Key_KeyCmp(
                    ((*var).key).as_mut_ptr(),
                    ((*ptr1).bytes).as_mut_ptr(),
                    (*var).slen as libc::c_int,
                    (*ptr1).keylen as libc::c_int,
                ) > 0 as libc::c_int
            {
                prevPtr = ptr1;
                ptr1 = (*ptr1).deplnk;
                if ptr1.is_null() {
                    break;
                }
            }
            if !ptr1.is_null()
                && UTIL_Key_KeyCmp(
                    ((*ptr1).bytes).as_mut_ptr(),
                    ((*var).key).as_mut_ptr(),
                    (*ptr1).keylen as libc::c_int,
                    (*var).slen as libc::c_int,
                ) == 0 as libc::c_int
            {
                if prevPtr.is_null() {
                    (*newPtrDp).deplnk = (*ptr1).deplnk;
                    (*symtab[fwd as usize].data).deplnk = newPtrDp;
                    free(ptr1 as *mut libc::c_void);
                } else {
                    (*newPtrDp).deplnk = (*ptr1).deplnk;
                    (*prevPtr).deplnk = newPtrDp;
                    free(ptr1 as *mut libc::c_void);
                }
            } else if !ptr1.is_null()
                && UTIL_Key_KeyCmp(
                    ((*var).key).as_mut_ptr(),
                    ((*ptr1).bytes).as_mut_ptr(),
                    (*var).slen as libc::c_int,
                    (*ptr1).keylen as libc::c_int,
                ) < 0 as libc::c_int
            {
                if !ptr1.is_null() {
                    if prevPtr.is_null() {
                        (*newPtrDp).deplnk = ptr1;
                        (*symtab[fwd as usize].data).deplnk = newPtrDp;
                    } else {
                        (*newPtrDp).deplnk = ptr1;
                        (*prevPtr).deplnk = newPtrDp;
                    }
                }
            } else if ptr1.is_null() && !prevPtr.is_null() {
                (*newPtrDp).deplnk = 0 as *mut libc::c_void as *mut ST_depend;
                (*prevPtr).deplnk = newPtrDp;
            }
        } else {
            (*symtab[fwd as usize].data).deplnk = newPtrDp;
        }
    }
    return (*data).len as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Data(
    mut var: *mut mvar,
    mut buf: *mut u_char,
) -> libc::c_short {
    let mut ptr1: libc::c_int = 0;
    let mut depPtr: *mut ST_depend = 0 as *mut ST_depend;
    if (*var).volset != 0 {
        ptr1 = ST_LocateIdx((*var).volset as libc::c_int - 1 as libc::c_int)
            as libc::c_int;
    } else {
        ptr1 = ST_Locate((*var).name) as libc::c_int;
    }
    if ptr1 == -(1 as libc::c_int) {
        memcpy(
            buf as *mut libc::c_void,
            b"0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
        return 1 as libc::c_int as libc::c_short;
    }
    if (symtab[ptr1 as usize].data).is_null() {
        memcpy(
            buf as *mut libc::c_void,
            b"0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
        return 1 as libc::c_int as libc::c_short;
    }
    if (*var).slen as libc::c_int > 0 as libc::c_int {
        let mut i: libc::c_int = (*var).slen as libc::c_int;
        depPtr = (*symtab[ptr1 as usize].data).deplnk;
        if !depPtr.is_null() {
            if ((*depPtr).keylen as libc::c_int) < i {
                i = (*depPtr).keylen as libc::c_int;
            }
        }
        while !depPtr.is_null()
            && memcmp(
                ((*depPtr).bytes).as_mut_ptr() as *const libc::c_void,
                ((*var).key).as_mut_ptr() as *const libc::c_void,
                i as libc::c_ulong,
            ) < 0 as libc::c_int
        {
            depPtr = (*depPtr).deplnk;
            if depPtr.is_null() {
                break;
            }
            i = (*var).slen as libc::c_int;
            if ((*depPtr).keylen as libc::c_int) < i {
                i = (*depPtr).keylen as libc::c_int;
            }
        }
        if depPtr.is_null() {
            memcpy(
                buf as *mut libc::c_void,
                b"0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            return 1 as libc::c_int as libc::c_short;
        }
        i = (*var).slen as libc::c_int;
        if ((*depPtr).keylen as libc::c_int) < i {
            i = (*depPtr).keylen as libc::c_int;
        }
        while !depPtr.is_null()
            && memcmp(
                ((*depPtr).bytes).as_mut_ptr() as *const libc::c_void,
                ((*var).key).as_mut_ptr() as *const libc::c_void,
                i as libc::c_ulong,
            ) == 0 as libc::c_int
            && ((*depPtr).keylen as libc::c_int) < (*var).slen as libc::c_int
            || !depPtr.is_null()
                && memcmp(
                    ((*depPtr).bytes).as_mut_ptr() as *const libc::c_void,
                    ((*var).key).as_mut_ptr() as *const libc::c_void,
                    i as libc::c_ulong,
                ) < 0 as libc::c_int
        {
            depPtr = (*depPtr).deplnk;
            if depPtr.is_null() {
                break;
            }
            i = (*var).slen as libc::c_int;
            if ((*depPtr).keylen as libc::c_int) < i {
                i = (*depPtr).keylen as libc::c_int;
            }
        }
        if depPtr.is_null() {
            memcpy(
                buf as *mut libc::c_void,
                b"0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            return 1 as libc::c_int as libc::c_short;
        }
        if !depPtr.is_null()
            && memcmp(
                ((*depPtr).bytes).as_mut_ptr() as *const libc::c_void,
                ((*var).key).as_mut_ptr() as *const libc::c_void,
                i as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            if (*depPtr).keylen as libc::c_int == (*var).slen as libc::c_int {
                depPtr = (*depPtr).deplnk;
                if depPtr.is_null() {
                    memcpy(
                        buf as *mut libc::c_void,
                        b"1\0\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        2 as libc::c_int as libc::c_ulong,
                    );
                    return 1 as libc::c_int as libc::c_short;
                }
                i = (*var).slen as libc::c_int;
                if ((*depPtr).keylen as libc::c_int) < i {
                    i = (*depPtr).keylen as libc::c_int;
                }
                if memcmp(
                    ((*depPtr).bytes).as_mut_ptr() as *const libc::c_void,
                    ((*var).key).as_mut_ptr() as *const libc::c_void,
                    i as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    memcpy(
                        buf as *mut libc::c_void,
                        b"11\0\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        3 as libc::c_int as libc::c_ulong,
                    );
                    return 2 as libc::c_int as libc::c_short;
                } else {
                    memcpy(
                        buf as *mut libc::c_void,
                        b"1\0\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        2 as libc::c_int as libc::c_ulong,
                    );
                    return 1 as libc::c_int as libc::c_short;
                }
            } else {
                memcpy(
                    buf as *mut libc::c_void,
                    b"10\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                );
                return 2 as libc::c_int as libc::c_short;
            }
        } else {
            memcpy(
                buf as *mut libc::c_void,
                b"0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            return 1 as libc::c_int as libc::c_short;
        }
    } else {
        if (*symtab[ptr1 as usize].data).dbc as libc::c_int
            != 65534 as libc::c_int + 1 as libc::c_int
            && ((*symtab[ptr1 as usize].data).deplnk).is_null()
        {
            memcpy(
                buf as *mut libc::c_void,
                b"1\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            return 1 as libc::c_int as libc::c_short;
        }
        if (*symtab[ptr1 as usize].data).dbc as libc::c_int
            == 65534 as libc::c_int + 1 as libc::c_int
            && !((*symtab[ptr1 as usize].data).deplnk).is_null()
        {
            memcpy(
                buf as *mut libc::c_void,
                b"10\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            );
            return 2 as libc::c_int as libc::c_short;
        }
        if (*symtab[ptr1 as usize].data).dbc as libc::c_int
            != 65534 as libc::c_int + 1 as libc::c_int
            && !((*symtab[ptr1 as usize].data).deplnk).is_null()
        {
            memcpy(
                buf as *mut libc::c_void,
                b"11\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            );
            return 2 as libc::c_int as libc::c_short;
        }
    }
    memcpy(
        buf as *mut libc::c_void,
        b"0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    return 1 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Order(
    mut var: *mut mvar,
    mut buf: *mut u_char,
    mut dir: libc::c_int,
) -> libc::c_short {
    let mut ptr1: libc::c_int = 0;
    let mut current: *mut ST_depend = 0 as *mut ST_depend;
    let mut prev: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    let mut pieces: libc::c_int = 0 as libc::c_int;
    let mut subs: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut keysub: [libc::c_char; 256] = [0; 256];
    let mut upOneLev: [u_char; 256] = [0; 256];
    let mut crud: [u_char; 256] = [0; 256];
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_short = 0 as libc::c_int as libc::c_short;
    if (*var).volset != 0 {
        ptr1 = ST_LocateIdx((*var).volset as libc::c_int - 1 as libc::c_int)
            as libc::c_int;
    } else {
        ptr1 = ST_Locate((*var).name) as libc::c_int;
    }
    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if ptr1 < 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    if (*var).slen as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    UTIL_Key_Chars_In_Subs(
        ((*var).key).as_mut_ptr() as *mut libc::c_char,
        (*var).slen as libc::c_int,
        255 as libc::c_int,
        &mut pieces,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    upOneLev[0 as libc::c_int
        as usize] = UTIL_Key_Chars_In_Subs(
        ((*var).key).as_mut_ptr() as *mut libc::c_char,
        (*var).slen as libc::c_int,
        pieces - 1 as libc::c_int,
        &mut subs,
        &mut *upOneLev.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut u_char
            as *mut libc::c_char,
    ) as u_char;
    if (symtab[ptr1 as usize].data).is_null() {
        return 0 as libc::c_int as libc::c_short;
    }
    current = (*symtab[ptr1 as usize].data).deplnk;
    while !current.is_null()
        && UTIL_Key_KeyCmp(
            ((*var).key).as_mut_ptr(),
            ((*current).bytes).as_mut_ptr(),
            (*var).slen as libc::c_int,
            (*current).keylen as libc::c_int,
        ) > 0 as libc::c_int
    {
        prev = current;
        current = (*current).deplnk;
    }
    if current.is_null() {
        if dir == 1 as libc::c_int {
            return 0 as libc::c_int as libc::c_short;
        }
    }
    if dir == -(1 as libc::c_int) {
        current = prev;
        if current.is_null() {
            return 0 as libc::c_int as libc::c_short;
        }
        crud[0 as libc::c_int
            as usize] = UTIL_Key_Chars_In_Subs(
            ((*current).bytes).as_mut_ptr() as *mut libc::c_char,
            (*current).keylen as libc::c_int,
            pieces - 1 as libc::c_int,
            &mut subs,
            &mut *crud.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut u_char
                as *mut libc::c_char,
        ) as u_char;
        if crud[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
            && upOneLev[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
        {
            if crud[0 as libc::c_int as usize] as libc::c_int
                != upOneLev[0 as libc::c_int as usize] as libc::c_int
            {
                return 0 as libc::c_int as libc::c_short;
            }
            if memcmp(
                &mut *crud.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut u_char
                    as *const libc::c_void,
                &mut *upOneLev.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                upOneLev[0 as libc::c_int as usize] as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                return 0 as libc::c_int as libc::c_short;
            }
        }
    } else {
        while !current.is_null()
            && memcmp(
                ((*current).bytes).as_mut_ptr() as *const libc::c_void,
                ((*var).key).as_mut_ptr() as *const libc::c_void,
                (*var).slen as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            current = (*current).deplnk;
        }
        if current.is_null() {
            return 0 as libc::c_int as libc::c_short;
        }
        if UTIL_Key_KeyCmp(
            ((*var).key).as_mut_ptr(),
            ((*current).bytes).as_mut_ptr(),
            (*var).slen as libc::c_int,
            (*current).keylen as libc::c_int,
        ) != 0 as libc::c_int
        {
            crud[0 as libc::c_int
                as usize] = UTIL_Key_Chars_In_Subs(
                ((*current).bytes).as_mut_ptr() as *mut libc::c_char,
                (*current).keylen as libc::c_int,
                pieces - 1 as libc::c_int,
                &mut subs,
                &mut *crud.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut u_char
                    as *mut libc::c_char,
            ) as u_char;
            if crud[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                && upOneLev[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
            {
                if memcmp(
                    &mut *crud.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    &mut *upOneLev.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    upOneLev[0 as libc::c_int as usize] as libc::c_ulong,
                ) != 0 as libc::c_int
                {
                    return 0 as libc::c_int as libc::c_short;
                }
            }
        }
    }
    if current.is_null() {
        return 0 as libc::c_int as libc::c_short;
    }
    i = 1 as libc::c_int;
    while i <= pieces {
        let mut upto: libc::c_int = 0 as libc::c_int;
        ret = UTIL_Key_Extract(
            &mut *((*current).bytes).as_mut_ptr().offset(index as isize),
            keysub.as_mut_ptr() as *mut u_char,
            &mut upto,
        );
        index = index + upto;
        if index >= (*current).keylen as libc::c_int && i < pieces {
            return 0 as libc::c_int as libc::c_short;
        }
        i += 1;
        i;
    }
    return mcopy(keysub.as_mut_ptr() as *mut u_char, buf, ret as libc::c_int)
        as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Query(
    mut var: *mut mvar,
    mut buf: *mut u_char,
    mut dir: libc::c_int,
) -> libc::c_short {
    let mut ptr1: libc::c_int = 0;
    let mut current: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    let mut prev: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    let mut askeylen: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut outputVar: mvar = *var;
    if (*var).volset != 0 {
        ptr1 = ST_LocateIdx((*var).volset as libc::c_int - 1 as libc::c_int)
            as libc::c_int;
    } else {
        ptr1 = ST_Locate((*var).name) as libc::c_int;
    }
    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if ptr1 < 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    if (symtab[ptr1 as usize].data).is_null() {
        return 0 as libc::c_int as libc::c_short;
    }
    current = (*symtab[ptr1 as usize].data).deplnk;
    if current.is_null() {
        return 0 as libc::c_int as libc::c_short;
    }
    while !current.is_null()
        && UTIL_Key_KeyCmp(
            ((*var).key).as_mut_ptr(),
            ((*current).bytes).as_mut_ptr(),
            (*var).slen as libc::c_int,
            (*current).keylen as libc::c_int,
        ) > 0 as libc::c_int
    {
        prev = current;
        current = (*current).deplnk;
    }
    if (*var).slen as libc::c_int > 0 as libc::c_int {
        if dir == -(1 as libc::c_int) {
            if !prev.is_null() {
                current = prev;
            }
        } else if !current.is_null()
            && UTIL_Key_KeyCmp(
                ((*var).key).as_mut_ptr(),
                ((*current).bytes).as_mut_ptr(),
                (*var).slen as libc::c_int,
                (*current).keylen as libc::c_int,
            ) == 0 as libc::c_int
        {
            current = (*current).deplnk;
        }
    }
    if current.is_null() {
        return 0 as libc::c_int as libc::c_short;
    }
    if dir == -(1 as libc::c_int) && (*var).slen as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    outputVar.slen = (*current).keylen;
    memcpy(
        (outputVar.key).as_mut_ptr() as *mut libc::c_void,
        ((*current).bytes).as_mut_ptr() as *const libc::c_void,
        (*current).keylen as libc::c_int as libc::c_ulong,
    );
    if current == (*symtab[ptr1 as usize].data).deplnk && prev != current
        && dir == -(1 as libc::c_int) && (*var).slen as libc::c_int > 0 as libc::c_int
    {
        outputVar.slen = 0 as libc::c_int as u_char;
    }
    askeylen = UTIL_String_Mvar(&mut outputVar, buf, 63 as libc::c_int);
    return askeylen;
}
#[no_mangle]
pub unsafe extern "C" fn ST_GetAdd(
    mut var: *mut mvar,
    mut add: *mut *mut cstring,
) -> libc::c_int {
    let mut ptr1: libc::c_int = 0;
    let mut depPtr: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    let mut i: libc::c_int = 0;
    if (*var).volset != 0 {
        ptr1 = ST_LocateIdx((*var).volset as libc::c_int - 1 as libc::c_int)
            as libc::c_int;
    } else {
        ptr1 = ST_Locate((*var).name) as libc::c_int;
    }
    if ptr1 > (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int
        || ptr1 < -(1 as libc::c_int)
    {
        panic(
            b"ST_GetAdd: Junk pointer returned from ST_LocateIdx\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    } else if ptr1 >= 0 as libc::c_int {
        if (symtab[ptr1 as usize].data).is_null() {
            return -(6 as libc::c_int);
        }
        if (*var).slen as libc::c_int > 0 as libc::c_int {
            depPtr = (*symtab[ptr1 as usize].data).deplnk;
            while !depPtr.is_null() {
                i = UTIL_Key_KeyCmp(
                    ((*var).key).as_mut_ptr(),
                    ((*depPtr).bytes).as_mut_ptr(),
                    (*var).slen as libc::c_int,
                    (*depPtr).keylen as libc::c_int,
                );
                if i == -(1 as libc::c_int) {
                    return -(6 as libc::c_int);
                }
                if i == 0 as libc::c_int {
                    break;
                }
                depPtr = (*depPtr).deplnk;
            }
            if depPtr.is_null() {
                return -(6 as libc::c_int);
            }
            i = (*depPtr).keylen as libc::c_int;
            if i & 1 as libc::c_int != 0 {
                i += 1;
                i;
            }
            *add = &mut *((*depPtr).bytes).as_mut_ptr().offset(i as isize) as *mut u_char
                as *mut cstring;
            return (**add).len as libc::c_int;
        } else {
            *add = &mut (*(*symtab.as_mut_ptr().offset(ptr1 as isize)).data).dbc
                as *mut u_short as *mut cstring;
            i = (*symtab[ptr1 as usize].data).dbc as libc::c_int;
            if i == 65534 as libc::c_int + 1 as libc::c_int {
                return -(6 as libc::c_int);
            }
            return i;
        }
    }
    return -(6 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ST_Dump() -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut cdata: *mut cstring = 0 as *mut cstring;
    let mut dump: [u_char; 512] = [0; 512];
    let mut depPtr: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    i = 0 as libc::c_int;
    while i < (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
        if !(symtab[i as usize].data).is_null() {
            if !(symtab[i as usize].varnam.var_cu[0 as libc::c_int as usize]
                as libc::c_int == '$' as i32)
            {
                let mut var_i: u_int = 0 as libc::c_int as u_int;
                while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    partab
                        .src_var
                        .name
                        .var_qu[var_i
                        as usize] = symtab[i as usize].varnam.var_qu[var_i as usize];
                    var_i = var_i.wrapping_add(1);
                    var_i;
                }
                partab.src_var.uci = 255 as libc::c_int as u_char;
                partab.src_var.slen = 0 as libc::c_int as u_char;
                partab.src_var.volset = 0 as libc::c_int as u_char;
                cdata = &mut *dump.as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut cstring;
                if (*symtab[i as usize].data).dbc as libc::c_int
                    != 65534 as libc::c_int + 1 as libc::c_int
                {
                    (*cdata)
                        .len = UTIL_String_Mvar(
                        &mut partab.src_var,
                        ((*cdata).buf).as_mut_ptr(),
                        63 as libc::c_int,
                    ) as u_short;
                    let fresh2 = (*cdata).len;
                    (*cdata).len = ((*cdata).len).wrapping_add(1);
                    (*cdata).buf[fresh2 as usize] = '=' as i32 as u_char;
                    s = SQ_Write(cdata);
                    if s < 0 as libc::c_int {
                        return s as libc::c_short;
                    }
                    s = SQ_Write(
                        &mut (*(*symtab.as_mut_ptr().offset(i as isize)).data).dbc
                            as *mut u_short as *mut cstring,
                    );
                    if s < 0 as libc::c_int {
                        return s as libc::c_short;
                    }
                    s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                    if s < 0 as libc::c_int {
                        return s as libc::c_short;
                    }
                }
                cdata = 0 as *mut cstring;
                depPtr = (*symtab[i as usize].data).deplnk;
                while !depPtr.is_null() {
                    let mut var_i_0: u_int = 0 as libc::c_int as u_int;
                    while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        partab
                            .src_var
                            .name
                            .var_qu[var_i_0
                            as usize] = symtab[i as usize]
                            .varnam
                            .var_qu[var_i_0 as usize];
                        var_i_0 = var_i_0.wrapping_add(1);
                        var_i_0;
                    }
                    partab.src_var.uci = 255 as libc::c_int as u_char;
                    partab.src_var.slen = (*depPtr).keylen;
                    partab.src_var.volset = 0 as libc::c_int as u_char;
                    memcpy(
                        (partab.src_var.key).as_mut_ptr() as *mut libc::c_void,
                        ((*depPtr).bytes).as_mut_ptr() as *const libc::c_void,
                        (*depPtr).keylen as libc::c_ulong,
                    );
                    cdata = &mut *dump.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut u_char as *mut cstring;
                    (*cdata)
                        .len = UTIL_String_Mvar(
                        &mut partab.src_var,
                        ((*cdata).buf).as_mut_ptr(),
                        63 as libc::c_int,
                    ) as u_short;
                    let fresh3 = (*cdata).len;
                    (*cdata).len = ((*cdata).len).wrapping_add(1);
                    (*cdata).buf[fresh3 as usize] = '=' as i32 as u_char;
                    s = SQ_Write(cdata);
                    if s < 0 as libc::c_int {
                        return s as libc::c_short;
                    }
                    j = (*depPtr).keylen as libc::c_int;
                    if j & 1 as libc::c_int != 0 as libc::c_int {
                        j += 1;
                        j;
                    }
                    s = SQ_Write(
                        &mut *((*depPtr).bytes).as_mut_ptr().offset(j as isize)
                            as *mut u_char as *mut cstring,
                    );
                    if s < 0 as libc::c_int {
                        return s as libc::c_short;
                    }
                    s = SQ_WriteFormat(-(1 as libc::c_int)) as libc::c_int;
                    if s < 0 as libc::c_int {
                        return s as libc::c_short;
                    }
                    depPtr = (*depPtr).deplnk;
                }
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_QueryD(
    mut var: *mut mvar,
    mut buf: *mut u_char,
) -> libc::c_int {
    let mut ptr1: libc::c_int = 0;
    let mut cdata: *mut cstring = 0 as *mut cstring;
    let mut current: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    let mut i: libc::c_int = 0;
    if (*var).volset != 0 {
        ptr1 = ST_LocateIdx((*var).volset as libc::c_int - 1 as libc::c_int)
            as libc::c_int;
    } else {
        ptr1 = ST_Locate((*var).name) as libc::c_int;
    }
    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if ptr1 < 0 as libc::c_int {
        return -(6 as libc::c_int);
    }
    if (symtab[ptr1 as usize].data).is_null() {
        return -(6 as libc::c_int);
    }
    current = (*symtab[ptr1 as usize].data).deplnk;
    if current.is_null() {
        return -(55 as libc::c_int + 200 as libc::c_int);
    }
    while !current.is_null()
        && UTIL_Key_KeyCmp(
            ((*var).key).as_mut_ptr(),
            ((*current).bytes).as_mut_ptr(),
            (*var).slen as libc::c_int,
            (*current).keylen as libc::c_int,
        ) > 0 as libc::c_int
    {
        current = (*current).deplnk;
    }
    if !current.is_null()
        && UTIL_Key_KeyCmp(
            ((*var).key).as_mut_ptr(),
            ((*current).bytes).as_mut_ptr(),
            (*var).slen as libc::c_int,
            (*current).keylen as libc::c_int,
        ) == 0 as libc::c_int
    {
        current = (*current).deplnk;
    }
    if current.is_null() {
        return -(55 as libc::c_int + 200 as libc::c_int);
    }
    memcpy(
        ((*var).key).as_mut_ptr() as *mut libc::c_void,
        ((*current).bytes).as_mut_ptr() as *const libc::c_void,
        (*current).keylen as libc::c_ulong,
    );
    (*var).slen = (*current).keylen;
    i = (*current).keylen as libc::c_int;
    if i & 1 as libc::c_int != 0 as libc::c_int {
        i += 1;
        i;
    }
    cdata = &mut *((*current).bytes).as_mut_ptr().offset(i as isize) as *mut u_char
        as *mut cstring;
    return mcopy(((*cdata).buf).as_mut_ptr(), buf, (*cdata).len as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ST_DumpV(mut global: *mut mvar) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_short = 0;
    let mut t: libc::c_int = 0;
    let mut gs: libc::c_short = 0;
    let mut gks: [u_char; 255] = [0; 255];
    let mut cdata: *mut cstring = 0 as *mut cstring;
    let mut dump: [u_char; 1024] = [0; 1024];
    let mut depPtr: *mut ST_depend = 0 as *mut libc::c_void as *mut ST_depend;
    cdata = dump.as_mut_ptr() as *mut cstring;
    partab.src_var.uci = 255 as libc::c_int as u_char;
    partab.src_var.volset = 0 as libc::c_int as u_char;
    gs = (*global).slen as libc::c_short;
    memcpy(
        gks.as_mut_ptr() as *mut libc::c_void,
        ((*global).key).as_mut_ptr() as *const libc::c_void,
        (*global).slen as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
        if !(symtab[i as usize].data).is_null() {
            if !(symtab[i as usize].varnam.var_cu[0 as libc::c_int as usize]
                as libc::c_int == '$' as i32)
            {
                if !(var_empty(symtab[i as usize].varnam) != 0) {
                    let mut var_i: u_int = 0 as libc::c_int as u_int;
                    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        partab
                            .src_var
                            .name
                            .var_qu[var_i
                            as usize] = symtab[i as usize].varnam.var_qu[var_i as usize];
                        var_i = var_i.wrapping_add(1);
                        var_i;
                    }
                    partab.src_var.slen = 0 as libc::c_int as u_char;
                    if (*symtab[i as usize].data).dbc as libc::c_int
                        != 65534 as libc::c_int + 1 as libc::c_int
                    {
                        s = UTIL_String_Mvar(
                            &mut partab.src_var,
                            ((*cdata).buf).as_mut_ptr(),
                            63 as libc::c_int,
                        );
                        if (s as libc::c_int) < 0 as libc::c_int {
                            return s;
                        }
                        (*cdata).len = s as u_short;
                        memcpy(
                            ((*global).key).as_mut_ptr() as *mut libc::c_void,
                            gks.as_mut_ptr() as *const libc::c_void,
                            gs as libc::c_ulong,
                        );
                        (*global).slen = gs as u_char;
                        (*global)
                            .slen = ((*global).slen as libc::c_int
                            + UTIL_Key_Build(
                                cdata,
                                &mut *((*global).key).as_mut_ptr().offset(gs as isize),
                            ) as libc::c_int) as u_char;
                        t = DB_Set(
                            global,
                            &mut (*(*symtab.as_mut_ptr().offset(i as isize)).data).dbc
                                as *mut u_short as *mut cstring,
                        );
                        if t == -(75 as libc::c_int) {
                            j = (*symtab[i as usize].data).dbc as libc::c_int;
                            (*symtab[i as usize].data)
                                .dbc = 900 as libc::c_int as u_short;
                            t = DB_Set(
                                global,
                                &mut (*(*symtab.as_mut_ptr().offset(i as isize)).data).dbc
                                    as *mut u_short as *mut cstring,
                            );
                            (*symtab[i as usize].data).dbc = j as u_short;
                        }
                    }
                    depPtr = (*symtab[i as usize].data).deplnk;
                    while !depPtr.is_null() {
                        partab.src_var.slen = (*depPtr).keylen;
                        memcpy(
                            (partab.src_var.key).as_mut_ptr() as *mut libc::c_void,
                            ((*depPtr).bytes).as_mut_ptr() as *const libc::c_void,
                            (*depPtr).keylen as libc::c_ulong,
                        );
                        cdata = &mut *dump.as_mut_ptr().offset(0 as libc::c_int as isize)
                            as *mut u_char as *mut cstring;
                        s = UTIL_String_Mvar(
                            &mut partab.src_var,
                            ((*cdata).buf).as_mut_ptr(),
                            63 as libc::c_int,
                        );
                        if (s as libc::c_int) < 0 as libc::c_int {
                            return s;
                        }
                        (*cdata).len = s as u_short;
                        j = (*depPtr).keylen as libc::c_int;
                        if j & 1 as libc::c_int != 0 as libc::c_int {
                            j += 1;
                            j;
                        }
                        memcpy(
                            ((*global).key).as_mut_ptr() as *mut libc::c_void,
                            gks.as_mut_ptr() as *const libc::c_void,
                            gs as libc::c_ulong,
                        );
                        (*global).slen = gs as u_char;
                        (*global)
                            .slen = ((*global).slen as libc::c_int
                            + UTIL_Key_Build(
                                cdata,
                                &mut *((*global).key).as_mut_ptr().offset(gs as isize),
                            ) as libc::c_int) as u_char;
                        t = DB_Set(
                            global,
                            &mut *((*depPtr).bytes).as_mut_ptr().offset(j as isize)
                                as *mut u_char as *mut cstring,
                        );
                        if t < 0 as libc::c_int {
                            return t as libc::c_short;
                        }
                        depPtr = (*depPtr).deplnk;
                    }
                }
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_KillAll(
    mut count: libc::c_int,
    mut keep: *mut var_u,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    partab.src_var.uci = 255 as libc::c_int as u_char;
    partab.src_var.slen = 0 as libc::c_int as u_char;
    partab.src_var.volset = 0 as libc::c_int as u_char;
    i = 0 as libc::c_int;
    while i < (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
        if !(symtab[i as usize].varnam.var_cu[0 as libc::c_int as usize] as libc::c_int
            == '$' as i32
            || symtab[i as usize].varnam.var_cu[0 as libc::c_int as usize] as libc::c_int
                == '\0' as i32)
        {
            if !(symtab[i as usize].data).is_null() {
                j = 0 as libc::c_int;
                while j < count {
                    if var_equal(symtab[i as usize].varnam, *keep.offset(j as isize))
                        != 0
                    {
                        break;
                    }
                    j += 1;
                    j;
                }
                if !(j < count) {
                    let mut var_i: u_int = 0 as libc::c_int as u_int;
                    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        partab
                            .src_var
                            .name
                            .var_qu[var_i
                            as usize] = symtab[i as usize].varnam.var_qu[var_i as usize];
                        var_i = var_i.wrapping_add(1);
                        var_i;
                    }
                    ST_Kill(&mut partab.src_var);
                }
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_SymAtt(mut var: var_u) -> libc::c_short {
    let mut pos: libc::c_short = 0;
    pos = ST_Create(var);
    if pos as libc::c_int >= 0 as libc::c_int {
        symtab[pos as usize].usage += 1;
        symtab[pos as usize].usage;
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn ST_SymDet(
    mut count: libc::c_int,
    mut list: *mut libc::c_short,
) {
    let mut i: libc::c_int = 0;
    let mut current_block_6: u64;
    i = 0 as libc::c_int;
    while i < count {
        if *list.offset(i as isize) as libc::c_int >= 0 as libc::c_int {
            symtab[*list.offset(i as isize) as usize].usage -= 1;
            symtab[*list.offset(i as isize) as usize].usage;
            if !(symtab[*list.offset(i as isize) as usize].usage as libc::c_int
                > 0 as libc::c_int)
            {
                if !(symtab[*list.offset(i as isize) as usize].data).is_null() {
                    if (*symtab[*list.offset(i as isize) as usize].data).dbc
                        as libc::c_int != 65534 as libc::c_int + 1 as libc::c_int
                    {
                        current_block_6 = 11174649648027449784;
                    } else if !((*symtab[*list.offset(i as isize) as usize].data).deplnk)
                        .is_null()
                    {
                        current_block_6 = 11174649648027449784;
                    } else {
                        free(
                            symtab[*list.offset(i as isize) as usize].data
                                as *mut libc::c_void,
                        );
                        symtab[*list.offset(i as isize) as usize]
                            .data = 0 as *mut libc::c_void as *mut ST_data;
                        current_block_6 = 13513818773234778473;
                    }
                } else {
                    current_block_6 = 13513818773234778473;
                }
                match current_block_6 {
                    11174649648027449784 => {}
                    _ => {
                        if (symtab[*list.offset(i as isize) as usize].data).is_null() {
                            ST_Free(symtab[*list.offset(i as isize) as usize].varnam);
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
    free(list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ST_SymSet(
    mut pos: libc::c_short,
    mut data: *mut cstring,
) -> libc::c_short {
    let mut i: u_int = 0;
    let mut ptr: *mut ST_data = 0 as *mut ST_data;
    i = (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<u_char>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong)
        .wrapping_add((*data).len as libc::c_ulong) as u_int;
    if (i as libc::c_ulong)
        < (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<u_char>() as libc::c_ulong)
                    .wrapping_mul(20 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong)
    {
        i = (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<u_char>() as libc::c_ulong)
                    .wrapping_mul(20 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong)
            as u_int;
    }
    if (symtab[pos as usize].data).is_null() {
        symtab[pos as usize].data = malloc(i as libc::c_ulong) as *mut ST_DATA;
        if (symtab[pos as usize].data).is_null() {
            return -(56 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        (*symtab[pos as usize].data).deplnk = 0 as *mut libc::c_void as *mut ST_depend;
        (*symtab[pos as usize].data).attach = 1 as libc::c_int as libc::c_short;
    } else if ((*symtab[pos as usize].data).dbc as libc::c_int)
        < (*data).len as libc::c_int
    {
        ptr = realloc(symtab[pos as usize].data as *mut libc::c_void, i as libc::c_ulong)
            as *mut ST_data;
        if ptr.is_null() {
            return -(56 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        if ptr != symtab[pos as usize].data
            && (*ptr).attach as libc::c_int > 1 as libc::c_int
        {
            FixData(symtab[pos as usize].data, ptr, (*ptr).attach as libc::c_int);
        } else {
            symtab[pos as usize].data = ptr;
        }
    }
    (*symtab[pos as usize].data).dbc = (*data).len;
    memcpy(
        &mut (*(*symtab.as_mut_ptr().offset(pos as isize)).data).data
            as *mut [u_char; 65535] as *mut libc::c_void,
        &mut *((*data).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_void,
        ((*data).len as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
    );
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ST_SymKill(mut pos: libc::c_short) -> libc::c_short {
    let mut dptr: *mut ST_depend = 0 as *mut ST_depend;
    let mut fptr: *mut ST_depend = 0 as *mut ST_depend;
    if !(symtab[pos as usize].data).is_null() {
        dptr = (*symtab[pos as usize].data).deplnk;
        (*symtab[pos as usize].data).deplnk = 0 as *mut libc::c_void as *mut ST_depend;
        while !dptr.is_null() {
            fptr = dptr;
            dptr = (*fptr).deplnk;
            free(fptr as *mut libc::c_void);
        }
        if ((*symtab[pos as usize].data).attach as libc::c_int) < 2 as libc::c_int {
            free(symtab[pos as usize].data as *mut libc::c_void);
            symtab[pos as usize].data = 0 as *mut libc::c_void as *mut ST_data;
        }
    }
    if (symtab[pos as usize].usage as libc::c_int) < 1 as libc::c_int {
        ST_Free(symtab[pos as usize].varnam);
    }
    return 0 as libc::c_int as libc::c_short;
}
