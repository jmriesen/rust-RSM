#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    pub type RBD;
    static mut __stdinp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fflush(_: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn freopen(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn close(_: libc::c_int) -> libc::c_int;
    fn lseek(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    fn ctime(_: *const time_t) -> *mut libc::c_char;
    fn __error() -> *mut libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn kill(_: pid_t, _: libc::c_int) -> libc::c_int;
    fn shmctl(_: libc::c_int, _: libc::c_int, _: *mut __shmid_ds_new) -> libc::c_int;
    fn mkdir(_: *const libc::c_char, _: mode_t) -> libc::c_int;
    fn semctl(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    static mut curr_lock: libc::c_int;
    static mut volnum: libc::c_int;
    static mut chunk: *mut cstring;
    static mut record: *mut cstring;
    static mut idx: *mut u_short;
    static mut iidx: *mut libc::c_int;
    fn Free_GBD(free_0: *mut gbd);
    fn Align_record();
    fn Free_block(blknum: libc::c_int);
    fn current_time(local: libc::c_short) -> time_t;
    fn ForkIt(cft: libc::c_int) -> libc::c_int;
    fn panic(msg: *mut libc::c_char);
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
    fn ic_map(flag: libc::c_int);
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_gid_t = __uint32_t;
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
pub type mode_t = __darwin_mode_t;
pub type gid_t = __darwin_gid_t;
pub type time_t = __darwin_time_t;
pub type key_t = __int32_t;
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
pub static mut dbfd: libc::c_int = 0;
#[no_mangle]
pub static mut myslot: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn DB_Daemon(
    mut slot: libc::c_int,
    mut vol: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut fit: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    let mut logfile: [libc::c_char; 274] = [0; 274];
    let mut t: time_t = 0;
    volnum = vol;
    fit = ForkIt(-(1 as libc::c_int));
    if fit > 0 as libc::c_int {
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .wd_tab[slot as usize]
            .pid = fit;
        return 0 as libc::c_int;
    }
    if fit < 0 as libc::c_int {
        return *__error();
    }
    curr_lock = 0 as libc::c_int;
    k = strlen(
        ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).file_name).as_mut_ptr(),
    ) as libc::c_int;
    i = k - 1 as libc::c_int;
    while (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).file_name[i as usize]
        as libc::c_int != '/' as i32 && i > -(1 as libc::c_int)
    {
        i -= 1;
        i;
    }
    strncpy(
        logfile.as_mut_ptr(),
        ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).file_name).as_mut_ptr(),
        (i + 1 as libc::c_int) as libc::c_ulong,
    );
    logfile[(i + 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    sprintf(
        &mut *logfile
            .as_mut_ptr()
            .offset(
                (strlen
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                    ) -> libc::c_ulong)(logfile.as_mut_ptr()) as isize,
            ) as *mut libc::c_char,
        b"log/\0" as *const u8 as *const libc::c_char,
    );
    if mkdir(
        logfile.as_mut_ptr(),
        (0o700 as libc::c_int | 0o40 as libc::c_int | 0o10 as libc::c_int
            | 0o4 as libc::c_int | 0o1 as libc::c_int) as mode_t,
    ) == -(1 as libc::c_int)
    {
        if *__error() != 17 as libc::c_int {
            return *__error();
        }
    }
    sprintf(
        &mut *logfile
            .as_mut_ptr()
            .offset(
                (strlen
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                    ) -> libc::c_ulong)(logfile.as_mut_ptr()) as isize,
            ) as *mut libc::c_char,
        b"daemon-%d.log\0" as *const u8 as *const libc::c_char,
        volnum,
    );
    myslot = slot;
    if close(partab.vol_fds[(volnum - 1 as libc::c_int) as usize]) == -(1 as libc::c_int)
    {
        return *__error();
    }
    if (freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        __stdinp,
    ))
        .is_null()
    {
        return *__error();
    }
    if (freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
        __stdoutp,
    ))
        .is_null()
    {
        return *__error();
    }
    if (freopen(
        logfile.as_mut_ptr(),
        b"a\0" as *const u8 as *const libc::c_char,
        __stderrp,
    ))
        .is_null()
    {
        return *__error();
    }
    pid = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .wd_tab[slot as usize]
        .pid;
    dbfd = open(
        ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).file_name).as_mut_ptr(),
        0x2 as libc::c_int,
    );
    t = current_time(0 as libc::c_int as libc::c_short);
    if dbfd < 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s [%6d]: Daemon %2d failed to attach to %s - exiting \n\0" as *const u8
                as *const libc::c_char,
            strtok(ctime(&mut t), b"\n\0" as *const u8 as *const libc::c_char),
            pid,
            myslot,
            ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).file_name)
                .as_mut_ptr(),
        );
        fflush(__stderrp);
        return *__error();
    }
    fprintf(
        __stderrp,
        b"%s [%6d]: Daemon %2d started and attached to %s\n\0" as *const u8
            as *const libc::c_char,
        strtok(ctime(&mut t), b"\n\0" as *const u8 as *const libc::c_char),
        pid,
        myslot,
        ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).file_name).as_mut_ptr(),
    );
    fflush(__stderrp);
    if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).upto != 0 && myslot == 0 {
        ic_map(-(3 as libc::c_int));
    }
    i = sleep(2 as libc::c_int as libc::c_uint) as libc::c_int;
    if i != 0 as libc::c_int {
        sleep(i as libc::c_uint);
    }
    loop {
        do_daemon();
        sleep(1 as libc::c_int as libc::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_daemon() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    let mut file_off: off_t = 0;
    let mut t: time_t = 0;
    loop {
        daemon_check();
        if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .wd_tab[myslot as usize]
            .doing == 0 as libc::c_int
        {
            if myslot == 0
                && (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map_dirty_flag
                    != 0
            {
                file_off = lseek(dbfd, 0 as libc::c_int as off_t, 0 as libc::c_int);
                if file_off == -(1 as libc::c_int) as off_t {
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .diskerrors = ((*(*systab)
                        .vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .diskerrors)
                        .wrapping_add(1);
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .diskerrors;
                    panic(
                        b"do_daemon: lseek() to start of file failed\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                i = write(
                    dbfd,
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab
                        as *const libc::c_void,
                    (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                        .header_bytes as size_t,
                ) as libc::c_int;
                if i == -(1 as libc::c_int) {
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .diskerrors = ((*(*systab)
                        .vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .diskerrors)
                        .wrapping_add(1);
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .diskerrors;
                    panic(
                        b"do_daemon: write() map block failed\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .stats
                    .phywt = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .stats
                    .phywt)
                    .wrapping_add(1);
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.phywt;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .map_dirty_flag = 0 as libc::c_int;
            }
            if myslot == 0
                && (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).writelock
                    < 0 as libc::c_int
            {
                loop {
                    i = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .dirtyQ[(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .dirtyQr as usize] != 0 as *mut libc::c_void as *mut GBD)
                        as libc::c_int;
                    i = (i != 0
                        || (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .garbQ[(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .garbQr as usize] != 0 as libc::c_int as u_int)
                        as libc::c_int;
                    j = 1 as libc::c_int;
                    while j
                        < (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .num_of_daemons
                    {
                        i = (i != 0
                            || (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                                .wd_tab[myslot as usize]
                                .doing != 0 as libc::c_int) as libc::c_int;
                        j += 1;
                        j;
                    }
                    if i == 0 {
                        break;
                    }
                    daemon_check();
                    sleep(1 as libc::c_int as libc::c_uint);
                }
                sleep(1 as libc::c_int as libc::c_uint);
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .writelock = abs(
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).writelock,
                );
            }
            while SemOp(
                4 as libc::c_int,
                ((*systab).maxjob).wrapping_neg() as libc::c_int,
            ) != 0
            {}
            if !((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .dirtyQ[(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).dirtyQr
                as usize])
                .is_null()
            {
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .wd_tab[myslot as usize]
                    .currmsg
                    .gbddata = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .dirtyQ[(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .dirtyQr as usize];
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .wd_tab[myslot as usize]
                    .doing = 2 as libc::c_int;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .dirtyQ[(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .dirtyQr as usize] = 0 as *mut GBD;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).dirtyQr += 1;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).dirtyQr;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).dirtyQr
                    &= 1024 as libc::c_int - 1 as libc::c_int;
            } else if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .garbQ[(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQr
                as usize] != 0
            {
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .wd_tab[myslot as usize]
                    .currmsg
                    .intdata = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .garbQ[(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQr
                    as usize];
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .wd_tab[myslot as usize]
                    .doing = 3 as libc::c_int;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .garbQ[(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQr
                    as usize] = 0 as libc::c_int as u_int;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQr += 1;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQr;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQr
                    &= 8192 as libc::c_int - 1 as libc::c_int;
            }
            SemOp(4 as libc::c_int, -(((*systab).maxjob).wrapping_neg() as libc::c_int));
        }
        pid = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .wd_tab[myslot as usize]
            .pid;
        if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .wd_tab[myslot as usize]
            .doing == 0 as libc::c_int
        {
            if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).dismount_flag != 0
            {
                if myslot != 0 {
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .wd_tab[myslot as usize]
                        .pid = 0 as libc::c_int;
                    t = current_time(0 as libc::c_int as libc::c_short);
                    fprintf(
                        __stderrp,
                        b"%s [%6d]: Daemon %2d stopped and detached from %s\n\0"
                            as *const u8 as *const libc::c_char,
                        strtok(
                            ctime(&mut t),
                            b"\n\0" as *const u8 as *const libc::c_char,
                        ),
                        pid,
                        myslot,
                        ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .file_name)
                            .as_mut_ptr(),
                    );
                    fflush(__stderrp);
                    exit(0 as libc::c_int);
                }
                do_dismount();
                exit(0 as libc::c_int);
            } else {
                return
            }
        }
        if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .wd_tab[myslot as usize]
            .doing == 2 as libc::c_int
        {
            do_write();
        } else {
            if !((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .wd_tab[myslot as usize]
                .doing == 3 as libc::c_int)
            {
                break;
            }
            do_garb();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_dismount() {
    let mut ret: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
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
    let mut semvals: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut t: time_t = 0;
    ret = shmctl(
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).shm_id,
        0 as libc::c_int,
        &mut sbuf,
    );
    if ret == -(1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"errno = %d %s\n\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
    }
    if volnum == 1 as libc::c_int {
        let mut i: u_int = 0 as libc::c_int as u_int;
        while i < (*systab).maxjob {
            pid = (*((*systab).jobtab).offset(i as isize)).pid;
            if pid != 0 {
                if kill(pid, 15 as libc::c_int) != 0 {
                    (*((*systab).jobtab).offset(i as isize))
                        .trap = (1 as libc::c_uint) << 15 as libc::c_int;
                    (*((*systab).jobtab).offset(i as isize))
                        .attention = 1 as libc::c_int;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    let mut i_0: u_int = 0 as libc::c_int as u_int;
    while i_0 < (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).num_gbd {
        if (*((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).gbd_head)
            .offset(i_0 as isize))
            .block != 0
            && (*((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).gbd_head)
                .offset(i_0 as isize))
                .last_accessed != 0 as libc::c_int as time_t
            && !((*((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).gbd_head)
                .offset(i_0 as isize))
                .dirty)
                .is_null()
        {
            let ref mut fresh0 = (*((*(*systab)
                .vol[(volnum - 1 as libc::c_int) as usize])
                .gbd_head)
                .offset(i_0 as isize))
                .dirty;
            *fresh0 = &mut *((**((*systab).vol)
                .as_mut_ptr()
                .offset((volnum - 1 as libc::c_int) as isize))
                .gbd_head)
                .offset(i_0 as isize) as *mut GBD;
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .wd_tab[0 as libc::c_int as usize]
                .currmsg
                .gbddata = &mut *((**((*systab).vol)
                .as_mut_ptr()
                .offset((volnum - 1 as libc::c_int) as isize))
                .gbd_head)
                .offset(i_0 as isize) as *mut GBD;
            do_write();
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    cnt = 1 as libc::c_int;
    while cnt != 0 {
        cnt = 0 as libc::c_int;
        SemOp(4 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int);
        let mut j: libc::c_int = 1 as libc::c_int;
        while j < (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).num_of_daemons {
            if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .wd_tab[j as usize]
                .pid != 0
            {
                if kill(
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .wd_tab[j as usize]
                        .pid,
                    0 as libc::c_int,
                ) == -(1 as libc::c_int)
                {
                    if *__error() == 3 as libc::c_int {
                        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .wd_tab[j as usize]
                            .pid = 0 as libc::c_int;
                    } else {
                        cnt = 1 as libc::c_int;
                    }
                } else {
                    cnt = 1 as libc::c_int;
                }
            }
            j += 1;
            j;
        }
        SemOp(4 as libc::c_int, -(((*systab).maxjob).wrapping_neg() as libc::c_int));
        if cnt != 0 {
            sleep(1 as libc::c_int as libc::c_uint);
        }
    }
    pid = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .wd_tab[myslot as usize]
        .pid;
    t = current_time(0 as libc::c_int as libc::c_short);
    fprintf(
        __stderrp,
        b"%s [%6d]: Daemon %2d writing out clean flag as clean\n\0" as *const u8
            as *const libc::c_char,
        strtok(ctime(&mut t), b"\n\0" as *const u8 as *const libc::c_char),
        pid,
        myslot,
    );
    (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
        .clean = 1 as libc::c_int as u_char;
    lseek(dbfd, 0 as libc::c_int as off_t, 0 as libc::c_int);
    ret = write(
        dbfd,
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab
            as *const libc::c_void,
        (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).header_bytes
            as size_t,
    ) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"errno = %d %s\n\0" as *const u8 as *const libc::c_char,
            *__error(),
            strerror(*__error()),
        );
    }
    if volnum == 1 as libc::c_int {
        ret = semctl((*systab).sem_id, 0 as libc::c_int, 0 as libc::c_int, semvals);
        if ret == -(1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"errno = %d %s\n\0" as *const u8 as *const libc::c_char,
                *__error(),
                strerror(*__error()),
            );
        }
    }
    t = current_time(0 as libc::c_int as libc::c_short);
    fprintf(
        __stderrp,
        b"%s [%6d]: Daemon %2d stopped and detached from %s\n\0" as *const u8
            as *const libc::c_char,
        strtok(ctime(&mut t), b"\n\0" as *const u8 as *const libc::c_char),
        pid,
        myslot,
        ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).file_name).as_mut_ptr(),
    );
    fflush(__stderrp);
}
#[no_mangle]
pub unsafe extern "C" fn do_write() {
    let mut file_off: off_t = 0;
    let mut i: libc::c_int = 0;
    let mut gbdptr: *mut gbd = 0 as *mut gbd;
    let mut lastptr: *mut gbd = 0 as *mut gbd;
    gbdptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .wd_tab[myslot as usize]
        .currmsg
        .gbddata;
    if gbdptr.is_null() {
        panic(
            b"Daemon: write message GBD is NULL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if curr_lock == 0 as libc::c_int {
        SemOp(2 as libc::c_int, -(1 as libc::c_int));
    }
    loop {
        if (*gbdptr).last_accessed == 0 as libc::c_int as time_t {
            (*gbdptr).block = 0 as libc::c_int as u_int;
        } else {
            file_off = (*gbdptr).block as off_t - 1 as libc::c_int as off_t;
            file_off = file_off
                * (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                    .block_size as off_t
                + (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                    .header_bytes as off_t;
            file_off = lseek(dbfd, file_off, 0 as libc::c_int);
            if file_off < 1 as libc::c_int as off_t {
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .stats
                    .diskerrors = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .stats
                    .diskerrors)
                    .wrapping_add(1);
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.diskerrors;
                panic(
                    b"lseek failed in Write_Chain()!!\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            i = write(
                dbfd,
                (*gbdptr).mem as *const libc::c_void,
                (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                    .block_size as size_t,
            ) as libc::c_int;
            if i < 0 as libc::c_int {
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .stats
                    .diskerrors = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .stats
                    .diskerrors)
                    .wrapping_add(1);
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.diskerrors;
                panic(
                    b"write failed in Write_Chain()!!\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .stats
                .phywt = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .stats
                .phywt)
                .wrapping_add(1);
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.phywt;
        }
        if ((*gbdptr).dirty).is_null() {
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .wd_tab[myslot as usize]
                .currmsg
                .gbddata = 0 as *mut GBD;
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .wd_tab[myslot as usize]
                .doing = 0 as libc::c_int;
            break;
        } else {
            lastptr = gbdptr;
            gbdptr = (*gbdptr).dirty;
            if lastptr != gbdptr {
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .wd_tab[myslot as usize]
                    .currmsg
                    .gbddata = gbdptr;
            } else {
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .wd_tab[myslot as usize]
                    .currmsg
                    .gbddata = 0 as *mut GBD;
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .wd_tab[myslot as usize]
                    .doing = 0 as libc::c_int;
            }
            (*lastptr).dirty = 0 as *mut GBD;
            if lastptr == gbdptr {
                break;
            }
        }
    }
    SemOp(2 as libc::c_int, -curr_lock);
}
#[no_mangle]
pub unsafe extern "C" fn do_garb() {
    let mut gb: u_int = 0;
    if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .wd_tab[myslot as usize]
        .currmsg
        .intdata == 0 as libc::c_int as u_int
    {
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .wd_tab[myslot as usize]
            .doing = 0 as libc::c_int;
        return;
    }
    gb = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .wd_tab[myslot as usize]
        .currmsg
        .intdata;
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .wd_tab[myslot as usize]
        .currmsg
        .intdata = 0 as libc::c_int as u_int;
    do_zot(gb);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .wd_tab[myslot as usize]
        .doing = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn do_zot(mut gb: u_int) -> libc::c_int {
    let mut i: u_int = 0;
    let mut ret: libc::c_int = 0;
    let mut Idx: libc::c_int = 0;
    let mut bptr: *mut DB_Block = 0 as *mut DB_Block;
    let mut file_off: off_t = 0;
    let mut file_ret: off_t = 0;
    let mut typ: libc::c_int = 0;
    let mut zot_data: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut gbd = 0 as *mut gbd;
    bptr = malloc(
        (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).block_size
            as libc::c_ulong,
    ) as *mut DB_Block;
    if bptr.is_null() {
        fprintf(
            __stderrp,
            b"do_zot: malloc for block %u failed\n\0" as *const u8
                as *const libc::c_char,
            gb,
        );
        fflush(__stderrp);
        return -(1 as libc::c_int);
    }
    file_off = gb as off_t - 1 as libc::c_int as off_t;
    file_off = file_off
        * (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).block_size
            as off_t
        + (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).header_bytes
            as off_t;
    while SemOp(2 as libc::c_int, -(1 as libc::c_int)) != 0 {}
    ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .gbd_hash[(gb & (1024 as libc::c_int - 1 as libc::c_int) as u_int) as usize];
    while !ptr.is_null() {
        if (*ptr).block == gb {
            memcpy(
                bptr as *mut libc::c_void,
                (*ptr).mem as *const libc::c_void,
                (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                    .block_size as libc::c_ulong,
            );
            (*ptr).last_accessed = 0 as libc::c_int as time_t;
            break;
        } else {
            ptr = (*ptr).next;
        }
    }
    SemOp(2 as libc::c_int, -curr_lock);
    if ptr.is_null() {
        file_ret = lseek(dbfd, file_off, 0 as libc::c_int);
        if file_ret < 1 as libc::c_int as off_t {
            fprintf(
                __stderrp,
                b"do_zot: seek to block %u failed\n\0" as *const u8
                    as *const libc::c_char,
                gb,
            );
            fflush(__stderrp);
            free(bptr as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        ret = read(
            dbfd,
            bptr as *mut libc::c_void,
            (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).block_size
                as size_t,
        ) as libc::c_int;
        if ret < 0 as libc::c_int {
            fprintf(
                __stderrp,
                b"do_zot: read of block %u failed\n\0" as *const u8
                    as *const libc::c_char,
                gb,
            );
            fflush(__stderrp);
            free(bptr as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
    }
    typ = (*bptr).0.type_0 as libc::c_int;
    if !(typ > 64 as libc::c_int) {
        Idx = (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
            / 2 as libc::c_int as u_int) as libc::c_int;
        while Idx <= (*bptr).0.last_idx as libc::c_int {
            idx = bptr as *mut u_short;
            iidx = bptr as *mut libc::c_int;
            chunk = &mut *iidx.offset(*idx.offset(Idx as isize) as isize)
                as *mut libc::c_int as *mut cstring;
            record = &mut *((*chunk).buf)
                .as_mut_ptr()
                .offset(
                    (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                        as libc::c_int + 2 as libc::c_int) as isize,
                ) as *mut u_char as *mut cstring;
            Align_record();
            i = *(record as *mut u_int);
            if zot_data != 0 {
                file_ret = i as off_t - 1 as libc::c_int as off_t;
                file_ret = file_ret
                    * (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                        .block_size as off_t
                    + (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                        .header_bytes as off_t;
                file_ret = lseek(dbfd, file_ret, 0 as libc::c_int);
                if file_ret < 1 as libc::c_int as off_t {
                    fprintf(
                        __stderrp,
                        b"do_zot: seek to block %u failed\n\0" as *const u8
                            as *const libc::c_char,
                        i,
                    );
                    fflush(__stderrp);
                } else {
                    ret = write(
                        dbfd,
                        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                            .zero_block,
                        (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                            .block_size as size_t,
                    ) as libc::c_int;
                    if ret == -(1 as libc::c_int) {
                        fprintf(
                            __stderrp,
                            b"do_zot: zero of block %u failed\n\0" as *const u8
                                as *const libc::c_char,
                            i,
                        );
                        fflush(__stderrp);
                    }
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .phywt = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .phywt)
                        .wrapping_add(1);
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.phywt;
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .logwt = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                        .stats
                        .logwt)
                        .wrapping_add(1);
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.logwt;
                    do_free(i);
                }
            } else {
                ret = do_zot(i);
                if ret > 64 as libc::c_int {
                    zot_data = 1 as libc::c_int;
                }
            }
            Idx += 1;
            Idx;
        }
    }
    file_ret = lseek(dbfd, file_off, 0 as libc::c_int);
    if file_ret < 1 as libc::c_int as off_t {
        fprintf(
            __stderrp,
            b"do_zot: zeroing seek to block %u failed\n\0" as *const u8
                as *const libc::c_char,
            gb,
        );
        fflush(__stderrp);
        free(bptr as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    ret = write(
        dbfd,
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).zero_block,
        (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).block_size
            as size_t,
    ) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"do_zot: zero of block %u failed\n\0" as *const u8 as *const libc::c_char,
            gb,
        );
        fflush(__stderrp);
        typ = -(1 as libc::c_int);
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .phywt = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.phywt)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.phywt;
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .logwt = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.logwt)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.logwt;
    free(bptr as *mut libc::c_void);
    do_free(gb);
    return typ;
}
#[no_mangle]
pub unsafe extern "C" fn do_free(mut gb: u_int) {
    let mut ptr: *mut gbd = 0 as *mut gbd;
    loop {
        daemon_check();
        if SemOp(2 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int) == 0
        {
            break;
        }
        sleep(1 as libc::c_int as libc::c_uint);
    }
    Free_block(gb as libc::c_int);
    ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .gbd_hash[(gb & (1024 as libc::c_int - 1 as libc::c_int) as u_int) as usize];
    while !ptr.is_null() {
        if (*ptr).block == gb {
            if (*ptr).dirty < 3 as libc::c_int as *mut gbd {
                Free_GBD(ptr);
            } else {
                (*ptr).last_accessed = 0 as libc::c_int as time_t;
            }
            break;
        } else {
            ptr = (*ptr).next;
        }
    }
    SemOp(2 as libc::c_int, -curr_lock);
}
#[no_mangle]
pub unsafe extern "C" fn daemon_check() {
    let mut i: libc::c_int = 0;
    while SemOp(4 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int) != 0
    {}
    i = 0 as libc::c_int;
    while i < (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).num_of_daemons {
        if i != myslot {
            if kill(
                (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                    .wd_tab[i as usize]
                    .pid,
                0 as libc::c_int,
            ) == -(1 as libc::c_int)
            {
                if *__error() == 3 as libc::c_int {
                    DB_Daemon(i, volnum);
                }
            }
        }
        i += 1;
        i;
    }
    SemOp(4 as libc::c_int, -(((*systab).maxjob).wrapping_neg() as libc::c_int));
}
