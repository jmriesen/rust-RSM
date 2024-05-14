#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __error() -> *mut libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    fn getError(type_0: libc::c_int, errnum: libc::c_int) -> libc::c_int;
}
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type u_char = libc::c_uchar;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
#[no_mangle]
pub unsafe extern "C" fn SQ_File_Open(
    mut file: *mut libc::c_char,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut flag: libc::c_int = 0;
    let mut fid: libc::c_int = 0;
    match op {
        1 => {
            flag = 0x1 as libc::c_int | 0x400 as libc::c_int | 0x200 as libc::c_int;
        }
        2 => {
            flag = 0 as libc::c_int;
        }
        3 => {
            flag = 0x1 as libc::c_int | 0x8 as libc::c_int | 0x200 as libc::c_int;
        }
        4 => {
            flag = 0x2 as libc::c_int | 0x200 as libc::c_int;
        }
        _ => return getError(1 as libc::c_int, 21 as libc::c_int),
    }
    fid = open(
        file,
        flag,
        0o400 as libc::c_int | 0o200 as libc::c_int | 0o40 as libc::c_int
            | 0o4 as libc::c_int,
    );
    if fid == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    return fid;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_File_Write(
    mut fid: libc::c_int,
    mut writebuf: *mut u_char,
    mut nbytes: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = write(fid, writebuf as *const libc::c_void, nbytes as size_t) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_File_Read(
    mut fid: libc::c_int,
    mut readbuf: *mut u_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = read(fid, readbuf as *mut libc::c_void, 1 as libc::c_int as size_t)
        as libc::c_int;
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error())
    } else {
        return ret
    };
}
