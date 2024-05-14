#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type GBD;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    static mut addstk: [*mut u_char; 0];
    static mut rsmpc: *mut u_char;
    fn UTIL_Key_Build(src: *mut cstring, dest: *mut u_char) -> libc::c_short;
    fn UTIL_Key_Last(var: *mut mvar) -> libc::c_int;
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
#[inline]
unsafe extern "C" fn var_empty(mut var: var_u) -> u_int {
    if var.var_q == 0 as libc::c_int as u_int64 {
        return 1 as libc::c_int as u_int
    } else {
        return 0 as libc::c_int as u_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn getvol(mut vol: *mut cstring) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut s: u_short = 0;
    s = (*vol).len;
    if (s as libc::c_int) < 32 as libc::c_int {
        s = s.wrapping_add(1);
        s;
    }
    i = 0 as libc::c_int;
    while i < 1 as libc::c_int {
        if !((*systab).vol[i as usize]).is_null() {
            if !((*(*systab).vol[i as usize]).vollab).is_null() {
                if !(memcmp(
                    ((*vol).buf).as_mut_ptr() as *const libc::c_void,
                    &mut *((*(**((*systab).vol).as_mut_ptr().offset(i as isize)).vollab)
                        .volnam
                        .var_cu)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut u_char
                        as *const libc::c_void,
                    s as libc::c_ulong,
                ) != 0 as libc::c_int)
                {
                    return (i + 1 as libc::c_int) as libc::c_short;
                }
            }
        }
        i += 1;
        i;
    }
    return -(26 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn getuci(
    mut uci: *mut cstring,
    mut vol: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut s: u_short = 0;
    s = (*uci).len;
    if (s as libc::c_int) < 32 as libc::c_int {
        s = s.wrapping_add(1);
        s;
    }
    if vol == 0 as libc::c_int {
        vol = (*partab.jobtab).vol as libc::c_int;
    }
    vol -= 1;
    vol;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if memcmp(
            ((*uci).buf).as_mut_ptr() as *const libc::c_void,
            &mut *((*((*(**((*systab).vol).as_mut_ptr().offset(vol as isize)).vollab)
                .uci)
                .as_mut_ptr()
                .offset(i as isize))
                .name
                .var_cu)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut u_char as *const libc::c_void,
            s as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return (i + 1 as libc::c_int) as libc::c_short;
        }
        i += 1;
        i;
    }
    return -(26 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn buildmvar(
    mut var: *mut mvar,
    mut nul_ok: libc::c_int,
    mut asp: libc::c_int,
) -> libc::c_short {
    let mut type_0: u_char = 0;
    let mut subs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ptr: *mut cstring = 0 as *mut cstring;
    let mut s: libc::c_short = 0;
    let mut vt: *mut var_u = 0 as *mut var_u;
    let mut p: *mut rbd = 0 as *mut rbd;
    let mut ind: *mut mvar = 0 as *mut mvar;
    let fresh0 = rsmpc;
    rsmpc = rsmpc.offset(1);
    type_0 = *fresh0;
    if (type_0 as libc::c_int) < 252 as libc::c_int {
        subs = type_0 as libc::c_int & 63 as libc::c_int;
        type_0 = (type_0 as libc::c_int & !(63 as libc::c_int)) as u_char;
    } else {
        let fresh1 = rsmpc;
        rsmpc = rsmpc.offset(1);
        subs = *fresh1 as libc::c_int;
    }
    (*var).volset = 0 as libc::c_int as u_char;
    (*var)
        .uci = (if (type_0 as libc::c_int) < 128 as libc::c_int {
        255 as libc::c_int
    } else {
        0 as libc::c_int
    }) as u_char;
    (*var).slen = 0 as libc::c_int as u_char;
    if type_0 as libc::c_int == 252 as libc::c_int {
        if var_empty((*partab.jobtab).last_ref.name) != 0 {
            return -(1 as libc::c_int) as libc::c_short;
        }
        i = UTIL_Key_Last(&mut (*partab.jobtab).last_ref);
        if i < 0 as libc::c_int {
            return -(1 as libc::c_int) as libc::c_short;
        }
        memcpy(
            var as *mut libc::c_void,
            &mut (*partab.jobtab).last_ref as *mut mvar as *const libc::c_void,
            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                .wrapping_add(i as libc::c_ulong),
        );
        (*var).slen = i as u_char;
    } else if type_0 as libc::c_int == 255 as libc::c_int {
        ind = *addstk.as_mut_ptr().offset((asp - subs - 1 as libc::c_int) as isize)
            as *mut mvar;
        memmove(
            var as *mut libc::c_void,
            ind as *const libc::c_void,
            ((*ind).slen as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<var_u>() as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong),
        );
    } else if type_0 as libc::c_int & 64 as libc::c_int != 0
        && (type_0 as libc::c_int) < 128 as libc::c_int
    {
        let fresh2 = rsmpc;
        rsmpc = rsmpc.offset(1);
        i = *fresh2 as libc::c_int;
        if i < 255 as libc::c_int {
            (*var).volset = (i + 1 as libc::c_int) as u_char;
            let mut var_i: u_int = 0 as libc::c_int as u_int;
            while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                (*var).name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
                var_i = var_i.wrapping_add(1);
                var_i;
            }
        } else {
            p = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].routine
                as *mut rbd;
            vt = (p as *mut u_char).offset((*p).var_tbl as libc::c_int as isize)
                as *mut var_u;
            let mut var_i_0: u_int = 0 as libc::c_int as u_int;
            while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                (*var)
                    .name
                    .var_qu[var_i_0
                    as usize] = (*vt.offset(i as isize)).var_qu[var_i_0 as usize];
                var_i_0 = var_i_0.wrapping_add(1);
                var_i_0;
            }
        }
    } else {
        memmove(
            &mut (*var).name as *mut var_u as *mut libc::c_void,
            rsmpc as *const libc::c_void,
            32 as libc::c_int as libc::c_ulong,
        );
        rsmpc = rsmpc.offset(32 as libc::c_int as isize);
    }
    i = 0 as libc::c_int;
    while i < subs {
        ptr = *addstk.as_mut_ptr().offset((asp - subs + i) as isize) as *mut cstring;
        if (*ptr).len as libc::c_int == 0 as libc::c_int
            && (nul_ok == 0 || i != subs - 1 as libc::c_int)
        {
            return -(16 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        s = UTIL_Key_Build(
            ptr,
            &mut *((*var).key).as_mut_ptr().offset((*var).slen as isize),
        );
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        if s as libc::c_int + (*var).slen as libc::c_int > 255 as libc::c_int {
            return -(2 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        (*var).slen = (s as libc::c_int + (*var).slen as libc::c_int) as u_char;
        i += 1;
        i;
    }
    if type_0 as libc::c_int == 254 as libc::c_int {
        ptr = *addstk.as_mut_ptr().offset((asp - subs - 1 as libc::c_int) as isize)
            as *mut cstring;
        s = getvol(ptr);
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        (*var).volset = s as u_char;
    }
    if type_0 as libc::c_int == 253 as libc::c_int
        || type_0 as libc::c_int == 254 as libc::c_int
    {
        ptr = *addstk
            .as_mut_ptr()
            .offset(
                (asp - subs - 1 as libc::c_int
                    - (type_0 as libc::c_int == 254 as libc::c_int) as libc::c_int)
                    as isize,
            ) as *mut cstring;
        s = getuci(ptr, (*var).volset as libc::c_int);
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        (*var).uci = s as u_char;
    }
    if type_0 as libc::c_int == 255 as libc::c_int {
        asp -= 1;
        asp;
    }
    return (asp - subs - (type_0 as libc::c_int == 253 as libc::c_int) as libc::c_int
        - (type_0 as libc::c_int == 254 as libc::c_int) as libc::c_int
            * 2 as libc::c_int) as libc::c_short;
}
