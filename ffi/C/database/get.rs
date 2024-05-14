#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type RBD;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    static mut curr_lock: libc::c_int;
    static mut db_var: mvar;
    static mut volnum: libc::c_int;
    static mut blk: [*mut gbd; 12];
    static mut level: libc::c_int;
    static mut Index: u_int;
    static mut chunk: *mut cstring;
    static mut record: *mut cstring;
    static mut idx: *mut u_short;
    static mut iidx: *mut libc::c_int;
    static mut writing: libc::c_int;
    fn Get_block(blknum: u_int) -> libc::c_short;
    fn Locate(key: *mut u_char) -> libc::c_short;
    fn Align_record();
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
pub unsafe extern "C" fn Get_data(mut dir: libc::c_int) -> libc::c_int {
    let mut block: u_int = 0;
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut tmp: [u_char; 36] = [0; 36];
    let mut ptr: *mut gbd = 0 as *mut gbd;
    if curr_lock == 0 {
        s = SemOp(2 as libc::c_int, -(1 as libc::c_int)) as libc::c_int;
        if s < 0 as libc::c_int {
            return s;
        }
    }
    if ((*systab).vol[(volnum - 1 as libc::c_int) as usize]).is_null() {
        return -(26 as libc::c_int);
    }
    if memcmp(
        &mut *(db_var.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_char as *const libc::c_void,
        b"$GLOBAL\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int || dir != 0 as libc::c_int
        || (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab)
            .journal_available as libc::c_int != 0 && writing != 0
    {
        *((*systab).last_blk_used)
            .as_mut_ptr()
            .offset(
                ((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                    + ((*systab).maxjob * (volnum - 1 as libc::c_int) as u_int)
                        as libc::c_long) as isize,
            ) = 0 as libc::c_int as u_int;
    } else {
        block = *((*systab).last_blk_used)
            .as_mut_ptr()
            .offset(
                ((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                    + ((*systab).maxjob * (volnum - 1 as libc::c_int) as u_int)
                        as libc::c_long) as isize,
            );
        if block != 0
            && *((*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).map
                as *mut u_char)
                .offset((block >> 3 as libc::c_int) as isize) as libc::c_uint
                & (1 as libc::c_uint) << (block & 7 as libc::c_int as u_int) != 0
        {
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .stats
                .lasttry = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .stats
                .lasttry)
                .wrapping_add(1);
            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.lasttry;
            ptr = (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                .gbd_hash[(block & (1024 as libc::c_int - 1 as libc::c_int) as u_int)
                as usize];
            while !ptr.is_null() {
                if (*ptr).block == block {
                    if var_equal((*(*ptr).mem).0.global, db_var.name) == 0
                        || (*(*ptr).mem).0.type_0 as libc::c_int
                            != db_var.uci as libc::c_int + 64 as libc::c_int
                        || (*ptr).last_accessed == 0 as libc::c_int as time_t
                    {
                        break;
                    } else {
                        level = 3 as libc::c_int;
                        blk[level as usize] = ptr;
                        s = Locate(&mut db_var.slen) as libc::c_int;
                        if s >= 0 as libc::c_int
                            || {
                                s = -(7 as libc::c_int);
                                s != 0
                                    && Index
                                        <= (*(*blk[level as usize]).mem).0.last_idx as u_int
                                    && Index
                                        > ::core::mem::size_of::<DB_Block>() as libc::c_ulong
                                            as u_int / 2 as libc::c_int as u_int
                            }
                        {
                            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                                .stats
                                .lastok = ((*(*systab)
                                .vol[(volnum - 1 as libc::c_int) as usize])
                                .stats
                                .lastok)
                                .wrapping_add(1);
                            (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
                                .stats
                                .lastok;
                            (*blk[level as usize])
                                .last_accessed = current_time(
                                1 as libc::c_int as libc::c_short,
                            );
                            i = 0 as libc::c_int;
                            while i < level {
                                let fresh0 = i;
                                i = i + 1;
                                blk[fresh0 as usize] = 0 as *mut gbd;
                            }
                            if s == 0 {
                                s = (*record).len as libc::c_int;
                            }
                            if writing != 0 && ((*blk[level as usize]).dirty).is_null() {
                                (*blk[level as usize]).dirty = 1 as libc::c_int as *mut gbd;
                            }
                            if db_var.slen == 0 && s == 0
                                && (*partab.jobtab).last_block_flags
                                    & 2 as libc::c_int as u_int == 0 as libc::c_int as u_int
                            {
                                s = -(7 as libc::c_int);
                            }
                            return s;
                        }
                        blk[level as usize] = 0 as *mut gbd;
                        level = 0 as libc::c_int;
                        break;
                    }
                } else {
                    ptr = (*ptr).next;
                }
            }
        }
        *((*systab).last_blk_used)
            .as_mut_ptr()
            .offset(
                ((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                    + ((*systab).maxjob * (volnum - 1 as libc::c_int) as u_int)
                        as libc::c_long) as isize,
            ) = 0 as libc::c_int as u_int;
    }
    block = (*(*(*systab)
        .vol[(db_var.volset as libc::c_int - 1 as libc::c_int) as usize])
        .vollab)
        .uci[(db_var.uci as libc::c_int - 1 as libc::c_int) as usize]
        .global;
    if block == 0 {
        return -(26 as libc::c_int);
    }
    level = 0 as libc::c_int;
    s = Get_block(block) as libc::c_int;
    if s < 0 as libc::c_int {
        return s;
    }
    if memcmp(
        &mut *(db_var.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut u_char as *const libc::c_void,
        b"$GLOBAL\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        s = Locate(&mut db_var.slen) as libc::c_int;
        if s >= 0 as libc::c_int {
            Align_record();
        }
        return s;
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
    if s < 0 as libc::c_int {
        return s;
    }
    (*partab.jobtab).last_block_flags = 0 as libc::c_int as u_int;
    Align_record();
    block = *(record as *mut u_int);
    if block == 0 {
        return -(7 as libc::c_int);
    }
    (*partab.jobtab)
        .last_block_flags = *(record as *mut u_int).offset(1 as libc::c_int as isize);
    if (*partab.jobtab).last_block_flags > 3 as libc::c_int as u_int {
        (*partab.jobtab).last_block_flags &= 3 as libc::c_int as u_int;
        *(record as *mut u_int)
            .offset(1 as libc::c_int as isize) = (*partab.jobtab).last_block_flags;
    }
    level += 1;
    level;
    s = Get_block(block) as libc::c_int;
    if s < 0 as libc::c_int {
        return s;
    }
    while ((*(*blk[level as usize]).mem).0.type_0 as libc::c_int) < 65 as libc::c_int {
        if var_equal((*(*blk[level as usize]).mem).0.global, db_var.name) == 0 {
            return -(61 as libc::c_int + 200 as libc::c_int);
        }
        s = Locate(&mut db_var.slen) as libc::c_int;
        if s == -(7 as libc::c_int) {
            Index = Index.wrapping_sub(1);
            Index;
        } else if s < 0 as libc::c_int {
            return s
        } else if dir < 0 as libc::c_int {
            Index = Index.wrapping_sub(1);
            Index;
            if Index
                < ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                    / 2 as libc::c_int as u_int
            {
                panic(
                    b"Get_data: Problem with negative direction\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        chunk = &mut *iidx.offset(*idx.offset(Index as isize) as isize)
            as *mut libc::c_int as *mut cstring;
        record = &mut *((*chunk).buf)
            .as_mut_ptr()
            .offset(
                (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as libc::c_int + 2 as libc::c_int) as isize,
            ) as *mut u_char as *mut cstring;
        Align_record();
        if level == dir {
            return s;
        }
        block = *(record as *mut u_int);
        level += 1;
        level;
        s = Get_block(block) as libc::c_int;
        if s < 0 as libc::c_int {
            return s;
        }
    }
    if var_equal((*(*blk[level as usize]).mem).0.global, db_var.name) == 0 {
        return -(61 as libc::c_int + 200 as libc::c_int);
    }
    s = Locate(&mut db_var.slen) as libc::c_int;
    if dir < 1 as libc::c_int {
        *((*systab).last_blk_used)
            .as_mut_ptr()
            .offset(
                ((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                    + ((*systab).maxjob * (volnum - 1 as libc::c_int) as u_int)
                        as libc::c_long) as isize,
            ) = block;
    }
    if db_var.slen == 0 && s == 0
        && (*partab.jobtab).last_block_flags & 2 as libc::c_int as u_int
            == 0 as libc::c_int as u_int
    {
        if (*record).len == 0 {
            s = -(7 as libc::c_int);
        }
    }
    if s == 0 {
        s = (*record).len as libc::c_int;
    }
    return s;
}
