#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn atoi(_: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn close(_: libc::c_int) -> libc::c_int;
    fn getError(type_0: libc::c_int, errnum: libc::c_int) -> libc::c_int;
    fn SQ_Socket_Create(nonblock: libc::c_int) -> libc::c_int;
    fn SQ_Socket_Bind(sid: libc::c_int, port: u_short) -> libc::c_int;
    fn SQ_Socket_Listen(sid: libc::c_int) -> libc::c_int;
    fn SQ_Socket_Accept(sid: libc::c_int, tout: libc::c_int) -> libc::c_int;
    fn SQ_Socket_Connect(
        sid: libc::c_int,
        addr: *mut libc::c_char,
        port: u_short,
    ) -> libc::c_int;
    fn SQ_Socket_Write(
        sid: libc::c_int,
        writebuf: *mut u_char,
        nbytes: libc::c_int,
    ) -> libc::c_int;
    fn SQ_Socket_Read(
        sid: libc::c_int,
        readbuf: *mut u_char,
        tout: libc::c_int,
    ) -> libc::c_int;
}
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
#[no_mangle]
pub unsafe extern "C" fn SQ_Tcpip_Open(
    mut bind: *mut libc::c_char,
    mut op: libc::c_int,
) -> libc::c_int {
    match op {
        6 => return SQ_Tcpip_Open_Server(bind),
        5 => return SQ_Tcpip_Open_Client(bind),
        _ => return getError(1 as libc::c_int, 21 as libc::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Tcpip_Accept(
    mut sid: libc::c_int,
    mut tout: libc::c_int,
) -> libc::c_int {
    return SQ_Socket_Accept(sid, tout);
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Tcpip_Write(
    mut sid: libc::c_int,
    mut writebuf: *mut u_char,
    mut nbytes: libc::c_int,
) -> libc::c_int {
    return SQ_Socket_Write(sid, writebuf, nbytes);
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Tcpip_Read(
    mut sid: libc::c_int,
    mut readbuf: *mut u_char,
    mut tout: libc::c_int,
) -> libc::c_int {
    return SQ_Socket_Read(sid, readbuf, tout);
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Tcpip_Open_Server(
    mut bind: *mut libc::c_char,
) -> libc::c_int {
    let mut sid: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut port: u_short = 0;
    sid = SQ_Socket_Create(1 as libc::c_int);
    if sid < 0 as libc::c_int {
        return sid;
    }
    port = atoi(bind) as u_short;
    ret = SQ_Socket_Bind(sid, port);
    if ret < 0 as libc::c_int {
        close(sid);
        return ret;
    }
    ret = SQ_Socket_Listen(sid);
    if ret < 0 as libc::c_int {
        close(sid);
        return ret;
    }
    return sid;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Tcpip_Open_Client(
    mut conn: *mut libc::c_char,
) -> libc::c_int {
    let mut sid: libc::c_int = 0;
    let mut portptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addrptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: u_short = 0;
    let mut ret: libc::c_int = 0;
    let mut xxxx: [libc::c_char; 100] = [0; 100];
    strcpy(xxxx.as_mut_ptr(), conn);
    sid = SQ_Socket_Create(0 as libc::c_int);
    if sid < 0 as libc::c_int {
        return sid;
    }
    portptr = strpbrk(xxxx.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    if portptr.is_null() {
        close(sid);
        return getError(1 as libc::c_int, 28 as libc::c_int);
    }
    *portptr = '\0' as i32 as libc::c_char;
    addrptr = xxxx.as_mut_ptr();
    portptr = portptr.offset(1);
    portptr;
    port = atoi(portptr) as u_short;
    ret = SQ_Socket_Connect(sid, addrptr, port);
    if ret < 0 as libc::c_int {
        close(sid);
        return ret;
    }
    return sid;
}
