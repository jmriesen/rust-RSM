#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type RBD;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn close(_: libc::c_int) -> libc::c_int;
    fn lseek(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    fn __error() -> *mut libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    static mut curr_lock: libc::c_int;
    static mut gbd_expired: libc::c_int;
    static mut volnum: libc::c_int;
    static mut blk: [*mut gbd; 12];
    static mut level: libc::c_int;
    static mut Index: u_int;
    static mut idx: *mut u_short;
    static mut iidx: *mut libc::c_int;
    static mut writing: libc::c_int;
    static mut hash_start: libc::c_int;
    fn current_time(local: libc::c_short) -> time_t;
    fn SchedYield();
    fn panic(msg: *mut libc::c_char);
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
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
#[no_mangle]
pub unsafe extern "C" fn Get_block(mut blknum: u_int) -> libc::c_short {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut s: libc::c_short = -(1 as libc::c_int) as libc::c_short;
    let mut file_off: off_t = 0;
    let mut ptr: *mut gbd = 0 as *mut gbd;
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .logrd = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.logrd)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.logrd;
    ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .gbd_hash[(blknum & (1024 as libc::c_int - 1 as libc::c_int) as u_int) as usize];
    loop {
        if ptr.is_null() {
            current_block = 8515828400728868193;
            break;
        }
        if (*ptr).block == blknum {
            blk[level as usize] = ptr;
            while (*ptr).last_accessed == 0 as libc::c_int as time_t {
                SchedYield();
            }
            current_block = 1148824969881194880;
            break;
        } else {
            ptr = (*ptr).next;
        }
    }
    match current_block {
        8515828400728868193 => {
            if writing == 0 {
                SemOp(2 as libc::c_int, -curr_lock);
                s = SemOp(
                    2 as libc::c_int,
                    ((*systab).maxjob).wrapping_neg() as libc::c_int,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    return s;
                }
                ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .gbd_hash[(blknum
                    & (1024 as libc::c_int - 1 as libc::c_int) as u_int) as usize];
                loop {
                    if ptr.is_null() {
                        current_block = 2838571290723028321;
                        break;
                    }
                    if (*ptr).block == blknum {
                        blk[level as usize] = ptr;
                        SemOp(
                            2 as libc::c_int,
                            ((*systab).maxjob).wrapping_sub(1 as libc::c_int as u_int)
                                as libc::c_int,
                        );
                        while (*ptr).last_accessed == 0 as libc::c_int as time_t {
                            SchedYield();
                        }
                        current_block = 1148824969881194880;
                        break;
                    } else {
                        ptr = (*ptr).next;
                    }
                }
            } else {
                current_block = 2838571290723028321;
            }
            match current_block {
                1148824969881194880 => {}
                _ => {
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .phyrd = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .phyrd)
                        .wrapping_add(1);
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.phyrd;
                    Get_GBD();
                    (*blk[level as usize]).block = blknum;
                    (*blk[level as usize]).last_accessed = 0 as libc::c_int as time_t;
                    i = (blknum & (1024 as libc::c_int - 1 as libc::c_int) as u_int)
                        as libc::c_int;
                    (*blk[level as usize])
                        .next = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .gbd_hash[i as usize];
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .gbd_hash[i as usize] = blk[level as usize];
                    if writing == 0 {
                        SemOp(
                            2 as libc::c_int,
                            ((*systab).maxjob).wrapping_sub(1 as libc::c_int as u_int)
                                as libc::c_int,
                        );
                    }
                    file_off = blknum as off_t - 1 as libc::c_int as off_t;
                    file_off = file_off
                        * (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .vollab)
                            .block_size as off_t
                        + (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .vollab)
                            .header_bytes as off_t;
                    if volnum > 1 as libc::c_int {
                        if volnum > 1 as libc::c_int {
                            return -(26 as libc::c_int) as libc::c_short;
                        }
                        if partab.vol_fds[(volnum - 1 as libc::c_int) as usize]
                            == 0 as libc::c_int
                        {
                            if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                                .file_name[0 as libc::c_int as usize] as libc::c_int
                                == 0 as libc::c_int
                            {
                                return -(72 as libc::c_int + 200 as libc::c_int)
                                    as libc::c_short;
                            }
                            i = open(
                                ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                                    .file_name)
                                    .as_mut_ptr(),
                                0 as libc::c_int,
                            );
                            if i < 0 as libc::c_int {
                                return -(200 as libc::c_int + 200 as libc::c_int
                                    + *__error()) as libc::c_short;
                            }
                            partab.vol_fds[(volnum - 1 as libc::c_int) as usize] = i;
                        } else if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .file_name[0 as libc::c_int as usize] as libc::c_int
                            == 0 as libc::c_int
                        {
                            close(partab.vol_fds[(volnum - 1 as libc::c_int) as usize]);
                            partab
                                .vol_fds[(volnum - 1 as libc::c_int)
                                as usize] = 0 as libc::c_int;
                            return -(72 as libc::c_int + 200 as libc::c_int)
                                as libc::c_short;
                        }
                    }
                    file_off = lseek(
                        partab.vol_fds[(volnum - 1 as libc::c_int) as usize],
                        file_off,
                        0 as libc::c_int,
                    );
                    if file_off < 1 as libc::c_int as off_t {
                        panic(
                            b"Get_block: lseek() failed!\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    i = read(
                        partab.vol_fds[(volnum - 1 as libc::c_int) as usize],
                        (*blk[level as usize]).mem as *mut libc::c_void,
                        (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                            .block_size as size_t,
                    ) as libc::c_int;
                    if i < 0 as libc::c_int {
                        panic(
                            b"Get_block: read() failed!\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                }
            }
        }
        _ => {}
    }
    (*blk[level as usize])
        .last_accessed = current_time(1 as libc::c_int as libc::c_short);
    if writing != 0 && (*blk[level as usize]).dirty < 5 as libc::c_int as *mut gbd {
        (*blk[level as usize]).dirty = 1 as libc::c_int as *mut gbd;
    }
    Index = ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int;
    idx = (*blk[level as usize]).mem as *mut u_short;
    iidx = (*blk[level as usize]).mem as *mut libc::c_int;
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn New_block() -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut blknum: u_int = 0;
    let mut c: *mut u_char = 0 as *mut u_char;
    let mut end: *mut u_char = 0 as *mut u_char;
    Get_GBD();
    Index = ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int;
    c = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).first_free as *mut u_char;
    end = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map as *mut u_char)
        .offset(
            ((*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).max_block
                >> 3 as libc::c_int) as isize,
        );
    while c <= end {
        if *c as libc::c_int != 255 as libc::c_int {
            blknum = (c
                .offset_from(
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map
                        as *mut u_char,
                ) as libc::c_long * 8 as libc::c_int as libc::c_long) as u_int;
            i = 0 as libc::c_int;
            while (1 as libc::c_uint) << i & *c as libc::c_uint != 0 {
                i += 1;
                i;
            }
            blknum = blknum.wrapping_add(i as u_int);
            if blknum
                <= (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                    .max_block
            {
                *c = (*c as libc::c_uint | (1 as libc::c_uint) << i) as u_char;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .stats
                    .blkalloc = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .stats
                    .blkalloc)
                    .wrapping_add(1);
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.blkalloc;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map_dirty_flag
                    += 1;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map_dirty_flag;
                (*blk[level as usize]).block = blknum;
                (*blk[level as usize])
                    .next = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .gbd_hash[(blknum
                    & (1024 as libc::c_int - 1 as libc::c_int) as u_int) as usize];
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .gbd_hash[(blknum
                    & (1024 as libc::c_int - 1 as libc::c_int) as u_int)
                    as usize] = blk[level as usize];
                memset(
                    (*blk[level as usize]).mem as *mut libc::c_void,
                    0 as libc::c_int,
                    (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                        .block_size as libc::c_ulong,
                );
                (*blk[level as usize]).dirty = 1 as libc::c_int as *mut gbd;
                (*blk[level as usize])
                    .last_accessed = current_time(1 as libc::c_int as libc::c_short);
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .first_free = c as *mut libc::c_void;
                return 0 as libc::c_int as libc::c_short;
            }
        }
        c = c.offset(1);
        c;
    }
    Free_GBD(blk[level as usize]);
    return -(11 as libc::c_int + 200 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Get_GBDs(mut greqd: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut curr: libc::c_int = 0;
    let mut ptr: *mut gbd = 0 as *mut gbd;
    let mut last: *mut gbd = 0 as *mut gbd;
    let mut now: time_t = 0;
    let mut pass: libc::c_int = 0 as libc::c_int;
    loop {
        while SemOp(2 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int)
            != 0
        {}
        ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .gbd_hash[1024 as libc::c_int as usize];
        curr = 0 as libc::c_int;
        while !ptr.is_null() {
            curr += 1;
            curr;
            if curr >= greqd {
                return;
            }
            ptr = (*ptr).next;
        }
        now = current_time(1 as libc::c_int as libc::c_short)
            + 1 as libc::c_int as time_t;
        i = hash_start + 1 as libc::c_int & 1024 as libc::c_int - 1 as libc::c_int;
        loop {
            ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .gbd_hash[i as usize];
            last = 0 as *mut gbd;
            while !ptr.is_null() {
                if (*ptr).block == 0 as libc::c_int as u_int {
                    if last.is_null() {
                        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .gbd_hash[i as usize] = (*ptr).next;
                    } else {
                        (*last).next = (*ptr).next;
                    }
                    (*ptr)
                        .next = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .gbd_hash[1024 as libc::c_int as usize];
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .gbd_hash[1024 as libc::c_int as usize] = ptr;
                    (*ptr).dirty = 0 as *mut GBD;
                    (*ptr).last_accessed = 0 as libc::c_int as time_t;
                    curr += 1;
                    curr;
                    if curr >= greqd {
                        return;
                    }
                    if last.is_null() {
                        ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .gbd_hash[i as usize];
                    } else {
                        ptr = (*last).next;
                    }
                } else {
                    if ((*ptr).dirty).is_null() && (*ptr).last_accessed < now
                        && (*ptr).last_accessed > 0 as libc::c_int as time_t
                    {
                        curr += 1;
                        curr;
                        if curr >= greqd {
                            return;
                        }
                    }
                    last = ptr;
                    ptr = (*ptr).next;
                }
            }
            i = i + 1 as libc::c_int & 1024 as libc::c_int - 1 as libc::c_int;
            if i == hash_start {
                break;
            }
        }
        SemOp(2 as libc::c_int, -curr_lock);
        sleep(1 as libc::c_int as libc::c_uint);
        pass += 1;
        pass;
        if pass > 60 as libc::c_int {
            panic(
                b"Get_GBDs: Can't get enough GBDs after 60 seconds\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Get_GBD() {
    let mut i: libc::c_int = 0;
    let mut now: time_t = 0;
    let mut exp: time_t = 0;
    let mut old: time_t = 0;
    let mut hash: libc::c_int = -(1 as libc::c_int);
    let mut ptr: *mut gbd = 0 as *mut gbd;
    let mut oldptr: *mut gbd = 0 as *mut gbd;
    let mut last: *mut gbd = 0 as *mut gbd;
    '_start: loop {
        if !((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .gbd_hash[1024 as libc::c_int as usize])
            .is_null()
        {
            blk[level
                as usize] = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .gbd_hash[1024 as libc::c_int as usize];
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .gbd_hash[1024 as libc::c_int as usize] = (*blk[level as usize]).next;
            break;
        } else {
            now = current_time(1 as libc::c_int as libc::c_short);
            old = now + 1 as libc::c_int as time_t;
            exp = now - gbd_expired as time_t;
            i = hash_start + 1 as libc::c_int & 1024 as libc::c_int - 1 as libc::c_int;
            loop {
                ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .gbd_hash[i as usize];
                last = 0 as *mut gbd;
                while !ptr.is_null() {
                    if (*ptr).block == 0 as libc::c_int as u_int
                        || ((*ptr).dirty).is_null() && (*ptr).last_accessed < exp
                            && (*ptr).last_accessed > 0 as libc::c_int as time_t
                    {
                        if last.is_null() {
                            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                                .gbd_hash[i as usize] = (*ptr).next;
                        } else {
                            (*last).next = (*ptr).next;
                        }
                        hash_start = i;
                        blk[level as usize] = ptr;
                        break '_start;
                    } else {
                        if ((*ptr).dirty).is_null() && (*ptr).last_accessed < old
                            && (*ptr).last_accessed > 0 as libc::c_int as time_t
                        {
                            old = (*ptr).last_accessed;
                            oldptr = ptr;
                            hash = i;
                        }
                        last = ptr;
                        ptr = (*ptr).next;
                    }
                }
                i = i + 1 as libc::c_int & 1024 as libc::c_int - 1 as libc::c_int;
                if i == hash_start {
                    break;
                }
            }
            if oldptr.is_null() {
                if writing != 0 {
                    panic(
                        b"Get_GBD: Failed to find an available GBD while writing\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                SemOp(2 as libc::c_int, -curr_lock);
                sleep(1 as libc::c_int as libc::c_uint);
                while SemOp(
                    2 as libc::c_int,
                    ((*systab).maxjob).wrapping_neg() as libc::c_int,
                ) != 0
                {}
            } else {
                ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .gbd_hash[hash as usize];
                if ptr == oldptr {
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .gbd_hash[hash as usize] = (*ptr).next;
                } else {
                    while (*ptr).next != oldptr {
                        ptr = (*ptr).next;
                    }
                    (*ptr).next = (*oldptr).next;
                }
                blk[level as usize] = oldptr;
                break;
            }
        }
    }
    (*blk[level as usize]).block = 0 as libc::c_int as u_int;
    (*blk[level as usize]).next = 0 as *mut GBD;
    (*blk[level as usize]).dirty = 0 as *mut GBD;
    (*blk[level as usize]).last_accessed = 0 as libc::c_int as time_t;
    idx = (*blk[level as usize]).mem as *mut u_short;
    iidx = (*blk[level as usize]).mem as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Free_GBD(mut free: *mut gbd) {
    let mut ptr: *mut gbd = 0 as *mut gbd;
    if (*free).block != 0 {
        ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .gbd_hash[((*free).block & (1024 as libc::c_int - 1 as libc::c_int) as u_int)
            as usize];
        if ptr == free {
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .gbd_hash[((*free).block
                & (1024 as libc::c_int - 1 as libc::c_int) as u_int)
                as usize] = (*free).next;
        } else {
            while (*ptr).next != free {
                ptr = (*ptr).next;
            }
            (*ptr).next = (*free).next;
        }
    }
    (*free)
        .next = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .gbd_hash[1024 as libc::c_int as usize];
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .gbd_hash[1024 as libc::c_int as usize] = free;
    (*free).block = 0 as libc::c_int as u_int;
    (*free).dirty = 0 as *mut GBD;
    (*free).last_accessed = 0 as libc::c_int as time_t;
}
