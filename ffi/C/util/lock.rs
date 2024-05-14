#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type GBD;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn itocstring(buf: *mut u_char, n: libc::c_int) -> u_short;
    fn current_time(local: libc::c_short) -> time_t;
    fn SS_Norm(var: *mut mvar) -> libc::c_short;
    fn UTIL_String_Key(
        key: *mut u_char,
        str: *mut u_char,
        max_subs: libc::c_int,
    ) -> libc::c_short;
    fn panic(msg: *mut libc::c_char);
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type off_t = __darwin_off_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
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
pub struct RBD {
    pub fwd_link: *mut RBD,
    pub chunk_size: u_int,
    pub attached: u_int,
    pub last_access: time_t,
    pub rnam: var_u,
    pub uci: u_char,
    pub vol: u_char,
    pub rou_size: u_short,
    pub comp_ver: u_short,
    pub comp_user: u_short,
    pub comp_date: libc::c_int,
    pub comp_time: libc::c_int,
    pub tag_tbl: u_short,
    pub num_tags: u_short,
    pub var_tbl: u_short,
    pub num_vars: u_short,
    pub code: u_short,
    pub code_size: u_short,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct UCI_TAB {
    pub name: var_u,
    pub global: u_int,
}
pub type uci_tab = UCI_TAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union DATA_UNION {
    pub gbddata: *mut GBD,
    pub intdata: u_int,
}
pub type msg_data = DATA_UNION;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct WD_TAB {
    pub pid: libc::c_int,
    pub doing: libc::c_int,
    pub currmsg: msg_data,
}
pub type wdtab_struct = WD_TAB;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct DB_STAT {
    pub dbget: u_int,
    pub dbset: u_int,
    pub dbkil: u_int,
    pub dbdat: u_int,
    pub dbord: u_int,
    pub dbqry: u_int,
    pub lasttry: u_int,
    pub lastok: u_int,
    pub logrd: u_int,
    pub phyrd: u_int,
    pub logwt: u_int,
    pub phywt: u_int,
    pub blkalloc: u_int,
    pub blkdeall: u_int,
    pub blkreorg: u_int,
    pub diskerrors: u_int,
}
pub type db_stat = DB_STAT;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct VOL_DEF {
    pub vollab: *mut label_block,
    pub map: *mut libc::c_void,
    pub first_free: *mut libc::c_void,
    pub gbd_hash: [*mut GBD; 1025],
    pub gbd_head: *mut GBD,
    pub num_gbd: u_int,
    pub global_buf: *mut libc::c_void,
    pub zero_block: *mut libc::c_void,
    pub rbd_hash: [*mut RBD; 1024],
    pub rbd_head: *mut libc::c_void,
    pub rbd_end: *mut libc::c_void,
    pub num_of_daemons: libc::c_int,
    pub wd_tab: [wdtab_struct; 20],
    pub dismount_flag: libc::c_int,
    pub map_dirty_flag: libc::c_int,
    pub writelock: libc::c_int,
    pub upto: u_int,
    pub shm_id: libc::c_int,
    pub dirtyQ: [*mut GBD; 1024],
    pub dirtyQw: libc::c_int,
    pub dirtyQr: libc::c_int,
    pub garbQ: [u_int; 8192],
    pub garbQw: libc::c_int,
    pub garbQr: libc::c_int,
    pub jrn_next: off_t,
    pub file_name: [libc::c_char; 256],
    pub stats: db_stat,
}
pub type vol_def = VOL_DEF;
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
pub struct LOCKTAB {
    pub fwd_link: *mut LOCKTAB,
    pub size: libc::c_int,
    pub job: libc::c_short,
    pub lock_count: libc::c_short,
    pub byte_count: libc::c_short,
    pub vol: u_char,
    pub uci: u_char,
    pub name: var_u,
    pub key: [u_char; 256],
}
pub type locktab = LOCKTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct TRANTAB {
    pub from_global: var_u,
    pub from_vol: u_char,
    pub from_uci: u_char,
    pub to_global: var_u,
    pub to_vol: u_char,
    pub to_uci: u_char,
}
pub type trantab = TRANTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SYSTAB {
    pub address: *mut libc::c_void,
    pub jobtab: *mut jobtab,
    pub maxjob: u_int,
    pub sem_id: libc::c_int,
    pub historic: libc::c_int,
    pub precision: libc::c_int,
    pub max_tt: libc::c_int,
    pub tt: [trantab; 8],
    pub start_user: libc::c_int,
    pub lockstart: *mut libc::c_void,
    pub locksize: libc::c_int,
    pub lockhead: *mut locktab,
    pub lockfree: *mut locktab,
    pub addoff: u_long,
    pub addsize: u_long,
    pub vol: [*mut vol_def; 1],
    pub last_blk_used: [u_int; 1],
}
pub type systab_struct = SYSTAB;
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
pub type rbd = RBD;
pub type lck_add = LCK_ADD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LCK_ADD {
    pub count: libc::c_int,
    pub to: libc::c_int,
    pub done: libc::c_int,
    pub tryagain: libc::c_int,
    pub currtime: time_t,
    pub strttime: time_t,
    pub x: libc::c_short,
    pub lptr: *mut locktab,
}
#[inline]
unsafe extern "C" fn var_equal(mut var1: var_u, mut var2: var_u) -> u_int {
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        if var1.var_qu[var_i as usize] != var2.var_qu[var_i as usize] {
            return 0 as libc::c_int as u_int;
        }
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    return 1 as libc::c_int as u_int;
}
unsafe extern "C" fn failed(mut pctx: *mut lck_add) -> libc::c_int {
    (*pctx).done = (*pctx).count + 1 as libc::c_int;
    if (*pctx).to == 0 as libc::c_int {
        (*pctx).tryagain = 0 as libc::c_int;
    }
    if (*pctx).to > 0 as libc::c_int {
        (*pctx).currtime = current_time(1 as libc::c_int as libc::c_short);
        if ((*pctx).strttime + (*pctx).to as time_t) < (*pctx).currtime {
            (*pctx).tryagain = 0 as libc::c_int;
        }
    }
    if (*pctx).tryagain == 1 as libc::c_int {
        (*pctx).x = SemOp(1 as libc::c_int, (*systab).maxjob as libc::c_int);
        sleep(1 as libc::c_int as libc::c_uint);
    }
    if (*pctx).tryagain == 0 as libc::c_int {
        (*partab.jobtab).test = 0 as libc::c_int as u_char;
    }
    if (*partab.jobtab).attention != 0 {
        if (*partab.jobtab).trap
            & ((1 as libc::c_uint) << 2 as libc::c_int
                | (1 as libc::c_uint) << 3 as libc::c_int
                | (1 as libc::c_uint) << 15 as libc::c_int
                | (1 as libc::c_uint) << 17 as libc::c_int) != 0
        {
            if (*pctx).tryagain == 0 as libc::c_int {
                (*pctx).x = SemOp(1 as libc::c_int, (*systab).maxjob as libc::c_int);
            }
            return -(51 as libc::c_int + 200 as libc::c_int);
        }
    }
    (*pctx).lptr = 0 as *mut locktab;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_String_Lock(
    mut var: *mut locktab,
    mut str: *mut u_char,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0 as libc::c_int;
    let mut slen: libc::c_int = 0;
    let mut up: uci_tab = UCI_TAB {
        name: VAR_U { var_q: 0 },
        global: 0,
    };
    let mut vp: *mut u_char = 0 as *mut u_char;
    if (*var).uci as libc::c_int != 255 as libc::c_int {
        let fresh0 = p;
        p = p + 1;
        *str.offset(fresh0 as isize) = '^' as i32 as u_char;
        let fresh1 = p;
        p = p + 1;
        *str.offset(fresh1 as isize) = '[' as i32 as u_char;
        let fresh2 = p;
        p = p + 1;
        *str.offset(fresh2 as isize) = '"' as i32 as u_char;
        up = (*(*(*systab).vol[((*var).vol as libc::c_int - 1 as libc::c_int) as usize])
            .vollab)
            .uci[((*var).uci as libc::c_int - 1 as libc::c_int) as usize];
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if up.name.var_cu[i as usize] as libc::c_int == '\0' as i32 {
                break;
            }
            let fresh3 = p;
            p = p + 1;
            *str.offset(fresh3 as isize) = up.name.var_cu[i as usize];
            i += 1;
            i;
        }
        let fresh4 = p;
        p = p + 1;
        *str.offset(fresh4 as isize) = '"' as i32 as u_char;
        let fresh5 = p;
        p = p + 1;
        *str.offset(fresh5 as isize) = ',' as i32 as u_char;
        let fresh6 = p;
        p = p + 1;
        *str.offset(fresh6 as isize) = '"' as i32 as u_char;
        vp = ((*(*(*systab).vol[((*var).vol as libc::c_int - 1 as libc::c_int) as usize])
            .vollab)
            .volnam
            .var_cu)
            .as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if *vp.offset(i as isize) as libc::c_int == '\0' as i32 {
                break;
            }
            let fresh7 = p;
            p = p + 1;
            *str.offset(fresh7 as isize) = *vp.offset(i as isize);
            i += 1;
            i;
        }
        let fresh8 = p;
        p = p + 1;
        *str.offset(fresh8 as isize) = '"' as i32 as u_char;
        let fresh9 = p;
        p = p + 1;
        *str.offset(fresh9 as isize) = ']' as i32 as u_char;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*var).name.var_cu[i as usize] as libc::c_int == '\0' as i32 {
            break;
        }
        let fresh10 = p;
        p = p + 1;
        *str.offset(fresh10 as isize) = (*var).name.var_cu[i as usize];
        i += 1;
        i;
    }
    slen = ((*var).byte_count as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<var_u>() as libc::c_ulong)
        .wrapping_sub(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<u_char>() as libc::c_ulong),
        ) as libc::c_int;
    if slen != 0 as libc::c_int {
        let mut save: u_char = (*var)
            .name
            .var_cu[(32 as libc::c_int - 1 as libc::c_int) as usize];
        (*var)
            .name
            .var_cu[(32 as libc::c_int - 1 as libc::c_int) as usize] = slen as u_char;
        i = UTIL_String_Key(
            &mut *((*var).name.var_cu)
                .as_mut_ptr()
                .offset((32 as libc::c_int - 1 as libc::c_int) as isize),
            &mut *str.offset(p as isize),
            63 as libc::c_int,
        ) as libc::c_int;
        (*var).name.var_cu[(32 as libc::c_int - 1 as libc::c_int) as usize] = save;
        if i < 0 as libc::c_int {
            return i as libc::c_short;
        }
        p += i;
    }
    *str.offset(p as isize) = '\0' as i32 as u_char;
    return p as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_mvartolock(
    mut var: *mut mvar,
    mut buf: *mut u_char,
) -> libc::c_short {
    let mut s: libc::c_short = 0;
    let mut vt: *mut var_u = 0 as *mut var_u;
    if (*var).uci as libc::c_int == 255 as libc::c_int {
        if (*var).volset != 0 {
            let mut p: *mut rbd = (*partab.jobtab)
                .dostk[(*partab.jobtab).cur_do as usize]
                .routine as *mut rbd;
            vt = (p as *mut u_char).offset((*p).var_tbl as libc::c_int as isize)
                as *mut var_u;
            let mut var_i: u_int = 0 as libc::c_int as u_int;
            while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                (*(&mut *buf.offset(2 as libc::c_int as isize) as *mut u_char
                    as *mut var_u))
                    .var_qu[var_i
                    as usize] = (*vt
                    .offset(((*var).volset as libc::c_int - 1 as libc::c_int) as isize))
                    .var_qu[var_i as usize];
                var_i = var_i.wrapping_add(1);
                var_i;
            }
        } else {
            let mut var_i_0: u_int = 0 as libc::c_int as u_int;
            while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                (*(&mut *buf.offset(2 as libc::c_int as isize) as *mut u_char
                    as *mut var_u))
                    .var_qu[var_i_0 as usize] = (*var).name.var_qu[var_i_0 as usize];
                var_i_0 = var_i_0.wrapping_add(1);
                var_i_0;
            }
        }
        memmove(
            &mut *buf
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(::core::mem::size_of::<var_u>() as libc::c_ulong)
                        as isize,
                ) as *mut u_char as *mut libc::c_void,
            &mut *((*var).key).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            (*var).slen as libc::c_ulong,
        );
        s = ((*var).slen as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<var_u>() as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_short;
        *buf.offset(0 as libc::c_int as isize) = 0 as libc::c_int as u_char;
        *buf.offset(1 as libc::c_int as isize) = 255 as libc::c_int as u_char;
        *buf.offset(s as isize) = '\0' as i32 as u_char;
        return s;
    }
    if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int == '$' as i32 {
        s = SS_Norm(var);
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
    }
    *buf.offset(0 as libc::c_int as isize) = (*var).volset;
    *buf.offset(1 as libc::c_int as isize) = (*var).uci;
    let mut var_i_1: u_int = 0 as libc::c_int as u_int;
    while var_i_1 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        (*(&mut *buf.offset(2 as libc::c_int as isize) as *mut u_char as *mut var_u))
            .var_qu[var_i_1 as usize] = (*var).name.var_qu[var_i_1 as usize];
        var_i_1 = var_i_1.wrapping_add(1);
        var_i_1;
    }
    if *buf.offset(0 as libc::c_int as isize) == 0 {
        *buf.offset(0 as libc::c_int as isize) = (*partab.jobtab).lvol;
    }
    if *buf.offset(1 as libc::c_int as isize) == 0 {
        if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
            *buf.offset(1 as libc::c_int as isize) = 1 as libc::c_int as u_char;
        } else {
            *buf.offset(1 as libc::c_int as isize) = (*partab.jobtab).luci;
        }
    }
    if (*var).volset as libc::c_int == 0 as libc::c_int
        && (*var).uci as libc::c_int == 0 as libc::c_int
    {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*systab).max_tt {
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int
                == (*systab).tt[i as usize].from_vol as libc::c_int
                && *buf.offset(1 as libc::c_int as isize) as libc::c_int
                    == (*systab).tt[i as usize].from_uci as libc::c_int
                && var_equal((*var).name, (*systab).tt[i as usize].from_global) != 0
            {
                *buf.offset(0 as libc::c_int as isize) = (*systab).tt[i as usize].to_vol;
                *buf.offset(1 as libc::c_int as isize) = (*systab).tt[i as usize].to_uci;
                let mut var_i_2: u_int = 0 as libc::c_int as u_int;
                while var_i_2 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*(&mut *buf.offset(2 as libc::c_int as isize) as *mut u_char
                        as *mut var_u))
                        .var_qu[var_i_2
                        as usize] = (*systab)
                        .tt[i as usize]
                        .to_global
                        .var_qu[var_i_2 as usize];
                    var_i_2 = var_i_2.wrapping_add(1);
                    var_i_2;
                }
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    memmove(
        &mut *buf
            .offset(
                (::core::mem::size_of::<var_u>() as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut u_char as *mut libc::c_void,
        &mut *((*var).key).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *const libc::c_void,
        (*var).slen as libc::c_ulong,
    );
    s = ((*var).slen as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<var_u>() as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_short;
    *buf.offset(s as isize) = '\0' as i32 as u_char;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Combine(mut ptr: *mut locktab) -> libc::c_short {
    let mut next: *mut locktab = 0 as *mut locktab;
    if ptr.is_null() {
        panic(
            b"Null pointer passed to LCK_Combine()\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else {
        loop {
            next = (ptr as *mut u_char).offset((*ptr).size as isize) as *mut locktab;
            if next as *mut libc::c_char
                >= ((*systab).lockstart as *mut libc::c_char)
                    .offset((*systab).locksize as isize)
            {
                break;
            }
            if next != (*ptr).fwd_link {
                break;
            }
            if (*next).job as libc::c_int > -(1 as libc::c_int) {
                panic(
                    b"Attempt to combine non-free in LCK_Combine()\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            (*ptr).size += (*(*ptr).fwd_link).size;
            (*ptr).fwd_link = (*next).fwd_link;
        }
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Free(mut ptr: *mut locktab) -> libc::c_short {
    let mut currptr: *mut locktab = 0 as *mut locktab;
    let mut prevptr: *mut locktab = 0 as *mut locktab;
    if ptr.is_null() {
        panic(
            b"Null pointer passed to LCK_Free()\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    currptr = (*systab).lockfree;
    if ptr < currptr || currptr.is_null() {
        (*ptr).fwd_link = currptr;
        (*systab).lockfree = ptr;
        return LCK_Combine((*systab).lockfree);
    }
    while !currptr.is_null() && ptr > currptr {
        prevptr = currptr;
        currptr = (*currptr).fwd_link;
    }
    (*ptr).fwd_link = currptr;
    (*prevptr).fwd_link = ptr;
    return LCK_Combine(prevptr);
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Insert(mut size: libc::c_int) -> *mut locktab {
    let mut free_curr: *mut locktab = 0 as *mut locktab;
    let mut free_prev: *mut locktab = 0 as *mut locktab;
    let mut ptr: *mut locktab = 0 as *mut locktab;
    let mut p: *mut locktab = 0 as *mut locktab;
    let mut prevptr: *mut locktab = 0 as *mut locktab;
    let mut ret: libc::c_int = (*systab).locksize + 1 as libc::c_int;
    free_curr = (*systab).lockfree;
    while !free_curr.is_null() {
        if (*free_curr).size < ret && (*free_curr).size > size {
            ptr = free_curr;
            prevptr = free_prev;
            ret = (*free_curr).size;
        }
        free_prev = free_curr;
        free_curr = (*free_curr).fwd_link;
    }
    if ptr.is_null() || ret == (*systab).locksize + 1 as libc::c_int {
        return 0 as *mut locktab
    } else {
        if (size + 64 as libc::c_int) < (*ptr).size {
            p = (ptr as *mut u_char).offset(size as isize) as *mut locktab;
            (*p).fwd_link = (*ptr).fwd_link;
            (*p).size = (*ptr).size - size;
            (*p).job = -(1 as libc::c_int) as libc::c_short;
            (*p).lock_count = 0 as libc::c_int as libc::c_short;
            (*p).byte_count = 0 as libc::c_int as libc::c_short;
            if prevptr.is_null() {
                (*systab).lockfree = p;
            } else {
                (*prevptr).fwd_link = p;
            }
            (*ptr).size = size;
        } else if prevptr.is_null() {
            (*systab).lockfree = (*ptr).fwd_link;
        } else {
            (*prevptr).fwd_link = (*ptr).fwd_link;
        }
        return ptr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Order(
    mut ent: *mut cstring,
    mut buf: *mut u_char,
    mut dir: libc::c_int,
) -> libc::c_short {
    let mut lptr: *mut locktab = 0 as *mut locktab;
    let mut plptr: *mut locktab = 0 as *mut locktab;
    let mut s: libc::c_short = 0;
    let mut x: libc::c_short = 0;
    x = SemOp(1 as libc::c_int, -(1 as libc::c_int));
    if (x as libc::c_int) < 0 as libc::c_int {
        return x;
    }
    lptr = (*systab).lockhead;
    plptr = 0 as *mut locktab;
    while !lptr.is_null() {
        let mut i: libc::c_int = (*ent).len as libc::c_int;
        if i > (*lptr).byte_count as libc::c_int {
            i = (*lptr).byte_count as libc::c_int;
        }
        i = memcmp(
            ((*ent).buf).as_mut_ptr() as *const libc::c_void,
            &mut (*lptr).vol as *mut u_char as *const libc::c_void,
            i as libc::c_ulong,
        );
        if i == 0 as libc::c_int
            && (*lptr).byte_count as libc::c_int != (*ent).len as libc::c_int
        {
            i = 1 as libc::c_int;
            if (*lptr).byte_count as libc::c_int > (*ent).len as libc::c_int {
                i = -(1 as libc::c_int);
            }
        }
        if dir > 0 as libc::c_int && i < 0 as libc::c_int {
            break;
        }
        if dir < 0 as libc::c_int && i <= 0 as libc::c_int {
            lptr = plptr;
            break;
        } else {
            plptr = lptr;
            lptr = (*lptr).fwd_link;
        }
    }
    if dir < 0 as libc::c_int && lptr.is_null() {
        lptr = plptr;
    }
    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    s = 0 as libc::c_int as libc::c_short;
    if !lptr.is_null() {
        s = UTIL_String_Lock(lptr, buf);
    }
    SemOp(1 as libc::c_int, 1 as libc::c_int);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Get(
    mut ent: *mut cstring,
    mut buf: *mut u_char,
) -> libc::c_short {
    let mut lptr: *mut locktab = 0 as *mut locktab;
    let mut i: libc::c_int = 0;
    let mut s: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut x: libc::c_short = 0;
    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    x = SemOp(1 as libc::c_int, -(1 as libc::c_int));
    if (x as libc::c_int) < 0 as libc::c_int {
        return x;
    }
    lptr = (*systab).lockhead;
    i = (*ent).len as libc::c_int;
    while !lptr.is_null() {
        if i == (*lptr).byte_count as libc::c_int
            && memcmp(
                ((*ent).buf).as_mut_ptr() as *const libc::c_void,
                &mut (*lptr).vol as *mut u_char as *const libc::c_void,
                i as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            s = itocstring(buf, (*lptr).job as libc::c_int) as libc::c_short;
            let fresh11 = s;
            s = s + 1;
            *buf.offset(fresh11 as isize) = ',' as i32 as u_char;
            s = (s as libc::c_int
                + itocstring(
                    &mut *buf.offset(s as isize),
                    (*lptr).lock_count as libc::c_int,
                ) as libc::c_int) as libc::c_short;
            break;
        } else {
            lptr = (*lptr).fwd_link;
        }
    }
    SemOp(1 as libc::c_int, 1 as libc::c_int);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Kill(mut ent: *mut cstring) -> libc::c_short {
    let mut lptr: *mut locktab = 0 as *mut locktab;
    let mut plptr: *mut locktab = 0 as *mut locktab;
    let mut i: libc::c_int = 0;
    lptr = (*systab).lockhead;
    plptr = 0 as *mut locktab;
    i = (*ent).len as libc::c_int;
    while !lptr.is_null() {
        if i == (*lptr).byte_count as libc::c_int {
            if memcmp(
                ((*ent).buf).as_mut_ptr() as *const libc::c_void,
                &mut (*lptr).vol as *mut u_char as *const libc::c_void,
                i as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if plptr.is_null() {
                    (*systab).lockhead = (*lptr).fwd_link;
                } else {
                    (*plptr).fwd_link = (*lptr).fwd_link;
                }
                (*lptr).job = -(1 as libc::c_int) as libc::c_short;
                LCK_Free(lptr);
                break;
            }
        }
        plptr = lptr;
        lptr = (*lptr).fwd_link;
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Remove(mut job: libc::c_int) {
    let mut lptr: *mut locktab = 0 as *mut locktab;
    let mut plptr: *mut locktab = 0 as *mut locktab;
    let mut x: libc::c_short = 0;
    if job == 0 {
        job = ((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as libc::c_int;
    }
    x = SemOp(1 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int);
    if (x as libc::c_int) < 0 as libc::c_int {
        return;
    }
    lptr = (*systab).lockhead;
    plptr = 0 as *mut locktab;
    while !lptr.is_null() {
        if (*lptr).job as libc::c_int == job {
            if plptr.is_null() {
                (*systab).lockhead = (*lptr).fwd_link;
                (*lptr).job = -(1 as libc::c_int) as libc::c_short;
                LCK_Free(lptr);
                lptr = (*systab).lockhead;
                plptr = 0 as *mut locktab;
            }
            if !plptr.is_null() && !lptr.is_null() {
                (*plptr).fwd_link = (*lptr).fwd_link;
                (*lptr).job = -(1 as libc::c_int) as libc::c_short;
                LCK_Free(lptr);
                lptr = (*plptr).fwd_link;
            }
        } else {
            plptr = lptr;
            lptr = (*lptr).fwd_link;
        }
    }
    SemOp(1 as libc::c_int, (*systab).maxjob as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Old(
    mut count: libc::c_int,
    mut list: *mut cstring,
    mut to: libc::c_int,
) -> libc::c_short {
    LCK_Remove(0 as libc::c_int);
    if (*partab.jobtab).trap
        & ((1 as libc::c_uint) << 2 as libc::c_int
            | (1 as libc::c_uint) << 3 as libc::c_int
            | (1 as libc::c_uint) << 15 as libc::c_int
            | (1 as libc::c_uint) << 17 as libc::c_int) != 0
    {
        return -(51 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if count < 1 as libc::c_int {
        return 0 as libc::c_int as libc::c_short;
    }
    return LCK_Add(count, list, to);
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Add(
    mut p_count: libc::c_int,
    mut list: *mut cstring,
    mut p_to: libc::c_int,
) -> libc::c_short {
    let mut current: *mut cstring = 0 as *mut cstring;
    let mut tempc: *mut cstring = 0 as *mut cstring;
    let mut removedone: libc::c_int = 0;
    let mut posr: libc::c_int = 0;
    let mut toremove: libc::c_int = 0;
    let mut plptr: *mut locktab = 0 as *mut locktab;
    let mut nlptr: *mut locktab = 0 as *mut locktab;
    let mut i: libc::c_int = 0;
    let mut reqd: libc::c_int = 0;
    let mut ctx: lck_add = LCK_ADD {
        count: 0,
        to: 0,
        done: 0,
        tryagain: 0,
        currtime: 0,
        strttime: 0,
        x: 0,
        lptr: 0 as *mut locktab,
    };
    let mut pctx: *mut lck_add = 0 as *mut lck_add;
    pctx = &mut ctx;
    (*pctx).count = p_count;
    (*pctx).to = p_to;
    (*pctx).done = 0 as libc::c_int;
    (*pctx).tryagain = 1 as libc::c_int;
    (*pctx).strttime = current_time(1 as libc::c_int as libc::c_short);
    while (*pctx).tryagain != 0 {
        let mut size: libc::c_int = 0 as libc::c_int;
        let mut pos: libc::c_int = 0 as libc::c_int;
        (*pctx).tryagain = 0 as libc::c_int;
        if (*pctx).to > -(1 as libc::c_int) {
            (*partab.jobtab).test = 1 as libc::c_int as u_char;
        }
        (*pctx).done = 0 as libc::c_int;
        (*pctx)
            .x = SemOp(
            1 as libc::c_int,
            ((*systab).maxjob).wrapping_neg() as libc::c_int,
        );
        if ((*pctx).x as libc::c_int) < 0 as libc::c_int {
            return (*pctx).x;
        }
        while (*pctx).done < (*pctx).count && (*pctx).tryagain == 0 as libc::c_int {
            current = &mut *(list as *mut u_char).offset(pos as isize) as *mut u_char
                as *mut cstring;
            reqd = (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<*mut locktab>() as libc::c_ulong)
                .wrapping_add((*current).len as libc::c_ulong) as libc::c_int;
            (*pctx).lptr = (*systab).lockhead;
            plptr = 0 as *mut locktab;
            if ((*pctx).lptr).is_null() {
                (*(*systab).lockfree).fwd_link = 0 as *mut LOCKTAB;
                nlptr = LCK_Insert(reqd);
                if nlptr.is_null() {
                    (*pctx).tryagain = 1 as libc::c_int;
                    toremove = (*pctx).done;
                    removedone = 0 as libc::c_int;
                    posr = 0 as libc::c_int;
                    size = 0 as libc::c_int;
                    while removedone < toremove {
                        LCK_Kill(
                            &mut *(list as *mut u_char).offset(posr as isize)
                                as *mut u_char as *mut cstring,
                        );
                        tempc = &mut *(list as *mut u_char).offset(posr as isize)
                            as *mut u_char as *mut cstring;
                        size = (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                            .wrapping_add((*tempc).len as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ) as libc::c_int;
                        if size & 1 as libc::c_int != 0 {
                            size += 1;
                            size;
                        }
                        posr += size;
                        removedone += 1;
                        removedone;
                    }
                    if failed(pctx) != 0 {
                        return -(51 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                    }
                } else {
                    (*nlptr)
                        .job = ((partab.jobtab).offset_from((*systab).jobtab)
                        as libc::c_long + 1 as libc::c_int as libc::c_long)
                        as libc::c_short;
                    (*nlptr).lock_count = 1 as libc::c_int as libc::c_short;
                    (*nlptr).byte_count = (*current).len as libc::c_short;
                    memcpy(
                        &mut (*nlptr).vol as *mut u_char as *mut libc::c_void,
                        ((*current).buf).as_mut_ptr() as *const libc::c_void,
                        (*current).len as libc::c_ulong,
                    );
                    (*nlptr).fwd_link = (*systab).lockhead;
                    (*systab).lockhead = nlptr;
                }
            }
            while !((*pctx).lptr).is_null() {
                i = (*current).len as libc::c_int;
                if ((*(*pctx).lptr).byte_count as libc::c_int) < i {
                    i = (*(*pctx).lptr).byte_count as libc::c_int;
                }
                while !((*pctx).lptr).is_null()
                    && memcmp(
                        ((*current).buf).as_mut_ptr() as *const libc::c_void,
                        &mut (*(*pctx).lptr).vol as *mut u_char as *const libc::c_void,
                        i as libc::c_ulong,
                    ) > 0 as libc::c_int
                {
                    plptr = (*pctx).lptr;
                    (*pctx).lptr = (*(*pctx).lptr).fwd_link;
                    if ((*pctx).lptr).is_null() {
                        break;
                    }
                    i = (*current).len as libc::c_int;
                    if ((*(*pctx).lptr).byte_count as libc::c_int) < i {
                        i = (*(*pctx).lptr).byte_count as libc::c_int;
                    }
                }
                if !((*pctx).lptr).is_null()
                    && memcmp(
                        ((*current).buf).as_mut_ptr() as *const libc::c_void,
                        &mut (*(*pctx).lptr).vol as *mut u_char as *const libc::c_void,
                        i as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if (*(*pctx).lptr).job as libc::c_long
                        == (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    {
                        while !((*pctx).lptr).is_null()
                            && memcmp(
                                ((*current).buf).as_mut_ptr() as *const libc::c_void,
                                &mut (*(*pctx).lptr).vol as *mut u_char
                                    as *const libc::c_void,
                                i as libc::c_ulong,
                            ) == 0 as libc::c_int
                            && ((*(*pctx).lptr).byte_count as libc::c_int)
                                < (*current).len as libc::c_int
                        {
                            plptr = (*pctx).lptr;
                            (*pctx).lptr = (*(*pctx).lptr).fwd_link;
                            if ((*pctx).lptr).is_null() {
                                break;
                            }
                            i = (*current).len as libc::c_int;
                            if ((*(*pctx).lptr).byte_count as libc::c_int) < i {
                                i = (*(*pctx).lptr).byte_count as libc::c_int;
                            }
                        }
                        if !((*pctx).lptr).is_null() {
                            if memcmp(
                                ((*current).buf).as_mut_ptr() as *const libc::c_void,
                                &mut (*(*pctx).lptr).vol as *mut u_char
                                    as *const libc::c_void,
                                i as libc::c_ulong,
                            ) == 0 as libc::c_int
                            {
                                if (*(*pctx).lptr).byte_count as libc::c_int
                                    == (*current).len as libc::c_int
                                {
                                    (*(*pctx).lptr).lock_count += 1;
                                    (*(*pctx).lptr).lock_count;
                                    (*pctx).lptr = 0 as *mut locktab;
                                } else {
                                    nlptr = LCK_Insert(reqd);
                                    if nlptr.is_null() {
                                        (*pctx).tryagain = 1 as libc::c_int;
                                        toremove = (*pctx).done;
                                        removedone = 0 as libc::c_int;
                                        posr = 0 as libc::c_int;
                                        size = 0 as libc::c_int;
                                        while removedone < toremove {
                                            LCK_Kill(
                                                &mut *(list as *mut u_char).offset(posr as isize)
                                                    as *mut u_char as *mut cstring,
                                            );
                                            tempc = &mut *(list as *mut u_char).offset(posr as isize)
                                                as *mut u_char as *mut cstring;
                                            size = (::core::mem::size_of::<libc::c_short>()
                                                as libc::c_ulong)
                                                .wrapping_add((*tempc).len as libc::c_ulong)
                                                .wrapping_add(
                                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ) as libc::c_int;
                                            if size & 1 as libc::c_int != 0 {
                                                size += 1;
                                                size;
                                            }
                                            posr += size;
                                            removedone += 1;
                                            removedone;
                                        }
                                        if failed(pctx) != 0 {
                                            return -(51 as libc::c_int + 200 as libc::c_int)
                                                as libc::c_short;
                                        }
                                    } else {
                                        (*nlptr)
                                            .job = ((partab.jobtab).offset_from((*systab).jobtab)
                                            as libc::c_long + 1 as libc::c_int as libc::c_long)
                                            as libc::c_short;
                                        (*nlptr).lock_count = 1 as libc::c_int as libc::c_short;
                                        (*nlptr).byte_count = (*current).len as libc::c_short;
                                        memcpy(
                                            &mut (*nlptr).vol as *mut u_char as *mut libc::c_void,
                                            ((*current).buf).as_mut_ptr() as *const libc::c_void,
                                            (*current).len as libc::c_ulong,
                                        );
                                        if !plptr.is_null() {
                                            (*plptr).fwd_link = nlptr;
                                        } else {
                                            (*systab).lockhead = nlptr;
                                        }
                                        (*nlptr).fwd_link = (*pctx).lptr;
                                        (*pctx).lptr = 0 as *mut locktab;
                                    }
                                }
                            } else {
                                nlptr = LCK_Insert(reqd);
                                if nlptr.is_null() {
                                    (*pctx).tryagain = 1 as libc::c_int;
                                    toremove = (*pctx).done;
                                    removedone = 0 as libc::c_int;
                                    posr = 0 as libc::c_int;
                                    size = 0 as libc::c_int;
                                    while removedone < toremove {
                                        LCK_Kill(
                                            &mut *(list as *mut u_char).offset(posr as isize)
                                                as *mut u_char as *mut cstring,
                                        );
                                        tempc = &mut *(list as *mut u_char).offset(posr as isize)
                                            as *mut u_char as *mut cstring;
                                        size = (::core::mem::size_of::<libc::c_short>()
                                            as libc::c_ulong)
                                            .wrapping_add((*tempc).len as libc::c_ulong)
                                            .wrapping_add(
                                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ) as libc::c_int;
                                        if size & 1 as libc::c_int != 0 {
                                            size += 1;
                                            size;
                                        }
                                        posr += size;
                                        removedone += 1;
                                        removedone;
                                    }
                                    if failed(pctx) != 0 {
                                        return -(51 as libc::c_int + 200 as libc::c_int)
                                            as libc::c_short;
                                    }
                                } else {
                                    (*nlptr)
                                        .job = ((partab.jobtab).offset_from((*systab).jobtab)
                                        as libc::c_long + 1 as libc::c_int as libc::c_long)
                                        as libc::c_short;
                                    (*nlptr).lock_count = 1 as libc::c_int as libc::c_short;
                                    (*nlptr).byte_count = (*current).len as libc::c_short;
                                    memcpy(
                                        &mut (*nlptr).vol as *mut u_char as *mut libc::c_void,
                                        ((*current).buf).as_mut_ptr() as *const libc::c_void,
                                        (*current).len as libc::c_ulong,
                                    );
                                    (*plptr).fwd_link = nlptr;
                                    (*nlptr).fwd_link = (*pctx).lptr;
                                    (*pctx).lptr = 0 as *mut locktab;
                                }
                            }
                        } else {
                            nlptr = LCK_Insert(reqd);
                            if nlptr.is_null() {
                                (*pctx).tryagain = 1 as libc::c_int;
                                toremove = (*pctx).done;
                                removedone = 0 as libc::c_int;
                                posr = 0 as libc::c_int;
                                size = 0 as libc::c_int;
                                while removedone < toremove {
                                    LCK_Kill(
                                        &mut *(list as *mut u_char).offset(posr as isize)
                                            as *mut u_char as *mut cstring,
                                    );
                                    tempc = &mut *(list as *mut u_char).offset(posr as isize)
                                        as *mut u_char as *mut cstring;
                                    size = (::core::mem::size_of::<libc::c_short>()
                                        as libc::c_ulong)
                                        .wrapping_add((*tempc).len as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ) as libc::c_int;
                                    if size & 1 as libc::c_int != 0 {
                                        size += 1;
                                        size;
                                    }
                                    posr += size;
                                    removedone += 1;
                                    removedone;
                                }
                                if failed(pctx) != 0 {
                                    return -(51 as libc::c_int + 200 as libc::c_int)
                                        as libc::c_short;
                                }
                            } else {
                                (*nlptr)
                                    .job = ((partab.jobtab).offset_from((*systab).jobtab)
                                    as libc::c_long + 1 as libc::c_int as libc::c_long)
                                    as libc::c_short;
                                (*nlptr).lock_count = 1 as libc::c_int as libc::c_short;
                                (*nlptr).byte_count = (*current).len as libc::c_short;
                                memcpy(
                                    &mut (*nlptr).vol as *mut u_char as *mut libc::c_void,
                                    ((*current).buf).as_mut_ptr() as *const libc::c_void,
                                    (*current).len as libc::c_ulong,
                                );
                                (*plptr).fwd_link = nlptr;
                                (*nlptr).fwd_link = (*pctx).lptr;
                                (*pctx).lptr = 0 as *mut locktab;
                            }
                        }
                    } else {
                        (*pctx).tryagain = 1 as libc::c_int;
                        toremove = (*pctx).done;
                        removedone = 0 as libc::c_int;
                        posr = 0 as libc::c_int;
                        size = 0 as libc::c_int;
                        while removedone < toremove {
                            LCK_Kill(
                                &mut *(list as *mut u_char).offset(posr as isize)
                                    as *mut u_char as *mut cstring,
                            );
                            tempc = &mut *(list as *mut u_char).offset(posr as isize)
                                as *mut u_char as *mut cstring;
                            size = (::core::mem::size_of::<libc::c_short>()
                                as libc::c_ulong)
                                .wrapping_add((*tempc).len as libc::c_ulong)
                                .wrapping_add(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ) as libc::c_int;
                            if size & 1 as libc::c_int != 0 {
                                size += 1;
                                size;
                            }
                            posr += size;
                            removedone += 1;
                            removedone;
                        }
                        if failed(pctx) != 0 {
                            return -(51 as libc::c_int + 200 as libc::c_int)
                                as libc::c_short;
                        }
                    }
                } else if !((*pctx).lptr).is_null()
                    && memcmp(
                        ((*current).buf).as_mut_ptr() as *const libc::c_void,
                        &mut (*(*pctx).lptr).vol as *mut u_char as *const libc::c_void,
                        i as libc::c_ulong,
                    ) < 0 as libc::c_int
                {
                    nlptr = LCK_Insert(reqd);
                    if nlptr.is_null() {
                        (*pctx).tryagain = 1 as libc::c_int;
                        toremove = (*pctx).done;
                        removedone = 0 as libc::c_int;
                        posr = 0 as libc::c_int;
                        size = 0 as libc::c_int;
                        while removedone < toremove {
                            LCK_Kill(
                                &mut *(list as *mut u_char).offset(posr as isize)
                                    as *mut u_char as *mut cstring,
                            );
                            tempc = &mut *(list as *mut u_char).offset(posr as isize)
                                as *mut u_char as *mut cstring;
                            size = (::core::mem::size_of::<libc::c_short>()
                                as libc::c_ulong)
                                .wrapping_add((*tempc).len as libc::c_ulong)
                                .wrapping_add(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ) as libc::c_int;
                            if size & 1 as libc::c_int != 0 {
                                size += 1;
                                size;
                            }
                            posr += size;
                            removedone += 1;
                            removedone;
                        }
                        if failed(pctx) != 0 {
                            return -(51 as libc::c_int + 200 as libc::c_int)
                                as libc::c_short;
                        }
                    } else {
                        (*nlptr)
                            .job = ((partab.jobtab).offset_from((*systab).jobtab)
                            as libc::c_long + 1 as libc::c_int as libc::c_long)
                            as libc::c_short;
                        (*nlptr).lock_count = 1 as libc::c_int as libc::c_short;
                        (*nlptr).byte_count = (*current).len as libc::c_short;
                        memcpy(
                            &mut (*nlptr).vol as *mut u_char as *mut libc::c_void,
                            ((*current).buf).as_mut_ptr() as *const libc::c_void,
                            (*current).len as libc::c_ulong,
                        );
                        (*nlptr).fwd_link = (*pctx).lptr;
                        (*pctx).lptr = 0 as *mut locktab;
                        if plptr.is_null() {
                            (*systab).lockhead = nlptr;
                        } else {
                            (*plptr).fwd_link = nlptr;
                        }
                    }
                } else if ((*pctx).lptr).is_null() {
                    nlptr = LCK_Insert(reqd);
                    if nlptr.is_null() {
                        (*pctx).tryagain = 1 as libc::c_int;
                        toremove = (*pctx).done;
                        removedone = 0 as libc::c_int;
                        posr = 0 as libc::c_int;
                        size = 0 as libc::c_int;
                        while removedone < toremove {
                            LCK_Kill(
                                &mut *(list as *mut u_char).offset(posr as isize)
                                    as *mut u_char as *mut cstring,
                            );
                            tempc = &mut *(list as *mut u_char).offset(posr as isize)
                                as *mut u_char as *mut cstring;
                            size = (::core::mem::size_of::<libc::c_short>()
                                as libc::c_ulong)
                                .wrapping_add((*tempc).len as libc::c_ulong)
                                .wrapping_add(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ) as libc::c_int;
                            if size & 1 as libc::c_int != 0 {
                                size += 1;
                                size;
                            }
                            posr += size;
                            removedone += 1;
                            removedone;
                        }
                        if failed(pctx) != 0 {
                            return -(51 as libc::c_int + 200 as libc::c_int)
                                as libc::c_short;
                        }
                    } else {
                        (*nlptr)
                            .job = ((partab.jobtab).offset_from((*systab).jobtab)
                            as libc::c_long + 1 as libc::c_int as libc::c_long)
                            as libc::c_short;
                        (*nlptr).lock_count = 1 as libc::c_int as libc::c_short;
                        (*nlptr).byte_count = (*current).len as libc::c_short;
                        memcpy(
                            &mut (*nlptr).vol as *mut u_char as *mut libc::c_void,
                            ((*current).buf).as_mut_ptr() as *const libc::c_void,
                            (*current).len as libc::c_ulong,
                        );
                        (*nlptr).fwd_link = (*pctx).lptr;
                        (*pctx).lptr = 0 as *mut locktab;
                        if plptr.is_null() {
                            (*systab).lockhead = nlptr;
                        } else {
                            (*plptr).fwd_link = nlptr;
                        }
                    }
                }
            }
            if (*pctx).tryagain == 0 as libc::c_int {
                size = (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                    .wrapping_add((*current).len as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ) as libc::c_int;
                if size & 1 as libc::c_int != 0 {
                    size += 1 as libc::c_int;
                }
                pos += size;
                (*pctx).done += 1;
                (*pctx).done;
            }
        }
    }
    (*pctx).x = SemOp(1 as libc::c_int, (*systab).maxjob as libc::c_int);
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn LCK_Sub(
    mut count: libc::c_int,
    mut list: *mut cstring,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut current: *mut cstring = 0 as *mut cstring;
    let mut lptr: *mut locktab = 0 as *mut locktab;
    let mut x: libc::c_short = 0;
    x = SemOp(1 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int);
    if (x as libc::c_int) < 0 as libc::c_int {
        return x;
    }
    while done < count {
        let mut size: u_int = 0;
        current = &mut *(list as *mut u_char).offset(pos as isize) as *mut u_char
            as *mut cstring;
        lptr = (*systab).lockhead;
        while !lptr.is_null() {
            i = (*current).len as libc::c_int;
            if ((*lptr).byte_count as libc::c_int) < i {
                i = (*lptr).byte_count as libc::c_int;
            }
            while !lptr.is_null()
                && memcmp(
                    ((*current).buf).as_mut_ptr() as *const libc::c_void,
                    &mut (*lptr).vol as *mut u_char as *const libc::c_void,
                    i as libc::c_ulong,
                ) > 0 as libc::c_int
            {
                lptr = (*lptr).fwd_link;
                if lptr.is_null() {
                    break;
                }
                i = (*current).len as libc::c_int;
                if ((*lptr).byte_count as libc::c_int) < i {
                    i = (*lptr).byte_count as libc::c_int;
                }
            }
            if !lptr.is_null()
                && memcmp(
                    ((*current).buf).as_mut_ptr() as *const libc::c_void,
                    &mut (*lptr).vol as *mut u_char as *const libc::c_void,
                    i as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                if (*lptr).job as libc::c_long
                    == (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                {
                    while !lptr.is_null()
                        && memcmp(
                            ((*current).buf).as_mut_ptr() as *const libc::c_void,
                            &mut (*lptr).vol as *mut u_char as *const libc::c_void,
                            i as libc::c_ulong,
                        ) == 0 as libc::c_int
                        && ((*lptr).byte_count as libc::c_int)
                            < (*current).len as libc::c_int
                    {
                        lptr = (*lptr).fwd_link;
                        if lptr.is_null() {
                            break;
                        }
                        i = (*current).len as libc::c_int;
                        if ((*lptr).byte_count as libc::c_int) < i {
                            i = (*lptr).byte_count as libc::c_int;
                        }
                    }
                    if !lptr.is_null() {
                        if memcmp(
                            ((*current).buf).as_mut_ptr() as *const libc::c_void,
                            &mut (*lptr).vol as *mut u_char as *const libc::c_void,
                            i as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            if (*lptr).byte_count as libc::c_int
                                == (*current).len as libc::c_int
                            {
                                (*lptr).lock_count -= 1;
                                (*lptr).lock_count;
                                if (*lptr).lock_count as libc::c_int <= 0 as libc::c_int {
                                    LCK_Kill(current);
                                    lptr = (*lptr).fwd_link;
                                } else {
                                    lptr = (*lptr).fwd_link;
                                }
                            } else {
                                lptr = (*lptr).fwd_link;
                            }
                        } else {
                            lptr = (*lptr).fwd_link;
                        }
                    }
                } else {
                    lptr = (*lptr).fwd_link;
                }
            } else if !lptr.is_null() {
                lptr = (*lptr).fwd_link;
            }
        }
        size = (::core::mem::size_of::<u_short>() as libc::c_ulong)
            .wrapping_add((*current).len as libc::c_ulong) as u_int;
        if size & 1 as libc::c_int as u_int != 0 {
            size = size.wrapping_add(1 as libc::c_int as u_int);
        }
        pos = (pos as u_int).wrapping_add(size) as libc::c_int as libc::c_int;
        done += 1;
        done;
    }
    SemOp(1 as libc::c_int, (*systab).maxjob as libc::c_int);
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Dump_lt() {
    let mut lptr: *mut locktab = 0 as *mut locktab;
    let mut x: libc::c_short = 0;
    let mut keystr: [u_char; 260] = [0; 260];
    let mut workstr: [u_char; 260] = [0; 260];
    x = SemOp(1 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int);
    if (x as libc::c_int) < 0 as libc::c_int {
        return;
    }
    lptr = (*systab).lockstart as *mut locktab;
    printf(
        b"Dump of Lockspace starting at %p\r\n\r\n\0" as *const u8
            as *const libc::c_char,
        lptr,
    );
    printf(
        b"Lock Head starts at %p\r\n\0" as *const u8 as *const libc::c_char,
        (*systab).lockhead,
    );
    printf(
        b"Lock Free starts at %p\r\n\0" as *const u8 as *const libc::c_char,
        (*systab).lockfree,
    );
    printf(
        b"      Lock_Ptr       Fwd_Link    Size    Job Lock_Cnt Byte_Cnt  VOL  UCI  Var(Key)\r\n\0"
            as *const u8 as *const libc::c_char,
    );
    while !lptr.is_null() {
        keystr[0 as libc::c_int as usize] = '\0' as i32 as u_char;
        if (*lptr).byte_count as libc::c_int > 32 as libc::c_int + 2 as libc::c_int {
            workstr[0 as libc::c_int
                as usize] = ((*lptr).byte_count as libc::c_int
                - (32 as libc::c_int + 2 as libc::c_int)) as u_char;
            memcpy(
                &mut *workstr.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_void,
                ((*lptr).key).as_mut_ptr() as *const libc::c_void,
                workstr[0 as libc::c_int as usize] as libc::c_ulong,
            );
            x = UTIL_String_Key(
                workstr.as_mut_ptr(),
                keystr.as_mut_ptr(),
                63 as libc::c_int,
            );
            if (x as libc::c_int) < 0 as libc::c_int {
                sprintf(
                    keystr.as_mut_ptr() as *mut libc::c_char,
                    b" ERROR: %d\0" as *const u8 as *const libc::c_char,
                    x as libc::c_int,
                );
            }
        }
        if (*lptr).job as libc::c_int == -(1 as libc::c_int) {
            printf(
                b"%10p %14p %7d %6d %8d %8d %4d %4d  %.32s%s\r\n\0" as *const u8
                    as *const libc::c_char,
                lptr,
                (*lptr).fwd_link,
                (*lptr).size,
                (*lptr).job as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            printf(
                b"%10p %14p %7d %6d %8d %8d %4d %4d  %.32s%s\r\n\0" as *const u8
                    as *const libc::c_char,
                lptr,
                (*lptr).fwd_link,
                (*lptr).size,
                (*lptr).job as libc::c_int,
                (*lptr).lock_count as libc::c_int,
                (*lptr).byte_count as libc::c_int,
                (*lptr).vol as libc::c_int,
                (*lptr).uci as libc::c_int,
                ((*lptr).name.var_cu).as_mut_ptr(),
                keystr.as_mut_ptr(),
            );
        }
        lptr = (lptr as *mut u_char).offset((*lptr).size as isize) as *mut locktab;
        if lptr as *mut u_char
            >= ((*systab).lockstart as *mut u_char).offset((*systab).locksize as isize)
        {
            break;
        }
    }
    SemOp(1 as libc::c_int, (*systab).maxjob as libc::c_int);
}
