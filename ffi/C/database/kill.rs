#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type RBD;
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
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    static mut curr_lock: libc::c_int;
    static mut db_var: mvar;
    static mut volnum: libc::c_int;
    static mut blk: [*mut gbd; 12];
    static mut level: libc::c_int;
    static mut rekey_blk: [u_int; 36];
    static mut rekey_lvl: [libc::c_int; 36];
    static mut Index: u_int;
    static mut chunk: *mut cstring;
    static mut record: *mut cstring;
    static mut keybuf: [u_char; 260];
    static mut idx: *mut u_short;
    static mut iidx: *mut libc::c_int;
    static mut writing: libc::c_int;
    fn Get_GBDs(greqd: libc::c_int);
    fn Get_data(dir: libc::c_int) -> libc::c_int;
    fn Locate(key: *mut u_char) -> libc::c_short;
    fn Add_rekey(block: u_int, level_0: libc::c_int) -> libc::c_short;
    fn Re_key() -> libc::c_short;
    fn Align_record();
    fn Copy_data(fptr: *mut gbd, fidx: libc::c_int);
    fn DoJournal(jj: *mut jrnrec, data: *mut cstring);
    fn Garbit(blknum: u_int);
    fn Insert(key: *mut u_char, data: *mut cstring) -> libc::c_short;
    fn Queit();
    fn Tidy_block();
    fn current_time(local: libc::c_short) -> time_t;
    fn panic(msg: *mut libc::c_char);
    fn SemOp(sem_num: libc::c_int, numb: libc::c_int) -> libc::c_short;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
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
pub unsafe extern "C" fn Kill_data() -> libc::c_short {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: u_int = 0;
    let mut rblk: [*mut gbd; 12] = [0 as *mut gbd; 12];
    let mut leftblk: *mut gbd = 0 as *mut gbd;
    let mut ptr: *mut gbd = 0 as *mut gbd;
    let mut rlevel: libc::c_int = 0;
    let mut blknum: u_int = 0;
    let mut tmp: [u_char; 36] = [0; 36];
    let mut top: libc::c_int = 0;
    let mut p: *mut u_char = 0 as *mut u_char;
    let mut c: *mut cstring = 0 as *mut cstring;
    let mut ui: *mut u_int = 0 as *mut u_int;
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
    SemOp(2 as libc::c_int, -curr_lock);
    *((*systab).last_blk_used)
        .as_mut_ptr()
        .offset(
            ((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                + ((*systab).maxjob * (volnum - 1 as libc::c_int) as u_int)
                    as libc::c_long) as isize,
        ) = 0 as libc::c_int as u_int;
    '_start: loop {
        Get_GBDs(12 as libc::c_int * 2 as libc::c_int);
        j = 0 as libc::c_int as u_int;
        i = 0 as libc::c_int;
        while i < 8192 as libc::c_int {
            if (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).garbQ[i as usize]
                == 0 as libc::c_int as u_int
            {
                let fresh0 = j;
                j = j.wrapping_add(1);
                if fresh0 >= (8192 as libc::c_int / 2 as libc::c_int) as u_int {
                    break '_start;
                }
            }
            i += 1;
            i;
        }
        SemOp(2 as libc::c_int, -curr_lock);
        sleep(1 as libc::c_int as libc::c_uint);
    }
    writing = 1 as libc::c_int;
    level = 0 as libc::c_int;
    s = Get_data(0 as libc::c_int);
    if s < 0 as libc::c_int && s != -(7 as libc::c_int) {
        return s as libc::c_short;
    }
    if (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).journal_available
        as libc::c_int != 0
        && (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
            .journal_requested as libc::c_int != 0
        && (*partab.jobtab).last_block_flags & 1 as libc::c_int as u_int != 0
    {
        let mut jj: jrnrec = JRNREC {
            size: 0,
            action: 0,
            uci: 0,
            time: 0,
            name: VAR_U { var_q: 0 },
            slen: 0,
            key: [0; 256],
        };
        jj.action = 5 as libc::c_int as u_char;
        jj.uci = db_var.uci;
        let mut var_i: u_int = 0 as libc::c_int as u_int;
        while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            jj.name.var_qu[var_i as usize] = db_var.name.var_qu[var_i as usize];
            var_i = var_i.wrapping_add(1);
            var_i;
        }
        jj.slen = db_var.slen;
        memcpy(
            (jj.key).as_mut_ptr() as *mut libc::c_void,
            (db_var.key).as_mut_ptr() as *const libc::c_void,
            jj.slen as libc::c_ulong,
        );
        DoJournal(&mut jj, 0 as *mut cstring);
    }
    if db_var.slen as libc::c_int == 0 as libc::c_int {
        while level != 0 {
            if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                (*blk[level as usize]).dirty = 0 as *mut GBD;
            }
            level -= 1;
            level;
        }
        tmp[1 as libc::c_int as usize] = 128 as libc::c_int as u_char;
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if db_var.name.var_cu[i as usize] as libc::c_int == '\0' as i32 {
                break;
            }
            tmp[(i + 2 as libc::c_int) as usize] = db_var.name.var_cu[i as usize];
            i += 1;
            i;
        }
        i += 2 as libc::c_int;
        tmp[i as usize] = '\0' as i32 as u_char;
        tmp[0 as libc::c_int as usize] = i as u_char;
        s = Locate(tmp.as_mut_ptr()) as libc::c_int;
        if s == -(7 as libc::c_int) {
            if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                (*blk[level as usize]).dirty = 0 as *mut GBD;
            }
            return 0 as libc::c_int as libc::c_short;
        }
        Align_record();
        blknum = *(record as *mut u_int);
        *(record as *mut u_int) = 0 as libc::c_int as u_int;
        Tidy_block();
        if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
            (*blk[level as usize]).dirty = blk[level as usize];
            Queit();
        }
        Garbit(blknum);
        memset(
            &mut *((*systab).last_blk_used)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut u_int as *mut libc::c_void,
            0 as libc::c_int,
            ((*systab).maxjob as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong),
        );
        level -= 1;
        level;
        return 0 as libc::c_int as libc::c_short;
    }
    *((*systab).last_blk_used)
        .as_mut_ptr()
        .offset(
            ((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                + ((*systab).maxjob * (volnum - 1 as libc::c_int) as u_int)
                    as libc::c_long) as isize,
        ) = 0 as libc::c_int as u_int;
    while level >= 0 as libc::c_int {
        if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
            (*blk[level as usize]).dirty = 0 as *mut GBD;
        }
        level -= 1;
        level;
    }
    let fresh1 = db_var.slen;
    db_var.slen = (db_var.slen).wrapping_add(1);
    db_var.key[fresh1 as usize] = 255 as libc::c_int as u_char;
    s = Get_data(0 as libc::c_int);
    if s != -(7 as libc::c_int) {
        return -(61 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    db_var.slen = (db_var.slen).wrapping_sub(1);
    db_var.slen;
    rlevel = level;
    i = 0 as libc::c_int;
    while i <= level {
        rblk[i as usize] = blk[i as usize];
        i += 1;
        i;
    }
    level = 0 as libc::c_int;
    *((*systab).last_blk_used)
        .as_mut_ptr()
        .offset(
            ((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                + ((*systab).maxjob * (volnum - 1 as libc::c_int) as u_int)
                    as libc::c_long) as isize,
        ) = 0 as libc::c_int as u_int;
    s = Get_data(-(1 as libc::c_int));
    if s < 0 as libc::c_int && s != -(7 as libc::c_int) {
        return s as libc::c_short;
    }
    if rlevel != level {
        panic(
            b"Kill_data: left level not equal to right level\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    level = 0 as libc::c_int;
    while level < rlevel {
        if blk[(level + 1 as libc::c_int) as usize]
            != rblk[(level + 1 as libc::c_int) as usize]
        {
            break;
        }
        level += 1;
        level;
    }
    if level == rlevel {
        i = Index as libc::c_int;
        while i <= (*(*blk[level as usize]).mem).0.last_idx as libc::c_int {
            chunk = &mut *iidx.offset(*idx.offset(i as isize) as isize)
                as *mut libc::c_int as *mut cstring;
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
            if (keybuf[0 as libc::c_int as usize] as libc::c_int)
                < db_var.slen as libc::c_int
                || memcmp(
                    &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    &mut db_var.key as *mut [u_char; 256] as *const libc::c_void,
                    db_var.slen as libc::c_ulong,
                ) != 0
            {
                break;
            } else {
                record = &mut *((*chunk).buf)
                    .as_mut_ptr()
                    .offset(
                        (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                            as libc::c_int + 2 as libc::c_int) as isize,
                    ) as *mut u_char as *mut cstring;
                (*record).len = (65534 as libc::c_int + 1 as libc::c_int) as u_short;
                i += 1;
                i;
            }
        }
        Tidy_block();
        if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
            (*blk[level as usize]).dirty = blk[level as usize];
            Queit();
        }
        level = 0 as libc::c_int;
        while level < rlevel {
            if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                (*blk[level as usize]).dirty = 0 as *mut GBD;
            }
            level += 1;
            level;
        }
        return 0 as libc::c_int as libc::c_short;
    }
    top = level;
    i = 0 as libc::c_int;
    while i < top {
        if (*blk[i as usize]).dirty == 1 as libc::c_int as *mut gbd {
            (*blk[i as usize]).dirty = 0 as *mut GBD;
        }
        i += 1;
        i;
    }
    level = top;
    while level <= rlevel {
        s = Locate(&mut db_var.slen) as libc::c_int;
        if s < 0 as libc::c_int && s != -(7 as libc::c_int) {
            return s as libc::c_short;
        }
        i = Index as libc::c_int;
        while i <= (*(*blk[level as usize]).mem).0.last_idx as libc::c_int {
            chunk = &mut *iidx.offset(*idx.offset(i as isize) as isize)
                as *mut libc::c_int as *mut cstring;
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
            if (keybuf[0 as libc::c_int as usize] as libc::c_int)
                < db_var.slen as libc::c_int
                || memcmp(
                    &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    &mut db_var.key as *mut [u_char; 256] as *const libc::c_void,
                    db_var.slen as libc::c_ulong,
                ) != 0
            {
                break;
            } else {
                record = &mut *((*chunk).buf)
                    .as_mut_ptr()
                    .offset(
                        (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                            as libc::c_int + 2 as libc::c_int) as isize,
                    ) as *mut u_char as *mut cstring;
                if level != rlevel {
                    Align_record();
                    j = *(record as *mut u_int);
                    if j != (*rblk[(level + 1 as libc::c_int) as usize]).block {
                        Garbit(j);
                    }
                    *(record as *mut u_int) = 0 as libc::c_int as u_int;
                } else {
                    (*record).len = (65534 as libc::c_int + 1 as libc::c_int) as u_short;
                }
                (*(*blk[level as usize]).mem)
                    .0
                    .flags = ((*(*blk[level as usize]).mem).0.flags as libc::c_int
                    | 1 as libc::c_int) as u_char;
                i += 1;
                i;
            }
        }
        if (*(*blk[level as usize]).mem).0.flags as libc::c_int & 1 as libc::c_int != 0 {
            Tidy_block();
        }
        if level > top {
            (*(*blk[level as usize]).mem).0.right_ptr = (*rblk[level as usize]).block;
        }
        level += 1;
        level;
    }
    level = rlevel;
    while level > top {
        leftblk = blk[level as usize];
        blk[level as usize] = rblk[level as usize];
        idx = (*blk[level as usize]).mem as *mut u_short;
        iidx = (*blk[level as usize]).mem as *mut libc::c_int;
        Index = ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
            / 2 as libc::c_int as u_int;
        while Index <= (*(*blk[level as usize]).mem).0.last_idx as u_int {
            chunk = &mut *iidx.offset(*idx.offset(Index as isize) as isize)
                as *mut libc::c_int as *mut cstring;
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
            if (keybuf[0 as libc::c_int as usize] as libc::c_int)
                < db_var.slen as libc::c_int
                || memcmp(
                    &mut *keybuf.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut u_char as *const libc::c_void,
                    &mut db_var.key as *mut [u_char; 256] as *const libc::c_void,
                    db_var.slen as libc::c_ulong,
                ) != 0
            {
                break;
            } else {
                record = &mut *((*chunk).buf)
                    .as_mut_ptr()
                    .offset(
                        (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                            as libc::c_int + 2 as libc::c_int) as isize,
                    ) as *mut u_char as *mut cstring;
                if level != rlevel {
                    Align_record();
                    j = *(record as *mut u_int);
                    if !(rblk[(level + 1 as libc::c_int) as usize]).is_null() {
                        if j != (*rblk[(level + 1 as libc::c_int) as usize]).block {
                            Garbit(j);
                        }
                    } else {
                        Garbit(j);
                    }
                    *(record as *mut u_int) = 0 as libc::c_int as u_int;
                } else {
                    (*record).len = (65534 as libc::c_int + 1 as libc::c_int) as u_short;
                }
                (*(*blk[level as usize]).mem)
                    .0
                    .flags = ((*(*blk[level as usize]).mem).0.flags as libc::c_int
                    | 1 as libc::c_int) as u_char;
                Index = Index.wrapping_add(1);
                Index;
            }
        }
        if (*(*blk[level as usize]).mem).0.flags as libc::c_int & 1 as libc::c_int != 0 {
            Tidy_block();
        }
        if level < rlevel && !(rblk[(level + 1 as libc::c_int) as usize]).is_null() {
            idx = (*rblk[(level + 1 as libc::c_int) as usize]).mem as *mut u_short;
            iidx = (*rblk[(level + 1 as libc::c_int) as usize]).mem as *mut libc::c_int;
            chunk = &mut *iidx
                .offset(
                    *idx
                        .offset(
                            (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                                / 2 as libc::c_int as u_int) as isize,
                        ) as isize,
                ) as *mut libc::c_int as *mut cstring;
            p = &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut u_char;
            s = Locate(p) as libc::c_int;
            if s == -(7 as libc::c_int) {
                c = tmp.as_mut_ptr() as *mut cstring;
                (*c).len = 4 as libc::c_int as u_short;
                ui = ((*c).buf).as_mut_ptr() as *mut u_int;
                *ui = (*rblk[(level + 1 as libc::c_int) as usize]).block;
                s = Insert(p, c) as libc::c_int;
                if s == -(62 as libc::c_int + 200 as libc::c_int) {
                    s = Add_rekey(
                        (*rblk[(level + 1 as libc::c_int) as usize]).block,
                        level + 1 as libc::c_int,
                    ) as libc::c_int;
                } else if s < 0 as libc::c_int {
                    return s as libc::c_short
                }
            }
        }
        if ((((*(*leftblk).mem).0.last_free as libc::c_int * 2 as libc::c_int
            + 1 as libc::c_int - (*(*leftblk).mem).0.last_idx as libc::c_int)
            * 2 as libc::c_int) as u_long)
            .wrapping_add(
                (((*(*blk[level as usize]).mem).0.last_free as libc::c_int
                    * 2 as libc::c_int + 1 as libc::c_int
                    - (*(*blk[level as usize]).mem).0.last_idx as libc::c_int)
                    * 2 as libc::c_int) as u_long,
            )
            > ((*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
                .block_size as libc::c_ulong)
                .wrapping_sub(::core::mem::size_of::<DB_Block>() as libc::c_ulong)
            || ((*(*blk[level as usize]).mem).0.last_idx as u_int)
                < ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                    / 2 as libc::c_int as u_int
        {
            ptr = blk[level as usize];
            blk[level as usize] = leftblk;
            idx = (*blk[level as usize]).mem as *mut u_short;
            iidx = (*blk[level as usize]).mem as *mut libc::c_int;
            if (*(*ptr).mem).0.last_idx as u_int
                > (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                    / 2 as libc::c_int as u_int)
                    .wrapping_sub(1 as libc::c_int as u_int)
            {
                Copy_data(
                    ptr,
                    (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                        / 2 as libc::c_int as u_int) as libc::c_int,
                );
            }
            (*(*blk[level as usize]).mem).0.right_ptr = (*(*ptr).mem).0.right_ptr;
            (*(*ptr).mem).0.type_0 = 65 as libc::c_int as u_char;
            (*ptr).last_accessed = current_time(1 as libc::c_int as libc::c_short);
            Garbit((*ptr).block);
            rblk[level as usize] = 0 as *mut gbd;
        }
        blk[level as usize] = leftblk;
        level -= 1;
        level;
    }
    if !(rblk[(top + 1 as libc::c_int) as usize]).is_null() {
        idx = (*rblk[(top + 1 as libc::c_int) as usize]).mem as *mut u_short;
        iidx = (*rblk[(top + 1 as libc::c_int) as usize]).mem as *mut libc::c_int;
        chunk = &mut *iidx
            .offset(
                *idx
                    .offset(
                        (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                            / 2 as libc::c_int as u_int) as isize,
                    ) as isize,
            ) as *mut libc::c_int as *mut cstring;
        p = &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut u_char;
        s = Locate(p) as libc::c_int;
        if s == -(7 as libc::c_int) {
            c = tmp.as_mut_ptr() as *mut cstring;
            (*c).len = 4 as libc::c_int as u_short;
            ui = ((*c).buf).as_mut_ptr() as *mut u_int;
            *ui = (*rblk[(level + 1 as libc::c_int) as usize]).block;
            s = Insert(p, c) as libc::c_int;
            if s == -(62 as libc::c_int + 200 as libc::c_int) {
                s = Add_rekey(
                    (*rblk[(level + 1 as libc::c_int) as usize]).block,
                    level + 1 as libc::c_int,
                ) as libc::c_int;
            } else if s < 0 as libc::c_int {
                return s as libc::c_short
            }
        }
    }
    level = 12 as libc::c_int - 1 as libc::c_int;
    blk[level as usize] = 0 as *mut gbd;
    i = top;
    while i <= rlevel {
        if (*blk[i as usize]).dirty == 1 as libc::c_int as *mut gbd {
            if (blk[level as usize]).is_null() {
                (*blk[i as usize]).dirty = blk[i as usize];
            } else {
                (*blk[i as usize]).dirty = blk[level as usize];
            }
            blk[level as usize] = blk[i as usize];
        }
        i += 1;
        i;
    }
    i = top + 1 as libc::c_int;
    while i <= rlevel {
        if !(rblk[i as usize]).is_null() {
            if (*rblk[i as usize]).dirty == 1 as libc::c_int as *mut gbd {
                if (blk[level as usize]).is_null() {
                    (*rblk[i as usize]).dirty = rblk[i as usize];
                } else {
                    (*rblk[i as usize]).dirty = blk[level as usize];
                }
                blk[level as usize] = rblk[i as usize];
            }
        }
        i += 1;
        i;
    }
    if !(blk[level as usize]).is_null() {
        Queit();
    }
    memset(
        &mut *((*systab).last_blk_used).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_int as *mut libc::c_void,
        0 as libc::c_int,
        ((*systab).maxjob as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
    );
    return Re_key();
}
