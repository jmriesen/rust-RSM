#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    pub type GBD;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __error() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ctime(_: *const time_t) -> *mut libc::c_char;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn DB_GetLen(var: *mut mvar, lock: libc::c_int, buf: *mut u_char) -> libc::c_int;
    fn current_time(local: libc::c_short) -> time_t;
    fn UTIL_Key_Build(src: *mut cstring, dest: *mut u_char) -> libc::c_short;
    fn panic(msg: *mut libc::c_char);
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
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
pub type off_t = __darwin_off_t;
pub type time_t = __darwin_time_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
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
#[no_mangle]
pub unsafe extern "C" fn Routine_Init(mut vol: libc::c_int) {
    let mut rou: *mut rbd = 0 as *mut rbd;
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < 1023 as libc::c_int as u_int {
        (*(*systab).vol[vol as usize]).rbd_hash[i as usize] = 0 as *mut RBD;
        i = i.wrapping_add(1);
        i;
    }
    rou = (*(*systab).vol[vol as usize]).rbd_head as *mut rbd;
    (*(*systab).vol[vol as usize]).rbd_hash[1023 as libc::c_int as usize] = rou;
    i = ((*(*systab).vol[vol as usize]).rbd_end as *mut libc::c_char)
        .offset_from((*(*systab).vol[vol as usize]).rbd_head as *mut libc::c_char)
        as libc::c_long as u_int;
    (*rou).fwd_link = 0 as *mut RBD;
    (*rou).chunk_size = i;
    (*rou).attached = 0 as libc::c_int as u_int;
    (*rou).last_access = 0 as libc::c_int as time_t;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        (*rou).rnam.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    (*rou).uci = 0 as libc::c_int as u_char;
    (*rou).vol = 0 as libc::c_int as u_char;
    (*rou).rou_size = 0 as libc::c_int as u_short;
}
#[no_mangle]
pub unsafe extern "C" fn Dump_rbd() {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_short = 0;
    let mut p: *mut rbd = 0 as *mut rbd;
    let mut tmp: [libc::c_char; 34] = [0; 34];
    let mut t: time_t = 0;
    s = SemOp(3 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int);
    if (s as libc::c_int) < 0 as libc::c_int {
        return;
    }
    p = (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_head as *mut rbd;
    t = current_time(0 as libc::c_int as libc::c_short);
    printf(
        b"Dump of all Routine Buffer Descriptors on %s\r\n\0" as *const u8
            as *const libc::c_char,
        ctime(&mut t),
    );
    printf(
        b"Free at %10p\r\n\0" as *const u8 as *const libc::c_char,
        (*(*systab)
            .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
            .rbd_hash[1023 as libc::c_int as usize],
    );
    printf(
        b"       Address       Fwd_Link Chunk_Size Attach Last_Access VOL UCI Routine_Size Routine_Name\r\n\0"
            as *const u8 as *const libc::c_char,
    );
    tmp[32 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    loop {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            tmp[i as usize] = ' ' as i32 as libc::c_char;
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if (*p).rnam.var_cu[i as usize] as libc::c_int == 0 as libc::c_int {
                break;
            }
            tmp[i as usize] = (*p).rnam.var_cu[i as usize] as libc::c_char;
            i += 1;
            i;
        }
        printf(
            b"%10p %14p %10u %6u %11lld %3d %3d %12d %s\r\n\0" as *const u8
                as *const libc::c_char,
            p,
            (*p).fwd_link,
            (*p).chunk_size,
            (*p).attached,
            (*p).last_access as libc::c_longlong,
            (*p).vol as libc::c_int,
            (*p).uci as libc::c_int,
            (*p).rou_size as libc::c_int,
            tmp.as_mut_ptr(),
        );
        p = (p as *mut u_char).offset((*p).chunk_size as isize) as *mut rbd;
        if p
            >= (*(*systab)
                .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
                .rbd_end as *mut rbd
        {
            break;
        }
    }
    SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Routine_Hash(mut routine: var_u) -> libc::c_int {
    let mut hash: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: [[libc::c_int; 8]; 4] = [
        [
            3 as libc::c_int,
            5 as libc::c_int,
            7 as libc::c_int,
            11 as libc::c_int,
            13 as libc::c_int,
            17 as libc::c_int,
            19 as libc::c_int,
            23 as libc::c_int,
        ],
        [
            29 as libc::c_int,
            31 as libc::c_int,
            37 as libc::c_int,
            41 as libc::c_int,
            43 as libc::c_int,
            47 as libc::c_int,
            53 as libc::c_int,
            59 as libc::c_int,
        ],
        [
            61 as libc::c_int,
            67 as libc::c_int,
            71 as libc::c_int,
            73 as libc::c_int,
            79 as libc::c_int,
            83 as libc::c_int,
            89 as libc::c_int,
            97 as libc::c_int,
        ],
        [
            101 as libc::c_int,
            103 as libc::c_int,
            107 as libc::c_int,
            109 as libc::c_int,
            113 as libc::c_int,
            127 as libc::c_int,
            131 as libc::c_int,
            137 as libc::c_int,
        ],
    ];
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int / 8 as libc::c_int {
        if routine.var_qu[i as usize] == 0 as libc::c_int as u_int64 {
            break;
        }
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            if routine.var_qu[i as usize] == 0 as libc::c_int as u_int64 {
                break;
            }
            hash = ((routine.var_qu[i as usize] & 0xff as libc::c_int as u_int64)
                * p[i as usize][j as usize] as u_int64)
                .wrapping_add(hash as u_int64) as libc::c_int;
            routine.var_qu[i as usize] = routine.var_qu[i as usize] >> 8 as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return hash % 1023 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Routine_Combine(mut pointer: *mut rbd) {
    let mut ptr: *mut rbd = 0 as *mut rbd;
    let mut p: *mut rbd = 0 as *mut rbd;
    ptr = (pointer as *mut u_char).offset((*pointer).chunk_size as isize) as *mut rbd;
    p = (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_hash[1023 as libc::c_int as usize];
    if p == ptr {
        (*(*systab)
            .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
            .rbd_hash[1023 as libc::c_int as usize] = (*ptr).fwd_link;
    } else {
        while (*p).fwd_link != ptr {
            p = (*p).fwd_link;
        }
        (*p).fwd_link = (*ptr).fwd_link;
    }
    (*pointer).chunk_size = ((*pointer).chunk_size).wrapping_add((*ptr).chunk_size);
}
#[no_mangle]
pub unsafe extern "C" fn Routine_Free(mut pointer: *mut rbd) {
    let mut ptr: *mut rbd = 0 as *mut rbd;
    let mut p: *mut rbd = 0 as *mut rbd;
    let mut hash: libc::c_int = 0;
    (*pointer).rou_size = 0 as libc::c_int as u_short;
    hash = Routine_Hash((*pointer).rnam);
    ptr = (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_hash[hash as usize];
    if ptr == pointer {
        (*(*systab)
            .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
            .rbd_hash[hash as usize] = (*pointer).fwd_link;
    } else {
        loop {
            if ptr.is_null() {
                ptr = (*(*systab)
                    .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int)
                    as usize])
                    .rbd_hash[1023 as libc::c_int as usize];
                if pointer == ptr {
                    (*(*systab)
                        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int)
                        as usize])
                        .rbd_hash[1023 as libc::c_int as usize] = (*pointer).fwd_link;
                } else {
                    while !ptr.is_null() {
                        if (*ptr).fwd_link == pointer {
                            (*ptr).fwd_link = (*pointer).fwd_link;
                            break;
                        } else {
                            ptr = (*ptr).fwd_link;
                        }
                    }
                }
                break;
            } else if (*ptr).fwd_link == pointer {
                (*ptr).fwd_link = (*pointer).fwd_link;
                break;
            } else {
                ptr = (*ptr).fwd_link;
            }
        }
    }
    (*pointer)
        .fwd_link = (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_hash[1023 as libc::c_int as usize];
    (*(*systab).vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_hash[1023 as libc::c_int as usize] = pointer;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        (*pointer).rnam.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    (*pointer).uci = 0 as libc::c_int as u_char;
    (*pointer).vol = 0 as libc::c_int as u_char;
    (*pointer).last_access = 0 as libc::c_int as time_t;
    loop {
        ptr = (pointer as *mut u_char).offset((*pointer).chunk_size as isize)
            as *mut rbd;
        if ptr
            >= (*(*systab)
                .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
                .rbd_end as *mut rbd
        {
            break;
        }
        if !((*ptr).rou_size as libc::c_int == 0 as libc::c_int) {
            break;
        }
        Routine_Combine(pointer);
    }
    loop {
        ptr = (*(*systab)
            .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
            .rbd_head as *mut rbd;
        if ptr == pointer {
            return;
        }
        loop {
            p = (ptr as *mut u_char).offset((*ptr).chunk_size as isize) as *mut rbd;
            if p == pointer {
                break;
            }
            ptr = p;
        }
        if (*ptr).rou_size as libc::c_int == 0 as libc::c_int {
            pointer = ptr;
            Routine_Combine(pointer);
        } else {
            return
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Routine_Collect(mut off: time_t) {
    let mut ptr: *mut rbd = 0 as *mut rbd;
    off = current_time(1 as libc::c_int as libc::c_short) - off;
    ptr = (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_head as *mut rbd;
    loop {
        if (*ptr).attached < 1 as libc::c_int as u_int && (*ptr).last_access < off
            && (*ptr).rou_size as libc::c_int > 0 as libc::c_int
        {
            Routine_Free(ptr);
            ptr = (*(*systab)
                .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
                .rbd_head as *mut rbd;
        }
        ptr = (ptr as *mut u_char).offset((*ptr).chunk_size as isize) as *mut rbd;
        if ptr
            >= (*(*systab)
                .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
                .rbd_end as *mut rbd
        {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Routine_Find(mut size: u_int) -> *mut rbd {
    let mut ptr: *mut rbd = 0 as *mut rbd;
    let mut p: *mut rbd = 0 as *mut rbd;
    ptr = (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_hash[1023 as libc::c_int as usize];
    while !ptr.is_null() {
        if (*ptr).chunk_size >= size {
            break;
        }
        ptr = (*ptr).fwd_link;
    }
    if ptr.is_null() {
        Routine_Collect((20 as libc::c_int * 60 as libc::c_int) as time_t);
        ptr = (*(*systab)
            .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
            .rbd_hash[1023 as libc::c_int as usize];
        while !ptr.is_null() {
            if (*ptr).chunk_size >= size {
                break;
            }
            ptr = (*ptr).fwd_link;
        }
        if ptr.is_null() {
            Routine_Collect(0 as libc::c_int as time_t);
            ptr = (*(*systab)
                .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
                .rbd_hash[1023 as libc::c_int as usize];
            while !ptr.is_null() {
                if (*ptr).chunk_size >= size {
                    break;
                }
                ptr = (*ptr).fwd_link;
            }
            if ptr.is_null() {
                return 0 as *mut rbd;
            }
        }
    }
    if size.wrapping_add(1024 as libc::c_int as u_int) < (*ptr).chunk_size {
        p = (ptr as *mut u_char).offset(size as isize) as *mut rbd;
        (*p).fwd_link = (*ptr).fwd_link;
        (*p).chunk_size = ((*ptr).chunk_size).wrapping_sub(size);
        (*p).attached = 0 as libc::c_int as u_int;
        (*p).last_access = 0 as libc::c_int as time_t;
        let mut var_i: u_int = 0 as libc::c_int as u_int;
        while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            (*p).rnam.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
            var_i = var_i.wrapping_add(1);
            var_i;
        }
        (*p).uci = 0 as libc::c_int as u_char;
        (*p).vol = 0 as libc::c_int as u_char;
        (*p).rou_size = 0 as libc::c_int as u_short;
        (*ptr).fwd_link = p;
        (*ptr).chunk_size = size;
    }
    if (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_hash[1023 as libc::c_int as usize] == ptr
    {
        (*(*systab)
            .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
            .rbd_hash[1023 as libc::c_int as usize] = (*ptr).fwd_link;
    } else {
        p = (*(*systab)
            .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
            .rbd_hash[1023 as libc::c_int as usize];
        while (*p).fwd_link != ptr {
            p = (*p).fwd_link;
        }
        (*p).fwd_link = (*ptr).fwd_link;
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn Routine_Attach(mut routine: var_u) -> *mut RBD {
    let mut hash: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut size: u_int = 0;
    let mut s: libc::c_short = 0;
    let mut t: libc::c_int = 0;
    let mut ptr: *mut rbd = 0 as *mut rbd;
    let mut p: *mut rbd = 0 as *mut rbd;
    let mut tmp: [u_char; 36] = [0; 36];
    let mut cptr: *mut cstring = 0 as *mut cstring;
    let mut rouglob: mvar = MVAR {
        name: VAR_U { var_q: 0 },
        volset: 0,
        uci: 0,
        slen: 0,
        key: [0; 256],
    };
    let mut uci: u_char = 0;
    let mut vol: u_char = 0;
    hash = Routine_Hash(routine);
    s = SemOp(3 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int);
    if (s as libc::c_int) < 0 as libc::c_int {
        return 0 as *mut RBD;
    }
    p = (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_hash[hash as usize];
    ptr = p;
    uci = (*partab.jobtab).ruci;
    vol = (*partab.jobtab).rvol;
    if routine.var_cu[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
        vol = 1 as libc::c_int as u_char;
        uci = vol;
    }
    while !ptr.is_null() {
        if var_equal((*ptr).rnam, routine) != 0
            && (*ptr).uci as libc::c_int == uci as libc::c_int
            && (*ptr).vol as libc::c_int == vol as libc::c_int
        {
            (*ptr).attached = ((*ptr).attached).wrapping_add(1);
            (*ptr).attached;
            SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
            return ptr;
        }
        ptr = (*ptr).fwd_link;
    }
    SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        rouglob.name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    memcpy(
        (rouglob.name.var_cu).as_mut_ptr() as *mut libc::c_void,
        b"$ROUTINE\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    rouglob.volset = vol;
    rouglob.uci = uci;
    cptr = tmp.as_mut_ptr() as *mut cstring;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if routine.var_cu[i as usize] as libc::c_int == '\0' as i32 {
            break;
        }
        (*cptr).buf[i as usize] = routine.var_cu[i as usize];
        i += 1;
        i;
    }
    (*cptr).buf[i as usize] = '\0' as i32 as u_char;
    (*cptr).len = i as u_short;
    s = UTIL_Key_Build(cptr, (rouglob.key).as_mut_ptr());
    rouglob.slen = s as u_char;
    (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
    (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
    (*cptr).len = 1 as libc::c_int as u_short;
    s = UTIL_Key_Build(cptr, &mut *(rouglob.key).as_mut_ptr().offset(s as isize));
    rouglob.slen = (rouglob.slen as libc::c_int + s as libc::c_int) as u_char;
    t = DB_GetLen(&mut rouglob, 0 as libc::c_int, 0 as *mut u_char);
    if t < 1 as libc::c_int {
        return 0 as *mut RBD;
    }
    s = SemOp(3 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int);
    if (s as libc::c_int) < 0 as libc::c_int {
        return 0 as *mut RBD;
    }
    p = (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_hash[hash as usize];
    ptr = p;
    while !ptr.is_null() {
        if var_equal((*ptr).rnam, routine) != 0
            && (*ptr).uci as libc::c_int == uci as libc::c_int
            && (*ptr).vol as libc::c_int == vol as libc::c_int
        {
            (*ptr).attached = ((*ptr).attached).wrapping_add(1);
            (*ptr).attached;
            SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
            return ptr;
        }
        p = ptr;
        ptr = (*ptr).fwd_link;
    }
    t = DB_GetLen(&mut rouglob, 1 as libc::c_int, 0 as *mut u_char);
    if t < 1 as libc::c_int {
        DB_GetLen(&mut rouglob, -(1 as libc::c_int), 0 as *mut u_char);
        SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
        return 0 as *mut RBD;
    }
    size = (t as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<*mut rbd>() as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<u_int>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(::core::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<var_u>() as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<u_char>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong),
        )
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as u_int;
    if size & 7 as libc::c_int as u_int != 0 {
        size = (size & !(7 as libc::c_int) as u_int)
            .wrapping_add(8 as libc::c_int as u_int);
    }
    ptr = Routine_Find(size);
    if ptr.is_null() {
        SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
        DB_GetLen(&mut rouglob, -(1 as libc::c_int), 0 as *mut u_char);
        return -(1 as libc::c_int) as *mut rbd;
    }
    if p.is_null() {
        (*(*systab)
            .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
            .rbd_hash[hash as usize] = ptr;
    } else {
        (*p).fwd_link = ptr;
    }
    (*ptr).fwd_link = 0 as *mut RBD;
    (*ptr).attached = 1 as libc::c_int as u_int;
    (*ptr).last_access = current_time(1 as libc::c_int as libc::c_short);
    let mut var_i_0: u_int = 0 as libc::c_int as u_int;
    while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        (*ptr).rnam.var_qu[var_i_0 as usize] = routine.var_qu[var_i_0 as usize];
        var_i_0 = var_i_0.wrapping_add(1);
        var_i_0;
    }
    (*ptr).uci = uci;
    (*ptr).vol = vol;
    (*ptr).rou_size = t as u_short;
    t = DB_GetLen(
        &mut rouglob,
        -(1 as libc::c_int),
        &mut (*ptr).comp_ver as *mut u_short as *mut u_char,
    );
    if t != (*ptr).rou_size as libc::c_int {
        panic(
            b"routine load - size wrong\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (*ptr)
        .tag_tbl = ((*ptr).tag_tbl as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<*mut rbd>() as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<u_int>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(::core::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<var_u>() as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<u_char>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong),
        ) as u_short as u_short;
    (*ptr)
        .var_tbl = ((*ptr).var_tbl as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<*mut rbd>() as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<u_int>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(::core::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<var_u>() as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<u_char>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong),
        ) as u_short as u_short;
    (*ptr)
        .code = ((*ptr).code as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<*mut rbd>() as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<u_int>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(::core::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<var_u>() as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<u_char>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong),
        ) as u_short as u_short;
    if (*ptr).comp_ver as libc::c_int != 8 as libc::c_int {
        (*ptr).attached = ((*ptr).attached).wrapping_sub(1);
        (*ptr).attached;
        Routine_Free(ptr);
        SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
        return -(2 as libc::c_int) as *mut rbd;
    }
    SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn Routine_Detach(mut pointer: *mut rbd) {
    let mut s: libc::c_short = 0;
    while (SemOp(3 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int)
        as libc::c_int) < 0 as libc::c_int
    {}
    if (*pointer).attached > 0 as libc::c_int as u_int {
        (*pointer).attached = ((*pointer).attached).wrapping_sub(1);
        (*pointer).attached;
    }
    if (*pointer).uci as libc::c_int == 0 as libc::c_int
        && (*pointer).attached == 0 as libc::c_int as u_int
    {
        Routine_Free(pointer);
    }
    s = SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
    if (s as libc::c_int) < 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn Routine_Delete(mut routine: var_u, mut uci: libc::c_int) {
    let mut hash: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut rbd = 0 as *mut rbd;
    hash = Routine_Hash(routine);
    ptr = (*(*systab)
        .vol[((*partab.jobtab).rvol as libc::c_int - 1 as libc::c_int) as usize])
        .rbd_hash[hash as usize];
    while !ptr.is_null() {
        if var_equal((*ptr).rnam, routine) != 0 && (*ptr).uci as libc::c_int == uci {
            (*ptr).uci = 0 as libc::c_int as u_char;
            if (*ptr).attached == 0 as libc::c_int as u_int {
                Routine_Free(ptr);
            }
            break;
        } else {
            ptr = (*ptr).fwd_link;
        }
    }
}
