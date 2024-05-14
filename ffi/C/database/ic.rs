#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type RBD;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn lseek(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    static mut curr_lock: libc::c_int;
    static mut gbd_expired: libc::c_int;
    static mut volnum: libc::c_int;
    static mut blk: [*mut gbd; 12];
    static mut level: libc::c_int;
    static mut chunk: *mut cstring;
    static mut record: *mut cstring;
    static mut idx: *mut u_short;
    static mut iidx: *mut libc::c_int;
    static mut writing: libc::c_int;
    fn Get_block(blknum: u_int) -> libc::c_short;
    fn Align_record();
    fn SQ_Write(buf: *mut cstring) -> libc::c_int;
    fn SQ_WriteFormat(count: libc::c_int) -> libc::c_short;
    fn UTIL_Key_KeyCmp(
        key1: *mut u_char,
        key2: *mut u_char,
        kleng1: libc::c_int,
        kleng2: libc::c_int,
    ) -> libc::c_int;
    fn UTIL_strerror(err: libc::c_int, buf: *mut u_char) -> u_short;
    fn panic(msg: *mut libc::c_char);
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
    static mut dbfd: libc::c_int;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type off_t = __darwin_off_t;
pub type ssize_t = __darwin_ssize_t;
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
pub struct GBD {
    pub block: u_int,
    pub next: *mut GBD,
    pub mem: *mut DB_BLOCK,
    pub dirty: *mut GBD,
    pub last_accessed: time_t,
}
#[derive(Copy, Clone)]
#[repr(C, align(4))]
pub struct DB_BLOCK(pub DB_BLOCK_Inner);
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct DB_BLOCK_Inner {
    pub type_0: u_char,
    pub flags: u_char,
    pub spare: u_short,
    pub right_ptr: u_int,
    pub last_idx: u_short,
    pub last_free: u_short,
    pub global: var_u,
}
#[allow(dead_code, non_upper_case_globals)]
const DB_BLOCK_PADDING: usize = ::core::mem::size_of::<DB_BLOCK>()
    - ::core::mem::size_of::<DB_BLOCK_Inner>();
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
pub type DB_Block = DB_BLOCK;
pub type gbd = GBD;
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
#[inline]
unsafe extern "C" fn var_empty(mut var: var_u) -> u_int {
    if var.var_q == 0 as libc::c_int as u_int64 {
        return 1 as libc::c_int as u_int
    } else {
        return 0 as libc::c_int as u_int
    };
}
#[no_mangle]
pub static mut icerr: libc::c_int = 0;
#[no_mangle]
pub static mut doing_full: libc::c_int = 0;
#[no_mangle]
pub static mut wrt_buf: [u_char; 100] = [0; 100];
#[no_mangle]
pub static mut outc: *mut cstring = 0 as *const cstring as *mut cstring;
#[no_mangle]
pub static mut rlnk: *mut u_char = 0 as *const u_char as *mut u_char;
#[no_mangle]
pub static mut dlnk: *mut u_char = 0 as *const u_char as *mut u_char;
#[no_mangle]
pub static mut used: *mut u_char = 0 as *const u_char as *mut u_char;
#[no_mangle]
pub static mut volsiz: u_int = 0;
#[no_mangle]
pub unsafe extern "C" fn DB_ic(
    mut vol: libc::c_int,
    mut block: libc::c_int,
) -> libc::c_int {
    let mut uci: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    if vol > 1 as libc::c_int {
        return -(26 as libc::c_int);
    }
    if ((*systab).vol[(vol - 1 as libc::c_int) as usize]).is_null() {
        return -(26 as libc::c_int);
    }
    volnum = vol;
    curr_lock = 0 as libc::c_int;
    writing = 0 as libc::c_int;
    icerr = 0 as libc::c_int;
    doing_full = 0 as libc::c_int;
    outc = wrt_buf.as_mut_ptr() as *mut cstring;
    used = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map as *mut u_char;
    volsiz = (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).max_block;
    gbd_expired = 0 as libc::c_int;
    level = 0 as libc::c_int;
    while level < 12 as libc::c_int {
        let fresh0 = level;
        level = level + 1;
        blk[fresh0 as usize] = 0 as *mut gbd;
    }
    if block == 0 as libc::c_int {
        level = 0 as libc::c_int;
        ic_full();
        gbd_expired = 60 as libc::c_int;
        return icerr;
    } else if block > 0 as libc::c_int {
        level = 1 as libc::c_int;
        uci = 0 as libc::c_int;
        while uci < 64 as libc::c_int {
            b1 = (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                .uci[uci as usize]
                .global as libc::c_int;
            if b1 == block {
                level = 0 as libc::c_int;
                break;
            } else {
                uci += 1;
                uci;
            }
        }
        ic_block(
            block as u_int,
            0 as libc::c_int as u_int,
            0 as *mut u_char,
            VAR_U {
                var_q: 0 as libc::c_ulonglong,
            },
        );
        gbd_expired = 60 as libc::c_int;
        return icerr;
    }
    dbfd = partab.vol_fds[(volnum - 1 as libc::c_int) as usize];
    ic_map(block);
    return icerr;
}
#[no_mangle]
pub unsafe extern "C" fn ic_full() {
    let mut size: u_int = 0;
    let mut uci: libc::c_int = 0;
    let mut b1: u_int = 0;
    let mut off: u_char = 0;
    let mut msg: [u_char; 20] = [0; 20];
    doing_full = 1 as libc::c_int;
    size = (volsiz / 8 as libc::c_int as u_int).wrapping_add(1 as libc::c_int as u_int);
    rlnk = malloc(size as libc::c_ulong) as *mut u_char;
    if rlnk.is_null() {
        panic(
            b"ic_full: can't get memory for rlnk\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    dlnk = malloc(size as libc::c_ulong) as *mut u_char;
    if dlnk.is_null() {
        panic(
            b"ic_full: can't get memory for dlnk\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    memset(rlnk as *mut libc::c_void, 0 as libc::c_int, size as libc::c_ulong);
    memset(dlnk as *mut libc::c_void, 0 as libc::c_int, size as libc::c_ulong);
    *rlnk.offset(0 as libc::c_int as isize) = 1 as libc::c_int as u_char;
    *dlnk.offset(0 as libc::c_int as isize) = 1 as libc::c_int as u_char;
    uci = 0 as libc::c_int;
    while uci < 64 as libc::c_int {
        b1 = (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
            .uci[uci as usize]
            .global;
        if !(b1 == 0 as libc::c_int as u_int) {
            if *used.offset((b1 / 8 as libc::c_int as u_int) as isize) as libc::c_uint
                & (1 as libc::c_uint) << (b1 & 7 as libc::c_int as u_int)
                == 0 as libc::c_int as libc::c_uint
            {
                (*outc)
                    .len = sprintf(
                    &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut u_char as *mut libc::c_char,
                    b"%10u free (global directory for UCI %d) - skipped\0" as *const u8
                        as *const libc::c_char,
                    b1,
                    uci + 1 as libc::c_int,
                ) as u_short;
                icerr += 1;
                icerr;
                SQ_Write(outc);
                SQ_WriteFormat(-(1 as libc::c_int));
            } else {
                ic_bits(b1, 3 as libc::c_int, 0 as libc::c_int as u_int);
                level = 0 as libc::c_int;
                ic_block(
                    b1,
                    0 as libc::c_int as u_int,
                    0 as *mut u_char,
                    VAR_U {
                        var_q: 0 as libc::c_ulonglong,
                    },
                );
            }
        }
        uci += 1;
        uci;
    }
    let mut i: u_int = 0 as libc::c_int as u_int;
    while i < volsiz / 8 as libc::c_int as u_int {
        let mut j: u_int = 0 as libc::c_int as u_int;
        while j < 8 as libc::c_int as u_int {
            off = ((1 as libc::c_uint) << j) as u_char;
            b1 = (i * 8 as libc::c_int as u_int).wrapping_add(j);
            memcpy(
                msg.as_mut_ptr() as *mut libc::c_void,
                b"both pointers\0\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                14 as libc::c_int as libc::c_ulong,
            );
            if *used.offset(i as isize) as libc::c_int & off as libc::c_int
                != 0 as libc::c_int
            {
                if *rlnk.offset(i as isize) as libc::c_int & off as libc::c_int
                    == 0 as libc::c_int
                    || *dlnk.offset(i as isize) as libc::c_int & off as libc::c_int
                        == 0 as libc::c_int
                {
                    if *rlnk.offset(i as isize) as libc::c_int & off as libc::c_int
                        != 0 as libc::c_int
                    {
                        memcpy(
                            msg.as_mut_ptr() as *mut libc::c_void,
                            b"down pointer\0\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            13 as libc::c_int as libc::c_ulong,
                        );
                    } else if *dlnk.offset(i as isize) as libc::c_int
                        & off as libc::c_int != 0 as libc::c_int
                    {
                        memcpy(
                            msg.as_mut_ptr() as *mut libc::c_void,
                            b"right pointer\0\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            14 as libc::c_int as libc::c_ulong,
                        );
                    }
                    (*outc)
                        .len = sprintf(
                        &mut *((*outc).buf)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut u_char
                            as *mut libc::c_char,
                        b"%10u is used, missing %s\0" as *const u8
                            as *const libc::c_char,
                        b1,
                        msg.as_mut_ptr(),
                    ) as u_short;
                    icerr += 1;
                    icerr;
                    SQ_Write(outc);
                    SQ_WriteFormat(-(1 as libc::c_int));
                }
            } else if *rlnk.offset(i as isize) as libc::c_int & off as libc::c_int
                != 0 as libc::c_int
                || *dlnk.offset(i as isize) as libc::c_int & off as libc::c_int
                    != 0 as libc::c_int
            {
                (*outc)
                    .len = sprintf(
                    &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut u_char as *mut libc::c_char,
                    b"%10u is unused but is pointed to\0" as *const u8
                        as *const libc::c_char,
                    b1,
                ) as u_short;
                icerr += 1;
                icerr;
                SQ_Write(outc);
                SQ_WriteFormat(-(1 as libc::c_int));
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(rlnk as *mut libc::c_void);
    free(dlnk as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ic_bits(
    mut block: u_int,
    mut flag: libc::c_int,
    mut points_at: u_int,
) {
    let mut i: u_int = 0;
    let mut off: u_char = 0;
    i = block >> 3 as libc::c_int;
    off = ((1 as libc::c_uint) << (block & 7 as libc::c_int as u_int)) as u_char;
    if flag & 1 as libc::c_int != 0 {
        if *rlnk.offset(i as isize) as libc::c_int & off as libc::c_int != 0 {
            (*outc)
                .len = sprintf(
                &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_char,
                b"%10u <- %10u - duplicate right pointer\0" as *const u8
                    as *const libc::c_char,
                block,
                points_at,
            ) as u_short;
            icerr += 1;
            icerr;
            SQ_Write(outc);
            SQ_WriteFormat(-(1 as libc::c_int));
        } else {
            let ref mut fresh1 = *rlnk.offset(i as isize);
            *fresh1 = (*fresh1 as libc::c_int | off as libc::c_int) as u_char;
        }
    }
    if flag & 2 as libc::c_int != 0 {
        if *dlnk.offset(i as isize) as libc::c_int & off as libc::c_int != 0 {
            (*outc)
                .len = sprintf(
                &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_char,
                b"%10u <- %10u - duplicate down pointer\0" as *const u8
                    as *const libc::c_char,
                block,
                points_at,
            ) as u_short;
            icerr += 1;
            icerr;
            SQ_Write(outc);
            SQ_WriteFormat(-(1 as libc::c_int));
        } else {
            let ref mut fresh2 = *dlnk.offset(i as isize);
            *fresh2 = (*fresh2 as libc::c_int | off as libc::c_int) as u_char;
        }
    }
    if points_at != 0
        && *used.offset(i as isize) as libc::c_int & off as libc::c_int
            == 0 as libc::c_int
    {
        (*outc)
            .len = sprintf(
            &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_char,
            b"%10u <- %10u - block is free\0" as *const u8 as *const libc::c_char,
            block,
            points_at,
        ) as u_short;
        icerr += 1;
        icerr;
        SQ_Write(outc);
        SQ_WriteFormat(-(1 as libc::c_int));
    }
}
#[no_mangle]
pub unsafe extern "C" fn ic_block(
    mut block: u_int,
    mut points_at: u_int,
    mut kin: *mut u_char,
    mut global: var_u,
) -> u_int {
    let mut s: libc::c_short = 0;
    let mut left_edge: libc::c_int = 0;
    let mut emsg: [u_char; 80] = [0; 80];
    let mut isdata: libc::c_int = 0;
    let mut Llevel: libc::c_int = 0;
    let mut Lgbd: *mut gbd = 0 as *mut gbd;
    let mut b1: u_int = 0;
    let mut k: [u_char; 260] = [0; 260];
    let mut isx: *mut u_short = 0 as *mut u_short;
    let mut iix: *mut u_int = 0 as *mut u_int;
    let mut k1: [u_char; 260] = [0; 260];
    let mut k2: [u_char; 260] = [0; 260];
    let mut c: *mut cstring = 0 as *mut cstring;
    let mut r: *mut cstring = 0 as *mut cstring;
    let mut eob: *mut u_char = 0 as *mut u_char;
    let mut lb: u_int = 0;
    let mut brl: u_int = 0;
    while SemOp(2 as libc::c_int, -(1 as libc::c_int)) != 0 {}
    s = Get_block(block);
    if (s as libc::c_int) < 0 as libc::c_int {
        UTIL_strerror(s as libc::c_int, emsg.as_mut_ptr());
        (*outc)
            .len = sprintf(
            &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_char,
            b"%10u <- %10u - error getting - %s\0" as *const u8 as *const libc::c_char,
            block,
            points_at,
            emsg.as_mut_ptr(),
        ) as u_short;
        icerr += 1;
        icerr;
        SQ_Write(outc);
        SQ_WriteFormat(-(1 as libc::c_int));
        SemOp(2 as libc::c_int, -curr_lock);
        return 0 as libc::c_int as u_int;
    }
    if *used.offset((block / 8 as libc::c_int as u_int) as isize) as libc::c_uint
        & (1 as libc::c_uint) << (block & 7 as libc::c_int as u_int)
        == 0 as libc::c_int as libc::c_uint
    {
        (*outc)
            .len = sprintf(
            &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_char,
            b"%10u <- %10u marked free, type = %d\0" as *const u8 as *const libc::c_char,
            block,
            points_at,
            (*(*blk[level as usize]).mem).0.type_0 as libc::c_int,
        ) as u_short;
        icerr += 1;
        icerr;
        SQ_Write(outc);
        SQ_WriteFormat(-(1 as libc::c_int));
        return 0 as libc::c_int as u_int;
    }
    eob = ((*blk[level as usize]).mem as *mut u_char)
        .offset(
            (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).block_size
                as isize,
        )
        .offset(-(1 as libc::c_int as isize));
    if ((*blk[level as usize]).dirty).is_null() {
        (*blk[level as usize]).dirty = 3 as libc::c_int as *mut gbd;
    }
    isdata = ((*(*blk[level as usize]).mem).0.type_0 as libc::c_int > 64 as libc::c_int
        && level != 0) as libc::c_int;
    Llevel = level;
    Lgbd = blk[level as usize];
    s = SemOp(2 as libc::c_int, -curr_lock);
    if var_empty(global) == 0 {
        if var_equal(global, (*(*blk[level as usize]).mem).0.global) == 0 {
            (*outc)
                .len = sprintf(
                &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_char,
                b"%10u <- %10u - global is wrong\0" as *const u8 as *const libc::c_char,
                block,
                points_at,
            ) as u_short;
            icerr += 1;
            icerr;
            SQ_Write(outc);
            SQ_WriteFormat(-(1 as libc::c_int));
        }
    }
    chunk = &mut *iidx
        .offset(
            *idx
                .offset(
                    (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                        / 2 as libc::c_int as u_int) as isize,
                ) as isize,
        ) as *mut libc::c_int as *mut cstring;
    left_edge = ((*chunk).buf[1 as libc::c_int as usize] == 0) as libc::c_int;
    if (*chunk).buf[0 as libc::c_int as usize] != 0 {
        (*outc)
            .len = sprintf(
            &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_char,
            b"%10u <- %10u - non-zero CCC on first key\0" as *const u8
                as *const libc::c_char,
            block,
            points_at,
        ) as u_short;
        icerr += 1;
        icerr;
        SQ_Write(outc);
        SQ_WriteFormat(-(1 as libc::c_int));
    } else if !kin.is_null() {
        if memcmp(
            &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            kin as *const libc::c_void,
            (*kin.offset(0 as libc::c_int as isize) as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong,
        ) != 0
        {
            (*outc)
                .len = sprintf(
                &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_char,
                b"%10u <- %10u - down link differs from first key\0" as *const u8
                    as *const libc::c_char,
                block,
                points_at,
            ) as u_short;
            icerr += 1;
            icerr;
            SQ_Write(outc);
            SQ_WriteFormat(-(1 as libc::c_int));
        }
    }
    if isdata == 0 {
        let mut Llast: libc::c_int = (*(*blk[level as usize]).mem).0.last_idx
            as libc::c_int;
        lb = 0 as libc::c_int as u_int;
        brl = 0 as libc::c_int as u_int;
        let mut Lidx: libc::c_int = (::core::mem::size_of::<DB_Block>() as libc::c_ulong
            as u_int / 2 as libc::c_int as u_int) as libc::c_int;
        while Lidx <= Llast {
            level = Llevel;
            if !(level == 0
                && Lidx as u_int
                    == ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                        / 2 as libc::c_int as u_int)
            {
                blk[level as usize] = Lgbd;
                idx = (*blk[level as usize]).mem as *mut u_short;
                iidx = (*blk[level as usize]).mem as *mut libc::c_int;
                chunk = &mut *iidx.offset(*idx.offset(Lidx as isize) as isize)
                    as *mut libc::c_int as *mut cstring;
                if level == 0 {
                    k[0 as libc::c_int as usize] = '\0' as i32 as u_char;
                } else {
                    memcpy(
                        &mut *k
                            .as_mut_ptr()
                            .offset(
                                (*((*chunk).buf)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as libc::c_int
                                    + 1 as libc::c_int) as isize,
                            ) as *mut u_char as *mut libc::c_void,
                        &mut *((*chunk).buf)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize) as *mut u_char
                            as *const libc::c_void,
                        (*chunk).buf[1 as libc::c_int as usize] as libc::c_ulong,
                    );
                    k[0 as libc::c_int
                        as usize] = ((*chunk).buf[0 as libc::c_int as usize]
                        as libc::c_int
                        + (*chunk).buf[1 as libc::c_int as usize] as libc::c_int)
                        as u_char;
                }
                record = &mut *((*chunk).buf)
                    .as_mut_ptr()
                    .offset(
                        (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                            as libc::c_int + 2 as libc::c_int) as isize,
                    ) as *mut u_char as *mut cstring;
                Align_record();
                b1 = *(record as *mut u_int);
                if b1 > volsiz || b1 == 0 {
                    (*outc)
                        .len = sprintf(
                        &mut *((*outc).buf)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut u_char
                            as *mut libc::c_char,
                        b"%10u <- %10u - (%d) block %u outside volume - skipped\0"
                            as *const u8 as *const libc::c_char,
                        block,
                        points_at,
                        Lidx,
                        b1,
                    ) as u_short;
                    icerr += 1;
                    icerr;
                    SQ_Write(outc);
                    SQ_WriteFormat(-(1 as libc::c_int));
                } else {
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while i <= level {
                        if !(blk[i as usize]).is_null() && (*blk[i as usize]).block == b1
                        {
                            (*outc)
                                .len = sprintf(
                                &mut *((*outc).buf)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as *mut u_char
                                    as *mut libc::c_char,
                                b"%10u <- %10u - points at itself\0" as *const u8
                                    as *const libc::c_char,
                                b1,
                                block,
                            ) as u_short;
                            icerr += 1;
                            icerr;
                            SQ_Write(outc);
                            SQ_WriteFormat(-(1 as libc::c_int));
                            b1 = 0 as libc::c_int as u_int;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    if !(b1 == 0) {
                        if doing_full != 0 {
                            ic_bits(
                                b1,
                                2 as libc::c_int
                                    + (left_edge != 0 || level == 0) as libc::c_int,
                                block,
                            );
                        }
                        if lb != 0 && level != 0 {
                            if brl != b1 {
                                (*outc)
                                    .len = sprintf(
                                    &mut *((*outc).buf)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut u_char
                                        as *mut libc::c_char,
                                    b"%10d <- %10d - right is %10d next down is %10d\0"
                                        as *const u8 as *const libc::c_char,
                                    lb,
                                    block,
                                    brl,
                                    b1,
                                ) as u_short;
                                icerr += 1;
                                icerr;
                                SQ_Write(outc);
                                SQ_WriteFormat(-(1 as libc::c_int));
                            }
                        }
                        lb = b1;
                        left_edge = 0 as libc::c_int;
                        level += 1;
                        level;
                        if level > 1 as libc::c_int {
                            brl = ic_block(
                                b1,
                                block,
                                k.as_mut_ptr(),
                                (*(*blk[(level - 1 as libc::c_int) as usize]).mem).0.global,
                            );
                        } else {
                            brl = ic_block(
                                b1,
                                block,
                                k.as_mut_ptr(),
                                VAR_U {
                                    var_q: 0 as libc::c_ulonglong,
                                },
                            );
                        }
                    }
                }
            }
            Lidx += 1;
            Lidx;
        }
    }
    level = Llevel;
    blk[level as usize] = Lgbd;
    idx = (*blk[level as usize]).mem as *mut u_short;
    iidx = (*blk[level as usize]).mem as *mut libc::c_int;
    if (*(*blk[level as usize]).mem).0.right_ptr != 0 && doing_full != 0 {
        ic_bits((*(*blk[level as usize]).mem).0.right_ptr, 1 as libc::c_int, block);
    }
    if (*blk[level as usize]).dirty == 3 as libc::c_int as *mut gbd {
        (*blk[level as usize]).dirty = 0 as *mut GBD;
    }
    if ((*(*blk[level as usize]).mem).0.last_idx as u_int)
        < ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
            / 2 as libc::c_int as u_int
    {
        (*outc)
            .len = sprintf(
            &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_char,
            b"%10u <- %10u - last index is too low\0" as *const u8
                as *const libc::c_char,
            block,
            points_at,
        ) as u_short;
        icerr += 1;
        icerr;
        SQ_Write(outc);
        SQ_WriteFormat(-(1 as libc::c_int));
    }
    if (((*(*blk[level as usize]).mem).0.last_free as libc::c_int * 2 as libc::c_int
        + 1 as libc::c_int - (*(*blk[level as usize]).mem).0.last_idx as libc::c_int)
        * 2 as libc::c_int) < 0 as libc::c_int
    {
        (*outc)
            .len = sprintf(
            &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *mut libc::c_char,
            b"%10u <- %10u - last index is too high or last free is too low\0"
                as *const u8 as *const libc::c_char,
            block,
            points_at,
        ) as u_short;
        icerr += 1;
        icerr;
        SQ_Write(outc);
        SQ_WriteFormat(-(1 as libc::c_int));
    }
    isdata = ((*(*blk[level as usize]).mem).0.type_0 as libc::c_int > 64 as libc::c_int
        && level != 0) as libc::c_int;
    isx = (*blk[level as usize]).mem as *mut u_short;
    iix = (*blk[level as usize]).mem as *mut u_int;
    k1[0 as libc::c_int as usize] = 0 as libc::c_int as u_char;
    let mut i_0: u_int = ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int;
    while i_0 <= (*(*blk[level as usize]).mem).0.last_idx as u_int {
        c = &mut *iix.offset(*isx.offset(i_0 as isize) as isize) as *mut u_int
            as *mut cstring;
        if &mut *((*c).buf)
            .as_mut_ptr()
            .offset(((*c).len as libc::c_int - 3 as libc::c_int) as isize) as *mut u_char
            > eob
        {
            (*outc)
                .len = sprintf(
                &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *mut libc::c_char,
                b"%10u <- %10u - chunk size is too big - overflows block\0" as *const u8
                    as *const libc::c_char,
                block,
                points_at,
            ) as u_short;
            icerr += 1;
            icerr;
            SQ_Write(outc);
            SQ_WriteFormat(-(1 as libc::c_int));
        }
        r = &mut *((*c).buf)
            .as_mut_ptr()
            .offset(
                (*((*c).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as libc::c_int + 2 as libc::c_int) as isize,
            ) as *mut u_char as *mut cstring;
        if isdata != 0
            && (*r).len as libc::c_int != 65534 as libc::c_int + 1 as libc::c_int
        {
            if &mut *((*r).buf)
                .as_mut_ptr()
                .offset(((*r).len as libc::c_int - 1 as libc::c_int) as isize)
                as *mut u_char > eob
            {
                (*outc)
                    .len = sprintf(
                    &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut u_char as *mut libc::c_char,
                    b"%10u <- %10u - DBC is too big - overflows block\0" as *const u8
                        as *const libc::c_char,
                    block,
                    points_at,
                ) as u_short;
                icerr += 1;
                icerr;
                SQ_Write(outc);
                SQ_WriteFormat(-(1 as libc::c_int));
            }
        }
        if !((*c).buf[0 as libc::c_int as usize] as libc::c_int == 255 as libc::c_int) {
            if i_0
                == ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                    / 2 as libc::c_int as u_int
                && (*c).buf[0 as libc::c_int as usize] as libc::c_int != 0
            {
                (*outc)
                    .len = sprintf(
                    &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut u_char as *mut libc::c_char,
                    b"%10u <- %10u - non-zero CCC in first record\0" as *const u8
                        as *const libc::c_char,
                    block,
                    points_at,
                ) as u_short;
                icerr += 1;
                icerr;
                SQ_Write(outc);
                SQ_WriteFormat(-(1 as libc::c_int));
            }
            if i_0
                > ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                    / 2 as libc::c_int as u_int
                && (*c).buf[1 as libc::c_int as usize] == 0
            {
                (*outc)
                    .len = sprintf(
                    &mut *((*outc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut u_char as *mut libc::c_char,
                    b"%10u <- %10u - zero UCC found\0" as *const u8
                        as *const libc::c_char,
                    block,
                    points_at,
                ) as u_short;
                icerr += 1;
                icerr;
                SQ_Write(outc);
                SQ_WriteFormat(-(1 as libc::c_int));
            }
            memcpy(
                &mut *k2
                    .as_mut_ptr()
                    .offset(
                        (*((*c).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                            as libc::c_int + 1 as libc::c_int) as isize,
                    ) as *mut u_char as *mut libc::c_void,
                &mut *((*c).buf).as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                (*c).buf[1 as libc::c_int as usize] as libc::c_ulong,
            );
            k2[0 as libc::c_int
                as usize] = ((*c).buf[0 as libc::c_int as usize] as libc::c_int
                + (*c).buf[1 as libc::c_int as usize] as libc::c_int) as u_char;
            if k2[0 as libc::c_int as usize] as libc::c_int != 0
                || i_0
                    > ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                        / 2 as libc::c_int as u_int
            {
                if UTIL_Key_KeyCmp(
                    &mut *k1.as_mut_ptr().offset(1 as libc::c_int as isize),
                    &mut *k2.as_mut_ptr().offset(1 as libc::c_int as isize),
                    k1[0 as libc::c_int as usize] as libc::c_int,
                    k2[0 as libc::c_int as usize] as libc::c_int,
                ) != -(1 as libc::c_int)
                {
                    (*outc)
                        .len = sprintf(
                        &mut *((*outc).buf)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut u_char
                            as *mut libc::c_char,
                        b"%10u <- %10u - (%u) key does not follow previous\0"
                            as *const u8 as *const libc::c_char,
                        block,
                        points_at,
                        i_0,
                    ) as u_short;
                    icerr += 1;
                    icerr;
                    SQ_Write(outc);
                    SQ_WriteFormat(-(1 as libc::c_int));
                }
            }
            memcpy(
                k1.as_mut_ptr() as *mut libc::c_void,
                k2.as_mut_ptr() as *const libc::c_void,
                (k2[0 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int)
                    as libc::c_ulong,
            );
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    return (*(*blk[level as usize]).mem).0.right_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn ic_map(mut flag: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut block: u_int = 0;
    let mut file_off: off_t = 0;
    let mut lock: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut c: *mut u_char = 0 as *mut u_char;
    let mut e: *mut u_char = 0 as *mut u_char;
    let mut ptr: *mut gbd = 0 as *mut gbd;
    let mut status: libc::c_int = 0;
    let mut type_byte: u_char = 0;
    lock = if flag == -(1 as libc::c_int) {
        -(1 as libc::c_int)
    } else {
        ((*systab).maxjob).wrapping_neg() as libc::c_int
    };
    c = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map as *mut u_char;
    e = &mut *c
        .offset(
            ((*(**((*systab).vol)
                .as_mut_ptr()
                .offset((volnum - 1 as libc::c_int) as isize))
                .vollab)
                .max_block >> 3 as libc::c_int) as isize,
        ) as *mut u_char;
    off = 1 as libc::c_int;
    while c <= e {
        let mut base: u_int = (c
            .offset_from(
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map as *mut u_char,
            ) as libc::c_long as u_int) << 3 as libc::c_int;
        while SemOp(2 as libc::c_int, lock) != 0 {}
        while off < 8 as libc::c_int {
            block = base.wrapping_add(off as u_int);
            status = -(1 as libc::c_int);
            if !(block
                > (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                    .max_block)
            {
                ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .gbd_hash[(block & (1024 as libc::c_int - 1 as libc::c_int) as u_int)
                    as usize];
                while !ptr.is_null() {
                    if (*ptr).block == block {
                        type_byte = (*(*ptr).mem).0.type_0;
                        if (*(*ptr).mem).0.type_0 != 0 {
                            status = 1 as libc::c_int;
                        } else {
                            status = 0 as libc::c_int;
                        }
                        break;
                    } else {
                        ptr = (*ptr).next;
                    }
                }
                if status == -(1 as libc::c_int) {
                    file_off = block as off_t - 1 as libc::c_int as off_t;
                    file_off = file_off
                        * (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .vollab)
                            .block_size as off_t
                        + (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .vollab)
                            .header_bytes as off_t;
                    file_off = lseek(dbfd, file_off, 0 as libc::c_int);
                    if file_off < 1 as libc::c_int as off_t {
                        panic(
                            b"ic_map: lseek() failed!!\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    i = read(
                        dbfd,
                        &mut type_byte as *mut u_char as *mut libc::c_void,
                        1 as libc::c_int as size_t,
                    ) as libc::c_int;
                    if i < 0 as libc::c_int {
                        panic(
                            b"ic_map: read() failed!!\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    status = (type_byte as libc::c_int != 0 as libc::c_int)
                        as libc::c_int;
                }
                if !(*c as libc::c_uint & (1 as libc::c_uint) << off != 0 && status != 0)
                {
                    if !(*c as libc::c_uint & (1 as libc::c_uint) << off
                        == 0 as libc::c_int as libc::c_uint && status == 0)
                    {
                        icerr += 1;
                        icerr;
                        if !(flag == -(1 as libc::c_int)) {
                            if status != 0 {
                                *c = (*c as libc::c_int
                                    | ((1 as libc::c_uint) << off) as u_char as libc::c_int)
                                    as u_char;
                            } else {
                                *c = (*c as libc::c_int
                                    & !((1 as libc::c_uint) << off) as u_char as libc::c_int)
                                    as u_char;
                            }
                            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                                .map_dirty_flag = 1 as libc::c_int;
                        }
                    }
                }
            }
            off += 1;
            off;
        }
        SemOp(2 as libc::c_int, -curr_lock);
        c = c.offset(1);
        c;
        off = 0 as libc::c_int;
        if flag == -(3 as libc::c_int) {
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .upto = (e.offset_from(c) as libc::c_long as u_int) << 3 as libc::c_int;
        }
    }
    if flag == -(3 as libc::c_int) {
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .upto = 0 as libc::c_int as u_int;
    }
}
