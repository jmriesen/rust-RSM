#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    pub type RBD;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(_: libc::c_int) -> libc::c_int;
    fn lseek(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn __error() -> *mut libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    static mut db_var: mvar;
    static mut volnum: libc::c_int;
    static mut blk: [*mut gbd; 12];
    static mut level: libc::c_int;
    static mut Index: u_int;
    static mut chunk: *mut cstring;
    static mut record: *mut cstring;
    static mut keybuf: [u_char; 260];
    static mut idx: *mut u_short;
    static mut iidx: *mut libc::c_int;
    static mut writing: libc::c_int;
    fn Get_block(blknum: u_int) -> libc::c_short;
    fn Get_GBD();
    fn Get_GBDs(greqd: libc::c_int);
    fn Free_GBD(free: *mut gbd);
    fn Get_data(dir: libc::c_int) -> libc::c_int;
    fn Locate(key: *mut u_char) -> libc::c_short;
    fn Add_rekey(block: u_int, level_0: libc::c_int) -> libc::c_short;
    fn Re_key() -> libc::c_short;
    fn Un_key();
    fn current_time(local: libc::c_short) -> time_t;
    fn panic(msg: *mut libc::c_char);
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct C2RustUnnamed {
    pub magic: u_int,
    pub free: off_t,
}
#[no_mangle]
pub unsafe extern "C" fn Insert(
    mut key: *mut u_char,
    mut data: *mut cstring,
) -> libc::c_short {
    let mut isdata: libc::c_int = 0;
    let mut rs: libc::c_int = 0;
    let mut ccc: u_char = 0;
    let mut ucc: u_char = 0;
    let mut flags: u_int = 0 as libc::c_int as u_int;
    isdata = ((*(*blk[level as usize]).mem).0.type_0 as libc::c_int > 64 as libc::c_int
        && level != 0) as libc::c_int;
    if (*(*blk[level as usize]).mem).0.last_idx as u_int
        > (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
            / 2 as libc::c_int as u_int)
            .wrapping_sub(1 as libc::c_int as u_int)
    {
        let mut s: libc::c_short = Locate(key);
        if s as libc::c_int >= 0 as libc::c_int {
            return -(61 as libc::c_int + 200 as libc::c_int) as libc::c_short
        } else if s as libc::c_int != -(7 as libc::c_int) {
            return s
        }
    } else {
        Index = ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
            / 2 as libc::c_int as u_int;
        idx = (*blk[level as usize]).mem as *mut u_short;
        iidx = (*blk[level as usize]).mem as *mut libc::c_int;
    }
    if level == 0 {
        chunk = &mut *iidx
            .offset(
                *idx
                    .offset(
                        (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                            / 2 as libc::c_int as u_int) as isize,
                    ) as isize,
            ) as *mut libc::c_int as *mut cstring;
        record = &mut *((*chunk).buf)
            .as_mut_ptr()
            .offset(
                (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as libc::c_int + 2 as libc::c_int) as isize,
            ) as *mut u_char as *mut cstring;
        Align_record();
        flags = *(record as *mut u_int).offset(1 as libc::c_int as isize);
        (*partab.jobtab).last_block_flags = flags;
    }
    keybuf[0 as libc::c_int as usize] = 0 as libc::c_int as u_char;
    let mut i: u_int = ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int;
    while i < Index {
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
    ccc = 0 as libc::c_int as u_char;
    if *key.offset(0 as libc::c_int as isize) as libc::c_int != 0
        && keybuf[0 as libc::c_int as usize] as libc::c_int != 0
    {
        while *key.offset((ccc as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int
            == keybuf[(ccc as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
        {
            if ccc as libc::c_int
                == *key.offset(0 as libc::c_int as isize) as libc::c_int
                || ccc as libc::c_int == keybuf[0 as libc::c_int as usize] as libc::c_int
            {
                break;
            }
            ccc = ccc.wrapping_add(1);
            ccc;
        }
    }
    ucc = (*key.offset(0 as libc::c_int as isize) as libc::c_int - ccc as libc::c_int)
        as u_char;
    rs = (::core::mem::size_of::<u_short>() as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(ucc as libc::c_ulong)
        .wrapping_add((*data).len as libc::c_ulong) as libc::c_int;
    if isdata != 0 {
        rs = (rs as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<u_short>() as libc::c_ulong)
            as libc::c_int as libc::c_int;
    } else if level == 0 {
        rs += 4 as libc::c_int;
    }
    if rs & 3 as libc::c_int != 0 {
        rs += 4 as libc::c_int - (rs & 3 as libc::c_int);
    }
    rs += 4 as libc::c_int;
    if rs > 65534 as libc::c_int {
        return -(75 as libc::c_int) as libc::c_short;
    }
    if rs
        > ((*(*blk[level as usize]).mem).0.last_free as libc::c_int * 2 as libc::c_int
            + 1 as libc::c_int - (*(*blk[level as usize]).mem).0.last_idx as libc::c_int)
            * 2 as libc::c_int
    {
        if (*(*blk[level as usize]).mem).0.flags as libc::c_int & 1 as libc::c_int == 0 {
            return -(62 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
        Tidy_block();
        if rs
            > ((*(*blk[level as usize]).mem).0.last_free as libc::c_int
                * 2 as libc::c_int + 1 as libc::c_int
                - (*(*blk[level as usize]).mem).0.last_idx as libc::c_int)
                * 2 as libc::c_int
        {
            return -(62 as libc::c_int + 200 as libc::c_int) as libc::c_short;
        }
    }
    rs -= 4 as libc::c_int;
    let mut i_0: u_int = (*(*blk[level as usize]).mem).0.last_idx as u_int;
    while i_0 >= Index {
        *idx
            .offset(
                i_0.wrapping_add(1 as libc::c_int as u_int) as isize,
            ) = *idx.offset(i_0 as isize);
        i_0 = i_0.wrapping_sub(1);
        i_0;
    }
    *idx
        .offset(
            Index as isize,
        ) = ((*(*blk[level as usize]).mem).0.last_free as libc::c_int
        - rs / 4 as libc::c_int + 1 as libc::c_int) as u_short;
    chunk = &mut *iidx.offset(*idx.offset(Index as isize) as isize) as *mut libc::c_int
        as *mut cstring;
    record = &mut *((*chunk).buf)
        .as_mut_ptr()
        .offset((ucc as libc::c_int + 2 as libc::c_int) as isize) as *mut u_char
        as *mut cstring;
    (*chunk).len = rs as u_short;
    (*chunk).buf[0 as libc::c_int as usize] = ccc;
    (*chunk).buf[1 as libc::c_int as usize] = ucc;
    memcpy(
        &mut *((*chunk).buf).as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut u_char as *mut libc::c_void,
        &mut *key.offset((ccc as libc::c_int + 1 as libc::c_int) as isize) as *mut u_char
            as *const libc::c_void,
        ucc as libc::c_ulong,
    );
    if isdata != 0 {
        (*record).len = (*data).len;
        memcpy(
            ((*record).buf).as_mut_ptr() as *mut libc::c_void,
            ((*data).buf).as_mut_ptr() as *const libc::c_void,
            (*data).len as libc::c_ulong,
        );
    } else {
        Align_record();
        memcpy(
            record as *mut libc::c_void,
            ((*data).buf).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        );
        if level == 0 {
            *(record as *mut u_int).offset(1 as libc::c_int as isize) = flags;
        }
    }
    (*(*blk[level as usize]).mem)
        .0
        .last_free = ((*(*blk[level as usize]).mem).0.last_free as libc::c_int
        - rs / 4 as libc::c_int) as u_short;
    (*(*blk[level as usize]).mem)
        .0
        .last_idx = ((*(*blk[level as usize]).mem).0.last_idx).wrapping_add(1);
    (*(*blk[level as usize]).mem).0.last_idx;
    (*(*blk[level as usize]).mem)
        .0
        .flags = ((*(*blk[level as usize]).mem).0.flags as libc::c_int
        | 1 as libc::c_int) as u_char;
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Queit() {
    let mut i: libc::c_int = 0;
    let mut ptr: *mut gbd = 0 as *mut gbd;
    ptr = blk[level as usize];
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .logwt = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.logwt)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.logwt;
    while (*ptr).dirty != ptr {
        ptr = (*ptr).dirty;
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .stats
            .logwt = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.logwt)
            .wrapping_add(1);
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.logwt;
    }
    i = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).dirtyQw;
    while !((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).dirtyQ[i as usize])
        .is_null()
    {
        sleep(1 as libc::c_int as libc::c_uint);
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .dirtyQ[i as usize] = blk[level as usize];
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .dirtyQw = i + 1 as libc::c_int & 1024 as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Garbit(mut blknum: u_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQw;
    j = 0 as libc::c_int;
    while !((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQ[i as usize]
        == 0 as libc::c_int as u_int)
    {
        if j == 9 as libc::c_int {
            panic(
                b"Garbit: could not get a garbage slot after 10 seconds\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        sleep(1 as libc::c_int as libc::c_uint);
        j += 1;
        j;
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQ[i as usize] = blknum;
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .garbQw = i + 1 as libc::c_int & 8192 as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Free_block(mut blknum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut map: *mut u_char = 0 as *mut u_char;
    map = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map as *mut u_char;
    i = blknum >> 3 as libc::c_int;
    off = blknum & 7 as libc::c_int;
    off = ((1 as libc::c_uint) << off) as libc::c_int;
    if *map.offset(i as isize) as libc::c_int & off == 0 as libc::c_int {
        return;
    }
    let ref mut fresh0 = *map.offset(i as isize);
    *fresh0 = (*fresh0 as libc::c_int & !off) as u_char;
    if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).first_free
        > &mut *map.offset(i as isize) as *mut u_char as *mut libc::c_void
    {
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .first_free = &mut *map.offset(i as isize) as *mut u_char
            as *mut libc::c_void;
    }
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .blkdeall = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .blkdeall)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.blkdeall;
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map_dirty_flag += 1;
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map_dirty_flag;
}
#[no_mangle]
pub unsafe extern "C" fn Used_block(mut blknum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut map: *mut u_char = 0 as *mut u_char;
    map = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map as *mut u_char;
    i = blknum >> 3 as libc::c_int;
    off = blknum & 7 as libc::c_int;
    off = ((1 as libc::c_uint) << off) as libc::c_int;
    if *map.offset(i as isize) as libc::c_int & off != 0 {
        return;
    }
    let ref mut fresh1 = *map.offset(i as isize);
    *fresh1 = (*fresh1 as libc::c_int | off) as u_char;
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .blkalloc = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
        .stats
        .blkalloc)
        .wrapping_add(1);
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.blkalloc;
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map_dirty_flag += 1;
    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map_dirty_flag;
}
#[no_mangle]
pub unsafe extern "C" fn Tidy_block() {
    let mut ptr: *mut gbd = 0 as *mut gbd;
    let mut btmp: *mut DB_Block = 0 as *mut DB_Block;
    ptr = blk[level as usize];
    Get_GBD();
    memset(
        (*blk[level as usize]).mem as *mut libc::c_void,
        0 as libc::c_int,
        (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).block_size
            as libc::c_ulong,
    );
    (*(*blk[level as usize]).mem).0.type_0 = (*(*ptr).mem).0.type_0;
    if level == 0 {
        (*(*blk[level as usize]).mem)
            .0
            .type_0 = ((*(*blk[level as usize]).mem).0.type_0 as libc::c_int
            | 64 as libc::c_int) as u_char;
    }
    (*(*blk[level as usize]).mem).0.right_ptr = (*(*ptr).mem).0.right_ptr;
    let mut var_i: u_int = 0 as libc::c_int as u_int;
    while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
        (*(*blk[level as usize]).mem)
            .0
            .global
            .var_qu[var_i as usize] = (*(*ptr).mem).0.global.var_qu[var_i as usize];
        var_i = var_i.wrapping_add(1);
        var_i;
    }
    (*(*blk[level as usize]).mem)
        .0
        .last_idx = (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int)
        .wrapping_sub(1 as libc::c_int as u_int) as u_short;
    (*(*blk[level as usize]).mem)
        .0
        .last_free = ((*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
        .block_size >> 2 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as u_int) as u_short;
    Copy_data(
        ptr,
        (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
            / 2 as libc::c_int as u_int) as libc::c_int,
    );
    btmp = (*blk[level as usize]).mem;
    (*blk[level as usize]).mem = (*ptr).mem;
    (*ptr).mem = btmp;
    Free_GBD(blk[level as usize]);
    blk[level as usize] = ptr;
    idx = (*blk[level as usize]).mem as *mut u_short;
    iidx = (*blk[level as usize]).mem as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Copy_data(mut fptr: *mut gbd, mut fidx: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut sfidx: *mut u_short = 0 as *mut u_short;
    let mut fiidx: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fk: [u_char; 260] = [0; 260];
    let mut isdata: libc::c_int = 0;
    let mut c: *mut cstring = 0 as *mut cstring;
    let mut ccc: u_char = 0;
    let mut ucc: u_char = 0;
    let mut cs: libc::c_int = 0;
    isdata = ((*(*blk[level as usize]).mem).0.type_0 as libc::c_int > 64 as libc::c_int
        && level != 0) as libc::c_int;
    sfidx = (*fptr).mem as *mut u_short;
    fiidx = (*fptr).mem as *mut libc::c_int;
    keybuf[0 as libc::c_int as usize] = 0 as libc::c_int as u_char;
    i = (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int) as libc::c_int;
    while i <= (*(*blk[level as usize]).mem).0.last_idx as libc::c_int {
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
        i += 1;
        i;
    }
    let mut current_block_55: u64;
    i = (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
        / 2 as libc::c_int as u_int) as libc::c_int;
    while i <= (*(*fptr).mem).0.last_idx as libc::c_int {
        c = &mut *fiidx.offset(*sfidx.offset(i as isize) as isize) as *mut libc::c_int
            as *mut cstring;
        memcpy(
            &mut *fk
                .as_mut_ptr()
                .offset(
                    (*((*c).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 1 as libc::c_int) as isize,
                ) as *mut u_char as *mut libc::c_void,
            &mut *((*c).buf).as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            (*c).buf[1 as libc::c_int as usize] as libc::c_ulong,
        );
        fk[0 as libc::c_int
            as usize] = ((*c).buf[0 as libc::c_int as usize] as libc::c_int
            + (*c).buf[1 as libc::c_int as usize] as libc::c_int) as u_char;
        if !(i < fidx) {
            c = &mut *((*c).buf)
                .as_mut_ptr()
                .offset(
                    (*((*c).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                        as libc::c_int + 2 as libc::c_int) as isize,
                ) as *mut u_char as *mut cstring;
            if isdata != 0 {
                if (*c).len as libc::c_int == 65534 as libc::c_int + 1 as libc::c_int {
                    current_block_55 = 7746791466490516765;
                } else {
                    current_block_55 = 15089075282327824602;
                }
            } else {
                if c as libc::c_long & 3 as libc::c_int as libc::c_long != 0 {
                    c = &mut *((*c).buf)
                        .as_mut_ptr()
                        .offset(
                            (2 as libc::c_int as libc::c_long
                                - (c as libc::c_long & 3 as libc::c_int as libc::c_long))
                                as isize,
                        ) as *mut u_char as *mut cstring;
                }
                if *(c as *mut libc::c_int) == 0 as libc::c_int {
                    current_block_55 = 7746791466490516765;
                } else {
                    current_block_55 = 15089075282327824602;
                }
            }
            match current_block_55 {
                7746791466490516765 => {}
                _ => {
                    ccc = 0 as libc::c_int as u_char;
                    if fk[0 as libc::c_int as usize] as libc::c_int != 0
                        && keybuf[0 as libc::c_int as usize] as libc::c_int != 0
                    {
                        while fk[(ccc as libc::c_int + 1 as libc::c_int) as usize]
                            as libc::c_int
                            == keybuf[(ccc as libc::c_int + 1 as libc::c_int) as usize]
                                as libc::c_int
                        {
                            if ccc as libc::c_int
                                == fk[0 as libc::c_int as usize] as libc::c_int
                                || ccc as libc::c_int
                                    == keybuf[0 as libc::c_int as usize] as libc::c_int
                            {
                                break;
                            }
                            ccc = ccc.wrapping_add(1);
                            ccc;
                        }
                    }
                    ucc = (fk[0 as libc::c_int as usize] as libc::c_int
                        - ccc as libc::c_int) as u_char;
                    cs = 4 as libc::c_int + ucc as libc::c_int
                        + (if isdata != 0 {
                            (*c).len as libc::c_int + 2 as libc::c_int
                        } else {
                            4 as libc::c_int
                        });
                    if level == 0 {
                        cs += 4 as libc::c_int;
                    }
                    if cs & 3 as libc::c_int != 0 {
                        cs += 4 as libc::c_int - (cs & 3 as libc::c_int);
                    }
                    if cs
                        >= ((*(*blk[level as usize]).mem).0.last_free as libc::c_int
                            * 2 as libc::c_int + 1 as libc::c_int
                            - (*(*blk[level as usize]).mem).0.last_idx as libc::c_int)
                            * 2 as libc::c_int
                    {
                        if fidx == -(1 as libc::c_int) {
                            return;
                        }
                        panic(
                            b"Copy_data: about to overflow block\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    (*(*blk[level as usize]).mem)
                        .0
                        .last_free = ((*(*blk[level as usize]).mem).0.last_free
                        as libc::c_int - cs / 4 as libc::c_int) as u_short;
                    (*(*blk[level as usize]).mem)
                        .0
                        .last_idx = ((*(*blk[level as usize]).mem).0.last_idx)
                        .wrapping_add(1);
                    *idx
                        .offset(
                            (*(*blk[level as usize]).mem).0.last_idx as isize,
                        ) = ((*(*blk[level as usize]).mem).0.last_free as libc::c_int
                        + 1 as libc::c_int) as u_short;
                    chunk = &mut *iidx
                        .offset(
                            ((*(**blk.as_mut_ptr().offset(level as isize)).mem)
                                .0
                                .last_free as libc::c_int + 1 as libc::c_int) as isize,
                        ) as *mut libc::c_int as *mut cstring;
                    (*chunk).len = cs as u_short;
                    (*chunk).buf[0 as libc::c_int as usize] = ccc;
                    (*chunk).buf[1 as libc::c_int as usize] = ucc;
                    memcpy(
                        &mut *((*chunk).buf)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize) as *mut u_char
                            as *mut libc::c_void,
                        &mut *fk
                            .as_mut_ptr()
                            .offset((ccc as libc::c_int + 1 as libc::c_int) as isize)
                            as *mut u_char as *const libc::c_void,
                        ucc as libc::c_ulong,
                    );
                    record = &mut *((*chunk).buf)
                        .as_mut_ptr()
                        .offset((ucc as libc::c_int + 2 as libc::c_int) as isize)
                        as *mut u_char as *mut cstring;
                    if isdata != 0 {
                        (*record).len = (*c).len;
                        memcpy(
                            ((*record).buf).as_mut_ptr() as *mut libc::c_void,
                            ((*c).buf).as_mut_ptr() as *const libc::c_void,
                            (*c).len as libc::c_ulong,
                        );
                        if fidx == -(1 as libc::c_int) {
                            (*c)
                                .len = (65534 as libc::c_int + 1 as libc::c_int) as u_short;
                        }
                    } else {
                        Align_record();
                        *(record as *mut u_int) = *(c as *mut u_int);
                        if fidx == -(1 as libc::c_int) {
                            *(c as *mut libc::c_int) = 0 as libc::c_int;
                        }
                        if level == 0 {
                            *(record as *mut u_int)
                                .offset(
                                    1 as libc::c_int as isize,
                                ) = *(c as *mut u_int).offset(1 as libc::c_int as isize)
                                & 3 as libc::c_int as u_int;
                        }
                    }
                    memcpy(
                        keybuf.as_mut_ptr() as *mut libc::c_void,
                        fk.as_mut_ptr() as *const libc::c_void,
                        (fk[0 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong,
                    );
                }
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Align_record() {
    if record as libc::c_long & 3 as libc::c_int as libc::c_long != 0 {
        record = &mut *((*record).buf)
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_long
                    - (record as libc::c_long & 3 as libc::c_int as libc::c_long))
                    as isize,
            ) as *mut u_char as *mut cstring;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Compress1() -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut curlevel: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    Get_GBDs(12 as libc::c_int * 2 as libc::c_int);
    curlevel = level;
    writing = 1 as libc::c_int;
    s = Get_data(curlevel);
    if s == -(7 as libc::c_int) && db_var.slen == 0 {
        s = 0 as libc::c_int;
    }
    if s == -(7 as libc::c_int) {
        while level >= 0 as libc::c_int {
            if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                (*blk[level as usize]).dirty = 0 as *mut GBD;
            }
            level -= 1;
            level;
        }
        return 0 as libc::c_int as libc::c_short;
    }
    if s < 0 as libc::c_int {
        return s as libc::c_short;
    }
    if (*(*blk[level as usize]).mem).0.right_ptr == 0 {
        if level == 2 as libc::c_int && db_var.slen == 0 {
            let mut gtmp: [u_char; 36] = [0; 36];
            level = 0 as libc::c_int;
            gtmp[1 as libc::c_int as usize] = 128 as libc::c_int as u_char;
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                if db_var.name.var_cu[i as usize] as libc::c_int == '\0' as i32 {
                    break;
                }
                gtmp[(i + 2 as libc::c_int) as usize] = db_var.name.var_cu[i as usize];
                i += 1;
                i;
            }
            i += 2 as libc::c_int;
            gtmp[i as usize] = '\0' as i32 as u_char;
            gtmp[0 as libc::c_int as usize] = i as u_char;
            s = Locate(gtmp.as_mut_ptr()) as libc::c_int;
            if s < 0 as libc::c_int {
                return s as libc::c_short;
            }
            Align_record();
            *(record as *mut u_int) = (*blk[2 as libc::c_int as usize]).block;
            if (*blk[level as usize]).dirty < 5 as libc::c_int as *mut gbd {
                (*blk[level as usize]).dirty = blk[level as usize];
                Queit();
            }
            (*(*blk[1 as libc::c_int as usize]).mem)
                .0
                .type_0 = 65 as libc::c_int as u_char;
            (*blk[1 as libc::c_int as usize])
                .last_accessed = current_time(1 as libc::c_int as libc::c_short);
            Garbit((*blk[1 as libc::c_int as usize]).block);
            memset(
                &mut (*partab.jobtab).last_ref as *mut mvar as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<mvar>() as libc::c_ulong,
            );
            return 0 as libc::c_int as libc::c_short;
        }
        while level >= 0 as libc::c_int {
            if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                (*blk[level as usize]).dirty = 0 as *mut GBD;
            }
            level -= 1;
            level;
        }
        return 0 as libc::c_int as libc::c_short;
    }
    blk[(level + 1 as libc::c_int) as usize] = blk[level as usize];
    s = Get_block((*(*blk[level as usize]).mem).0.right_ptr) as libc::c_int;
    if s < 0 as libc::c_int {
        while level >= 0 as libc::c_int {
            if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                (*blk[level as usize]).dirty = 0 as *mut GBD;
            }
            level -= 1;
            level;
        }
        return s as libc::c_short;
    }
    i = ((*(*blk[(level + 1 as libc::c_int) as usize]).mem).0.last_free as libc::c_int
        * 2 as libc::c_int + 1 as libc::c_int
        - (*(*blk[(level + 1 as libc::c_int) as usize]).mem).0.last_idx as libc::c_int)
        * 2 as libc::c_int
        + ((*(*blk[level as usize]).mem).0.last_free as libc::c_int * 2 as libc::c_int
            + 1 as libc::c_int - (*(*blk[level as usize]).mem).0.last_idx as libc::c_int)
            * 2 as libc::c_int;
    if i < 1024 as libc::c_int {
        level += 1;
        level;
        while level >= 0 as libc::c_int {
            if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                (*blk[level as usize]).dirty = 0 as *mut GBD;
            }
            level -= 1;
            level;
        }
        return s as libc::c_short;
    }
    Un_key();
    level += 1;
    level;
    Tidy_block();
    Copy_data(blk[(level - 1 as libc::c_int) as usize], -(1 as libc::c_int));
    if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
        (*blk[level as usize]).dirty = 2 as libc::c_int as *mut gbd;
    }
    level -= 1;
    level;
    Tidy_block();
    if ((*(*blk[level as usize]).mem).0.last_idx as u_int)
        < ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
            / 2 as libc::c_int as u_int
    {
        (*(*blk[level as usize]).mem).0.type_0 = 65 as libc::c_int as u_char;
        (*blk[level as usize])
            .last_accessed = current_time(1 as libc::c_int as libc::c_short);
        (*(*blk[(level + 1 as libc::c_int) as usize]).mem)
            .0
            .right_ptr = (*(*blk[level as usize]).mem).0.right_ptr;
        Garbit((*blk[level as usize]).block);
        blk[level as usize] = 0 as *mut gbd;
        if (*(*blk[(level + 1 as libc::c_int) as usize]).mem).0.right_ptr != 0 {
            s = Get_block((*(*blk[(level + 1 as libc::c_int) as usize]).mem).0.right_ptr)
                as libc::c_int;
        }
    } else if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
        (*blk[level as usize]).dirty = 2 as libc::c_int as *mut gbd;
        s = Add_rekey((*blk[level as usize]).block, level) as libc::c_int;
    }
    if !(blk[level as usize]).is_null() {
        if (*blk[level as usize]).dirty == 2 as libc::c_int as *mut gbd {
            chunk = &mut *iidx
                .offset(
                    *idx
                        .offset(
                            (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                                / 2 as libc::c_int as u_int) as isize,
                        ) as isize,
                ) as *mut libc::c_int as *mut cstring;
            memcpy(
                &mut (*partab.jobtab).last_ref.slen as *mut u_char as *mut libc::c_void,
                &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                ((*chunk).buf[1 as libc::c_int as usize] as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            );
        }
    } else {
        memset(
            &mut (*partab.jobtab).last_ref as *mut mvar as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mvar>() as libc::c_ulong,
        );
    }
    level += 2 as libc::c_int;
    blk[level as usize] = 0 as *mut gbd;
    i = level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if !(blk[i as usize]).is_null() {
            if (*blk[i as usize]).dirty == 2 as libc::c_int as *mut gbd {
                if (blk[level as usize]).is_null() {
                    (*blk[i as usize]).dirty = blk[i as usize];
                } else {
                    (*blk[i as usize]).dirty = blk[level as usize];
                }
                blk[level as usize] = blk[i as usize];
            } else if (*blk[i as usize]).dirty == 1 as libc::c_int as *mut gbd {
                (*blk[i as usize]).dirty = 0 as *mut GBD;
            }
        }
        i -= 1;
        i;
    }
    if !(blk[level as usize]).is_null() {
        Queit();
    }
    return Re_key();
}
#[no_mangle]
pub unsafe extern "C" fn ClearJournal(mut vol: libc::c_int) {
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
    let mut i: libc::c_int = 0;
    let mut tmp: C2RustUnnamed = C2RustUnnamed { magic: 0, free: 0 };
    jfd = open(
        ((*(*(*systab).vol[vol as usize]).vollab).journal_file).as_mut_ptr(),
        0x200 as libc::c_int | 0x400 as libc::c_int | 0x1 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int | 0o40 as libc::c_int
            | 0o20 as libc::c_int,
    );
    if jfd > 0 as libc::c_int {
        tmp
            .magic = (4155766917 as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
        tmp.free = 24 as libc::c_int as off_t;
        i = write(
            jfd,
            &mut tmp as *mut C2RustUnnamed as *mut u_char as *const libc::c_void,
            12 as libc::c_int as size_t,
        ) as libc::c_int;
        if i < 0 as libc::c_int {
            fprintf(
                __stderrp,
                b"errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
                *__error(),
                strerror(*__error()),
            );
        }
        jj.size = 12 as libc::c_int as u_short;
        jj.time = current_time(1 as libc::c_int as libc::c_short) as u_int64;
        jj.action = 0 as libc::c_int as u_char;
        jj.uci = 0 as libc::c_int as u_char;
        i = write(jfd, &mut jj as *mut jrnrec as *const libc::c_void, jj.size as size_t)
            as libc::c_int;
        if i < 0 as libc::c_int {
            fprintf(
                __stderrp,
                b"errno = %d - %s\n\0" as *const u8 as *const libc::c_char,
                *__error(),
                strerror(*__error()),
            );
        }
        close(jfd);
        (*(*systab).vol[vol as usize]).jrn_next = tmp.free;
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoJournal(mut jj: *mut jrnrec, mut data: *mut cstring) {
    let mut current_block: u64;
    let mut jptr: off_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    jptr = lseek(
        partab.jnl_fds[(volnum - 1 as libc::c_int) as usize],
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).jrn_next,
        0 as libc::c_int,
    );
    if !(jptr != (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).jrn_next) {
        (*jj)
            .size = (13 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<var_u>() as libc::c_ulong)
            .wrapping_add((*jj).slen as libc::c_ulong) as u_short;
        if (*jj).action as libc::c_int != 4 as libc::c_int
            && (*jj).action as libc::c_int != 5 as libc::c_int
        {
            (*jj).size = 12 as libc::c_int as u_short;
        }
        i = (*jj).size as libc::c_int;
        if (*jj).action as libc::c_int == 4 as libc::c_int {
            (*jj)
                .size = ((*jj).size as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                        .wrapping_add((*data).len as libc::c_ulong),
                ) as u_short as u_short;
        }
        (*jj).time = current_time(1 as libc::c_int as libc::c_short) as u_int64;
        j = write(
            partab.jnl_fds[(volnum - 1 as libc::c_int) as usize],
            jj as *const libc::c_void,
            i as size_t,
        ) as libc::c_int;
        if !(j != i) {
            if (*jj).action as libc::c_int == 4 as libc::c_int {
                i = (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                    .wrapping_add((*data).len as libc::c_ulong) as libc::c_int;
                j = write(
                    partab.jnl_fds[(volnum - 1 as libc::c_int) as usize],
                    data as *const libc::c_void,
                    i as size_t,
                ) as libc::c_int;
                if j != i {
                    current_block = 18265478822016080852;
                } else {
                    current_block = 2979737022853876585;
                }
            } else {
                current_block = 2979737022853876585;
            }
            match current_block {
                18265478822016080852 => {}
                _ => {
                    if (*jj).size as libc::c_int & 3 as libc::c_int != 0 {
                        (*jj)
                            .size = ((*jj).size as libc::c_int
                            + (4 as libc::c_int
                                - ((*jj).size as libc::c_int & 3 as libc::c_int)))
                            as u_short;
                    }
                    (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).jrn_next
                        += (*jj).size as off_t;
                    jptr = lseek(
                        partab.jnl_fds[(volnum - 1 as libc::c_int) as usize],
                        4 as libc::c_int as off_t,
                        0 as libc::c_int,
                    );
                    if !(jptr != 4 as libc::c_int as off_t) {
                        j = write(
                            partab.jnl_fds[(volnum - 1 as libc::c_int) as usize],
                            &mut (**((*systab).vol)
                                .as_mut_ptr()
                                .offset((volnum - 1 as libc::c_int) as isize))
                                .jrn_next as *mut off_t as *const libc::c_void,
                            ::core::mem::size_of::<off_t>() as libc::c_ulong,
                        ) as libc::c_int;
                        if !(j < 0 as libc::c_int) {
                            return;
                        }
                    }
                }
            }
        }
    }
    (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
        .journal_available = 0 as libc::c_int as u_char;
    close(partab.jnl_fds[(volnum - 1 as libc::c_int) as usize]);
}
