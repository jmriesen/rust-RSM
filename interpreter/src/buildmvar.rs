/*
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::iter;

use ffi::{addstk, cstring, partab, systab, var_u, CSTRING, ERRM26, GBD, MVAR, TYPVARNAKED};

pub type __int64_t = libc::c_longlong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type off_t = __darwin_off_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type time_t = __darwin_time_t;
pub type u_int64 = libc::c_ulonglong;
pub type mvar = MVAR;
#[repr(C, packed)]
pub struct RBD {
    pub fwd_link: *mut RBD,
    pub chunk_size: u_int,
    pub attached: u_int,
    pub last_access: time_t,
    pub rnam: var_u,
    pub uci: u_char,
    pub vol: u_char,
    pub rou_size: u_short,
    pub comp_ver: u_short,
    pub comp_user: u_short,
    pub comp_date: libc::c_int,
    pub comp_time: libc::c_int,
    pub tag_tbl: u_short,
    pub num_tags: u_short,
    pub var_tbl: u_short,
    pub num_vars: u_short,
    pub code: u_short,
    pub code_size: u_short,
}
#[repr(C, packed)]
pub struct UCI_TAB {
    pub name: var_u,
    pub global: u_int,
}
pub type uci_tab = UCI_TAB;
#[repr(C, packed)]
pub union DATA_UNION {
    pub gbddata: *mut GBD,
    pub intdata: u_int,
}
pub type msg_data = DATA_UNION;
#[repr(C, packed)]
pub struct WD_TAB {
    pub pid: libc::c_int,
    pub doing: libc::c_int,
    pub currmsg: msg_data,
}
pub type wdtab_struct = WD_TAB;
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
#[repr(C, packed)]
pub struct FORKTAB {
    pub job_no: libc::c_int,
    pub pid: libc::c_int,
}
pub type forktab = FORKTAB;
#[repr(C, packed)]
pub struct SERVERTAB {
    pub slots: libc::c_int,
    pub taken: libc::c_int,
    pub cid: libc::c_int,
    pub name: [u_char; 256],
    pub forked: *mut forktab,
}
pub type servertab = SERVERTAB;
#[repr(C)]
pub union IN_TERM {
    pub iterm: u_int64,
    pub interm: [u_int64; 2],
}
pub type IN_Term = IN_TERM;
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
#[repr(C, packed)]
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
    pub lp: *mut *mut CSTRING,
    pub ln: *mut libc::c_int,
    pub src_var: mvar,
}
pub type partab_struct = PARTAB;
pub type rbd = RBD;

fn var_empty(var: &var_u) -> libc::c_int {
    unsafe { var.var_q == 0 }.into()
}

#[no_mangle]
pub unsafe extern "C" fn getvol(mut vol: *mut cstring) -> libc::c_short {
    let vol = String::from_utf8((*vol).buf[..((*vol).len as usize)].to_vec())
        //As far as I know cstrings should only contain ASCII
        //ASCII is valid UTF8 I think this should just work.
        //NOTE I am not using from_utf8_unchecked since I am not 100% my assumption is correct.
        .unwrap();

    let sys_tab = unsafe { systab.cast::<crate::shared_seg::sys_tab::SYSTAB>().as_mut() }.unwrap();
    sys_tab
        .get_vol(&vol)
        .map(|x| (x + 1) as libc::c_short)
        .unwrap_or(-(ERRM26 as libc::c_short))
}

#[no_mangle]
pub unsafe extern "C" fn getuci(mut uci: *mut cstring, mut vol: libc::c_int) -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut s: u_short = 0;
    s = (*uci).len;
    if (s as libc::c_int) < 32 as libc::c_int {
        s = s.wrapping_add(1);
        s;
    }
    if vol == 0 as libc::c_int {
        vol = (*partab.jobtab).vol as libc::c_int;
    }
    vol -= 1;
    vol;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if true
        /*memcmp(
            ((*uci).buf).as_mut_ptr() as *const libc::c_void,
            &mut *((*((*(**((*systab).vol).as_mut_ptr().offset(vol as isize)).vollab)
                .uci)
                .as_mut_ptr()
                .offset(i as isize))
                .name
                .var_cu)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut u_char as *const libc::c_void,
            s as libc::c_ulong,
        ) == 0 as libc::c_int*/
        {
            return (i + 1 as libc::c_int) as libc::c_short;
        }
        i += 1;
        i;
    }
    return -(26 as libc::c_int) as libc::c_short;
}

