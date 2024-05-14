#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    pub type RBD;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn realpath(_: *const libc::c_char, _: *mut libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __error() -> *mut libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn close(_: libc::c_int) -> libc::c_int;
    fn lseek(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    fn getpagesize() -> libc::c_int;
    fn ftok(_: *const libc::c_char, _: libc::c_int) -> key_t;
    fn shmctl(_: libc::c_int, _: libc::c_int, _: *mut __shmid_ds_new) -> libc::c_int;
    fn shmdt(_: *const libc::c_void) -> libc::c_int;
    fn shmget(_: key_t, _: size_t, _: libc::c_int) -> libc::c_int;
    fn stat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn DB_Daemon(slot: libc::c_int, vol: libc::c_int) -> libc::c_int;
    fn ClearJournal(vol: libc::c_int);
    fn current_time(local: libc::c_short) -> time_t;
    fn Routine_Init(vol: libc::c_int);
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __uint64_t = libc::c_ulonglong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
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
pub type ssize_t = __darwin_ssize_t;
pub type pid_t = __darwin_pid_t;
pub type uid_t = __darwin_uid_t;
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type key_t = __int32_t;
pub type nlink_t = __uint16_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct ipc_perm {
    pub uid: uid_t,
    pub gid: gid_t,
    pub cuid: uid_t,
    pub cgid: gid_t,
    pub mode: mode_t,
    pub _seq: libc::c_ushort,
    pub _key: key_t,
}
pub type shmatt_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct __shmid_ds_new {
    pub shm_perm: ipc_perm,
    pub shm_segsz: size_t,
    pub shm_lpid: pid_t,
    pub shm_cpid: pid_t,
    pub shm_nattch: shmatt_t,
    pub shm_atime: time_t,
    pub shm_dtime: time_t,
    pub shm_ctime: time_t,
    pub shm_internal: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
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
pub type gbd = GBD;
pub type DB_Block = DB_BLOCK;
pub type jrnrec = JRNREC;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct JRNREC {
    pub size: u_short,
    pub action: u_char,
    pub uci: u_char,
    pub time: u_int64,
    pub name: var_u,
    pub slen: u_char,
    pub key: [u_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub magic: u_int,
    pub tmp: [u_char; 4],
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
#[no_mangle]
pub unsafe extern "C" fn DB_Mount(
    mut file: *mut libc::c_char,
    mut vol: libc::c_int,
    mut gmb: u_int,
    mut rmb: u_int,
) -> libc::c_short {
    let mut dbfd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n_gbd: u_int = 0;
    let mut avbl: libc::c_longlong = 0;
    let mut indx: libc::c_int = 0;
    let mut shar_mem_key: key_t = 0;
    let mut shar_mem_id: libc::c_int = 0;
    let mut volset_size: libc::c_longlong = 0;
    let mut pagesize: libc::c_int = 0;
    let mut sbuf: __shmid_ds_new = __shmid_ds_new {
        shm_perm: ipc_perm {
            uid: 0,
            gid: 0,
            cuid: 0,
            cgid: 0,
            mode: 0,
            _seq: 0,
            _key: 0,
        },
        shm_segsz: 0,
        shm_lpid: 0,
        shm_cpid: 0,
        shm_nattch: 0,
        shm_atime: 0,
        shm_dtime: 0,
        shm_ctime: 0,
        shm_internal: 0 as *mut libc::c_void,
    };
    let mut fullpathvol: [libc::c_char; 1024] = [0; 1024];
    let mut gptr: *mut gbd = 0 as *mut gbd;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut labelblock: *mut label_block = 0 as *mut label_block;
    if vol < 1 as libc::c_int || vol >= 1 as libc::c_int {
        return -(71 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if !((*systab).vol[vol as usize]).is_null() {
        return -(71 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if (*systab).maxjob == 1 as libc::c_int as u_int {
        return -(60 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    let mut j: u_int = 0 as libc::c_int as u_int;
    while j < 1 as libc::c_int as u_int {
        if !((*systab).vol[j as usize]).is_null() {
            if ftok(
                ((*(*systab).vol[j as usize]).file_name).as_mut_ptr(),
                50 as libc::c_int,
            ) == ftok(file, 50 as libc::c_int)
            {
                return -(71 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    if gmb > 131072 as libc::c_int as u_int {
        return -(200 as libc::c_int + 200 as libc::c_int + 22 as libc::c_int)
            as libc::c_short;
    }
    if rmb > 4095 as libc::c_int as u_int {
        return -(200 as libc::c_int + 200 as libc::c_int + 22 as libc::c_int)
            as libc::c_short;
    }
    let mut jobs: libc::c_int = (*systab).maxjob as libc::c_int;
    if gmb == 0 as libc::c_int as u_int {
        gmb = (jobs / 2 as libc::c_int) as u_int;
    }
    if gmb < 1 as libc::c_int as u_int {
        gmb = 1 as libc::c_int as u_int;
    }
    if rmb == 0 as libc::c_int as u_int {
        rmb = (jobs / 8 as libc::c_int) as u_int;
    }
    if rmb < 1 as libc::c_int as u_int {
        rmb = 1 as libc::c_int as u_int;
    }
    if (*systab).addsize
        < (::core::mem::size_of::<vol_def>() as libc::c_ulong)
            .wrapping_add((gmb * 1048576 as libc::c_int as u_int) as libc::c_ulong)
            .wrapping_add((rmb * 1048576 as libc::c_int as u_int) as libc::c_ulong)
    {
        return -(60 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    dbfd = open(file, 0x2 as libc::c_int);
    if dbfd == -(1 as libc::c_int) {
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    labelblock = ((*systab).address as *mut libc::c_char)
        .offset((*systab).addoff as isize)
        .offset(::core::mem::size_of::<vol_def>() as libc::c_ulong as isize)
        as *mut label_block;
    i = read(
        dbfd,
        labelblock as *mut libc::c_void,
        ::core::mem::size_of::<label_block>() as libc::c_ulong,
    ) as libc::c_int;
    if i == -(1 as libc::c_int) {
        return -(73 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if i < ::core::mem::size_of::<label_block>() as libc::c_ulong as libc::c_int {
        return -(73 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if (*labelblock).magic != 4155766917 as libc::c_uint {
        return -(73 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if (*labelblock).db_ver as libc::c_int != 2 as libc::c_int {
        return -(73 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < 1 as libc::c_int {
        if !(j_0 == vol || ((*systab).vol[j_0 as usize]).is_null()) {
            if var_equal(
                (*labelblock).volnam,
                (*(*(*systab).vol[j_0 as usize]).vollab).volnam,
            ) != 0
            {
                return -(73 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
        }
        j_0 += 1;
        j_0;
    }
    (*systab)
        .vol[vol
        as usize] = ((*systab).address as *mut libc::c_char)
        .offset((*systab).addoff as isize) as *mut vol_def;
    (*(*systab).vol[vol as usize]).vollab = labelblock;
    pagesize = getpagesize();
    n_gbd = (gmb as libc::c_long * 1048576 as libc::c_int as libc::c_long
        / (*labelblock).block_size as libc::c_long) as u_int;
    while n_gbd < 64 as libc::c_int as u_int {
        gmb = gmb.wrapping_add(1);
        gmb;
        n_gbd = (gmb as libc::c_long * 1048576 as libc::c_int as libc::c_long
            / (*labelblock).block_size as libc::c_long) as u_int;
    }
    shar_mem_key = ftok(
        ((*(*systab).vol[0 as libc::c_int as usize]).file_name).as_mut_ptr(),
        50 as libc::c_int,
    );
    if shar_mem_key == -(1 as libc::c_int) {
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    shar_mem_id = shmget(shar_mem_key, 0 as libc::c_int as size_t, 0 as libc::c_int);
    if shar_mem_id == -(1 as libc::c_int) {
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    volset_size = (::core::mem::size_of::<vol_def>() as libc::c_ulong)
        .wrapping_add((*labelblock).header_bytes as libc::c_ulong)
        .wrapping_add(
            (n_gbd as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<gbd>() as libc::c_ulong),
        )
        .wrapping_add(
            (gmb as libc::c_long * 1048576 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        )
        .wrapping_add((*labelblock).block_size as libc::c_ulong)
        .wrapping_add(
            (rmb as libc::c_long * 1048576 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        ) as libc::c_longlong;
    volset_size = ((volset_size - 1 as libc::c_int as libc::c_longlong)
        / pagesize as libc::c_longlong + 1 as libc::c_int as libc::c_longlong)
        * pagesize as libc::c_longlong;
    if (*systab).addsize < volset_size as u_long {
        return -(60 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    avbl = ((*systab).addsize as libc::c_ulonglong)
        .wrapping_sub(volset_size as libc::c_ulonglong) as libc::c_longlong;
    (*(*systab).vol[vol as usize])
        .map = ((*(*systab).vol[vol as usize]).vollab as *mut libc::c_char)
        .offset(::core::mem::size_of::<label_block>() as libc::c_ulong as isize)
        as *mut libc::c_void;
    (*(*systab).vol[vol as usize]).first_free = (*(*systab).vol[vol as usize]).map;
    (*(*systab).vol[vol as usize])
        .gbd_head = ((*(*systab).vol[vol as usize]).vollab as *mut libc::c_char)
        .offset((*labelblock).header_bytes as isize) as *mut gbd;
    (*(*systab).vol[vol as usize]).num_gbd = n_gbd;
    (*(*systab).vol[vol as usize])
        .global_buf = &mut *((**((*systab).vol).as_mut_ptr().offset(vol as isize))
        .gbd_head)
        .offset(n_gbd as isize) as *mut GBD as *mut libc::c_void;
    (*(*systab).vol[vol as usize])
        .zero_block = &mut *((**((*systab).vol).as_mut_ptr().offset(vol as isize))
        .global_buf as *mut u_char)
        .offset((gmb as libc::c_long * 1048576 as libc::c_int as libc::c_long) as isize)
        as *mut u_char as *mut libc::c_void;
    (*(*systab).vol[vol as usize])
        .rbd_head = ((*(*systab).vol[vol as usize]).zero_block as *mut libc::c_char)
        .offset((*labelblock).block_size as isize) as *mut libc::c_void;
    (*(*systab).vol[vol as usize])
        .rbd_end = ((*(*systab).vol[vol as usize]).vollab as *mut libc::c_char)
        .offset(volset_size as isize) as *mut libc::c_void;
    (*(*systab).vol[vol as usize]).shm_id = shar_mem_id;
    (*(*systab).vol[vol as usize]).map_dirty_flag = 0 as libc::c_int;
    (*systab)
        .addoff = ((*systab).addoff)
        .wrapping_add(
            ((*(*systab).vol[vol as usize]).rbd_end as *mut libc::c_char)
                .offset_from(
                    ((*systab).address as *mut libc::c_char)
                        .offset((*systab).addoff as isize),
                ) as libc::c_long as u_long,
        );
    (*systab).addsize = avbl as u_long;
    if !(realpath(file, fullpathvol.as_mut_ptr())).is_null() {
        if strlen(fullpathvol.as_mut_ptr()) < 256 as libc::c_int as libc::c_ulong {
            strcpy(
                ((*(*systab).vol[vol as usize]).file_name).as_mut_ptr(),
                fullpathvol.as_mut_ptr(),
            );
        } else {
            i = (256 as libc::c_int as libc::c_ulong)
                .wrapping_sub(strlen(fullpathvol.as_mut_ptr())) as libc::c_int;
            strcpy(
                ((*(*systab).vol[vol as usize]).file_name).as_mut_ptr(),
                &mut *fullpathvol.as_mut_ptr().offset(i as isize),
            );
        }
    } else {
        shmdt(systab as *const libc::c_void);
        shmctl(shar_mem_id, 0 as libc::c_int, &mut sbuf);
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    if shmctl(shar_mem_id, 2 as libc::c_int, &mut sbuf) == -(1 as libc::c_int) {
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    if lseek(dbfd, 0 as libc::c_int as off_t, 0 as libc::c_int)
        == -(1 as libc::c_int) as off_t
    {
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    i = read(
        dbfd,
        (*(*systab).vol[vol as usize]).vollab as *mut libc::c_void,
        (*labelblock).header_bytes as size_t,
    ) as libc::c_int;
    if i < (*labelblock).header_bytes as libc::c_int {
        fprintf(
            __stderrp,
            b"Read of label/map block failed - %s\n\0" as *const u8
                as *const libc::c_char,
            strerror(*__error()),
        );
        shmdt(systab as *const libc::c_void);
        shmctl(shar_mem_id, 0 as libc::c_int, &mut sbuf);
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    if (*(*(*systab).vol[vol as usize]).vollab).clean as libc::c_int == 0 as libc::c_int
    {
        fprintf(
            __stderrp,
            b"WARNING: Volume was not dismounted properly!\n\0" as *const u8
                as *const libc::c_char,
        );
        (*(*systab).vol[vol as usize]).upto = 1 as libc::c_int as u_int;
    } else {
        (*(*(*systab).vol[vol as usize]).vollab).clean = 1 as libc::c_int as u_char;
        (*(*systab).vol[vol as usize]).map_dirty_flag = 1 as libc::c_int;
    }
    jobs = jobs / 10 as libc::c_int;
    if jobs < 2 as libc::c_int {
        jobs = 2 as libc::c_int;
    }
    if jobs > 20 as libc::c_int {
        jobs = 20 as libc::c_int;
    }
    (*(*systab).vol[vol as usize]).num_of_daemons = jobs;
    while SemOp(4 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int) != 0
    {}
    partab.vol_fds[vol as usize] = dbfd;
    indx = 0 as libc::c_int;
    while indx < jobs {
        i = DB_Daemon(indx, vol + 1 as libc::c_int);
        if i != 0 as libc::c_int {
            fprintf(
                __stderrp,
                b"**** Died on error - %s ***\n\n\0" as *const u8 as *const libc::c_char,
                strerror(i),
            );
            shmdt(systab as *const libc::c_void);
            return -(200 as libc::c_int + 200 as libc::c_int + i) as libc::c_short;
        }
        indx += 1;
        indx;
    }
    if (*(*(*systab).vol[vol as usize]).vollab).journal_requested as libc::c_int != 0
        && (*(*(*systab).vol[vol as usize]).vollab)
            .journal_file[0 as libc::c_int as usize] as libc::c_int != 0
    {
        let mut sb: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_birthtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        let mut jptr: off_t = 0;
        let mut jj: jrnrec = JRNREC {
            size: 0,
            action: 0,
            uci: 0,
            time: 0,
            name: VAR_U { var_q: 0 },
            slen: 0,
            key: [0; 256],
        };
        let mut jfd: libc::c_int = 0;
        while SemOp(2 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int)
            != 0
        {}
        (*(*(*systab).vol[vol as usize]).vollab)
            .journal_available = 0 as libc::c_int as u_char;
        i = stat(
            ((*(*(*systab).vol[vol as usize]).vollab).journal_file).as_mut_ptr(),
            &mut sb,
        );
        if i == -(1 as libc::c_int) && *__error() != 2 as libc::c_int {
            fprintf(
                __stderrp,
                b"Failed to access journal file: %s\n\0" as *const u8
                    as *const libc::c_char,
                ((*(*(*systab).vol[vol as usize]).vollab).journal_file).as_mut_ptr(),
            );
        } else {
            if i == -(1 as libc::c_int) {
                ClearJournal(vol);
            }
            jfd = open(
                ((*(*(*systab).vol[vol as usize]).vollab).journal_file).as_mut_ptr(),
                0x2 as libc::c_int,
            );
            if jfd == -(1 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"Failed to open journal file: %s\nerrno = %d\n\0" as *const u8
                        as *const libc::c_char,
                    ((*(*(*systab).vol[vol as usize]).vollab).journal_file).as_mut_ptr(),
                    *__error(),
                );
            } else {
                let mut temp: C2RustUnnamed = C2RustUnnamed { magic: 0 };
                lseek(jfd, 0 as libc::c_int as off_t, 0 as libc::c_int);
                *__error() = 0 as libc::c_int;
                i = read(
                    jfd,
                    (temp.tmp).as_mut_ptr() as *mut libc::c_void,
                    4 as libc::c_int as size_t,
                ) as libc::c_int;
                if i != 4 as libc::c_int
                    || temp.magic
                        != (4155766917 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    fprintf(
                        __stderrp,
                        b"Failed to open journal file: %s\nWRONG MAGIC\n\0" as *const u8
                            as *const libc::c_char,
                        ((*(*(*systab).vol[vol as usize]).vollab).journal_file)
                            .as_mut_ptr(),
                    );
                    close(jfd);
                } else {
                    i = read(
                        jfd,
                        &mut (**((*systab).vol).as_mut_ptr().offset(vol as isize))
                            .jrn_next as *mut off_t as *mut libc::c_void,
                        ::core::mem::size_of::<off_t>() as libc::c_ulong,
                    ) as libc::c_int;
                    if i as libc::c_ulong
                        != ::core::mem::size_of::<off_t>() as libc::c_ulong
                    {
                        fprintf(
                            __stderrp,
                            b"Failed to use journal file: %s\nRead failed - %d\n\0"
                                as *const u8 as *const libc::c_char,
                            ((*(*(*systab).vol[vol as usize]).vollab).journal_file)
                                .as_mut_ptr(),
                            *__error(),
                        );
                        close(jfd);
                    } else {
                        jptr = lseek(
                            jfd,
                            (*(*systab).vol[vol as usize]).jrn_next,
                            0 as libc::c_int,
                        );
                        if jptr != (*(*systab).vol[vol as usize]).jrn_next {
                            fprintf(
                                __stderrp,
                                b"Failed journal file: %s\nlseek failed - %d\n\0"
                                    as *const u8 as *const libc::c_char,
                                ((*(*(*systab).vol[vol as usize]).vollab).journal_file)
                                    .as_mut_ptr(),
                                *__error(),
                            );
                            close(jfd);
                        } else {
                            jj.action = 1 as libc::c_int as u_char;
                            jj.uci = 0 as libc::c_int as u_char;
                            jj.size = 12 as libc::c_int as u_short;
                            jj
                                .time = current_time(1 as libc::c_int as libc::c_short)
                                as u_int64;
                            i = write(
                                jfd,
                                &mut jj as *mut jrnrec as *const libc::c_void,
                                jj.size as size_t,
                            ) as libc::c_int;
                            if i == -(1 as libc::c_int) {
                                return -(200 as libc::c_int + 200 as libc::c_int
                                    + *__error()) as libc::c_short;
                            }
                            (*(*systab).vol[vol as usize]).jrn_next += jj.size as off_t;
                            lseek(jfd, 4 as libc::c_int as off_t, 0 as libc::c_int);
                            i = write(
                                jfd,
                                &mut (**((*systab).vol).as_mut_ptr().offset(vol as isize))
                                    .jrn_next as *mut off_t as *const libc::c_void,
                                ::core::mem::size_of::<off_t>() as libc::c_ulong,
                            ) as libc::c_int;
                            if i == -(1 as libc::c_int) {
                                return -(200 as libc::c_int + 200 as libc::c_int
                                    + *__error()) as libc::c_short;
                            }
                            i = close(jfd);
                            if i == -(1 as libc::c_int) {
                                return -(200 as libc::c_int + 200 as libc::c_int
                                    + *__error()) as libc::c_short;
                            }
                            (*(*(*systab).vol[vol as usize]).vollab)
                                .journal_available = 1 as libc::c_int as u_char;
                            printf(
                                b"Journaling started to %s\n\0" as *const u8
                                    as *const libc::c_char,
                                ((*(*(*systab).vol[vol as usize]).vollab).journal_file)
                                    .as_mut_ptr(),
                            );
                        }
                    }
                }
            }
        }
        SemOp(2 as libc::c_int, -(((*systab).maxjob).wrapping_neg() as libc::c_int));
    }
    SemOp(4 as libc::c_int, -(((*systab).maxjob).wrapping_neg() as libc::c_int));
    gptr = (*(*systab).vol[vol as usize]).gbd_head;
    ptr = (*(*systab).vol[vol as usize]).global_buf as *mut u_char;
    let mut j_1: u_int = 0 as libc::c_int as u_int;
    while j_1 < (*(*systab).vol[vol as usize]).num_gbd {
        let ref mut fresh0 = (*gptr.offset(j_1 as isize)).mem;
        *fresh0 = ptr as *mut DB_Block;
        ptr = ptr.offset((*(*(*systab).vol[vol as usize]).vollab).block_size as isize);
        if j_1
            < ((*(*systab).vol[vol as usize]).num_gbd)
                .wrapping_sub(1 as libc::c_int as u_int)
        {
            let ref mut fresh1 = (*gptr.offset(j_1 as isize)).next;
            *fresh1 = &mut *gptr
                .offset(j_1.wrapping_add(1 as libc::c_int as u_int) as isize)
                as *mut gbd;
        } else {
            let ref mut fresh2 = (*gptr.offset(j_1 as isize)).next;
            *fresh2 = 0 as *mut GBD;
        }
        (*(*systab).vol[vol as usize]).gbd_hash[1024 as libc::c_int as usize] = gptr;
        j_1 = j_1.wrapping_add(1);
        j_1;
    }
    Routine_Init(vol);
    i = close(dbfd);
    if i == -(1 as libc::c_int) {
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    dbfd = open(file, 0 as libc::c_int);
    if dbfd == -(1 as libc::c_int) {
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    partab.vol_fds[vol as usize] = dbfd;
    return 0 as libc::c_int as libc::c_short;
}
