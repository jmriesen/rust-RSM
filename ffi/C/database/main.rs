#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type RBD;
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
    fn close(_: libc::c_int) -> libc::c_int;
    fn lseek(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn __error() -> *mut libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    fn Get_block(blknum: u_int) -> libc::c_short;
    fn Get_GBDs(greqd: libc::c_int);
    fn Get_data(dir: libc::c_int) -> libc::c_int;
    fn Kill_data() -> libc::c_short;
    fn Locate_next() -> libc::c_short;
    fn Set_data(data: *mut cstring) -> libc::c_int;
    fn DoJournal(jj: *mut jrnrec, data: *mut cstring);
    fn Queit();
    fn Compress1() -> libc::c_short;
    fn itocstring(buf: *mut u_char, n: libc::c_int) -> u_short;
    fn uitocstring(buf: *mut u_char, n: u_int) -> u_short;
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
    fn UTIL_Key_Last(var: *mut mvar) -> libc::c_int;
    fn mcopy(src: *mut u_char, dst: *mut u_char, bytes: libc::c_int) -> libc::c_int;
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
pub type jrnrec = JRNREC;
#[no_mangle]
pub static mut curr_lock: libc::c_int = 0;
#[no_mangle]
pub static mut db_var: mvar = MVAR {
    name: VAR_U { var_q: 0 },
    volset: 0,
    uci: 0,
    slen: 0,
    key: [0; 256],
};
#[no_mangle]
pub static mut volnum: libc::c_int = 0;
#[no_mangle]
pub static mut blk: [*mut gbd; 12] = [0 as *const gbd as *mut gbd; 12];
#[no_mangle]
pub static mut level: libc::c_int = 0;
#[no_mangle]
pub static mut rekey_blk: [u_int; 36] = [0; 36];
#[no_mangle]
pub static mut rekey_lvl: [libc::c_int; 36] = [0; 36];
#[no_mangle]
pub static mut Index: u_int = 0;
#[no_mangle]
pub static mut chunk: *mut cstring = 0 as *const cstring as *mut cstring;
#[no_mangle]
pub static mut record: *mut cstring = 0 as *const cstring as *mut cstring;
#[no_mangle]
pub static mut keybuf: [u_char; 260] = [0; 260];
#[no_mangle]
pub static mut idx: *mut u_short = 0 as *const u_short as *mut u_short;
#[no_mangle]
pub static mut iidx: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut writing: libc::c_int = 0;
#[no_mangle]
pub static mut hash_start: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn Copy2local(mut var: *mut mvar) -> libc::c_short {
    let mut i: libc::c_int = 0;
    (*partab.jobtab).grefs = ((*partab.jobtab).grefs).wrapping_add(1);
    (*partab.jobtab).grefs;
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        let fresh0 = i;
        i = i + 1;
        blk[fresh0 as usize] = 0 as *mut gbd;
    }
    curr_lock = 0 as libc::c_int;
    writing = 0 as libc::c_int;
    level = -(1 as libc::c_int);
    memcpy(
        &mut db_var as *mut mvar as *mut libc::c_void,
        var as *const libc::c_void,
        (::core::mem::size_of::<var_u>() as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
            .wrapping_add((*var).slen as libc::c_ulong),
    );
    if db_var.volset as libc::c_int == 0 as libc::c_int {
        db_var.volset = (*partab.jobtab).vol;
    }
    if db_var.volset as libc::c_int > 1 as libc::c_int {
        return -(26 as libc::c_int) as libc::c_short;
    }
    if ((*systab).vol[(db_var.volset as libc::c_int - 1 as libc::c_int) as usize])
        .is_null()
    {
        return -(26 as libc::c_int) as libc::c_short;
    }
    if db_var.uci as libc::c_int == 0 as libc::c_int {
        if db_var.name.var_cu[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
            db_var.uci = 1 as libc::c_int as u_char;
        } else {
            db_var.uci = (*partab.jobtab).uci;
        }
    }
    if db_var.uci as libc::c_int > 64 as libc::c_int {
        return -(26 as libc::c_int) as libc::c_short;
    }
    if (*var).volset as libc::c_int == 0 as libc::c_int
        && (*var).uci as libc::c_int == 0 as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < (*systab).max_tt {
            if memcmp(
                &mut db_var as *mut mvar as *const libc::c_void,
                &mut *((*systab).tt).as_mut_ptr().offset(i as isize) as *mut trantab
                    as *const libc::c_void,
                (::core::mem::size_of::<var_u>() as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            {
                if (*systab).tt[i as usize].to_vol as libc::c_int == 0 as libc::c_int {
                    return (i + 1 as libc::c_int) as libc::c_short;
                }
                memcpy(
                    &mut db_var.name as *mut var_u as *mut libc::c_void,
                    (&mut *((*systab).tt).as_mut_ptr().offset(i as isize) as *mut trantab
                        as *mut libc::c_char)
                        .offset(34 as libc::c_ulong as isize) as *const libc::c_void,
                    (::core::mem::size_of::<var_u>() as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong),
                );
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    if (*(*(*systab).vol[(db_var.volset as libc::c_int - 1 as libc::c_int) as usize])
        .vollab)
        .uci[(db_var.uci as libc::c_int - 1 as libc::c_int) as usize]
        .name
        .var_cu[0 as libc::c_int as usize] as libc::c_int == '\0' as i32
    {
        return -(26 as libc::c_int) as libc::c_short;
    }
    if db_var.name.var_cu[0 as libc::c_int as usize] as libc::c_int == '%' as i32
        && db_var.uci as libc::c_int != 1 as libc::c_int
    {
        return -(26 as libc::c_int) as libc::c_short;
    }
    volnum = db_var.volset as libc::c_int;
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn DB_Get(
    mut var: *mut mvar,
    mut buf: *mut u_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = Copy2local(var) as libc::c_int;
    if s < 0 as libc::c_int {
        return s;
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .dbget = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbget)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbget;
    s = Get_data(0 as libc::c_int);
    if s >= 0 as libc::c_int {
        if memcmp(
            &mut *(db_var.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            b"$GLOBAL\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            s = uitocstring(buf, *(record as *mut u_int)) as libc::c_int;
        } else {
            s = mcopy(((*record).buf).as_mut_ptr(), buf, (*record).len as libc::c_int);
        }
    }
    if curr_lock != 0 {
        SemOp(2 as libc::c_int, -curr_lock);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn DB_Set(
    mut var: *mut mvar,
    mut data: *mut cstring,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    s = Copy2local(var) as libc::c_int;
    if s < 0 as libc::c_int {
        return s;
    }
    i = 4 as libc::c_int + db_var.slen as libc::c_int + 2 as libc::c_int
        + (*data).len as libc::c_int;
    if i & 3 as libc::c_int != 0 {
        i += 4 as libc::c_int - (i & 3 as libc::c_int);
    }
    i += 4 as libc::c_int;
    if i
        > ((*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).block_size
            as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<DB_Block>() as libc::c_ulong)
            as libc::c_int
    {
        return -(75 as libc::c_int);
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .dbset = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbset)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbset;
    writing = 1 as libc::c_int;
    while (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).writelock != 0 {
        sleep(1 as libc::c_int as libc::c_uint);
        if (*partab.jobtab).attention != 0 {
            return -(51 as libc::c_int + 200 as libc::c_int);
        }
    }
    i = ((*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).max_block
        >> 3 as libc::c_int) as libc::c_int;
    while i != 0 {
        let fresh1 = i;
        i = i - 1;
        if *((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map as *mut u_char)
            .offset(fresh1 as isize) as libc::c_int == 0 as libc::c_int
        {
            break;
        }
    }
    if i == 0 {
        return -(11 as libc::c_int + 200 as libc::c_int);
    }
    s = Set_data(data);
    if curr_lock != 0 {
        SemOp(2 as libc::c_int, -curr_lock);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn DB_Data(
    mut var: *mut mvar,
    mut buf: *mut u_char,
) -> libc::c_short {
    let mut s: libc::c_short = 0;
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    s = Copy2local(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .dbdat = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbdat)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbdat;
    t = Get_data(0 as libc::c_int);
    i = 1 as libc::c_int;
    if t == -(7 as libc::c_int) {
        i = 0 as libc::c_int;
        if level == 0 as libc::c_int
            && memcmp(
                &mut *(db_var.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                b"$GLOBAL\0\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
        {
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            *buf.offset(0 as libc::c_int as isize) = '0' as i32 as u_char;
            *buf.offset(1 as libc::c_int as isize) = '\0' as i32 as u_char;
            return 1 as libc::c_int as libc::c_short;
        }
    } else if t < 0 as libc::c_int {
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        return t as libc::c_short;
    }
    if db_var.slen == 0 && i == 0 {
        Index = Index.wrapping_add(1);
        Index;
    }
    if i != 0 || Index > (*(*blk[level as usize]).mem).0.last_idx as u_int {
        s = Locate_next();
        if s as libc::c_int == -(7 as libc::c_int) {
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return itocstring(buf, i) as libc::c_short;
        } else if (s as libc::c_int) < 0 as libc::c_int {
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return s;
        }
    }
    if (db_var.slen as libc::c_int) < keybuf[0 as libc::c_int as usize] as libc::c_int
        && memcmp(
            &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut u_char
                as *const libc::c_void,
            (db_var.key).as_mut_ptr() as *const libc::c_void,
            db_var.slen as libc::c_ulong,
        ) == 0 as libc::c_int || db_var.slen == 0
    {
        i += 10 as libc::c_int;
    }
    if curr_lock != 0 {
        SemOp(2 as libc::c_int, -curr_lock);
    }
    return itocstring(buf, i) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn DB_Kill(mut var: *mut mvar) -> libc::c_short {
    let mut s: libc::c_int = 0;
    s = Copy2local(var) as libc::c_int;
    if s < 0 as libc::c_int {
        return s as libc::c_short;
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .dbkil = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbkil)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbkil;
    while (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).writelock != 0 {
        sleep(1 as libc::c_int as libc::c_uint);
        if (*partab.jobtab).attention != 0 {
            return -(51 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
    }
    s = Get_data(0 as libc::c_int);
    if s == -(7 as libc::c_int) && level == 0 as libc::c_int
        || s < 0 as libc::c_int && s != -(7 as libc::c_int)
    {
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        if s == -(7 as libc::c_int) {
            s = 0 as libc::c_int;
        }
        return s as libc::c_short;
    }
    if s == -(7 as libc::c_int) && db_var.slen as libc::c_int != 0 {
        if Index <= (*(*blk[level as usize]).mem).0.last_idx as u_int {
            if db_var.slen as libc::c_int
                > keybuf[0 as libc::c_int as usize] as libc::c_int
                || memcmp(
                    &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    (db_var.key).as_mut_ptr() as *const libc::c_void,
                    db_var.slen as libc::c_ulong,
                ) != 0
            {
                if curr_lock != 0 {
                    SemOp(2 as libc::c_int, -curr_lock);
                }
                return 0 as libc::c_int as libc::c_short;
            }
        } else {
            s = Locate_next() as libc::c_int;
            if s == 0 {
                if db_var.slen as libc::c_int
                    > keybuf[0 as libc::c_int as usize] as libc::c_int
                    || memcmp(
                        &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize)
                            as *mut u_char as *const libc::c_void,
                        (db_var.key).as_mut_ptr() as *const libc::c_void,
                        db_var.slen as libc::c_ulong,
                    ) != 0
                {
                    s = -(7 as libc::c_int);
                }
            }
            if s < 0 as libc::c_int {
                if curr_lock != 0 {
                    SemOp(2 as libc::c_int, -curr_lock);
                }
                if s == -(7 as libc::c_int) {
                    s = 0 as libc::c_int;
                }
                return 0 as libc::c_int as libc::c_short;
            }
        }
    }
    s = Kill_data() as libc::c_int;
    if curr_lock != 0 {
        SemOp(2 as libc::c_int, -curr_lock);
    }
    return s as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn DB_Order(
    mut var: *mut mvar,
    mut buf: *mut u_char,
    mut dir: libc::c_int,
) -> libc::c_short {
    let mut s: libc::c_short = 0;
    let mut t: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut last_key: libc::c_int = 0;
    s = Copy2local(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .dbord = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbord)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbord;
    last_key = UTIL_Key_Last(&mut db_var);
    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
    if dir < 0 as libc::c_int {
        t = Get_data(-(1 as libc::c_int));
        if t < 0 as libc::c_int && t != -(7 as libc::c_int) {
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return t as libc::c_short;
        }
        if level == 0 as libc::c_int && s as libc::c_int == -(7 as libc::c_int)
            && memcmp(
                &mut *(db_var.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                b"$GLOBAL\0\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            ) != 0
        {
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return 0 as libc::c_int as libc::c_short;
        }
        Index = Index.wrapping_sub(1);
        Index;
        if Index
            < ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                / 2 as libc::c_int as u_int
        {
            panic(
                b"DB_Order: Problem with negative direction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        chunk = &mut *iidx.offset(*idx.offset(Index as isize) as isize)
            as *mut libc::c_int as *mut cstring;
        record = &mut *((*chunk).buf)
            .as_mut_ptr()
            .offset(
                (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as libc::c_int + 4 as libc::c_int) as isize,
            ) as *mut u_char as *mut cstring;
    } else {
        let fresh2 = db_var.slen;
        db_var.slen = (db_var.slen).wrapping_add(1);
        db_var.key[fresh2 as usize] = 255 as libc::c_int as u_char;
        t = Get_data(0 as libc::c_int);
        if t != -(7 as libc::c_int) {
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return (if t < 0 as libc::c_int {
                t as libc::c_short as libc::c_int
            } else {
                -(61 as libc::c_int + 200 as libc::c_int)
            }) as libc::c_short;
        }
        if level == 0 as libc::c_int
            && memcmp(
                &mut *(db_var.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                b"$GLOBAL\0\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            ) != 0
        {
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return 0 as libc::c_int as libc::c_short;
        }
        if Index > (*(*blk[level as usize]).mem).0.last_idx as u_int {
            s = Locate_next();
            if (s as libc::c_int) < 0 as libc::c_int {
                if curr_lock != 0 {
                    SemOp(2 as libc::c_int, -curr_lock);
                }
                return (if s as libc::c_int == -(7 as libc::c_int) {
                    0 as libc::c_int
                } else {
                    s as libc::c_int
                }) as libc::c_short;
            }
        }
    }
    let mut i: u_int = ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int;
    while i <= Index {
        chunk = &mut *iidx.offset(*idx.offset(i as isize) as isize) as *mut libc::c_int
            as *mut cstring;
        memcpy(
            &mut *keybuf
                .as_mut_ptr()
                .offset(
                    (*((*chunk).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 1 as libc::c_int) as isize,
                ) as *mut u_char as *mut libc::c_void,
            &mut *((*chunk).buf).as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            (*chunk).buf[1 as libc::c_int as usize] as libc::c_ulong,
        );
        keybuf[0 as libc::c_int
            as usize] = ((*chunk).buf[0 as libc::c_int as usize] as libc::c_int
            + (*chunk).buf[1 as libc::c_int as usize] as libc::c_int) as u_char;
        i = i.wrapping_add(1);
        i;
    }
    if curr_lock != 0 {
        SemOp(2 as libc::c_int, -curr_lock);
    }
    if (keybuf[0 as libc::c_int as usize] as libc::c_int) < last_key + 1 as libc::c_int
        || memcmp(
            &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut u_char
                as *const libc::c_void,
            (db_var.key).as_mut_ptr() as *const libc::c_void,
            last_key as libc::c_ulong,
        ) != 0
    {
        return 0 as libc::c_int as libc::c_short;
    }
    cnt = 0 as libc::c_int;
    return UTIL_Key_Extract(
        &mut *keybuf.as_mut_ptr().offset((last_key + 1 as libc::c_int) as isize),
        buf,
        &mut cnt,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DB_Query(
    mut var: *mut mvar,
    mut buf: *mut u_char,
    mut dir: libc::c_int,
) -> libc::c_short {
    let mut s: libc::c_short = 0;
    let mut t: libc::c_int = 0;
    s = Copy2local(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .dbqry = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbqry)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbqry;
    if dir < 0 as libc::c_int {
        t = Get_data(-(1 as libc::c_int));
        if t < 0 as libc::c_int && t != -(7 as libc::c_int) {
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return t as libc::c_short;
        }
        if level == 0 as libc::c_int && t == -(7 as libc::c_int) {
            *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return 0 as libc::c_int as libc::c_short;
        }
        Index = Index.wrapping_sub(1);
        Index;
        if Index
            < ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                / 2 as libc::c_int as u_int
        {
            panic(
                b"DB_Query: Problem with negative direction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        chunk = &mut *iidx.offset(*idx.offset(Index as isize) as isize)
            as *mut libc::c_int as *mut cstring;
        record = &mut *((*chunk).buf)
            .as_mut_ptr()
            .offset(
                (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as libc::c_int + 4 as libc::c_int) as isize,
            ) as *mut u_char as *mut cstring;
        if (*chunk).buf[0 as libc::c_int as usize] == 0
            && (*chunk).buf[1 as libc::c_int as usize] == 0
            && (*partab.jobtab).last_block_flags & 2 as libc::c_int as u_int
                == 0 as libc::c_int as u_int
        {
            *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return 0 as libc::c_int as libc::c_short;
        }
    } else {
        t = Get_data(0 as libc::c_int);
        if t < 0 as libc::c_int && t != -(7 as libc::c_int) {
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return t as libc::c_short;
        }
        if level == 0 as libc::c_int && s as libc::c_int == -(7 as libc::c_int) {
            *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
            if curr_lock != 0 {
                SemOp(2 as libc::c_int, -curr_lock);
            }
            return 0 as libc::c_int as libc::c_short;
        }
        if t < 0 as libc::c_int && db_var.slen == 0 {
            Index = Index.wrapping_add(1);
            Index;
        }
        if Index > (*(*blk[level as usize]).mem).0.last_idx as u_int
            || t >= 0 as libc::c_int
        {
            s = Locate_next();
            if (s as libc::c_int) < 0 as libc::c_int {
                if curr_lock != 0 {
                    SemOp(2 as libc::c_int, -curr_lock);
                }
                *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
                if s as libc::c_int == -(7 as libc::c_int) {
                    s = 0 as libc::c_int as libc::c_short;
                }
                return s;
            }
        }
    }
    let mut i: u_int = ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int;
    while i <= Index {
        chunk = &mut *iidx.offset(*idx.offset(i as isize) as isize) as *mut libc::c_int
            as *mut cstring;
        memcpy(
            &mut *keybuf
                .as_mut_ptr()
                .offset(
                    (*((*chunk).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 1 as libc::c_int) as isize,
                ) as *mut u_char as *mut libc::c_void,
            &mut *((*chunk).buf).as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            (*chunk).buf[1 as libc::c_int as usize] as libc::c_ulong,
        );
        keybuf[0 as libc::c_int
            as usize] = ((*chunk).buf[0 as libc::c_int as usize] as libc::c_int
            + (*chunk).buf[1 as libc::c_int as usize] as libc::c_int) as u_char;
        i = i.wrapping_add(1);
        i;
    }
    if curr_lock != 0 {
        SemOp(2 as libc::c_int, -curr_lock);
    }
    db_var.uci = (*var).uci;
    db_var.volset = (*var).volset;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        db_var.name.var_qu[var_i as usize] = (*var).name.var_qu[var_i as usize];
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    db_var.slen = keybuf[0 as libc::c_int as usize];
    memcpy(
        &mut *(db_var.key).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
            as *mut libc::c_void,
        &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut u_char
            as *const libc::c_void,
        keybuf[0 as libc::c_int as usize] as libc::c_ulong,
    );
    return UTIL_String_Mvar(&mut db_var, buf, 63 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn DB_QueryD(
    mut var: *mut mvar,
    mut buf: *mut u_char,
) -> libc::c_short {
    let mut s: libc::c_short = 0;
    let mut t: libc::c_int = 0;
    s = Copy2local(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s;
    }
    t = Get_data(0 as libc::c_int);
    if t < 0 as libc::c_int && t != -(7 as libc::c_int) {
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        return t as libc::c_short;
    }
    if level == 0 as libc::c_int && t == -(7 as libc::c_int) {
        *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as u_char;
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        return -(55 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if t < 0 as libc::c_int && db_var.slen as libc::c_int != 0 {
        Index = Index.wrapping_sub(1);
        Index;
    }
    s = Locate_next();
    if (s as libc::c_int) < 0 as libc::c_int {
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        if s as libc::c_int == -(7 as libc::c_int) {
            s = -(55 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        return s;
    }
    memcpy(
        ((*var).key).as_mut_ptr() as *mut libc::c_void,
        &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut u_char
            as *const libc::c_void,
        keybuf[0 as libc::c_int as usize] as libc::c_int as libc::c_ulong,
    );
    (*var).slen = keybuf[0 as libc::c_int as usize];
    t = mcopy(((*record).buf).as_mut_ptr(), buf, (*record).len as libc::c_int);
    if curr_lock != 0 {
        SemOp(2 as libc::c_int, -curr_lock);
    }
    return t as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn DB_GetLen(
    mut var: *mut mvar,
    mut lock: libc::c_int,
    mut buf: *mut u_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sav: libc::c_int = 0;
    if lock == -(1 as libc::c_int) && buf.is_null() {
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        return 0 as libc::c_int;
    }
    sav = curr_lock;
    s = Copy2local(var) as libc::c_int;
    curr_lock = sav;
    if s < 0 as libc::c_int {
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        return s;
    }
    s = Get_data(0 as libc::c_int);
    if s < 0 as libc::c_int {
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        return s;
    }
    if !buf.is_null() {
        s = mcopy(((*record).buf).as_mut_ptr(), buf, (*record).len as libc::c_int);
    }
    if lock != 1 as libc::c_int && curr_lock != 0 {
        SemOp(2 as libc::c_int, -curr_lock);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn DB_Free(mut vol: libc::c_int) -> libc::c_int {
    let mut s: libc::c_short = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    s = SemOp(2 as libc::c_int, -(1 as libc::c_int));
    if (s as libc::c_int) < 0 as libc::c_int {
        return s as libc::c_int;
    }
    let mut i: u_int = 1 as libc::c_int as u_int;
    while i <= (*(*(*systab).vol[(vol - 1 as libc::c_int) as usize]).vollab).max_block {
        count
            += (*((*(*systab).vol[(vol - 1 as libc::c_int) as usize]).map as *mut u_char)
                .offset((i >> 3 as libc::c_int) as isize) as libc::c_uint
                & (1 as libc::c_uint) << (i & 7 as libc::c_int as u_int)
                == 0 as libc::c_int as libc::c_uint) as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    SemOp(2 as libc::c_int, -curr_lock);
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn DB_Expand(
    mut vol: libc::c_int,
    mut vsiz: u_int,
) -> libc::c_short {
    let mut fptr: off_t = 0;
    let mut fres: off_t = 0;
    let mut vexp: u_int = 0;
    let mut p: *mut u_char = 0 as *mut u_char;
    let mut dbfd: libc::c_int = 0;
    p = malloc((*(*(*systab).vol[vol as usize]).vollab).block_size as libc::c_ulong)
        as *mut u_char;
    if p.is_null() {
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        (*(*(*systab).vol[vol as usize]).vollab).block_size as libc::c_ulong,
    );
    dbfd = open(
        ((*(*systab).vol[vol as usize]).file_name).as_mut_ptr(),
        0x2 as libc::c_int,
    );
    if dbfd == -(1 as libc::c_int) {
        free(p as *mut libc::c_void);
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    fptr = (*(*(*systab).vol[vol as usize]).vollab).max_block as off_t;
    fptr = fptr * (*(*(*systab).vol[vol as usize]).vollab).block_size as off_t
        + (*(*(*systab).vol[vol as usize]).vollab).header_bytes as off_t;
    fres = lseek(dbfd, fptr, 0 as libc::c_int);
    if fres != fptr {
        free(p as *mut libc::c_void);
        return -(200 as libc::c_int + 200 as libc::c_int + *__error()) as libc::c_short;
    }
    vexp = vsiz.wrapping_sub((*(*(*systab).vol[vol as usize]).vollab).max_block);
    while vexp != 0 {
        let mut i: libc::c_int = write(
            dbfd,
            p as *const libc::c_void,
            (*(*(*systab).vol[vol as usize]).vollab).block_size as size_t,
        ) as libc::c_int;
        if i == -(1 as libc::c_int) {
            free(p as *mut libc::c_void);
            return -(200 as libc::c_int + 200 as libc::c_int + *__error())
                as libc::c_short;
        }
        vexp = vexp.wrapping_sub(1);
        vexp;
    }
    free(p as *mut libc::c_void);
    close(dbfd);
    (*(*(*systab).vol[vol as usize]).vollab).max_block = vsiz;
    (*(*systab).vol[vol as usize]).map_dirty_flag = 1 as libc::c_int;
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn DB_Dismount(mut vol: libc::c_int) -> libc::c_int {
    DB_StopJournal(vol, 3 as libc::c_int as u_char);
    (*(*systab).vol[(vol - 1 as libc::c_int) as usize]).dismount_flag = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DB_StopJournal(mut vol: libc::c_int, mut action: u_char) {
    let mut jj: jrnrec = JRNREC {
        size: 0,
        action: 0,
        uci: 0,
        time: 0,
        name: VAR_U { var_q: 0 },
        slen: 0,
        key: [0; 256],
    };
    volnum = vol;
    if (*(*(*systab).vol[(vol - 1 as libc::c_int) as usize]).vollab).journal_available
        == 0
    {
        return;
    }
    while SemOp(2 as libc::c_int, ((*systab).maxjob).wrapping_neg() as libc::c_int) != 0
    {
        sleep(1 as libc::c_int as libc::c_uint);
    }
    jj.action = action;
    jj.uci = 0 as libc::c_int as u_char;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        jj.name.var_qu[var_i as usize] = 0 as libc::c_int as u_int64;
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    jj.slen = 0 as libc::c_int as u_char;
    DoJournal(&mut jj, 0 as *mut cstring);
    (*(*(*systab).vol[(vol - 1 as libc::c_int) as usize]).vollab)
        .journal_available = 0 as libc::c_int as u_char;
}
#[no_mangle]
pub unsafe extern "C" fn DB_GetFlags(mut var: *mut mvar) -> libc::c_int {
    let mut s: libc::c_short = 0;
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    s = Copy2local(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s as libc::c_int;
    }
    t = Get_data(0 as libc::c_int);
    if t < 0 as libc::c_int && t != -(7 as libc::c_int) {
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        return t;
    }
    i = *(record as *mut libc::c_int).offset(1 as libc::c_int as isize);
    if curr_lock != 0 {
        SemOp(2 as libc::c_int, -curr_lock);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn DB_SetFlags(
    mut var: *mut mvar,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut clearit: libc::c_int = 0 as libc::c_int;
    let mut s: libc::c_short = 0;
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    if flags < 0 as libc::c_int {
        clearit = 1 as libc::c_int;
        flags = -flags;
    }
    s = Copy2local(var);
    if (s as libc::c_int) < 0 as libc::c_int {
        return s as libc::c_int;
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .dbset = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbset)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.dbset;
    writing = 1 as libc::c_int;
    while (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).writelock != 0 {
        sleep(1 as libc::c_int as libc::c_uint);
        if (*partab.jobtab).attention != 0 {
            return -(51 as libc::c_int + 200 as libc::c_int);
        }
    }
    Get_GBDs(1 as libc::c_int);
    t = Get_data(0 as libc::c_int);
    if t < 0 as libc::c_int && t != -(7 as libc::c_int) {
        if curr_lock != 0 {
            SemOp(2 as libc::c_int, -curr_lock);
        }
        return t;
    }
    i = *(record as *mut libc::c_int).offset(1 as libc::c_int as isize);
    if clearit != 0 {
        i = i & !flags;
    } else {
        i = i | flags;
    }
    *(record as *mut libc::c_int).offset(1 as libc::c_int as isize) = i;
    if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
        (*blk[level as usize]).dirty = blk[level as usize];
        Queit();
    }
    SemOp(2 as libc::c_int, -curr_lock);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn DB_Compress(
    mut var: *mut mvar,
    mut flags: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut retlevel: libc::c_short = 0;
    flags &= 15 as libc::c_int;
    s = Copy2local(var) as libc::c_int;
    if s < 0 as libc::c_int {
        return s as libc::c_short;
    }
    memset(
        rekey_blk.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((12 as libc::c_int * 3 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<u_int>() as libc::c_ulong),
    );
    memset(
        rekey_lvl.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((12 as libc::c_int * 3 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    memcpy(
        var as *mut libc::c_void,
        &mut db_var as *mut mvar as *const libc::c_void,
        ::core::mem::size_of::<mvar>() as libc::c_ulong,
    );
    s = Get_data(flags);
    retlevel = level as libc::c_short;
    if level == 0 {
        SemOp(2 as libc::c_int, -curr_lock);
        return -(7 as libc::c_int) as libc::c_short;
    }
    chunk = &mut *iidx
        .offset(
            *idx
                .offset(
                    (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                        / 2 as libc::c_int as u_int) as isize,
                ) as isize,
        ) as *mut libc::c_int as *mut cstring;
    memcpy(
        &mut (*var).slen as *mut u_char as *mut libc::c_void,
        &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut u_char as *const libc::c_void,
        ((*chunk).buf[1 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong,
    );
    loop {
        memcpy(
            &mut db_var as *mut mvar as *mut libc::c_void,
            var as *const libc::c_void,
            ::core::mem::size_of::<mvar>() as libc::c_ulong,
        );
        writing = 0 as libc::c_int;
        while (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).writelock != 0 {
            sleep(1 as libc::c_int as libc::c_uint);
            if (*partab.jobtab).attention != 0 {
                return -(51 as libc::c_int + 200 as libc::c_int) as libc::c_short;
            }
        }
        if (*partab.jobtab).attention != 0 {
            return -(51 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        s = Get_data(retlevel as libc::c_int);
        if s == -(7 as libc::c_int) && db_var.slen == 0 {
            s = 0 as libc::c_int;
        }
        if s == -(7 as libc::c_int) {
            if (*(*blk[level as usize]).mem).0.right_ptr != 0 {
                chunk = &mut *iidx
                    .offset(
                        *idx
                            .offset(
                                (::core::mem::size_of::<DB_Block>() as libc::c_ulong
                                    as u_int / 2 as libc::c_int as u_int) as isize,
                            ) as isize,
                    ) as *mut libc::c_int as *mut cstring;
                memcpy(
                    &mut db_var.slen as *mut u_char as *mut libc::c_void,
                    &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    ((*chunk).buf[1 as libc::c_int as usize] as libc::c_int
                        + 1 as libc::c_int) as libc::c_ulong,
                );
                SemOp(2 as libc::c_int, -curr_lock);
            } else {
                SemOp(2 as libc::c_int, -curr_lock);
                return retlevel;
            }
        } else {
            if s < 0 as libc::c_int {
                SemOp(2 as libc::c_int, -curr_lock);
                return s as libc::c_short;
            }
            if (*(*blk[level as usize]).mem).0.right_ptr == 0 {
                SemOp(2 as libc::c_int, -curr_lock);
                if retlevel as libc::c_int == 2 as libc::c_int && db_var.slen == 0 {
                    s = Compress1() as libc::c_int;
                    SemOp(2 as libc::c_int, -curr_lock);
                    if s < 0 as libc::c_int {
                        return s as libc::c_short;
                    }
                }
                return retlevel;
            }
            level += 1;
            level;
            s = Get_block((*(*blk[(level - 1 as libc::c_int) as usize]).mem).0.right_ptr)
                as libc::c_int;
            if s < 0 as libc::c_int {
                SemOp(2 as libc::c_int, -curr_lock);
                return s as libc::c_short;
            }
            i = ((*(*blk[(level - 1 as libc::c_int) as usize]).mem).0.last_free
                as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                - (*(*blk[(level - 1 as libc::c_int) as usize]).mem).0.last_idx
                    as libc::c_int) * 2 as libc::c_int
                + ((*(*blk[level as usize]).mem).0.last_free as libc::c_int
                    * 2 as libc::c_int + 1 as libc::c_int
                    - (*(*blk[level as usize]).mem).0.last_idx as libc::c_int)
                    * 2 as libc::c_int;
            if i < 1024 as libc::c_int {
                chunk = &mut *iidx
                    .offset(
                        *idx
                            .offset(
                                (::core::mem::size_of::<DB_Block>() as libc::c_ulong
                                    as u_int / 2 as libc::c_int as u_int) as isize,
                            ) as isize,
                    ) as *mut libc::c_int as *mut cstring;
                memcpy(
                    &mut (*var).slen as *mut u_char as *mut libc::c_void,
                    &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    ((*chunk).buf[1 as libc::c_int as usize] as libc::c_int
                        + 1 as libc::c_int) as libc::c_ulong,
                );
                SemOp(2 as libc::c_int, -curr_lock);
            } else {
                level = retlevel as libc::c_int;
                SemOp(2 as libc::c_int, -curr_lock);
                s = Compress1() as libc::c_int;
                SemOp(2 as libc::c_int, -curr_lock);
                if s < 0 as libc::c_int {
                    return s as libc::c_short;
                }
                if (*var).volset == 0 {
                    return retlevel;
                }
            }
        }
    };
}