unsafe fn is_null(string: *mut CSTRING) -> bool {
    (*string).len == 0
}

pub fn buildmvar(
    mut var: *mut mvar,
    mut nul_ok: bool,
    mut asp: usize,
    //TODO update return type so it returns the new mvar.
) -> Result<usize, libc::c_short> {
    use ffi::rsmpc;
    let pop = || {
        //TODO build an abstraction over the rsmpc type.
        let temp = unsafe { *rsmpc };
        unsafe { rsmpc = rsmpc.offset(1) };
        temp
    };
    let (var_type, numb_subs) = {
        let metadata = pop();
        if (metadata) < TYPVARNAKED as u8 {
            (
                metadata & !ffi::TYPMAXSUB as u8,
                metadata & ffi::TYPMAXSUB as u8,
            )
        } else {
            (metadata, pop())
        }
    };

    //see if I can get ride of the var param.
    unsafe {
        (*var).volset = 0;
        (*var).uci = if var_type < ffi::TYPVARGBL as u8 {
            ffi::UCI_IS_LOCALVAR as u8
        } else {
            0
        };
        (*var).slen = 0;
    }

    match var_type as u32 {
        ffi::TYPVARNAKED => {
            todo!()
            /*
               if (var_empty(partab.jobtab->last_ref.name)) return -ERRM1;
               i = UTIL_Key_Last(&partab.jobtab->last_ref)
               if (i < 0) return -ERRM1;
               memcpy(var, &partab.jobtab->last_ref, sizeof(var_u) + 5 + i)
               var->slen = (u_char) i;
            */
        }
        ffi::TYPVARIND => {
            todo!()
            /*
               ind = (mvar *) addstk[asp - subs - 1]
               memmove(var, ind, ind->slen + sizeof(var_u) + 5)
            */
        }
        x if (x & ffi::TYPVARIDX != 0 && x < ffi::TYPVARGBL) => {
            todo!()
            /*
                    i = *rsmpc+

                    if (i < 255) {
                    var->volset = i + 1;
                    VAR_CLEAR(var->name)
            } else {
                    p = (rbd *) (partab.jobtab->dostk[partab.jobtab->cur_do].routine);
                    vt = (var_u *) (((u_char *) p) + p->var_tbl)
                    VAR_COPY(var->name, vt[i]
            }
                 */
        }
        _ => iter::repeat_with(|| pop())
            .take(ffi::VAR_LEN as usize)
            .map(|x| char::from(x))
            .collect::<String>(),
    };

    for i in (0..numb_subs as usize).rev() {
        let sub = unsafe {
            //TODO build abstraction over the addstk
            // this should be a pop
            addstk[asp - i].cast::<CSTRING>()
        };
        //Null is only allowed if it is the last index and nul_ok is true.
        if unsafe { is_null(sub) } && (!nul_ok || (i != 0)) {
            return Err(-((ffi::ERRZ16 + ffi::ERRMLAST) as i16)); // complain
        } else {

            /*
            s = UTIL_Key_Build(ptr, &var->key[var->slen]);                          // get one subscript
            if (s < 0) return s;                                                    // die on error
            if ((s + var->slen) > 255) return -(ERRZ2 + ERRMLAST);                  // check how big and complain on error
            var->slen = s + var->slen;                                              // add it in

            */
        }

        /*
            if (var_type == TYPVARGBLUCIENV) {                                              // need vol?
            let ptr = unsafe{
            addstk[asp - numb_subs -1].cast::<CSTRING>();
        };
            s = getvol(ptr);                                                        // get volume
            if (s < 0) return s;                                                    // die on error
            var->volset = (u_char) s;                                               // save the value
        }

            if ((type == TYPVARGBLUCI) || (type == TYPVARGBLUCIENV)) {                  // need UCI?
            ptr = (cstring *) addstk[asp - subs - 1 - (type == TYPVARGBLUCIENV)];   // point at the string
            s = getuci(ptr, var->volset);                                           // get UCI
            if (s < 0) return s;                                                    // die on error
            var->uci = (u_char) s;                                                  // save the value
        }

            if (type == TYPVARIND) asp--;                                               // fixup asp for return
             */
    }
    //    return (asp - numb_subs - (var_type == TYPVARGBLUCI) - ((var_type == TYPVARGBLUCIENV) * 2)); // all done

    0
}
*/
