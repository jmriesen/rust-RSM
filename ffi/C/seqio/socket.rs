#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __error() -> *mut libc::c_int;
    fn accept(_: libc::c_int, _: *mut sockaddr, _: *mut socklen_t) -> libc::c_int;
    fn bind(_: libc::c_int, _: *const sockaddr, _: socklen_t) -> libc::c_int;
    fn connect(_: libc::c_int, _: *const sockaddr, _: socklen_t) -> libc::c_int;
    fn listen(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn recv(_: libc::c_int, _: *mut libc::c_void, _: size_t, _: libc::c_int) -> ssize_t;
    fn send(
        _: libc::c_int,
        _: *const libc::c_void,
        _: size_t,
        _: libc::c_int,
    ) -> ssize_t;
    fn setsockopt(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_void,
        _: socklen_t,
    ) -> libc::c_int;
    fn socket(_: libc::c_int, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn inet_pton(
        _: libc::c_int,
        _: *const libc::c_char,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn fcntl(_: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    fn raise(_: libc::c_int) -> libc::c_int;
    fn close(_: libc::c_int) -> libc::c_int;
    fn seqioSelect(
        sid: libc::c_int,
        type_0: libc::c_int,
        tout: libc::c_int,
    ) -> libc::c_int;
    fn getError(type_0: libc::c_int, errnum: libc::c_int) -> libc::c_int;
    static mut proto_family: libc::c_short;
    static mut addr_family: libc::c_short;
    static mut sock_type: libc::c_short;
    static mut sock_proto: libc::c_short;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = libc::c_long;
pub type u_int32_t = libc::c_uint;
pub type size_t = __darwin_size_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type in_addr_t = __uint32_t;
pub type in_port_t = __uint16_t;
pub type ssize_t = __darwin_ssize_t;
pub type sa_family_t = __uint8_t;
pub type socklen_t = __darwin_socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_len: __uint8_t,
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_len: __uint8_t,
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_char; 8],
}
#[inline]
unsafe extern "C" fn _OSSwapInt16(mut _data: __uint16_t) -> __uint16_t {
    return ((_data as libc::c_int) << 8 as libc::c_int
        | _data as libc::c_int >> 8 as libc::c_int) as __uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Socket_Create(mut nonblock: libc::c_int) -> libc::c_int {
    let mut sid: libc::c_int = 0;
    sid = socket(
        proto_family as libc::c_int,
        sock_type as libc::c_int,
        sock_proto as libc::c_int,
    );
    if sid == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    if nonblock != 0 {
        let mut flag: libc::c_int = fcntl(sid, 3 as libc::c_int, 0 as libc::c_int);
        let mut ret: libc::c_int = 0;
        if flag == -(1 as libc::c_int) {
            close(sid);
            return getError(0 as libc::c_int, *__error());
        }
        flag |= 0x4 as libc::c_int;
        ret = fcntl(sid, 4 as libc::c_int, flag);
        if ret == -(1 as libc::c_int) {
            close(sid);
            return getError(0 as libc::c_int, *__error());
        }
    }
    return sid;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Socket_Bind(
    mut sid: libc::c_int,
    mut port: u_short,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut soid: libc::c_int = 0;
    let mut sin: sockaddr_in = sockaddr_in {
        sin_len: 0,
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    opt = 1 as libc::c_int;
    soid = setsockopt(
        sid,
        0xffff as libc::c_int,
        0x4 as libc::c_int,
        &mut opt as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if soid == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    sin.sin_family = addr_family as sa_family_t;
    sin
        .sin_port = (if 0 != 0 {
        ((port as libc::c_uint & 0xff00 as libc::c_uint) >> 8 as libc::c_int
            | (port as libc::c_uint & 0xff as libc::c_uint) << 8 as libc::c_int)
            as __uint16_t as libc::c_int
    } else {
        _OSSwapInt16(port) as libc::c_int
    }) as __uint16_t;
    sin.sin_addr.s_addr = 0 as libc::c_int as u_int32_t;
    ret = bind(
        sid,
        &mut sin as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Socket_Listen(mut sid: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = listen(sid, 5 as libc::c_int);
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Socket_Accept(
    mut sid: libc::c_int,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_len: 0,
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    ret = seqioSelect(sid, 0 as libc::c_int, tout);
    if ret < 0 as libc::c_int {
        return ret;
    }
    len = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_int;
    ret = accept(
        sid,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        &mut len as *mut libc::c_int as *mut socklen_t,
    );
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Socket_Connect(
    mut sid: libc::c_int,
    mut addr: *mut libc::c_char,
    mut port: u_short,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut inaddr: in_addr = in_addr { s_addr: 0 };
    let mut sin: sockaddr_in = sockaddr_in {
        sin_len: 0,
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    sin.sin_family = addr_family as sa_family_t;
    sin
        .sin_port = (if 0 != 0 {
        ((port as libc::c_uint & 0xff00 as libc::c_uint) >> 8 as libc::c_int
            | (port as libc::c_uint & 0xff as libc::c_uint) << 8 as libc::c_int)
            as __uint16_t as libc::c_int
    } else {
        _OSSwapInt16(port) as libc::c_int
    }) as __uint16_t;
    ret = inet_pton(
        addr_family as libc::c_int,
        addr,
        &mut inaddr as *mut in_addr as *mut libc::c_void,
    );
    if ret == 0 as libc::c_int {
        return getError(1 as libc::c_int, 48 as libc::c_int);
    }
    sin.sin_addr.s_addr = inaddr.s_addr;
    ret = connect(
        sid,
        &mut sin as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if ret == -(1 as libc::c_int) {
        if *__error() == 36 as libc::c_int {
            ret = seqioSelect(sid, 1 as libc::c_int, -(1 as libc::c_int));
            if ret < 0 as libc::c_int {
                return ret;
            }
            return sid;
        }
        return getError(0 as libc::c_int, *__error());
    }
    return sid;
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Socket_Write(
    mut sid: libc::c_int,
    mut writebuf: *mut u_char,
    mut nbytes: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = send(sid, writebuf as *const libc::c_void, nbytes as size_t, 0 as libc::c_int)
        as libc::c_int;
    if ret == -(1 as libc::c_int) {
        if *__error() == 32 as libc::c_int {
            return getError(1 as libc::c_int, 46 as libc::c_int)
        } else if *__error() == 35 as libc::c_int {
            return 0 as libc::c_int
        } else {
            return getError(0 as libc::c_int, *__error())
        }
    } else {
        return ret
    };
}
#[no_mangle]
pub unsafe extern "C" fn SQ_Socket_Read(
    mut sid: libc::c_int,
    mut readbuf: *mut u_char,
    mut tout: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if tout != 0 as libc::c_int {
        ret = seqioSelect(sid, 0 as libc::c_int, tout);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    ret = recv(
        sid,
        readbuf as *mut libc::c_void,
        1 as libc::c_int as size_t,
        0 as libc::c_int,
    ) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        if *__error() == 35 as libc::c_int {
            if raise(14 as libc::c_int) != 0 {
                return getError(0 as libc::c_int, *__error());
            }
        }
        return getError(0 as libc::c_int, *__error());
    } else if ret == 0 as libc::c_int {
        return ret
    } else {
        return ret
    };
}
