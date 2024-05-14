#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type __sFILEX;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __error() -> *mut libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn close(_: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    fn current_time(local: libc::c_short) -> time_t;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub _read: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub _seek: Option::<
        unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t,
    >,
    pub _write: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type ssize_t = __darwin_ssize_t;
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
pub type DB_Block = DB_BLOCK;
#[derive(Copy, Clone)]
#[repr(C)]
pub union temp_tag {
    pub buff: [libc::c_int; 131072],
    pub cuff: [libc::c_char; 2593],
}
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
pub unsafe extern "C" fn isalpha(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x100 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn INIT_Create_File(
    mut blocks: u_int,
    mut bsize: u_int,
    mut map: u_int,
    mut volnam: *mut libc::c_char,
    mut env: *mut libc::c_char,
    mut file: *mut libc::c_char,
) -> libc::c_int {
    let mut namlen: libc::c_int = 0;
    let mut envlen: libc::c_int = 0;
    let mut us: u_short = 0;
    let mut ret: libc::c_int = 0;
    let mut fid: libc::c_int = 0;
    let mut mgrblk: *mut DB_Block = 0 as *mut DB_Block;
    let mut labelblock: *mut label_block = 0 as *mut label_block;
    let mut hunk: *mut cstring = 0 as *mut cstring;
    let blklen: libc::c_int = 512 as libc::c_int * 1024 as libc::c_int;
    let mut x: temp_tag = temp_tag { buff: [0; 131072] };
    namlen = strlen(volnam) as libc::c_int;
    if namlen < 1 as libc::c_int || namlen > 32 as libc::c_int {
        fprintf(
            __stderrp,
            b"Volume set name must be from 1 to %d alpha characters\n\0" as *const u8
                as *const libc::c_char,
            32 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < namlen {
        if isalpha(*volnam.offset(i as isize) as libc::c_int) == 0 {
            fprintf(
                __stderrp,
                b"Volume set name must be from 1 to %d alpha characters\n\0" as *const u8
                    as *const libc::c_char,
                32 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    if !env.is_null() {
        envlen = strlen(env) as libc::c_int;
        if envlen < 1 as libc::c_int || envlen > 32 as libc::c_int {
            fprintf(
                __stderrp,
                b"Environment (UCI) name must be from 1 to %d alpha characters\n\0"
                    as *const u8 as *const libc::c_char,
                32 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < envlen {
            if isalpha(*env.offset(i_0 as isize) as libc::c_int) == 0 {
                fprintf(
                    __stderrp,
                    b"Environment (UCI) name must be from 1 to %d alpha characters\n\0"
                        as *const u8 as *const libc::c_char,
                    32 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
            i_0 += 1;
            i_0;
        }
    }
    if (bsize / 1024 as libc::c_int as u_int) < 1 as libc::c_int as u_int
        || bsize / 1024 as libc::c_int as u_int > 256 as libc::c_int as u_int
    {
        fprintf(
            __stderrp,
            b"Block size must be from 1 to 256 KiB\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if blocks < 100 as libc::c_int as u_int || blocks > 2147483647 as libc::c_uint {
        fprintf(
            __stderrp,
            b"Database size must be from 100 to %u blocks\n\0" as *const u8
                as *const libc::c_char,
            2147483647 as libc::c_uint,
        );
        return -(1 as libc::c_int);
    }
    blocks |= 7 as libc::c_int as u_int;
    if map == 0 as libc::c_int as u_int {
        map = ((blocks.wrapping_add(7 as libc::c_int as u_int)
            / 8 as libc::c_int as u_int)
            .wrapping_add(1 as libc::c_int as u_int) as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<label_block>() as libc::c_ulong)
            as u_int;
    }
    if map & 1023 as libc::c_int as u_int != 0 {
        map = (map / 1024 as libc::c_int as u_int)
            .wrapping_add(1 as libc::c_int as u_int) * 1024 as libc::c_int as u_int;
    }
    if map < bsize {
        map = bsize;
    }
    if (map as libc::c_ulong)
        < ((blocks.wrapping_add(7 as libc::c_int as u_int) / 8 as libc::c_int as u_int)
            .wrapping_add(1 as libc::c_int as u_int) as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<label_block>() as libc::c_ulong)
    {
        fprintf(
            __stderrp,
            b"Map block size of %u KiB smaller than required by database size\n\0"
                as *const u8 as *const libc::c_char,
            map / 1024 as libc::c_int as u_int,
        );
        return -(1 as libc::c_int);
    }
    if map
        > (((2147483647 as libc::c_uint).wrapping_div(8 as libc::c_int as libc::c_uint)
            as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<label_block>() as libc::c_ulong)
            as u_int / 1024 as libc::c_int as u_int)
            .wrapping_add(1 as libc::c_int as u_int) * 1024 as libc::c_int as u_int
    {
        fprintf(
            __stderrp,
            b"Map block size must be from 0 to %u KiB\n\0" as *const u8
                as *const libc::c_char,
            (((2147483647 as libc::c_uint).wrapping_div(8 as libc::c_int as libc::c_uint)
                as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<label_block>() as libc::c_ulong)
                as u_int / 1024 as libc::c_int as u_int)
                .wrapping_add(1 as libc::c_int as u_int),
        );
        return -(1 as libc::c_int);
    }
    printf(
        b"Creating volume set %s in file %s\n\0" as *const u8 as *const libc::c_char,
        volnam,
        file,
    );
    if !env.is_null() {
        printf(
            b"using %s as the name of the manager environment (UCI)\n\0" as *const u8
                as *const libc::c_char,
            env,
        );
    }
    printf(
        b"with %u x %u KiB blocks \0" as *const u8 as *const libc::c_char,
        blocks,
        bsize / 1024 as libc::c_int as u_int,
    );
    printf(
        b"and a %u KiB label/map block.\n\0" as *const u8 as *const libc::c_char,
        map / 1024 as libc::c_int as u_int,
    );
    ret = 0 as libc::c_int;
    *__error() = 0 as libc::c_int;
    fid = open(
        file,
        0x200 as libc::c_int | 0x400 as libc::c_int | 0x1 as libc::c_int
            | 0x800 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int | 0o40 as libc::c_int,
    );
    if fid < 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"Create of %s failed - %s\n\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*__error()),
        );
        return *__error();
    }
    labelblock = (x.buff).as_mut_ptr() as *mut label_block;
    memset(
        (x.buff).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (if map > blklen as u_int { blklen as u_int } else { map }) as libc::c_ulong,
    );
    (*labelblock).magic = 4155766917 as libc::c_uint;
    (*labelblock).max_block = blocks;
    (*labelblock).header_bytes = map;
    (*labelblock).block_size = bsize;
    (*labelblock)
        .creation_time = current_time(1 as libc::c_int as libc::c_short) as u_int64;
    memcpy(
        ((*labelblock).volnam.var_cu).as_mut_ptr() as *mut libc::c_void,
        volnam as *const libc::c_void,
        namlen as libc::c_ulong,
    );
    (*labelblock).db_ver = 2 as libc::c_int as u_short;
    (*labelblock).clean = 1 as libc::c_int as u_char;
    if !env.is_null() {
        memcpy(
            ((*labelblock).uci[0 as libc::c_int as usize].name.var_cu).as_mut_ptr()
                as *mut libc::c_void,
            env as *const libc::c_void,
            strlen(env),
        );
    } else {
        memcpy(
            ((*labelblock).uci[0 as libc::c_int as usize].name.var_cu).as_mut_ptr()
                as *mut libc::c_void,
            b"MGR\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        );
    }
    (*labelblock).uci[0 as libc::c_int as usize].global = 1 as libc::c_int as u_int;
    x
        .cuff[::core::mem::size_of::<label_block>() as libc::c_ulong
        as usize] = 3 as libc::c_int as libc::c_char;
    if map > blklen as u_int {
        ret = write(fid, (x.buff).as_mut_ptr() as *const libc::c_void, blklen as size_t)
            as libc::c_int;
        if ret < blklen {
            close(fid);
            fprintf(
                __stderrp,
                b"Database file write failed - %s\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__error()),
            );
            return *__error();
        }
        memset(
            (x.buff).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            blklen as libc::c_ulong,
        );
        let mut i_1: u_int = 0 as libc::c_int as u_int;
        while i_1 < (map / blklen as u_int).wrapping_sub(1 as libc::c_int as u_int) {
            ret = write(
                fid,
                (x.buff).as_mut_ptr() as *const libc::c_void,
                blklen as size_t,
            ) as libc::c_int;
            if ret < blklen {
                close(fid);
                fprintf(
                    __stderrp,
                    b"Database file write failed - %s\n\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__error()),
                );
                return *__error();
            }
            i_1 = i_1.wrapping_add(1);
            i_1;
        }
        if map % blklen as u_int != 0 {
            ret = write(
                fid,
                (x.buff).as_mut_ptr() as *const libc::c_void,
                (map % blklen as u_int) as size_t,
            ) as libc::c_int;
            if ret < (map % blklen as u_int) as libc::c_int {
                close(fid);
                fprintf(
                    __stderrp,
                    b"Database file write failed - %s\n\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__error()),
                );
                return *__error();
            }
        }
    } else {
        ret = write(fid, (x.buff).as_mut_ptr() as *const libc::c_void, map as size_t)
            as libc::c_int;
        if ret < map as libc::c_int {
            close(fid);
            fprintf(
                __stderrp,
                b"Database file write failed - %s\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__error()),
            );
            return *__error();
        }
    }
    mgrblk = (x.buff).as_mut_ptr() as *mut DB_Block;
    memset(
        (x.buff).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        bsize as libc::c_ulong,
    );
    (*mgrblk).0.type_0 = 65 as libc::c_int as u_char;
    (*mgrblk)
        .0
        .last_idx = (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int) as u_short;
    (*mgrblk)
        .0
        .last_free = (bsize / 4 as libc::c_int as u_int)
        .wrapping_sub(7 as libc::c_int as u_int) as u_short;
    memcpy(
        &mut (*mgrblk).0.global as *mut var_u as *mut libc::c_void,
        b"$GLOBAL\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        7 as libc::c_int as libc::c_ulong,
    );
    us = (bsize / 4 as libc::c_int as u_int).wrapping_sub(6 as libc::c_int as u_int)
        as u_short;
    memcpy(
        &mut *(x.cuff)
            .as_mut_ptr()
            .offset(::core::mem::size_of::<DB_Block>() as libc::c_ulong as isize)
            as *mut libc::c_char as *mut libc::c_void,
        &mut us as *mut u_short as *const libc::c_void,
        ::core::mem::size_of::<u_short>() as libc::c_ulong,
    );
    hunk = &mut *(x.buff).as_mut_ptr().offset(us as isize) as *mut libc::c_int
        as *mut cstring;
    (*hunk).len = 24 as libc::c_int as u_short;
    (*hunk).buf[1 as libc::c_int as usize] = 9 as libc::c_int as u_char;
    memcpy(
        &mut *((*hunk).buf).as_mut_ptr().offset(2 as libc::c_int as isize) as *mut u_char
            as *mut libc::c_void,
        b"\x80$GLOBAL\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        9 as libc::c_int as libc::c_ulong,
    );
    us = (us as libc::c_int + 4 as libc::c_int) as u_short;
    x.buff[us as usize] = 1 as libc::c_int;
    ret = write(fid, (x.buff).as_mut_ptr() as *const libc::c_void, bsize as size_t)
        as libc::c_int;
    if ret < bsize as libc::c_int {
        close(fid);
        fprintf(
            __stderrp,
            b"Database file write failed - %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__error()),
        );
        return *__error();
    }
    memset(
        (x.buff).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        bsize as libc::c_ulong,
    );
    let mut i_2: u_int = 0 as libc::c_int as u_int;
    while i_2 < blocks.wrapping_sub(1 as libc::c_int as u_int) {
        ret = write(fid, (x.buff).as_mut_ptr() as *const libc::c_void, bsize as size_t)
            as libc::c_int;
        if ret < 1 as libc::c_int {
            close(fid);
            fprintf(
                __stderrp,
                b"Database file write failed - %s\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__error()),
            );
            return *__error();
        }
        i_2 = i_2.wrapping_add(1);
        i_2;
    }
    close(fid);
    printf(b"Database file created.\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
