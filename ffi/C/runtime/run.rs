#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(linkage)]
extern "C" {
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
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    fn sleep(_: libc::c_uint) -> libc::c_uint;
    fn __error() -> *mut libc::c_int;
    static mut systab: *mut systab_struct;
    static mut partab: partab_struct;
    static mut addstk: [*mut u_char; 0];
    static mut strstk: [u_char; 0];
    static mut rsmpc: *mut u_char;
    fn DB_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn DB_Set(var: *mut mvar, data: *mut cstring) -> libc::c_int;
    fn DB_Kill(var: *mut mvar) -> libc::c_short;
    fn DB_QueryD(var: *mut mvar, buf: *mut u_char) -> libc::c_short;
    fn DB_Compress(var: *mut mvar, flags: libc::c_int) -> libc::c_short;
    fn DB_ic(vol: libc::c_int, block: libc::c_int) -> libc::c_int;
    fn DB_ViewGet(vol: u_int, block: u_int) -> *mut GBD;
    fn DB_ViewPut(vol: u_int, ptr: *mut GBD);
    fn DB_ViewRel(vol: u_int, ptr: *mut GBD);
    fn SQ_Open(
        chan: libc::c_int,
        object: *mut cstring,
        op: *mut cstring,
        tout: libc::c_int,
    ) -> libc::c_short;
    fn SQ_Use(
        chan: libc::c_int,
        interm: *mut cstring,
        outerm: *mut cstring,
        par: libc::c_int,
    ) -> libc::c_short;
    fn SQ_Close(chan: libc::c_int) -> libc::c_short;
    fn SQ_Write(buf: *mut cstring) -> libc::c_int;
    fn SQ_WriteStar(c: u_char) -> libc::c_short;
    fn SQ_WriteFormat(count: libc::c_int) -> libc::c_short;
    fn SQ_Read(buf: *mut u_char, tout: libc::c_int, maxbyt: libc::c_int) -> libc::c_int;
    fn SQ_ReadStar(result: *mut libc::c_int, timeout: libc::c_int) -> libc::c_short;
    fn SQ_Flush() -> libc::c_short;
    fn SQ_Device(buf: *mut u_char) -> libc::c_int;
    fn Compile_Routine(
        rou: *mut mvar,
        src: *mut mvar,
        stack: *mut u_char,
    ) -> libc::c_int;
    fn eval();
    fn parse();
    fn cstringtoi(str: *mut cstring) -> libc::c_int;
    fn cstringtob(str: *mut cstring) -> libc::c_int;
    fn itocstring(buf: *mut u_char, n: libc::c_int) -> u_short;
    fn uitocstring(buf: *mut u_char, n: u_int) -> u_short;
    fn Set_Error(
        err: libc::c_int,
        user: *mut cstring,
        space: *mut cstring,
    ) -> libc::c_int;
    fn buildmvar(var: *mut mvar, nul_ok: libc::c_int, asp: libc::c_int) -> libc::c_short;
    fn patmat(str: *mut cstring, code: *mut cstring) -> libc::c_short;
    fn attention() -> libc::c_short;
    fn ForkIt(cft: libc::c_int) -> libc::c_int;
    fn SchedYield();
    fn runtime_add(a: *mut libc::c_char, b: *mut libc::c_char) -> libc::c_short;
    fn runtime_mul(a: *mut libc::c_char, b: *mut libc::c_char) -> libc::c_short;
    fn runtime_div(
        uu: *mut libc::c_char,
        v: *mut libc::c_char,
        typ: libc::c_short,
    ) -> libc::c_short;
    fn runtime_power(a: *mut libc::c_char, b: *mut libc::c_char) -> libc::c_short;
    fn runtime_comp(s: *mut libc::c_char, t: *mut libc::c_char) -> libc::c_short;
    fn Dascii1(ret_buffer: *mut u_char, expr: *mut cstring) -> libc::c_short;
    fn Dascii2(
        ret_buffer: *mut u_char,
        expr: *mut cstring,
        posn: libc::c_int,
    ) -> libc::c_short;
    fn Dchar(ret_buffer: *mut u_char, i: libc::c_int) -> libc::c_short;
    fn Ddata(ret_buffer: *mut u_char, var: *mut mvar) -> libc::c_short;
    fn Dextract(
        ret_buffer: *mut u_char,
        expr: *mut cstring,
        start: libc::c_int,
        stop: libc::c_int,
    ) -> libc::c_int;
    fn Dfind2(
        ret_buffer: *mut u_char,
        expr1: *mut cstring,
        expr2: *mut cstring,
    ) -> libc::c_int;
    fn Dfind3(
        ret_buffer: *mut u_char,
        expr1: *mut cstring,
        expr2: *mut cstring,
        start: libc::c_int,
    ) -> libc::c_int;
    fn Dfind3x(
        expr1: *mut cstring,
        expr2: *mut cstring,
        start: libc::c_int,
    ) -> libc::c_int;
    fn Dfnumber2(
        ret_buffer: *mut u_char,
        numexp: *mut cstring,
        code: *mut cstring,
    ) -> libc::c_int;
    fn Dfnumber3(
        ret_buffer: *mut u_char,
        numexp: *mut cstring,
        code: *mut cstring,
        rnd: libc::c_int,
    ) -> libc::c_int;
    fn Dget1(ret_buffer: *mut u_char, var: *mut mvar) -> libc::c_int;
    fn Dget2(ret_buffer: *mut u_char, var: *mut mvar, expr: *mut cstring) -> libc::c_int;
    fn Dincrement1(ret_buffer: *mut u_char, var: *mut mvar) -> libc::c_short;
    fn Dincrement2(
        ret_buffer: *mut u_char,
        var: *mut mvar,
        numexpr: *mut cstring,
    ) -> libc::c_short;
    fn Djustify2(
        ret_buffer: *mut u_char,
        expr: *mut cstring,
        size: libc::c_int,
    ) -> libc::c_int;
    fn Djustify3(
        ret_buffer: *mut u_char,
        expr: *mut cstring,
        size: libc::c_int,
        round: libc::c_int,
    ) -> libc::c_int;
    fn Dlength1(ret_buffer: *mut u_char, expr: *mut cstring) -> libc::c_short;
    fn Dlength2(
        ret_buffer: *mut u_char,
        expr: *mut cstring,
        delim: *mut cstring,
    ) -> libc::c_short;
    fn Dname1(ret_buffer: *mut u_char, var: *mut mvar) -> libc::c_short;
    fn Dname2(
        ret_buffer: *mut u_char,
        var: *mut mvar,
        sub: libc::c_int,
    ) -> libc::c_short;
    fn Dorder1(ret_buffer: *mut u_char, var: *mut mvar) -> libc::c_short;
    fn Dorder2(
        ret_buffer: *mut u_char,
        var: *mut mvar,
        dir: libc::c_int,
    ) -> libc::c_short;
    fn Dpiece2(
        ret_buffer: *mut u_char,
        expr: *mut cstring,
        delim: *mut cstring,
    ) -> libc::c_int;
    fn Dpiece3(
        ret_buffer: *mut u_char,
        expr: *mut cstring,
        delim: *mut cstring,
        i1: libc::c_int,
    ) -> libc::c_int;
    fn Dpiece4(
        ret_buffer: *mut u_char,
        expr: *mut cstring,
        delim: *mut cstring,
        i1: libc::c_int,
        i2: libc::c_int,
    ) -> libc::c_int;
    fn Dquery2(
        ret_buffer: *mut u_char,
        var: *mut mvar,
        dir: libc::c_int,
    ) -> libc::c_short;
    fn Drandom(ret_buffer: *mut u_char, seed: libc::c_int) -> libc::c_short;
    fn Dreverse(ret_buffer: *mut u_char, expr: *mut cstring) -> libc::c_int;
    fn Dstack1(ret_buffer: *mut u_char, level: libc::c_int) -> libc::c_short;
    fn Dstack2(
        ret_buffer: *mut u_char,
        level: libc::c_int,
        code: *mut cstring,
    ) -> libc::c_int;
    fn Dtext(ret_buffer: *mut u_char, str: *mut cstring) -> libc::c_int;
    fn Dtranslate2(
        ret_buffer: *mut u_char,
        expr1: *mut cstring,
        expr2: *mut cstring,
    ) -> libc::c_int;
    fn Dtranslate3(
        ret_buffer: *mut u_char,
        expr1: *mut cstring,
        expr2: *mut cstring,
        expr3: *mut cstring,
    ) -> libc::c_int;
    fn Dview(
        ret_buffer: *mut u_char,
        chan: libc::c_int,
        loc: libc::c_int,
        size: libc::c_int,
        value: *mut cstring,
    ) -> libc::c_int;
    fn DSetextract(
        tmp: *mut u_char,
        cptr: *mut cstring,
        var: *mut mvar,
        i1: libc::c_int,
        i2: libc::c_int,
    ) -> libc::c_int;
    fn DSetpiece(
        tmp: *mut u_char,
        cptr: *mut cstring,
        var: *mut mvar,
        dptr: *mut cstring,
        i1: libc::c_int,
        i2: libc::c_int,
    ) -> libc::c_int;
    fn Vecode(ret_buffer: *mut u_char) -> libc::c_int;
    fn Vetrap(ret_buffer: *mut u_char) -> libc::c_int;
    fn Vhorolog(ret_buffer: *mut u_char) -> libc::c_short;
    fn Vkey(ret_buffer: *mut u_char) -> libc::c_short;
    fn Vreference(ret_buffer: *mut u_char) -> libc::c_short;
    fn Vsystem(ret_buffer: *mut u_char) -> libc::c_short;
    fn Vx(ret_buffer: *mut u_char) -> libc::c_short;
    fn Vy(ret_buffer: *mut u_char) -> libc::c_short;
    fn Vset(var: *mut mvar, cptr: *mut cstring) -> libc::c_int;
    fn ST_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn ST_GetAdd(var: *mut mvar, add: *mut *mut cstring) -> libc::c_int;
    fn ST_Set(var: *mut mvar, data: *mut cstring) -> libc::c_int;
    fn ST_Kill(var: *mut mvar) -> libc::c_short;
    fn ST_KillAll(count: libc::c_int, keep: *mut var_u) -> libc::c_short;
    fn ST_QueryD(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn ST_SymAtt(var: var_u) -> libc::c_short;
    fn ST_SymDet(count: libc::c_int, list: *mut libc::c_short);
    fn ST_SymSet(syment: libc::c_short, data: *mut cstring) -> libc::c_short;
    fn ST_New(count: libc::c_int, list: *mut var_u) -> libc::c_short;
    fn ST_NewAll(count: libc::c_int, list: *mut var_u) -> libc::c_short;
    fn ST_ConData(var: *mut mvar, data: *mut u_char) -> libc::c_short;
    fn SS_Get(var: *mut mvar, buf: *mut u_char) -> libc::c_int;
    fn SS_Set(var: *mut mvar, data: *mut cstring) -> libc::c_short;
    fn SS_Kill(var: *mut mvar) -> libc::c_short;
    fn UTIL_Key_Build(src: *mut cstring, dest: *mut u_char) -> libc::c_short;
    fn UTIL_Key_Extract(
        key: *mut u_char,
        str: *mut u_char,
        cnt: *mut libc::c_int,
    ) -> libc::c_short;
    fn UTIL_MvarFromCStr(src: *mut cstring, var: *mut mvar) -> libc::c_short;
    fn mcopy(src: *mut u_char, dst: *mut u_char, bytes: libc::c_int) -> libc::c_int;
    fn ncopy(src: *mut *mut u_char, dst: *mut u_char) -> libc::c_short;
    fn CleanJob(job: libc::c_int);
    fn panic(msg: *mut libc::c_char);
    fn Routine_Attach(routine: var_u) -> *mut RBD;
    fn Routine_Detach(pointer: *mut RBD);
    fn UTIL_mvartolock(var: *mut mvar, buf: *mut u_char) -> libc::c_short;
    fn LCK_Remove(job: libc::c_int);
    fn LCK_Old(count: libc::c_int, list: *mut cstring, to: libc::c_int) -> libc::c_short;
    fn LCK_Add(count: libc::c_int, list: *mut cstring, to: libc::c_int) -> libc::c_short;
    fn LCK_Sub(count: libc::c_int, list: *mut cstring) -> libc::c_short;
    fn Xcall_host(
        ret_buffer: *mut libc::c_char,
        name: *mut cstring,
        dum2: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_file(
        ret_buffer: *mut libc::c_char,
        file: *mut cstring,
        attr: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_debug(
        ret_buffer: *mut libc::c_char,
        arg1: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_wait(
        ret_buffer: *mut libc::c_char,
        arg1: *mut cstring,
        arg2: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_directory(
        ret_buffer: *mut libc::c_char,
        file: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_errmsg(
        ret_buffer: *mut libc::c_char,
        err: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_opcom(
        ret_buffer: *mut libc::c_char,
        msg: *mut cstring,
        device: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_signal(
        ret_buffer: *mut libc::c_char,
        pid: *mut cstring,
        sig: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_spawn(
        ret_buffer: *mut libc::c_char,
        cmd: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_version(
        ret_buffer: *mut libc::c_char,
        name: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_zwrite(
        ret_buffer: *mut libc::c_char,
        tmp: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_e(
        ret_buffer: *mut libc::c_char,
        istr: *mut cstring,
        STR_mask: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_paschk(
        ret_buffer: *mut libc::c_char,
        user: *mut cstring,
        pwd: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_v(
        ret_buffer: *mut libc::c_char,
        lin: *mut cstring,
        col: *mut cstring,
    ) -> libc::c_int;
    fn Xcall_x(
        ret_buffer: *mut libc::c_char,
        str: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_int;
    fn Xcall_xrsm(
        ret_buffer: *mut libc::c_char,
        str: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_getenv(
        ret_buffer: *mut libc::c_char,
        env: *mut cstring,
        dummy: *mut cstring,
    ) -> libc::c_int;
    fn Xcall_setenv(
        ret_buffer: *mut libc::c_char,
        env: *mut cstring,
        value: *mut cstring,
    ) -> libc::c_short;
    fn Xcall_fork(
        ret_buffer: *mut libc::c_char,
        dum1: *mut cstring,
        dum2: *mut cstring,
    ) -> libc::c_short;
    static mut source_ptr: *mut u_char;
    static mut comp_ptr: *mut u_char;
    static mut indstk: [u_char; 0];
    static mut isp: libc::c_long;
    fn parse_close();
    fn parse_do(runtime: libc::c_int);
    fn parse_goto(runtime: libc::c_int);
    fn parse_hang();
    fn parse_if(i: libc::c_long);
    fn parse_job(runtime: libc::c_int);
    fn parse_kill();
    fn parse_lock();
    fn parse_merge();
    fn parse_new();
    fn parse_open();
    fn parse_read();
    fn parse_set();
    fn parse_use();
    fn parse_write();
    fn parse_xecute();
    fn localvar() -> libc::c_short;
    fn Debug_off();
    fn Debug_on(param: *mut cstring) -> libc::c_short;
    fn Debug(
        savasp: libc::c_int,
        savssp: libc::c_int,
        dot: libc::c_int,
    ) -> libc::c_short;
    static mut symtab: [symtab_struct; 0];
    fn ST_Create(var: var_u) -> libc::c_short;
    fn ST_Restore(newtab: *mut ST_newtab);
    static mut in_hist: libc::c_short;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type off_t = __darwin_off_t;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type time_t = __darwin_time_t;
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
pub type for_stack = FOR_STACK;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct FOR_STACK {
    pub type_0: libc::c_short,
    pub svar: libc::c_short,
    pub var: *mut mvar,
    pub nxtarg: *mut u_char,
    pub startpc: *mut u_char,
    pub quit: *mut u_char,
    pub increment: *mut u_char,
    pub done: *mut u_char,
}
pub type ST_data = ST_DATA;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_DATA {
    pub deplnk: *mut ST_depend,
    pub attach: libc::c_short,
    pub dbc: u_short,
    pub data: [u_char; 65535],
}
pub type ST_depend = ST_DEPEND;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_DEPEND {
    pub deplnk: *mut ST_DEPEND,
    pub keylen: u_char,
    pub bytes: [u_char; 65794],
}
pub type symtab_struct = SYMTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SYMTAB {
    pub fwd_link: libc::c_short,
    pub usage: libc::c_short,
    pub data: *mut ST_DATA,
    pub varnam: var_u,
}
pub type rbd = RBD;
pub type ST_newtab = ST_NEWTAB;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_NEWTAB {
    pub fwd_link: *mut ST_NEWTAB,
    pub count_enn: libc::c_short,
    pub stindex: *mut libc::c_short,
    pub count_new: libc::c_short,
    pub locdata: *mut ST_locdata,
}
pub type ST_locdata = ST_LOCDATA;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ST_LOCDATA {
    pub stindex: libc::c_short,
    pub data: *mut ST_data,
}
pub type tags = TAGS;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct TAGS {
    pub name: var_u,
    pub code: u_short,
}
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
pub unsafe extern "C" fn isalnum(mut _c: libc::c_int) -> libc::c_int {
    return __istype(
        _c,
        (0x100 as libc::c_long | 0x400 as libc::c_long) as libc::c_ulong,
    );
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
pub unsafe extern "C" fn toupper(mut _c: libc::c_int) -> libc::c_int {
    return __toupper(_c);
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
#[inline]
unsafe extern "C" fn var_empty(mut var: var_u) -> u_int {
    if var.var_q == 0 as libc::c_int as u_int64 {
        return 1 as libc::c_int as u_int
    } else {
        return 0 as libc::c_int as u_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn run(
    mut savasp: libc::c_int,
    mut savssp: libc::c_int,
) -> libc::c_short {
    let mut opc: libc::c_int = 0;
    let mut infor: libc::c_int = 0 as libc::c_int;
    let mut offset: libc::c_int = 0;
    let mut s: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut us: u_short = 0 as libc::c_int as u_short;
    let mut t: libc::c_int = 0 as libc::c_int;
    let mut hist: libc::c_short = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut args: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut cptr: *mut cstring = 0 as *mut cstring;
    let mut ptr1: *mut cstring = 0 as *mut cstring;
    let mut ptr2: *mut cstring = 0 as *mut cstring;
    let mut tmp: *mut cstring = 0 as *mut cstring;
    let mut var: *mut mvar = 0 as *mut mvar;
    let mut var2: *mut mvar = 0 as *mut mvar;
    let mut p: *mut u_char = 0 as *mut u_char;
    let mut var_undefined: u_short = (65534 as libc::c_int + 1 as libc::c_int)
        as u_short;
    let mut rou: var_u = VAR_U { var_q: 0 };
    let mut tag: var_u = VAR_U { var_q: 0 };
    let mut list: *mut var_u = 0 as *mut var_u;
    let mut ttbl: *mut tags = 0 as *mut tags;
    let mut rouadd: *mut rbd = 0 as *mut rbd;
    let mut curframe: *mut do_frame = 0 as *mut do_frame;
    let mut asp: libc::c_int = 0;
    let mut ssp: libc::c_int = 0;
    let mut temp: [u_char; 256] = [0; 256];
    let mut forx: *mut for_stack = 0 as *mut for_stack;
    let mut vt: *mut var_u = 0 as *mut var_u;
    let mut data: *mut ST_data = 0 as *mut ST_data;
    asp = savasp;
    ssp = savssp;
    loop {
        if ssp >= 1048576 as libc::c_int * 2 as libc::c_int {
            panic(
                b"String Stack overflow in runtime!!\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if (*partab.jobtab).attention != 0 {
            s = attention();
            if s as libc::c_int == 256 as libc::c_int {
                if Debug(asp, ssp, -(1 as libc::c_int)) as libc::c_int
                    == 1 as libc::c_int
                {
                    return 1 as libc::c_int as libc::c_short;
                }
                continue;
            } else {
                if s as libc::c_int > 0 as libc::c_int {
                    return s;
                }
                if (s as libc::c_int) < 0 as libc::c_int
                    || (*partab.jobtab).error_frame as libc::c_int
                        > (*partab.jobtab).cur_do
                {
                    savasp = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].asp
                        as libc::c_int;
                    savssp = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].ssp
                        as libc::c_int;
                    ssp = savssp;
                    asp = savasp;
                    infor = 0 as libc::c_int;
                    if (*partab.jobtab).error_frame == 0 {
                        (*partab.jobtab)
                            .error_frame = (*partab.jobtab).cur_do as libc::c_short;
                        memcpy(
                            &mut *((*partab.jobtab).dostk)
                                .as_mut_ptr()
                                .offset((128 as libc::c_int - 1 as libc::c_int) as isize)
                                as *mut do_frame as *mut libc::c_void,
                            &mut *((*partab.jobtab).dostk)
                                .as_mut_ptr()
                                .offset((*partab.jobtab).cur_do as isize) as *mut do_frame
                                as *const libc::c_void,
                            ::core::mem::size_of::<do_frame>() as libc::c_ulong,
                        );
                    }
                    if s as libc::c_int == -(51 as libc::c_int + 200 as libc::c_int) {
                        (*partab.jobtab).io = 0 as libc::c_int as u_char;
                    }
                    (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].pc = rsmpc;
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    var = &mut partab.src_var;
                    (*var).volset = 0 as libc::c_int as u_char;
                    (*var).uci = 255 as libc::c_int as u_char;
                    flag = 0 as libc::c_int;
                    if s != 0 {
                        flag = Set_Error(s as libc::c_int, ptr1, cptr);
                    }
                    (*cptr).len = 0 as libc::c_int as u_short;
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
                        b"$ETRAP\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        6 as libc::c_int as libc::c_ulong,
                    );
                    (*var).slen = 0 as libc::c_int as u_char;
                    t = ST_Get(var, ((*cptr).buf).as_mut_ptr());
                    if t < 0 as libc::c_int {
                        t = 0 as libc::c_int;
                    }
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            ((*cptr).len as libc::c_ulong)
                                .wrapping_add(
                                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    p = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char;
                    comp_ptr = p;
                    if (*cptr).len as libc::c_int != 0 && flag == 0 {
                        source_ptr = ((*cptr).buf).as_mut_ptr();
                        parse();
                        (*partab.jobtab)
                            .etrap_at = (*partab.jobtab).cur_do as libc::c_short;
                    }
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .endlin = comp_ptr;
                    if (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].type_0
                        as libc::c_int == 4 as libc::c_int
                    {
                        let fresh0 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh0 = 60 as libc::c_int as u_char;
                        let fresh1 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh1 = 0 as libc::c_int as u_char;
                        let fresh2 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh2 = 0 as libc::c_int as u_char;
                        let fresh3 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh3 = '\0' as i32 as u_char;
                        let fresh4 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh4 = 158 as libc::c_int as u_char;
                    } else if (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .type_0 as libc::c_int != 1 as libc::c_int
                    {
                        let fresh5 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh5 = 157 as libc::c_int as u_char;
                    }
                    let fresh6 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh6 = 0 as libc::c_int as u_char;
                    let fresh7 = comp_ptr;
                    comp_ptr = comp_ptr.offset(1);
                    *fresh7 = 0 as libc::c_int as u_char;
                    i = comp_ptr.offset_from(p) as libc::c_long as libc::c_int;
                    ssp += i;
                    rsmpc = p;
                    savasp = asp;
                    savssp = ssp;
                }
            }
        }
        let fresh8 = rsmpc;
        rsmpc = rsmpc.offset(1);
        opc = *fresh8 as libc::c_int;
        let mut current_block_2879: u64;
        match opc {
            0 => {
                if *rsmpc as libc::c_int == 0 as libc::c_int {
                    return 0 as libc::c_int as libc::c_short;
                }
                current_block_2879 = 16169013146210219323;
            }
            1 => {
                CleanJob(0 as libc::c_int);
                return 1 as libc::c_int as libc::c_short;
            }
            2 => {
                memcpy(
                    &mut s as *mut libc::c_short as *mut libc::c_void,
                    rsmpc as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                );
                rsmpc = rsmpc
                    .offset(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                    );
                (*partab.jobtab).async_error = s;
                (*partab.jobtab).attention = 1 as libc::c_int;
                current_block_2879 = 16169013146210219323;
            }
            3 => {
                asp -= 1;
                i = cstringtob(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                if i == 0 as libc::c_int {
                    (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                }
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh9 = asp;
                asp = asp + 1;
                let ref mut fresh10 = *addstk.as_mut_ptr().offset(fresh9 as isize);
                *fresh10 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            4 => {
                asp = savasp;
                ssp = savssp;
                isp = (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].isp;
                current_block_2879 = 16169013146210219323;
            }
            5 => {
                memcpy(
                    &mut s as *mut libc::c_short as *mut libc::c_void,
                    rsmpc as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                );
                rsmpc = rsmpc
                    .offset(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                    );
                asp -= 1;
                if cstringtob(*addstk.as_mut_ptr().offset(asp as isize) as *mut cstring)
                    == 0 as libc::c_int
                {
                    rsmpc = rsmpc.offset(s as libc::c_int as isize);
                }
                current_block_2879 = 16169013146210219323;
            }
            6 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                if (*partab.jobtab).test as libc::c_int == 0 as libc::c_int {
                    if infor != 0 {
                        i = 1 as libc::c_int;
                        if (*(*addstk
                            .as_mut_ptr()
                            .offset((savasp - 1 as libc::c_int) as isize)
                            as *mut for_stack))
                            .type_0 as libc::c_int & 7 as libc::c_int == 0 as libc::c_int
                        {
                            i = 3 as libc::c_int;
                            ssp = savssp;
                            asp = savasp;
                        }
                        rsmpc = ((*(*addstk
                            .as_mut_ptr()
                            .offset((savasp - 1 as libc::c_int) as isize)
                            as *mut for_stack))
                            .quit)
                            .offset(-(i as isize));
                    } else {
                        rsmpc = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .endlin;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            7 | 8 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                (*partab.jobtab).test = 1 as libc::c_int as u_char;
                asp -= 1;
                if cstringtob(*addstk.as_mut_ptr().offset(asp as isize) as *mut cstring)
                    == 0 as libc::c_int
                {
                    (*partab.jobtab).test = 0 as libc::c_int as u_char;
                    if opc == 8 as libc::c_int {
                        memcpy(
                            &mut isp as *mut libc::c_long as *mut libc::c_void,
                            rsmpc as *const libc::c_void,
                            ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
                        );
                        rsmpc = rsmpc
                            .offset(
                                ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                                    as isize,
                            );
                    }
                    if infor != 0 {
                        i = 1 as libc::c_int;
                        if (*(*addstk
                            .as_mut_ptr()
                            .offset((savasp - 1 as libc::c_int) as isize)
                            as *mut for_stack))
                            .type_0 as libc::c_int & 7 as libc::c_int == 0 as libc::c_int
                        {
                            i = 3 as libc::c_int;
                            ssp = savssp;
                            asp = savasp;
                        }
                        rsmpc = ((*(*addstk
                            .as_mut_ptr()
                            .offset((savasp - 1 as libc::c_int) as isize)
                            as *mut for_stack))
                            .quit)
                            .offset(-(i as isize));
                    } else {
                        rsmpc = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .endlin;
                    }
                } else if opc == 8 as libc::c_int {
                    rsmpc = rsmpc
                        .offset(
                            ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                                as isize,
                        );
                }
                current_block_2879 = 16169013146210219323;
            }
            9 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                if (*partab.jobtab).test as libc::c_int != 0 as libc::c_int {
                    if infor != 0 {
                        i = 1 as libc::c_int;
                        if (*(*addstk
                            .as_mut_ptr()
                            .offset((savasp - 1 as libc::c_int) as isize)
                            as *mut for_stack))
                            .type_0 as libc::c_int & 7 as libc::c_int == 0 as libc::c_int
                        {
                            i = 3 as libc::c_int;
                            ssp = savssp;
                            asp = savasp;
                        }
                        rsmpc = ((*(*addstk
                            .as_mut_ptr()
                            .offset((savasp - 1 as libc::c_int) as isize)
                            as *mut for_stack))
                            .quit)
                            .offset(-(i as isize));
                    } else {
                        rsmpc = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .endlin;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            10 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr2).buf).as_mut_ptr();
                s = ncopy(&mut p, temp.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        s = runtime_add(
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                            temp.as_mut_ptr() as *mut libc::c_char,
                        );
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            (*cptr).len = s as u_short;
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    (s as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                        )
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                            let fresh11 = asp;
                            asp = asp + 1;
                            let ref mut fresh12 = *addstk
                                .as_mut_ptr()
                                .offset(fresh11 as isize);
                            *fresh12 = cptr as *mut u_char;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            11 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr2).buf).as_mut_ptr();
                s = ncopy(
                    &mut p,
                    &mut *temp.as_mut_ptr().offset(1 as libc::c_int as isize),
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        temp[0 as libc::c_int as usize] = '-' as i32 as u_char;
                        s = 0 as libc::c_int as libc::c_short;
                        if temp[1 as libc::c_int as usize] as libc::c_int == '0' as i32
                            && temp[2 as libc::c_int as usize] as libc::c_int
                                == '\0' as i32
                        {
                            s = 1 as libc::c_int as libc::c_short;
                        } else if temp[1 as libc::c_int as usize] as libc::c_int
                            == '-' as i32
                        {
                            s = 2 as libc::c_int as libc::c_short;
                        }
                        s = runtime_add(
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                            &mut *temp.as_mut_ptr().offset(s as isize) as *mut u_char
                                as *mut libc::c_char,
                        );
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            (*cptr).len = s as u_short;
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    (s as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                        )
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                            let fresh13 = asp;
                            asp = asp + 1;
                            let ref mut fresh14 = *addstk
                                .as_mut_ptr()
                                .offset(fresh13 as isize);
                            *fresh14 = cptr as *mut u_char;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            12 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr2).buf).as_mut_ptr();
                s = ncopy(&mut p, temp.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        s = runtime_mul(
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                            temp.as_mut_ptr() as *mut libc::c_char,
                        );
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            (*cptr).len = s as u_short;
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    (s as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                        )
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                            let fresh15 = asp;
                            asp = asp + 1;
                            let ref mut fresh16 = *addstk
                                .as_mut_ptr()
                                .offset(fresh15 as isize);
                            *fresh16 = cptr as *mut u_char;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            13 => {
                current_block_2879 = 16624893707949258064;
            }
            14 | 15 => {
                current_block_2879 = 16624893707949258064;
            }
            16 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr2).buf).as_mut_ptr();
                s = ncopy(&mut p, temp.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        s = runtime_power(
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                            temp.as_mut_ptr() as *mut libc::c_char,
                        );
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            (*cptr).len = s as u_short;
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    (s as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                        )
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                            let fresh19 = asp;
                            asp = asp + 1;
                            let ref mut fresh20 = *addstk
                                .as_mut_ptr()
                                .offset(fresh19 as isize);
                            *fresh20 = cptr as *mut u_char;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            17 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                if (*ptr1).len as libc::c_int + (*ptr2).len as libc::c_int
                    > 65534 as libc::c_int
                {
                    (*partab.jobtab).async_error = -(75 as libc::c_int) as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    if ptr1 as *mut u_char >= partab.strstk_start
                        && (ptr1 as *mut u_char) < partab.strstk_last
                        && (rsmpc < partab.strstk_start || rsmpc > partab.strstk_last)
                    {
                        cptr = ptr1;
                        current_block_2879 = 1254794021369287194;
                    } else {
                        cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut cstring;
                        t = mcopy(
                            ((*ptr1).buf).as_mut_ptr(),
                            ((*cptr).buf).as_mut_ptr(),
                            (*ptr1).len as libc::c_int,
                        );
                        if t < 0 as libc::c_int {
                            (*partab.jobtab).async_error = t as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else {
                            (*cptr).len = t as u_short;
                            current_block_2879 = 1254794021369287194;
                        }
                    }
                    match current_block_2879 {
                        16169013146210219323 => {}
                        _ => {
                            t = mcopy(
                                ((*ptr2).buf).as_mut_ptr(),
                                &mut *((*cptr).buf)
                                    .as_mut_ptr()
                                    .offset((*cptr).len as isize),
                                (*ptr2).len as libc::c_int,
                            );
                            if t < 0 as libc::c_int {
                                (*partab.jobtab).async_error = t as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                            } else {
                                (*cptr).len = ((*cptr).len as libc::c_int + t) as u_short;
                                ssp = (ssp as libc::c_ulong)
                                    .wrapping_add(
                                        ((*cptr).len as libc::c_ulong)
                                            .wrapping_add(
                                                ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                            )
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as libc::c_int as libc::c_int;
                                let fresh21 = asp;
                                asp = asp + 1;
                                let ref mut fresh22 = *addstk
                                    .as_mut_ptr()
                                    .offset(fresh21 as isize);
                                *fresh22 = cptr as *mut u_char;
                            }
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            18 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr1).buf).as_mut_ptr();
                s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (s as libc::c_ulong)
                                .wrapping_add(
                                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh23 = asp;
                    asp = asp + 1;
                    let ref mut fresh24 = *addstk.as_mut_ptr().offset(fresh23 as isize);
                    *fresh24 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            19 => {
                cptr = &mut *strstk
                    .as_mut_ptr()
                    .offset((ssp + 1 as libc::c_int) as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr1).buf).as_mut_ptr();
                s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                if s as libc::c_int > 1 as libc::c_int
                    || (*cptr).buf[0 as libc::c_int as usize] as libc::c_int
                        != '0' as i32
                {
                    if (*cptr).buf[0 as libc::c_int as usize] as libc::c_int
                        == '-' as i32
                    {
                        s -= 1;
                        s;
                        cptr = (cptr as *mut u_char).offset(1 as libc::c_int as isize)
                            as *mut cstring;
                    } else {
                        s += 1;
                        s;
                        cptr = (cptr as *mut u_char).offset(-(1 as libc::c_int as isize))
                            as *mut cstring;
                        (*cptr).buf[0 as libc::c_int as usize] = '-' as i32 as u_char;
                    }
                }
                (*cptr).len = s as u_short;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (s as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<u_short>() as libc::c_ulong,
                            )
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh25 = asp;
                asp = asp + 1;
                let ref mut fresh26 = *addstk.as_mut_ptr().offset(fresh25 as isize);
                *fresh26 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            20 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                if (*ptr1).len as libc::c_int == (*ptr2).len as libc::c_int {
                    if memcmp(
                        ((*ptr1).buf).as_mut_ptr() as *const libc::c_void,
                        ((*ptr2).buf).as_mut_ptr() as *const libc::c_void,
                        (*ptr1).len as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                    }
                }
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh27 = asp;
                asp = asp + 1;
                let ref mut fresh28 = *addstk.as_mut_ptr().offset(fresh27 as isize);
                *fresh28 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            21 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr2).buf).as_mut_ptr();
                s = ncopy(&mut p, temp.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = 1 as libc::c_int as u_short;
                        (*cptr)
                            .buf[0 as libc::c_int
                            as usize] = (if runtime_comp(
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                            temp.as_mut_ptr() as *mut libc::c_char,
                        ) as libc::c_int != 0
                        {
                            '1' as i32
                        } else {
                            '0' as i32
                        }) as u_char;
                        (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh29 = asp;
                        asp = asp + 1;
                        let ref mut fresh30 = *addstk
                            .as_mut_ptr()
                            .offset(fresh29 as isize);
                        *fresh30 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            22 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr2).buf).as_mut_ptr();
                s = ncopy(&mut p, temp.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = 1 as libc::c_int as u_short;
                        (*cptr)
                            .buf[0 as libc::c_int
                            as usize] = (if runtime_comp(
                            temp.as_mut_ptr() as *mut libc::c_char,
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                        ) as libc::c_int != 0
                        {
                            '1' as i32
                        } else {
                            '0' as i32
                        }) as u_char;
                        (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh31 = asp;
                        asp = asp + 1;
                        let ref mut fresh32 = *addstk
                            .as_mut_ptr()
                            .offset(fresh31 as isize);
                        *fresh32 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            23 => {
                asp -= 1;
                i = cstringtob(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                j = cstringtob(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                if i != 0 as libc::c_int && j != 0 as libc::c_int {
                    (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                }
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh33 = asp;
                asp = asp + 1;
                let ref mut fresh34 = *addstk.as_mut_ptr().offset(fresh33 as isize);
                *fresh34 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            24 => {
                asp -= 1;
                i = cstringtob(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                j = cstringtob(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                if i != 0 as libc::c_int || j != 0 as libc::c_int {
                    (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                }
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh35 = asp;
                asp = asp + 1;
                let ref mut fresh36 = *addstk.as_mut_ptr().offset(fresh35 as isize);
                *fresh36 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            25 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                t = Dfind3x(ptr1, ptr2, 1 as libc::c_int);
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr)
                    .buf[0 as libc::c_int
                    as usize] = (if t == 0 as libc::c_int {
                    '0' as i32
                } else {
                    '1' as i32
                }) as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh37 = asp;
                asp = asp + 1;
                let ref mut fresh38 = *addstk.as_mut_ptr().offset(fresh37 as isize);
                *fresh38 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            26 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                t = (*ptr1).len as libc::c_int;
                if t > (*ptr2).len as libc::c_int {
                    t = (*ptr2).len as libc::c_int;
                }
                i = memcmp(
                    ((*ptr1).buf).as_mut_ptr() as *const libc::c_void,
                    ((*ptr2).buf).as_mut_ptr() as *const libc::c_void,
                    t as libc::c_ulong,
                );
                if i > 0 as libc::c_int {
                    (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                }
                if i == 0 as libc::c_int
                    && ((*ptr2).len as libc::c_int) < (*ptr1).len as libc::c_int
                {
                    (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                }
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh39 = asp;
                asp = asp + 1;
                let ref mut fresh40 = *addstk.as_mut_ptr().offset(fresh39 as isize);
                *fresh40 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            27 => {
                ptr2 = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                s = UTIL_Key_Build(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ((*ptr2).buf).as_mut_ptr(),
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*ptr2).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (s as libc::c_ulong)
                                .wrapping_add(
                                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    ptr1 = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    asp -= 1;
                    s = UTIL_Key_Build(
                        *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                        ((*ptr1).buf).as_mut_ptr(),
                    );
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*ptr1).len = s as u_short;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (s as libc::c_ulong)
                                    .wrapping_add(
                                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                    )
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut cstring;
                        (*cptr).len = 1 as libc::c_int as u_short;
                        (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                        (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                        t = (*ptr1).len as libc::c_int;
                        if t < (*ptr2).len as libc::c_int {
                            t = (*ptr2).len as libc::c_int;
                        }
                        i = memcmp(
                            ((*ptr1).buf).as_mut_ptr() as *const libc::c_void,
                            ((*ptr2).buf).as_mut_ptr() as *const libc::c_void,
                            t as libc::c_ulong,
                        );
                        if i <= 0 as libc::c_int {
                            (*cptr)
                                .buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                        }
                        if i == 0 as libc::c_int
                            && ((*ptr2).buf).as_mut_ptr() > ((*ptr1).buf).as_mut_ptr()
                        {
                            (*cptr)
                                .buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                        }
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh41 = asp;
                        asp = asp + 1;
                        let ref mut fresh42 = *addstk
                            .as_mut_ptr()
                            .offset(fresh41 as isize);
                        *fresh42 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            28 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                s = patmat(ptr1, ptr2);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = 1 as libc::c_int as u_short;
                    (*cptr)
                        .buf[0 as libc::c_int
                        as usize] = ('0' as i32 + s as libc::c_int) as u_char;
                    (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add(2 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh43 = asp;
                    asp = asp + 1;
                    let ref mut fresh44 = *addstk.as_mut_ptr().offset(fresh43 as isize);
                    *fresh44 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            29 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                if i < 1 as libc::c_int {
                    SchedYield();
                } else {
                    i = sleep(i as libc::c_uint) as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            30 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                if (*ptr1).len as libc::c_int == (*ptr2).len as libc::c_int {
                    if memcmp(
                        ((*ptr1).buf).as_mut_ptr() as *const libc::c_void,
                        ((*ptr2).buf).as_mut_ptr() as *const libc::c_void,
                        (*ptr1).len as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                    }
                }
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh45 = asp;
                asp = asp + 1;
                let ref mut fresh46 = *addstk.as_mut_ptr().offset(fresh45 as isize);
                *fresh46 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            31 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr2).buf).as_mut_ptr();
                s = ncopy(&mut p, temp.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = 1 as libc::c_int as u_short;
                        (*cptr)
                            .buf[0 as libc::c_int
                            as usize] = (if runtime_comp(
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                            temp.as_mut_ptr() as *mut libc::c_char,
                        ) == 0
                        {
                            '1' as i32
                        } else {
                            '0' as i32
                        }) as u_char;
                        (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh47 = asp;
                        asp = asp + 1;
                        let ref mut fresh48 = *addstk
                            .as_mut_ptr()
                            .offset(fresh47 as isize);
                        *fresh48 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            32 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr2).buf).as_mut_ptr();
                s = ncopy(&mut p, temp.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = 1 as libc::c_int as u_short;
                        (*cptr)
                            .buf[0 as libc::c_int
                            as usize] = (if runtime_comp(
                            temp.as_mut_ptr() as *mut libc::c_char,
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                        ) == 0
                        {
                            '1' as i32
                        } else {
                            '0' as i32
                        }) as u_char;
                        (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh49 = asp;
                        asp = asp + 1;
                        let ref mut fresh50 = *addstk
                            .as_mut_ptr()
                            .offset(fresh49 as isize);
                        *fresh50 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            33 => {
                asp -= 1;
                i = cstringtob(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                j = cstringtob(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                if i != 0 as libc::c_int && j != 0 as libc::c_int {
                    (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                }
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh51 = asp;
                asp = asp + 1;
                let ref mut fresh52 = *addstk.as_mut_ptr().offset(fresh51 as isize);
                *fresh52 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            34 => {
                asp -= 1;
                i = cstringtob(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                j = cstringtob(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                if i != 0 as libc::c_int || j != 0 as libc::c_int {
                    (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                }
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh53 = asp;
                asp = asp + 1;
                let ref mut fresh54 = *addstk.as_mut_ptr().offset(fresh53 as isize);
                *fresh54 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            35 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                i = Dfind3x(ptr1, ptr2, 1 as libc::c_int);
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr)
                    .buf[0 as libc::c_int
                    as usize] = (if i == 0 as libc::c_int {
                    '1' as i32
                } else {
                    '0' as i32
                }) as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh55 = asp;
                asp = asp + 1;
                let ref mut fresh56 = *addstk.as_mut_ptr().offset(fresh55 as isize);
                *fresh56 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            36 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                t = (*ptr1).len as libc::c_int;
                if t > (*ptr2).len as libc::c_int {
                    t = (*ptr2).len as libc::c_int;
                }
                i = memcmp(
                    ((*ptr1).buf).as_mut_ptr() as *const libc::c_void,
                    ((*ptr2).buf).as_mut_ptr() as *const libc::c_void,
                    t as libc::c_ulong,
                );
                if i > 0 as libc::c_int {
                    (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                }
                if i == 0 as libc::c_int
                    && ((*ptr2).len as libc::c_int) < (*ptr1).len as libc::c_int
                {
                    (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                }
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh57 = asp;
                asp = asp + 1;
                let ref mut fresh58 = *addstk.as_mut_ptr().offset(fresh57 as isize);
                *fresh58 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            37 => {
                ptr2 = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                s = UTIL_Key_Build(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ((*ptr2).buf).as_mut_ptr(),
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*ptr2).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (s as libc::c_ulong)
                                .wrapping_add(
                                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    ptr1 = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    asp -= 1;
                    s = UTIL_Key_Build(
                        *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                        ((*ptr1).buf).as_mut_ptr(),
                    );
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*ptr1).len = s as u_short;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (s as libc::c_ulong)
                                    .wrapping_add(
                                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                    )
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut cstring;
                        (*cptr).len = 1 as libc::c_int as u_short;
                        (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                        (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                        t = (*ptr1).len as libc::c_int;
                        if t < (*ptr2).len as libc::c_int {
                            t = (*ptr2).len as libc::c_int;
                        }
                        i = memcmp(
                            ((*ptr1).buf).as_mut_ptr() as *const libc::c_void,
                            ((*ptr2).buf).as_mut_ptr() as *const libc::c_void,
                            t as libc::c_ulong,
                        );
                        if i <= 0 as libc::c_int {
                            (*cptr)
                                .buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                        }
                        if i == 0 as libc::c_int
                            && ((*ptr2).buf).as_mut_ptr() > ((*ptr1).buf).as_mut_ptr()
                        {
                            (*cptr)
                                .buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                        }
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh59 = asp;
                        asp = asp + 1;
                        let ref mut fresh60 = *addstk
                            .as_mut_ptr()
                            .offset(fresh59 as isize);
                        *fresh60 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            38 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                s = patmat(ptr1, ptr2);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = 1 as libc::c_int as u_short;
                    (*cptr)
                        .buf[0 as libc::c_int
                        as usize] = ('0' as i32 + (s == 0) as libc::c_int) as u_char;
                    (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add(2 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh61 = asp;
                    asp = asp + 1;
                    let ref mut fresh62 = *addstk.as_mut_ptr().offset(fresh61 as isize);
                    *fresh62 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            41 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                ptr1 = *addstk.as_mut_ptr().offset((asp - 1 as libc::c_int) as isize)
                    as *mut cstring;
                if (*var).uci as libc::c_int == 255 as libc::c_int {
                    if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int
                        == '$' as i32
                    {
                        t = Vset(var, ptr1);
                    } else {
                        t = ST_Set(var, ptr1);
                    }
                } else if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int
                    == '$' as i32
                {
                    t = SS_Set(var, ptr1) as libc::c_int;
                } else {
                    memcpy(
                        &mut (*partab.jobtab).last_ref as *mut mvar as *mut libc::c_void,
                        var as *const libc::c_void,
                        (::core::mem::size_of::<var_u>() as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong)
                            .wrapping_add((*var).slen as libc::c_ulong),
                    );
                    t = DB_Set(var, ptr1);
                }
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            42 | 43 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                p = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char;
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                if i > 65534 as libc::c_int + 1 as libc::c_int
                    || j > 65534 as libc::c_int + 1 as libc::c_int
                {
                    (*partab.jobtab).async_error = -(75 as libc::c_int) as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    ptr1 = 0 as *mut cstring;
                    if opc == 43 as libc::c_int {
                        asp -= 1;
                        ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                        if (*ptr1).len as libc::c_int * i
                            > 65534 as libc::c_int + 1 as libc::c_int
                            || (*ptr1).len as libc::c_int * j
                                > 65534 as libc::c_int + 1 as libc::c_int
                        {
                            (*partab.jobtab)
                                .async_error = -(75 as libc::c_int) as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else {
                            current_block_2879 = 2280147619541540655;
                        }
                    } else {
                        current_block_2879 = 2280147619541540655;
                    }
                    match current_block_2879 {
                        16169013146210219323 => {}
                        _ => {
                            asp -= 1;
                            var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                            cptr = *addstk
                                .as_mut_ptr()
                                .offset((asp - 1 as libc::c_int) as isize) as *mut cstring;
                            if (*var).name.var_cu[0 as libc::c_int as usize]
                                as libc::c_int == '$' as i32
                            {
                                (*partab.jobtab)
                                    .async_error = -(8 as libc::c_int) as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                            } else {
                                if opc == 43 as libc::c_int {
                                    t = DSetpiece(p, cptr, var, ptr1, i, j);
                                } else {
                                    t = DSetextract(p, cptr, var, i, j);
                                }
                                if t < 0 as libc::c_int {
                                    (*partab.jobtab).async_error = t as libc::c_short;
                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                }
                            }
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            44 => {
                var = *addstk.as_mut_ptr().offset((asp - 1 as libc::c_int) as isize)
                    as *mut mvar;
                if (*var).uci as libc::c_int != 255 as libc::c_int
                    && (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int
                        != '$' as i32
                {
                    memcpy(
                        &mut (*partab.jobtab).last_ref as *mut mvar as *mut libc::c_void,
                        var as *const libc::c_void,
                        (::core::mem::size_of::<var_u>() as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong)
                            .wrapping_add((*var).slen as libc::c_ulong),
                    );
                }
                current_block_2879 = 16169013146210219323;
            }
            45 => {
                SQ_Flush();
                current_block_2879 = 16169013146210219323;
            }
            46 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                hist = in_hist;
                in_hist = -(1 as libc::c_int) as libc::c_short;
                s = SQ_ReadStar(&mut i, -(1 as libc::c_int));
                in_hist = hist;
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    (*cptr).len = itocstring(((*cptr).buf).as_mut_ptr(), i);
                    if (*var).uci as libc::c_int == 255 as libc::c_int {
                        t = ST_Set(var, cptr);
                    } else if (*var).name.var_cu[0 as libc::c_int as usize]
                        as libc::c_int == '$' as i32
                    {
                        t = SS_Set(var, cptr) as libc::c_int;
                    } else {
                        memcpy(
                            &mut (*partab.jobtab).last_ref as *mut mvar
                                as *mut libc::c_void,
                            var as *const libc::c_void,
                            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                .wrapping_add((*var).slen as libc::c_ulong),
                        );
                        t = DB_Set(var, cptr);
                    }
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            47 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                if j < 0 as libc::c_int {
                    j = 0 as libc::c_int;
                }
                hist = in_hist;
                in_hist = -(1 as libc::c_int) as libc::c_short;
                s = SQ_ReadStar(&mut i, j);
                in_hist = hist;
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    (*cptr).len = itocstring(((*cptr).buf).as_mut_ptr(), i);
                    if (*var).uci as libc::c_int == 255 as libc::c_int {
                        t = ST_Set(var, cptr);
                    } else if (*var).name.var_cu[0 as libc::c_int as usize]
                        as libc::c_int == '$' as i32
                    {
                        t = SS_Set(var, cptr) as libc::c_int;
                    } else {
                        memcpy(
                            &mut (*partab.jobtab).last_ref as *mut mvar
                                as *mut libc::c_void,
                            var as *const libc::c_void,
                            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                .wrapping_add((*var).slen as libc::c_ulong),
                        );
                        t = DB_Set(var, cptr);
                    }
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            48 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                hist = in_hist;
                in_hist = -(1 as libc::c_int) as libc::c_short;
                t = SQ_Read(
                    ((*cptr).buf).as_mut_ptr(),
                    -(1 as libc::c_int),
                    -(1 as libc::c_int),
                );
                in_hist = hist;
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    if (*var).uci as libc::c_int == 255 as libc::c_int {
                        t = ST_Set(var, cptr);
                    } else if (*var).name.var_cu[0 as libc::c_int as usize]
                        as libc::c_int == '$' as i32
                    {
                        t = SS_Set(var, cptr) as libc::c_int;
                    } else {
                        memcpy(
                            &mut (*partab.jobtab).last_ref as *mut mvar
                                as *mut libc::c_void,
                            var as *const libc::c_void,
                            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                .wrapping_add((*var).slen as libc::c_ulong),
                        );
                        t = DB_Set(var, cptr);
                    }
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            49 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                if j < 0 as libc::c_int {
                    j = 0 as libc::c_int;
                }
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                hist = in_hist;
                in_hist = -(1 as libc::c_int) as libc::c_short;
                t = SQ_Read(((*cptr).buf).as_mut_ptr(), j, -(1 as libc::c_int));
                in_hist = hist;
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    if (*var).uci as libc::c_int == 255 as libc::c_int {
                        t = ST_Set(var, cptr);
                    } else if (*var).name.var_cu[0 as libc::c_int as usize]
                        as libc::c_int == '$' as i32
                    {
                        t = SS_Set(var, cptr) as libc::c_int;
                    } else {
                        memcpy(
                            &mut (*partab.jobtab).last_ref as *mut mvar
                                as *mut libc::c_void,
                            var as *const libc::c_void,
                            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                .wrapping_add((*var).slen as libc::c_ulong),
                        );
                        t = DB_Set(var, cptr);
                    }
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            50 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                hist = in_hist;
                in_hist = -(1 as libc::c_int) as libc::c_short;
                t = SQ_Read(((*cptr).buf).as_mut_ptr(), -(1 as libc::c_int), i);
                in_hist = hist;
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    if (*var).uci as libc::c_int == 255 as libc::c_int {
                        t = ST_Set(var, cptr);
                    } else if (*var).name.var_cu[0 as libc::c_int as usize]
                        as libc::c_int == '$' as i32
                    {
                        t = SS_Set(var, cptr) as libc::c_int;
                    } else {
                        memcpy(
                            &mut (*partab.jobtab).last_ref as *mut mvar
                                as *mut libc::c_void,
                            var as *const libc::c_void,
                            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                .wrapping_add((*var).slen as libc::c_ulong),
                        );
                        t = DB_Set(var, cptr);
                    }
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            51 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                hist = in_hist;
                in_hist = -(1 as libc::c_int) as libc::c_short;
                t = SQ_Read(((*cptr).buf).as_mut_ptr(), j, i);
                in_hist = hist;
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    if (*var).uci as libc::c_int == 255 as libc::c_int {
                        t = ST_Set(var, cptr);
                    } else if (*var).name.var_cu[0 as libc::c_int as usize]
                        as libc::c_int == '$' as i32
                    {
                        t = SS_Set(var, cptr) as libc::c_int;
                    } else {
                        memcpy(
                            &mut (*partab.jobtab).last_ref as *mut mvar
                                as *mut libc::c_void,
                            var as *const libc::c_void,
                            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                .wrapping_add((*var).slen as libc::c_ulong),
                        );
                        t = DB_Set(var, cptr);
                    }
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            52 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                s = SQ_WriteStar(i as u_char);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            53 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                s = SQ_WriteFormat(-(1 as libc::c_int));
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            54 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                s = SQ_WriteFormat(-(2 as libc::c_int));
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            55 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                if i < 1 as libc::c_int {
                    current_block_2879 = 16169013146210219323;
                } else {
                    s = SQ_WriteFormat(i);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    }
                    current_block_2879 = 16169013146210219323;
                }
            }
            56 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                t = SQ_Write(*addstk.as_mut_ptr().offset(asp as isize) as *mut cstring);
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            57 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                let mut var_i_0: u_int = 0 as libc::c_int as u_int;
                while var_i_0 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    rou.var_qu[var_i_0 as usize] = 0 as libc::c_int as u_int64;
                    var_i_0 = var_i_0.wrapping_add(1);
                    var_i_0;
                }
                let fresh63 = rsmpc;
                rsmpc = rsmpc.offset(1);
                args = *fresh63 as libc::c_int;
                ptr1 = 0 as *mut libc::c_void as *mut cstring;
                ptr2 = 0 as *mut libc::c_void as *mut cstring;
                i = 0 as libc::c_int;
                while args != 0 {
                    asp -= 1;
                    tmp = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                    args -= 1;
                    args;
                    if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"terminator=\0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        ptr1 = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut cstring;
                        t = mcopy(
                            &mut *((*tmp).buf)
                                .as_mut_ptr()
                                .offset(11 as libc::c_int as isize),
                            ((*ptr1).buf).as_mut_ptr(),
                            (*tmp).len as libc::c_int - 11 as libc::c_int,
                        );
                        if t < 0 as libc::c_int {
                            (*partab.jobtab).async_error = t as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            break;
                        } else {
                            (*ptr1).len = t as u_short;
                        }
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"output=\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        ptr2 = &mut *strstk
                            .as_mut_ptr()
                            .offset((ssp + 256 as libc::c_int) as isize) as *mut u_char
                            as *mut cstring;
                        t = mcopy(
                            &mut *((*tmp).buf)
                                .as_mut_ptr()
                                .offset(7 as libc::c_int as isize),
                            ((*ptr2).buf).as_mut_ptr(),
                            (*tmp).len as libc::c_int - 7 as libc::c_int,
                        );
                        if t < 0 as libc::c_int {
                            (*partab.jobtab).async_error = t as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            break;
                        } else {
                            (*ptr2).len = t as u_short;
                        }
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"escape\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 4 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"noescape\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 16 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"echo\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 1 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"noecho\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 2 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"disconnect\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 128 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"delete=none\0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 256 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"delete=back\0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 512 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"delete=delete\0" as *const u8 as *const libc::c_char,
                        13 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 1024 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"delete=both\0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 2048 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"controlc\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 4096 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"nocontrolc\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 8192 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"controlt\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 16384 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"nocontrolt\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i |= 32768 as libc::c_int;
                    } else if strncasecmp(
                        ((*tmp).buf).as_mut_ptr() as *const libc::c_char,
                        b"namespace=\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        p = &mut *((*tmp).buf)
                            .as_mut_ptr()
                            .offset(10 as libc::c_int as isize) as *mut u_char;
                        j = 0 as libc::c_int;
                        while j < 32 as libc::c_int {
                            if *p.offset(j as isize) == 0 {
                                break;
                            }
                            if j == 0
                                && *p.offset(j as isize) as libc::c_int != '%' as i32
                                && isalpha(*p.offset(j as isize) as libc::c_int) == 0
                            {
                                (*partab.jobtab)
                                    .async_error = -(36 as libc::c_int) as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                                break;
                            } else if j != 0
                                && isalnum(*p.offset(j as isize) as libc::c_int) == 0
                            {
                                (*partab.jobtab)
                                    .async_error = -(36 as libc::c_int) as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                                break;
                            } else {
                                *(&mut rou as *mut var_u as *mut u_char)
                                    .offset(j as isize) = *p.offset(j as isize);
                                j += 1;
                                j;
                            }
                        }
                    } else {
                        (*partab.jobtab)
                            .async_error = -(13 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        break;
                    }
                }
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                s = SQ_Use(j, ptr1, ptr2, i);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else if var_empty(rou) == 0 {
                    let mut var_i_1: u_int = 0 as libc::c_int as u_int;
                    while var_i_1 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        (*partab.jobtab)
                            .seqio[(*partab.jobtab).io as usize]
                            .namespace
                            .var_qu[var_i_1 as usize] = rou.var_qu[var_i_1 as usize];
                        var_i_1 = var_i_1.wrapping_add(1);
                        var_i_1;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            58 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                let mut var_i_2: u_int = 0 as libc::c_int as u_int;
                while var_i_2 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    rou.var_qu[var_i_2 as usize] = 0 as libc::c_int as u_int64;
                    var_i_2 = var_i_2.wrapping_add(1);
                    var_i_2;
                }
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                s = SQ_Open(j, ptr1, ptr2, i);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    if (*cptr).len as libc::c_int != 0
                        && strncasecmp(
                            ((*cptr).buf).as_mut_ptr() as *const libc::c_char,
                            b"namespace=\0" as *const u8 as *const libc::c_char,
                            10 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                    {
                        p = &mut *((*cptr).buf)
                            .as_mut_ptr()
                            .offset(10 as libc::c_int as isize) as *mut u_char;
                        i = 0 as libc::c_int;
                        while i < 32 as libc::c_int {
                            if *p.offset(i as isize) == 0 {
                                break;
                            }
                            if i == 0
                                && *p.offset(i as isize) as libc::c_int != '%' as i32
                                && isalpha(*p.offset(i as isize) as libc::c_int) == 0
                            {
                                (*partab.jobtab)
                                    .async_error = -(36 as libc::c_int) as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                                break;
                            } else if i != 0
                                && isalnum(*p.offset(i as isize) as libc::c_int) == 0
                            {
                                (*partab.jobtab)
                                    .async_error = -(36 as libc::c_int) as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                                break;
                            } else {
                                *(&mut rou as *mut var_u as *mut u_char)
                                    .offset(i as isize) = *p.offset(i as isize);
                                i += 1;
                                i;
                            }
                        }
                    }
                    let mut var_i_3: u_int = 0 as libc::c_int as u_int;
                    while var_i_3 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        (*partab.jobtab)
                            .seqio[j as usize]
                            .namespace
                            .var_qu[var_i_3 as usize] = rou.var_qu[var_i_3 as usize];
                        var_i_3 = var_i_3.wrapping_add(1);
                        var_i_3;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            59 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                s = SQ_Close(j);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            60 => {
                let fresh64 = asp;
                asp = asp + 1;
                let ref mut fresh65 = *addstk.as_mut_ptr().offset(fresh64 as isize);
                *fresh65 = rsmpc;
                rsmpc = rsmpc
                    .offset((*(rsmpc as *mut cstring)).len as libc::c_int as isize)
                    .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize)
                    .offset(1 as libc::c_int as isize);
                current_block_2879 = 16169013146210219323;
            }
            61 => {
                s = buildmvar(&mut partab.src_var, 0 as libc::c_int, asp);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    asp = s as libc::c_int;
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    if partab.src_var.uci as libc::c_int == 255 as libc::c_int {
                        t = ST_Get(&mut partab.src_var, ((*cptr).buf).as_mut_ptr());
                    } else if partab.src_var.name.var_cu[0 as libc::c_int as usize]
                        as libc::c_int == '$' as i32
                    {
                        t = SS_Get(&mut partab.src_var, ((*cptr).buf).as_mut_ptr());
                    } else {
                        memcpy(
                            &mut (*partab.jobtab).last_ref as *mut mvar
                                as *mut libc::c_void,
                            &mut partab.src_var as *mut mvar as *const libc::c_void,
                            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                .wrapping_add(partab.src_var.slen as libc::c_ulong),
                        );
                        t = DB_Get(&mut partab.src_var, ((*cptr).buf).as_mut_ptr());
                    }
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = t as u_short;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (t as libc::c_ulong)
                                    .wrapping_add(
                                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                    )
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh66 = asp;
                        asp = asp + 1;
                        let ref mut fresh67 = *addstk
                            .as_mut_ptr()
                            .offset(fresh66 as isize);
                        *fresh67 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            62 => {
                current_block_2879 = 587670741345194828;
            }
            63 | 64 => {
                current_block_2879 = 587670741345194828;
            }
            65 => {
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                if (((*cptr).len as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(isp as libc::c_ulong)
                    > 65536 as libc::c_int as libc::c_ulong
                {
                    (*partab.jobtab)
                        .async_error = -(58 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    source_ptr = ((*cptr).buf).as_mut_ptr();
                    comp_ptr = &mut *indstk.as_mut_ptr().offset(isp as isize)
                        as *mut u_char;
                    i = 0 as libc::c_int;
                    while *source_ptr as libc::c_int == '@' as i32 {
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                        i += 1;
                        i;
                    }
                    if i != 0 {
                        source_ptr = source_ptr.offset(-1);
                        source_ptr;
                        s = localvar();
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else {
                            loop {
                                let fresh70 = i;
                                i = i - 1;
                                if !(fresh70 != 0) {
                                    break;
                                }
                                let fresh71 = comp_ptr;
                                comp_ptr = comp_ptr.offset(1);
                                *fresh71 = 65 as libc::c_int as u_char;
                            }
                            current_block_2879 = 17652617608064773918;
                        }
                    } else {
                        eval();
                        current_block_2879 = 17652617608064773918;
                    }
                    match current_block_2879 {
                        16169013146210219323 => {}
                        _ => {
                            if *source_ptr as libc::c_int != '\0' as i32 {
                                (*partab.jobtab)
                                    .async_error = -(57 as libc::c_int + 200 as libc::c_int)
                                    as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                            } else if comp_ptr
                                .offset(
                                    (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
                                )
                                .offset(1 as libc::c_int as isize)
                                >= &mut *indstk
                                    .as_mut_ptr()
                                    .offset(65536 as libc::c_int as isize) as *mut u_char
                            {
                                (*partab.jobtab)
                                    .async_error = -(58 as libc::c_int + 200 as libc::c_int)
                                    as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                            } else {
                                let fresh72 = comp_ptr;
                                comp_ptr = comp_ptr.offset(1);
                                *fresh72 = 180 as libc::c_int as u_char;
                                memcpy(
                                    comp_ptr as *mut libc::c_void,
                                    &mut isp as *mut libc::c_long as *const libc::c_void,
                                    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
                                );
                                comp_ptr = comp_ptr
                                    .offset(
                                        ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                                            as isize,
                                    );
                                memcpy(
                                    comp_ptr as *mut libc::c_void,
                                    &mut rsmpc as *mut *mut u_char as *const libc::c_void,
                                    ::core::mem::size_of::<*mut u_char>() as libc::c_ulong,
                                );
                                comp_ptr = comp_ptr
                                    .offset(
                                        ::core::mem::size_of::<*mut u_char>() as libc::c_ulong
                                            as isize,
                                    );
                                rsmpc = &mut *indstk.as_mut_ptr().offset(isp as isize)
                                    as *mut u_char;
                                isp = comp_ptr
                                    .offset_from(
                                        &mut *indstk.as_mut_ptr().offset(isp as isize)
                                            as *mut u_char,
                                    ) as libc::c_long + isp;
                            }
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            66 => {
                current_block_2879 = 11581551973594529044;
            }
            67 | 68 => {
                current_block_2879 = 11581551973594529044;
            }
            72 => {
                let ref mut fresh77 = *addstk.as_mut_ptr().offset(asp as isize);
                *fresh77 = *addstk
                    .as_mut_ptr()
                    .offset((asp - 1 as libc::c_int) as isize);
                asp += 1;
                asp;
                current_block_2879 = 16169013146210219323;
            }
            70 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                s = Debug(savasp, savssp, 1 as libc::c_int);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else if s as libc::c_int == 1 as libc::c_int {
                    return 1 as libc::c_int as libc::c_short
                }
                current_block_2879 = 16169013146210219323;
            }
            71 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                if (*ptr1).len as libc::c_int == 0 as libc::c_int {
                    Debug_off();
                } else {
                    s = Debug_on(ptr1);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            80 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = SQ_Device(((*cptr).buf).as_mut_ptr());
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh78 = asp;
                    asp = asp + 1;
                    let ref mut fresh79 = *addstk.as_mut_ptr().offset(fresh78 as isize);
                    *fresh79 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            81 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = Vecode(((*cptr).buf).as_mut_ptr());
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh80 = asp;
                    asp = asp + 1;
                    let ref mut fresh81 = *addstk.as_mut_ptr().offset(fresh80 as isize);
                    *fresh81 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            82 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr)
                    .len = itocstring(
                    ((*cptr).buf).as_mut_ptr(),
                    (*partab.jobtab).cur_do
                        - (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].estack
                            as libc::c_int,
                );
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh82 = asp;
                asp = asp + 1;
                let ref mut fresh83 = *addstk.as_mut_ptr().offset(fresh82 as isize);
                *fresh83 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            83 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = Vetrap(((*cptr).buf).as_mut_ptr());
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh84 = asp;
                    asp = asp + 1;
                    let ref mut fresh85 = *addstk.as_mut_ptr().offset(fresh84 as isize);
                    *fresh85 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            84 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Vhorolog(((*cptr).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh86 = asp;
                    asp = asp + 1;
                    let ref mut fresh87 = *addstk.as_mut_ptr().offset(fresh86 as isize);
                    *fresh87 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            85 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr)
                    .len = uitocstring(
                    ((*cptr).buf).as_mut_ptr(),
                    (*partab.jobtab).io as u_int,
                );
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh88 = asp;
                asp = asp + 1;
                let ref mut fresh89 = *addstk.as_mut_ptr().offset(fresh88 as isize);
                *fresh89 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            86 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr)
                    .len = itocstring(
                    ((*cptr).buf).as_mut_ptr(),
                    ((partab.jobtab).offset_from((*systab).jobtab) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as libc::c_int,
                );
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh90 = asp;
                asp = asp + 1;
                let ref mut fresh91 = *addstk.as_mut_ptr().offset(fresh90 as isize);
                *fresh91 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            87 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Vkey(((*cptr).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh92 = asp;
                    asp = asp + 1;
                    let ref mut fresh93 = *addstk.as_mut_ptr().offset(fresh92 as isize);
                    *fresh93 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            88 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 1 as libc::c_int as u_short;
                (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh94 = asp;
                asp = asp + 1;
                let ref mut fresh95 = *addstk.as_mut_ptr().offset(fresh94 as isize);
                *fresh95 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            89 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                if (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].type_0
                    as libc::c_int == 4 as libc::c_int
                {
                    (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                }
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                (*cptr).len = 1 as libc::c_int as u_short;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh96 = asp;
                asp = asp + 1;
                let ref mut fresh97 = *addstk.as_mut_ptr().offset(fresh96 as isize);
                *fresh97 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            90 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Vreference(((*cptr).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh98 = asp;
                    asp = asp + 1;
                    let ref mut fresh99 = *addstk.as_mut_ptr().offset(fresh98 as isize);
                    *fresh99 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            91 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = 0 as libc::c_int as libc::c_short;
                i = 0 as libc::c_int;
                while i < (1023 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
                    if ((*symtab.as_mut_ptr().offset(i as isize)).data).is_null() {
                        s += 1;
                        s;
                    }
                    i += 1;
                    i;
                }
                (*cptr).len = itocstring(((*cptr).buf).as_mut_ptr(), s as libc::c_int);
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh100 = asp;
                asp = asp + 1;
                let ref mut fresh101 = *addstk.as_mut_ptr().offset(fresh100 as isize);
                *fresh101 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            92 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr)
                    .len = itocstring(
                    ((*cptr).buf).as_mut_ptr(),
                    (*partab.jobtab).cur_do,
                );
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh102 = asp;
                asp = asp + 1;
                let ref mut fresh103 = *addstk.as_mut_ptr().offset(fresh102 as isize);
                *fresh103 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            93 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Vsystem(((*cptr).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh104 = asp;
                    asp = asp + 1;
                    let ref mut fresh105 = *addstk
                        .as_mut_ptr()
                        .offset(fresh104 as isize);
                    *fresh105 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            94 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).buf[0 as libc::c_int as usize] = '0' as i32 as u_char;
                if (*partab.jobtab).test as libc::c_int != 0 as libc::c_int {
                    (*cptr).buf[0 as libc::c_int as usize] = '1' as i32 as u_char;
                }
                (*cptr).buf[1 as libc::c_int as usize] = '\0' as i32 as u_char;
                (*cptr).len = 1 as libc::c_int as u_short;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh106 = asp;
                asp = asp + 1;
                let ref mut fresh107 = *addstk.as_mut_ptr().offset(fresh106 as isize);
                *fresh107 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            95 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Vx(((*cptr).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh108 = asp;
                    asp = asp + 1;
                    let ref mut fresh109 = *addstk
                        .as_mut_ptr()
                        .offset(fresh108 as isize);
                    *fresh109 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            96 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Vy(((*cptr).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh110 = asp;
                    asp = asp + 1;
                    let ref mut fresh111 = *addstk
                        .as_mut_ptr()
                        .offset(fresh110 as isize);
                    *fresh111 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            100 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                s = Dascii1(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh112 = asp;
                    asp = asp + 1;
                    let ref mut fresh113 = *addstk
                        .as_mut_ptr()
                        .offset(fresh112 as isize);
                    *fresh113 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            101 => {
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                s = Dascii2(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    i,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh114 = asp;
                    asp = asp + 1;
                    let ref mut fresh115 = *addstk
                        .as_mut_ptr()
                        .offset(fresh114 as isize);
                    *fresh115 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            102 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                (*cptr).len = 0 as libc::c_int as u_short;
                let fresh116 = rsmpc;
                rsmpc = rsmpc.offset(1);
                args = *fresh116 as libc::c_int;
                i = 0 as libc::c_int;
                while i < args {
                    s = Dchar(
                        &mut *((*cptr).buf).as_mut_ptr().offset((*cptr).len as isize),
                        cstringtoi(
                            *addstk.as_mut_ptr().offset((asp - args + i) as isize)
                                as *mut cstring,
                        ),
                    );
                    (*cptr)
                        .len = ((*cptr).len as libc::c_int + s as libc::c_int)
                        as u_short;
                    i += 1;
                    i;
                }
                asp -= args;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let fresh117 = asp;
                asp = asp + 1;
                let ref mut fresh118 = *addstk.as_mut_ptr().offset(fresh117 as isize);
                *fresh118 = cptr as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            103 => {
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Ddata(((*cptr).buf).as_mut_ptr(), var);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh119 = asp;
                    asp = asp + 1;
                    let ref mut fresh120 = *addstk
                        .as_mut_ptr()
                        .offset(fresh119 as isize);
                    *fresh120 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            104 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                t = Dextract(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh121 = asp;
                    asp = asp + 1;
                    let ref mut fresh122 = *addstk
                        .as_mut_ptr()
                        .offset(fresh121 as isize);
                    *fresh122 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            105 => {
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                t = Dextract(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    i,
                    i,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh123 = asp;
                    asp = asp + 1;
                    let ref mut fresh124 = *addstk
                        .as_mut_ptr()
                        .offset(fresh123 as isize);
                    *fresh124 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            106 => {
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                t = Dextract(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    i,
                    j,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh125 = asp;
                    asp = asp + 1;
                    let ref mut fresh126 = *addstk
                        .as_mut_ptr()
                        .offset(fresh125 as isize);
                    *fresh126 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            107 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                t = Dfind2(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ptr1,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh127 = asp;
                    asp = asp + 1;
                    let ref mut fresh128 = *addstk
                        .as_mut_ptr()
                        .offset(fresh127 as isize);
                    *fresh128 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            108 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                t = Dfind3(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ptr1,
                    i,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh129 = asp;
                    asp = asp + 1;
                    let ref mut fresh130 = *addstk
                        .as_mut_ptr()
                        .offset(fresh129 as isize);
                    *fresh130 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            109 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*cptr).buf).as_mut_ptr();
                ptr1 = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = ncopy(&mut p, ((*ptr1).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*ptr1).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*ptr1).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    t = Dfnumber2(((*cptr).buf).as_mut_ptr(), ptr1, ptr2);
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = t as u_short;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add((*cptr).len as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh131 = asp;
                        asp = asp + 1;
                        let ref mut fresh132 = *addstk
                            .as_mut_ptr()
                            .offset(fresh131 as isize);
                        *fresh132 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            110 => {
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*cptr).buf).as_mut_ptr();
                ptr1 = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = ncopy(&mut p, ((*ptr1).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*ptr1).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*ptr1).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    t = Dfnumber3(((*cptr).buf).as_mut_ptr(), ptr1, ptr2, i);
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = t as u_short;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add((*cptr).len as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh133 = asp;
                        asp = asp + 1;
                        let ref mut fresh134 = *addstk
                            .as_mut_ptr()
                            .offset(fresh133 as isize);
                        *fresh134 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            111 => {
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = Dget1(((*cptr).buf).as_mut_ptr(), var);
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh135 = asp;
                    asp = asp + 1;
                    let ref mut fresh136 = *addstk
                        .as_mut_ptr()
                        .offset(fresh135 as isize);
                    *fresh136 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            112 => {
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = Dget2(((*cptr).buf).as_mut_ptr(), var, ptr1);
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh137 = asp;
                    asp = asp + 1;
                    let ref mut fresh138 = *addstk
                        .as_mut_ptr()
                        .offset(fresh137 as isize);
                    *fresh138 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            98 => {
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Dincrement1(((*cptr).buf).as_mut_ptr(), var);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh139 = asp;
                    asp = asp + 1;
                    let ref mut fresh140 = *addstk
                        .as_mut_ptr()
                        .offset(fresh139 as isize);
                    *fresh140 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            99 => {
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*cptr).buf).as_mut_ptr();
                ptr1 = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = ncopy(&mut p, ((*ptr1).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*ptr1).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*ptr1).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    asp -= 1;
                    var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    s = Dincrement2(((*cptr).buf).as_mut_ptr(), var, ptr1);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = s as u_short;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add((*cptr).len as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh141 = asp;
                        asp = asp + 1;
                        let ref mut fresh142 = *addstk
                            .as_mut_ptr()
                            .offset(fresh141 as isize);
                        *fresh142 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            113 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                t = Djustify2(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    i,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh143 = asp;
                    asp = asp + 1;
                    let ref mut fresh144 = *addstk
                        .as_mut_ptr()
                        .offset(fresh143 as isize);
                    *fresh144 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            114 => {
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*cptr).buf).as_mut_ptr();
                ptr1 = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = ncopy(&mut p, ((*ptr1).buf).as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*ptr1).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*ptr1).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    t = Djustify3(((*cptr).buf).as_mut_ptr(), ptr1, i, j);
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = t as u_short;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add((*cptr).len as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh145 = asp;
                        asp = asp + 1;
                        let ref mut fresh146 = *addstk
                            .as_mut_ptr()
                            .offset(fresh145 as isize);
                        *fresh146 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            115 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                s = Dlength1(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh147 = asp;
                    asp = asp + 1;
                    let ref mut fresh148 = *addstk
                        .as_mut_ptr()
                        .offset(fresh147 as isize);
                    *fresh148 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            116 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                s = Dlength2(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ptr1,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh149 = asp;
                    asp = asp + 1;
                    let ref mut fresh150 = *addstk
                        .as_mut_ptr()
                        .offset(fresh149 as isize);
                    *fresh150 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            117 => {
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Dname1(((*cptr).buf).as_mut_ptr(), var);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh151 = asp;
                    asp = asp + 1;
                    let ref mut fresh152 = *addstk
                        .as_mut_ptr()
                        .offset(fresh151 as isize);
                    *fresh152 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            118 => {
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Dname2(((*cptr).buf).as_mut_ptr(), var, i);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh153 = asp;
                    asp = asp + 1;
                    let ref mut fresh154 = *addstk
                        .as_mut_ptr()
                        .offset(fresh153 as isize);
                    *fresh154 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            119 => {
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Dorder1(((*cptr).buf).as_mut_ptr(), var);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh155 = asp;
                    asp = asp + 1;
                    let ref mut fresh156 = *addstk
                        .as_mut_ptr()
                        .offset(fresh155 as isize);
                    *fresh156 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            120 => {
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Dorder2(((*cptr).buf).as_mut_ptr(), var, i);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh157 = asp;
                    asp = asp + 1;
                    let ref mut fresh158 = *addstk
                        .as_mut_ptr()
                        .offset(fresh157 as isize);
                    *fresh158 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            121 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                t = Dpiece2(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ptr1,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh159 = asp;
                    asp = asp + 1;
                    let ref mut fresh160 = *addstk
                        .as_mut_ptr()
                        .offset(fresh159 as isize);
                    *fresh160 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            122 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                t = Dpiece3(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ptr1,
                    i,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh161 = asp;
                    asp = asp + 1;
                    let ref mut fresh162 = *addstk
                        .as_mut_ptr()
                        .offset(fresh161 as isize);
                    *fresh162 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            123 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                t = Dpiece4(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ptr1,
                    i,
                    j,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh163 = asp;
                    asp = asp + 1;
                    let ref mut fresh164 = *addstk
                        .as_mut_ptr()
                        .offset(fresh163 as isize);
                    *fresh164 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            124 => {
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                var = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut mvar;
                s = UTIL_MvarFromCStr(cptr, var);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    cptr = var as *mut cstring;
                    (*cptr)
                        .len = itocstring(((*cptr).buf).as_mut_ptr(), s as libc::c_int);
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh165 = asp;
                    asp = asp + 1;
                    let ref mut fresh166 = *addstk
                        .as_mut_ptr()
                        .offset(fresh165 as isize);
                    *fresh166 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            125 => {
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                if j < -(1 as libc::c_int) {
                    (*partab.jobtab)
                        .async_error = -(12 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    asp -= 1;
                    ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                    var = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut mvar;
                    s = UTIL_MvarFromCStr(ptr1, var);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<var_u>() as libc::c_ulong)
                                    .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                    .wrapping_add((*var).slen as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut cstring;
                        let fresh167 = asp;
                        asp = asp + 1;
                        let ref mut fresh168 = *addstk
                            .as_mut_ptr()
                            .offset(fresh167 as isize);
                        *fresh168 = cptr as *mut u_char;
                        if j == -(1 as libc::c_int) {
                            if (*var).uci as libc::c_int == 255 as libc::c_int
                                || (*var).uci as libc::c_int == 0 as libc::c_int
                            {
                                (*cptr).len = 0 as libc::c_int as u_short;
                                (*cptr)
                                    .buf[0 as libc::c_int as usize] = '\0' as i32 as u_char;
                                ssp = (ssp as libc::c_ulong)
                                    .wrapping_add(
                                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                            .wrapping_add((*cptr).len as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as libc::c_int as libc::c_int;
                            } else {
                                i = 0 as libc::c_int;
                                if (*var).volset != 0 {
                                    if ((*systab)
                                        .vol[((*var).volset as libc::c_int - 1 as libc::c_int)
                                        as usize])
                                        .is_null()
                                    {
                                        (*partab.jobtab)
                                            .async_error = -(26 as libc::c_int) as libc::c_short;
                                        (*partab.jobtab).attention = 1 as libc::c_int;
                                        current_block_2879 = 16169013146210219323;
                                    } else {
                                        list = &mut (*((*(**((*systab).vol)
                                            .as_mut_ptr()
                                            .offset(
                                                ((*var).volset as libc::c_int - 1 as libc::c_int) as isize,
                                            ))
                                            .vollab)
                                            .uci)
                                            .as_mut_ptr()
                                            .offset(
                                                ((*var).uci as libc::c_int - 1 as libc::c_int) as isize,
                                            ))
                                            .name;
                                        args = 0 as libc::c_int;
                                        while args < 32 as libc::c_int {
                                            if (*list).var_cu[args as usize] as libc::c_int
                                                == 0 as libc::c_int
                                            {
                                                break;
                                            }
                                            let fresh169 = i;
                                            i = i + 1;
                                            (*cptr)
                                                .buf[fresh169 as usize] = (*list).var_cu[args as usize];
                                            args += 1;
                                            args;
                                        }
                                        let fresh170 = i;
                                        i = i + 1;
                                        (*cptr).buf[fresh170 as usize] = ',' as i32 as u_char;
                                        list = &mut (*(**((*systab).vol)
                                            .as_mut_ptr()
                                            .offset(
                                                ((*var).volset as libc::c_int - 1 as libc::c_int) as isize,
                                            ))
                                            .vollab)
                                            .volnam;
                                        args = 0 as libc::c_int;
                                        while args < 32 as libc::c_int {
                                            if (*list).var_cu[args as usize] as libc::c_int
                                                == 0 as libc::c_int
                                            {
                                                break;
                                            }
                                            let fresh171 = i;
                                            i = i + 1;
                                            (*cptr)
                                                .buf[fresh171 as usize] = (*list).var_cu[args as usize];
                                            args += 1;
                                            args;
                                        }
                                        current_block_2879 = 7575672726450265966;
                                    }
                                } else {
                                    current_block_2879 = 7575672726450265966;
                                }
                                match current_block_2879 {
                                    16169013146210219323 => {}
                                    _ => {
                                        (*cptr).buf[i as usize] = '\0' as i32 as u_char;
                                        (*cptr).len = i as u_short;
                                        ssp = (ssp as libc::c_ulong)
                                            .wrapping_add(
                                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                                    .wrapping_add((*cptr).len as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            ) as libc::c_int as libc::c_int;
                                    }
                                }
                            }
                        } else if j == 0 as libc::c_int {
                            let mut k: libc::c_int = 0 as libc::c_int;
                            if (*var).uci as libc::c_int != 255 as libc::c_int {
                                (*cptr)
                                    .buf[0 as libc::c_int as usize] = '^' as i32 as u_char;
                                k += 1;
                                k;
                            }
                            i = 0 as libc::c_int;
                            while i < 32 as libc::c_int {
                                if (*var).name.var_cu[i as usize] as libc::c_int
                                    == '\0' as i32
                                {
                                    break;
                                }
                                (*cptr)
                                    .buf[(i + k) as usize] = (*var).name.var_cu[i as usize];
                                i += 1;
                                i;
                            }
                            (*cptr).buf[(i + k) as usize] = '\0' as i32 as u_char;
                            (*cptr).len = (i + k) as u_short;
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                        .wrapping_add((*cptr).len as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if j > s as libc::c_int {
                            (*cptr).len = 0 as libc::c_int as u_short;
                            (*cptr)
                                .buf[0 as libc::c_int as usize] = '\0' as i32 as u_char;
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                        .wrapping_add((*cptr).len as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else {
                            args = 0 as libc::c_int;
                            while j != 0 {
                                i = 0 as libc::c_int;
                                s = UTIL_Key_Extract(
                                    &mut *((*var).key).as_mut_ptr().offset(args as isize),
                                    ((*cptr).buf).as_mut_ptr(),
                                    &mut i,
                                );
                                args += i;
                                j -= 1;
                                j;
                            }
                            (*cptr).len = s as u_short;
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                        .wrapping_add((*cptr).len as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            126 | 127 => {
                i = 1 as libc::c_int;
                if opc == 127 as libc::c_int {
                    asp -= 1;
                    i = cstringtoi(
                        *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    );
                }
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Dquery2(((*cptr).buf).as_mut_ptr(), var, i);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh172 = asp;
                    asp = asp + 1;
                    let ref mut fresh173 = *addstk
                        .as_mut_ptr()
                        .offset(fresh172 as isize);
                    *fresh173 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            128 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                s = Drandom(((*cptr).buf).as_mut_ptr(), i);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh174 = asp;
                    asp = asp + 1;
                    let ref mut fresh175 = *addstk
                        .as_mut_ptr()
                        .offset(fresh174 as isize);
                    *fresh175 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            129 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                t = Dreverse(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh176 = asp;
                    asp = asp + 1;
                    let ref mut fresh177 = *addstk
                        .as_mut_ptr()
                        .offset(fresh176 as isize);
                    *fresh177 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            130 => {
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Dstack1(((*cptr).buf).as_mut_ptr(), i);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh178 = asp;
                    asp = asp + 1;
                    let ref mut fresh179 = *addstk
                        .as_mut_ptr()
                        .offset(fresh178 as isize);
                    *fresh179 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            131 => {
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = Dstack2(((*cptr).buf).as_mut_ptr(), i, ptr1);
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh180 = asp;
                    asp = asp + 1;
                    let ref mut fresh181 = *addstk
                        .as_mut_ptr()
                        .offset(fresh180 as isize);
                    *fresh181 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            132 => {
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = Dtext(((*cptr).buf).as_mut_ptr(), ptr1);
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh182 = asp;
                    asp = asp + 1;
                    let ref mut fresh183 = *addstk
                        .as_mut_ptr()
                        .offset(fresh182 as isize);
                    *fresh183 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            133 => {
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                t = Dtranslate2(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ptr1,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh184 = asp;
                    asp = asp + 1;
                    let ref mut fresh185 = *addstk
                        .as_mut_ptr()
                        .offset(fresh184 as isize);
                    *fresh185 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            134 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                t = Dtranslate3(
                    ((*cptr).buf).as_mut_ptr(),
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ptr1,
                    ptr2,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh186 = asp;
                    asp = asp + 1;
                    let ref mut fresh187 = *addstk
                        .as_mut_ptr()
                        .offset(fresh186 as isize);
                    *fresh187 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            135 => {
                current_block_2879 = 2413469331564126937;
            }
            136 | 137 => {
                current_block_2879 = 2413469331564126937;
            }
            138 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                let fresh190 = rsmpc;
                rsmpc = rsmpc.offset(1);
                args = *fresh190 as libc::c_int;
                if args == 2 as libc::c_int {
                    asp -= 1;
                    j = cstringtoi(
                        *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    );
                    asp -= 1;
                    i = cstringtoi(
                        *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    );
                    if i > -(1 as libc::c_int) {
                        (*partab.jobtab)
                            .async_error = -(63 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        i = -i;
                        if i > 1 as libc::c_int {
                            (*partab.jobtab)
                                .async_error = -(63 as libc::c_int + 200 as libc::c_int)
                                as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else if j > -(1 as libc::c_int) {
                            if !(*((*partab.jobtab).view)
                                .as_mut_ptr()
                                .offset((i - 1 as libc::c_int) as isize))
                                .is_null()
                            {
                                DB_ViewRel(
                                    i as u_int,
                                    *((*partab.jobtab).view)
                                        .as_mut_ptr()
                                        .offset((i - 1 as libc::c_int) as isize),
                                );
                            }
                            let ref mut fresh191 = *((*partab.jobtab).view)
                                .as_mut_ptr()
                                .offset((i - 1 as libc::c_int) as isize);
                            *fresh191 = 0 as *mut GBD;
                            if !(j == 0 as libc::c_int) {
                                let ref mut fresh192 = *((*partab.jobtab).view)
                                    .as_mut_ptr()
                                    .offset((i - 1 as libc::c_int) as isize);
                                *fresh192 = DB_ViewGet(i as u_int, j as u_int);
                                if (*((*partab.jobtab).view)
                                    .as_mut_ptr()
                                    .offset((i - 1 as libc::c_int) as isize))
                                    .is_null()
                                {
                                    (*partab.jobtab)
                                        .async_error = -(63 as libc::c_int + 200 as libc::c_int)
                                        as libc::c_short;
                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                }
                            }
                        } else {
                            j = -j;
                            if (*((*partab.jobtab).view)
                                .as_mut_ptr()
                                .offset((i - 1 as libc::c_int) as isize))
                                .is_null()
                            {
                                (*partab.jobtab)
                                    .async_error = -(63 as libc::c_int + 200 as libc::c_int)
                                    as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                            } else if (**((*partab.jobtab).view)
                                .as_mut_ptr()
                                .offset((i - 1 as libc::c_int) as isize))
                                .block as libc::c_int != j
                            {
                                (*partab.jobtab)
                                    .async_error = -(63 as libc::c_int + 200 as libc::c_int)
                                    as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                            } else {
                                DB_ViewPut(
                                    i as u_int,
                                    *((*partab.jobtab).view)
                                        .as_mut_ptr()
                                        .offset((i - 1 as libc::c_int) as isize),
                                );
                                let ref mut fresh193 = *((*partab.jobtab).view)
                                    .as_mut_ptr()
                                    .offset((i - 1 as libc::c_int) as isize);
                                *fresh193 = 0 as *mut GBD;
                            }
                        }
                    }
                } else {
                    (*partab.jobtab)
                        .async_error = -(63 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            139 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                asp -= 1;
                var2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int
                    == '$' as i32
                {
                    if toupper(
                        (*var).name.var_cu[1 as libc::c_int as usize] as libc::c_int,
                    ) != 'R' as i32
                    {
                        (*partab.jobtab)
                            .async_error = -(29 as libc::c_int) as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        t = Compile_Routine(
                            var,
                            var2,
                            &mut *strstk.as_mut_ptr().offset(ssp as isize),
                        );
                        if t < 0 as libc::c_int {
                            (*partab.jobtab).async_error = t as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        }
                    }
                } else if (*var2).name.var_cu[0 as libc::c_int as usize] as libc::c_int
                    == '$' as i32
                {
                    (*partab.jobtab).async_error = -(29 as libc::c_int) as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    s = Ddata(temp.as_mut_ptr(), var2);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else if !(temp[0 as libc::c_int as usize] as libc::c_int
                        == '0' as i32)
                    {
                        cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut cstring;
                        if (*var2).uci as libc::c_int == 255 as libc::c_int {
                            t = ST_Get(var2, ((*cptr).buf).as_mut_ptr());
                        } else {
                            t = DB_Get(var2, ((*cptr).buf).as_mut_ptr());
                        }
                        if t < 0 as libc::c_int && t != -(6 as libc::c_int)
                            && t != -(7 as libc::c_int)
                        {
                            (*partab.jobtab).async_error = t as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            if t >= 0 as libc::c_int {
                                (*cptr).len = t as u_short;
                                if (*var).uci as libc::c_int == 255 as libc::c_int {
                                    t = ST_Set(var, cptr);
                                } else {
                                    t = DB_Set(var, cptr);
                                }
                                if t < 0 as libc::c_int {
                                    (*partab.jobtab).async_error = t as libc::c_short;
                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                    current_block_2879 = 16169013146210219323;
                                } else {
                                    current_block_2879 = 8908401620413876961;
                                }
                            } else {
                                current_block_2879 = 8908401620413876961;
                            }
                            match current_block_2879 {
                                16169013146210219323 => {}
                                _ => {
                                    i = (*var).slen as libc::c_int;
                                    j = (*var2).slen as libc::c_int;
                                    memcpy(
                                        temp.as_mut_ptr() as *mut libc::c_void,
                                        ((*var2).key).as_mut_ptr() as *const libc::c_void,
                                        j as libc::c_ulong,
                                    );
                                    loop {
                                        if (*var2).uci as libc::c_int == 255 as libc::c_int {
                                            t = ST_QueryD(var2, ((*cptr).buf).as_mut_ptr());
                                        } else {
                                            t = DB_QueryD(var2, ((*cptr).buf).as_mut_ptr())
                                                as libc::c_int;
                                        }
                                        if t == -(55 as libc::c_int + 200 as libc::c_int) {
                                            break;
                                        }
                                        if t < 0 as libc::c_int {
                                            (*partab.jobtab).async_error = t as libc::c_short;
                                            (*partab.jobtab).attention = 1 as libc::c_int;
                                            break;
                                        } else {
                                            if memcmp(
                                                ((*var2).key).as_mut_ptr() as *const libc::c_void,
                                                temp.as_mut_ptr() as *const libc::c_void,
                                                j as libc::c_ulong,
                                            ) != 0
                                            {
                                                break;
                                            }
                                            (*cptr).len = t as u_short;
                                            memmove(
                                                &mut *((*var).key).as_mut_ptr().offset(i as isize)
                                                    as *mut u_char as *mut libc::c_void,
                                                &mut *((*var2).key).as_mut_ptr().offset(j as isize)
                                                    as *mut u_char as *const libc::c_void,
                                                ((*var2).slen as libc::c_int - j) as libc::c_ulong,
                                            );
                                            if i + (*var2).slen as libc::c_int - j > 255 as libc::c_int
                                            {
                                                (*partab.jobtab)
                                                    .async_error = -(2 as libc::c_int + 200 as libc::c_int)
                                                    as libc::c_short;
                                                (*partab.jobtab).attention = 1 as libc::c_int;
                                                break;
                                            } else {
                                                (*var)
                                                    .slen = (i + (*var2).slen as libc::c_int - j) as u_char;
                                                if (*var).uci as libc::c_int == 255 as libc::c_int {
                                                    t = ST_Set(var, cptr);
                                                } else {
                                                    t = DB_Set(var, cptr);
                                                }
                                                if !(t < 0 as libc::c_int) {
                                                    continue;
                                                }
                                                (*partab.jobtab).async_error = t as libc::c_short;
                                                (*partab.jobtab).attention = 1 as libc::c_int;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            140 => {
                current_block_2879 = 8991618826084423040;
            }
            141 => {
                current_block_2879 = 8991618826084423040;
            }
            142 => {
                current_block_2879 = 7162237048274468123;
            }
            143 => {
                current_block_2879 = 538692631886353810;
            }
            144 | 145 => {
                current_block_2879 = 3911658486967328820;
            }
            146 => {
                current_block_2879 = 14853549334210594354;
            }
            147 => {
                current_block_2879 = 14853549334210594354;
            }
            148 | 149 => {
                current_block_2879 = 6240798591469247094;
            }
            150 => {
                current_block_2879 = 912176160540298864;
            }
            151 => {
                current_block_2879 = 912176160540298864;
            }
            152 | 153 => {
                current_block_2879 = 5259252906397300293;
            }
            154 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                tmp = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                source_ptr = ((*tmp).buf).as_mut_ptr();
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                comp_ptr = ((*cptr).buf).as_mut_ptr();
                parse();
                let fresh214 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh214 = 157 as libc::c_int as u_char;
                let fresh215 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh215 = 0 as libc::c_int as u_char;
                let fresh216 = comp_ptr;
                comp_ptr = comp_ptr.offset(1);
                *fresh216 = 0 as libc::c_int as u_char;
                (*cptr)
                    .len = comp_ptr.offset_from(((*cptr).buf).as_mut_ptr())
                    as libc::c_long as u_short;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<u_short>() as libc::c_ulong)
                            .wrapping_add((*cptr).len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                if (*partab.jobtab).cur_do >= 128 as libc::c_int {
                    (*partab.jobtab)
                        .async_error = -(8 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].pc = rsmpc;
                    (*partab.jobtab).cur_do += 1;
                    (*partab.jobtab).cur_do;
                    rsmpc = ((*cptr).buf).as_mut_ptr();
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .routine = ((*tmp).buf).as_mut_ptr();
                    (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].pc = rsmpc;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .symbol = 0 as *mut libc::c_short;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .newtab = 0 as *mut u_char;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .endlin = rsmpc
                        .offset((*cptr).len as libc::c_int as isize)
                        .offset(-(4 as libc::c_int as isize));
                    let mut var_i_23: u_int = 0 as libc::c_int as u_int;
                    while var_i_23 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .rounam
                            .var_qu[var_i_23 as usize] = 0 as libc::c_int as u_int64;
                        var_i_23 = var_i_23.wrapping_add(1);
                        var_i_23;
                    }
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .vol = (*partab.jobtab).vol;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .uci = (*partab.jobtab).uci;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .line_num = 0 as libc::c_int as u_short;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .type_0 = 5 as libc::c_int as u_char;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .level = 0 as libc::c_int as u_char;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .estack = (*partab.jobtab)
                        .dostk[((*partab.jobtab).cur_do - 1 as libc::c_int) as usize]
                        .estack;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .flags = 0 as libc::c_int as u_char;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .savasp = savasp as libc::c_long;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .savssp = savssp as libc::c_long;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .asp = asp as libc::c_long;
                    (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .ssp = ssp as libc::c_long;
                    (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].isp = isp;
                    savasp = asp;
                    savssp = ssp;
                    if infor != 0 {
                        (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .flags = ((*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .flags as libc::c_int | 4 as libc::c_int) as u_char;
                        infor = 0 as libc::c_int;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            156 => {
                let fresh217 = rsmpc;
                rsmpc = rsmpc.offset(1);
                i = *fresh217 as libc::c_int;
                if i
                    == (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].level
                        as libc::c_int
                {
                    current_block_2879 = 16169013146210219323;
                } else if i
                    > (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].level
                        as libc::c_int
                {
                    rsmpc = (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .endlin;
                    current_block_2879 = 16169013146210219323;
                } else {
                    opc = 157 as libc::c_int;
                    (*partab.jobtab)
                        .commands = ((*partab.jobtab).commands).wrapping_sub(1);
                    (*partab.jobtab).commands;
                    current_block_2879 = 5984327314049431062;
                }
            }
            157 | 158 => {
                current_block_2879 = 5984327314049431062;
            }
            160 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                LCK_Remove(0 as libc::c_int);
                current_block_2879 = 16169013146210219323;
            }
            161 => {
                current_block_2879 = 3859637062629325859;
            }
            162 | 163 => {
                current_block_2879 = 3859637062629325859;
            }
            164 | 165 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                cptr = 0 as *mut cstring;
                list = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut var_u;
                let mut var_i_24: u_int = 0 as libc::c_int as u_int;
                while var_i_24 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*list).var_qu[var_i_24 as usize] = 0 as libc::c_int as u_int64;
                    var_i_24 = var_i_24.wrapping_add(1);
                    var_i_24;
                }
                let fresh221 = rsmpc;
                rsmpc = rsmpc.offset(1);
                flag = *fresh221 as libc::c_int;
                args = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < flag {
                    asp -= 1;
                    var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                    let mut var_i_25: u_int = 0 as libc::c_int as u_int;
                    while var_i_25 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        (*list.offset(args as isize))
                            .var_qu[var_i_25
                            as usize] = (*var).name.var_qu[var_i_25 as usize];
                        var_i_25 = var_i_25.wrapping_add(1);
                        var_i_25;
                    }
                    args += 1;
                    args;
                    if var_empty(*list.offset((args - 1 as libc::c_int) as isize)) != 0 {
                        rouadd = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .routine as *mut rbd;
                        vt = (rouadd as *mut u_char)
                            .offset((*rouadd).var_tbl as libc::c_int as isize)
                            as *mut var_u;
                        let mut var_i_26: u_int = 0 as libc::c_int as u_int;
                        while var_i_26 < (32 as libc::c_int / 8 as libc::c_int) as u_int
                        {
                            (*list.offset((args - 1 as libc::c_int) as isize))
                                .var_qu[var_i_26
                                as usize] = (*vt
                                .offset(
                                    ((*var).volset as libc::c_int - 1 as libc::c_int) as isize,
                                ))
                                .var_qu[var_i_26 as usize];
                            var_i_26 = var_i_26.wrapping_add(1);
                            var_i_26;
                        }
                    }
                    if (*var).slen != 0 {
                        (*partab.jobtab)
                            .async_error = -(13 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        break;
                    } else if (*var).uci as libc::c_int != 255 as libc::c_int {
                        (*partab.jobtab)
                            .async_error = -(13 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        break;
                    } else {
                        if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int
                            == '$' as i32
                        {
                            if opc != 164 as libc::c_int {
                                (*partab.jobtab)
                                    .async_error = -(8 as libc::c_int) as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                                break;
                            } else if strncasecmp(
                                &mut *((*var).name.var_cu)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as *mut u_char
                                    as *const libc::c_char,
                                b"$et\0\0" as *const u8 as *const libc::c_char,
                                4 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                                || strncasecmp(
                                    &mut *((*var).name.var_cu)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut u_char
                                        as *const libc::c_char,
                                    b"$etrap\0\0" as *const u8 as *const libc::c_char,
                                    7 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int
                            {
                                let mut var_i_27: u_int = 0 as libc::c_int as u_int;
                                while var_i_27
                                    < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                {
                                    (*var)
                                        .name
                                        .var_qu[var_i_27 as usize] = 0 as libc::c_int as u_int64;
                                    var_i_27 = var_i_27.wrapping_add(1);
                                    var_i_27;
                                }
                                memcpy(
                                    &mut *((*var).name.var_cu)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut u_char
                                        as *mut libc::c_void,
                                    b"$ETRAP\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    6 as libc::c_int as libc::c_ulong,
                                );
                                let mut var_i_28: u_int = 0 as libc::c_int as u_int;
                                while var_i_28
                                    < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                {
                                    (*list.offset((args - 1 as libc::c_int) as isize))
                                        .var_qu[var_i_28
                                        as usize] = (*var).name.var_qu[var_i_28 as usize];
                                    var_i_28 = var_i_28.wrapping_add(1);
                                    var_i_28;
                                }
                                t = ST_GetAdd(var, &mut cptr);
                                if t < 1 as libc::c_int {
                                    cptr = 0 as *mut cstring;
                                }
                            } else if strncasecmp(
                                &mut *((*var).name.var_cu)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as *mut u_char
                                    as *const libc::c_char,
                                b"$es\0\0" as *const u8 as *const libc::c_char,
                                4 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                                || strncasecmp(
                                    &mut *((*var).name.var_cu)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut u_char
                                        as *const libc::c_char,
                                    b"$estack\0\0" as *const u8 as *const libc::c_char,
                                    8 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int
                            {
                                (*partab.jobtab)
                                    .dostk[(*partab.jobtab).cur_do as usize]
                                    .estack = (*partab.jobtab).cur_do as u_char;
                                args -= 1;
                                args;
                            } else {
                                (*partab.jobtab)
                                    .async_error = -(8 as libc::c_int) as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                                break;
                            }
                        }
                        i += 1;
                        i;
                    }
                }
                if args == 0 as libc::c_int && flag != 0 {
                    current_block_2879 = 16169013146210219323;
                } else {
                    if opc == 164 as libc::c_int {
                        s = ST_New(args, list);
                    } else {
                        s = ST_NewAll(args, list);
                    }
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else if !cptr.is_null() {
                        var = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut mvar;
                        let mut var_i_29: u_int = 0 as libc::c_int as u_int;
                        while var_i_29 < (32 as libc::c_int / 8 as libc::c_int) as u_int
                        {
                            (*var)
                                .name
                                .var_qu[var_i_29 as usize] = 0 as libc::c_int as u_int64;
                            var_i_29 = var_i_29.wrapping_add(1);
                            var_i_29;
                        }
                        memcpy(
                            &mut *((*var).name.var_cu)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut u_char
                                as *mut libc::c_void,
                            b"$ETRAP\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            6 as libc::c_int as libc::c_ulong,
                        );
                        (*var).uci = 255 as libc::c_int as u_char;
                        (*var).volset = 0 as libc::c_int as u_char;
                        (*var).slen = 0 as libc::c_int as u_char;
                        t = ST_Set(var, cptr);
                        if t < 0 as libc::c_int {
                            (*partab.jobtab).async_error = t as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        }
                    }
                    current_block_2879 = 16169013146210219323;
                }
            }
            166 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                if (*var).uci as libc::c_int == 255 as libc::c_int {
                    if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int
                        == '$' as i32
                    {
                        (*partab.jobtab)
                            .async_error = -(8 as libc::c_int) as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        current_block_2879 = 16169013146210219323;
                    } else {
                        s = ST_Kill(var);
                        current_block_2879 = 17595112014471143284;
                    }
                } else {
                    if (*var).name.var_cu[0 as libc::c_int as usize] as libc::c_int
                        == '$' as i32
                    {
                        s = SS_Kill(var);
                    } else {
                        memcpy(
                            &mut (*partab.jobtab).last_ref as *mut mvar
                                as *mut libc::c_void,
                            var as *const libc::c_void,
                            (::core::mem::size_of::<var_u>() as libc::c_ulong)
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                .wrapping_add((*var).slen as libc::c_ulong),
                        );
                        s = DB_Kill(var);
                    }
                    current_block_2879 = 17595112014471143284;
                }
                match current_block_2879 {
                    16169013146210219323 => {}
                    _ => {
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        }
                        current_block_2879 = 16169013146210219323;
                    }
                }
            }
            167 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                list = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut var_u;
                let mut var_i_30: u_int = 0 as libc::c_int as u_int;
                while var_i_30 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*list).var_qu[var_i_30 as usize] = 0 as libc::c_int as u_int64;
                    var_i_30 = var_i_30.wrapping_add(1);
                    var_i_30;
                }
                let fresh222 = rsmpc;
                rsmpc = rsmpc.offset(1);
                args = *fresh222 as libc::c_int;
                i = 0 as libc::c_int;
                while i < args {
                    asp -= 1;
                    var = *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar;
                    if (*var).uci as libc::c_int != 255 as libc::c_int {
                        (*partab.jobtab)
                            .async_error = -(13 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        break;
                    } else if (*var).slen != 0 {
                        (*partab.jobtab)
                            .async_error = -(13 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        break;
                    } else {
                        if (*var).volset != 0 {
                            rouadd = (*partab.jobtab)
                                .dostk[(*partab.jobtab).cur_do as usize]
                                .routine as *mut rbd;
                            vt = (rouadd as *mut u_char)
                                .offset((*rouadd).var_tbl as libc::c_int as isize)
                                as *mut var_u;
                            let mut var_i_31: u_int = 0 as libc::c_int as u_int;
                            while var_i_31
                                < (32 as libc::c_int / 8 as libc::c_int) as u_int
                            {
                                (*var)
                                    .name
                                    .var_qu[var_i_31
                                    as usize] = (*vt
                                    .offset(
                                        ((*var).volset as libc::c_int - 1 as libc::c_int) as isize,
                                    ))
                                    .var_qu[var_i_31 as usize];
                                var_i_31 = var_i_31.wrapping_add(1);
                                var_i_31;
                            }
                        }
                        let mut var_i_32: u_int = 0 as libc::c_int as u_int;
                        while var_i_32 < (32 as libc::c_int / 8 as libc::c_int) as u_int
                        {
                            (*list.offset(i as isize))
                                .var_qu[var_i_32
                                as usize] = (*var).name.var_qu[var_i_32 as usize];
                            var_i_32 = var_i_32.wrapping_add(1);
                            var_i_32;
                        }
                        i += 1;
                        i;
                    }
                }
                s = ST_KillAll(args, list);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            168 => {
                var = *addstk.as_mut_ptr().offset((asp - 1 as libc::c_int) as isize)
                    as *mut mvar;
                if (*var).uci as libc::c_int != 255 as libc::c_int {
                    (*partab.jobtab)
                        .async_error = -(13 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else if (*var).slen != 0 {
                    (*partab.jobtab)
                        .async_error = -(13 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    if (*var).volset != 0 {
                        rouadd = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .routine as *mut rbd;
                        vt = (rouadd as *mut u_char)
                            .offset((*rouadd).var_tbl as libc::c_int as isize)
                            as *mut var_u;
                        let mut var_i_33: u_int = 0 as libc::c_int as u_int;
                        while var_i_33 < (32 as libc::c_int / 8 as libc::c_int) as u_int
                        {
                            (*var)
                                .name
                                .var_qu[var_i_33
                                as usize] = (*vt
                                .offset(
                                    ((*var).volset as libc::c_int - 1 as libc::c_int) as isize,
                                ))
                                .var_qu[var_i_33 as usize];
                            var_i_33 = var_i_33.wrapping_add(1);
                            var_i_33;
                        }
                        (*var).volset = 0 as libc::c_int as u_char;
                    }
                    s = ST_Create((*var).name);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        if ((*symtab.as_mut_ptr().offset(s as isize)).data).is_null() {
                            let ref mut fresh223 = (*symtab
                                .as_mut_ptr()
                                .offset(s as isize))
                                .data;
                            *fresh223 = malloc(
                                (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                                    .wrapping_add(
                                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                    )
                                    .wrapping_add(
                                        (::core::mem::size_of::<u_char>() as libc::c_ulong)
                                            .wrapping_mul(20 as libc::c_int as libc::c_ulong),
                                    )
                                    .wrapping_add(
                                        ::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong,
                                    ),
                            ) as *mut ST_DATA;
                            if ((*symtab.as_mut_ptr().offset(s as isize)).data).is_null()
                            {
                                (*partab.jobtab)
                                    .async_error = -(56 as libc::c_int + 200 as libc::c_int)
                                    as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                                current_block_2879 = 16169013146210219323;
                            } else {
                                memset(
                                    (*symtab.as_mut_ptr().offset(s as isize)).data
                                        as *mut libc::c_void,
                                    0 as libc::c_int,
                                    (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                        )
                                        .wrapping_add(
                                            (::core::mem::size_of::<u_char>() as libc::c_ulong)
                                                .wrapping_mul(20 as libc::c_int as libc::c_ulong),
                                        )
                                        .wrapping_add(
                                            ::core::mem::size_of::<*mut ST_depend>() as libc::c_ulong,
                                        ),
                                );
                                (*(*symtab.as_mut_ptr().offset(s as isize)).data)
                                    .attach = 1 as libc::c_int as libc::c_short;
                                (*(*symtab.as_mut_ptr().offset(s as isize)).data)
                                    .dbc = (65534 as libc::c_int + 1 as libc::c_int) as u_short;
                                current_block_2879 = 10699050853939942514;
                            }
                        } else {
                            current_block_2879 = 10699050853939942514;
                        }
                        match current_block_2879 {
                            16169013146210219323 => {}
                            _ => {
                                let fresh224 = asp;
                                asp = asp + 1;
                                let ref mut fresh225 = *addstk
                                    .as_mut_ptr()
                                    .offset(fresh224 as isize);
                                *fresh225 = (*symtab.as_mut_ptr().offset(s as isize)).data
                                    as *mut u_char;
                                let fresh226 = asp;
                                asp = asp + 1;
                                let ref mut fresh227 = *addstk
                                    .as_mut_ptr()
                                    .offset(fresh226 as isize);
                                *fresh227 = 0 as *mut u_char;
                            }
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            169 => {
                let fresh228 = asp;
                asp = asp + 1;
                let ref mut fresh229 = *addstk.as_mut_ptr().offset(fresh228 as isize);
                *fresh229 = &mut var_undefined as *mut u_short as *mut u_char;
                current_block_2879 = 16169013146210219323;
            }
            170 => {
                memcpy(
                    &mut us as *mut u_short as *mut libc::c_void,
                    rsmpc as *const libc::c_void,
                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                );
                rsmpc = rsmpc
                    .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
                (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].line_num = us;
                memcpy(
                    &mut us as *mut u_short as *mut libc::c_void,
                    rsmpc as *const libc::c_void,
                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                );
                (*partab.jobtab)
                    .dostk[(*partab.jobtab).cur_do as usize]
                    .endlin = rsmpc.offset(us as libc::c_int as isize);
                rsmpc = rsmpc
                    .offset(::core::mem::size_of::<u_short>() as libc::c_ulong as isize);
                if partab.debug == -(1 as libc::c_int) {
                    s = Debug(savasp, savssp, 0 as libc::c_int);
                    if s as libc::c_int == 1 as libc::c_int {
                        return s;
                    }
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        current_block_2879 = 16169013146210219323;
                    } else {
                        current_block_2879 = 15063832803681787214;
                    }
                } else {
                    current_block_2879 = 15063832803681787214;
                }
                match current_block_2879 {
                    16169013146210219323 => {}
                    _ => {
                        current_block_2879 = 16169013146210219323;
                    }
                }
            }
            171 => {
                if (*partab.jobtab).dostk[(*partab.jobtab).cur_do as usize].level != 0 {
                    i = *rsmpc as libc::c_int + 1 as libc::c_int;
                    if *rsmpc.offset(i as isize) as libc::c_int == 170 as libc::c_int {
                        i = (i as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        if *rsmpc.offset(i as isize) as libc::c_int == 156 as libc::c_int
                        {
                            rsmpc = &mut *rsmpc.offset(i as isize) as *mut u_char;
                            current_block_2879 = 16169013146210219323;
                        } else {
                            current_block_2879 = 14343008795547224030;
                        }
                    } else {
                        current_block_2879 = 14343008795547224030;
                    }
                } else {
                    current_block_2879 = 14343008795547224030;
                }
                match current_block_2879 {
                    16169013146210219323 => {}
                    _ => {
                        (*partab.jobtab)
                            .async_error = -(11 as libc::c_int) as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        current_block_2879 = 16169013146210219323;
                    }
                }
            }
            172 => {
                memcpy(
                    &mut s as *mut libc::c_short as *mut libc::c_void,
                    rsmpc as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                );
                rsmpc = rsmpc
                    .offset(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                    );
                rsmpc = rsmpc.offset(s as libc::c_int as isize);
                current_block_2879 = 16169013146210219323;
            }
            173 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                forx = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut for_stack;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<for_stack>() as libc::c_ulong)
                    as libc::c_int as libc::c_int;
                let fresh230 = asp;
                asp = asp + 1;
                let ref mut fresh231 = *addstk.as_mut_ptr().offset(fresh230 as isize);
                *fresh231 = forx as *mut u_char;
                savasp = asp;
                savssp = ssp;
                (*forx).type_0 = 0 as libc::c_int as libc::c_short;
                if infor != 0 {
                    (*forx)
                        .type_0 = ((*forx).type_0 as libc::c_int | 16 as libc::c_int)
                        as libc::c_short;
                }
                memcpy(
                    &mut s as *mut libc::c_short as *mut libc::c_void,
                    rsmpc as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                );
                rsmpc = rsmpc
                    .offset(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as isize,
                    );
                (*forx).quit = rsmpc.offset(s as libc::c_int as isize);
                infor = 1 as libc::c_int;
                current_block_2879 = 16169013146210219323;
            }
            174 => {
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                forx = *addstk.as_mut_ptr().offset((asp - 1 as libc::c_int) as isize)
                    as *mut for_stack;
                (*forx)
                    .type_0 = ((*forx).type_0 as libc::c_int & !(15 as libc::c_int)
                    | 1 as libc::c_int) as libc::c_short;
                s = (*forx).svar;
                if s as libc::c_int == -(1 as libc::c_int) {
                    t = ST_Set((*forx).var, cptr);
                } else {
                    t = ST_SymSet(s, cptr) as libc::c_int;
                }
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else if rsmpc == (*forx).startpc {
                    (*forx).nxtarg = 0 as *mut u_char;
                } else {
                    (*forx).nxtarg = rsmpc;
                    rsmpc = (*forx).startpc;
                }
                current_block_2879 = 16169013146210219323;
            }
            175 => {
                forx = *addstk.as_mut_ptr().offset((asp - 3 as libc::c_int) as isize)
                    as *mut for_stack;
                (*forx)
                    .type_0 = ((*forx).type_0 as libc::c_int & !(15 as libc::c_int)
                    | 2 as libc::c_int) as libc::c_short;
                (*forx)
                    .increment = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                    as *mut u_char;
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*cptr).buf).as_mut_ptr();
                s = ncopy(&mut p, (*forx).increment);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    ssp += s as libc::c_int + 2 as libc::c_int;
                    savssp = ssp;
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    asp -= 1;
                    ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = s as u_short;
                        ssp += s as libc::c_int + 4 as libc::c_int;
                        s = (*forx).svar;
                        if s as libc::c_int == -(1 as libc::c_int) {
                            t = ST_Set((*forx).var, cptr);
                        } else {
                            t = ST_SymSet(s, cptr) as libc::c_int;
                        }
                        if t < 0 as libc::c_int {
                            (*partab.jobtab).async_error = t as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else if rsmpc == (*forx).startpc {
                            (*forx).nxtarg = 0 as *mut u_char;
                        } else {
                            (*forx).nxtarg = rsmpc;
                            rsmpc = (*forx).startpc;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            176 => {
                forx = *addstk.as_mut_ptr().offset((asp - 4 as libc::c_int) as isize)
                    as *mut for_stack;
                (*forx)
                    .type_0 = ((*forx).type_0 as libc::c_int & !(15 as libc::c_int)
                    | 3 as libc::c_int) as libc::c_short;
                (*forx)
                    .done = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                    as *mut u_char;
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*cptr).buf).as_mut_ptr();
                s = ncopy(&mut p, (*forx).done);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    ssp += s as libc::c_int + 2 as libc::c_int;
                    (*forx)
                        .increment = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                        as *mut u_char;
                    asp -= 1;
                    cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                    p = ((*cptr).buf).as_mut_ptr();
                    s = ncopy(&mut p, (*forx).increment);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        ssp += s as libc::c_int + 2 as libc::c_int;
                        savssp = ssp;
                        cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut cstring;
                        asp -= 1;
                        ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                        p = ((*ptr1).buf).as_mut_ptr();
                        s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            (*cptr).len = s as u_short;
                            s = (*forx).svar;
                            if s as libc::c_int == -(1 as libc::c_int) {
                                t = ST_Set((*forx).var, cptr);
                            } else {
                                t = ST_SymSet(s, cptr) as libc::c_int;
                            }
                            if t < 0 as libc::c_int {
                                (*partab.jobtab).async_error = t as libc::c_short;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                            } else {
                                if rsmpc == (*forx).startpc {
                                    (*forx).nxtarg = 0 as *mut u_char;
                                } else {
                                    (*forx).nxtarg = rsmpc;
                                    rsmpc = (*forx).startpc;
                                }
                                if *((*forx).increment).offset(0 as libc::c_int as isize)
                                    as libc::c_int == '-' as i32
                                {
                                    i = runtime_comp(
                                        ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                                        (*forx).done as *mut libc::c_char,
                                    ) as libc::c_int;
                                } else {
                                    i = runtime_comp(
                                        (*forx).done as *mut libc::c_char,
                                        ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                                    ) as libc::c_int;
                                }
                                if i != 0 {
                                    rsmpc = (*forx).nxtarg;
                                    if !rsmpc.is_null() {
                                        current_block_2879 = 16169013146210219323;
                                    } else {
                                        rsmpc = (*forx).quit;
                                        infor = (*forx).type_0 as libc::c_int & 16 as libc::c_int;
                                        asp -= 1;
                                        asp;
                                        ssp = (forx as *mut u_char).offset_from(strstk.as_mut_ptr())
                                            as libc::c_long as libc::c_int;
                                        savasp = asp;
                                        savssp = ssp;
                                        current_block_2879 = 14222046820874116842;
                                    }
                                } else {
                                    current_block_2879 = 14222046820874116842;
                                }
                                match current_block_2879 {
                                    16169013146210219323 => {}
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            177 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                i = *rsmpc as libc::c_int;
                var = 0 as *mut mvar;
                if i == 64 as libc::c_int {
                    rsmpc = rsmpc.offset(1);
                    rsmpc;
                    let fresh232 = rsmpc;
                    rsmpc = rsmpc.offset(1);
                    i = *fresh232 as libc::c_int;
                    s = *((*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .symbol)
                        .offset(i as isize);
                    if s as libc::c_int == -(1 as libc::c_int) {
                        rouadd = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .routine as *mut rbd;
                        vt = (rouadd as *mut u_char)
                            .offset((*rouadd).var_tbl as libc::c_int as isize)
                            as *mut var_u;
                        let mut var_i_34: u_int = 0 as libc::c_int as u_int;
                        while var_i_34 < (32 as libc::c_int / 8 as libc::c_int) as u_int
                        {
                            tag
                                .var_qu[var_i_34
                                as usize] = (*vt.offset(i as isize))
                                .var_qu[var_i_34 as usize];
                            var_i_34 = var_i_34.wrapping_add(1);
                            var_i_34;
                        }
                        s = ST_SymAtt(tag);
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else {
                            *((*partab.jobtab)
                                .dostk[(*partab.jobtab).cur_do as usize]
                                .symbol)
                                .offset(i as isize) = s;
                            current_block_2879 = 7503431037401570299;
                        }
                    } else {
                        current_block_2879 = 7503431037401570299;
                    }
                } else {
                    var = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut mvar;
                    s = buildmvar(var, 0 as libc::c_int, asp);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        current_block_2879 = 16169013146210219323;
                    } else {
                        asp = s as libc::c_int;
                        if (*var).uci as libc::c_int != 255 as libc::c_int {
                            (*partab.jobtab)
                                .async_error = -(13 as libc::c_int + 200 as libc::c_int)
                                as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else {
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    ((*var).slen as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<var_u>() as libc::c_ulong,
                                        )
                                        .wrapping_add(
                                            (::core::mem::size_of::<u_char>() as libc::c_ulong)
                                                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                                        ),
                                ) as libc::c_int as libc::c_int;
                            s = -(1 as libc::c_int) as libc::c_short;
                            current_block_2879 = 7503431037401570299;
                        }
                    }
                }
                match current_block_2879 {
                    16169013146210219323 => {}
                    _ => {
                        forx = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                            as *mut u_char as *mut for_stack;
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<for_stack>() as libc::c_ulong,
                            ) as libc::c_int as libc::c_int;
                        let fresh233 = asp;
                        asp = asp + 1;
                        let ref mut fresh234 = *addstk
                            .as_mut_ptr()
                            .offset(fresh233 as isize);
                        *fresh234 = forx as *mut u_char;
                        savasp = asp;
                        savssp = ssp;
                        (*forx).type_0 = 0 as libc::c_int as libc::c_short;
                        if infor != 0 {
                            (*forx)
                                .type_0 = ((*forx).type_0 as libc::c_int
                                | 16 as libc::c_int) as libc::c_short;
                        }
                        (*forx).svar = s;
                        (*forx).var = var;
                        memcpy(
                            &mut s as *mut libc::c_short as *mut libc::c_void,
                            rsmpc as *const libc::c_void,
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                        );
                        rsmpc = rsmpc
                            .offset(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                    as isize,
                            );
                        (*forx).startpc = rsmpc.offset(s as libc::c_int as isize);
                        memcpy(
                            &mut s as *mut libc::c_short as *mut libc::c_void,
                            rsmpc as *const libc::c_void,
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                        );
                        rsmpc = rsmpc
                            .offset(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                    as isize,
                            );
                        (*forx).quit = rsmpc.offset(s as libc::c_int as isize);
                        infor = 1 as libc::c_int;
                        current_block_2879 = 16169013146210219323;
                    }
                }
            }
            178 => {
                forx = *addstk.as_mut_ptr().offset((savasp - 1 as libc::c_int) as isize)
                    as *mut for_stack;
                if (*forx).type_0 as libc::c_int & 7 as libc::c_int == 1 as libc::c_int {
                    rsmpc = (*forx).nxtarg;
                    if rsmpc.is_null() {
                        rsmpc = (*forx).quit;
                        infor = (*forx).type_0 as libc::c_int & 16 as libc::c_int;
                        savasp -= 1;
                        savasp;
                        savssp = (forx as *mut u_char).offset_from(strstk.as_mut_ptr())
                            as libc::c_long as libc::c_int;
                        ssp = savssp;
                        asp = savasp;
                    }
                } else {
                    s = (*forx).svar;
                    if s as libc::c_int == -(1 as libc::c_int) {
                        t = ST_GetAdd((*forx).var, &mut ptr1);
                        if t == -(6 as libc::c_int) {
                            (*partab.jobtab)
                                .async_error = -(15 as libc::c_int) as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else if t < 0 as libc::c_int {
                            (*partab.jobtab).async_error = t as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else {
                            current_block_2879 = 14072219012773433691;
                        }
                    } else {
                        data = (*symtab.as_mut_ptr().offset(s as isize)).data;
                        if data.is_null() {
                            (*partab.jobtab)
                                .async_error = -(15 as libc::c_int) as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else if (*data).dbc as libc::c_int
                            == 65534 as libc::c_int + 1 as libc::c_int
                        {
                            (*partab.jobtab)
                                .async_error = -(15 as libc::c_int) as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else {
                            ptr1 = &mut (*data).dbc as *mut u_short as *mut cstring;
                            current_block_2879 = 14072219012773433691;
                        }
                    }
                    match current_block_2879 {
                        16169013146210219323 => {}
                        _ => {
                            p = ((*ptr1).buf).as_mut_ptr();
                            cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                                as *mut u_char as *mut cstring;
                            s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                            if (s as libc::c_int) < 0 as libc::c_int {
                                (*partab.jobtab).async_error = s;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                            } else {
                                strcpy(
                                    temp.as_mut_ptr() as *mut libc::c_char,
                                    (*forx).increment as *mut libc::c_char,
                                );
                                s = runtime_add(
                                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                                    temp.as_mut_ptr() as *mut libc::c_char,
                                );
                                if (s as libc::c_int) < 0 as libc::c_int {
                                    (*partab.jobtab).async_error = s;
                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                } else {
                                    (*cptr).len = s as u_short;
                                    if (*forx).type_0 as libc::c_int & 7 as libc::c_int
                                        == 3 as libc::c_int
                                    {
                                        if *((*forx).increment).offset(0 as libc::c_int as isize)
                                            as libc::c_int == '-' as i32
                                        {
                                            i = runtime_comp(
                                                ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                                                (*forx).done as *mut libc::c_char,
                                            ) as libc::c_int;
                                        } else {
                                            i = runtime_comp(
                                                (*forx).done as *mut libc::c_char,
                                                ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                                            ) as libc::c_int;
                                        }
                                        if i != 0 {
                                            rsmpc = (*forx).nxtarg;
                                            if !rsmpc.is_null() {
                                                current_block_2879 = 16169013146210219323;
                                            } else {
                                                rsmpc = (*forx).quit;
                                                infor = (*forx).type_0 as libc::c_int & 16 as libc::c_int;
                                                savasp -= 1;
                                                savasp;
                                                savssp = (forx as *mut u_char)
                                                    .offset_from(strstk.as_mut_ptr()) as libc::c_long
                                                    as libc::c_int;
                                                ssp = savssp;
                                                asp = savasp;
                                                current_block_2879 = 16169013146210219323;
                                            }
                                        } else {
                                            current_block_2879 = 16828004236742790775;
                                        }
                                    } else {
                                        current_block_2879 = 16828004236742790775;
                                    }
                                    match current_block_2879 {
                                        16169013146210219323 => {}
                                        _ => {
                                            s = (*forx).svar;
                                            if s as libc::c_int == -(1 as libc::c_int) {
                                                t = ST_Set((*forx).var, cptr);
                                            } else {
                                                t = ST_SymSet(s, cptr) as libc::c_int;
                                            }
                                            if t < 0 as libc::c_int {
                                                (*partab.jobtab).async_error = t as libc::c_short;
                                                (*partab.jobtab).attention = 1 as libc::c_int;
                                            } else {
                                                rsmpc = (*forx).startpc;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            179 => {
                current_block_2879 = 16169013146210219323;
            }
            180 => {
                memcpy(
                    &mut isp as *mut libc::c_long as *mut libc::c_void,
                    rsmpc as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
                );
                rsmpc = rsmpc
                    .offset(
                        ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as isize,
                    );
                memcpy(
                    &mut rsmpc as *mut *mut u_char as *mut libc::c_void,
                    rsmpc as *const libc::c_void,
                    ::core::mem::size_of::<*mut u_char>() as libc::c_ulong,
                );
                current_block_2879 = 16169013146210219323;
            }
            181 => {
                current_block_2879 = 7600179106401104090;
            }
            182 => {
                current_block_2879 = 7600179106401104090;
            }
            183 => {
                current_block_2879 = 18310540048570063734;
            }
            184 => {
                current_block_2879 = 15420190669783501340;
            }
            185 => {
                current_block_2879 = 1974722652628827707;
            }
            186 => {
                current_block_2879 = 17123654937568747916;
            }
            187 => {
                current_block_2879 = 13327607154860047783;
            }
            188 => {
                current_block_2879 = 16115092360499891106;
            }
            189 => {
                current_block_2879 = 8762070809544863061;
            }
            190 => {
                current_block_2879 = 13553578021377382533;
            }
            191 => {
                current_block_2879 = 8590759965554521070;
            }
            192 => {
                current_block_2879 = 14874714617003418979;
            }
            193 => {
                current_block_2879 = 15369043757567109248;
            }
            194 => {
                current_block_2879 = 9658992782896750533;
            }
            195 | 196 => {
                current_block_2879 = 14124765838191351603;
            }
            235 => {
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                if (j & 15 as libc::c_int) < 1 as libc::c_int {
                    (*partab.jobtab)
                        .async_error = -(64 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    var = &mut (*partab.jobtab).last_ref;
                    s = UTIL_MvarFromCStr(ptr1, var);
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        s = DB_Compress(var, j);
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                                as *mut u_char as *mut cstring;
                            (*cptr)
                                .len = itocstring(
                                ((*cptr).buf).as_mut_ptr(),
                                s as libc::c_int,
                            );
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                        .wrapping_add((*cptr).len as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                            let fresh236 = asp;
                            asp = asp + 1;
                            let ref mut fresh237 = *addstk
                                .as_mut_ptr()
                                .offset(fresh236 as isize);
                            *fresh237 = cptr as *mut u_char;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            237 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_host(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh238 = asp;
                    asp = asp + 1;
                    let ref mut fresh239 = *addstk
                        .as_mut_ptr()
                        .offset(fresh238 as isize);
                    *fresh239 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            238 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_file(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh240 = asp;
                    asp = asp + 1;
                    let ref mut fresh241 = *addstk
                        .as_mut_ptr()
                        .offset(fresh240 as isize);
                    *fresh241 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            234 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_wait(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh242 = asp;
                    asp = asp + 1;
                    let ref mut fresh243 = *addstk
                        .as_mut_ptr()
                        .offset(fresh242 as isize);
                    *fresh243 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            239 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_debug(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh244 = asp;
                    asp = asp + 1;
                    let ref mut fresh245 = *addstk
                        .as_mut_ptr()
                        .offset(fresh244 as isize);
                    *fresh245 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            240 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_directory(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh246 = asp;
                    asp = asp + 1;
                    let ref mut fresh247 = *addstk
                        .as_mut_ptr()
                        .offset(fresh246 as isize);
                    *fresh247 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            241 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_errmsg(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh248 = asp;
                    asp = asp + 1;
                    let ref mut fresh249 = *addstk
                        .as_mut_ptr()
                        .offset(fresh248 as isize);
                    *fresh249 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            242 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_opcom(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh250 = asp;
                    asp = asp + 1;
                    let ref mut fresh251 = *addstk
                        .as_mut_ptr()
                        .offset(fresh250 as isize);
                    *fresh251 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            236 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_signal(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh252 = asp;
                    asp = asp + 1;
                    let ref mut fresh253 = *addstk
                        .as_mut_ptr()
                        .offset(fresh252 as isize);
                    *fresh253 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            243 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_spawn(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh254 = asp;
                    asp = asp + 1;
                    let ref mut fresh255 = *addstk
                        .as_mut_ptr()
                        .offset(fresh254 as isize);
                    *fresh255 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            244 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_version(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh256 = asp;
                    asp = asp + 1;
                    let ref mut fresh257 = *addstk
                        .as_mut_ptr()
                        .offset(fresh256 as isize);
                    *fresh257 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            245 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_zwrite(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh258 = asp;
                    asp = asp + 1;
                    let ref mut fresh259 = *addstk
                        .as_mut_ptr()
                        .offset(fresh258 as isize);
                    *fresh259 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            246 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_e(((*cptr).buf).as_mut_ptr() as *mut libc::c_char, ptr1, ptr2);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh260 = asp;
                    asp = asp + 1;
                    let ref mut fresh261 = *addstk
                        .as_mut_ptr()
                        .offset(fresh260 as isize);
                    *fresh261 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            247 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_paschk(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh262 = asp;
                    asp = asp + 1;
                    let ref mut fresh263 = *addstk
                        .as_mut_ptr()
                        .offset(fresh262 as isize);
                    *fresh263 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            248 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = Xcall_v(((*cptr).buf).as_mut_ptr() as *mut libc::c_char, ptr1, ptr2);
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh264 = asp;
                    asp = asp + 1;
                    let ref mut fresh265 = *addstk
                        .as_mut_ptr()
                        .offset(fresh264 as isize);
                    *fresh265 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            249 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = Xcall_x(((*cptr).buf).as_mut_ptr() as *mut libc::c_char, ptr1, ptr2);
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh266 = asp;
                    asp = asp + 1;
                    let ref mut fresh267 = *addstk
                        .as_mut_ptr()
                        .offset(fresh266 as isize);
                    *fresh267 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            250 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_xrsm(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh268 = asp;
                    asp = asp + 1;
                    let ref mut fresh269 = *addstk
                        .as_mut_ptr()
                        .offset(fresh268 as isize);
                    *fresh269 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            251 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_setenv(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh270 = asp;
                    asp = asp + 1;
                    let ref mut fresh271 = *addstk
                        .as_mut_ptr()
                        .offset(fresh270 as isize);
                    *fresh271 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            252 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                t = Xcall_getenv(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh272 = asp;
                    asp = asp + 1;
                    let ref mut fresh273 = *addstk
                        .as_mut_ptr()
                        .offset(fresh272 as isize);
                    *fresh273 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            253 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                var2 = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut mvar;
                ssp = (ssp as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<mvar>() as libc::c_ulong)
                    as libc::c_int as libc::c_int;
                let mut var_i_35: u_int = 0 as libc::c_int as u_int;
                while var_i_35 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    (*var2).name.var_qu[var_i_35 as usize] = 0 as libc::c_int as u_int64;
                    var_i_35 = var_i_35.wrapping_add(1);
                    var_i_35;
                }
                memcpy(
                    &mut (*var2).name.var_cu as *mut [u_char; 32] as *mut libc::c_void,
                    b"$ROUTINE\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    8 as libc::c_int as libc::c_ulong,
                );
                (*var2).volset = (*partab.jobtab).rvol;
                (*var2).uci = (*partab.jobtab).ruci;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = UTIL_Key_Build(
                    ptr1,
                    &mut *((*var2).key).as_mut_ptr().offset(0 as libc::c_int as isize),
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*var2).slen = s as u_char;
                    t = Compile_Routine(
                        0 as *mut libc::c_void as *mut mvar,
                        var2,
                        &mut *strstk.as_mut_ptr().offset(ssp as isize),
                    );
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        cptr = var2 as *mut cstring;
                        s = itocstring(((*cptr).buf).as_mut_ptr(), t) as libc::c_short;
                        (*cptr).len = s as u_short;
                        let fresh274 = asp;
                        asp = asp + 1;
                        let ref mut fresh275 = *addstk
                            .as_mut_ptr()
                            .offset(fresh274 as isize);
                        *fresh275 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            254 => {
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                s = Xcall_fork(
                    ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                    ptr1,
                    ptr2,
                );
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = s as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh276 = asp;
                    asp = asp + 1;
                    let ref mut fresh277 = *addstk
                        .as_mut_ptr()
                        .offset(fresh276 as isize);
                    *fresh277 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            255 => {
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                if i > 1 as libc::c_int || i < 1 as libc::c_int {
                    (*partab.jobtab).async_error = -(26 as libc::c_int) as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else if j < -(2 as libc::c_int) {
                    (*partab.jobtab)
                        .async_error = -(64 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                        as *mut cstring;
                    t = DB_ic(i, j);
                    if t < 0 as libc::c_int {
                        (*partab.jobtab).async_error = t as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        (*cptr).len = itocstring(((*cptr).buf).as_mut_ptr(), t);
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                    .wrapping_add((*cptr).len as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        let fresh278 = asp;
                        asp = asp + 1;
                        let ref mut fresh279 = *addstk
                            .as_mut_ptr()
                            .offset(fresh278 as isize);
                        *fresh279 = cptr as *mut u_char;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            _ => {
                (*partab.jobtab)
                    .async_error = -(14 as libc::c_int + 200 as libc::c_int)
                    as libc::c_short;
                (*partab.jobtab).attention = 1 as libc::c_int;
                current_block_2879 = 16169013146210219323;
            }
        }
        match current_block_2879 {
            3859637062629325859 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                j = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                let fresh220 = rsmpc;
                rsmpc = rsmpc.offset(1);
                args = *fresh220 as libc::c_int;
                p = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char;
                if p as libc::c_long & 1 as libc::c_int as libc::c_long != 0 {
                    p = p.offset(1);
                    p;
                }
                cptr = p as *mut cstring;
                i = 0 as libc::c_int;
                while i < args {
                    asp -= 1;
                    s = UTIL_mvartolock(
                        *addstk.as_mut_ptr().offset(asp as isize) as *mut mvar,
                        p
                            .offset(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                                    as isize,
                            ),
                    );
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        break;
                    } else {
                        *(p as *mut libc::c_short) = s;
                        p = p
                            .offset(
                                (s as libc::c_ulong)
                                    .wrapping_add(
                                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                                    ) as isize,
                            );
                        if p as libc::c_long & 1 as libc::c_int as libc::c_long != 0 {
                            p = p.offset(1);
                            p;
                        }
                        i += 1;
                        i;
                    }
                }
                if opc == 161 as libc::c_int {
                    s = LCK_Old(args, cptr, j);
                } else if opc == 162 as libc::c_int {
                    s = LCK_Add(args, cptr, j);
                } else {
                    s = LCK_Sub(args, cptr);
                }
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                }
                current_block_2879 = 16169013146210219323;
            }
            5984327314049431062 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                if infor != 0 {
                    if opc == 158 as libc::c_int {
                        (*partab.jobtab)
                            .async_error = -(16 as libc::c_int) as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        asp = savasp;
                        asp -= 1;
                        forx = *addstk.as_mut_ptr().offset(asp as isize)
                            as *mut for_stack;
                        infor = (*forx).type_0 as libc::c_int & 16 as libc::c_int;
                        rsmpc = (*forx).quit;
                        ssp = (forx as *mut u_char).offset_from(strstk.as_mut_ptr())
                            as libc::c_long as libc::c_int;
                        savssp = ssp;
                        savasp = asp;
                    }
                } else {
                    curframe = &mut *((*partab.jobtab).dostk)
                        .as_mut_ptr()
                        .offset((*partab.jobtab).cur_do as isize) as *mut do_frame;
                    if (*curframe).type_0 as libc::c_int == 1 as libc::c_int
                        || (*curframe).type_0 as libc::c_int == 2 as libc::c_int
                    {
                        if opc == 157 as libc::c_int {
                            return opc as libc::c_short;
                        }
                        asp -= 1;
                        return (cstringtoi(
                            *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                        ) | 16384 as libc::c_int) as libc::c_short;
                    }
                    if (*curframe).type_0 as libc::c_int == 4 as libc::c_int
                        && opc == 157 as libc::c_int
                    {
                        (*partab.jobtab)
                            .async_error = -(17 as libc::c_int) as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else if (*curframe).type_0 as libc::c_int != 4 as libc::c_int
                        && opc != 157 as libc::c_int
                    {
                        (*partab.jobtab)
                            .async_error = -(16 as libc::c_int) as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        if !((*curframe).newtab).is_null() {
                            ST_Restore((*curframe).newtab as *mut ST_newtab);
                        }
                        infor = (*curframe).flags as libc::c_int & 4 as libc::c_int;
                        if (*partab.jobtab).error_frame != 0 {
                            (*partab.jobtab)
                                .dostk[(*partab.jobtab).cur_do as usize]
                                .symbol = 0 as *mut libc::c_short;
                        }
                        if (*curframe).flags as libc::c_int & 2 as libc::c_int != 0 {
                            if !((*curframe).symbol).is_null() {
                                ST_SymDet(
                                    (*((*curframe).routine as *mut rbd)).num_vars
                                        as libc::c_int,
                                    (*curframe).symbol,
                                );
                            }
                            Routine_Detach((*curframe).routine as *mut rbd);
                        }
                        cptr = 0 as *mut cstring;
                        if opc == 158 as libc::c_int {
                            asp -= 1;
                            cptr = *addstk.as_mut_ptr().offset(asp as isize)
                                as *mut cstring;
                        }
                        savasp = (*curframe).savasp as libc::c_int;
                        savssp = (*curframe).savssp as libc::c_int;
                        asp = (*curframe).asp as libc::c_int;
                        ssp = (*curframe).ssp as libc::c_int;
                        isp = (*curframe).isp;
                        if opc == 158 as libc::c_int {
                            ptr1 = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                                as *mut u_char as *mut cstring;
                            memmove(
                                ptr1 as *mut libc::c_void,
                                cptr as *const libc::c_void,
                                ((*cptr).len as libc::c_ulong)
                                    .wrapping_add(
                                        ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                    )
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    ((*ptr1).len as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                        )
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                            let fresh218 = asp;
                            asp = asp + 1;
                            let ref mut fresh219 = *addstk
                                .as_mut_ptr()
                                .offset(fresh218 as isize);
                            *fresh219 = ptr1 as *mut u_char;
                        }
                        if (*curframe).type_0 as libc::c_int == 4 as libc::c_int
                            || (*curframe).level as libc::c_int != 0
                        {
                            (*partab.jobtab)
                                .test = ((*curframe).flags as libc::c_int
                                & 1 as libc::c_int) as u_char;
                        }
                        (*partab.jobtab).cur_do -= 1;
                        rsmpc = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .pc;
                        if (*partab.jobtab).error_frame as libc::c_int
                            > (*partab.jobtab).cur_do
                            && (*partab.jobtab).cur_do
                                < (*partab.jobtab).etrap_at as libc::c_int
                        {
                            (*partab.jobtab)
                                .async_error = 0 as libc::c_int as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            2413469331564126937 => {
                ptr1 = 0 as *mut cstring;
                j = 1 as libc::c_int;
                if opc == 137 as libc::c_int {
                    asp -= 1;
                    ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                }
                if opc == 136 as libc::c_int || opc == 137 as libc::c_int {
                    asp -= 1;
                    j = cstringtoi(
                        *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    );
                }
                asp -= 1;
                i = cstringtoi(
                    *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                );
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                t = Dview(
                    ((*cptr).buf).as_mut_ptr(),
                    cstringtoi(
                        *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                    ),
                    i,
                    j,
                    ptr1,
                );
                if t < 0 as libc::c_int {
                    (*partab.jobtab).async_error = t as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    (*cptr).len = t as u_short;
                    ssp = (ssp as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<u_short>() as libc::c_ulong)
                                .wrapping_add((*cptr).len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    let fresh188 = asp;
                    asp = asp + 1;
                    let ref mut fresh189 = *addstk
                        .as_mut_ptr()
                        .offset(fresh188 as isize);
                    *fresh189 = cptr as *mut u_char;
                }
                current_block_2879 = 16169013146210219323;
            }
            11581551973594529044 => {
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                if (((*cptr).len as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(isp as libc::c_ulong)
                    > 65536 as libc::c_int as libc::c_ulong
                {
                    (*partab.jobtab)
                        .async_error = -(58 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    source_ptr = ((*cptr).buf).as_mut_ptr();
                    comp_ptr = &mut *indstk.as_mut_ptr().offset(isp as isize)
                        as *mut u_char;
                    i = 0 as libc::c_int;
                    while *source_ptr as libc::c_int == '@' as i32 {
                        source_ptr = source_ptr.offset(1);
                        source_ptr;
                        i += 1;
                        i;
                    }
                    if i != 0 {
                        source_ptr = source_ptr.offset(-1);
                        source_ptr;
                    }
                    s = localvar();
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        if i != 0
                            && *comp_ptr.offset(-(2 as libc::c_int as isize))
                                as libc::c_int != 255 as libc::c_int
                        {
                            loop {
                                let fresh73 = i;
                                i = i - 1;
                                if !(fresh73 != 0) {
                                    break;
                                }
                                let fresh74 = comp_ptr;
                                comp_ptr = comp_ptr.offset(1);
                                *fresh74 = 65 as libc::c_int as u_char;
                            }
                            let fresh75 = comp_ptr;
                            comp_ptr = comp_ptr.offset(1);
                            *fresh75 = 66 as libc::c_int as u_char;
                        } else {
                            *indstk
                                .as_mut_ptr()
                                .offset(
                                    (isp + s as libc::c_long) as isize,
                                ) = 62 as libc::c_int as u_char;
                            if opc == 67 as libc::c_int {
                                *indstk
                                    .as_mut_ptr()
                                    .offset(
                                        (isp + s as libc::c_long) as isize,
                                    ) = 63 as libc::c_int as u_char;
                            }
                            if opc == 68 as libc::c_int {
                                *indstk
                                    .as_mut_ptr()
                                    .offset(
                                        (isp + s as libc::c_long) as isize,
                                    ) = 64 as libc::c_int as u_char;
                            }
                        }
                        if *source_ptr as libc::c_int != '\0' as i32 {
                            (*partab.jobtab)
                                .async_error = -(57 as libc::c_int + 200 as libc::c_int)
                                as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else if comp_ptr
                            .offset(
                                (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
                            )
                            .offset(1 as libc::c_int as isize)
                            >= &mut *indstk
                                .as_mut_ptr()
                                .offset(65536 as libc::c_int as isize) as *mut u_char
                        {
                            (*partab.jobtab)
                                .async_error = -(58 as libc::c_int + 200 as libc::c_int)
                                as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            let fresh76 = comp_ptr;
                            comp_ptr = comp_ptr.offset(1);
                            *fresh76 = 180 as libc::c_int as u_char;
                            memcpy(
                                comp_ptr as *mut libc::c_void,
                                &mut isp as *mut libc::c_long as *const libc::c_void,
                                ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
                            );
                            comp_ptr = comp_ptr
                                .offset(
                                    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                                        as isize,
                                );
                            memcpy(
                                comp_ptr as *mut libc::c_void,
                                &mut rsmpc as *mut *mut u_char as *const libc::c_void,
                                ::core::mem::size_of::<*mut u_char>() as libc::c_ulong,
                            );
                            comp_ptr = comp_ptr
                                .offset(
                                    ::core::mem::size_of::<*mut u_char>() as libc::c_ulong
                                        as isize,
                                );
                            rsmpc = &mut *indstk.as_mut_ptr().offset(isp as isize)
                                as *mut u_char;
                            isp = comp_ptr
                                .offset_from(
                                    &mut *indstk.as_mut_ptr().offset(isp as isize)
                                        as *mut u_char,
                                ) as libc::c_long + isp;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            587670741345194828 => {
                var = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut mvar;
                s = buildmvar(var, (opc == 63 as libc::c_int) as libc::c_int, asp);
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    asp = s as libc::c_int;
                    let fresh68 = asp;
                    asp = asp + 1;
                    let ref mut fresh69 = *addstk.as_mut_ptr().offset(fresh68 as isize);
                    *fresh69 = var as *mut u_char;
                    if opc == 64 as libc::c_int {
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<mvar>() as libc::c_ulong,
                            ) as libc::c_int as libc::c_int;
                    } else {
                        ssp = (ssp as libc::c_ulong)
                            .wrapping_add(
                                ((*var).slen as libc::c_ulong)
                                    .wrapping_add(
                                        ::core::mem::size_of::<var_u>() as libc::c_ulong,
                                    )
                                    .wrapping_add(
                                        (5 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<u_char>() as libc::c_ulong,
                                            ),
                                    ),
                            ) as libc::c_int as libc::c_int;
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            16624893707949258064 => {
                cptr = &mut *strstk.as_mut_ptr().offset(ssp as isize) as *mut u_char
                    as *mut cstring;
                asp -= 1;
                ptr2 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                asp -= 1;
                ptr1 = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                p = ((*ptr2).buf).as_mut_ptr();
                s = ncopy(&mut p, temp.as_mut_ptr());
                if (s as libc::c_int) < 0 as libc::c_int {
                    (*partab.jobtab).async_error = s;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    p = ((*ptr1).buf).as_mut_ptr();
                    s = ncopy(&mut p, ((*cptr).buf).as_mut_ptr());
                    if (s as libc::c_int) < 0 as libc::c_int {
                        (*partab.jobtab).async_error = s;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        s = runtime_div(
                            ((*cptr).buf).as_mut_ptr() as *mut libc::c_char,
                            temp.as_mut_ptr() as *mut libc::c_char,
                            opc as libc::c_short,
                        );
                        if (s as libc::c_int) < 0 as libc::c_int {
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            (*cptr).len = s as u_short;
                            ssp = (ssp as libc::c_ulong)
                                .wrapping_add(
                                    (s as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                        )
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                            let fresh17 = asp;
                            asp = asp + 1;
                            let ref mut fresh18 = *addstk
                                .as_mut_ptr()
                                .offset(fresh17 as isize);
                            *fresh18 = cptr as *mut u_char;
                        }
                    }
                }
                current_block_2879 = 16169013146210219323;
            }
            8991618826084423040 => {
                current_block_2879 = 7162237048274468123;
            }
            14853549334210594354 => {
                current_block_2879 = 6240798591469247094;
            }
            912176160540298864 => {
                current_block_2879 = 5259252906397300293;
            }
            7600179106401104090 => {
                current_block_2879 = 18310540048570063734;
            }
            _ => {}
        }
        match current_block_2879 {
            5259252906397300293 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                while infor != 0 {
                    asp -= 1;
                    forx = *addstk.as_mut_ptr().offset(asp as isize) as *mut for_stack;
                    infor = (*forx).type_0 as libc::c_int & 16 as libc::c_int;
                    ssp = (forx as *mut u_char).offset_from(strstk.as_mut_ptr())
                        as libc::c_long as libc::c_int;
                    savssp = ssp;
                    savasp = asp;
                }
                offset = 0 as libc::c_int;
                let mut var_i_17: u_int = 0 as libc::c_int as u_int;
                while var_i_17 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    rou
                        .var_qu[var_i_17
                        as usize] = (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .rounam
                        .var_qu[var_i_17 as usize];
                    var_i_17 = var_i_17.wrapping_add(1);
                    var_i_17;
                }
                let mut var_i_18: u_int = 0 as libc::c_int as u_int;
                while var_i_18 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    tag.var_qu[var_i_18 as usize] = 0 as libc::c_int as u_int64;
                    var_i_18 = var_i_18.wrapping_add(1);
                    var_i_18;
                }
                if opc == 151 as libc::c_int || opc == 152 as libc::c_int
                    || opc == 153 as libc::c_int
                {
                    memcpy(
                        &mut rou as *mut var_u as *mut libc::c_void,
                        rsmpc as *const libc::c_void,
                        32 as libc::c_int as libc::c_ulong,
                    );
                    rsmpc = rsmpc.offset(32 as libc::c_int as isize);
                }
                if opc == 153 as libc::c_int && var_empty(rou) != 0 {
                    let mut var_i_19: u_int = 0 as libc::c_int as u_int;
                    while var_i_19 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        rou
                            .var_qu[var_i_19
                            as usize] = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .rounam
                            .var_qu[var_i_19 as usize];
                        var_i_19 = var_i_19.wrapping_add(1);
                        var_i_19;
                    }
                }
                if opc == 150 as libc::c_int || opc == 152 as libc::c_int
                    || opc == 153 as libc::c_int
                {
                    memcpy(
                        &mut tag as *mut var_u as *mut libc::c_void,
                        rsmpc as *const libc::c_void,
                        32 as libc::c_int as libc::c_ulong,
                    );
                    rsmpc = rsmpc.offset(32 as libc::c_int as isize);
                }
                if opc == 153 as libc::c_int {
                    if var_empty(tag) != 0 {
                        (*partab.jobtab)
                            .async_error = -(20 as libc::c_int) as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        current_block_2879 = 16169013146210219323;
                    } else if (*systab).historic & 2 as libc::c_int == 0 {
                        (*partab.jobtab)
                            .async_error = -(70 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        current_block_2879 = 16169013146210219323;
                    } else {
                        memcpy(
                            &mut us as *mut u_short as *mut libc::c_void,
                            rsmpc as *const libc::c_void,
                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                        );
                        rsmpc = rsmpc
                            .offset(
                                ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                            );
                        offset = us as libc::c_int;
                        current_block_2879 = 10100579718872087557;
                    }
                } else {
                    current_block_2879 = 10100579718872087557;
                }
                match current_block_2879 {
                    16169013146210219323 => {}
                    _ => {
                        curframe = &mut *((*partab.jobtab).dostk)
                            .as_mut_ptr()
                            .offset((*partab.jobtab).cur_do as isize) as *mut do_frame;
                        rouadd = 0 as *mut rbd;
                        if var_empty(rou) != 0 {
                            i = (*partab.jobtab).cur_do - 1 as libc::c_int;
                            while i > 0 as libc::c_int {
                                if var_empty((*partab.jobtab).dostk[i as usize].rounam) == 0
                                {
                                    let mut var_i_20: u_int = 0 as libc::c_int as u_int;
                                    while var_i_20
                                        < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                    {
                                        rou
                                            .var_qu[var_i_20
                                            as usize] = (*partab.jobtab)
                                            .dostk[i as usize]
                                            .rounam
                                            .var_qu[var_i_20 as usize];
                                        var_i_20 = var_i_20.wrapping_add(1);
                                        var_i_20;
                                    }
                                    break;
                                } else {
                                    i -= 1;
                                    i;
                                }
                            }
                        }
                        if var_empty(rou) != 0 {
                            (*partab.jobtab)
                                .async_error = -(13 as libc::c_int) as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else if var_equal(
                            rou,
                            (*partab.jobtab)
                                .dostk[(*partab.jobtab).cur_do as usize]
                                .rounam,
                        ) == 0
                            && (*partab.jobtab)
                                .dostk[(*partab.jobtab).cur_do as usize]
                                .level as libc::c_int != 0
                        {
                            (*partab.jobtab)
                                .async_error = -(14 as libc::c_int) as libc::c_short;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                        } else {
                            i = (*partab.jobtab).cur_do;
                            while i > 0 as libc::c_int {
                                if var_equal(rou, (*partab.jobtab).dostk[i as usize].rounam)
                                    != 0
                                    && (*partab.jobtab).ruci as libc::c_int
                                        == (*partab.jobtab).dostk[i as usize].uci as libc::c_int
                                    && (*partab.jobtab).rvol as libc::c_int
                                        == (*partab.jobtab).dostk[i as usize].vol as libc::c_int
                                {
                                    rouadd = (*partab.jobtab).dostk[i as usize].routine
                                        as *mut rbd;
                                    if i != (*partab.jobtab).cur_do {
                                        if (*curframe).flags as libc::c_int & 2 as libc::c_int != 0
                                        {
                                            ST_SymDet(
                                                (*((*curframe).routine as *mut rbd)).num_vars
                                                    as libc::c_int,
                                                (*curframe).symbol,
                                            );
                                            Routine_Detach((*curframe).routine as *mut rbd);
                                        }
                                        (*curframe).routine = rouadd as *mut u_char;
                                        (*curframe)
                                            .symbol = (*partab.jobtab).dostk[i as usize].symbol;
                                        let mut var_i_21: u_int = 0 as libc::c_int as u_int;
                                        while var_i_21
                                            < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                        {
                                            (*curframe)
                                                .rounam
                                                .var_qu[var_i_21 as usize] = rou.var_qu[var_i_21 as usize];
                                            var_i_21 = var_i_21.wrapping_add(1);
                                            var_i_21;
                                        }
                                        (*curframe).vol = (*partab.jobtab).rvol;
                                        (*curframe).uci = (*partab.jobtab).ruci;
                                        (*curframe)
                                            .flags = ((*curframe).flags as libc::c_int
                                            & !(2 as libc::c_int)) as u_char;
                                    }
                                    break;
                                } else {
                                    i -= 1;
                                    i;
                                }
                            }
                            if rouadd.is_null() {
                                rouadd = Routine_Attach(rou);
                                if rouadd.is_null() {
                                    (*partab.jobtab)
                                        .async_error = -(13 as libc::c_int) as libc::c_short;
                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                    current_block_2879 = 16169013146210219323;
                                } else if rouadd == -(1 as libc::c_int) as *mut rbd {
                                    (*partab.jobtab)
                                        .async_error = -(52 as libc::c_int + 200 as libc::c_int)
                                        as libc::c_short;
                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                    current_block_2879 = 16169013146210219323;
                                } else if rouadd == -(2 as libc::c_int) as *mut rbd {
                                    (*partab.jobtab)
                                        .async_error = -(59 as libc::c_int + 200 as libc::c_int)
                                        as libc::c_short;
                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                    current_block_2879 = 16169013146210219323;
                                } else {
                                    if (*curframe).flags as libc::c_int & 2 as libc::c_int != 0
                                    {
                                        ST_SymDet(
                                            (*((*curframe).routine as *mut rbd)).num_vars
                                                as libc::c_int,
                                            (*curframe).symbol,
                                        );
                                        Routine_Detach((*curframe).routine as *mut rbd);
                                    }
                                    (*curframe).routine = rouadd as *mut u_char;
                                    (*curframe).symbol = 0 as *mut libc::c_short;
                                    let mut var_i_22: u_int = 0 as libc::c_int as u_int;
                                    while var_i_22
                                        < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                    {
                                        (*curframe)
                                            .rounam
                                            .var_qu[var_i_22 as usize] = rou.var_qu[var_i_22 as usize];
                                        var_i_22 = var_i_22.wrapping_add(1);
                                        var_i_22;
                                    }
                                    (*curframe).vol = (*partab.jobtab).rvol;
                                    (*curframe).uci = (*partab.jobtab).ruci;
                                    (*curframe)
                                        .flags = ((*curframe).flags as libc::c_int
                                        | 2 as libc::c_int) as u_char;
                                    current_block_2879 = 809706028127738298;
                                }
                            } else {
                                current_block_2879 = 809706028127738298;
                            }
                            match current_block_2879 {
                                16169013146210219323 => {}
                                _ => {
                                    (*curframe)
                                        .pc = &mut *(rouadd as *mut u_char)
                                        .offset((*rouadd).code as isize) as *mut u_char;
                                    if var_empty(tag) == 0 {
                                        ttbl = &mut *(rouadd as *mut u_char)
                                            .offset((*rouadd).tag_tbl as isize) as *mut u_char
                                            as *mut tags;
                                        j = 0 as libc::c_int;
                                        i = 0 as libc::c_int;
                                        while i < (*rouadd).num_tags as libc::c_int {
                                            if var_equal((*ttbl.offset(i as isize)).name, tag) != 0 {
                                                (*curframe)
                                                    .pc = ((*curframe).pc)
                                                    .offset(
                                                        (*ttbl.offset(i as isize)).code as libc::c_int as isize,
                                                    );
                                                j = 1 as libc::c_int;
                                                break;
                                            } else {
                                                i += 1;
                                                i;
                                            }
                                        }
                                        if j == 0 as libc::c_int {
                                            (*partab.jobtab)
                                                .async_error = -(13 as libc::c_int) as libc::c_short;
                                            (*partab.jobtab).attention = 1 as libc::c_int;
                                            current_block_2879 = 16169013146210219323;
                                        } else {
                                            while offset != 0 {
                                                i = 0 as libc::c_int;
                                                if *(*curframe).pc as libc::c_int == 171 as libc::c_int {
                                                    i = *((*curframe).pc).offset(1 as libc::c_int as isize)
                                                        as libc::c_int + 2 as libc::c_int;
                                                }
                                                if *((*curframe).pc).offset(i as isize) as libc::c_int
                                                    != 170 as libc::c_int
                                                {
                                                    (*partab.jobtab)
                                                        .async_error = -(13 as libc::c_int) as libc::c_short;
                                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                                    break;
                                                } else {
                                                    (*curframe)
                                                        .pc = &mut *((*curframe).pc)
                                                        .offset((i + 3 as libc::c_int) as isize) as *mut u_char;
                                                    i = *((*curframe).pc as *mut u_short) as libc::c_int;
                                                    (*curframe)
                                                        .pc = &mut *((*curframe).pc)
                                                        .offset((i + 1 as libc::c_int) as isize) as *mut u_char;
                                                    offset -= 1;
                                                    offset;
                                                }
                                            }
                                            current_block_2879 = 17349428871623779863;
                                        }
                                    } else {
                                        current_block_2879 = 17349428871623779863;
                                    }
                                    match current_block_2879 {
                                        16169013146210219323 => {}
                                        _ => {
                                            (*curframe).line_num = 1 as libc::c_int as u_short;
                                            p = (*curframe).pc;
                                            let fresh212 = p;
                                            p = p.offset(1);
                                            if *fresh212 as libc::c_int == 170 as libc::c_int {
                                                memcpy(
                                                    &mut us as *mut u_short as *mut libc::c_void,
                                                    p as *const libc::c_void,
                                                    ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                                );
                                                p = p
                                                    .offset(
                                                        ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                                                    );
                                                (*curframe).line_num = us;
                                                p = p
                                                    .offset(
                                                        ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                                                    );
                                            }
                                            i = 0 as libc::c_int;
                                            let fresh213 = p;
                                            p = p.offset(1);
                                            if *fresh213 as libc::c_int == 156 as libc::c_int {
                                                i = *p as libc::c_int;
                                            }
                                            if (*curframe).level as libc::c_int != i {
                                                (*partab.jobtab)
                                                    .async_error = -(45 as libc::c_int) as libc::c_short;
                                                (*partab.jobtab).attention = 1 as libc::c_int;
                                            } else {
                                                if ((*curframe).symbol).is_null()
                                                    && (*rouadd).num_vars as libc::c_int != 0
                                                {
                                                    (*curframe)
                                                        .symbol = malloc(
                                                        ((*rouadd).num_vars as libc::c_ulong)
                                                            .wrapping_mul(
                                                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                                                            ),
                                                    ) as *mut libc::c_short;
                                                    i = 0 as libc::c_int;
                                                    while i < (*rouadd).num_vars as libc::c_int {
                                                        *((*curframe).symbol)
                                                            .offset(i as isize) = -(1 as libc::c_int) as libc::c_short;
                                                        i += 1;
                                                        i;
                                                    }
                                                }
                                                rsmpc = (*curframe).pc;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        current_block_2879 = 16169013146210219323;
                    }
                }
            }
            6240798591469247094 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                offset = 0 as libc::c_int;
                let mut var_i_14: u_int = 0 as libc::c_int as u_int;
                while var_i_14 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    rou
                        .var_qu[var_i_14
                        as usize] = (*partab.jobtab)
                        .dostk[(*partab.jobtab).cur_do as usize]
                        .rounam
                        .var_qu[var_i_14 as usize];
                    var_i_14 = var_i_14.wrapping_add(1);
                    var_i_14;
                }
                let mut var_i_15: u_int = 0 as libc::c_int as u_int;
                while var_i_15 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    tag.var_qu[var_i_15 as usize] = 0 as libc::c_int as u_int64;
                    var_i_15 = var_i_15.wrapping_add(1);
                    var_i_15;
                }
                if opc == 147 as libc::c_int || opc == 148 as libc::c_int
                    || opc == 149 as libc::c_int
                {
                    memcpy(
                        &mut rou as *mut var_u as *mut libc::c_void,
                        rsmpc as *const libc::c_void,
                        32 as libc::c_int as libc::c_ulong,
                    );
                    rsmpc = rsmpc.offset(32 as libc::c_int as isize);
                }
                if opc == 149 as libc::c_int && var_empty(rou) != 0 {
                    let mut var_i_16: u_int = 0 as libc::c_int as u_int;
                    while var_i_16 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        rou
                            .var_qu[var_i_16
                            as usize] = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .rounam
                            .var_qu[var_i_16 as usize];
                        var_i_16 = var_i_16.wrapping_add(1);
                        var_i_16;
                    }
                }
                if opc == 146 as libc::c_int || opc == 148 as libc::c_int
                    || opc == 149 as libc::c_int
                {
                    memcpy(
                        &mut tag as *mut var_u as *mut libc::c_void,
                        rsmpc as *const libc::c_void,
                        32 as libc::c_int as libc::c_ulong,
                    );
                    rsmpc = rsmpc.offset(32 as libc::c_int as isize);
                }
                j = -(1 as libc::c_int);
                if opc == 149 as libc::c_int {
                    if (*systab).historic & 2 as libc::c_int == 0 {
                        (*partab.jobtab)
                            .async_error = -(70 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                        current_block_2879 = 16169013146210219323;
                    } else {
                        memcpy(
                            &mut us as *mut u_short as *mut libc::c_void,
                            rsmpc as *const libc::c_void,
                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                        );
                        rsmpc = rsmpc
                            .offset(
                                ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                            );
                        offset = us as libc::c_int;
                        current_block_2879 = 3392612551464673475;
                    }
                } else {
                    current_block_2879 = 3392612551464673475;
                }
                match current_block_2879 {
                    16169013146210219323 => {}
                    _ => {
                        let fresh198 = rsmpc;
                        rsmpc = rsmpc.offset(1);
                        args = *fresh198 as libc::c_int;
                        if args & 128 as libc::c_int != 0 {
                            asp -= 1;
                            j = cstringtoi(
                                *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring,
                            );
                            args &= 127 as libc::c_int;
                            (*partab.jobtab).test = 1 as libc::c_int as u_char;
                        }
                        i = ForkIt(0 as libc::c_int);
                        if i > 0 as libc::c_int {
                            current_block_2879 = 16169013146210219323;
                        } else {
                            if i == 0 as libc::c_int {
                                if *__error() != 0 {
                                    (*partab.jobtab)
                                        .async_error = -(200 as libc::c_int + 200 as libc::c_int
                                        + *__error()) as libc::c_short;
                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                } else {
                                    (*partab.jobtab)
                                        .async_error = -(49 as libc::c_int + 200 as libc::c_int)
                                        as libc::c_short;
                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                }
                            } else {
                                i = 0 as libc::c_int;
                                let fresh199 = i;
                                i = i + 1;
                                *strstk
                                    .as_mut_ptr()
                                    .offset(fresh199 as isize) = 'D' as i32 as u_char;
                                let fresh200 = i;
                                i = i + 1;
                                *strstk
                                    .as_mut_ptr()
                                    .offset(fresh200 as isize) = ' ' as i32 as u_char;
                                list = &mut tag as *mut var_u;
                                j = 0 as libc::c_int;
                                while j < 32 as libc::c_int
                                    && (*list).var_cu[j as usize] as libc::c_int
                                        != 0 as libc::c_int
                                {
                                    let fresh201 = j;
                                    j = j + 1;
                                    let fresh202 = i;
                                    i = i + 1;
                                    *strstk
                                        .as_mut_ptr()
                                        .offset(
                                            fresh202 as isize,
                                        ) = (*list).var_cu[fresh201 as usize];
                                }
                                if offset != 0 {
                                    let fresh203 = i;
                                    i = i + 1;
                                    *strstk
                                        .as_mut_ptr()
                                        .offset(fresh203 as isize) = '+' as i32 as u_char;
                                    i
                                        += itocstring(
                                            &mut *strstk.as_mut_ptr().offset(i as isize),
                                            offset,
                                        ) as libc::c_int;
                                }
                                let fresh204 = i;
                                i = i + 1;
                                *strstk
                                    .as_mut_ptr()
                                    .offset(fresh204 as isize) = '^' as i32 as u_char;
                                list = &mut rou as *mut var_u;
                                j = 0 as libc::c_int;
                                while j < 32 as libc::c_int
                                    && (*list).var_cu[j as usize] as libc::c_int
                                        != 0 as libc::c_int
                                {
                                    let fresh205 = j;
                                    j = j + 1;
                                    let fresh206 = i;
                                    i = i + 1;
                                    *strstk
                                        .as_mut_ptr()
                                        .offset(
                                            fresh206 as isize,
                                        ) = (*list).var_cu[fresh205 as usize];
                                }
                                if args != 0 {
                                    let fresh207 = i;
                                    i = i + 1;
                                    *strstk
                                        .as_mut_ptr()
                                        .offset(fresh207 as isize) = '(' as i32 as u_char;
                                    j = args - 1 as libc::c_int;
                                    while j > 0 as libc::c_int {
                                        let fresh208 = i;
                                        i = i + 1;
                                        *strstk
                                            .as_mut_ptr()
                                            .offset(fresh208 as isize) = '"' as i32 as u_char;
                                        cptr = *addstk.as_mut_ptr().offset((asp - j) as isize)
                                            as *mut cstring;
                                        memmove(
                                            &mut *strstk.as_mut_ptr().offset(i as isize) as *mut u_char
                                                as *mut libc::c_void,
                                            ((*cptr).buf).as_mut_ptr() as *const libc::c_void,
                                            (*cptr).len as libc::c_ulong,
                                        );
                                        i = i + (*cptr).len as libc::c_int;
                                        let fresh209 = i;
                                        i = i + 1;
                                        *strstk
                                            .as_mut_ptr()
                                            .offset(fresh209 as isize) = '"' as i32 as u_char;
                                        let fresh210 = i;
                                        i = i + 1;
                                        *strstk
                                            .as_mut_ptr()
                                            .offset(fresh210 as isize) = ',' as i32 as u_char;
                                        j -= 1;
                                        j;
                                    }
                                    i -= 1;
                                    i;
                                    let fresh211 = i;
                                    i = i + 1;
                                    *strstk
                                        .as_mut_ptr()
                                        .offset(fresh211 as isize) = ')' as i32 as u_char;
                                }
                                *strstk
                                    .as_mut_ptr()
                                    .offset(i as isize) = '\0' as i32 as u_char;
                                return 512 as libc::c_int as libc::c_short;
                            }
                            current_block_2879 = 16169013146210219323;
                        }
                    }
                }
            }
            7162237048274468123 => {
                current_block_2879 = 538692631886353810;
            }
            18310540048570063734 => {
                current_block_2879 = 15420190669783501340;
            }
            _ => {}
        }
        match current_block_2879 {
            538692631886353810 => {
                current_block_2879 = 3911658486967328820;
            }
            15420190669783501340 => {
                current_block_2879 = 1974722652628827707;
            }
            _ => {}
        }
        match current_block_2879 {
            3911658486967328820 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                offset = 0 as libc::c_int;
                let mut var_i_4: u_int = 0 as libc::c_int as u_int;
                while var_i_4 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                    tag.var_qu[var_i_4 as usize] = 0 as libc::c_int as u_int64;
                    var_i_4 = var_i_4.wrapping_add(1);
                    var_i_4;
                }
                if opc == 140 as libc::c_int {
                    let mut var_i_5: u_int = 0 as libc::c_int as u_int;
                    while var_i_5 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        rou
                            .var_qu[var_i_5
                            as usize] = (*partab.jobtab)
                            .seqio[(*partab.jobtab).io as usize]
                            .namespace
                            .var_qu[var_i_5 as usize];
                        var_i_5 = var_i_5.wrapping_add(1);
                        var_i_5;
                    }
                    current_block_2879 = 12033850209884029991;
                } else {
                    let mut var_i_6: u_int = 0 as libc::c_int as u_int;
                    while var_i_6 < (32 as libc::c_int / 8 as libc::c_int) as u_int {
                        rou
                            .var_qu[var_i_6
                            as usize] = (*partab.jobtab)
                            .dostk[(*partab.jobtab).cur_do as usize]
                            .rounam
                            .var_qu[var_i_6 as usize];
                        var_i_6 = var_i_6.wrapping_add(1);
                        var_i_6;
                    }
                    if opc == 142 as libc::c_int || opc == 143 as libc::c_int
                        || opc == 144 as libc::c_int
                    {
                        if *rsmpc as libc::c_int == 2 as libc::c_int {
                            rsmpc = rsmpc.offset(1);
                            s = *(rsmpc as *mut libc::c_short);
                            (*partab.jobtab).async_error = s;
                            (*partab.jobtab).attention = 1 as libc::c_int;
                            current_block_2879 = 16169013146210219323;
                        } else {
                            memcpy(
                                &mut rou as *mut var_u as *mut libc::c_void,
                                rsmpc as *const libc::c_void,
                                32 as libc::c_int as libc::c_ulong,
                            );
                            rsmpc = rsmpc.offset(32 as libc::c_int as isize);
                            current_block_2879 = 15553434323870277286;
                        }
                    } else {
                        current_block_2879 = 15553434323870277286;
                    }
                    match current_block_2879 {
                        16169013146210219323 => {}
                        _ => {
                            if opc == 144 as libc::c_int && var_empty(rou) != 0 {
                                let mut var_i_7: u_int = 0 as libc::c_int as u_int;
                                while var_i_7
                                    < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                {
                                    rou
                                        .var_qu[var_i_7
                                        as usize] = (*partab.jobtab)
                                        .dostk[(*partab.jobtab).cur_do as usize]
                                        .rounam
                                        .var_qu[var_i_7 as usize];
                                    var_i_7 = var_i_7.wrapping_add(1);
                                    var_i_7;
                                }
                            }
                            current_block_2879 = 12033850209884029991;
                        }
                    }
                }
                match current_block_2879 {
                    16169013146210219323 => {}
                    _ => {
                        if opc == 141 as libc::c_int || opc == 143 as libc::c_int
                            || opc == 144 as libc::c_int || opc == 140 as libc::c_int
                        {
                            if *rsmpc as libc::c_int == 2 as libc::c_int {
                                rsmpc = rsmpc.offset(1);
                                s = *(rsmpc as *mut libc::c_short);
                                (*partab.jobtab).async_error = s;
                                (*partab.jobtab).attention = 1 as libc::c_int;
                                current_block_2879 = 16169013146210219323;
                            } else {
                                memcpy(
                                    &mut tag as *mut var_u as *mut libc::c_void,
                                    rsmpc as *const libc::c_void,
                                    32 as libc::c_int as libc::c_ulong,
                                );
                                rsmpc = rsmpc.offset(32 as libc::c_int as isize);
                                current_block_2879 = 15852491067404001676;
                            }
                        } else {
                            current_block_2879 = 15852491067404001676;
                        }
                        match current_block_2879 {
                            16169013146210219323 => {}
                            _ => {
                                if opc == 144 as libc::c_int {
                                    if (*systab).historic & 2 as libc::c_int == 0 {
                                        (*partab.jobtab)
                                            .async_error = -(70 as libc::c_int + 200 as libc::c_int)
                                            as libc::c_short;
                                        (*partab.jobtab).attention = 1 as libc::c_int;
                                        current_block_2879 = 16169013146210219323;
                                    } else {
                                        memcpy(
                                            &mut us as *mut u_short as *mut libc::c_void,
                                            rsmpc as *const libc::c_void,
                                            ::core::mem::size_of::<u_short>() as libc::c_ulong,
                                        );
                                        rsmpc = rsmpc
                                            .offset(
                                                ::core::mem::size_of::<u_short>() as libc::c_ulong as isize,
                                            );
                                        offset = us as libc::c_int;
                                        current_block_2879 = 18208734352373644153;
                                    }
                                } else {
                                    current_block_2879 = 18208734352373644153;
                                }
                                match current_block_2879 {
                                    16169013146210219323 => {}
                                    _ => {
                                        args = 0 as libc::c_int;
                                        if opc != 145 as libc::c_int {
                                            let fresh194 = rsmpc;
                                            rsmpc = rsmpc.offset(1);
                                            args = *fresh194 as libc::c_int;
                                        }
                                        if (args != 0 || var_empty(tag) != 0) && offset != 0 {
                                            (*partab.jobtab)
                                                .async_error = -(20 as libc::c_int) as libc::c_short;
                                            (*partab.jobtab).attention = 1 as libc::c_int;
                                        } else if (*partab.jobtab).cur_do + 1 as libc::c_int
                                            == 128 as libc::c_int
                                        {
                                            (*partab.jobtab)
                                                .async_error = -(7 as libc::c_int + 200 as libc::c_int)
                                                as libc::c_short;
                                            (*partab.jobtab).attention = 1 as libc::c_int;
                                        } else {
                                            if var_empty(rou) != 0 && opc != 145 as libc::c_int {
                                                i = (*partab.jobtab).cur_do - 1 as libc::c_int;
                                                while i > 0 as libc::c_int {
                                                    if var_empty((*partab.jobtab).dostk[i as usize].rounam) == 0
                                                    {
                                                        let mut var_i_8: u_int = 0 as libc::c_int as u_int;
                                                        while var_i_8
                                                            < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                                        {
                                                            rou
                                                                .var_qu[var_i_8
                                                                as usize] = (*partab.jobtab)
                                                                .dostk[i as usize]
                                                                .rounam
                                                                .var_qu[var_i_8 as usize];
                                                            var_i_8 = var_i_8.wrapping_add(1);
                                                            var_i_8;
                                                        }
                                                        break;
                                                    } else {
                                                        i -= 1;
                                                        i;
                                                    }
                                                }
                                            }
                                            if var_empty(rou) != 0 {
                                                (*partab.jobtab)
                                                    .async_error = -(13 as libc::c_int) as libc::c_short;
                                                (*partab.jobtab).attention = 1 as libc::c_int;
                                            } else {
                                                (*partab.jobtab)
                                                    .dostk[(*partab.jobtab).cur_do as usize]
                                                    .pc = rsmpc;
                                                (*partab.jobtab).cur_do += 1;
                                                (*partab.jobtab).cur_do;
                                                curframe = &mut *((*partab.jobtab).dostk)
                                                    .as_mut_ptr()
                                                    .offset((*partab.jobtab).cur_do as isize) as *mut do_frame;
                                                rouadd = 0 as *mut rbd;
                                                i = (*partab.jobtab).cur_do - 1 as libc::c_int;
                                                while i > 0 as libc::c_int {
                                                    if var_equal(rou, (*partab.jobtab).dostk[i as usize].rounam)
                                                        != 0
                                                        && (*partab.jobtab).ruci as libc::c_int
                                                            == (*partab.jobtab).dostk[i as usize].uci as libc::c_int
                                                        && (*partab.jobtab).rvol as libc::c_int
                                                            == (*partab.jobtab).dostk[i as usize].vol as libc::c_int
                                                    {
                                                        memcpy(
                                                            curframe as *mut libc::c_void,
                                                            &mut *((*partab.jobtab).dostk)
                                                                .as_mut_ptr()
                                                                .offset(i as isize) as *mut do_frame as *const libc::c_void,
                                                            ::core::mem::size_of::<do_frame>() as libc::c_ulong,
                                                        );
                                                        (*curframe).flags = 0 as libc::c_int as u_char;
                                                        rouadd = (*partab.jobtab).dostk[i as usize].routine
                                                            as *mut rbd;
                                                        break;
                                                    } else {
                                                        i -= 1;
                                                        i;
                                                    }
                                                }
                                                if rouadd.is_null() {
                                                    rouadd = Routine_Attach(rou);
                                                    if rouadd.is_null() {
                                                        (*partab.jobtab).cur_do -= 1;
                                                        (*partab.jobtab).cur_do;
                                                        (*partab.jobtab)
                                                            .async_error = -(13 as libc::c_int) as libc::c_short;
                                                        (*partab.jobtab).attention = 1 as libc::c_int;
                                                        current_block_2879 = 16169013146210219323;
                                                    } else if rouadd == -(1 as libc::c_int) as *mut rbd {
                                                        (*partab.jobtab).cur_do -= 1;
                                                        (*partab.jobtab).cur_do;
                                                        (*partab.jobtab)
                                                            .async_error = -(52 as libc::c_int + 200 as libc::c_int)
                                                            as libc::c_short;
                                                        (*partab.jobtab).attention = 1 as libc::c_int;
                                                        current_block_2879 = 16169013146210219323;
                                                    } else if rouadd == -(2 as libc::c_int) as *mut rbd {
                                                        (*partab.jobtab).cur_do -= 1;
                                                        (*partab.jobtab).cur_do;
                                                        (*partab.jobtab)
                                                            .async_error = -(59 as libc::c_int + 200 as libc::c_int)
                                                            as libc::c_short;
                                                        (*partab.jobtab).attention = 1 as libc::c_int;
                                                        current_block_2879 = 16169013146210219323;
                                                    } else {
                                                        (*curframe).routine = rouadd as *mut u_char;
                                                        (*curframe).symbol = 0 as *mut libc::c_short;
                                                        let mut var_i_9: u_int = 0 as libc::c_int as u_int;
                                                        while var_i_9
                                                            < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                                        {
                                                            (*curframe)
                                                                .rounam
                                                                .var_qu[var_i_9 as usize] = rou.var_qu[var_i_9 as usize];
                                                            var_i_9 = var_i_9.wrapping_add(1);
                                                            var_i_9;
                                                        }
                                                        (*curframe).vol = (*partab.jobtab).rvol;
                                                        (*curframe).uci = (*partab.jobtab).ruci;
                                                        (*curframe).flags = 2 as libc::c_int as u_char;
                                                        current_block_2879 = 2096804162013291468;
                                                    }
                                                } else {
                                                    current_block_2879 = 2096804162013291468;
                                                }
                                                match current_block_2879 {
                                                    16169013146210219323 => {}
                                                    _ => {
                                                        if infor != 0 {
                                                            (*curframe)
                                                                .flags = ((*curframe).flags as libc::c_int
                                                                | 4 as libc::c_int) as u_char;
                                                            infor = 0 as libc::c_int;
                                                        }
                                                        (*curframe)
                                                            .pc = &mut *(rouadd as *mut u_char)
                                                            .offset((*rouadd).code as isize) as *mut u_char;
                                                        if var_empty(tag) == 0 {
                                                            ttbl = &mut *(rouadd as *mut u_char)
                                                                .offset((*rouadd).tag_tbl as isize) as *mut u_char
                                                                as *mut tags;
                                                            j = 0 as libc::c_int;
                                                            i = 0 as libc::c_int;
                                                            while i < (*rouadd).num_tags as libc::c_int {
                                                                if var_equal((*ttbl.offset(i as isize)).name, tag) != 0 {
                                                                    (*curframe)
                                                                        .pc = ((*curframe).pc)
                                                                        .offset(
                                                                            (*ttbl.offset(i as isize)).code as libc::c_int as isize,
                                                                        );
                                                                    j = 1 as libc::c_int;
                                                                    break;
                                                                } else {
                                                                    i += 1;
                                                                    i;
                                                                }
                                                            }
                                                            if j == 0 as libc::c_int {
                                                                (*partab.jobtab).cur_do -= 1;
                                                                (*partab.jobtab).cur_do;
                                                                (*partab.jobtab)
                                                                    .async_error = -(13 as libc::c_int) as libc::c_short;
                                                                (*partab.jobtab).attention = 1 as libc::c_int;
                                                                current_block_2879 = 16169013146210219323;
                                                            } else {
                                                                while offset != 0 {
                                                                    i = 0 as libc::c_int;
                                                                    if *(*curframe).pc as libc::c_int == 171 as libc::c_int {
                                                                        i = *((*curframe).pc).offset(1 as libc::c_int as isize)
                                                                            as libc::c_int + 2 as libc::c_int;
                                                                    }
                                                                    if *((*curframe).pc).offset(i as isize) as libc::c_int
                                                                        != 170 as libc::c_int
                                                                    {
                                                                        (*partab.jobtab).cur_do -= 1;
                                                                        (*partab.jobtab).cur_do;
                                                                        (*partab.jobtab)
                                                                            .async_error = -(13 as libc::c_int) as libc::c_short;
                                                                        (*partab.jobtab).attention = 1 as libc::c_int;
                                                                        break;
                                                                    } else {
                                                                        (*curframe)
                                                                            .pc = &mut *((*curframe).pc)
                                                                            .offset((i + 3 as libc::c_int) as isize) as *mut u_char;
                                                                        i = *((*curframe).pc as *mut u_short) as libc::c_int;
                                                                        (*curframe)
                                                                            .pc = &mut *((*curframe).pc)
                                                                            .offset((i + 1 as libc::c_int) as isize) as *mut u_char;
                                                                        offset -= 1;
                                                                        offset;
                                                                    }
                                                                }
                                                                current_block_2879 = 11239778701799712862;
                                                            }
                                                        } else {
                                                            current_block_2879 = 11239778701799712862;
                                                        }
                                                        match current_block_2879 {
                                                            16169013146210219323 => {}
                                                            _ => {
                                                                (*curframe).newtab = 0 as *mut u_char;
                                                                (*curframe)
                                                                    .estack = (*partab.jobtab)
                                                                    .dostk[((*partab.jobtab).cur_do - 1 as libc::c_int)
                                                                        as usize]
                                                                    .estack;
                                                                (*curframe).line_num = 1 as libc::c_int as u_short;
                                                                (*curframe)
                                                                    .type_0 = (if args & 128 as libc::c_int != 0 {
                                                                    4 as libc::c_int
                                                                } else {
                                                                    3 as libc::c_int
                                                                }) as u_char;
                                                                (*curframe).level = 0 as libc::c_int as u_char;
                                                                if opc == 145 as libc::c_int {
                                                                    (*curframe)
                                                                        .level = ((*partab.jobtab)
                                                                        .dostk[((*partab.jobtab).cur_do - 1 as libc::c_int)
                                                                            as usize]
                                                                        .level as libc::c_int + 1 as libc::c_int) as u_char;
                                                                    (*curframe)
                                                                        .pc = (*partab.jobtab)
                                                                        .dostk[((*partab.jobtab).cur_do - 1 as libc::c_int)
                                                                            as usize]
                                                                        .endlin;
                                                                    p = (*curframe).pc;
                                                                    if *p == 0 {
                                                                        p = p.offset(1);
                                                                        p;
                                                                    }
                                                                    if *p as libc::c_int == 170 as libc::c_int {
                                                                        p = p
                                                                            .offset(
                                                                                (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                                                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                                                            );
                                                                    }
                                                                    if *p as libc::c_int != 156 as libc::c_int {
                                                                        (*partab.jobtab).cur_do -= 1;
                                                                        (*partab.jobtab).cur_do;
                                                                        rsmpc = (*partab.jobtab)
                                                                            .dostk[(*partab.jobtab).cur_do as usize]
                                                                            .pc;
                                                                        current_block_2879 = 16169013146210219323;
                                                                    } else {
                                                                        current_block_2879 = 17669275503726800671;
                                                                    }
                                                                } else {
                                                                    current_block_2879 = 17669275503726800671;
                                                                }
                                                                match current_block_2879 {
                                                                    16169013146210219323 => {}
                                                                    _ => {
                                                                        if ((*curframe).symbol).is_null()
                                                                            && (*rouadd).num_vars as libc::c_int != 0
                                                                        {
                                                                            (*curframe)
                                                                                .symbol = malloc(
                                                                                ((*rouadd).num_vars as libc::c_ulong)
                                                                                    .wrapping_mul(
                                                                                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                                                                                    ),
                                                                            ) as *mut libc::c_short;
                                                                            i = 0 as libc::c_int;
                                                                            while i < (*rouadd).num_vars as libc::c_int {
                                                                                *((*curframe).symbol)
                                                                                    .offset(i as isize) = -(1 as libc::c_int) as libc::c_short;
                                                                                i += 1;
                                                                                i;
                                                                            }
                                                                        }
                                                                        rsmpc = (*curframe).pc;
                                                                        args &= 127 as libc::c_int;
                                                                        if args > 0 as libc::c_int {
                                                                            let fresh195 = rsmpc;
                                                                            rsmpc = rsmpc.offset(1);
                                                                            if *fresh195 as libc::c_int != 171 as libc::c_int {
                                                                                if !((*curframe).symbol).is_null() {
                                                                                    free((*curframe).symbol as *mut libc::c_void);
                                                                                    (*curframe).symbol = 0 as *mut libc::c_short;
                                                                                }
                                                                                (*partab.jobtab).cur_do -= 1;
                                                                                (*partab.jobtab).cur_do;
                                                                                s = -(58 as libc::c_int) as libc::c_short;
                                                                                rsmpc = rsmpc.offset(-1);
                                                                                if *rsmpc as libc::c_int == 2 as libc::c_int {
                                                                                    rsmpc = rsmpc.offset(1);
                                                                                    rsmpc;
                                                                                    s = *(rsmpc as *mut libc::c_short);
                                                                                }
                                                                                (*partab.jobtab).async_error = s;
                                                                                (*partab.jobtab).attention = 1 as libc::c_int;
                                                                                current_block_2879 = 16169013146210219323;
                                                                            } else {
                                                                                let fresh196 = rsmpc;
                                                                                rsmpc = rsmpc.offset(1);
                                                                                j = *fresh196 as libc::c_int;
                                                                                if args - 1 as libc::c_int > j {
                                                                                    (*curframe).symbol = 0 as *mut libc::c_short;
                                                                                    (*partab.jobtab).cur_do -= 1;
                                                                                    (*partab.jobtab).cur_do;
                                                                                    (*partab.jobtab)
                                                                                        .async_error = -(58 as libc::c_int) as libc::c_short;
                                                                                    (*partab.jobtab).attention = 1 as libc::c_int;
                                                                                    current_block_2879 = 16169013146210219323;
                                                                                } else {
                                                                                    list = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                                                                                        as *mut u_char as *mut var_u;
                                                                                    let mut var_i_10: u_int = 0 as libc::c_int as u_int;
                                                                                    while var_i_10
                                                                                        < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                                                                    {
                                                                                        (*list)
                                                                                            .var_qu[var_i_10 as usize] = 0 as libc::c_int as u_int64;
                                                                                        var_i_10 = var_i_10.wrapping_add(1);
                                                                                        var_i_10;
                                                                                    }
                                                                                    vt = (rouadd as *mut u_char)
                                                                                        .offset((*rouadd).var_tbl as libc::c_int as isize)
                                                                                        as *mut var_u;
                                                                                    i = 0 as libc::c_int;
                                                                                    while i < j {
                                                                                        let mut var_i_11: u_int = 0 as libc::c_int as u_int;
                                                                                        while var_i_11
                                                                                            < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                                                                        {
                                                                                            (*list.offset(i as isize))
                                                                                                .var_qu[var_i_11
                                                                                                as usize] = (*vt.offset(*rsmpc.offset(i as isize) as isize))
                                                                                                .var_qu[var_i_11 as usize];
                                                                                            var_i_11 = var_i_11.wrapping_add(1);
                                                                                            var_i_11;
                                                                                        }
                                                                                        i += 1;
                                                                                        i;
                                                                                    }
                                                                                    s = ST_New(j, list);
                                                                                    if (s as libc::c_int) < 0 as libc::c_int {
                                                                                        (*partab.jobtab).async_error = s;
                                                                                        (*partab.jobtab).attention = 1 as libc::c_int;
                                                                                        current_block_2879 = 16169013146210219323;
                                                                                    } else {
                                                                                        var = &mut *strstk.as_mut_ptr().offset(ssp as isize)
                                                                                            as *mut u_char as *mut mvar;
                                                                                        let mut var_i_12: u_int = 0 as libc::c_int as u_int;
                                                                                        while var_i_12
                                                                                            < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                                                                        {
                                                                                            (*var)
                                                                                                .name
                                                                                                .var_qu[var_i_12 as usize] = 0 as libc::c_int as u_int64;
                                                                                            var_i_12 = var_i_12.wrapping_add(1);
                                                                                            var_i_12;
                                                                                        }
                                                                                        (*var).uci = 255 as libc::c_int as u_char;
                                                                                        (*var).slen = 0 as libc::c_int as u_char;
                                                                                        t = 0 as libc::c_int;
                                                                                        i = args - 2 as libc::c_int;
                                                                                        while i >= 0 as libc::c_int {
                                                                                            (*var)
                                                                                                .volset = (*rsmpc.offset(i as isize) as libc::c_int
                                                                                                + 1 as libc::c_int) as u_char;
                                                                                            asp -= 1;
                                                                                            cptr = *addstk.as_mut_ptr().offset(asp as isize)
                                                                                                as *mut cstring;
                                                                                            if !cptr.is_null() {
                                                                                                if (*cptr).len as libc::c_int
                                                                                                    != 65534 as libc::c_int + 1 as libc::c_int
                                                                                                {
                                                                                                    t = ST_Set(var, cptr);
                                                                                                    if t < 0 as libc::c_int {
                                                                                                        break;
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                asp -= 1;
                                                                                                p = *addstk.as_mut_ptr().offset(asp as isize);
                                                                                                asp -= 1;
                                                                                                cptr = *addstk.as_mut_ptr().offset(asp as isize)
                                                                                                    as *mut cstring;
                                                                                                (*var).name = (*(cptr as *mut mvar)).name;
                                                                                                t = ST_ConData(var, p) as libc::c_int;
                                                                                                if t < 0 as libc::c_int {
                                                                                                    break;
                                                                                                }
                                                                                                let mut var_i_13: u_int = 0 as libc::c_int as u_int;
                                                                                                while var_i_13
                                                                                                    < (32 as libc::c_int / 8 as libc::c_int) as u_int
                                                                                                {
                                                                                                    (*var)
                                                                                                        .name
                                                                                                        .var_qu[var_i_13 as usize] = 0 as libc::c_int as u_int64;
                                                                                                    var_i_13 = var_i_13.wrapping_add(1);
                                                                                                    var_i_13;
                                                                                                }
                                                                                            }
                                                                                            i -= 1;
                                                                                            i;
                                                                                        }
                                                                                        if t < 0 as libc::c_int {
                                                                                            (*partab.jobtab).async_error = t as libc::c_short;
                                                                                            (*partab.jobtab).attention = 1 as libc::c_int;
                                                                                            current_block_2879 = 16169013146210219323;
                                                                                        } else {
                                                                                            rsmpc = rsmpc.offset(j as isize);
                                                                                            current_block_2879 = 9622094832090675059;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            current_block_2879 = 9622094832090675059;
                                                                        }
                                                                        match current_block_2879 {
                                                                            16169013146210219323 => {}
                                                                            _ => {
                                                                                if (*curframe).type_0 as libc::c_int & 4 as libc::c_int != 0
                                                                                    || (*curframe).level as libc::c_int != 0
                                                                                {
                                                                                    (*curframe)
                                                                                        .flags = ((*curframe).flags as libc::c_int
                                                                                        & !(1 as libc::c_int)) as u_char;
                                                                                    (*curframe)
                                                                                        .flags = ((*curframe).flags as libc::c_int
                                                                                        | (*partab.jobtab).test as libc::c_int) as u_char;
                                                                                }
                                                                                p = rsmpc;
                                                                                if *p == 0 {
                                                                                    p = p.offset(1);
                                                                                    p;
                                                                                }
                                                                                let fresh197 = p;
                                                                                p = p.offset(1);
                                                                                if *fresh197 as libc::c_int == 170 as libc::c_int {
                                                                                    (*curframe).line_num = *(p as *mut u_short);
                                                                                }
                                                                                (*curframe).savasp = savasp as libc::c_long;
                                                                                (*curframe).savssp = savssp as libc::c_long;
                                                                                (*curframe).asp = asp as libc::c_long;
                                                                                (*curframe).ssp = ssp as libc::c_long;
                                                                                (*curframe).isp = isp;
                                                                                savasp = asp;
                                                                                savssp = ssp;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        current_block_2879 = 16169013146210219323;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            1974722652628827707 => {
                current_block_2879 = 17123654937568747916;
            }
            _ => {}
        }
        match current_block_2879 {
            17123654937568747916 => {
                current_block_2879 = 13327607154860047783;
            }
            _ => {}
        }
        match current_block_2879 {
            13327607154860047783 => {
                current_block_2879 = 16115092360499891106;
            }
            _ => {}
        }
        match current_block_2879 {
            16115092360499891106 => {
                current_block_2879 = 8762070809544863061;
            }
            _ => {}
        }
        match current_block_2879 {
            8762070809544863061 => {
                current_block_2879 = 13553578021377382533;
            }
            _ => {}
        }
        match current_block_2879 {
            13553578021377382533 => {
                current_block_2879 = 8590759965554521070;
            }
            _ => {}
        }
        match current_block_2879 {
            8590759965554521070 => {
                current_block_2879 = 14874714617003418979;
            }
            _ => {}
        }
        match current_block_2879 {
            14874714617003418979 => {
                current_block_2879 = 15369043757567109248;
            }
            _ => {}
        }
        match current_block_2879 {
            15369043757567109248 => {
                current_block_2879 = 9658992782896750533;
            }
            _ => {}
        }
        match current_block_2879 {
            9658992782896750533 => {
                current_block_2879 = 14124765838191351603;
            }
            _ => {}
        }
        match current_block_2879 {
            14124765838191351603 => {
                (*partab.jobtab).commands = ((*partab.jobtab).commands).wrapping_add(1);
                (*partab.jobtab).commands;
                asp -= 1;
                cptr = *addstk.as_mut_ptr().offset(asp as isize) as *mut cstring;
                if (((*cptr).len as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(isp as libc::c_ulong)
                    > 65536 as libc::c_int as libc::c_ulong
                {
                    (*partab.jobtab)
                        .async_error = -(58 as libc::c_int + 200 as libc::c_int)
                        as libc::c_short;
                    (*partab.jobtab).attention = 1 as libc::c_int;
                } else {
                    source_ptr = ((*cptr).buf).as_mut_ptr();
                    comp_ptr = &mut *indstk.as_mut_ptr().offset(isp as isize)
                        as *mut u_char;
                    match opc {
                        181 => {
                            parse_close();
                        }
                        182 => {
                            parse_do(1 as libc::c_int);
                        }
                        183 => {
                            parse_goto(1 as libc::c_int);
                        }
                        184 => {
                            parse_hang();
                        }
                        185 => {
                            parse_if(isp);
                        }
                        186 => {
                            parse_job(1 as libc::c_int);
                        }
                        187 => {
                            parse_kill();
                        }
                        188 => {
                            parse_lock();
                        }
                        189 => {
                            parse_merge();
                        }
                        190 => {
                            parse_new();
                        }
                        191 => {
                            parse_open();
                        }
                        192 => {
                            parse_read();
                        }
                        193 => {
                            parse_set();
                        }
                        194 => {
                            parse_use();
                        }
                        195 => {
                            parse_write();
                        }
                        196 => {
                            parse_xecute();
                        }
                        _ => {}
                    }
                    if *source_ptr as libc::c_int != '\0' as i32 {
                        (*partab.jobtab)
                            .async_error = -(57 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else if comp_ptr
                        .offset(
                            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
                        )
                        .offset(1 as libc::c_int as isize)
                        >= &mut *indstk
                            .as_mut_ptr()
                            .offset(65536 as libc::c_int as isize) as *mut u_char
                    {
                        (*partab.jobtab)
                            .async_error = -(58 as libc::c_int + 200 as libc::c_int)
                            as libc::c_short;
                        (*partab.jobtab).attention = 1 as libc::c_int;
                    } else {
                        let fresh235 = comp_ptr;
                        comp_ptr = comp_ptr.offset(1);
                        *fresh235 = 180 as libc::c_int as u_char;
                        memcpy(
                            comp_ptr as *mut libc::c_void,
                            &mut isp as *mut libc::c_long as *const libc::c_void,
                            ::core::mem::size_of::<libc::c_long>() as libc::c_ulong,
                        );
                        comp_ptr = comp_ptr
                            .offset(
                                ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                                    as isize,
                            );
                        memcpy(
                            comp_ptr as *mut libc::c_void,
                            &mut rsmpc as *mut *mut u_char as *const libc::c_void,
                            ::core::mem::size_of::<*mut u_char>() as libc::c_ulong,
                        );
                        comp_ptr = comp_ptr
                            .offset(
                                ::core::mem::size_of::<*mut u_char>() as libc::c_ulong
                                    as isize,
                            );
                        rsmpc = &mut *indstk.as_mut_ptr().offset(isp as isize)
                            as *mut u_char;
                        isp = comp_ptr
                            .offset_from(
                                &mut *indstk.as_mut_ptr().offset(isp as isize)
                                    as *mut u_char,
                            ) as libc::c_long + isp;
                    }
                }
            }
            _ => {}
        }
    };
}
