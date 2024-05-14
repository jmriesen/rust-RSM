#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(linkage)]
extern "C" {
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn eval();
    fn routine(runtime: libc::c_int) -> libc::c_short;
    fn mcopy(src: *mut u_char, dst: *mut u_char, bytes: libc::c_int) -> libc::c_int;
    static mut source_ptr: *mut u_char;
    static mut comp_ptr: *mut u_char;
    fn localvar() -> libc::c_short;
    fn atom();
    fn comperror(err: libc::c_short);
}
pub type __uint32_t = libc::c_uint;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
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
#[inline]
unsafe extern "C" fn __isctype(
    mut _c: __darwin_ct_rune_t,
    mut _f: libc::c_ulong,
) -> __darwin_ct_rune_t {
    return if _c < 0 as libc::c_int || _c >= (1 as libc::c_int) << 8 as libc::c_int {
        0 as libc::c_int
    } else {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0)
            as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isalpha(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x100 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x400 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn toupper(mut _c: libc::c_int) -> libc::c_int {
    return __toupper(_c);
}
#[no_mangle]
pub unsafe extern "C" fn parse2eq(mut ptr: *const u_char) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut q: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = i;
        i = i + 1;
        let mut c: u_char = *ptr.offset(fresh0 as isize);
        if c as libc::c_int == '\0' as i32 {
            break;
        }
        if c as libc::c_int == '"' as i32 {
            q = (q == 0) as libc::c_int;
        } else {
            if q != 0 {
                continue;
            }
            if c as libc::c_int == '(' as i32 {
                b += 1;
                b;
            } else if c as libc::c_int == ')' as i32 {
                b -= 1;
                b;
                if b < 0 as libc::c_int {
                    break;
                }
            } else {
                if b != 0 {
                    continue;
                }
                if c as libc::c_int == '=' as i32 || c as libc::c_int == ' ' as i32
                    || c as libc::c_int == ',' as i32
                {
                    break;
                }
            }
        }
    }
    return i - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parse_close() {
    loop {
        let mut iflag: libc::c_int = (*source_ptr as libc::c_int == '@' as i32)
            as libc::c_int;
        eval();
        if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
            == 65 as libc::c_int && iflag != 0
        {
            *comp_ptr
                .offset(-(1 as libc::c_int as isize)) = 181 as libc::c_int as u_char;
        } else {
            let fresh1 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh1 = 59 as libc::c_int as u_char;
        }
        if *source_ptr as libc::c_int != ',' as i32 {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_do(mut runtime: libc::c_int) {
    let mut s: libc::c_short = 0;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut p: *mut u_char = 0 as *mut u_char;
    let mut save: [u_char; 1024] = [0; 1024];
    let mut savecount: libc::c_int = 0;
    loop {
        let mut i: libc::c_int = 0;
        ptr = comp_ptr;
        let fresh2 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh2 = 141 as libc::c_int as u_char;
        i = routine(runtime) as libc::c_int;
        if i == 0 as libc::c_int {
            *ptr = 179 as libc::c_int as u_char;
            let fresh3 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh3 = 182 as libc::c_int as u_char;
            if *source_ptr as libc::c_int == '(' as i32 {
                comp_ptr = ptr;
                comperror(-(70 as libc::c_int + 200 as libc::c_int) as libc::c_short);
            }
        } else {
            let mut args: libc::c_int = 0 as libc::c_int;
            if i == -(2 as libc::c_int) {
                *ptr = 143 as libc::c_int as u_char;
            } else if i == -(3 as libc::c_int) {
                *ptr = 142 as libc::c_int as u_char;
            } else if i == -(4 as libc::c_int) {
                *ptr = 144 as libc::c_int as u_char;
            }
            if *source_ptr as libc::c_int == '(' as i32 {
                savecount = comp_ptr.offset_from(ptr) as libc::c_long as libc::c_int;
                memcpy(
                    save.as_mut_ptr() as *mut libc::c_void,
                    ptr as *const libc::c_void,
                    savecount as libc::c_ulong,
                );
                comp_ptr = ptr;
                source_ptr = source_ptr.offset(1);
                source_ptr;
                loop {
                    if args > 127 as libc::c_int - 1 as libc::c_int {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    args += 1;
                    args;
                    if *source_ptr as libc::c_int == ')' as i32 {
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                        break;
                    } else {
                        if *source_ptr as libc::c_int == ',' as i32
                            || *source_ptr as libc::c_int == ')' as i32
                        {
                            let fresh4 = comp_ptr;
                            comp_ptr = comp_ptr.offset(1);
                            *fresh4 = 169 as libc::c_int as u_char;
                        } else if *source_ptr as libc::c_int == '.' as i32
                            && isdigit(
                                *source_ptr.offset(1 as libc::c_int as isize) as libc::c_int,
                            ) == 0 as libc::c_int
                        {
                            source_ptr = source_ptr.offset(1);
                            source_ptr;
                            if *source_ptr as libc::c_int == '@' as i32 {
                                source_ptr = source_ptr.offset(1);
                                source_ptr;
                                atom();
                                let fresh5 = comp_ptr;
                                comp_ptr = comp_ptr.offset(1);
                                *fresh5 = 66 as libc::c_int as u_char;
                            } else {
                                p = comp_ptr;
                                s = localvar();
                                if (s as libc::c_int) < 0 as libc::c_int {
                                    comperror(s);
                                    return;
                                }
                                p = p.offset(s as libc::c_int as isize);
                                *p = 62 as libc::c_int as u_char;
                            }
                            let fresh6 = comp_ptr;
                            comp_ptr = comp_ptr.offset(1);
                            *fresh6 = 168 as libc::c_int as u_char;
                        } else {
                            eval();
                        }
                        if *source_ptr as libc::c_int == ')' as i32 {
                            continue;
                        }
                        if *source_ptr as libc::c_int == ',' as i32 {
                            source_ptr = source_ptr.offset(1);
                            source_ptr;
                        } else {
                            comperror(
                                -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                            );
                            return;
                        }
                    }
                }
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    save.as_mut_ptr() as *const libc::c_void,
                    savecount as libc::c_ulong,
                );
                comp_ptr = comp_ptr.offset(savecount as isize);
            }
            let fresh7 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh7 = args as u_char;
        }
        if *source_ptr as libc::c_int == ':' as i32 {
            savecount = comp_ptr.offset_from(ptr) as libc::c_long as libc::c_int;
            memcpy(
                save.as_mut_ptr() as *mut libc::c_void,
                ptr as *const libc::c_void,
                savecount as libc::c_ulong,
            );
            comp_ptr = ptr;
            source_ptr = source_ptr.offset(1);
            source_ptr;
            eval();
            let fresh8 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh8 = 5 as libc::c_int as u_char;
            s = savecount as libc::c_short;
            memcpy(
                comp_ptr as *mut libc::c_void,
                &mut s as *mut libc::c_short as *const libc::c_void,
                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
            );
            comp_ptr = comp_ptr
                .offset(
                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                );
            memcpy(
                comp_ptr as *mut libc::c_void,
                save.as_mut_ptr() as *const libc::c_void,
                savecount as libc::c_ulong,
            );
            comp_ptr = comp_ptr.offset(savecount as isize);
        }
        if *source_ptr as libc::c_int != ',' as i32 {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_goto(mut runtime: libc::c_int) {
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut save: [u_char; 1024] = [0; 1024];
    let mut savecount: libc::c_short = 0;
    loop {
        let mut i: libc::c_int = 0;
        ptr = comp_ptr;
        let fresh9 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh9 = 150 as libc::c_int as u_char;
        i = routine(runtime) as libc::c_int;
        if i == 0 as libc::c_int {
            *ptr = 179 as libc::c_int as u_char;
            let fresh10 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh10 = 183 as libc::c_int as u_char;
        } else if i == -(2 as libc::c_int) {
            *ptr = 152 as libc::c_int as u_char;
        } else if i == -(3 as libc::c_int) {
            *ptr = 151 as libc::c_int as u_char;
        } else if i == -(4 as libc::c_int) {
            *ptr = 153 as libc::c_int as u_char;
        }
        if *source_ptr as libc::c_int == '(' as i32 {
            comp_ptr = ptr;
            comperror(-(45 as libc::c_int) as libc::c_short);
        }
        if *source_ptr as libc::c_int == ':' as i32 {
            savecount = comp_ptr.offset_from(ptr) as libc::c_long as libc::c_short;
            memcpy(
                save.as_mut_ptr() as *mut libc::c_void,
                ptr as *const libc::c_void,
                savecount as libc::c_ulong,
            );
            comp_ptr = ptr;
            source_ptr = source_ptr.offset(1);
            source_ptr;
            eval();
            let fresh11 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh11 = 5 as libc::c_int as u_char;
            memcpy(
                comp_ptr as *mut libc::c_void,
                &mut savecount as *mut libc::c_short as *const libc::c_void,
                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
            );
            comp_ptr = comp_ptr
                .offset(
                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                );
            memcpy(
                comp_ptr as *mut libc::c_void,
                save.as_mut_ptr() as *const libc::c_void,
                savecount as libc::c_ulong,
            );
            comp_ptr = comp_ptr.offset(savecount as libc::c_int as isize);
        }
        if *source_ptr as libc::c_int != ',' as i32 {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_hang() {
    loop {
        let mut iflag: libc::c_int = (*source_ptr as libc::c_int == '@' as i32)
            as libc::c_int;
        eval();
        if iflag != 0
            && *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                != 65 as libc::c_int
        {
            iflag = 0 as libc::c_int;
        }
        if iflag != 0 {
            *comp_ptr
                .offset(-(1 as libc::c_int as isize)) = 184 as libc::c_int as u_char;
        } else {
            let fresh12 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh12 = 29 as libc::c_int as u_char;
        }
        if !(*source_ptr as libc::c_int == ',' as i32) {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_if(mut i: libc::c_long) {
    loop {
        let mut iflag: libc::c_int = (*source_ptr as libc::c_int == '@' as i32)
            as libc::c_int;
        eval();
        if iflag != 0
            && *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                != 65 as libc::c_int
        {
            iflag = 0 as libc::c_int;
        }
        if iflag != 0 {
            *comp_ptr
                .offset(-(1 as libc::c_int as isize)) = 185 as libc::c_int as u_char;
        } else if i == -(1 as libc::c_int) as libc::c_long {
            let fresh13 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh13 = 7 as libc::c_int as u_char;
        } else {
            let fresh14 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh14 = 8 as libc::c_int as u_char;
            memcpy(
                comp_ptr as *mut libc::c_void,
                &mut i as *mut libc::c_long as *const libc::c_void,
                ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
            );
            comp_ptr = comp_ptr
                .offset(
                    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as isize,
                );
        }
        if *source_ptr as libc::c_int != ',' as i32 {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_job(mut runtime: libc::c_int) {
    let mut args: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut save: [u_char; 1024] = [0; 1024];
    let mut savecount: libc::c_int = 0;
    loop {
        let mut i: libc::c_int = 0;
        ptr = comp_ptr;
        let fresh15 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh15 = 146 as libc::c_int as u_char;
        i = routine(runtime) as libc::c_int;
        if i == 0 as libc::c_int {
            *ptr = 179 as libc::c_int as u_char;
            let fresh16 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh16 = 186 as libc::c_int as u_char;
            if *source_ptr as libc::c_int == '(' as i32 {
                comp_ptr = ptr;
                comperror(-(70 as libc::c_int + 200 as libc::c_int) as libc::c_short);
            }
        } else {
            args = 0 as libc::c_int;
            if i == -(2 as libc::c_int) {
                *ptr = 148 as libc::c_int as u_char;
            } else if i == -(3 as libc::c_int) {
                *ptr = 147 as libc::c_int as u_char;
            } else if i == -(4 as libc::c_int) {
                *ptr = 149 as libc::c_int as u_char;
            }
            if *source_ptr as libc::c_int == '(' as i32 {
                savecount = comp_ptr.offset_from(ptr) as libc::c_long as libc::c_int;
                memcpy(
                    save.as_mut_ptr() as *mut libc::c_void,
                    ptr as *const libc::c_void,
                    savecount as libc::c_ulong,
                );
                comp_ptr = ptr;
                source_ptr = source_ptr.offset(1);
                source_ptr;
                loop {
                    if args > 127 as libc::c_int - 1 as libc::c_int {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    args += 1;
                    args;
                    if *source_ptr as libc::c_int == ')' as i32 {
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                        break;
                    } else {
                        if *source_ptr as libc::c_int == '.' as i32 {
                            comperror(-(40 as libc::c_int) as libc::c_short);
                            return;
                        }
                        eval();
                        if *source_ptr as libc::c_int == ')' as i32 {
                            continue;
                        }
                        if *source_ptr as libc::c_int == ',' as i32 {
                            source_ptr = source_ptr.offset(1);
                            source_ptr;
                        } else {
                            comperror(
                                -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                            );
                            return;
                        }
                    }
                }
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    save.as_mut_ptr() as *const libc::c_void,
                    savecount as libc::c_ulong,
                );
                ptr = comp_ptr;
                comp_ptr = comp_ptr.offset(savecount as isize);
            }
        }
        if *source_ptr as libc::c_int == ':' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
            if *source_ptr as libc::c_int != ':' as i32 {
                let fresh17 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh17 = args as u_char;
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            source_ptr = source_ptr.offset(1);
            source_ptr;
            savecount = comp_ptr.offset_from(ptr) as libc::c_long as libc::c_int;
            memcpy(
                save.as_mut_ptr() as *mut libc::c_void,
                ptr as *const libc::c_void,
                savecount as libc::c_ulong,
            );
            comp_ptr = ptr;
            eval();
            memcpy(
                comp_ptr as *mut libc::c_void,
                save.as_mut_ptr() as *const libc::c_void,
                savecount as libc::c_ulong,
            );
            comp_ptr = comp_ptr.offset(savecount as isize);
            args |= 128 as libc::c_int;
        }
        let fresh18 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh18 = args as u_char;
        if *source_ptr as libc::c_int != ',' as i32 {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_kill() {
    let mut s: libc::c_short = 0;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    if *source_ptr as libc::c_int == '(' as i32 {
        let mut args: libc::c_int = 0 as libc::c_int;
        source_ptr = source_ptr.offset(1);
        source_ptr;
        loop {
            ptr = comp_ptr;
            s = localvar();
            if (s as libc::c_int) < 0 as libc::c_int {
                comperror(s);
                return;
            }
            let fresh19 = s;
            s = s + 1;
            *ptr.offset(fresh19 as isize) = 62 as libc::c_int as u_char;
            if *ptr.offset(s as isize) as libc::c_int != 0 as libc::c_int
                && *ptr.offset(s as isize) as libc::c_int != 64 as libc::c_int
            {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            args += 1;
            args;
            if *source_ptr as libc::c_int == ')' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                break;
            } else {
                if *source_ptr as libc::c_int != ',' as i32 {
                    break;
                }
                source_ptr = source_ptr.offset(1);
                source_ptr;
            }
        }
        let fresh20 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh20 = 167 as libc::c_int as u_char;
        let fresh21 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh21 = args as u_char;
    } else {
        loop {
            if *source_ptr as libc::c_int == '@' as i32 {
                atom();
                if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == 65 as libc::c_int
                {
                    *comp_ptr
                        .offset(
                            -(1 as libc::c_int as isize),
                        ) = 187 as libc::c_int as u_char;
                } else if *comp_ptr.offset(-(3 as libc::c_int as isize)) as libc::c_int
                    == 61 as libc::c_int
                {
                    *comp_ptr
                        .offset(
                            -(3 as libc::c_int as isize),
                        ) = 62 as libc::c_int as u_char;
                }
            } else {
                ptr = comp_ptr;
                s = localvar();
                if (s as libc::c_int) < 0 as libc::c_int {
                    comperror(s);
                    return;
                }
                ptr = &mut *ptr.offset(s as isize) as *mut u_char;
                *ptr = 62 as libc::c_int as u_char;
            }
            if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                != 187 as libc::c_int
            {
                let fresh22 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh22 = 166 as libc::c_int as u_char;
            }
            if *source_ptr as libc::c_int != ',' as i32 {
                break;
            }
            source_ptr = source_ptr.offset(1);
            source_ptr;
        }
    }
    if *source_ptr as libc::c_int == ',' as i32 {
        source_ptr = source_ptr.offset(1);
        source_ptr;
        parse_kill();
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_lock() {
    let mut s: libc::c_short = 0;
    let mut us: u_short = 0;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    loop {
        let fresh23 = source_ptr;
        source_ptr = source_ptr.offset(1);
        let mut c: libc::c_char = *fresh23 as libc::c_char;
        let mut type_0: libc::c_int = 0 as libc::c_int;
        let mut args: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        if c as libc::c_int == '+' as i32 {
            type_0 = 1 as libc::c_int;
        }
        if c as libc::c_int == '-' as i32 {
            type_0 = -(1 as libc::c_int);
        }
        if type_0 != 0 {
            let fresh24 = source_ptr;
            source_ptr = source_ptr.offset(1);
            c = *fresh24 as libc::c_char;
        }
        if c as libc::c_int == '(' as i32 {
            i += 1;
            i;
        } else {
            source_ptr = source_ptr.offset(-1);
            source_ptr;
        }
        loop {
            ptr = comp_ptr;
            if *source_ptr as libc::c_int == '@' as i32 {
                atom();
                if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == 65 as libc::c_int
                {
                    if type_0 == 0 {
                        *comp_ptr
                            .offset(
                                -(1 as libc::c_int as isize),
                            ) = 188 as libc::c_int as u_char;
                        return;
                    }
                    *comp_ptr
                        .offset(
                            -(1 as libc::c_int as isize),
                        ) = 66 as libc::c_int as u_char;
                } else if *comp_ptr.offset(-(3 as libc::c_int as isize)) as libc::c_int
                    == 61 as libc::c_int
                {
                    *comp_ptr
                        .offset(
                            -(3 as libc::c_int as isize),
                        ) = 62 as libc::c_int as u_char;
                }
            } else {
                s = localvar();
                if (s as libc::c_int) < 0 as libc::c_int {
                    comperror(s);
                    return;
                }
                *ptr.offset(s as isize) = 62 as libc::c_int as u_char;
            }
            args += 1;
            args;
            if i == 0 {
                break;
            }
            if *source_ptr as libc::c_int == ')' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                break;
            } else {
                if *source_ptr as libc::c_int != ',' as i32 {
                    break;
                }
                source_ptr = source_ptr.offset(1);
                source_ptr;
            }
        }
        if *source_ptr as libc::c_int == ':' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
            eval();
        } else {
            let fresh25 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh25 = 60 as libc::c_int as u_char;
            us = 2 as libc::c_int as u_short;
            memcpy(
                comp_ptr as *mut libc::c_void,
                &mut us as *mut u_short as *const libc::c_void,
                ::core::mem::size_of::<u_short>() as libc::c_ulong,
            );
            comp_ptr = comp_ptr
                .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
            let fresh26 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh26 = '-' as i32 as u_char;
            let fresh27 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh27 = '1' as i32 as u_char;
            let fresh28 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh28 = '\0' as i32 as u_char;
        }
        if type_0 == -(1 as libc::c_int) {
            let fresh29 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh29 = 163 as libc::c_int as u_char;
        }
        if type_0 == 0 as libc::c_int {
            let fresh30 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh30 = 161 as libc::c_int as u_char;
        }
        if type_0 == 1 as libc::c_int {
            let fresh31 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh31 = 162 as libc::c_int as u_char;
        }
        let fresh32 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh32 = args as u_char;
        if *source_ptr as libc::c_int != ',' as i32 {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_merge() {
    let mut s: libc::c_short = 0;
    loop {
        let mut ptr1: *mut u_char = 0 as *mut u_char;
        let mut ptr2: *mut u_char = 0 as *mut u_char;
        let mut ptr3: *mut u_char = 0 as *mut u_char;
        let mut i: libc::c_int = parse2eq(source_ptr);
        if *source_ptr.offset(i as isize) as libc::c_int == '=' as i32 {
            ptr1 = source_ptr;
            ptr2 = &mut *source_ptr.offset(i as isize) as *mut u_char;
            source_ptr = ptr2.offset(1 as libc::c_int as isize);
            if *source_ptr as libc::c_int == '@' as i32 {
                atom();
                ptr3 = comp_ptr.offset(-(1 as libc::c_int as isize));
                if *ptr3 as libc::c_int == 65 as libc::c_int {
                    *ptr3 = 68 as libc::c_int as u_char;
                } else {
                    ptr3 = ptr3.offset(-(2 as libc::c_int as isize));
                    if *ptr3 as libc::c_int == 61 as libc::c_int {
                        *ptr3 = 64 as libc::c_int as u_char;
                    }
                }
            } else {
                ptr3 = comp_ptr;
                s = localvar();
                if (s as libc::c_int) < 0 as libc::c_int {
                    comperror(s);
                    return;
                }
                ptr3 = &mut *ptr3.offset(s as isize) as *mut u_char;
                *ptr3 = 64 as libc::c_int as u_char;
            }
            let fresh33 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh33 = 44 as libc::c_int as u_char;
            ptr3 = source_ptr;
            source_ptr = ptr1;
            ptr1 = comp_ptr;
            if *source_ptr as libc::c_int == '@' as i32 {
                atom();
                if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == 65 as libc::c_int
                {
                    if *source_ptr as libc::c_int != '=' as i32 {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    *comp_ptr
                        .offset(
                            -(1 as libc::c_int as isize),
                        ) = 68 as libc::c_int as u_char;
                } else if *comp_ptr.offset(-(3 as libc::c_int as isize)) as libc::c_int
                    == 61 as libc::c_int
                {
                    *comp_ptr
                        .offset(
                            -(3 as libc::c_int as isize),
                        ) = 64 as libc::c_int as u_char;
                }
            } else {
                s = localvar();
                if (s as libc::c_int) < 0 as libc::c_int {
                    comperror(s);
                    return;
                }
                ptr1 = &mut *ptr1.offset(s as isize) as *mut u_char;
                *ptr1 = 64 as libc::c_int as u_char;
            }
            if source_ptr != ptr2 {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            source_ptr = ptr3;
            let fresh34 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh34 = 139 as libc::c_int as u_char;
            if *source_ptr as libc::c_int != ',' as i32 {
                break;
            }
            source_ptr = source_ptr.offset(1);
            source_ptr;
        } else {
            if *source_ptr as libc::c_int != '@' as i32 {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            atom();
            if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                == 65 as libc::c_int
            {
                *comp_ptr
                    .offset(-(1 as libc::c_int as isize)) = 189 as libc::c_int as u_char;
                if *source_ptr as libc::c_int != ',' as i32 {
                    break;
                }
                source_ptr = source_ptr.offset(1);
                source_ptr;
            } else {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_new() {
    let mut c: libc::c_char = 0;
    let mut s: libc::c_short = 0;
    let mut args: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let fresh35 = source_ptr;
    source_ptr = source_ptr.offset(1);
    c = *fresh35 as libc::c_char;
    if c as libc::c_int == '(' as i32 {
        args = 0 as libc::c_int;
        loop {
            ptr = comp_ptr;
            if *source_ptr as libc::c_int == '@' as i32 {
                if args != 0 {
                    let fresh36 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh36 = 164 as libc::c_int as u_char;
                    let fresh37 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh37 = args as u_char;
                    args = 0 as libc::c_int;
                }
                atom();
                if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == 65 as libc::c_int
                {
                    *comp_ptr
                        .offset(
                            -(1 as libc::c_int as isize),
                        ) = 190 as libc::c_int as u_char;
                } else {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
            } else {
                s = localvar();
                if (s as libc::c_int) < 0 as libc::c_int {
                    comperror(s);
                    return;
                }
                let fresh38 = s;
                s = s + 1;
                *ptr.offset(fresh38 as isize) = 62 as libc::c_int as u_char;
                if *ptr.offset(s as isize) as libc::c_int != 0 as libc::c_int
                    && *ptr.offset(s as isize) as libc::c_int != 64 as libc::c_int
                {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                args += 1;
                args;
            }
            if *source_ptr as libc::c_int == ')' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                break;
            } else {
                if *source_ptr as libc::c_int != ',' as i32 {
                    break;
                }
                source_ptr = source_ptr.offset(1);
                source_ptr;
            }
        }
        let fresh39 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh39 = 165 as libc::c_int as u_char;
        let fresh40 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh40 = args as u_char;
    } else {
        source_ptr = source_ptr.offset(-1);
        source_ptr;
        loop {
            ptr = comp_ptr;
            if *source_ptr as libc::c_int == '@' as i32 {
                if args != 0 {
                    let fresh41 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh41 = 164 as libc::c_int as u_char;
                    let fresh42 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh42 = args as u_char;
                    args = 0 as libc::c_int;
                }
                atom();
                if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == 65 as libc::c_int
                {
                    *comp_ptr
                        .offset(
                            -(1 as libc::c_int as isize),
                        ) = 190 as libc::c_int as u_char;
                } else {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
            } else {
                s = localvar();
                if (s as libc::c_int) < 0 as libc::c_int {
                    comperror(s);
                    return;
                }
                let fresh43 = s;
                s = s + 1;
                *ptr.offset(fresh43 as isize) = 62 as libc::c_int as u_char;
                if *ptr.offset(s as isize) as libc::c_int != 0 as libc::c_int
                    && *ptr.offset(s as isize) as libc::c_int != 64 as libc::c_int
                {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                args += 1;
                args;
            }
            if *source_ptr as libc::c_int != ',' as i32 {
                break;
            }
            source_ptr = source_ptr.offset(1);
            source_ptr;
            if *source_ptr as libc::c_int == '(' as i32 {
                break;
            }
        }
        if args != 0 {
            let fresh44 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh44 = 164 as libc::c_int as u_char;
            let fresh45 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh45 = args as u_char;
        }
    }
    if *source_ptr as libc::c_int == ',' as i32 {
        source_ptr = source_ptr.offset(1);
        source_ptr;
        parse_new();
    }
    if *source_ptr as libc::c_int == '(' as i32 {
        parse_new();
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_open() {
    let mut us: u_short = 0;
    loop {
        let mut iflag: libc::c_int = (*source_ptr as libc::c_int == '@' as i32)
            as libc::c_int;
        eval();
        if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
            == 65 as libc::c_int && iflag != 0
        {
            *comp_ptr
                .offset(-(1 as libc::c_int as isize)) = 191 as libc::c_int as u_char;
        } else {
            let fresh46 = source_ptr;
            source_ptr = source_ptr.offset(1);
            if *fresh46 as libc::c_int != ':' as i32 {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            if *source_ptr as libc::c_int == '(' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                eval();
                let fresh47 = source_ptr;
                source_ptr = source_ptr.offset(1);
                if *fresh47 as libc::c_int != ':' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                eval();
                let fresh48 = source_ptr;
                source_ptr = source_ptr.offset(1);
                if *fresh48 as libc::c_int != ')' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
            } else {
                let fresh49 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh49 = 60 as libc::c_int as u_char;
                let fresh50 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh50 = 0 as libc::c_int as u_char;
                let fresh51 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh51 = 0 as libc::c_int as u_char;
                let fresh52 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh52 = '\0' as i32 as u_char;
                let fresh53 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh53 = 60 as libc::c_int as u_char;
                let fresh54 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh54 = 0 as libc::c_int as u_char;
                let fresh55 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh55 = 0 as libc::c_int as u_char;
                let fresh56 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh56 = '\0' as i32 as u_char;
            }
            if *source_ptr as libc::c_int == ':' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                if *source_ptr as libc::c_int == ':' as i32 {
                    let fresh57 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh57 = 60 as libc::c_int as u_char;
                    us = 2 as libc::c_int as u_short;
                    memcpy(
                        comp_ptr as *mut libc::c_void,
                        &mut us as *mut u_short as *const libc::c_void,
                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                    );
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                        );
                    let fresh58 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh58 = '-' as i32 as u_char;
                    let fresh59 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh59 = '1' as i32 as u_char;
                    let fresh60 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh60 = '\0' as i32 as u_char;
                } else {
                    eval();
                }
            } else {
                let fresh61 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh61 = 60 as libc::c_int as u_char;
                us = 2 as libc::c_int as u_short;
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    &mut us as *mut u_short as *const libc::c_void,
                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                );
                comp_ptr = comp_ptr
                    .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
                let fresh62 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh62 = '-' as i32 as u_char;
                let fresh63 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh63 = '1' as i32 as u_char;
                let fresh64 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh64 = '\0' as i32 as u_char;
            }
            if *source_ptr as libc::c_int == ':' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                let fresh65 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh65 = 60 as libc::c_int as u_char;
                us = 10 as libc::c_int as u_short;
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    &mut us as *mut u_short as *const libc::c_void,
                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                );
                comp_ptr = comp_ptr
                    .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    b"namespace=\0\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    11 as libc::c_int as libc::c_ulong,
                );
                comp_ptr = comp_ptr.offset(11 as libc::c_int as isize);
                eval();
                let fresh66 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh66 = 17 as libc::c_int as u_char;
            } else {
                let fresh67 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh67 = 60 as libc::c_int as u_char;
                let fresh68 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh68 = 0 as libc::c_int as u_char;
                let fresh69 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh69 = 0 as libc::c_int as u_char;
                let fresh70 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh70 = '\0' as i32 as u_char;
            }
            let fresh71 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh71 = 58 as libc::c_int as u_char;
        }
        if *source_ptr as libc::c_int != ',' as i32 {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn write_fmt() {
    let mut i: libc::c_int = 0;
    let mut args: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    source_ptr = source_ptr.offset(1);
    source_ptr;
    ptr = comp_ptr;
    let fresh72 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh72 = 140 as libc::c_int as u_char;
    i = routine(0 as libc::c_int) as libc::c_int;
    if i != -(1 as libc::c_int) {
        comp_ptr = ptr;
        comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
        return;
    }
    if *source_ptr as libc::c_int == '(' as i32 {
        let mut savecount: libc::c_int = comp_ptr.offset_from(ptr) as libc::c_long
            as libc::c_int;
        let mut save: [u_char; 1024] = [0; 1024];
        memcpy(
            save.as_mut_ptr() as *mut libc::c_void,
            ptr as *const libc::c_void,
            savecount as libc::c_ulong,
        );
        comp_ptr = ptr;
        source_ptr = source_ptr.offset(1);
        source_ptr;
        loop {
            if args > 127 as libc::c_int - 1 as libc::c_int {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            args += 1;
            args;
            if *source_ptr as libc::c_int == ')' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                break;
            } else {
                if *source_ptr as libc::c_int == ',' as i32
                    || *source_ptr as libc::c_int == ')' as i32
                {
                    let fresh73 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh73 = 169 as libc::c_int as u_char;
                } else if *source_ptr as libc::c_int == '.' as i32
                    && isdigit(
                        *source_ptr.offset(1 as libc::c_int as isize) as libc::c_int,
                    ) == 0 as libc::c_int
                {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                } else {
                    eval();
                }
                if *source_ptr as libc::c_int == ')' as i32 {
                    continue;
                }
                if *source_ptr as libc::c_int == ',' as i32 {
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                } else {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
            }
        }
        memcpy(
            comp_ptr as *mut libc::c_void,
            save.as_mut_ptr() as *const libc::c_void,
            savecount as libc::c_ulong,
        );
        comp_ptr = comp_ptr.offset(savecount as isize);
    }
    let fresh74 = comp_ptr;
    comp_ptr = comp_ptr.offset(1);
    *fresh74 = args as u_char;
}
#[no_mangle]
pub unsafe extern "C" fn parse_read_var(mut star: libc::c_int) -> libc::c_short {
    let mut c: libc::c_char = 0;
    let mut s: libc::c_short = 0;
    let mut type_0: libc::c_int = 0;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    c = *source_ptr as libc::c_char;
    if c as libc::c_int == '@' as i32 {
        source_ptr = source_ptr.offset(1);
        source_ptr;
        atom();
        if *source_ptr as libc::c_int == '@' as i32 || star != 0 {
            let fresh75 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh75 = 66 as libc::c_int as u_char;
            ptr = comp_ptr;
            if *source_ptr as libc::c_int == '@' as i32 {
                s = localvar();
                if (s as libc::c_int) < 0 as libc::c_int {
                    comperror(s);
                    return s;
                }
                *ptr.offset(s as isize) = 62 as libc::c_int as u_char;
            }
            type_0 = if star != 0 { 46 as libc::c_int } else { 48 as libc::c_int };
            if star == 0 && *source_ptr as libc::c_int == '#' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                eval();
                type_0 = 50 as libc::c_int;
            }
            if *source_ptr as libc::c_int == ':' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                eval();
                if star != 0 {
                    type_0 = 47 as libc::c_int;
                } else {
                    type_0 = if type_0 == 48 as libc::c_int {
                        49 as libc::c_int
                    } else {
                        51 as libc::c_int
                    };
                }
            }
            let fresh76 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh76 = type_0 as u_char;
        } else {
            let fresh77 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh77 = 192 as libc::c_int as u_char;
        }
    } else {
        ptr = comp_ptr;
        s = localvar();
        if (s as libc::c_int) < 0 as libc::c_int {
            comperror(s);
            return s;
        }
        *ptr.offset(s as isize) = 62 as libc::c_int as u_char;
        type_0 = if star != 0 { 46 as libc::c_int } else { 48 as libc::c_int };
        if star == 0 && *source_ptr as libc::c_int == '#' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
            eval();
            type_0 = 50 as libc::c_int;
        }
        if *source_ptr as libc::c_int == ':' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
            eval();
            if star != 0 {
                type_0 = 47 as libc::c_int;
            } else {
                type_0 = if type_0 == 48 as libc::c_int {
                    49 as libc::c_int
                } else {
                    51 as libc::c_int
                };
            }
        }
        let fresh78 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh78 = type_0 as u_char;
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn parse_read() {
    let mut args: libc::c_int = 0 as libc::c_int;
    loop {
        let mut c: libc::c_char = *source_ptr as libc::c_char;
        if c as libc::c_int == '!' as i32 {
            let fresh79 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh79 = 53 as libc::c_int as u_char;
            source_ptr = source_ptr.offset(1);
            source_ptr;
            args += 1;
            args;
        } else if c as libc::c_int == '#' as i32 {
            let fresh80 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh80 = 54 as libc::c_int as u_char;
            source_ptr = source_ptr.offset(1);
            source_ptr;
            args += 1;
            args;
        } else if c as libc::c_int == '?' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
            eval();
            let fresh81 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh81 = 55 as libc::c_int as u_char;
            args += 1;
            args;
        } else if c as libc::c_int == '/' as i32 {
            write_fmt();
        } else if c as libc::c_int == '"' as i32 {
            atom();
            let fresh82 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh82 = 56 as libc::c_int as u_char;
            args += 1;
            args;
        } else if c as libc::c_int == '*' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
            if args != 0 {
                let fresh83 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh83 = 45 as libc::c_int as u_char;
                args = 0 as libc::c_int;
            }
            if parse_read_var(1 as libc::c_int) != 0 {
                return;
            }
        } else {
            if args != 0 {
                let fresh84 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh84 = 45 as libc::c_int as u_char;
                args = 0 as libc::c_int;
            }
            if parse_read_var(0 as libc::c_int) != 0 {
                return;
            }
        }
        if *source_ptr as libc::c_int == ' ' as i32
            || *source_ptr as libc::c_int == '\0' as i32
        {
            break;
        }
        if *source_ptr as libc::c_int == ',' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_set() {
    let mut s: libc::c_short = 0;
    let mut us: u_short = 0;
    let mut bracket: libc::c_int = 0;
    let mut args: libc::c_int = 0 as libc::c_int;
    let mut p: *mut u_char = 0 as *mut u_char;
    loop {
        let mut ptr1: *mut u_char = 0 as *mut u_char;
        let mut ptr2: *mut u_char = 0 as *mut u_char;
        let mut ptr3: *mut u_char = 0 as *mut u_char;
        let mut i: libc::c_int = parse2eq(source_ptr);
        if *source_ptr.offset(i as isize) as libc::c_int == '=' as i32 {
            ptr1 = source_ptr;
            ptr2 = &mut *source_ptr.offset(i as isize) as *mut u_char;
            source_ptr = ptr2.offset(1 as libc::c_int as isize);
            eval();
            ptr3 = source_ptr;
            source_ptr = ptr1;
            bracket = 0 as libc::c_int;
            if *source_ptr as libc::c_int == '(' as i32 {
                bracket = 1 as libc::c_int;
                source_ptr = source_ptr.offset(1);
                source_ptr;
            }
            loop {
                if strncasecmp(
                    source_ptr as *mut libc::c_char,
                    b"$e(\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                    || strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"$extract(\0" as *const u8 as *const libc::c_char,
                        9 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    || strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"$p(\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    || strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"$piece(\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    args = (toupper(
                        *source_ptr.offset(1 as libc::c_int as isize) as libc::c_int,
                    ) == 'P' as i32) as libc::c_int;
                    while *source_ptr as libc::c_int != '(' as i32
                        && *source_ptr as libc::c_int != 0
                    {
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                    }
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                    p = comp_ptr;
                    if *source_ptr as libc::c_int == '@' as i32 {
                        atom();
                        if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                            == 65 as libc::c_int
                        {
                            *comp_ptr
                                .offset(
                                    -(1 as libc::c_int as isize),
                                ) = 66 as libc::c_int as u_char;
                        } else if *comp_ptr.offset(-(3 as libc::c_int as isize))
                            as libc::c_int == 61 as libc::c_int
                        {
                            *comp_ptr
                                .offset(
                                    -(3 as libc::c_int as isize),
                                ) = 62 as libc::c_int as u_char;
                        }
                    } else {
                        s = localvar();
                        if (s as libc::c_int) < 0 as libc::c_int {
                            comperror(s);
                            return;
                        }
                        p = &mut *p.offset(s as isize) as *mut u_char;
                        *p = 62 as libc::c_int as u_char;
                    }
                    if args != 0 {
                        if *source_ptr as libc::c_int != ',' as i32 {
                            comperror(
                                -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                            );
                            return;
                        }
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                        eval();
                    }
                    if *source_ptr as libc::c_int == ')' as i32 {
                        let fresh85 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh85 = 60 as libc::c_int as u_char;
                        us = 1 as libc::c_int as u_short;
                        memcpy(
                            comp_ptr as *mut libc::c_void,
                            &mut us as *mut u_short as *const libc::c_void,
                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                        );
                        comp_ptr = comp_ptr
                            .offset(
                                ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                            );
                        let fresh86 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh86 = '1' as i32 as u_char;
                        let fresh87 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh87 = '\0' as i32 as u_char;
                    } else {
                        if *source_ptr as libc::c_int != ',' as i32 {
                            comperror(
                                -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                            );
                            return;
                        }
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                        eval();
                    }
                    if *source_ptr as libc::c_int == ')' as i32 {
                        let fresh88 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh88 = 72 as libc::c_int as u_char;
                    } else {
                        if *source_ptr as libc::c_int != ',' as i32 {
                            comperror(
                                -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                            );
                            return;
                        }
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                        eval();
                    }
                    let fresh89 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    if *fresh89 as libc::c_int != ')' as i32 {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    let fresh90 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh90 = (if args != 0 {
                        43 as libc::c_int
                    } else {
                        42 as libc::c_int
                    }) as u_char;
                } else if *source_ptr as libc::c_int == '@' as i32 {
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                    atom();
                    if *source_ptr as libc::c_int == '@' as i32 {
                        let fresh91 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh91 = 66 as libc::c_int as u_char;
                        p = comp_ptr;
                        s = localvar();
                        if (s as libc::c_int) < 0 as libc::c_int {
                            comperror(s);
                            return;
                        }
                        *p.offset(s as isize) = 62 as libc::c_int as u_char;
                        let fresh92 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh92 = 41 as libc::c_int as u_char;
                    } else {
                        let fresh93 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh93 = 66 as libc::c_int as u_char;
                        let fresh94 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh94 = 41 as libc::c_int as u_char;
                    }
                } else {
                    p = comp_ptr;
                    s = localvar();
                    if (s as libc::c_int) < 0 as libc::c_int {
                        comperror(s);
                        return;
                    }
                    *p.offset(s as isize) = 62 as libc::c_int as u_char;
                    let fresh95 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh95 = 41 as libc::c_int as u_char;
                }
                if bracket == 0 {
                    break;
                }
                if *source_ptr as libc::c_int == ')' as i32 {
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                    break;
                } else {
                    let fresh96 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    if *fresh96 as libc::c_int != ',' as i32 {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                }
            }
            if source_ptr != ptr2 {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            source_ptr = ptr3;
            if *source_ptr as libc::c_int != ',' as i32 {
                break;
            }
            source_ptr = source_ptr.offset(1);
            source_ptr;
        } else {
            if *source_ptr as libc::c_int != '@' as i32 {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
            atom();
            if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                == 65 as libc::c_int
            {
                *comp_ptr
                    .offset(-(1 as libc::c_int as isize)) = 193 as libc::c_int as u_char;
                if *source_ptr as libc::c_int != ',' as i32 {
                    break;
                }
                source_ptr = source_ptr.offset(1);
                source_ptr;
            } else {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_use() {
    let mut c: libc::c_char = 0;
    let mut us: u_short = 0;
    let mut i: libc::c_int = 0;
    let mut args: libc::c_int = 0;
    loop {
        let mut iflag: libc::c_int = (*source_ptr as libc::c_int == '@' as i32)
            as libc::c_int;
        eval();
        if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
            == 65 as libc::c_int && iflag != 0
        {
            *comp_ptr
                .offset(-(1 as libc::c_int as isize)) = 194 as libc::c_int as u_char;
        } else {
            args = 0 as libc::c_int;
            if *source_ptr as libc::c_int == ':' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                if *source_ptr as libc::c_int != ':' as i32 {
                    i = (*source_ptr as libc::c_int == '(' as i32) as libc::c_int;
                    if i != 0 {
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                    }
                    eval();
                    args += 1;
                    args;
                    while i != 0 {
                        let fresh97 = source_ptr;
                        source_ptr = source_ptr.offset(1);
                        c = *fresh97 as libc::c_char;
                        if c as libc::c_int == ')' as i32 {
                            break;
                        }
                        if c as libc::c_int != ':' as i32 {
                            comperror(
                                -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                            );
                            return;
                        }
                        eval();
                        args += 1;
                        args;
                    }
                }
            }
            if *source_ptr as libc::c_int == ':' as i32 {
                source_ptr = source_ptr.offset(1);
                source_ptr;
                args += 1;
                args;
                let fresh98 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh98 = 60 as libc::c_int as u_char;
                us = 10 as libc::c_int as u_short;
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    &mut us as *mut u_short as *const libc::c_void,
                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                );
                comp_ptr = comp_ptr
                    .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    b"namespace=\0\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    11 as libc::c_int as libc::c_ulong,
                );
                comp_ptr = comp_ptr.offset(11 as libc::c_int as isize);
                eval();
                let fresh99 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh99 = 17 as libc::c_int as u_char;
            }
            let fresh100 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh100 = 57 as libc::c_int as u_char;
            let fresh101 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh101 = args as u_char;
        }
        if *source_ptr as libc::c_int != ',' as i32 {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_write() {
    let mut iflag: libc::c_int = 0;
    loop {
        let mut c: libc::c_char = *source_ptr as libc::c_char;
        if c as libc::c_int == '!' as i32 {
            let fresh102 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh102 = 53 as libc::c_int as u_char;
            source_ptr = source_ptr.offset(1);
            source_ptr;
        } else if c as libc::c_int == '#' as i32 {
            let fresh103 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh103 = 54 as libc::c_int as u_char;
            source_ptr = source_ptr.offset(1);
            source_ptr;
        } else if c as libc::c_int == '?' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
            eval();
            let fresh104 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh104 = 55 as libc::c_int as u_char;
        } else if c as libc::c_int == '*' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
            eval();
            let fresh105 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh105 = 52 as libc::c_int as u_char;
        } else if c as libc::c_int == '/' as i32 {
            write_fmt();
        } else {
            iflag = (*source_ptr as libc::c_int == '@' as i32) as libc::c_int;
            eval();
            if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                == 65 as libc::c_int && iflag != 0
            {
                *comp_ptr
                    .offset(-(1 as libc::c_int as isize)) = 195 as libc::c_int as u_char;
            } else {
                let fresh106 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh106 = 56 as libc::c_int as u_char;
            }
        }
        c = *source_ptr as libc::c_char;
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\0' as i32 {
            break;
        }
        if c as libc::c_int == ',' as i32 {
            source_ptr = source_ptr.offset(1);
            source_ptr;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_xecute() {
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut save: [u_char; 1024] = [0; 1024];
    let mut savecount: libc::c_short = 0;
    loop {
        let mut iflag: libc::c_int = (*source_ptr as libc::c_int == '@' as i32)
            as libc::c_int;
        ptr = comp_ptr;
        eval();
        if *comp_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
            == 65 as libc::c_int && iflag != 0
        {
            *comp_ptr
                .offset(-(1 as libc::c_int as isize)) = 196 as libc::c_int as u_char;
        } else {
            let fresh107 = comp_ptr;
            comp_ptr = comp_ptr.offset(1);
            *fresh107 = 154 as libc::c_int as u_char;
            if *source_ptr as libc::c_int == ':' as i32 {
                savecount = comp_ptr.offset_from(ptr) as libc::c_long as libc::c_short;
                mcopy(ptr, save.as_mut_ptr(), savecount as libc::c_int);
                comp_ptr = ptr;
                source_ptr = source_ptr.offset(1);
                source_ptr;
                eval();
                let fresh108 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh108 = 5 as libc::c_int as u_char;
                memcpy(
                    comp_ptr as *mut libc::c_void,
                    &mut savecount as *mut libc::c_short as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                );
                comp_ptr = comp_ptr
                    .offset(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                    );
                mcopy(save.as_mut_ptr(), comp_ptr, savecount as libc::c_int);
                comp_ptr = comp_ptr.offset(savecount as libc::c_int as isize);
            }
        }
        if *source_ptr as libc::c_int != ',' as i32 {
            break;
        }
        source_ptr = source_ptr.offset(1);
        source_ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse() {
    let mut s: libc::c_short = 0;
    let mut i: libc::c_int = 0;
    let mut args: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut jmp_eoc: *mut u_char = 0 as *mut u_char;
    loop {
        let fresh109 = source_ptr;
        source_ptr = source_ptr.offset(1);
        let mut c: libc::c_char = toupper(*fresh109 as libc::c_int) as libc::c_char;
        jmp_eoc = 0 as *mut u_char;
        let mut current_block_606: u64;
        match c as libc::c_int {
            0 => {
                current_block_606 = 12723717990039353365;
            }
            59 => {
                current_block_606 = 12723717990039353365;
            }
            32 => {
                current_block_606 = 10653053999995228228;
            }
            66 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"reak\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(4 as libc::c_int as isize);
                }
                let fresh111 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh111 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh112 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh112 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh113 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh113 as libc::c_char;
                }
                if c as libc::c_int != '\0' as i32 {
                    if c as libc::c_int != ' ' as i32 {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    if *source_ptr as libc::c_int == ' ' as i32 {
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                        let fresh114 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh114 = 70 as libc::c_int as u_char;
                    } else {
                        loop {
                            eval();
                            let fresh115 = comp_ptr;
                            comp_ptr = comp_ptr.offset(1);
                            *fresh115 = 71 as libc::c_int as u_char;
                            if *source_ptr as libc::c_int != ',' as i32 {
                                break;
                            }
                            source_ptr = source_ptr.offset(1);
                            source_ptr;
                        }
                    }
                } else {
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    let fresh116 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh116 = 70 as libc::c_int as u_char;
                }
                current_block_606 = 10653053999995228228;
            }
            67 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"lose\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(4 as libc::c_int as isize);
                }
                let fresh117 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh117 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh118 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh118 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh119 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh119 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_close();
                current_block_606 = 10653053999995228228;
            }
            68 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"o\0" as *const u8 as *const libc::c_char,
                        1 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(1 as libc::c_int as isize);
                }
                let fresh120 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh120 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh121 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh121 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh122 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh122 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 && c as libc::c_int != '\0' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if c as libc::c_int == '\0' as i32 {
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                }
                if *source_ptr as libc::c_int == ' ' as i32
                    || *source_ptr as libc::c_int == '\0' as i32
                {
                    let fresh123 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh123 = 145 as libc::c_int as u_char;
                } else {
                    parse_do(0 as libc::c_int);
                }
                current_block_606 = 10653053999995228228;
            }
            69 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"lse\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(3 as libc::c_int as isize);
                }
                let fresh124 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh124 as libc::c_char;
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                let fresh125 = source_ptr;
                source_ptr = source_ptr.offset(1);
                if *fresh125 as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                let fresh126 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh126 = 9 as libc::c_int as u_char;
                current_block_606 = 10653053999995228228;
            }
            70 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"or\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(2 as libc::c_int as isize);
                }
                let fresh127 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh127 as libc::c_char;
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                let fresh128 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh128 as libc::c_char;
                if c as libc::c_int == ' ' as i32 {
                    let fresh129 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh129 = 173 as libc::c_int as u_char;
                    ptr = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    parse();
                    comp_ptr = comp_ptr.offset(-1);
                    comp_ptr;
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    let fresh130 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh130 = 4 as libc::c_int as u_char;
                    let fresh131 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh131 = 172 as libc::c_int as u_char;
                    s = ptr.offset_from(comp_ptr) as libc::c_long as libc::c_short;
                    memcpy(
                        comp_ptr as *mut libc::c_void,
                        &mut s as *mut libc::c_short as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    );
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh132 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh132 = 179 as libc::c_int as u_char;
                    *(ptr
                        as *mut libc::c_short) = (comp_ptr.offset_from(ptr)
                        as libc::c_long as libc::c_ulong)
                        .wrapping_sub(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as libc::c_short;
                } else {
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    ptr = comp_ptr;
                    s = localvar();
                    if (s as libc::c_int) < 0 as libc::c_int {
                        comperror(s);
                        return;
                    }
                    ptr = ptr.offset(s as libc::c_int as isize);
                    let fresh133 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    if *fresh133 as libc::c_int != '=' as i32 {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    *ptr = 177 as libc::c_int as u_char;
                    ptr = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
                        );
                    loop {
                        eval();
                        let fresh134 = source_ptr;
                        source_ptr = source_ptr.offset(1);
                        c = *fresh134 as libc::c_char;
                        if c as libc::c_int != ':' as i32 {
                            let fresh135 = comp_ptr;
                            comp_ptr = comp_ptr.offset(1);
                            *fresh135 = 174 as libc::c_int as u_char;
                            if c as libc::c_int != ',' as i32 {
                                break;
                            }
                        } else {
                            eval();
                            let fresh136 = source_ptr;
                            source_ptr = source_ptr.offset(1);
                            c = *fresh136 as libc::c_char;
                            if c as libc::c_int != ':' as i32 {
                                let fresh137 = comp_ptr;
                                comp_ptr = comp_ptr.offset(1);
                                *fresh137 = 175 as libc::c_int as u_char;
                                if c as libc::c_int != ',' as i32 {
                                    break;
                                }
                            } else {
                                eval();
                                let fresh138 = source_ptr;
                                source_ptr = source_ptr.offset(1);
                                c = *fresh138 as libc::c_char;
                                let fresh139 = comp_ptr;
                                comp_ptr = comp_ptr.offset(1);
                                *fresh139 = 176 as libc::c_int as u_char;
                                if c as libc::c_int != ',' as i32 {
                                    break;
                                }
                            }
                        }
                    }
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    s = (comp_ptr.offset_from(ptr) as libc::c_long as libc::c_ulong)
                        .wrapping_sub(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                        ) as libc::c_short;
                    memcpy(
                        ptr as *mut libc::c_void,
                        &mut s as *mut libc::c_short as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    );
                    ptr = ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    parse();
                    comp_ptr = comp_ptr.offset(-1);
                    comp_ptr;
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    let fresh140 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh140 = 178 as libc::c_int as u_char;
                    let fresh141 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh141 = 179 as libc::c_int as u_char;
                    *(ptr
                        as *mut libc::c_short) = (comp_ptr.offset_from(ptr)
                        as libc::c_long as libc::c_ulong)
                        .wrapping_sub(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as libc::c_short;
                }
                current_block_606 = 10653053999995228228;
            }
            71 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"oto\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(3 as libc::c_int as isize);
                }
                let fresh142 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh142 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh143 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh143 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh144 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh144 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_goto(0 as libc::c_int);
                current_block_606 = 10653053999995228228;
            }
            72 => {
                i = 0 as libc::c_int;
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"alt\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i = 1 as libc::c_int;
                    } else if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"ang\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i = 2 as libc::c_int;
                    } else {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(3 as libc::c_int as isize);
                }
                c = ' ' as i32 as libc::c_char;
                if *source_ptr as libc::c_int != '\0' as i32 {
                    let fresh145 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh145 as libc::c_char;
                }
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh146 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh146 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    c = ' ' as i32 as libc::c_char;
                    if *source_ptr as libc::c_int != '\0' as i32 {
                        let fresh147 = source_ptr;
                        source_ptr = source_ptr.offset(1);
                        c = *fresh147 as libc::c_char;
                    }
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                c = ' ' as i32 as libc::c_char;
                if *source_ptr as libc::c_int != '\0' as i32 {
                    let fresh148 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh148 as libc::c_char;
                }
                if c as libc::c_int == ' ' as i32 {
                    if i == 2 as libc::c_int {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    let fresh149 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh149 = 1 as libc::c_int as u_char;
                } else {
                    if i == 1 as libc::c_int {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    parse_hang();
                }
                current_block_606 = 10653053999995228228;
            }
            73 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"f\0" as *const u8 as *const libc::c_char,
                        1 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(1 as libc::c_int as isize);
                }
                let fresh150 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh150 as libc::c_char;
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if *source_ptr as libc::c_int == ' ' as i32 {
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                    let fresh151 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh151 = 6 as libc::c_int as u_char;
                } else {
                    parse_if(-(1 as libc::c_int) as libc::c_long);
                }
                current_block_606 = 10653053999995228228;
            }
            74 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"ob\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(2 as libc::c_int as isize);
                }
                let fresh152 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh152 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh153 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh153 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh154 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh154 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_job(0 as libc::c_int);
                current_block_606 = 10653053999995228228;
            }
            75 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"ill\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(3 as libc::c_int as isize);
                }
                c = ' ' as i32 as libc::c_char;
                if *source_ptr as libc::c_int != '\0' as i32 {
                    let fresh155 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh155 as libc::c_char;
                }
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh156 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh156 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    c = ' ' as i32 as libc::c_char;
                    if *source_ptr as libc::c_int != '\0' as i32 {
                        let fresh157 = source_ptr;
                        source_ptr = source_ptr.offset(1);
                        c = *fresh157 as libc::c_char;
                    }
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if *source_ptr as libc::c_int != '\0' as i32 {
                    let fresh158 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh158 as libc::c_char;
                }
                if c as libc::c_int == ' ' as i32 {
                    let fresh159 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh159 = 167 as libc::c_int as u_char;
                    let fresh160 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh160 = 0 as libc::c_int as u_char;
                } else {
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    parse_kill();
                }
                current_block_606 = 10653053999995228228;
            }
            76 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"ock\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(3 as libc::c_int as isize);
                }
                let fresh161 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh161 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh162 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh162 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh163 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh163 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 && c as libc::c_int != '\0' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if c as libc::c_int == '\0' as i32 {
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    c = ' ' as i32 as libc::c_char;
                } else {
                    let fresh164 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh164 as libc::c_char;
                }
                if c as libc::c_int == ' ' as i32 {
                    let fresh165 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh165 = 160 as libc::c_int as u_char;
                } else {
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    parse_lock();
                }
                current_block_606 = 10653053999995228228;
            }
            77 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"erge\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(4 as libc::c_int as isize);
                }
                let fresh166 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh166 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh167 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh167 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh168 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh168 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_merge();
                current_block_606 = 10653053999995228228;
            }
            78 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"ew\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(2 as libc::c_int as isize);
                }
                args = 0 as libc::c_int;
                c = ' ' as i32 as libc::c_char;
                if *source_ptr as libc::c_int != '\0' as i32 {
                    let fresh169 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh169 as libc::c_char;
                }
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh170 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh170 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    c = ' ' as i32 as libc::c_char;
                    if *source_ptr as libc::c_int != '\0' as i32 {
                        let fresh171 = source_ptr;
                        source_ptr = source_ptr.offset(1);
                        c = *fresh171 as libc::c_char;
                    }
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if *source_ptr as libc::c_int != '\0' as i32 {
                    let fresh172 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh172 as libc::c_char;
                }
                if c as libc::c_int == ' ' as i32 {
                    let fresh173 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh173 = 165 as libc::c_int as u_char;
                    let fresh174 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh174 = 0 as libc::c_int as u_char;
                } else {
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    parse_new();
                }
                current_block_606 = 10653053999995228228;
            }
            79 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"pen\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(3 as libc::c_int as isize);
                }
                let fresh175 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh175 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh176 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh176 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh177 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh177 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_open();
                current_block_606 = 10653053999995228228;
            }
            81 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"uit\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(3 as libc::c_int as isize);
                }
                c = ' ' as i32 as libc::c_char;
                if *source_ptr as libc::c_int != '\0' as i32 {
                    let fresh178 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh178 as libc::c_char;
                }
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh179 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh179 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    c = ' ' as i32 as libc::c_char;
                    if *source_ptr as libc::c_int != '\0' as i32 {
                        let fresh180 = source_ptr;
                        source_ptr = source_ptr.offset(1);
                        c = *fresh180 as libc::c_char;
                    }
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                if *source_ptr as libc::c_int != '\0' as i32 {
                    let fresh181 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh181 as libc::c_char;
                }
                if c as libc::c_int == ' ' as i32 {
                    let fresh182 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh182 = 157 as libc::c_int as u_char;
                } else {
                    source_ptr = source_ptr.offset(-1);
                    source_ptr;
                    eval();
                    let fresh183 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh183 = 158 as libc::c_int as u_char;
                }
                current_block_606 = 10653053999995228228;
            }
            82 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"ead\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(3 as libc::c_int as isize);
                }
                let fresh184 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh184 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh185 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh185 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh186 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh186 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_read();
                current_block_606 = 10653053999995228228;
            }
            83 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"et\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(2 as libc::c_int as isize);
                }
                let fresh187 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh187 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh188 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh188 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh189 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh189 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_set();
                current_block_606 = 10653053999995228228;
            }
            85 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"se\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(2 as libc::c_int as isize);
                }
                let fresh190 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh190 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh191 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh191 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh192 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh192 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_use();
                current_block_606 = 10653053999995228228;
            }
            86 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"iew\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(3 as libc::c_int as isize);
                }
                let fresh193 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh193 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh194 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh194 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh195 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh195 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                eval();
                args = 1 as libc::c_int;
                while args < 5 as libc::c_int {
                    if *source_ptr as libc::c_int != ':' as i32 {
                        break;
                    }
                    source_ptr = source_ptr.offset(1);
                    source_ptr;
                    eval();
                    args += 1;
                    args;
                }
                if args < 2 as libc::c_int {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                let fresh196 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh196 = 138 as libc::c_int as u_char;
                let fresh197 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh197 = args as u_char;
                current_block_606 = 10653053999995228228;
            }
            87 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"rite\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(4 as libc::c_int as isize);
                }
                let fresh198 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh198 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh199 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh199 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh200 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh200 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_write();
                current_block_606 = 10653053999995228228;
            }
            88 => {
                if isalpha(*source_ptr as libc::c_int) != 0 as libc::c_int {
                    if strncasecmp(
                        source_ptr as *mut libc::c_char,
                        b"ecute\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    {
                        comperror(
                            -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                        );
                        return;
                    }
                    source_ptr = source_ptr.offset(5 as libc::c_int as isize);
                }
                let fresh201 = source_ptr;
                source_ptr = source_ptr.offset(1);
                c = *fresh201 as libc::c_char;
                if c as libc::c_int == ':' as i32 {
                    eval();
                    let fresh202 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh202 = 5 as libc::c_int as u_char;
                    jmp_eoc = comp_ptr;
                    comp_ptr = comp_ptr
                        .offset(
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                as isize,
                        );
                    let fresh203 = source_ptr;
                    source_ptr = source_ptr.offset(1);
                    c = *fresh203 as libc::c_char;
                }
                if c as libc::c_int != ' ' as i32 {
                    comperror(
                        -(13 as libc::c_int + 200 as libc::c_int) as libc::c_short,
                    );
                    return;
                }
                parse_xecute();
                current_block_606 = 10653053999995228228;
            }
            _ => {
                comperror(-(13 as libc::c_int + 200 as libc::c_int) as libc::c_short);
                return;
            }
        }
        match current_block_606 {
            10653053999995228228 => {}
            _ => {
                let fresh110 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh110 = 0 as libc::c_int as u_char;
                return;
            }
        }
        if !jmp_eoc.is_null() {
            *(jmp_eoc
                as *mut libc::c_short) = (comp_ptr.offset_from(jmp_eoc) as libc::c_long
                as libc::c_ulong)
                .wrapping_sub(::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                as libc::c_short;
        }
        let fresh204 = comp_ptr;
        comp_ptr = comp_ptr.offset(1);
        *fresh204 = 4 as libc::c_int as u_char;
    };
}
