#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __error() -> *mut libc::c_int;
    fn sigaction(_: libc::c_int, _: *const sigaction, _: *mut sigaction) -> libc::c_int;
    fn setSignalBitMask(sig: libc::c_int);
    fn getError(type_0: libc::c_int, errnum: libc::c_int) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_uid_t = __uint32_t;
pub type pid_t = __darwin_pid_t;
pub type sigset_t = __darwin_sigset_t;
pub type uid_t = __darwin_uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __siginfo {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub si_pid: pid_t,
    pub si_uid: uid_t,
    pub si_status: libc::c_int,
    pub si_addr: *mut libc::c_void,
    pub si_value: sigval,
    pub si_band: libc::c_long,
    pub __pad: [libc::c_ulong; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __sigaction_u {
    pub __sa_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub __sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut __siginfo, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_mask: sigset_t,
    pub sa_flags: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn setSignal(
    mut sig: libc::c_int,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut action: sigaction = sigaction {
        __sigaction_u: __sigaction_u {
            __sa_handler: None,
        },
        sa_mask: 0,
        sa_flags: 0,
    };
    let mut handlermask: sigset_t = 0;
    let mut ret: libc::c_int = 0;
    handlermask = 0 as libc::c_int as sigset_t;
    handlermask = !(0 as libc::c_int as sigset_t);
    action.sa_mask = handlermask;
    action.sa_flags = 0 as libc::c_int;
    if flag == 0 as libc::c_int {
        action
            .__sigaction_u
            .__sa_handler = Some(
            signalHandler as unsafe extern "C" fn(libc::c_int) -> (),
        );
    } else {
        action
            .__sigaction_u
            .__sa_handler = ::core::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t);
    }
    ret = sigaction(sig, &mut action, 0 as *mut sigaction);
    if ret == -(1 as libc::c_int) {
        return getError(0 as libc::c_int, *__error());
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn setSignals() -> libc::c_int {
    let mut action: sigaction = sigaction {
        __sigaction_u: __sigaction_u {
            __sa_handler: None,
        },
        sa_mask: 0,
        sa_flags: 0,
    };
    let mut handlermask: sigset_t = 0;
    handlermask = 0 as libc::c_int as sigset_t;
    handlermask = !(0 as libc::c_int as sigset_t);
    action.sa_mask = handlermask;
    action.sa_flags = 0 as libc::c_int;
    action
        .__sigaction_u
        .__sa_handler = ::core::mem::transmute::<
        libc::intptr_t,
        Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    >(1 as libc::c_int as libc::intptr_t);
    if sigaction(1 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action
        .__sigaction_u
        .__sa_handler = Some(signalHandler as unsafe extern "C" fn(libc::c_int) -> ());
    if sigaction(2 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action.sa_flags = 0x2 as libc::c_int;
    if sigaction(3 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action.sa_flags = 0 as libc::c_int;
    action.__sigaction_u.__sa_handler = None;
    if sigaction(4 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action
        .__sigaction_u
        .__sa_handler = Some(signalHandler as unsafe extern "C" fn(libc::c_int) -> ());
    if sigaction(5 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(6 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action.__sigaction_u.__sa_handler = None;
    if sigaction(8 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(10 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(11 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action
        .__sigaction_u
        .__sa_handler = Some(signalHandler as unsafe extern "C" fn(libc::c_int) -> ());
    if sigaction(13 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(14 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(15 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(16 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action.__sigaction_u.__sa_handler = None;
    if sigaction(18 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(19 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(21 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(22 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action
        .__sigaction_u
        .__sa_handler = Some(signalHandler as unsafe extern "C" fn(libc::c_int) -> ());
    if sigaction(23 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(24 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(25 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(26 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(27 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action
        .__sigaction_u
        .__sa_handler = ::core::mem::transmute::<
        libc::intptr_t,
        Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    >(1 as libc::c_int as libc::intptr_t);
    if sigaction(28 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    action
        .__sigaction_u
        .__sa_handler = Some(signalHandler as unsafe extern "C" fn(libc::c_int) -> ());
    if sigaction(30 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    if sigaction(31 as libc::c_int, &mut action, 0 as *mut sigaction)
        == -(1 as libc::c_int)
    {
        return getError(0 as libc::c_int, *__error());
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn signalHandler(mut sig: libc::c_int) {
    setSignalBitMask(sig);
}
