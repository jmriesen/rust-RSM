#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __error() -> *mut libc::c_int;
    fn mkfifo(_: *const libc::c_char, _: mode_t) -> libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn raise(_: libc::c_int) -> libc::c_int;
    fn close(_: libc::c_int) -> libc::c_int;
    fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    fn unlink(_: *const libc::c_char) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    fn seqioSelect(
        sid: libc::c_int,
        type_0: libc::c_int,
        tout: libc::c_int,
    ) -> libc::c_int;
    fn getError(type_0: libc::c_int, errnum: libc::c_int) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_mode_t = __uint16_t;
pub type mode_t = __darwin_mode_t;
pub type u_char = libc::c_uchar;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
#[no_mangle]
pub unsafe extern "C" fn SQ_Pipe_Open(
    mut pipe: *mut libc::c_char,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    match op {
        10 => {
            ret = createPipe(pipe);
            if ret < 0 as libc::c_int {
                return ret;
            }
            flag = 0 as libc::c_int | 0x4 as libc::c_int;
        }
        9 => {
            flag = 0x1 as libc::c_int;
        }
        _ => return getError(1 as libc::c_int, 21 as libc::c_int),
    }
    pid = open(pipe, flag, 0 as libc::c_int);
    if pid == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    return pid;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Pipe_Close(
    mut pid: libc::c_int,
    mut pipe: *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut oid: libc::c_int = 0;
    ret = close(pid);
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    oid = open(pipe, 0x1 as libc::c_int | 0x4 as libc::c_int, 0 as libc::c_int);
    if oid == -(1 as libc::c_int) {
        ret = unlink(pipe);
        if ret == -(1 as libc::c_int) {
            return getError(0 as libc::c_int, *__error());
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Pipe_Write(
    mut pid: libc::c_int,
    mut writebuf: *mut u_char,
    mut nbytes: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = write(pid, writebuf as *const libc::c_void, nbytes as size_t) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        if *__error() == 32 as libc::c_int {
            return getError(1 as libc::c_int, 46 as libc::c_int)
        } else {
            return getError(0 as libc::c_int, *__error())
        }
    } else {
        return ret
    };
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Pipe_Read(
    mut pid: libc::c_int,
    mut readbuf: *mut u_char,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut bytesread: libc::c_int = 0;
    loop {
        ret = seqioSelect(pid, 0 as libc::c_int, tout);
        if ret < 0 as libc::c_int {
            return ret;
        }
        bytesread = read(pid, readbuf as *mut libc::c_void, 1 as libc::c_int as size_t)
            as libc::c_int;
        if bytesread == -(1 as libc::c_int) && *__error() == 35 as libc::c_int {
            sleep(1 as libc::c_int as libc::c_uint);
        } else if bytesread == -(1 as libc::c_int) {
            if *__error() == 35 as libc::c_int {
                sleep(1 as libc::c_int as libc::c_uint);
                *__error() = 0 as libc::c_int;
            } else {
                return getError(0 as libc::c_int, *__error())
            }
        } else if bytesread == 0 as libc::c_int {
            if tout == 0 as libc::c_int {
                if raise(14 as libc::c_int) != 0 {
                    return getError(0 as libc::c_int, *__error());
                }
                return -(1 as libc::c_int);
            }
            sleep(1 as libc::c_int as libc::c_uint);
            return 0 as libc::c_int;
        } else {
            return 1 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn createPipe(mut pipe: *mut libc::c_char) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = mkfifo(
        pipe,
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o40 as libc::c_int
            | 0o4 as libc::c_int) as mode_t,
    );
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error())
    } else {
        return ret
    };
}
