#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut blk: [*mut gbd; 12];
    static mut level: libc::c_int;
    static mut Index: u_int;
    static mut chunk: *mut cstring;
    static mut record: *mut cstring;
    static mut keybuf: [u_char; 260];
    static mut idx: *mut u_short;
    static mut iidx: *mut libc::c_int;
    fn Get_block(blknum: u_int) -> libc::c_short;
    fn UTIL_Key_KeyCmp(
        key1: *mut u_char,
        key2: *mut u_char,
        kleng1: libc::c_int,
        kleng2: libc::c_int,
    ) -> libc::c_int;
}
pub type __darwin_time_t = libc::c_long;
pub type time_t = __darwin_time_t;
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
pub type DB_Block = DB_BLOCK;
pub type gbd = GBD;
#[no_mangle]
pub unsafe extern "C" fn Locate(mut key: *mut u_char) -> libc::c_short {
    idx = (*blk[level as usize]).mem as *mut u_short;
    iidx = (*blk[level as usize]).mem as *mut libc::c_int;
    Index = ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int;
    loop {
        let mut i: libc::c_int = 0;
        chunk = &mut *iidx.offset(*idx.offset(Index as isize) as isize)
            as *mut libc::c_int as *mut cstring;
        memcpy(
            &mut *keybuf
                .as_mut_ptr()
                .offset(
                    (*((*chunk).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 1 as libc::c_int) as isize,
                ) as *mut u_char as *mut libc::c_void,
            &mut *((*chunk).buf).as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            (*chunk).buf[1 as libc::c_int as usize] as libc::c_ulong,
        );
        keybuf[0 as libc::c_int
            as usize] = ((*chunk).buf[0 as libc::c_int as usize] as libc::c_int
            + (*chunk).buf[1 as libc::c_int as usize] as libc::c_int) as u_char;
        record = &mut *((*chunk).buf)
            .as_mut_ptr()
            .offset(
                (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as libc::c_int + 2 as libc::c_int) as isize,
            ) as *mut u_char as *mut cstring;
        i = UTIL_Key_KeyCmp(
            &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize),
            &mut *key.offset(1 as libc::c_int as isize),
            keybuf[0 as libc::c_int as usize] as libc::c_int,
            *key.offset(0 as libc::c_int as isize) as libc::c_int,
        );
        if i == 0 as libc::c_int {
            return 0 as libc::c_int as libc::c_short;
        }
        if i == 1 as libc::c_int {
            return -(7 as libc::c_int) as libc::c_short;
        }
        Index = Index.wrapping_add(1);
        Index;
        if Index > (*(*blk[level as usize]).mem).0.last_idx as u_int {
            return -(7 as libc::c_int) as libc::c_short;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Locate_next() -> libc::c_short {
    Index = Index.wrapping_add(1);
    Index;
    if Index > (*(*blk[level as usize]).mem).0.last_idx as u_int {
        let mut i: libc::c_int = 0;
        let mut s: libc::c_short = 0;
        if (*(*blk[level as usize]).mem).0.right_ptr == 0 {
            return -(7 as libc::c_int) as libc::c_short;
        }
        i = (*(*blk[level as usize]).mem).0.right_ptr as libc::c_int;
        s = Get_block(i as u_int);
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
    }
    chunk = &mut *iidx.offset(*idx.offset(Index as isize) as isize) as *mut libc::c_int
        as *mut cstring;
    memcpy(
        &mut *keybuf
            .as_mut_ptr()
            .offset(
                (*((*chunk).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as libc::c_int + 1 as libc::c_int) as isize,
            ) as *mut u_char as *mut libc::c_void,
        &mut *((*chunk).buf).as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut u_char as *const libc::c_void,
        (*chunk).buf[1 as libc::c_int as usize] as libc::c_ulong,
    );
    keybuf[0 as libc::c_int
        as usize] = ((*chunk).buf[0 as libc::c_int as usize] as libc::c_int
        + (*chunk).buf[1 as libc::c_int as usize] as libc::c_int) as u_char;
    record = &mut *((*chunk).buf)
        .as_mut_ptr()
        .offset(
            (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                as libc::c_int + 2 as libc::c_int) as isize,
        ) as *mut u_char as *mut cstring;
    return 0 as libc::c_int as libc::c_short;
}
