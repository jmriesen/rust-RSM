#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type __sFILEX;
    pub type RBD;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getpriority(_: libc::c_int, _: id_t) -> libc::c_int;
    fn setpriority(_: libc::c_int, _: id_t, _: libc::c_int) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn atol(_: *const libc::c_char) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn setuid(_: uid_t) -> libc::c_int;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    fn shmctl(_: libc::c_int, _: libc::c_int, _: *mut __shmid_ds_new) -> libc::c_int;
    fn semctl(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    fn kill(_: pid_t, _: libc::c_int) -> libc::c_int;
    fn getpwuid(_: uid_t) -> *mut passwd;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __error() -> *mut libc::c_int;
    fn tcsetattr(_: libc::c_int, _: libc::c_int, _: *const termios) -> libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn DB_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn DB_Data(var: *mut mvar, buf: *mut u_char) -> libc::c_short;
    fn DB_Kill(var: *mut mvar) -> libc::c_short;
    fn DB_Mount(
        file: *mut libc::c_char,
        vol: libc::c_int,
        gmb: u_int,
        rmb: u_int,
    ) -> libc::c_short;
    fn DB_Order(var: *mut mvar, buf: *mut u_char, dir: libc::c_int) -> libc::c_short;
    fn DB_Free(vol: libc::c_int) -> libc::c_int;
    fn DB_UCISet(vol: libc::c_int, uci: libc::c_int, name: var_u) -> libc::c_short;
    fn DB_UCIKill(vol: libc::c_int, uci: libc::c_int) -> libc::c_short;
    fn DB_Expand(vol: libc::c_int, vsiz: u_int) -> libc::c_short;
    fn DB_Dismount(vol: libc::c_int) -> libc::c_int;
    fn ClearJournal(vol: libc::c_int);
    fn DB_StopJournal(vol: libc::c_int, action: u_char);
    fn DB_GetFlags(var: *mut mvar) -> libc::c_int;
    fn DB_SetFlags(var: *mut mvar, flags: libc::c_int) -> libc::c_int;
    fn cstringtoi(str: *mut cstring) -> libc::c_int;
    fn cstringtob(str: *mut cstring) -> libc::c_int;
    fn itocstring(buf: *mut u_char, n: libc::c_int) -> u_short;
    fn uitocstring(buf: *mut u_char, n: u_int) -> u_short;
    fn Dstack1x(
        ret_buffer: *mut u_char,
        level: libc::c_int,
        job: libc::c_int,
    ) -> libc::c_short;
    fn Dstack2x(
        ret_buffer: *mut u_char,
        level: libc::c_int,
        code: *mut cstring,
        job: libc::c_int,
    ) -> libc::c_int;
    fn UTIL_Key_Extract(
        key: *mut u_char,
        str: *mut u_char,
        cnt: *mut libc::c_int,
    ) -> libc::c_short;
    fn UTIL_String_Mvar(
        var: *mut mvar,
        str: *mut u_char,
        max_subs: libc::c_int,
    ) -> libc::c_short;
    fn UTIL_MvarFromCStr(src: *mut cstring, var: *mut mvar) -> libc::c_short;
    fn mcopy(src: *mut u_char, dst: *mut u_char, bytes: libc::c_int) -> libc::c_int;
    fn CleanJob(job: libc::c_int);
    fn Routine_Delete(routine: var_u, uci: libc::c_int);
    fn UTIL_mvartolock(var: *mut mvar, buf: *mut u_char) -> libc::c_short;
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
    fn LCK_Order(ent: *mut cstring, buf: *mut u_char, dir: libc::c_int) -> libc::c_short;
    fn LCK_Get(ent: *mut cstring, buf: *mut u_char) -> libc::c_short;
    fn LCK_Kill(ent: *mut cstring) -> libc::c_short;
    static mut tty_settings: termios;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
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
pub type pid_t = __darwin_pid_t;
pub type id_t = __darwin_id_t;
pub type uid_t = __darwin_uid_t;
pub type mode_t = __darwin_mode_t;
pub type gid_t = __darwin_gid_t;
pub type time_t = __darwin_time_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_change: __darwin_time_t,
    pub pw_class: *mut libc::c_char,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
    pub pw_expire: __darwin_time_t,
}
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
pub type tcflag_t = libc::c_ulong;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_cc: [cc_t; 20],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isalpha(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x100 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn iscntrl(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x200 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn toupper(mut _c: libc::c_int) -> libc::c_int {
    return __toupper(_c);
}
#[inline]
unsafe extern "C" fn var_empty(mut var: var_u) -> u_int {
    if var.var_q == 0 as libc::c_int as u_int64 {
        return 1 as libc::c_int as u_int
    } else {
        return 0 as libc::c_int as u_int
    };
}
#[export_name = "priv"]
pub unsafe extern "C" fn priv_0() -> libc::c_int {
    return ((*partab.jobtab).priv_0 as libc::c_int != 0
        || (*partab.jobtab)
            .dostk[(*partab.jobtab).cur_do as usize]
            .rounam
            .var_cu[0 as libc::c_int as usize] as libc::c_int == '%' as i32)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SS_Norm(mut var: *mut mvar) -> libc::c_short {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        (*var)
            .name
            .var_cu[i
            as usize] = toupper((*var).name.var_cu[i as usize] as libc::c_int) as u_char;
        i += 1;
        i;
    }
    match (*var).name.var_cu[1 as libc::c_int as usize] as libc::c_int {
        67 => {
            if (*var).name.var_cu[2 as libc::c_int as usize] as libc::c_int
                == '\0' as i32
                || memcmp(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut u_char
                        as *const libc::c_void,
                    b"CHARACTER\0\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let mut var_i: u_int = 0 as libc::c_int as u_int;
                while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*var).name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
                    var_i = var_i.wrapping_add(1);
                    var_i;
                }
                memcpy(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut u_char
                        as *mut libc::c_void,
                    b"$CHARACTER\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    10 as libc::c_int as libc::c_ulong,
                );
                if (*var).uci as libc::c_int == 0 as libc::c_int {
                    (*var).uci = 1 as libc::c_int as u_char;
                }
                if (*var).volset as libc::c_int == 0 as libc::c_int {
                    (*var).volset = 1 as libc::c_int as u_char;
                }
                if (*var).uci as libc::c_int != 1 as libc::c_int {
                    return -(59 as libc::c_int) as libc::c_short;
                }
                if (*var).volset as libc::c_int != 1 as libc::c_int {
                    return -(59 as libc::c_int) as libc::c_short;
                }
                return 0 as libc::c_int as libc::c_short;
            }
            return -(60 as libc::c_int) as libc::c_short;
        }
        68 => {
            if (*var).name.var_cu[2 as libc::c_int as usize] as libc::c_int
                == '\0' as i32
                || memcmp(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut u_char
                        as *const libc::c_void,
                    b"DEVICE\0\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let mut var_i_0: u_int = 0 as libc::c_int as u_int;
                while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*var).name.var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
                    var_i_0 = var_i_0.wrapping_add(1);
                    var_i_0;
                }
                memcpy(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut u_char
                        as *mut libc::c_void,
                    b"$DEVICE\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    7 as libc::c_int as libc::c_ulong,
                );
                if (*var).uci as libc::c_int == 0 as libc::c_int {
                    (*var).uci = 1 as libc::c_int as u_char;
                }
                if (*var).volset as libc::c_int == 0 as libc::c_int {
                    (*var).volset = 1 as libc::c_int as u_char;
                }
                if (*var).uci as libc::c_int != 1 as libc::c_int {
                    return -(59 as libc::c_int) as libc::c_short;
                }
                if (*var).volset as libc::c_int != 1 as libc::c_int {
                    return -(59 as libc::c_int) as libc::c_short;
                }
                return 0 as libc::c_int as libc::c_short;
            }
            return -(60 as libc::c_int) as libc::c_short;
        }
        71 => {
            if (*var).name.var_cu[2 as libc::c_int as usize] as libc::c_int
                == '\0' as i32
                || memcmp(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut u_char
                        as *const libc::c_void,
                    b"GLOBAL\0\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let mut var_i_1: u_int = 0 as libc::c_int as u_int;
                while var_i_1 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*var).name.var_qu[var_i_1 as usize] = 0 as libc::c_int as u_int64;
                    var_i_1 = var_i_1.wrapping_add(1);
                    var_i_1;
                }
                memcpy(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut u_char
                        as *mut libc::c_void,
                    b"$GLOBAL\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    7 as libc::c_int as libc::c_ulong,
                );
                if (*var).uci as libc::c_int == 0 as libc::c_int {
                    (*var).uci = (*partab.jobtab).uci;
                }
                if (*var).volset as libc::c_int == 0 as libc::c_int {
                    (*var).volset = (*partab.jobtab).vol;
                }
                return 0 as libc::c_int as libc::c_short;
            }
            return -(60 as libc::c_int) as libc::c_short;
        }
        74 => {
            if (*var).name.var_cu[2 as libc::c_int as usize] as libc::c_int
                == '\0' as i32
                || memcmp(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut u_char
                        as *const libc::c_void,
                    b"JOB\0\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let mut var_i_2: u_int = 0 as libc::c_int as u_int;
                while var_i_2 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*var).name.var_qu[var_i_2 as usize] = 0 as libc::c_int as u_int64;
                    var_i_2 = var_i_2.wrapping_add(1);
                    var_i_2;
                }
                memcpy(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut u_char
                        as *mut libc::c_void,
                    b"$JOB\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    4 as libc::c_int as libc::c_ulong,
                );
                if (*var).uci as libc::c_int == 0 as libc::c_int {
                    (*var).uci = 1 as libc::c_int as u_char;
                }
                if (*var).volset as libc::c_int == 0 as libc::c_int {
                    (*var).volset = 1 as libc::c_int as u_char;
                }
                if (*var).uci as libc::c_int != 1 as libc::c_int {
                    return -(59 as libc::c_int) as libc::c_short;
                }
                if (*var).volset as libc::c_int != 1 as libc::c_int {
                    return -(59 as libc::c_int) as libc::c_short;
                }
                return 0 as libc::c_int as libc::c_short;
            }
            return -(60 as libc::c_int) as libc::c_short;
        }
        76 => {
            if (*var).name.var_cu[2 as libc::c_int as usize] as libc::c_int
                == '\0' as i32
                || memcmp(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut u_char
                        as *const libc::c_void,
                    b"LOCK\0\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let mut var_i_3: u_int = 0 as libc::c_int as u_int;
                while var_i_3 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*var).name.var_qu[var_i_3 as usize] = 0 as libc::c_int as u_int64;
                    var_i_3 = var_i_3.wrapping_add(1);
                    var_i_3;
                }
                memcpy(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut u_char
                        as *mut libc::c_void,
                    b"$LOCK\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    5 as libc::c_int as libc::c_ulong,
                );
                if (*var).uci as libc::c_int == 0 as libc::c_int {
                    (*var).uci = 1 as libc::c_int as u_char;
                }
                if (*var).volset as libc::c_int == 0 as libc::c_int {
                    (*var).volset = (*partab.jobtab).lvol;
                }
                if (*var).uci as libc::c_int != 1 as libc::c_int {
                    return -(59 as libc::c_int) as libc::c_short;
                }
                return 0 as libc::c_int as libc::c_short;
            }
            return -(60 as libc::c_int) as libc::c_short;
        }
        82 => {
            if (*var).name.var_cu[2 as libc::c_int as usize] as libc::c_int
                == '\0' as i32
                || memcmp(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut u_char
                        as *const libc::c_void,
                    b"ROUTINE\0\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let mut var_i_4: u_int = 0 as libc::c_int as u_int;
                while var_i_4 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*var).name.var_qu[var_i_4 as usize] = 0 as libc::c_int as u_int64;
                    var_i_4 = var_i_4.wrapping_add(1);
                    var_i_4;
                }
                memcpy(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut u_char
                        as *mut libc::c_void,
                    b"$ROUTINE\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    8 as libc::c_int as libc::c_ulong,
                );
                if (*var).volset as libc::c_int == 0 as libc::c_int {
                    (*var).volset = (*partab.jobtab).rvol;
                }
                if (*var).uci as libc::c_int == 0 as libc::c_int {
                    if (*var).key[1 as libc::c_int as usize] as libc::c_int == '%' as i32
                    {
                        (*var).uci = 1 as libc::c_int as u_char;
                    } else {
                        (*var).uci = (*partab.jobtab).ruci;
                    }
                }
                return 0 as libc::c_int as libc::c_short;
            }
            return -(60 as libc::c_int) as libc::c_short;
        }
        83 => {
            if (*var).name.var_cu[2 as libc::c_int as usize] as libc::c_int
                == '\0' as i32
                || memcmp(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut u_char
                        as *const libc::c_void,
                    b"SYSTEM\0\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let mut var_i_5: u_int = 0 as libc::c_int as u_int;
                while var_i_5 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*var).name.var_qu[var_i_5 as usize] = 0 as libc::c_int as u_int64;
                    var_i_5 = var_i_5.wrapping_add(1);
                    var_i_5;
                }
                memcpy(
                    &mut *((*var).name.var_cu)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut u_char
                        as *mut libc::c_void,
                    b"$SYSTEM\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    7 as libc::c_int as libc::c_ulong,
                );
                if (*var).uci as libc::c_int == 0 as libc::c_int {
                    (*var).uci = 1 as libc::c_int as u_char;
                }
                if (*var).volset as libc::c_int == 0 as libc::c_int {
                    (*var).volset = 1 as libc::c_int as u_char;
                }
                if (*var).uci as libc::c_int != 1 as libc::c_int {
                    return -(59 as libc::c_int) as libc::c_short;
                }
                if (*var).volset as libc::c_int != 1 as libc::c_int {
                    return -(59 as libc::c_int) as libc::c_short;
                }
                return 0 as libc::c_int as libc::c_short;
            }
            return -(60 as libc::c_int) as libc::c_short;
        }
        _ => return -(60 as libc::c_int) as libc::c_short,
    };
}
#[no_mangle]
pub unsafe extern "C" fn SS_Get(
    mut var: *mut mvar,
    mut buf: *mut u_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut rounam: *mut var_u = 0 as *mut var_u;
    let mut tmp: [u_char; 1024] = [0; 1024];
    let mut ptmp: libc::c_int = 0 as libc::c_int;
    let mut nsubs: libc::c_int = 0 as libc::c_int;
    let mut subs: [*mut cstring; 4] = [0 as *mut cstring; 4];
    let mut vp: *mut mvar = 0 as *mut mvar;
    while i < (*var).slen as libc::c_int {
        cnt = 0 as libc::c_int;
        if nsubs > 3 as libc::c_int {
            return -(38 as libc::c_int);
        }
        subs[nsubs
            as usize] = &mut *tmp.as_mut_ptr().offset(ptmp as isize) as *mut u_char
            as *mut cstring;
        s = UTIL_Key_Extract(
            &mut *((*var).key).as_mut_ptr().offset(i as isize),
            ((*subs[nsubs as usize]).buf).as_mut_ptr(),
            &mut cnt,
        ) as libc::c_int;
        if s < 0 as libc::c_int {
            return s;
        }
        let fresh0 = nsubs;
        nsubs = nsubs + 1;
        (*subs[fresh0 as usize]).len = s as u_short;
        ptmp = (ptmp as libc::c_ulong)
            .wrapping_add(
                (s as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += cnt;
    }
    i = SS_Norm(var) as libc::c_int;
    if i < 0 as libc::c_int {
        return i;
    }
    match (*var).name.var_cu[1 as libc::c_int as usize] as libc::c_int {
        67 => {
            if nsubs == 0 as libc::c_int {
                return -(38 as libc::c_int);
            }
            if strncasecmp(
                ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                    as *mut libc::c_char,
                b"m\0\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                return -(38 as libc::c_int);
            }
            if nsubs == 2 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"collate\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"ident\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
            } else if nsubs == 3 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"patcode\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return -(38 as libc::c_int);
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"m\0\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as libc::c_ulong,
                ) != 0 as libc::c_int
                {
                    return -(38 as libc::c_int);
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"input\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"output\0\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
            }
            return -(38 as libc::c_int);
        }
        68 => {
            if nsubs == 0 as libc::c_int {
                return -(38 as libc::c_int);
            }
            i = cstringtoi(subs[0 as libc::c_int as usize]);
            if i < 0 as libc::c_int || i >= 64 as libc::c_int {
                return -(38 as libc::c_int);
            }
            if (*partab.jobtab).seqio[i as usize].type_0 as libc::c_int
                == 0 as libc::c_int
            {
                return -(38 as libc::c_int);
            }
            if nsubs == 2 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$x\0\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*partab.jobtab).seqio[i as usize].dx as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$y\0\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*partab.jobtab).seqio[i as usize].dy as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"character\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return mcopy(
                        b"M\0" as *const u8 as *const libc::c_char as *mut u_char,
                        buf,
                        1 as libc::c_int,
                    );
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"fd\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return itocstring(buf, (*partab.jobtab).seqio[i as usize].fid)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"mode\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 0 as libc::c_int
                    {
                        return mcopy(
                            b"PRINCIPAL\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            9 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 1 as libc::c_int
                    {
                        return mcopy(
                            b"WRITE\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            5 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 2 as libc::c_int
                    {
                        return mcopy(
                            b"READ\0" as *const u8 as *const libc::c_char as *mut u_char,
                            buf,
                            4 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 3 as libc::c_int
                    {
                        return mcopy(
                            b"APPEND\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            6 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 4 as libc::c_int
                    {
                        return mcopy(
                            b"IO\0" as *const u8 as *const libc::c_char as *mut u_char,
                            buf,
                            2 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 5 as libc::c_int
                    {
                        return mcopy(
                            b"TCPIP\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            5 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 6 as libc::c_int
                    {
                        return mcopy(
                            b"SERVER\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            6 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 7 as libc::c_int
                    {
                        return mcopy(
                            b"NOFORK\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            6 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 8 as libc::c_int
                    {
                        return mcopy(
                            b"FORKED\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            6 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 9 as libc::c_int
                    {
                        return mcopy(
                            b"PIPE\0" as *const u8 as *const libc::c_char as *mut u_char,
                            buf,
                            4 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].mode as libc::c_int
                        == 10 as libc::c_int
                    {
                        return mcopy(
                            b"NEWPIPE\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            7 as libc::c_int,
                        )
                    } else {
                        return mcopy(
                            b"UNKNOWN\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            7 as libc::c_int,
                        )
                    }
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"name\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return mcopy(
                        ((*partab.jobtab).seqio[i as usize].name).as_mut_ptr(),
                        buf,
                        256 as libc::c_int,
                    );
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"namespace\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    j = 0 as libc::c_int;
                    while j < 32 as libc::c_int {
                        if (*partab.jobtab)
                            .seqio[i as usize]
                            .namespace
                            .var_cu[j as usize] as libc::c_int == '\0' as i32
                        {
                            break;
                        }
                        j += 1;
                        j;
                    }
                    return mcopy(
                        ((*partab.jobtab).seqio[i as usize].namespace.var_cu)
                            .as_mut_ptr(),
                        buf,
                        j,
                    );
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"type\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if (*partab.jobtab).seqio[i as usize].type_0 as libc::c_int
                        == 1 as libc::c_int
                    {
                        return mcopy(
                            b"1,FILE\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            6 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].type_0 as libc::c_int
                        == 2 as libc::c_int
                    {
                        return mcopy(
                            b"2,SOCKET\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            8 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].type_0 as libc::c_int
                        == 3 as libc::c_int
                    {
                        return mcopy(
                            b"3,PIPE\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            6 as libc::c_int,
                        )
                    } else if (*partab.jobtab).seqio[i as usize].type_0 as libc::c_int
                        == 4 as libc::c_int
                    {
                        return mcopy(
                            b"4,TERMINAL\0" as *const u8 as *const libc::c_char
                                as *mut u_char,
                            buf,
                            10 as libc::c_int,
                        )
                    }
                }
            } else if nsubs == 3 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"options\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if strncasecmp(
                        ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                            as *mut libc::c_char,
                        b"delete\0\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        if (*partab.jobtab).seqio[i as usize].options as libc::c_int
                            & 32 as libc::c_int != 0
                            && (*partab.jobtab).seqio[i as usize].options as libc::c_int
                                & 16 as libc::c_int != 0
                        {
                            return mcopy(
                                b"BOTH\0" as *const u8 as *const libc::c_char
                                    as *mut u_char,
                                buf,
                                4 as libc::c_int,
                            )
                        } else if (*partab.jobtab).seqio[i as usize].options
                            as libc::c_int & 32 as libc::c_int != 0
                        {
                            return mcopy(
                                b"DELETE\0" as *const u8 as *const libc::c_char
                                    as *mut u_char,
                                buf,
                                6 as libc::c_int,
                            )
                        } else if (*partab.jobtab).seqio[i as usize].options
                            as libc::c_int & 16 as libc::c_int != 0
                        {
                            return mcopy(
                                b"BACK\0" as *const u8 as *const libc::c_char
                                    as *mut u_char,
                                buf,
                                4 as libc::c_int,
                            )
                        } else {
                            return mcopy(
                                b"NONE\0" as *const u8 as *const libc::c_char
                                    as *mut u_char,
                                buf,
                                4 as libc::c_int,
                            )
                        }
                    }
                    if strncasecmp(
                        ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                            as *mut libc::c_char,
                        b"echo\0\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        if (*partab.jobtab).seqio[i as usize].options as libc::c_int
                            & 8 as libc::c_int != 0
                        {
                            return mcopy(
                                b"1\0" as *const u8 as *const libc::c_char as *mut u_char,
                                buf,
                                1 as libc::c_int,
                            )
                        } else {
                            return mcopy(
                                b"0\0" as *const u8 as *const libc::c_char as *mut u_char,
                                buf,
                                1 as libc::c_int,
                            )
                        }
                    }
                    if strncasecmp(
                        ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                            as *mut libc::c_char,
                        b"escape\0\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        if (*partab.jobtab).seqio[i as usize].options as libc::c_int
                            & 4 as libc::c_int != 0
                        {
                            return mcopy(
                                b"1\0" as *const u8 as *const libc::c_char as *mut u_char,
                                buf,
                                1 as libc::c_int,
                            )
                        } else {
                            return mcopy(
                                b"0\0" as *const u8 as *const libc::c_char as *mut u_char,
                                buf,
                                1 as libc::c_int,
                            )
                        }
                    }
                    if strncasecmp(
                        ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                            as *mut libc::c_char,
                        b"output\0\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        if (*partab.jobtab).seqio[i as usize].options as libc::c_int
                            & 2 as libc::c_int != 0
                        {
                            let mut temp_buf: [libc::c_char; 24] = [0; 24];
                            s = 0 as libc::c_int;
                            j = 0 as libc::c_int;
                            while j
                                < (*partab.jobtab).seqio[i as usize].out_len as libc::c_int
                            {
                                if iscntrl(
                                    (*partab.jobtab).seqio[i as usize].out_term[j as usize]
                                        as libc::c_int,
                                ) != 0
                                {
                                    let fresh1 = s;
                                    s = s + 1;
                                    temp_buf[fresh1 as usize] = '\\' as i32 as libc::c_char;
                                }
                                match (*partab.jobtab)
                                    .seqio[i as usize]
                                    .out_term[j as usize] as libc::c_int
                                {
                                    7 => {
                                        let fresh2 = s;
                                        s = s + 1;
                                        temp_buf[fresh2 as usize] = 'a' as i32 as libc::c_char;
                                    }
                                    8 => {
                                        let fresh3 = s;
                                        s = s + 1;
                                        temp_buf[fresh3 as usize] = 'b' as i32 as libc::c_char;
                                    }
                                    12 => {
                                        let fresh4 = s;
                                        s = s + 1;
                                        temp_buf[fresh4 as usize] = 'f' as i32 as libc::c_char;
                                    }
                                    10 => {
                                        let fresh5 = s;
                                        s = s + 1;
                                        temp_buf[fresh5 as usize] = 'n' as i32 as libc::c_char;
                                    }
                                    13 => {
                                        let fresh6 = s;
                                        s = s + 1;
                                        temp_buf[fresh6 as usize] = 'r' as i32 as libc::c_char;
                                    }
                                    9 => {
                                        let fresh7 = s;
                                        s = s + 1;
                                        temp_buf[fresh7 as usize] = 't' as i32 as libc::c_char;
                                    }
                                    11 => {
                                        let fresh8 = s;
                                        s = s + 1;
                                        temp_buf[fresh8 as usize] = 'v' as i32 as libc::c_char;
                                    }
                                    _ => {
                                        if iscntrl(
                                            (*partab.jobtab).seqio[i as usize].out_term[j as usize]
                                                as libc::c_int,
                                        ) != 0
                                        {
                                            sprintf(
                                                &mut *temp_buf.as_mut_ptr().offset(s as isize)
                                                    as *mut libc::c_char,
                                                b"%03o\0" as *const u8 as *const libc::c_char,
                                                (*partab.jobtab).seqio[i as usize].out_term[j as usize]
                                                    as libc::c_int,
                                            );
                                            s += 3 as libc::c_int;
                                        } else {
                                            let fresh9 = s;
                                            s = s + 1;
                                            temp_buf[fresh9
                                                as usize] = (*partab.jobtab)
                                                .seqio[i as usize]
                                                .out_term[j as usize] as libc::c_char;
                                        }
                                    }
                                }
                                j += 1;
                                j;
                            }
                            return mcopy(temp_buf.as_mut_ptr() as *mut u_char, buf, s);
                        } else {
                            return 0 as libc::c_int
                        }
                    }
                    if strncasecmp(
                        ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                            as *mut libc::c_char,
                        b"terminator\0\0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        if (*partab.jobtab).seqio[i as usize].options as libc::c_int
                            & 1 as libc::c_int != 0
                        {
                            let mut in_term: u_int64 = (*partab.jobtab)
                                .seqio[i as usize]
                                .in_term
                                .interm[0 as libc::c_int as usize];
                            let mut temp_buf_0: [u_char; 402] = [0; 402];
                            let mut count: u_int64 = 0 as libc::c_int as u_int64;
                            s = 0 as libc::c_int;
                            j = 0 as libc::c_int;
                            while count < in_term {
                                count = ((1 as libc::c_uint) << j) as u_int64;
                                if in_term & count != 0 {
                                    s
                                        += itocstring(
                                            &mut *temp_buf_0.as_mut_ptr().offset(s as isize),
                                            j,
                                        ) as libc::c_int;
                                    let fresh10 = s;
                                    s = s + 1;
                                    temp_buf_0[fresh10 as usize] = ',' as i32 as u_char;
                                }
                                j += 1;
                                j;
                            }
                            in_term = (*partab.jobtab)
                                .seqio[i as usize]
                                .in_term
                                .interm[1 as libc::c_int as usize];
                            count = 0 as libc::c_int as u_int64;
                            j = 0 as libc::c_int;
                            while count < in_term {
                                count = ((1 as libc::c_uint) << j) as u_int64;
                                if in_term & count != 0 {
                                    s
                                        += itocstring(
                                            &mut *temp_buf_0.as_mut_ptr().offset(s as isize),
                                            j + 64 as libc::c_int,
                                        ) as libc::c_int;
                                    let fresh11 = s;
                                    s = s + 1;
                                    temp_buf_0[fresh11 as usize] = ',' as i32 as u_char;
                                }
                                j += 1;
                                j;
                            }
                            s -= 1;
                            return mcopy(temp_buf_0.as_mut_ptr(), buf, s);
                        } else {
                            return 0 as libc::c_int
                        }
                    }
                }
            }
            return -(38 as libc::c_int);
        }
        71 => {
            if nsubs == 2 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"character\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return mcopy(
                        b"M\0" as *const u8 as *const libc::c_char as *mut u_char,
                        buf,
                        1 as libc::c_int,
                    );
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"collate\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"journal\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    (*var)
                        .slen = (strlen(((*var).key).as_mut_ptr() as *mut libc::c_char))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as u_char;
                    i = DB_GetFlags(var);
                    if i < 0 as libc::c_int {
                        return i;
                    }
                    return itocstring(buf, i & 1 as libc::c_int) as libc::c_int;
                }
            }
            if nsubs > 1 as libc::c_int {
                return -(38 as libc::c_int);
            }
            return DB_Get(var, buf);
        }
        74 => {
            *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
            if nsubs == 0 as libc::c_int {
                return uitocstring(buf, (*systab).maxjob) as libc::c_int;
            }
            if nsubs < 2 as libc::c_int {
                return -(38 as libc::c_int);
            }
            i = cstringtoi(subs[0 as libc::c_int as usize]) - 1 as libc::c_int;
            if i < 0 as libc::c_int || i >= (*systab).maxjob as libc::c_int {
                return -(23 as libc::c_int);
            }
            if (*((*systab).jobtab).offset(i as isize)).pid == 0 as libc::c_int {
                return -(23 as libc::c_int);
            }
            if kill((*((*systab).jobtab).offset(i as isize)).pid, 0 as libc::c_int)
                == -(1 as libc::c_int)
            {
                if *__error() == 3 as libc::c_int {
                    CleanJob(i + 1 as libc::c_int);
                    return -(23 as libc::c_int);
                }
            }
            if nsubs == 2 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$io\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).io as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$reference\0\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    vp = &mut (*((*systab).jobtab).offset(i as isize)).last_ref;
                    if (*vp).name.var_cu[0 as libc::c_int as usize] as libc::c_int
                        == '\0' as i32
                    {
                        return 0 as libc::c_int;
                    }
                    memcpy(
                        tmp.as_mut_ptr() as *mut libc::c_void,
                        vp as *const libc::c_void,
                        ((*vp).slen as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<var_u>() as libc::c_ulong,
                            )
                            .wrapping_add(4 as libc::c_int as libc::c_ulong),
                    );
                    vp = tmp.as_mut_ptr() as *mut mvar;
                    if (*vp).uci as libc::c_int == 0 as libc::c_int {
                        (*vp).uci = (*((*systab).jobtab).offset(i as isize)).uci;
                    }
                    if (*vp).volset as libc::c_int == 0 as libc::c_int {
                        (*vp).volset = (*((*systab).jobtab).offset(i as isize)).vol;
                    }
                    return UTIL_String_Mvar(vp, buf, 63 as libc::c_int) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$stack\0\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return itocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).cur_do,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"character\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return mcopy(
                        b"M\0" as *const u8 as *const libc::c_char as *mut u_char,
                        buf,
                        1 as libc::c_int,
                    );
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"commands\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).commands,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"global\0\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).uci as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"global_vol\0\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).vol as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"grefs\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).grefs,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"lock\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).luci as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"lock_vol\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).lvol as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"owner\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    let mut pp: *mut passwd = getpwuid(
                        (*((*systab).jobtab).offset(i as isize)).user as uid_t,
                    );
                    if pp.is_null() {
                        return itocstring(
                            buf,
                            (*((*systab).jobtab).offset(i as isize)).user,
                        ) as libc::c_int;
                    }
                    strcpy(buf as *mut libc::c_char, (*pp).pw_name);
                    return strlen(buf as *mut libc::c_char) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"owner_id\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return itocstring(buf, (*((*systab).jobtab).offset(i as isize)).user)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"pid\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return itocstring(buf, (*((*systab).jobtab).offset(i as isize)).pid)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"precision\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return itocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).precision as libc::c_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"priority\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    *__error() = 0 as libc::c_int;
                    j = getpriority(
                        0 as libc::c_int,
                        (*((*systab).jobtab).offset(i as isize)).pid as id_t,
                    );
                    if *__error() != 0 as libc::c_int {
                        return -(200 as libc::c_int + 200 as libc::c_int + *__error());
                    }
                    return itocstring(buf, j) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"priv\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return itocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).priv_0 as libc::c_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"process_start\0\0" as *const u8 as *const libc::c_char,
                    14 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return mcopy(
                        ((*((*systab).jobtab).offset(i as isize)).start_dh).as_mut_ptr(),
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).start_len as libc::c_int,
                    );
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"routine\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).ruci as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"routine_name\0\0" as *const u8 as *const libc::c_char,
                    13 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    j = (*((*systab).jobtab).offset(i as isize)).cur_do;
                    rounam = &mut (*((*((*systab).jobtab).offset(i as isize)).dostk)
                        .as_mut_ptr()
                        .offset(j as isize))
                        .rounam;
                    s = 0 as libc::c_int;
                    while s < 32 as libc::c_int {
                        let ref mut fresh12 = *buf.offset(s as isize);
                        *fresh12 = (*rounam).var_cu[s as usize];
                        if *fresh12 as libc::c_int == 0 as libc::c_int {
                            break;
                        }
                        s += 1;
                        s;
                    }
                    *buf.offset(s as isize) = '\0' as i32 as u_char;
                    return s;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"routine_vol\0\0" as *const u8 as *const libc::c_char,
                    12 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*((*systab).jobtab).offset(i as isize)).rvol as u_int,
                    ) as libc::c_int;
                }
            } else if nsubs == 3 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$io\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    j = cstringtoi(subs[2 as libc::c_int as usize]);
                    if j < 0 as libc::c_int || j >= 64 as libc::c_int {
                        return -(38 as libc::c_int);
                    }
                    if (*((*systab).jobtab).offset(i as isize)).seqio[j as usize].type_0
                        as libc::c_int == 0 as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    cnt = strlen(
                        ((*((*systab).jobtab).offset(i as isize)).seqio[j as usize].name)
                            .as_mut_ptr() as *mut libc::c_char,
                    ) as libc::c_int;
                    return mcopy(
                        ((*((*systab).jobtab).offset(i as isize)).seqio[j as usize].name)
                            .as_mut_ptr(),
                        buf,
                        cnt,
                    );
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$stack\0\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return Dstack1x(buf, cstringtoi(subs[2 as libc::c_int as usize]), i)
                        as libc::c_int;
                }
            } else if nsubs == 4 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$stack\0\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return Dstack2x(
                        buf,
                        cstringtoi(subs[2 as libc::c_int as usize]),
                        subs[3 as libc::c_int as usize],
                        i,
                    );
                }
            }
            return -(38 as libc::c_int);
        }
        76 => {
            if nsubs != 1 as libc::c_int {
                return -(38 as libc::c_int);
            }
            if (*subs[0 as libc::c_int as usize]).len as libc::c_int > 511 as libc::c_int
            {
                return -(12 as libc::c_int + 200 as libc::c_int);
            }
            vp = &mut *tmp.as_mut_ptr().offset(512 as libc::c_int as isize)
                as *mut u_char as *mut mvar;
            s = UTIL_MvarFromCStr(subs[0 as libc::c_int as usize], vp) as libc::c_int;
            if s < 0 as libc::c_int {
                return s;
            }
            s = UTIL_mvartolock(
                vp,
                ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr(),
            ) as libc::c_int;
            if s < 0 as libc::c_int {
                return s;
            }
            (*subs[0 as libc::c_int as usize]).len = s as u_short;
            return LCK_Get(subs[0 as libc::c_int as usize], buf) as libc::c_int;
        }
        82 => {
            if nsubs > 2 as libc::c_int {
                return -(38 as libc::c_int);
            }
            if nsubs == 2 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"character\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return mcopy(
                        b"M\0" as *const u8 as *const libc::c_char as *mut u_char,
                        buf,
                        1 as libc::c_int,
                    );
                }
            }
            return DB_Get(var, buf);
        }
        83 => {
            if nsubs == 0 as libc::c_int {
                return -(38 as libc::c_int);
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$nextok\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                return itocstring(
                    buf,
                    ((*systab).historic & 4 as libc::c_int) / 4 as libc::c_int,
                ) as libc::c_int;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"eok\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                return itocstring(buf, (*systab).historic & 1 as libc::c_int)
                    as libc::c_int;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"offok\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                return itocstring(
                    buf,
                    ((*systab).historic & 2 as libc::c_int) / 2 as libc::c_int,
                ) as libc::c_int;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"big_endian\0\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let mut end: u_int = 0x1 as libc::c_int as u_int;
                return uitocstring(
                    buf,
                    (if *(&mut end as *mut u_int as *mut u_char) as libc::c_int
                        == 0x1 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as u_int,
                ) as libc::c_int;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"character\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                return mcopy(
                    b"M\0" as *const u8 as *const libc::c_char as *mut u_char,
                    buf,
                    1 as libc::c_int,
                );
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"name_length\0\0" as *const u8 as *const libc::c_char,
                    12 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                return uitocstring(buf, 32 as libc::c_int as u_int) as libc::c_int;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"precision\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                return itocstring(buf, (*systab).precision) as libc::c_int;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"string_max\0\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                return uitocstring(buf, 65534 as libc::c_int as u_int) as libc::c_int;
            }
            if strncasecmp(
                ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                    as *mut libc::c_char,
                b"trantab\0\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                i = cstringtoi(subs[1 as libc::c_int as usize]) - 1 as libc::c_int;
                if !(i < 8 as libc::c_int) || i < 0 as libc::c_int {
                    return -(38 as libc::c_int);
                }
                if nsubs != 2 as libc::c_int {
                    return -(38 as libc::c_int);
                }
                if (*systab).tt[i as usize].from_vol == 0 {
                    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
                    return 0 as libc::c_int;
                }
                s = UTIL_String_Mvar(
                    &mut (*((*systab).tt).as_mut_ptr().offset(i as isize)).to_global
                        as *mut var_u as *mut mvar,
                    buf,
                    0 as libc::c_int,
                ) as libc::c_int;
                let fresh13 = s;
                s = s + 1;
                *buf.offset(fresh13 as isize) = '=' as i32 as u_char;
                s
                    += UTIL_String_Mvar(
                        &mut (*((*systab).tt).as_mut_ptr().offset(i as isize))
                            .from_global as *mut var_u as *mut mvar,
                        &mut *buf.offset(s as isize),
                        0 as libc::c_int,
                    ) as libc::c_int;
                return s;
            }
            if strncasecmp(
                ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                    as *mut libc::c_char,
                b"vol\0\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                i = cstringtoi(subs[1 as libc::c_int as usize]) - 1 as libc::c_int;
                if !(i < 1 as libc::c_int) || i < 0 as libc::c_int
                    || ((*systab).vol[i as usize]).is_null()
                {
                    return -(38 as libc::c_int);
                }
                if nsubs < 3 as libc::c_int {
                    return -(38 as libc::c_int);
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"block\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*(*(*systab).vol[i as usize]).vollab).block_size,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"file\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    strcpy(
                        buf as *mut libc::c_char,
                        ((*(*systab).vol[i as usize]).file_name).as_mut_ptr(),
                    );
                    return strlen(buf as *mut libc::c_char) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"free\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return itocstring(buf, DB_Free(i + 1 as libc::c_int)) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"header\0\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*(*(*systab).vol[i as usize]).vollab).header_bytes,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"journal_available\0\0" as *const u8 as *const libc::c_char,
                    18 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*(*(*systab).vol[i as usize]).vollab).journal_available as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"journal_file\0\0" as *const u8 as *const libc::c_char,
                    13 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    strcpy(
                        buf as *mut libc::c_char,
                        ((*(*(*systab).vol[i as usize]).vollab).journal_file)
                            .as_mut_ptr(),
                    );
                    return strlen(buf as *mut libc::c_char) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"journal_requested\0\0" as *const u8 as *const libc::c_char,
                    18 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*(*(*systab).vol[i as usize]).vollab).journal_requested as u_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"journal_size\0\0" as *const u8 as *const libc::c_char,
                    13 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return itocstring(
                        buf,
                        (*(*systab).vol[i as usize]).jrn_next as libc::c_int,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"name\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    j = 0 as libc::c_int;
                    while j < 32 as libc::c_int {
                        let ref mut fresh14 = *buf.offset(j as isize);
                        *fresh14 = (*(*(*systab).vol[i as usize]).vollab)
                            .volnam
                            .var_cu[j as usize];
                        if *fresh14 as libc::c_int == 0 as libc::c_int {
                            break;
                        }
                        j += 1;
                        j;
                    }
                    *buf.offset(j as isize) = '\0' as i32 as u_char;
                    return j;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"size\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*(*(*systab).vol[i as usize]).vollab).max_block,
                    ) as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"uci\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if nsubs != 4 as libc::c_int {
                        return -(38 as libc::c_int);
                    }
                    j = cstringtoi(subs[3 as libc::c_int as usize]) - 1 as libc::c_int;
                    if !(j < 64 as libc::c_int) || j < 0 as libc::c_int {
                        return -(38 as libc::c_int);
                    }
                    s = 0 as libc::c_int;
                    while s < 32 as libc::c_int {
                        let ref mut fresh15 = *buf.offset(s as isize);
                        *fresh15 = (*(*(*systab).vol[i as usize]).vollab)
                            .uci[j as usize]
                            .name
                            .var_cu[s as usize];
                        if *fresh15 as libc::c_int == 0 as libc::c_int {
                            break;
                        }
                        s += 1;
                        s;
                    }
                    *buf.offset(s as isize) = '\0' as i32 as u_char;
                    return s;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"writelock\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return itocstring(buf, (*(*systab).vol[i as usize]).writelock)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"blkalloc\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.blkalloc)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"blkdeall\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.blkdeall)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"blkreorg\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.blkreorg)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"dbdat\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.dbdat)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"dbget\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.dbget)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"dbkil\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.dbkil)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"dbord\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.dbord)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"dbqry\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.dbqry)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"dbset\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.dbset)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"lastok\0\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.lastok)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"lasttry\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.lasttry)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"logrd\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.logrd)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"logwt\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.logwt)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"phyrd\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.phyrd)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"phywt\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(buf, (*(*systab).vol[i as usize]).stats.phywt)
                        as libc::c_int;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"diskerrors\0\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return uitocstring(
                        buf,
                        (*(*systab).vol[i as usize]).stats.diskerrors,
                    ) as libc::c_int;
                }
            }
            return -(38 as libc::c_int);
        }
        _ => {}
    }
    return -(38 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SS_Set(
    mut var: *mut mvar,
    mut data: *mut cstring,
) -> libc::c_short {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_short = 0;
    let mut cnt: libc::c_int = 0;
    let mut n: var_u = VAR_U { var_q: 0 };
    let mut tmp: [u_char; 1024] = [0; 1024];
    let mut ptmp: libc::c_int = 0 as libc::c_int;
    let mut nsubs: libc::c_int = 0 as libc::c_int;
    let mut subs: [*mut cstring; 4] = [0 as *mut cstring; 4];
    let mut tt: trantab = TRANTAB {
        from_global: VAR_U { var_q: 0 },
        from_vol: 0,
        from_uci: 0,
        to_global: VAR_U { var_q: 0 },
        to_vol: 0,
        to_uci: 0,
    };
    while i < (*var).slen as libc::c_int {
        cnt = 0 as libc::c_int;
        if nsubs > 3 as libc::c_int {
            return -(38 as libc::c_int) as libc::c_short;
        }
        subs[nsubs
            as usize] = &mut *tmp.as_mut_ptr().offset(ptmp as isize) as *mut u_char
            as *mut cstring;
        s = UTIL_Key_Extract(
            &mut *((*var).key).as_mut_ptr().offset(i as isize),
            ((*subs[nsubs as usize]).buf).as_mut_ptr(),
            &mut cnt,
        );
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        let fresh16 = nsubs;
        nsubs = nsubs + 1;
        (*subs[fresh16 as usize]).len = s as u_short;
        ptmp = (ptmp as libc::c_ulong)
            .wrapping_add(
                (s as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += cnt;
    }
    s = SS_Norm(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    match (*var).name.var_cu[1 as libc::c_int as usize] as libc::c_int {
        67 => return -(29 as libc::c_int) as libc::c_short,
        68 => return -(29 as libc::c_int) as libc::c_short,
        71 => {
            if nsubs == 2 as libc::c_int {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"journal\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    (*var)
                        .slen = (strlen(((*var).key).as_mut_ptr() as *mut libc::c_char))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as u_char;
                    i = cstringtob(data);
                    if i == 0 {
                        i = -(1 as libc::c_int);
                    }
                    i = DB_SetFlags(var, i);
                    if i < 0 as libc::c_int {
                        return i as libc::c_short;
                    }
                    return 0 as libc::c_int as libc::c_short;
                }
            }
            return -(29 as libc::c_int) as libc::c_short;
        }
        74 => {
            if nsubs != 2 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            i = cstringtoi(subs[0 as libc::c_int as usize]) - 1 as libc::c_int;
            if i < 0 as libc::c_int || i >= (*systab).maxjob as libc::c_int {
                return -(23 as libc::c_int) as libc::c_short;
            }
            if (*((*systab).jobtab).offset(i as isize)).pid == 0 as libc::c_int {
                return -(23 as libc::c_int) as libc::c_short;
            }
            j = cstringtoi(data);
            if (partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                == i as libc::c_long
            {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"global\0\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if j < 1 as libc::c_int || j > 64 as libc::c_int {
                        return -(26 as libc::c_int) as libc::c_short;
                    }
                    (*((*systab).jobtab).offset(i as isize)).uci = j as u_char;
                    let mut var_i: u_int = 0 as libc::c_int as u_int;
                    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        (*((*systab).jobtab).offset(i as isize))
                            .last_ref
                            .name
                            .var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
                        var_i = var_i.wrapping_add(1);
                        var_i;
                    }
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"global_vol\0\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if j < 1 as libc::c_int || j > 1 as libc::c_int {
                        return -(26 as libc::c_int) as libc::c_short;
                    }
                    if ((*systab).vol[(j - 1 as libc::c_int) as usize]).is_null() {
                        return -(26 as libc::c_int) as libc::c_short;
                    }
                    (*((*systab).jobtab).offset(i as isize)).vol = j as u_char;
                    let mut var_i_0: u_int = 0 as libc::c_int as u_int;
                    while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        (*((*systab).jobtab).offset(i as isize))
                            .last_ref
                            .name
                            .var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
                        var_i_0 = var_i_0.wrapping_add(1);
                        var_i_0;
                    }
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"lock\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if j < 1 as libc::c_int || j > 64 as libc::c_int {
                        return -(26 as libc::c_int) as libc::c_short;
                    }
                    (*((*systab).jobtab).offset(i as isize)).luci = j as u_char;
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"lock_vol\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if j < 1 as libc::c_int || j > 1 as libc::c_int {
                        return -(26 as libc::c_int) as libc::c_short;
                    }
                    if ((*systab).vol[(j - 1 as libc::c_int) as usize]).is_null() {
                        return -(26 as libc::c_int) as libc::c_short;
                    }
                    (*((*systab).jobtab).offset(i as isize)).lvol = j as u_char;
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"precision\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if j < 0 as libc::c_int || j > 64 as libc::c_int {
                        return -(28 as libc::c_int) as libc::c_short;
                    }
                    (*((*systab).jobtab).offset(i as isize))
                        .precision = j as libc::c_short;
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"routine\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if j < 1 as libc::c_int || j > 64 as libc::c_int {
                        return -(26 as libc::c_int) as libc::c_short;
                    }
                    (*((*systab).jobtab).offset(i as isize)).ruci = j as u_char;
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"routine_vol\0\0" as *const u8 as *const libc::c_char,
                    12 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if j < 1 as libc::c_int || j > 1 as libc::c_int {
                        return -(26 as libc::c_int) as libc::c_short;
                    }
                    if ((*systab).vol[(j - 1 as libc::c_int) as usize]).is_null() {
                        return -(26 as libc::c_int) as libc::c_short;
                    }
                    (*((*systab).jobtab).offset(i as isize)).rvol = j as u_char;
                    return 0 as libc::c_int as libc::c_short;
                }
            }
            if priv_0() != 0 {
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"owner_id\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    (*((*systab).jobtab).offset(i as isize)).user = j;
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"priority\0\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    *__error() = 0 as libc::c_int;
                    if setpriority(
                        0 as libc::c_int,
                        (*((*systab).jobtab).offset(i as isize)).pid as id_t,
                        j,
                    ) == -(1 as libc::c_int)
                    {
                        return -(200 as libc::c_int + 200 as libc::c_int + *__error())
                            as libc::c_short;
                    }
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[1 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"priv\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    (*((*systab).jobtab).offset(i as isize))
                        .priv_0 = (j != 0 || 0 as libc::c_int != 0) as libc::c_int
                        as libc::c_short;
                    if j == 0 {
                        j = setuid((*partab.jobtab).user as uid_t);
                    }
                    if j == -(1 as libc::c_int) {
                        return -(200 as libc::c_int + 200 as libc::c_int + *__error())
                            as libc::c_short;
                    }
                    return 0 as libc::c_int as libc::c_short;
                }
            }
            return -(29 as libc::c_int) as libc::c_short;
        }
        76 => return -(29 as libc::c_int) as libc::c_short,
        82 => {
            if nsubs > 2 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            return -(29 as libc::c_int) as libc::c_short;
        }
        83 => {
            if priv_0() == 0 {
                return -(38 as libc::c_int) as libc::c_short;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"$nextok\0\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                if cstringtob(data) != 0 {
                    (*systab).historic |= 4 as libc::c_int;
                } else {
                    (*systab).historic &= !(4 as libc::c_int);
                }
                return 0 as libc::c_int as libc::c_short;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"eok\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                if cstringtob(data) != 0 {
                    (*systab).historic |= 1 as libc::c_int;
                } else {
                    (*systab).historic &= !(1 as libc::c_int);
                }
                return 0 as libc::c_int as libc::c_short;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"offok\0\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                if cstringtob(data) != 0 {
                    (*systab).historic |= 2 as libc::c_int;
                } else {
                    (*systab).historic &= !(2 as libc::c_int);
                }
                return 0 as libc::c_int as libc::c_short;
            }
            if nsubs == 1 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"precision\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                j = cstringtoi(data);
                if j < 0 as libc::c_int || j > 64 as libc::c_int {
                    return -(28 as libc::c_int) as libc::c_short;
                }
                (*systab).precision = j;
                return 0 as libc::c_int as libc::c_short;
            }
            if strncasecmp(
                ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                    as *mut libc::c_char,
                b"trantab\0\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                cnt = cstringtoi(subs[1 as libc::c_int as usize]) - 1 as libc::c_int;
                if !(cnt < 8 as libc::c_int) || cnt < 0 as libc::c_int {
                    return -(38 as libc::c_int) as libc::c_short;
                }
                if nsubs != 2 as libc::c_int {
                    return -(38 as libc::c_int) as libc::c_short;
                }
                if (*data).len as libc::c_int == 0 as libc::c_int {
                    memset(
                        &mut *((*systab).tt).as_mut_ptr().offset(cnt as isize)
                            as *mut trantab as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<trantab>() as libc::c_ulong,
                    );
                    (*systab).max_tt = 0 as libc::c_int;
                    i = 8 as libc::c_int;
                    while i != 0 {
                        if (*systab).tt[(i - 1 as libc::c_int) as usize].to_uci != 0 {
                            (*systab).max_tt = i;
                            break;
                        } else {
                            i -= 1;
                            i;
                        }
                    }
                    return 0 as libc::c_int as libc::c_short;
                }
                subs[2 as libc::c_int as usize] = tmp.as_mut_ptr() as *mut cstring;
                subs[3 as libc::c_int
                    as usize] = &mut *tmp
                    .as_mut_ptr()
                    .offset(512 as libc::c_int as isize) as *mut u_char as *mut cstring;
                i = 0 as libc::c_int;
                loop {
                    if (*data).buf[i as usize] as libc::c_int == '=' as i32 {
                        (*subs[3 as libc::c_int as usize])
                            .buf[i as usize] = '\0' as i32 as u_char;
                        let fresh17 = i;
                        i = i + 1;
                        (*subs[3 as libc::c_int as usize]).len = fresh17 as u_short;
                        break;
                    } else {
                        (*subs[3 as libc::c_int as usize])
                            .buf[i as usize] = (*data).buf[i as usize];
                        i += 1;
                        i;
                    }
                }
                j = 0 as libc::c_int;
                loop {
                    let fresh18 = i;
                    i = i + 1;
                    let fresh19 = j;
                    j = j + 1;
                    (*subs[2 as libc::c_int as usize])
                        .buf[fresh19 as usize] = (*data).buf[fresh18 as usize];
                    if !((*subs[2 as libc::c_int as usize]).buf[fresh19 as usize] != 0) {
                        break;
                    }
                }
                s = UTIL_MvarFromCStr(
                    subs[2 as libc::c_int as usize],
                    &mut partab.src_var,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    return s;
                }
                if partab.src_var.uci as libc::c_int == 255 as libc::c_int {
                    return 0 as libc::c_int as libc::c_short
                } else if partab.src_var.uci == 0 {
                    if partab.src_var.name.var_cu[0 as libc::c_int as usize]
                        as libc::c_int == '%' as i32
                    {
                        partab.src_var.uci = 1 as libc::c_int as u_char;
                    } else {
                        partab.src_var.uci = (*partab.jobtab).uci;
                    }
                }
                if partab.src_var.volset == 0 {
                    partab.src_var.volset = (*partab.jobtab).vol;
                }
                memcpy(
                    &mut tt.from_global as *mut var_u as *mut libc::c_void,
                    &mut partab.src_var as *mut mvar as *const libc::c_void,
                    (::core::mem::size_of::<var_u>() as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong),
                );
                s = UTIL_MvarFromCStr(
                    subs[3 as libc::c_int as usize],
                    &mut partab.src_var,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    return s;
                }
                if partab.src_var.uci as libc::c_int == 255 as libc::c_int {
                    return 0 as libc::c_int as libc::c_short
                } else if partab.src_var.uci == 0 {
                    if partab.src_var.name.var_cu[0 as libc::c_int as usize]
                        as libc::c_int == '%' as i32
                    {
                        partab.src_var.uci = 1 as libc::c_int as u_char;
                    } else {
                        partab.src_var.uci = (*partab.jobtab).uci;
                    }
                }
                if partab.src_var.volset == 0 {
                    partab.src_var.volset = (*partab.jobtab).vol;
                }
                memcpy(
                    &mut tt.to_global as *mut var_u as *mut libc::c_void,
                    &mut partab.src_var as *mut mvar as *const libc::c_void,
                    (::core::mem::size_of::<var_u>() as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong),
                );
                memcpy(
                    &mut *((*systab).tt).as_mut_ptr().offset(cnt as isize)
                        as *mut trantab as *mut libc::c_void,
                    &mut tt as *mut trantab as *const libc::c_void,
                    ::core::mem::size_of::<trantab>() as libc::c_ulong,
                );
                if cnt + 1 as libc::c_int > (*systab).max_tt {
                    (*systab).max_tt = cnt + 1 as libc::c_int;
                }
                return 0 as libc::c_int as libc::c_short;
            }
            if nsubs == 4 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"vol\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                && strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"uci\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                i = cstringtoi(subs[1 as libc::c_int as usize]) - 1 as libc::c_int;
                j = cstringtoi(subs[3 as libc::c_int as usize]) - 1 as libc::c_int;
                if i < 0 as libc::c_int || i >= 1 as libc::c_int {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                if j < 0 as libc::c_int || j >= 64 as libc::c_int {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                if ((*data).len as libc::c_int) < 1 as libc::c_int
                    || (*data).len as libc::c_int > 32 as libc::c_int
                {
                    return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
                }
                let mut var_i_1: u_int = 0 as libc::c_int as u_int;
                while var_i_1 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    n.var_qu[var_i_1 as usize] = 0 as libc::c_int as u_int64;
                    var_i_1 = var_i_1.wrapping_add(1);
                    var_i_1;
                }
                s = 0 as libc::c_int as libc::c_short;
                while (s as libc::c_int) < (*data).len as libc::c_int {
                    if isalpha((*data).buf[s as usize] as libc::c_int)
                        == 0 as libc::c_int
                    {
                        return -(12 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                    }
                    n.var_cu[s as usize] = (*data).buf[s as usize];
                    s += 1;
                    s;
                }
                return DB_UCISet(i + 1 as libc::c_int, j + 1 as libc::c_int, n);
            }
            if nsubs == 3 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"vol\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                i = cstringtoi(subs[1 as libc::c_int as usize]) - 1 as libc::c_int;
                if i < 0 as libc::c_int || i >= 1 as libc::c_int {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"file\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if (*data).len as libc::c_int > 256 as libc::c_int {
                        return -(56 as libc::c_int) as libc::c_short;
                    }
                    s = DB_Mount(
                        ((*data).buf).as_mut_ptr() as *mut libc::c_char,
                        i,
                        0 as libc::c_int as u_int,
                        0 as libc::c_int as u_int,
                    );
                    if (s as libc::c_int) < 0 as libc::c_int {
                        return s;
                    }
                    (*(*systab).vol[i as usize]).map_dirty_flag = 1 as libc::c_int;
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"journal_file\0\0" as *const u8 as *const libc::c_char,
                    13 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int && (*systab).maxjob == 1 as libc::c_int as u_int
                {
                    if (*data).len as libc::c_int > 226 as libc::c_int {
                        return -(56 as libc::c_int) as libc::c_short;
                    }
                    strcpy(
                        ((*(*(*systab).vol[i as usize]).vollab).journal_file)
                            .as_mut_ptr(),
                        ((*data).buf).as_mut_ptr() as *mut libc::c_char,
                    );
                    (*(*systab).vol[i as usize]).map_dirty_flag = 1 as libc::c_int;
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"journal_requested\0\0" as *const u8 as *const libc::c_char,
                    18 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    (*(*(*systab).vol[i as usize]).vollab)
                        .journal_requested = cstringtob(data) as u_char;
                    if (*(*(*systab).vol[i as usize]).vollab).journal_requested == 0 {
                        DB_StopJournal(i + 1 as libc::c_int, 2 as libc::c_int as u_char);
                    }
                    (*(*systab).vol[i as usize]).map_dirty_flag = 1 as libc::c_int;
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"journal_size\0\0" as *const u8 as *const libc::c_char,
                    13 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int && cstringtoi(data) == 0 as libc::c_int
                {
                    while SemOp(
                        2 as libc::c_int,
                        ((*systab).maxjob).wrapping_neg() as libc::c_int,
                    ) != 0
                    {}
                    ClearJournal(i);
                    SemOp(2 as libc::c_int, (*systab).maxjob as libc::c_int);
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"name\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int && (*systab).maxjob == 1 as libc::c_int as u_int
                    && (*data).len as libc::c_int > 0 as libc::c_int
                    && ((*data).len as libc::c_int) < 32 as libc::c_int
                {
                    j = 0 as libc::c_int;
                    while j < (*data).len as libc::c_int {
                        if isalpha((*data).buf[j as usize] as libc::c_int)
                            == 0 as libc::c_int
                        {
                            return -(38 as libc::c_int) as libc::c_short;
                        }
                        j += 1;
                        j;
                    }
                    let mut var_i_2: u_int = 0 as libc::c_int as u_int;
                    while var_i_2 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        (*(*(*systab).vol[i as usize]).vollab)
                            .volnam
                            .var_qu[var_i_2 as usize] = 0 as libc::c_int as u_int64;
                        var_i_2 = var_i_2.wrapping_add(1);
                        var_i_2;
                    }
                    memcpy(
                        ((*(*(*systab).vol[i as usize]).vollab).volnam.var_cu)
                            .as_mut_ptr() as *mut libc::c_void,
                        ((*data).buf).as_mut_ptr() as *const libc::c_void,
                        (*data).len as libc::c_ulong,
                    );
                    (*(*systab).vol[i as usize]).map_dirty_flag = 1 as libc::c_int;
                    return 0 as libc::c_int as libc::c_short;
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"size\0\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int && (*systab).maxjob == 1 as libc::c_int as u_int
                {
                    let mut vsiz: u_int = 0;
                    vsiz = atol(((*data).buf).as_mut_ptr() as *mut libc::c_char)
                        as u_int;
                    if vsiz <= (*(*(*systab).vol[i as usize]).vollab).max_block {
                        return -(38 as libc::c_int) as libc::c_short;
                    }
                    vsiz |= 7 as libc::c_int as u_int;
                    if vsiz > 2147483647 as libc::c_uint {
                        return -(38 as libc::c_int) as libc::c_short;
                    }
                    if vsiz as libc::c_ulong
                        > ((*(*(*systab).vol[i as usize]).vollab).header_bytes
                            as libc::c_ulong)
                            .wrapping_sub(
                                ::core::mem::size_of::<label_block>() as libc::c_ulong,
                            )
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            | 7 as libc::c_int as libc::c_ulong
                    {
                        return -(38 as libc::c_int) as libc::c_short;
                    }
                    return DB_Expand(i, vsiz);
                }
                if strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"writelock\0\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if abs((*(*systab).vol[i as usize]).writelock)
                        == 512 as libc::c_int + 1 as libc::c_int
                    {
                        return 0 as libc::c_int as libc::c_short;
                    }
                    (*(*systab).vol[i as usize])
                        .writelock = (if cstringtob(data) != 0 {
                        -((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        0 as libc::c_int as libc::c_long
                    }) as libc::c_int;
                    return 0 as libc::c_int as libc::c_short;
                }
            }
            return -(38 as libc::c_int) as libc::c_short;
        }
        _ => {}
    }
    return -(38 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn SS_Data(
    mut var: *mut mvar,
    mut buf: *mut u_char,
) -> libc::c_short {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut s: libc::c_short = 0;
    let mut cnt: libc::c_int = 0;
    let mut tmp: [u_char; 1024] = [0; 1024];
    let mut ptmp: libc::c_int = 0 as libc::c_int;
    let mut nsubs: libc::c_int = 0 as libc::c_int;
    let mut vp: *mut mvar = 0 as *mut mvar;
    let mut subs: [*mut cstring; 4] = [0 as *mut cstring; 4];
    while i < (*var).slen as libc::c_int {
        cnt = 0 as libc::c_int;
        if nsubs > 3 as libc::c_int {
            return -(38 as libc::c_int) as libc::c_short;
        }
        subs[nsubs
            as usize] = &mut *tmp.as_mut_ptr().offset(ptmp as isize) as *mut u_char
            as *mut cstring;
        s = UTIL_Key_Extract(
            &mut *((*var).key).as_mut_ptr().offset(i as isize),
            ((*subs[nsubs as usize]).buf).as_mut_ptr(),
            &mut cnt,
        );
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        let fresh20 = nsubs;
        nsubs = nsubs + 1;
        (*subs[fresh20 as usize]).len = s as u_short;
        ptmp = (ptmp as libc::c_ulong)
            .wrapping_add(
                (s as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += cnt;
    }
    s = SS_Norm(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    match (*var).name.var_cu[1 as libc::c_int as usize] as libc::c_int {
        67 => return -(38 as libc::c_int) as libc::c_short,
        68 => return -(38 as libc::c_int) as libc::c_short,
        71 => {
            if nsubs > 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            return DB_Data(var, buf);
        }
        74 => {
            if nsubs != 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            i = cstringtoi(subs[0 as libc::c_int as usize]);
            if i < 1 as libc::c_int || i > (*systab).maxjob as libc::c_int {
                return -(23 as libc::c_int) as libc::c_short;
            }
            *buf.offset(0 as libc::c_int as isize) = '1' as i32 as u_char;
            *buf.offset(1 as libc::c_int as isize) = '\0' as i32 as u_char;
            if (*((*systab).jobtab).offset((i - 1 as libc::c_int) as isize)).pid
                == 0 as libc::c_int
            {
                *buf.offset(0 as libc::c_int as isize) = '0' as i32 as u_char;
            }
            return 1 as libc::c_int as libc::c_short;
        }
        76 => {
            if nsubs != 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            if (*subs[0 as libc::c_int as usize]).len as libc::c_int > 511 as libc::c_int
            {
                return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
            vp = &mut *tmp.as_mut_ptr().offset(512 as libc::c_int as isize)
                as *mut u_char as *mut mvar;
            s = UTIL_MvarFromCStr(subs[0 as libc::c_int as usize], vp);
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            s = UTIL_mvartolock(
                vp,
                ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr(),
            );
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            (*subs[0 as libc::c_int as usize]).len = s as u_short;
            s = LCK_Get(subs[0 as libc::c_int as usize], buf);
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            *buf
                .offset(
                    0 as libc::c_int as isize,
                ) = (if s as libc::c_int != 0 { '1' as i32 } else { '0' as i32 })
                as u_char;
            *buf.offset(1 as libc::c_int as isize) = '\0' as i32 as u_char;
            return 1 as libc::c_int as libc::c_short;
        }
        82 => {
            if nsubs > 2 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            return DB_Data(var, buf);
        }
        83 => return -(38 as libc::c_int) as libc::c_short,
        _ => {}
    }
    return -(38 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn SS_Kill(mut var: *mut mvar) -> libc::c_short {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_short = 0;
    let mut no_daemons: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut rou: var_u = VAR_U { var_q: 0 };
    let mut tmp: [u_char; 1024] = [0; 1024];
    let mut ptmp: libc::c_int = 0 as libc::c_int;
    let mut nsubs: libc::c_int = 0 as libc::c_int;
    let mut vp: *mut mvar = 0 as *mut mvar;
    let mut subs: [*mut cstring; 4] = [0 as *mut cstring; 4];
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
    while i < (*var).slen as libc::c_int {
        cnt = 0 as libc::c_int;
        if nsubs > 3 as libc::c_int {
            return -(38 as libc::c_int) as libc::c_short;
        }
        subs[nsubs
            as usize] = &mut *tmp.as_mut_ptr().offset(ptmp as isize) as *mut u_char
            as *mut cstring;
        s = UTIL_Key_Extract(
            &mut *((*var).key).as_mut_ptr().offset(i as isize),
            ((*subs[nsubs as usize]).buf).as_mut_ptr(),
            &mut cnt,
        );
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        let fresh21 = nsubs;
        nsubs = nsubs + 1;
        (*subs[fresh21 as usize]).len = s as u_short;
        ptmp = (ptmp as libc::c_ulong)
            .wrapping_add(
                (s as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += cnt;
    }
    s = SS_Norm(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    match (*var).name.var_cu[1 as libc::c_int as usize] as libc::c_int {
        67 => return -(29 as libc::c_int) as libc::c_short,
        68 => return -(29 as libc::c_int) as libc::c_short,
        71 => {
            if nsubs > 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            return -(29 as libc::c_int) as libc::c_short;
        }
        74 => {
            if nsubs > 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            if nsubs == 1 as libc::c_int {
                j = cstringtoi(subs[0 as libc::c_int as usize]) - 1 as libc::c_int;
                if j < 0 as libc::c_int || j >= (*systab).maxjob as libc::c_int {
                    return -(23 as libc::c_int) as libc::c_short;
                }
                i = (*((*systab).jobtab).offset(j as isize)).pid;
                if i == 0 as libc::c_int {
                    return -(23 as libc::c_int) as libc::c_short;
                }
                if priv_0() == 0
                    && (*((*systab).jobtab).offset(j as isize)).user
                        != (*partab.jobtab).user
                {
                    return -(29 as libc::c_int) as libc::c_short;
                }
                if kill(i, 15 as libc::c_int) == 0 {
                    return 0 as libc::c_int as libc::c_short;
                }
                (*((*systab).jobtab).offset(j as isize))
                    .trap = (1 as libc::c_uint) << 15 as libc::c_int;
                (*((*systab).jobtab).offset(j as isize)).attention = 1 as libc::c_int;
                return 0 as libc::c_int as libc::c_short;
            }
            if priv_0() == 0 {
                return -(29 as libc::c_int) as libc::c_short;
            }
            i = 1 as libc::c_int - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                no_daemons = 1 as libc::c_int;
                if !((*systab).vol[i as usize]).is_null() {
                    (*(*systab).vol[i as usize])
                        .writelock = -(512 as libc::c_int + 1 as libc::c_int);
                    while (*(*systab).vol[i as usize]).writelock < 0 as libc::c_int {
                        sleep(1 as libc::c_int as libc::c_uint);
                        j = 0 as libc::c_int;
                        while j < (*(*systab).vol[i as usize]).num_of_daemons {
                            if kill(
                                (*(*systab).vol[i as usize]).wd_tab[j as usize].pid,
                                0 as libc::c_int,
                            ) == 0 as libc::c_int
                            {
                                no_daemons = 0 as libc::c_int;
                                break;
                            } else {
                                j += 1;
                                j;
                            }
                        }
                        if no_daemons != 0 {
                            break;
                        }
                    }
                    (*(*systab).vol[i as usize])
                        .writelock = 512 as libc::c_int + 1 as libc::c_int;
                }
                i -= 1;
                i;
            }
            if shmctl(
                (*(*systab).vol[0 as libc::c_int as usize]).shm_id,
                0 as libc::c_int,
                &mut sbuf,
            ) == -(1 as libc::c_int)
            {
                return -(200 as libc::c_int + 200 as libc::c_int + *__error())
                    as libc::c_short;
            }
            (*systab).start_user = -(1 as libc::c_int);
            let mut k: u_int = 0 as libc::c_int as u_int;
            while k < (*systab).maxjob {
                cnt = (*((*systab).jobtab).offset(k as isize)).pid;
                if cnt != (*partab.jobtab).pid && cnt != 0 {
                    if kill(cnt, 15 as libc::c_int) == 0 {
                        (*((*systab).jobtab).offset(k as isize))
                            .trap = (1 as libc::c_uint) << 15 as libc::c_int;
                        (*((*systab).jobtab).offset(k as isize))
                            .attention = 1 as libc::c_int;
                    }
                }
                k = k.wrapping_add(1);
                k;
            }
            i = 1 as libc::c_int - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                if !((*systab).vol[i as usize]).is_null() {
                    DB_Dismount(i + 1 as libc::c_int);
                    if i == 0 as libc::c_int && no_daemons != 0 {
                        if semctl(
                            (*systab).sem_id,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            semvals,
                        ) == -(1 as libc::c_int)
                        {
                            fprintf(
                                __stderrp,
                                b"errno = %d %s\n\0" as *const u8 as *const libc::c_char,
                                *__error(),
                                strerror(*__error()),
                            );
                        }
                    }
                }
                i -= 1;
                i;
            }
            tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut tty_settings);
            exit(0 as libc::c_int);
        }
        76 => {
            if nsubs != 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            if priv_0() == 0 {
                return -(29 as libc::c_int) as libc::c_short;
            }
            if (*subs[0 as libc::c_int as usize]).len as libc::c_int > 511 as libc::c_int
            {
                return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
            vp = &mut *tmp.as_mut_ptr().offset(512 as libc::c_int as isize)
                as *mut u_char as *mut mvar;
            s = UTIL_MvarFromCStr(subs[0 as libc::c_int as usize], vp);
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            s = UTIL_mvartolock(
                vp,
                ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr(),
            );
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            (*subs[0 as libc::c_int as usize]).len = s as u_short;
            while SemOp(
                1 as libc::c_int,
                ((*systab).maxjob).wrapping_neg() as libc::c_int,
            ) != 0
            {
                sleep(1 as libc::c_int as libc::c_uint);
            }
            s = LCK_Kill(subs[0 as libc::c_int as usize]);
            SemOp(1 as libc::c_int, (*systab).maxjob as libc::c_int);
            return s;
        }
        82 => {
            if nsubs > 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            if (*var).slen as libc::c_int == '\0' as i32 {
                if priv_0() == 0 {
                    return -(29 as libc::c_int) as libc::c_short;
                }
                s = DB_Data(var, tmp.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    return s;
                }
                if s as libc::c_int > 1 as libc::c_int {
                    return -(33 as libc::c_int) as libc::c_short;
                }
                return DB_Kill(var);
            }
            if priv_0() == 0
                && ((*partab.jobtab).ruci as libc::c_int != (*var).uci as libc::c_int
                    || (*partab.jobtab).rvol as libc::c_int
                        != (*var).volset as libc::c_int)
            {
                return -(29 as libc::c_int) as libc::c_short;
            }
            let mut var_i: u_int = 0 as libc::c_int as u_int;
            while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                rou.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
                var_i = var_i.wrapping_add(1);
                var_i;
            }
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                rou
                    .var_cu[i
                    as usize] = (*subs[0 as libc::c_int as usize]).buf[i as usize];
                if rou.var_cu[i as usize] as libc::c_int == '\0' as i32 {
                    break;
                }
                i += 1;
                i;
            }
            s = SemOp(
                3 as libc::c_int,
                ((*systab).maxjob).wrapping_neg() as libc::c_int,
            );
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            s = DB_Kill(var);
            if s as libc::c_int >= 0 as libc::c_int {
                Routine_Delete(rou, (*var).uci as libc::c_int);
            }
            SemOp(3 as libc::c_int, (*systab).maxjob as libc::c_int);
            return s;
        }
        83 => {
            if nsubs == 4 as libc::c_int && priv_0() != 0
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"vol\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                && strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"uci\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                i = cstringtoi(subs[1 as libc::c_int as usize]) - 1 as libc::c_int;
                j = cstringtoi(subs[3 as libc::c_int as usize]) - 1 as libc::c_int;
                if i < 0 as libc::c_int || i >= 1 as libc::c_int {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                if j < 0 as libc::c_int || j >= 64 as libc::c_int {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                return DB_UCIKill(i + 1 as libc::c_int, j + 1 as libc::c_int);
            }
            if nsubs == 2 as libc::c_int && priv_0() != 0
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"vol\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                i = cstringtoi(subs[1 as libc::c_int as usize]) - 1 as libc::c_int;
                if i < 1 as libc::c_int || i >= 1 as libc::c_int {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                if ((*systab).vol[i as usize]).is_null() {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                return DB_Dismount(i + 1 as libc::c_int) as libc::c_short;
            }
            return -(38 as libc::c_int) as libc::c_short;
        }
        _ => {}
    }
    return -(38 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn SS_Order(
    mut var: *mut mvar,
    mut buf: *mut u_char,
    mut dir: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_short = 0;
    let mut cnt: libc::c_int = 0;
    let mut tmp: [u_char; 1024] = [0; 1024];
    let mut ptmp: libc::c_int = 0 as libc::c_int;
    let mut nsubs: libc::c_int = 0 as libc::c_int;
    let mut vp: *mut mvar = 0 as *mut mvar;
    let mut subs: [*mut cstring; 4] = [0 as *mut cstring; 4];
    while i < (*var).slen as libc::c_int {
        cnt = 0 as libc::c_int;
        if nsubs > 3 as libc::c_int {
            return -(38 as libc::c_int) as libc::c_short;
        }
        subs[nsubs
            as usize] = &mut *tmp.as_mut_ptr().offset(ptmp as isize) as *mut u_char
            as *mut cstring;
        s = UTIL_Key_Extract(
            &mut *((*var).key).as_mut_ptr().offset(i as isize),
            ((*subs[nsubs as usize]).buf).as_mut_ptr(),
            &mut cnt,
        );
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        let fresh22 = nsubs;
        nsubs = nsubs + 1;
        (*subs[fresh22 as usize]).len = s as u_short;
        ptmp = (ptmp as libc::c_ulong)
            .wrapping_add(
                (s as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += cnt;
    }
    s = SS_Norm(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    match (*var).name.var_cu[1 as libc::c_int as usize] as libc::c_int {
        67 => return -(38 as libc::c_int) as libc::c_short,
        68 => {
            if nsubs != 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            i = cstringtoi(subs[0 as libc::c_int as usize]);
            *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
            if dir < 0 as libc::c_int {
                if (*subs[0 as libc::c_int as usize]).buf[0 as libc::c_int as usize]
                    as libc::c_int == '\0' as i32
                {
                    i = 64 as libc::c_int;
                }
                if i == 0 as libc::c_int {
                    return 0 as libc::c_int as libc::c_short;
                }
                i -= 1 as libc::c_int;
                while i > -(1 as libc::c_int) {
                    if (*partab.jobtab).seqio[i as usize].type_0 as libc::c_int
                        != 0 as libc::c_int
                    {
                        break;
                    }
                    i -= 1;
                    i;
                }
            } else {
                if (*subs[0 as libc::c_int as usize]).buf[0 as libc::c_int as usize]
                    as libc::c_int == '\0' as i32
                {
                    return uitocstring(buf, 0 as libc::c_int as u_int) as libc::c_short;
                }
                i += 1 as libc::c_int;
                while i < 64 as libc::c_int {
                    if (*partab.jobtab).seqio[i as usize].type_0 as libc::c_int
                        != 0 as libc::c_int
                    {
                        break;
                    }
                    i += 1;
                    i;
                }
            }
            if i != 64 as libc::c_int {
                return itocstring(buf, i) as libc::c_short;
            }
            return 0 as libc::c_int as libc::c_short;
        }
        71 => {
            if nsubs != 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            return DB_Order(var, buf, dir);
        }
        74 => {
            if nsubs != 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            i = cstringtoi(subs[0 as libc::c_int as usize]);
            *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
            if dir < 0 as libc::c_int {
                if i == 0 as libc::c_int {
                    i = ((*systab).maxjob).wrapping_add(1 as libc::c_int as u_int)
                        as libc::c_int;
                }
                i = i - 2 as libc::c_int;
                while i > -(1 as libc::c_int) {
                    if (*((*systab).jobtab).offset(i as isize)).pid != 0 as libc::c_int {
                        if !(kill(
                            (*((*systab).jobtab).offset(i as isize)).pid,
                            0 as libc::c_int,
                        ) != 0 && *__error() == 3 as libc::c_int)
                        {
                            break;
                        }
                        CleanJob(i + 1 as libc::c_int);
                    }
                    i -= 1;
                    i;
                }
                i += 1;
                i;
            } else {
                while i < (*systab).maxjob as libc::c_int {
                    if (*((*systab).jobtab).offset(i as isize)).pid != 0 as libc::c_int {
                        if !(kill(
                            (*((*systab).jobtab).offset(i as isize)).pid,
                            0 as libc::c_int,
                        ) != 0 && *__error() == 3 as libc::c_int)
                        {
                            break;
                        }
                        CleanJob(i + 1 as libc::c_int);
                    }
                    i += 1;
                    i;
                }
                i += 1;
                i;
                if i > (*systab).maxjob as libc::c_int {
                    i = 0 as libc::c_int;
                }
            }
            if i != 0 {
                return itocstring(buf, i) as libc::c_short;
            }
            return 0 as libc::c_int as libc::c_short;
        }
        76 => {
            if nsubs != 1 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            if (*subs[0 as libc::c_int as usize]).len as libc::c_int > 511 as libc::c_int
            {
                return -(12 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
            vp = &mut *tmp.as_mut_ptr().offset(512 as libc::c_int as isize)
                as *mut u_char as *mut mvar;
            s = UTIL_MvarFromCStr(subs[0 as libc::c_int as usize], vp);
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            s = UTIL_mvartolock(
                vp,
                ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr(),
            );
            if (s as libc::c_int) < 0 as libc::c_int {
                return s;
            }
            (*subs[0 as libc::c_int as usize]).len = s as u_short;
            return LCK_Order(subs[0 as libc::c_int as usize], buf, dir);
        }
        82 => {
            if nsubs > 2 as libc::c_int {
                return -(38 as libc::c_int) as libc::c_short;
            }
            return DB_Order(var, buf, dir);
        }
        83 => {
            if nsubs == 2 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"vol\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                i = cstringtoi(subs[1 as libc::c_int as usize]) - 1 as libc::c_int;
                if i < -(1 as libc::c_int) || i >= 1 as libc::c_int {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
                if dir > 0 as libc::c_int {
                    j = i + 1 as libc::c_int;
                    while j < 1 as libc::c_int {
                        if !((*systab).vol[j as usize]).is_null() {
                            break;
                        }
                        j += 1;
                        j;
                    }
                    if j == 1 as libc::c_int {
                        return 0 as libc::c_int as libc::c_short;
                    }
                    return itocstring(buf, j + 1 as libc::c_int) as libc::c_short;
                }
                if i == -(1 as libc::c_int) {
                    i = 1 as libc::c_int;
                }
                j = i - 1 as libc::c_int;
                while j >= 0 as libc::c_int {
                    if !((*systab).vol[j as usize]).is_null() {
                        break;
                    }
                    j -= 1;
                    j;
                }
                if j < 0 as libc::c_int {
                    return 0 as libc::c_int as libc::c_short;
                }
                return itocstring(buf, j + 1 as libc::c_int) as libc::c_short;
            }
            if nsubs == 4 as libc::c_int
                && strncasecmp(
                    ((*subs[0 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"vol\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                && strncasecmp(
                    ((*subs[2 as libc::c_int as usize]).buf).as_mut_ptr()
                        as *mut libc::c_char,
                    b"uci\0\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                i = cstringtoi(subs[1 as libc::c_int as usize]) - 1 as libc::c_int;
                j = cstringtoi(subs[3 as libc::c_int as usize]) - 1 as libc::c_int;
                if i < 0 as libc::c_int || i >= 1 as libc::c_int {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                if j < -(1 as libc::c_int) || j >= 64 as libc::c_int {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                if ((*systab).vol[i as usize]).is_null() {
                    return -(60 as libc::c_int) as libc::c_short;
                }
                *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
                if dir > 0 as libc::c_int {
                    j = j + 1 as libc::c_int;
                    while j < 64 as libc::c_int {
                        if var_empty(
                            (*(*(*systab).vol[i as usize]).vollab).uci[j as usize].name,
                        ) == 0
                        {
                            break;
                        }
                        j += 1;
                        j;
                    }
                    if j == 64 as libc::c_int {
                        return 0 as libc::c_int as libc::c_short;
                    }
                    return itocstring(buf, j + 1 as libc::c_int) as libc::c_short;
                }
                if j == -(1 as libc::c_int) {
                    j = 64 as libc::c_int;
                }
                j = j - 1 as libc::c_int;
                while j >= 0 as libc::c_int {
                    if var_empty(
                        (*(*(*systab).vol[i as usize]).vollab).uci[j as usize].name,
                    ) == 0
                    {
                        break;
                    }
                    j -= 1;
                    j;
                }
                if j < 0 as libc::c_int {
                    return 0 as libc::c_int as libc::c_short;
                }
                return itocstring(buf, j + 1 as libc::c_int) as libc::c_short;
            }
            return -(38 as libc::c_int) as libc::c_short;
        }
        _ => {}
    }
    return -(38 as libc::c_int) as libc::c_short;
}
