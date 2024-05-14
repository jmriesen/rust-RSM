#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type GBD;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut partab: partab_struct;
    fn cstringtoi(str: *mut cstring) -> libc::c_int;
    fn uitocstring(buf: *mut u_char, n: u_int) -> u_short;
    fn rsm_version(ret_buffer: *mut u_char) -> libc::c_int;
    fn current_time(local: libc::c_short) -> time_t;
    fn ST_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn ST_Set(var: *mut mvar, data: *mut cstring) -> libc::c_int;
    fn ST_Kill(var: *mut mvar) -> libc::c_short;
    fn UTIL_String_Mvar(
        var: *mut mvar,
        str: *mut u_char,
        max_subs: libc::c_int,
    ) -> libc::c_short;
    fn mcopy(src: *mut u_char, dst: *mut u_char, bytes: libc::c_int) -> libc::c_int;
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
pub unsafe extern "C" fn Vecode(mut ret_buffer: *mut u_char) -> libc::c_int {
    let mut var: *mut mvar = 0 as *mut mvar;
    let mut s: libc::c_int = 0;
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
    (*var).slen = 0 as libc::c_int as u_char;
    s = ST_Get(var, ret_buffer);
    if s == -(6 as libc::c_int) {
        s = 0 as libc::c_int;
        *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn Vetrap(mut ret_buffer: *mut u_char) -> libc::c_int {
    let mut var: *mut mvar = 0 as *mut mvar;
    let mut s: libc::c_int = 0;
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
        b"$ETRAP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    (*var).volset = 0 as libc::c_int as u_char;
    (*var).uci = 255 as libc::c_int as u_char;
    (*var).slen = 0 as libc::c_int as u_char;
    s = ST_Get(var, ret_buffer);
    if s == -(6 as libc::c_int) {
        s = 0 as libc::c_int;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn Vhorolog(mut ret_buffer: *mut u_char) -> libc::c_short {
    let mut sec: time_t = current_time(1 as libc::c_int as libc::c_short);
    let mut day: libc::c_int = (sec / 86400 as libc::c_int as time_t
        + 47117 as libc::c_int as time_t) as libc::c_int;
    sec %= 86400 as libc::c_int as time_t;
    return sprintf(
        ret_buffer as *mut libc::c_char,
        b"%d,%d\0" as *const u8 as *const libc::c_char,
        day,
        sec as libc::c_int,
    ) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Vkey(mut ret_buffer: *mut u_char) -> libc::c_short {
    let mut ioptr: *mut SQ_Chan = 0 as *mut SQ_Chan;
    ioptr = &mut *((*partab.jobtab).seqio)
        .as_mut_ptr()
        .offset((*partab.jobtab).io as libc::c_int as isize) as *mut SQ_Chan;
    return mcopy(
        &mut *((*ioptr).dkey).as_mut_ptr().offset(0 as libc::c_int as isize),
        ret_buffer,
        (*ioptr).dkey_len as libc::c_int,
    ) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Vreference(mut ret_buffer: *mut u_char) -> libc::c_short {
    let mut var: *mut mvar = 0 as *mut mvar;
    var = &mut (*partab.jobtab).last_ref;
    *ret_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int as libc::c_short;
    }
    return UTIL_String_Mvar(var, ret_buffer, 63 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Vsystem(mut ret_buffer: *mut u_char) -> libc::c_short {
    let mut i: libc::c_int = uitocstring(ret_buffer, 50 as libc::c_int as u_int)
        as libc::c_int;
    let fresh0 = i;
    i = i + 1;
    *ret_buffer.offset(fresh0 as isize) = ',' as i32 as u_char;
    i = i + rsm_version(&mut *ret_buffer.offset(i as isize));
    return i as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Vx(mut ret_buffer: *mut u_char) -> libc::c_short {
    let mut ioptr: *mut SQ_Chan = &mut *((*partab.jobtab).seqio)
        .as_mut_ptr()
        .offset((*partab.jobtab).io as libc::c_int as isize) as *mut SQ_Chan;
    return uitocstring(ret_buffer, (*ioptr).dx as u_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Vy(mut ret_buffer: *mut u_char) -> libc::c_short {
    let mut ioptr: *mut SQ_Chan = &mut *((*partab.jobtab).seqio)
        .as_mut_ptr()
        .offset((*partab.jobtab).io as libc::c_int as isize) as *mut SQ_Chan;
    return uitocstring(ret_buffer, (*ioptr).dy as u_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Vset(
    mut var: *mut mvar,
    mut cptr: *mut cstring,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*var).slen as libc::c_int != 0 as libc::c_int {
        return -(8 as libc::c_int);
    }
    if strncasecmp(
        &mut *((*var).name.var_cu).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_char,
        b"ec\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || strncasecmp(
            &mut *((*var).name.var_cu).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_char,
            b"ecode\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        if (*cptr).len as libc::c_int > 1 as libc::c_int
            && (*cptr).buf[0 as libc::c_int as usize] as libc::c_int == ',' as i32
            && (*cptr).buf[((*cptr).len as libc::c_int - 1 as libc::c_int) as usize]
                as libc::c_int == ',' as i32
        {
            (*cptr).len = ((*cptr).len).wrapping_sub(1);
            (*cptr).len;
            let fresh1 = (*cptr).len;
            (*cptr).len = ((*cptr).len).wrapping_sub(1);
            memmove(
                &mut *((*cptr).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_void,
                &mut *((*cptr).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                fresh1 as libc::c_ulong,
            );
            (*cptr).buf[(*cptr).len as usize] = '\0' as i32 as u_char;
        }
        if ((*cptr).len as libc::c_int == 0 as libc::c_int
            || (*cptr).buf[0 as libc::c_int as usize] as libc::c_int == 'M' as i32
            || (*cptr).buf[0 as libc::c_int as usize] as libc::c_int == 'Z' as i32
            || (*cptr).buf[0 as libc::c_int as usize] as libc::c_int == 'U' as i32)
            && (*cptr).buf[((*cptr).len as libc::c_int - 1 as libc::c_int) as usize]
                as libc::c_int != ',' as i32
        {
            let mut code: *mut libc::c_char = strtok(
                ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                b",\0" as *const u8 as *const libc::c_char,
            );
            loop {
                code = strtok(
                    0 as *mut libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                );
                if code.is_null() {
                    break;
                }
                if *code.offset(0 as libc::c_int as isize) as libc::c_int != 'M' as i32
                    && *code.offset(0 as libc::c_int as isize) as libc::c_int
                        != 'Z' as i32
                    && *code.offset(0 as libc::c_int as isize) as libc::c_int
                        != 'U' as i32
                {
                    return -(101 as libc::c_int);
                }
                *code.offset(-(1 as libc::c_int) as isize) = ',' as i32 as libc::c_char;
            }
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
            (*partab.jobtab).error_frame = 0 as libc::c_int as libc::c_short;
            (*partab.jobtab).etrap_at = 0 as libc::c_int as libc::c_short;
            if (*cptr).len as libc::c_int == 0 as libc::c_int {
                return ST_Kill(var) as libc::c_int;
            }
            return -(9999 as libc::c_int);
        }
        return -(101 as libc::c_int);
    }
    if strncasecmp(
        &mut *((*var).name.var_cu).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_char,
        b"et\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || strncasecmp(
            &mut *((*var).name.var_cu).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_char,
            b"etrap\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        let mut var_i_0: u_int = 0 as libc::c_int as u_int;
        while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            (*var).name.var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
            var_i_0 = var_i_0.wrapping_add(1);
            var_i_0;
        }
        memcpy(
            &mut *((*var).name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_void,
            b"$ETRAP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        );
        if (*cptr).len as libc::c_int == 0 as libc::c_int {
            return ST_Kill(var) as libc::c_int;
        }
        return ST_Set(var, cptr);
    }
    if strncasecmp(
        &mut *((*var).name.var_cu).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_char,
        b"k\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || strncasecmp(
            &mut *((*var).name.var_cu).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_char,
            b"key\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        if (*cptr).len as libc::c_int > 16 as libc::c_int {
            return -(75 as libc::c_int);
        }
        memcpy(
            ((*partab.jobtab).seqio[(*partab.jobtab).io as usize].dkey).as_mut_ptr()
                as *mut libc::c_void,
            ((*cptr).buf).as_mut_ptr() as *const libc::c_void,
            ((*cptr).len as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
        (*partab.jobtab)
            .seqio[(*partab.jobtab).io as usize]
            .dkey_len = (*cptr).len as libc::c_short;
        return 0 as libc::c_int;
    }
    if strncasecmp(
        &mut *((*var).name.var_cu).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_char,
        b"x\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        i = cstringtoi(cptr);
        if i < 0 as libc::c_int {
            i = 0 as libc::c_int;
        }
        (*partab.jobtab).seqio[(*partab.jobtab).io as usize].dx = i as u_short;
        return 0 as libc::c_int;
    }
    if strncasecmp(
        &mut *((*var).name.var_cu).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_char,
        b"y\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        i = cstringtoi(cptr);
        if i < 0 as libc::c_int {
            i = 0 as libc::c_int;
        }
        (*partab.jobtab).seqio[(*partab.jobtab).io as usize].dy = i as u_short;
        return 0 as libc::c_int;
    }
    return -(8 as libc::c_int);
}
