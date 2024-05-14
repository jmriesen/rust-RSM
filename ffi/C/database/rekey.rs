#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type RBD;
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
    static mut systab: *mut systab_struct;
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
    fn Get_block(blknum: u_int) -> libc::c_short;
    fn New_block() -> libc::c_short;
    fn Get_GBD();
    fn Free_GBD(free: *mut gbd);
    fn Get_data(dir: libc::c_int) -> libc::c_int;
    fn Locate(key: *mut u_char) -> libc::c_short;
    fn Locate_next() -> libc::c_short;
    fn Align_record();
    fn Copy_data(fptr: *mut gbd, fidx: libc::c_int);
    fn Garbit(blknum: u_int);
    fn Insert(key: *mut u_char, data: *mut cstring) -> libc::c_short;
    fn Queit();
    fn Tidy_block();
    fn panic(msg: *mut libc::c_char);
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
pub type DB_Block = DB_BLOCK;
pub type gbd = GBD;
#[no_mangle]
pub unsafe extern "C" fn Set_key(
    mut ptr_blk: u_int,
    mut this_level: libc::c_int,
) -> libc::c_short {
    let mut current_block: u64;
    let mut s: libc::c_short = 0;
    let mut t: libc::c_int = 0;
    let mut tmp: [u_char; 8] = [0; 8];
    let mut gtmp: [u_char; 36] = [0; 36];
    let mut i: u_int = 0;
    let mut ui: *mut u_int = 0 as *mut u_int;
    let mut ptr: *mut cstring = 0 as *mut cstring;
    let mut rs: libc::c_int = 0;
    let mut ts: libc::c_int = 0;
    let mut rls: libc::c_int = 0;
    let mut trailings: u_int = 0;
    let mut cblk: [*mut gbd; 3] = [0 as *mut gbd; 3];
    let mut tgb: u_int = 0;
    let mut btmp: *mut DB_Block = 0 as *mut DB_Block;
    ptr = tmp.as_mut_ptr() as *mut cstring;
    (*ptr).len = 4 as libc::c_int as u_short;
    ui = ((*ptr).buf).as_mut_ptr() as *mut u_int;
    *ui = ptr_blk;
    level = this_level;
    if this_level == 0 {
        gtmp[1 as libc::c_int as usize] = 128 as libc::c_int as u_char;
        i = 0 as libc::c_int as u_int;
        while i < 32 as libc::c_int as u_int {
            if db_var.name.var_cu[i as usize] as libc::c_int == '\0' as i32 {
                break;
            }
            gtmp[i.wrapping_add(2 as libc::c_int as u_int)
                as usize] = db_var.name.var_cu[i as usize];
            i = i.wrapping_add(1);
            i;
        }
        i = i.wrapping_add(2 as libc::c_int as u_int);
        gtmp[i as usize] = '\0' as i32 as u_char;
        gtmp[0 as libc::c_int as usize] = i as u_char;
        s = Get_block(
            (*(*(*systab)
                .vol[(db_var.volset as libc::c_int - 1 as libc::c_int) as usize])
                .vollab)
                .uci[(db_var.uci as libc::c_int - 1 as libc::c_int) as usize]
                .global,
        );
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        s = Locate(gtmp.as_mut_ptr());
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        Align_record();
        tgb = *(record as *mut u_int);
        level = 1 as libc::c_int;
        s = New_block();
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        (*(*blk[level as usize]).mem).0.type_0 = db_var.uci;
        (*(*blk[level as usize]).mem)
            .0
            .last_idx = (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
            / 2 as libc::c_int as u_int) as u_short;
        (*(*blk[level as usize]).mem)
            .0
            .last_free = ((*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .vollab)
            .block_size >> 2 as libc::c_int)
            .wrapping_sub(3 as libc::c_int as u_int) as u_short;
        memcpy(
            &mut (*(**blk.as_mut_ptr().offset(level as isize)).mem).0.global
                as *mut var_u as *mut libc::c_void,
            &mut *(db_var.name.var_cu).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            32 as libc::c_int as libc::c_ulong,
        );
        *idx
            .offset(
                (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                    / 2 as libc::c_int as u_int) as isize,
            ) = ((*(*blk[level as usize]).mem).0.last_free as libc::c_int
            + 1 as libc::c_int) as u_short;
        chunk = &mut *iidx
            .offset(
                *idx
                    .offset(
                        (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                            / 2 as libc::c_int as u_int) as isize,
                    ) as isize,
            ) as *mut libc::c_int as *mut cstring;
        (*chunk).len = 8 as libc::c_int as u_short;
        (*chunk).buf[0 as libc::c_int as usize] = 0 as libc::c_int as u_char;
        (*chunk).buf[1 as libc::c_int as usize] = 0 as libc::c_int as u_char;
        record = &mut *((*chunk).buf)
            .as_mut_ptr()
            .offset(
                (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as libc::c_int + 2 as libc::c_int) as isize,
            ) as *mut u_char as *mut cstring;
        *(record as *mut u_int) = tgb;
        t = Insert(&mut db_var.slen, ptr) as libc::c_int;
        if t < 0 as libc::c_int {
            return t as libc::c_short;
        }
        level = 0 as libc::c_int;
        s = Locate(gtmp.as_mut_ptr());
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        Align_record();
        *(record as *mut u_int) = (*blk[1 as libc::c_int as usize]).block;
        level = 1 as libc::c_int;
        (*blk[level as usize]).dirty = blk[level as usize];
        if (*blk[0 as libc::c_int as usize]).dirty == 1 as libc::c_int as *mut gbd {
            (*blk[0 as libc::c_int as usize]).dirty = blk[level as usize];
            level = 0 as libc::c_int;
        }
        Queit();
        return 0 as libc::c_int as libc::c_short;
    }
    t = Get_data(this_level);
    if t >= 0 as libc::c_int {
        return -(61 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    if t != -(7 as libc::c_int) {
        return t as libc::c_short;
    }
    if (*blk[level as usize]).block == ptr_blk {
        return -(61 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    Index = Index.wrapping_add(1);
    Index;
    trailings = Index;
    if trailings
        < (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
            / 2 as libc::c_int as u_int)
            .wrapping_add(1 as libc::c_int as u_int)
    {
        return -(61 as libc::c_int + 200 as libc::c_int) as libc::c_short;
    }
    t = Insert(&mut db_var.slen, ptr) as libc::c_int;
    if t == 0 as libc::c_int {
        if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
            (*blk[level as usize]).dirty = blk[level as usize];
            Queit();
        }
        level -= 1;
        level;
        while level >= 0 as libc::c_int {
            if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                (*blk[level as usize]).dirty = 0 as *mut GBD;
            }
            level -= 1;
            level;
        }
        return 0 as libc::c_int as libc::c_short;
    } else if t != -(62 as libc::c_int + 200 as libc::c_int) {
        return t as libc::c_short
    }
    ts = 0 as libc::c_int;
    if trailings <= (*(*blk[level as usize]).mem).0.last_idx as u_int {
        i = trailings;
        chunk = &mut *iidx.offset(*idx.offset(i as isize) as isize) as *mut libc::c_int
            as *mut cstring;
        ts = (*chunk).buf[0 as libc::c_int as usize] as libc::c_int;
        while i <= (*(*blk[level as usize]).mem).0.last_idx as u_int {
            chunk = &mut *iidx.offset(*idx.offset(i as isize) as isize)
                as *mut libc::c_int as *mut cstring;
            ts += (*chunk).len as libc::c_int + 2 as libc::c_int;
            i = i.wrapping_add(1);
            i;
        }
    }
    rs = 4 as libc::c_int + db_var.slen as libc::c_int + 4 as libc::c_int;
    if rs & 3 as libc::c_int != 0 {
        rs += 4 as libc::c_int - (rs & 3 as libc::c_int);
    }
    rs += 4 as libc::c_int;
    cblk[0 as libc::c_int as usize] = blk[level as usize];
    cblk[1 as libc::c_int as usize] = 0 as *mut gbd;
    cblk[2 as libc::c_int as usize] = 0 as *mut gbd;
    rls = 0 as libc::c_int;
    if (*(*blk[level as usize]).mem).0.right_ptr != 0 {
        s = Get_block((*(*blk[level as usize]).mem).0.right_ptr);
        cblk[2 as libc::c_int as usize] = blk[level as usize];
        if (*(*blk[level as usize]).mem).0.flags as libc::c_int & 1 as libc::c_int != 0 {
            Tidy_block();
        }
        rls = ((*(*blk[level as usize]).mem).0.last_free as libc::c_int
            * 2 as libc::c_int + 1 as libc::c_int
            - (*(*blk[level as usize]).mem).0.last_idx as libc::c_int)
            * 2 as libc::c_int;
    }
    if ts < rls && ts != 0 {
        Un_key();
        Get_GBD();
        memset(
            (*blk[level as usize]).mem as *mut libc::c_void,
            0 as libc::c_int,
            (*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).vollab).block_size
                as libc::c_ulong,
        );
        (*(*blk[level as usize]).mem)
            .0
            .type_0 = (*(*cblk[2 as libc::c_int as usize]).mem).0.type_0;
        (*(*blk[level as usize]).mem)
            .0
            .right_ptr = (*(*cblk[2 as libc::c_int as usize]).mem).0.right_ptr;
        let mut var_i: u_int = 0 as libc::c_int as u_int;
        while var_i < (32 as libc::c_int / 8 as libc::c_int) as u_int {
            (*(*blk[level as usize]).mem)
                .0
                .global
                .var_qu[var_i
                as usize] = (*(*cblk[2 as libc::c_int as usize]).mem)
                .0
                .global
                .var_qu[var_i as usize];
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
            .last_free = ((*(*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .vollab)
            .block_size >> 2 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as u_int) as u_short;
        keybuf[0 as libc::c_int as usize] = 0 as libc::c_int as u_char;
        if ts + rs < rls {
            t = Insert(&mut db_var.slen, ptr) as libc::c_int;
            if t < 0 as libc::c_int {
                panic(
                    b"Set_key: Insert in new block (RL) failed\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            memcpy(
                keybuf.as_mut_ptr() as *mut libc::c_void,
                &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut u_char as *const libc::c_void,
                ((*chunk).buf[1 as libc::c_int as usize] as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            );
        }
        Copy_data(cblk[0 as libc::c_int as usize], trailings as libc::c_int);
        Copy_data(
            cblk[2 as libc::c_int as usize],
            (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                / 2 as libc::c_int as u_int) as libc::c_int,
        );
        btmp = (*blk[level as usize]).mem;
        (*blk[level as usize]).mem = (*cblk[2 as libc::c_int as usize]).mem;
        (*cblk[2 as libc::c_int as usize]).mem = btmp;
        Free_GBD(blk[level as usize]);
        blk[level as usize] = cblk[0 as libc::c_int as usize];
        idx = (*blk[level as usize]).mem as *mut u_short;
        iidx = (*blk[level as usize]).mem as *mut libc::c_int;
        i = trailings;
        while i <= (*(*blk[level as usize]).mem).0.last_idx as u_int {
            chunk = &mut *iidx.offset(*idx.offset(i as isize) as isize)
                as *mut libc::c_int as *mut cstring;
            record = &mut *((*chunk).buf)
                .as_mut_ptr()
                .offset(
                    (*((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                        as libc::c_int + 2 as libc::c_int) as isize,
                ) as *mut u_char as *mut cstring;
            Align_record();
            *(record as *mut libc::c_int) = 0 as libc::c_int;
            i = i.wrapping_add(1);
            i;
        }
        Tidy_block();
        if !(ts + rs < rls) {
            t = Insert(&mut db_var.slen, ptr) as libc::c_int;
            if !(t >= 0 as libc::c_int) {
                if t != -(62 as libc::c_int + 200 as libc::c_int) {
                    return t as libc::c_short;
                }
                s = New_block();
                if (s as libc::c_int) < 0 as libc::c_int {
                    panic(
                        b"Set_key: Failed to get new block for insert\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                (*(*blk[level as usize]).mem)
                    .0
                    .type_0 = (*(*cblk[0 as libc::c_int as usize]).mem).0.type_0;
                (*(*blk[level as usize]).mem)
                    .0
                    .right_ptr = (*(*cblk[0 as libc::c_int as usize]).mem).0.right_ptr;
                let mut var_i_0: u_int = 0 as libc::c_int as u_int;
                while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*(*blk[level as usize]).mem)
                        .0
                        .global
                        .var_qu[var_i_0
                        as usize] = (*(*cblk[0 as libc::c_int as usize]).mem)
                        .0
                        .global
                        .var_qu[var_i_0 as usize];
                    var_i_0 = var_i_0.wrapping_add(1);
                    var_i_0;
                }
                (*(*blk[level as usize]).mem)
                    .0
                    .last_idx = (::core::mem::size_of::<DB_Block>() as libc::c_ulong
                    as u_int / 2 as libc::c_int as u_int)
                    .wrapping_sub(1 as libc::c_int as u_int) as u_short;
                (*(*blk[level as usize]).mem)
                    .0
                    .last_free = ((*(*(*systab)
                    .vol[(volnum - 1 as libc::c_int) as usize])
                    .vollab)
                    .block_size >> 2 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as u_int) as u_short;
                keybuf[0 as libc::c_int as usize] = 0 as libc::c_int as u_char;
                (*(*cblk[0 as libc::c_int as usize]).mem)
                    .0
                    .right_ptr = (*blk[level as usize]).block;
                t = Insert(&mut db_var.slen, ptr) as libc::c_int;
                if t < 0 as libc::c_int {
                    panic(
                        b"Set_key: Insert in new block (insert) failed\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                cblk[1 as libc::c_int as usize] = blk[level as usize];
            }
        }
    } else {
        if rs < rls && ts == 0 {
            blk[level as usize] = cblk[2 as libc::c_int as usize];
            Un_key();
            t = Insert(&mut db_var.slen, ptr) as libc::c_int;
            if t >= 0 as libc::c_int {
                current_block = 11228237057954672213;
            } else {
                if t != -(62 as libc::c_int + 200 as libc::c_int) {
                    return t as libc::c_short;
                }
                current_block = 17294538769010429813;
            }
        } else {
            if !(cblk[2 as libc::c_int as usize]).is_null() {
                if (*cblk[2 as libc::c_int as usize]).dirty
                    == 1 as libc::c_int as *mut gbd
                {
                    (*cblk[2 as libc::c_int as usize]).dirty = 0 as *mut GBD;
                }
                cblk[2 as libc::c_int as usize] = 0 as *mut gbd;
            }
            current_block = 17294538769010429813;
        }
        match current_block {
            11228237057954672213 => {}
            _ => {
                s = New_block();
                if (s as libc::c_int) < 0 as libc::c_int {
                    panic(
                        b"Set_key: Failed to get new block for trailings\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                (*(*blk[level as usize]).mem)
                    .0
                    .type_0 = (*(*cblk[0 as libc::c_int as usize]).mem).0.type_0;
                (*(*blk[level as usize]).mem)
                    .0
                    .right_ptr = (*(*cblk[0 as libc::c_int as usize]).mem).0.right_ptr;
                let mut var_i_1: u_int = 0 as libc::c_int as u_int;
                while var_i_1 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*(*blk[level as usize]).mem)
                        .0
                        .global
                        .var_qu[var_i_1
                        as usize] = (*(*cblk[0 as libc::c_int as usize]).mem)
                        .0
                        .global
                        .var_qu[var_i_1 as usize];
                    var_i_1 = var_i_1.wrapping_add(1);
                    var_i_1;
                }
                (*(*blk[level as usize]).mem)
                    .0
                    .last_idx = (::core::mem::size_of::<DB_Block>() as libc::c_ulong
                    as u_int / 2 as libc::c_int as u_int)
                    .wrapping_sub(1 as libc::c_int as u_int) as u_short;
                (*(*blk[level as usize]).mem)
                    .0
                    .last_free = ((*(*(*systab)
                    .vol[(volnum - 1 as libc::c_int) as usize])
                    .vollab)
                    .block_size >> 2 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as u_int) as u_short;
                keybuf[0 as libc::c_int as usize] = 0 as libc::c_int as u_char;
                (*(*cblk[0 as libc::c_int as usize]).mem)
                    .0
                    .right_ptr = (*blk[level as usize]).block;
                cblk[1 as libc::c_int as usize] = blk[level as usize];
                if ts != 0 {
                    Copy_data(cblk[0 as libc::c_int as usize], trailings as libc::c_int);
                    blk[level as usize] = cblk[0 as libc::c_int as usize];
                    idx = (*blk[level as usize]).mem as *mut u_short;
                    iidx = (*blk[level as usize]).mem as *mut libc::c_int;
                    i = trailings;
                    while i <= (*(*blk[level as usize]).mem).0.last_idx as u_int {
                        chunk = &mut *iidx.offset(*idx.offset(i as isize) as isize)
                            as *mut libc::c_int as *mut cstring;
                        record = &mut *((*chunk).buf)
                            .as_mut_ptr()
                            .offset(
                                (*((*chunk).buf)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize) as libc::c_int
                                    + 2 as libc::c_int) as isize,
                            ) as *mut u_char as *mut cstring;
                        Align_record();
                        *(record as *mut libc::c_int) = 0 as libc::c_int;
                        i = i.wrapping_add(1);
                        i;
                    }
                    Tidy_block();
                    t = Insert(&mut db_var.slen, ptr) as libc::c_int;
                    if t >= 0 as libc::c_int {
                        current_block = 11228237057954672213;
                    } else {
                        if t != -(62 as libc::c_int + 200 as libc::c_int) {
                            return t as libc::c_short;
                        }
                        blk[level as usize] = cblk[1 as libc::c_int as usize];
                        idx = (*blk[level as usize]).mem as *mut u_short;
                        iidx = (*blk[level as usize]).mem as *mut libc::c_int;
                        current_block = 15514437232607373049;
                    }
                } else {
                    current_block = 15514437232607373049;
                }
                match current_block {
                    11228237057954672213 => {}
                    _ => {
                        t = Insert(&mut db_var.slen, ptr) as libc::c_int;
                        if !(t >= 0 as libc::c_int) {
                            if t != -(62 as libc::c_int + 200 as libc::c_int) {
                                return t as libc::c_short;
                            }
                            panic(
                                b"Set_key: Options 0->5 didn't work\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                    }
                }
            }
        }
    }
    blk[level as usize] = 0 as *mut gbd;
    let mut j: libc::c_int = level - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        if (*blk[j as usize]).dirty == 2 as libc::c_int as *mut gbd {
            if (blk[level as usize]).is_null() {
                (*blk[j as usize]).dirty = blk[j as usize];
            } else {
                (*blk[j as usize]).dirty = blk[level as usize];
            }
            blk[level as usize] = blk[j as usize];
        } else if (*blk[j as usize]).dirty == 1 as libc::c_int as *mut gbd {
            (*blk[j as usize]).dirty = 0 as *mut GBD;
        }
        j -= 1;
        j;
    }
    i = 0 as libc::c_int as u_int;
    while i < 3 as libc::c_int as u_int {
        if !(cblk[i as usize]).is_null() {
            if (*cblk[i as usize]).dirty == 1 as libc::c_int as *mut gbd {
                if (blk[level as usize]).is_null() {
                    (*cblk[i as usize]).dirty = cblk[i as usize];
                } else {
                    (*cblk[i as usize]).dirty = blk[level as usize];
                }
                blk[level as usize] = cblk[i as usize];
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if !(blk[level as usize]).is_null() {
        Queit();
    }
    i = 1 as libc::c_int as u_int;
    while i < 3 as libc::c_int as u_int {
        if !(cblk[i as usize]).is_null() {
            Add_rekey((*cblk[i as usize]).block, this_level);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Add_rekey(
    mut block: u_int,
    mut level_0: libc::c_int,
) -> libc::c_short {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int * 3 as libc::c_int {
        if rekey_blk[i as usize] == 0 {
            rekey_blk[i as usize] = block;
            rekey_lvl[i as usize] = level_0;
            return 0 as libc::c_int as libc::c_short;
        }
        i += 1;
        i;
    }
    panic(
        b"Add_rekey: Rekey table overflowed - database is corrupt\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    return 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn Re_key() -> libc::c_short {
    loop {
        let mut s: libc::c_short = 0;
        let mut low_level: libc::c_int = -(1 as libc::c_int);
        let mut low_index: libc::c_int = -(1 as libc::c_int);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 12 as libc::c_int * 3 as libc::c_int {
            if rekey_blk[i as usize] != 0 {
                if rekey_lvl[i as usize] > low_level {
                    low_level = rekey_lvl[i as usize];
                    low_index = i;
                }
            }
            i += 1;
            i;
        }
        if low_index == -(1 as libc::c_int) {
            return 0 as libc::c_int as libc::c_short;
        }
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .stats
            .blkreorg = ((*(*systab).vol[(volnum - 1 as libc::c_int) as usize])
            .stats
            .blkreorg)
            .wrapping_add(1);
        (*(*systab).vol[(volnum - 1 as libc::c_int) as usize]).stats.blkreorg;
        level = 0 as libc::c_int;
        s = Get_block(rekey_blk[low_index as usize]);
        if (s as libc::c_int) < 0 as libc::c_int {
            return -(61 as libc::c_int + 200 as libc::c_int) as libc::c_short;
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
            &mut db_var.slen as *mut u_char as *mut libc::c_void,
            &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut u_char as *const libc::c_void,
            ((*chunk).buf[1 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong,
        );
        if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
            (*blk[level as usize]).dirty = 0 as *mut GBD;
        }
        s = Set_key(rekey_blk[low_index as usize], low_level - 1 as libc::c_int);
        if (s as libc::c_int) < 0 as libc::c_int {
            return s;
        }
        rekey_lvl[low_index as usize] = 0 as libc::c_int;
        rekey_blk[low_index as usize] = 0 as libc::c_int as u_int;
        if low_level == 1 as libc::c_int {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 12 as libc::c_int * 3 as libc::c_int {
                if rekey_lvl[i_0 as usize] != 0 {
                    rekey_lvl[i_0 as usize] += 1;
                    rekey_lvl[i_0 as usize];
                }
                i_0 += 1;
                i_0;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Un_key() {
    let mut this_level: libc::c_int = 0;
    let mut save_level: libc::c_int = 0;
    let mut xxx_level: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut cstr: [u_char; 8] = [0; 8];
    let mut xptr: *mut cstring = 0 as *mut cstring;
    let mut save: *mut gbd = 0 as *mut gbd;
    let mut uptr: *mut u_char = 0 as *mut u_char;
    let mut lptr: *mut u_char = 0 as *mut u_char;
    let mut blkno: u_int = 0;
    this_level = level;
    idx = (*blk[level as usize]).mem as *mut u_short;
    iidx = (*blk[level as usize]).mem as *mut libc::c_int;
    chunk = &mut *iidx
        .offset(
            *idx
                .offset(
                    (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                        / 2 as libc::c_int as u_int) as isize,
                ) as isize,
        ) as *mut libc::c_int as *mut cstring;
    uptr = &mut *((*chunk).buf).as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut u_char;
    level = level - 1 as libc::c_int;
    while level != 0 {
        let mut s: libc::c_short = Locate(uptr);
        if s as libc::c_int == -(7 as libc::c_int) {
            if Index > (*(*blk[level as usize]).mem).0.last_idx as u_int {
                save = blk[level as usize];
                s = Locate_next();
                if s as libc::c_int == 0 as libc::c_int {
                    s = Locate(uptr);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                            (*blk[level as usize]).dirty = 0 as *mut GBD;
                        }
                        blk[level as usize] = save;
                    } else if (*save).dirty == 1 as libc::c_int as *mut gbd {
                        (*save).dirty = 0 as *mut GBD;
                    }
                }
            }
        }
        if !(s as libc::c_int == 0 as libc::c_int) {
            break;
        }
        Align_record();
        *(record as *mut libc::c_int) = 0 as libc::c_int;
        Tidy_block();
        if level < this_level - 1 as libc::c_int {
            if (*(*blk[(level + 1 as libc::c_int) as usize]).mem).0.last_idx as u_int
                > (::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                    / 2 as libc::c_int as u_int)
                    .wrapping_sub(1 as libc::c_int as u_int)
            {
                let mut xui: *mut u_int = 0 as *mut u_int;
                idx = (*blk[(level + 1 as libc::c_int) as usize]).mem as *mut u_short;
                iidx = (*blk[(level + 1 as libc::c_int) as usize]).mem
                    as *mut libc::c_int;
                chunk = &mut *iidx
                    .offset(
                        *idx
                            .offset(
                                (::core::mem::size_of::<DB_Block>() as libc::c_ulong
                                    as u_int / 2 as libc::c_int as u_int) as isize,
                            ) as isize,
                    ) as *mut libc::c_int as *mut cstring;
                lptr = &mut *((*chunk).buf)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut u_char;
                xptr = cstr.as_mut_ptr() as *mut cstring;
                (*xptr).len = 4 as libc::c_int as u_short;
                xui = ((*xptr).buf).as_mut_ptr() as *mut u_int;
                *xui = (*blk[(level + 1 as libc::c_int) as usize]).block;
                t = Insert(lptr, xptr) as libc::c_int;
                if t == -(62 as libc::c_int + 200 as libc::c_int) {
                    Add_rekey(
                        (*blk[(level + 1 as libc::c_int) as usize]).block,
                        level + 1 as libc::c_int,
                    );
                } else if t < 0 as libc::c_int {
                    panic(
                        b"Un_Key: Insert returned fatal value\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
            } else {
                save_level = level;
                blkno = 0 as libc::c_int as u_int;
                loop {
                    s = Locate(uptr);
                    if s as libc::c_int != -(7 as libc::c_int) {
                        panic(
                            b"Un_key: Key locate at 'level' didn't return -ERRM7\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    if Index
                        > ::core::mem::size_of::<DB_Block>() as libc::c_ulong as u_int
                            / 2 as libc::c_int as u_int
                    {
                        chunk = &mut *iidx
                            .offset(
                                *idx
                                    .offset(
                                        Index.wrapping_sub(1 as libc::c_int as u_int) as isize,
                                    ) as isize,
                            ) as *mut libc::c_int as *mut cstring;
                        record = &mut *((*chunk).buf)
                            .as_mut_ptr()
                            .offset(
                                (*((*chunk).buf)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize) as libc::c_int
                                    + 2 as libc::c_int) as isize,
                            ) as *mut u_char as *mut cstring;
                        Align_record();
                        blkno = *(record as *mut u_int);
                        break;
                    } else {
                        level -= 1;
                        level;
                        if level == 0 {
                            panic(
                                b"Un_key: Failed to determine left edge\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                    }
                }
                while level < save_level {
                    xxx_level = level;
                    level = 12 as libc::c_int - 1 as libc::c_int;
                    s = Get_block(blkno);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        panic(
                            b"Un_key: Get_block() failed in left block tree\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    s = Locate(uptr);
                    if s as libc::c_int != -(7 as libc::c_int) {
                        panic(
                            b"Un_key: Key locate in left edge didn't return -ERRM7\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    chunk = &mut *iidx
                        .offset(
                            *idx
                                .offset(
                                    Index.wrapping_sub(1 as libc::c_int as u_int) as isize,
                                ) as isize,
                        ) as *mut libc::c_int as *mut cstring;
                    record = &mut *((*chunk).buf)
                        .as_mut_ptr()
                        .offset(
                            (*((*chunk).buf)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize) as libc::c_int
                                + 2 as libc::c_int) as isize,
                        ) as *mut u_char as *mut cstring;
                    Align_record();
                    blkno = *(record as *mut u_int);
                    if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                        (*blk[level as usize]).dirty = 0 as *mut GBD;
                    }
                    level = xxx_level;
                    level += 1;
                    level;
                }
                xxx_level = 12 as libc::c_int - 1 as libc::c_int;
                level += 1;
                level;
                blk[xxx_level as usize] = blk[level as usize];
                s = Get_block(blkno);
                if (s as libc::c_int) < 0 as libc::c_int {
                    panic(
                        b"Un_key: Get_block() failed for left block\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                (*(*blk[level as usize]).mem)
                    .0
                    .right_ptr = (*(*blk[xxx_level as usize]).mem).0.right_ptr;
                if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
                    (*blk[level as usize]).dirty = 2 as libc::c_int as *mut gbd;
                }
                Garbit((*blk[xxx_level as usize]).block);
                level = save_level;
            }
        }
        if (*blk[level as usize]).dirty == 1 as libc::c_int as *mut gbd {
            (*blk[level as usize]).dirty = 2 as libc::c_int as *mut gbd;
        }
        level -= 1;
        level;
    }
    level = this_level;
}
