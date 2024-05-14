#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct CSTRING {
    pub len: u_short,
    pub buf: [u_char; 65535],
}
pub type cstring = CSTRING;
#[no_mangle]
pub unsafe extern "C" fn patmat(
    mut str: *mut cstring,
    mut code: *mut cstring,
) -> libc::c_short {
    let mut ch: u_char = 0;
    let mut f: u_char = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut group: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut buf: [u_char; 257] = [
        '\0' as i32 as u_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut tmp: *mut cstring = 0 as *mut cstring;
    tmp = buf.as_mut_ptr() as *mut cstring;
    ch = (*code).buf[x as usize];
    if (ch as libc::c_int > '9' as i32 || (ch as libc::c_int) < '0' as i32)
        && ch as libc::c_int != '.' as i32
    {
        return -(10 as libc::c_int) as libc::c_short;
    }
    (*tmp).buf[0 as libc::c_int as usize] = ch;
    i = 1 as libc::c_int;
    f = '1' as i32 as u_char;
    if ch as libc::c_int == '.' as i32 {
        j = 1 as libc::c_int;
    } else {
        j = 0 as libc::c_int;
    }
    group = 0 as libc::c_int;
    loop {
        x += 1;
        if !(x < (*code).len as libc::c_int) {
            break;
        }
        ch = (*code).buf[x as usize];
        if ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32 {
            let fresh0 = i;
            i = i + 1;
            (*tmp).buf[fresh0 as usize] = ch;
            f = '1' as i32 as u_char;
        } else if ch as libc::c_int == '.' as i32 {
            if j != 0 {
                return -(10 as libc::c_int) as libc::c_short;
            }
            j += 1;
            j;
            let fresh1 = i;
            i = i + 1;
            (*tmp).buf[fresh1 as usize] = ch;
            f = '1' as i32 as u_char;
        } else {
            j = 0 as libc::c_int;
            if ch as libc::c_int == '\'' as i32 {
                ch = (*code).buf[(x + 1 as libc::c_int) as usize];
                if ch as libc::c_int == '"' as i32
                    || ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32
                    || ch as libc::c_int >= 'a' as i32 && ch as libc::c_int <= 'z' as i32
                {
                    let fresh2 = i;
                    i = i + 1;
                    (*tmp).buf[fresh2 as usize] = '\'' as i32 as u_char;
                } else {
                    ch = '\'' as i32 as u_char;
                }
            }
            if ch as libc::c_int == '"' as i32 {
                if f as libc::c_int != '1' as i32 && f as libc::c_int != 'A' as i32 {
                    return -(10 as libc::c_int) as libc::c_short;
                }
                loop {
                    let fresh3 = i;
                    i = i + 1;
                    (*tmp).buf[fresh3 as usize] = ch;
                    x += 1;
                    ch = (*code).buf[x as usize];
                    if x >= (*code).len as libc::c_int {
                        return -(10 as libc::c_int) as libc::c_short;
                    }
                    if !(ch as libc::c_int == '"' as i32
                        || ch as libc::c_int == 255 as libc::c_int)
                    {
                        continue;
                    }
                    f = (*code).buf[(x + 1 as libc::c_int) as usize];
                    if f as libc::c_int != '"' as i32 {
                        ch = 255 as libc::c_int as u_char;
                        break;
                    } else {
                        x += 1;
                        x;
                    }
                }
                let fresh4 = i;
                i = i + 1;
                (*tmp).buf[fresh4 as usize] = ch;
                f = '"' as i32 as u_char;
            } else if ch as libc::c_int == '(' as i32 {
                if f as libc::c_int != '1' as i32 {
                    return -(10 as libc::c_int) as libc::c_short;
                }
                group += 1;
                group;
                f = '(' as i32 as u_char;
                let fresh5 = i;
                i = i + 1;
                (*tmp).buf[fresh5 as usize] = ch;
            } else {
                if group != 0
                    && (ch as libc::c_int == ',' as i32
                        || ch as libc::c_int == ')' as i32)
                {
                    if f as libc::c_int == '1' as i32 || f as libc::c_int == '(' as i32 {
                        return -(10 as libc::c_int) as libc::c_short;
                    }
                    if ch as libc::c_int == ',' as i32 {
                        f = '(' as i32 as u_char;
                        let fresh6 = i;
                        i = i + 1;
                        (*tmp).buf[fresh6 as usize] = ch;
                        continue;
                    } else if ch as libc::c_int == ')' as i32 {
                        group -= 1;
                        group;
                        let fresh7 = i;
                        i = i + 1;
                        (*tmp).buf[fresh7 as usize] = ch;
                        continue;
                    }
                }
                if ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32 {
                    ch = (ch as libc::c_int + 32 as libc::c_int) as u_char;
                }
                if ch as libc::c_int != 'c' as i32 && ch as libc::c_int != 'n' as i32
                    && ch as libc::c_int != 'p' as i32 && ch as libc::c_int != 'a' as i32
                    && ch as libc::c_int != 'l' as i32 && ch as libc::c_int != 'u' as i32
                    && ch as libc::c_int != 'e' as i32
                {
                    break;
                }
                if f as libc::c_int != '1' as i32 && f as libc::c_int != 'A' as i32 {
                    return -(10 as libc::c_int) as libc::c_short;
                }
                if j != 0 {
                    ch = (ch as libc::c_int - 32 as libc::c_int) as u_char;
                    j = 0 as libc::c_int;
                }
                let fresh8 = i;
                i = i + 1;
                (*tmp).buf[fresh8 as usize] = ch;
                f = 'A' as i32 as u_char;
            }
        }
    }
    if f as libc::c_int == '1' as i32 || group != 0 {
        return -(10 as libc::c_int) as libc::c_short;
    }
    let fresh9 = i;
    i = i + 1;
    (*tmp).buf[fresh9 as usize] = 255 as libc::c_int as u_char;
    (*tmp).len = i as u_short;
    return pattern(str, tmp);
}
#[no_mangle]
pub unsafe extern "C" fn pminmax(
    mut str: *mut cstring,
    mut min: *mut libc::c_int,
    mut max: *mut libc::c_int,
) {
    let mut mininc: libc::c_int = 0;
    let mut maxinc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    *min = 0 as libc::c_int;
    *max = 0 as libc::c_int;
    i = 0 as libc::c_int;
    ch = 0 as libc::c_int;
    while i < (*str).len as libc::c_int - 1 as libc::c_int {
        if (*str).buf[i as usize] as libc::c_int != '.' as i32 {
            let fresh10 = i;
            i = i + 1;
            ch = (*str).buf[fresh10 as usize] as libc::c_int - '0' as i32;
            while (*str).buf[i as usize] as libc::c_int >= '0' as i32
                && (*str).buf[i as usize] as libc::c_int <= '9' as i32
            {
                ch *= 10 as libc::c_int;
                let fresh11 = i;
                i = i + 1;
                ch += (*str).buf[fresh11 as usize] as libc::c_int - '0' as i32;
            }
            mininc = ch;
            maxinc = ch;
        } else {
            mininc = 0 as libc::c_int;
            maxinc = 255 as libc::c_int;
        }
        if (*str).buf[i as usize] as libc::c_int == '.' as i32 {
            i += 1;
            i;
            if (*str).buf[i as usize] as libc::c_int >= '0' as i32
                && (*str).buf[i as usize] as libc::c_int <= '9' as i32
            {
                let fresh12 = i;
                i = i + 1;
                ch = (*str).buf[fresh12 as usize] as libc::c_int - '0' as i32;
                while (*str).buf[i as usize] as libc::c_int >= '0' as i32
                    && (*str).buf[i as usize] as libc::c_int <= '9' as i32
                {
                    ch *= 10 as libc::c_int;
                    let fresh13 = i;
                    i = i + 1;
                    ch += (*str).buf[fresh13 as usize] as libc::c_int - '0' as i32;
                }
            } else {
                ch = 255 as libc::c_int;
            }
            maxinc = ch;
        }
        if (*str).buf[i as usize] as libc::c_int == '"' as i32 {
            let mut cnt: libc::c_int = 0;
            cnt = 0 as libc::c_int;
            loop {
                i += 1;
                if !((*str).buf[i as usize] as libc::c_int != 255 as libc::c_int) {
                    break;
                }
                cnt += 1;
                cnt;
            }
            mininc = mininc * cnt;
            maxinc = maxinc * cnt;
        }
        if (*str).buf[i as usize] as libc::c_int == '"' as i32 {
            loop {
                i += 1;
                if !((*str).buf[i as usize] as libc::c_int != 255 as libc::c_int) {
                    break;
                }
            }
            i += 1;
            i;
        } else if (*str).buf[i as usize] as libc::c_int == '(' as i32 {
            let mut temp: [u_char; 257] = [0; 257];
            let mut tmp: *mut cstring = 0 as *mut cstring;
            let mut tcur: *mut u_char = 0 as *mut u_char;
            let mut tmin: libc::c_int = 0;
            let mut tmax: libc::c_int = 0;
            let mut Tmin: libc::c_int = 0;
            let mut Tmax: libc::c_int = 0;
            let mut i1: libc::c_short = 0;
            tmp = temp.as_mut_ptr() as *mut cstring;
            tmin = 255 as libc::c_int;
            tmax = 0 as libc::c_int;
            loop {
                i1 = 1 as libc::c_int as libc::c_short;
                tcur = ((*tmp).buf).as_mut_ptr();
                while i1 != 0 {
                    i += 1;
                    ch = (*str).buf[i as usize] as libc::c_int;
                    let fresh14 = tcur;
                    tcur = tcur.offset(1);
                    *fresh14 = ch as u_char;
                    if ch == '"' as i32 {
                        loop {
                            i += 1;
                            let fresh15 = tcur;
                            tcur = tcur.offset(1);
                            *fresh15 = (*str).buf[i as usize];
                            if !(*fresh15 as libc::c_int != 255 as libc::c_int) {
                                break;
                            }
                        }
                    }
                    if ch == '(' as i32 {
                        i1 += 1;
                        i1;
                    }
                    if ch == ')' as i32 {
                        i1 -= 1;
                        i1;
                    }
                    if ch == ',' as i32 && i1 as libc::c_int == 1 as libc::c_int {
                        i1 -= 1;
                        i1;
                    }
                }
                tcur = tcur.offset(-1);
                *tcur = 255 as libc::c_int as u_char;
                (*tmp)
                    .len = (tcur.offset_from(((*tmp).buf).as_mut_ptr()) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as u_short;
                pminmax(tmp, &mut Tmin, &mut Tmax);
                if Tmin < tmin {
                    tmin = Tmin;
                }
                if Tmax > tmax {
                    tmax = Tmax;
                }
                if !((*str).buf[i as usize] as libc::c_int == ',' as i32) {
                    break;
                }
            }
            i += 1;
            i;
            mininc *= tmin;
            maxinc *= tmax;
        } else {
            loop {
                i += 1;
                if !((*str).buf[i as usize] as libc::c_int >= 'A' as i32) {
                    break;
                }
            }
        }
        *min += mininc;
        *max += maxinc;
    }
    if *max > 255 as libc::c_int {
        *max = 255 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pattern(
    mut a: *mut cstring,
    mut b: *mut cstring,
) -> libc::c_short {
    let mut levels: libc::c_short = 0;
    let mut patx: libc::c_int = 0;
    let mut notpatclass: libc::c_short = 0;
    let mut ptrpcd: [libc::c_short; 20] = [0; 20];
    let mut position: [libc::c_short; 20] = [0; 20];
    let mut mincnt: [libc::c_short; 20] = [
        0 as libc::c_int as libc::c_short,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut maxcnt: [libc::c_short; 20] = [0; 20];
    let mut actcnt: [libc::c_short; 20] = [0; 20];
    let mut Pflag: libc::c_short = 0;
    let mut Pchar: libc::c_short = 0;
    let mut altc: libc::c_short = 0;
    let mut altcnt: [libc::c_short; 20] = [0; 20];
    let mut gpmin: [[[u_char; 255]; 20]; 20] = [[[0; 255]; 20]; 20];
    let mut gp_position: [[libc::c_short; 20]; 20] = [[0; 20]; 20];
    let mut ptrtom: *mut u_char = 0 as *mut u_char;
    let mut patcode: u_char = 0;
    let mut ch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    notpatclass = 0 as libc::c_int as libc::c_short;
    patx = 0 as libc::c_int;
    x = 0 as libc::c_int;
    while x < (*b).len as libc::c_int - 1 as libc::c_int {
        mincnt[patx as usize] = 0 as libc::c_int as libc::c_short;
        maxcnt[patx as usize] = 255 as libc::c_int as libc::c_short;
        altcnt[patx as usize] = -(1 as libc::c_int) as libc::c_short;
        if (*b).buf[x as usize] as libc::c_int != '.' as i32 {
            let fresh16 = x;
            x = x + 1;
            ch = (*b).buf[fresh16 as usize] as libc::c_int - '0' as i32;
            while (*b).buf[x as usize] as libc::c_int >= '0' as i32
                && (*b).buf[x as usize] as libc::c_int <= '9' as i32
            {
                ch *= 10 as libc::c_int;
                let fresh17 = x;
                x = x + 1;
                ch += (*b).buf[fresh17 as usize] as libc::c_int - '0' as i32;
            }
            mincnt[patx as usize] = ch as libc::c_short;
            if (*b).buf[x as usize] as libc::c_int != '.' as i32 {
                maxcnt[patx as usize] = ch as libc::c_short;
            }
        }
        if (*b).buf[x as usize] as libc::c_int == '.' as i32 {
            x += 1;
            x;
            if (*b).buf[x as usize] as libc::c_int >= '0' as i32
                && (*b).buf[x as usize] as libc::c_int <= '9' as i32
            {
                let fresh18 = x;
                x = x + 1;
                ch = (*b).buf[fresh18 as usize] as libc::c_int - '0' as i32;
                while (*b).buf[x as usize] as libc::c_int >= '0' as i32
                    && (*b).buf[x as usize] as libc::c_int <= '9' as i32
                {
                    ch *= 10 as libc::c_int;
                    let fresh19 = x;
                    x = x + 1;
                    ch += (*b).buf[fresh19 as usize] as libc::c_int - '0' as i32;
                }
                maxcnt[patx as usize] = ch as libc::c_short;
            }
        }
        if (maxcnt[patx as usize] as libc::c_int) < mincnt[patx as usize] as libc::c_int
        {
            return -(10 as libc::c_int) as libc::c_short;
        }
        ptrpcd[patx as usize] = x as libc::c_short;
        actcnt[patx as usize] = 0 as libc::c_int as libc::c_short;
        position[patx as usize] = 0 as libc::c_int as libc::c_short;
        if (*b).buf[x as usize] as libc::c_int == '"' as i32
            || (*b).buf[x as usize] as libc::c_int == '\'' as i32
                && (*b).buf[(x + 1 as libc::c_int) as usize] as libc::c_int == '"' as i32
        {
            x += 1;
            if (*b).buf[x as usize] as libc::c_int == 255 as libc::c_int {
                x += 1;
                x;
                continue;
            } else {
                loop {
                    x += 1;
                    if !((*b).buf[x as usize] as libc::c_int != 255 as libc::c_int) {
                        break;
                    }
                }
                x += 1;
                x;
            }
        } else if (*b).buf[x as usize] as libc::c_int == '(' as i32 {
            i = 1 as libc::c_int;
            x += 1;
            x;
            while x < (*b).len as libc::c_int {
                ch = (*b).buf[x as usize] as libc::c_int;
                x += 1;
                x;
                if x == (*b).len as libc::c_int {
                    continue;
                }
                if ch == '"' as i32 {
                    loop {
                        x += 1;
                        if !((*b).buf[x as usize] as libc::c_int != 255 as libc::c_int) {
                            break;
                        }
                    }
                }
                if ch == '(' as i32 {
                    i += 1;
                    i;
                } else {
                    if !(ch == ')' as i32) {
                        continue;
                    }
                    i -= 1;
                    i;
                    if i < 1 as libc::c_int {
                        break;
                    }
                }
            }
        } else {
            loop {
                x += 1;
                if !((*b).buf[x as usize] as libc::c_int >= 'A' as i32
                    && (*b).buf[x as usize] as libc::c_int <= 'z' as i32)
                {
                    break;
                }
            }
        }
        patx += 1;
        if patx >= 20 as libc::c_int - 1 as libc::c_int {
            return -(10 as libc::c_int) as libc::c_short;
        }
    }
    levels = patx as libc::c_short;
    if (*b).buf[(x - 1 as libc::c_int) as usize] as libc::c_int == 'e' as i32
        && mincnt[(levels as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int
            == 0 as libc::c_int
        && maxcnt[(levels as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int
            == 255 as libc::c_int
    {
        (*b).buf[(x - 1 as libc::c_int) as usize] = '~' as i32 as u_char;
    }
    maxcnt[levels as usize] = 1 as libc::c_int as libc::c_short;
    mincnt[levels as usize] = maxcnt[levels as usize];
    actcnt[levels as usize] = 0 as libc::c_int as libc::c_short;
    ptrpcd[levels as usize] = x as libc::c_short;
    patx = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while patx <= levels as libc::c_int {
        's_323: while (actcnt[patx as usize] as libc::c_int)
            < mincnt[patx as usize] as libc::c_int
        {
            actcnt[patx as usize] += 1;
            actcnt[patx as usize];
            if y >= (*a).len as libc::c_int {
                if patx >= levels as libc::c_int {
                    return 1 as libc::c_int as libc::c_short;
                }
                if patx > 0 as libc::c_int {
                    if actcnt[(patx - 1 as libc::c_int) as usize] as libc::c_int
                        != maxcnt[(patx - 1 as libc::c_int) as usize] as libc::c_int
                    {
                        return 0 as libc::c_int as libc::c_short;
                    }
                    if (*b).buf[ptrpcd[(patx - 1 as libc::c_int) as usize] as usize]
                        as libc::c_int == '(' as i32
                    {
                        return 0 as libc::c_int as libc::c_short;
                    }
                }
                if (*b).buf[ptrpcd[patx as usize] as usize] as libc::c_int == '"' as i32
                {
                    return 0 as libc::c_int as libc::c_short;
                }
            }
            's_381: loop {
                ptrtom = &mut *((*b).buf)
                    .as_mut_ptr()
                    .offset(*ptrpcd.as_mut_ptr().offset(patx as isize) as isize)
                    as *mut u_char;
                ch = (*a).buf[y as usize] as libc::c_int;
                's_393: loop {
                    let fresh20 = ptrtom;
                    ptrtom = ptrtom.offset(1);
                    patcode = *fresh20;
                    notpatclass = (patcode as libc::c_int == '\'' as i32) as libc::c_int
                        as libc::c_short;
                    if notpatclass != 0 {
                        let fresh21 = ptrtom;
                        ptrtom = ptrtom.offset(1);
                        patcode = *fresh21;
                    }
                    match patcode as libc::c_int {
                        99 => {
                            if !((ch < ' ' as i32 && ch >= '\0' as i32
                                || ch == '\u{7f}' as i32) as libc::c_int
                                != notpatclass as libc::c_int)
                            {
                                continue;
                            }
                            break 's_381;
                        }
                        110 => {
                            if (ch <= '9' as i32 && ch >= '0' as i32) as libc::c_int
                                != notpatclass as libc::c_int
                            {
                                break 's_381;
                            }
                        }
                        112 => {
                            if !((ch >= ' ' as i32 && ch <= '/' as i32
                                || ch >= ':' as i32 && ch <= '@' as i32
                                || ch >= '[' as i32 && ch <= '`' as i32
                                || ch >= '{' as i32 && ch <= '~' as i32 || ch == -128i32)
                                as libc::c_int != notpatclass as libc::c_int)
                            {
                                continue;
                            }
                            break 's_381;
                        }
                        97 => {
                            if !((ch >= 'A' as i32 && ch <= 'Z' as i32
                                || ch >= 'a' as i32 && ch <= 'z' as i32) as libc::c_int
                                != notpatclass as libc::c_int)
                            {
                                continue;
                            }
                            break 's_381;
                        }
                        108 => {
                            if !((ch >= 'a' as i32 && ch <= 'z' as i32) as libc::c_int
                                != notpatclass as libc::c_int)
                            {
                                continue;
                            }
                            break 's_381;
                        }
                        117 => {
                            if !((ch >= 'A' as i32 && ch <= 'Z' as i32) as libc::c_int
                                != notpatclass as libc::c_int)
                            {
                                continue;
                            }
                            break 's_381;
                        }
                        101 => {
                            if (ch > 0 as libc::c_int) as libc::c_int
                                != notpatclass as libc::c_int
                            {
                                break 's_381;
                            }
                        }
                        34 => {
                            i = 0 as libc::c_int;
                            loop {
                                let fresh22 = i;
                                i = i + 1;
                                let fresh23 = ptrtom;
                                ptrtom = ptrtom.offset(1);
                                if !((*a).buf[(y + fresh22) as usize] as libc::c_int
                                    == *fresh23 as libc::c_int)
                                {
                                    break;
                                }
                            }
                            ptrtom = ptrtom.offset(-1);
                            if *ptrtom as libc::c_int == 255 as libc::c_int {
                                if notpatclass != 0 {
                                    break;
                                }
                                x = ptrpcd[patx as usize] as libc::c_int + 1 as libc::c_int;
                                loop {
                                    let fresh24 = x;
                                    x = x + 1;
                                    if !((*b).buf[fresh24 as usize] as libc::c_int
                                        != 255 as libc::c_int)
                                    {
                                        break;
                                    }
                                    y += 1;
                                    y;
                                }
                                continue 's_323;
                            } else if notpatclass != 0 {
                                i -= 1;
                                i;
                                loop {
                                    let fresh25 = ptrtom;
                                    ptrtom = ptrtom.offset(1);
                                    if !(*fresh25 as libc::c_int != 255 as libc::c_int) {
                                        break;
                                    }
                                    let fresh26 = i;
                                    i = i + 1;
                                    if (*a).buf[(y + fresh26) as usize] as libc::c_int
                                        == 255 as libc::c_int
                                    {
                                        break 's_393;
                                    }
                                }
                                x = ptrpcd[patx as usize] as libc::c_int + 2 as libc::c_int;
                                loop {
                                    let fresh27 = x;
                                    x = x + 1;
                                    if !((*b).buf[fresh27 as usize] as libc::c_int
                                        != 255 as libc::c_int)
                                    {
                                        break;
                                    }
                                    y += 1;
                                    y;
                                }
                                continue 's_323;
                            } else {
                                break;
                            }
                        }
                        126 => return 1 as libc::c_int as libc::c_short,
                        40 => {
                            let mut aaa: [u_char; 256] = [0; 256];
                            let mut aa: *mut cstring = 0 as *mut cstring;
                            let mut bbb: [u_char; 256] = [0; 256];
                            let mut bb: *mut cstring = 0 as *mut cstring;
                            let mut i1: libc::c_short = 0;
                            let mut min: libc::c_int = 0;
                            let mut max: libc::c_int = 0;
                            let mut pflag: libc::c_short = 0;
                            let mut pchar: libc::c_short = 0;
                            aa = aaa.as_mut_ptr() as *mut cstring;
                            bb = bbb.as_mut_ptr() as *mut cstring;
                            if Pflag != 0 {
                                pflag = Pflag;
                                pchar = Pchar;
                            } else {
                                pflag = 0 as libc::c_int as libc::c_short;
                                pchar = 255 as libc::c_int as libc::c_short;
                            }
                            if (altcnt[patx as usize] as libc::c_int) < 0 as libc::c_int
                            {
                                altc = 0 as libc::c_int as libc::c_short;
                                while (altc as libc::c_int) < 20 as libc::c_int {
                                    gpmin[patx
                                        as usize][altc
                                        as usize][1 as libc::c_int
                                        as usize] = 0 as libc::c_int as u_char;
                                    altc += 1;
                                    altc;
                                }
                            }
                            altcnt[patx as usize] = 0 as libc::c_int as libc::c_short;
                            loop {
                                i = 0 as libc::c_int;
                                i1 = 1 as libc::c_int as libc::c_short;
                                while i1 != 0 {
                                    let fresh28 = ptrtom;
                                    ptrtom = ptrtom.offset(1);
                                    (*bb).buf[i as usize] = *fresh28;
                                    if (*bb).buf[i as usize] as libc::c_int == '"' as i32 {
                                        loop {
                                            let fresh29 = ptrtom;
                                            ptrtom = ptrtom.offset(1);
                                            i += 1;
                                            (*bb).buf[i as usize] = *fresh29;
                                            if !((*bb).buf[i as usize] as libc::c_int
                                                != 255 as libc::c_int)
                                            {
                                                break;
                                            }
                                        }
                                    }
                                    if (*bb).buf[i as usize] as libc::c_int == '(' as i32 {
                                        i1 += 1;
                                        i1;
                                    }
                                    if (*bb).buf[i as usize] as libc::c_int == ')' as i32 {
                                        i1 -= 1;
                                        i1;
                                    }
                                    if (*bb).buf[i as usize] as libc::c_int == ',' as i32
                                        && i1 as libc::c_int == 1 as libc::c_int
                                    {
                                        i1 -= 1;
                                        i1;
                                    }
                                    i += 1;
                                    i;
                                }
                                i -= 1;
                                (*bb).len = i as u_short;
                                pminmax(bb, &mut min, &mut max);
                                i1 = gpmin[patx
                                    as usize][altcnt[patx as usize]
                                    as usize][actcnt[patx as usize] as usize] as libc::c_short;
                                if (i1 as libc::c_int) < min {
                                    i1 = min as libc::c_short;
                                    gpmin[patx
                                        as usize][altcnt[patx as usize]
                                        as usize][actcnt[patx as usize] as usize] = i1 as u_char;
                                }
                                gpmin[patx
                                    as usize][altcnt[patx as usize]
                                    as usize][(actcnt[patx as usize] as libc::c_int
                                    + 1 as libc::c_int) as usize] = 0 as libc::c_int as u_char;
                                if i1 as libc::c_int > max {
                                    if !(*ptrtom.offset(-(1 as libc::c_int as isize))
                                        as libc::c_int == ',' as i32)
                                    {
                                        break 's_393;
                                    }
                                    altcnt[patx as usize] += 1;
                                    altcnt[patx as usize];
                                } else {
                                    i = y;
                                    while i < i1 as libc::c_int + y {
                                        if i >= (*a).len as libc::c_int {
                                            break;
                                        }
                                        (*aa).buf[(i - y) as usize] = (*a).buf[i as usize];
                                        i += 1;
                                        i;
                                    }
                                    gp_position[patx
                                        as usize][actcnt[patx as usize]
                                        as usize] = y as libc::c_short;
                                    loop {
                                        (*aa).len = (i - y) as u_short;
                                        i1 = patmat(aa, bb);
                                        if i1 as libc::c_int == 1 as libc::c_int {
                                            gpmin[patx
                                                as usize][altcnt[patx as usize]
                                                as usize][actcnt[patx as usize]
                                                as usize] = (i - y) as u_char;
                                            y += i - y;
                                            continue 's_323;
                                        } else {
                                            if i1 as libc::c_int != 0 as libc::c_int {
                                                return i1;
                                            }
                                            if i >= (*a).len as libc::c_int {
                                                Pflag = pflag;
                                                Pchar = pchar;
                                                if *ptrtom.offset(-(1 as libc::c_int as isize))
                                                    as libc::c_int == ',' as i32
                                                {
                                                    altcnt[patx as usize] += 1;
                                                    altcnt[patx as usize];
                                                    break;
                                                } else {
                                                    return 0 as libc::c_int as libc::c_short
                                                }
                                            } else {
                                                (*aa).buf[(i - y) as usize] = (*a).buf[i as usize];
                                                i += 1;
                                                i;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }
                '_nomatch: loop {
                    if patcode as libc::c_int == '(' as i32 {
                        altc = 0 as libc::c_int as libc::c_short;
                        while altc as libc::c_int <= altcnt[patx as usize] as libc::c_int
                        {
                            gpmin[patx
                                as usize][altc
                                as usize][actcnt[patx as usize]
                                as usize] = 0 as libc::c_int as u_char;
                            altc += 1;
                            altc;
                        }
                        actcnt[patx as usize] -= 1;
                        if actcnt[patx as usize] as libc::c_int > 0 as libc::c_int {
                            altc = 0 as libc::c_int as libc::c_short;
                            while altc as libc::c_int
                                <= altcnt[patx as usize] as libc::c_int
                            {
                                gpmin[patx
                                    as usize][altc
                                    as usize][actcnt[patx as usize]
                                    as usize] = (gpmin[patx
                                    as usize][altc as usize][actcnt[patx as usize] as usize])
                                    .wrapping_add(1);
                                gpmin[patx
                                    as usize][altc as usize][actcnt[patx as usize] as usize];
                                altc += 1;
                                altc;
                            }
                            y = gp_position[patx
                                as usize][actcnt[patx as usize] as usize] as libc::c_int;
                            break;
                        }
                    }
                    loop {
                        actcnt[patx as usize] = 0 as libc::c_int as libc::c_short;
                        patx -= 1;
                        if patx < 0 as libc::c_int {
                            return 0 as libc::c_int as libc::c_short;
                        }
                        if (*b).buf[ptrpcd[patx as usize] as usize] as libc::c_int
                            == '(' as i32
                        {
                            if actcnt[patx as usize] as libc::c_int
                                >= maxcnt[patx as usize] as libc::c_int
                            {
                                actcnt[patx as usize] += 1;
                                actcnt[patx as usize];
                                patcode = '(' as i32 as u_char;
                                continue '_nomatch;
                            }
                        }
                        actcnt[patx as usize] += 1;
                        if !(actcnt[patx as usize] as libc::c_int
                            > maxcnt[patx as usize] as libc::c_int)
                        {
                            break;
                        }
                    }
                    y = position[patx as usize] as libc::c_int;
                    break;
                }
            }
            y += 1;
            y;
        }
        let fresh30 = patx;
        patx = patx + 1;
        position[fresh30 as usize] = y as libc::c_short;
    }
    return 0 as libc::c_int as libc::c_short;
}
