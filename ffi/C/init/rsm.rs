#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __sFILEX;
    static mut __stdinp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fclose(_: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(_: libc::c_int) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(_: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn getopt(
        _: libc::c_int,
        _: *const *mut libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn short_version(ret_buffer: *mut u_char, i: libc::c_int) -> libc::c_int;
    fn INIT_Create_File(
        blocks: u_int,
        bsize: u_int,
        map: u_int,
        volnam: *mut libc::c_char,
        env: *mut libc::c_char,
        file: *mut libc::c_char,
    ) -> libc::c_int;
    fn INIT_Start(
        file: *mut libc::c_char,
        jobs: u_int,
        gmb: u_int,
        rmb: u_int,
        addmb: u_int,
    ) -> libc::c_int;
    fn INIT_Run(
        file: *mut libc::c_char,
        env: *mut libc::c_char,
        cmd: *mut libc::c_char,
    ) -> libc::c_int;
    fn help();
    fn info(file: *mut libc::c_char);
    fn shutdown(file: *mut libc::c_char);
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
pub type u_char = libc::c_uchar;
pub type u_int = libc::c_uint;
#[no_mangle]
pub static mut restricted: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut bsize: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut k: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gmb: libc::c_int = 0 as libc::c_int;
    let mut jobs: libc::c_int = 0 as libc::c_int;
    let mut map: libc::c_int = 0 as libc::c_int;
    let mut rmb: libc::c_int = 0 as libc::c_int;
    let mut addmb: libc::c_int = 0 as libc::c_int;
    let mut blocks: libc::c_int = 0 as libc::c_int;
    let mut volnam: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dbfile: *mut libc::c_char = getenv(
        b"RSM_DBFILE\0" as *const u8 as *const libc::c_char,
    );
    let mut file: [libc::c_char; 256] = [0; 256];
    let mut version: [libc::c_char; 20] = [0; 20];
    if argc < 2 as libc::c_int && dbfile.is_null() {
        help();
    }
    loop {
        c = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"a:b:e:g:hij:km:r:s:v:x:RV\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            97 => {
                addmb = atoi(optarg);
            }
            98 => {
                bsize = atoi(optarg);
            }
            101 => {
                env = optarg;
            }
            103 => {
                gmb = atoi(optarg);
            }
            104 => {
                if i == 0 && k == 0 {
                    help();
                }
            }
            105 => {
                i = 1 as libc::c_int as libc::c_char;
            }
            106 => {
                jobs = atoi(optarg);
            }
            107 => {
                k = 1 as libc::c_int as libc::c_char;
            }
            109 => {
                map = atoi(optarg);
            }
            114 => {
                rmb = atoi(optarg);
            }
            115 => {
                blocks = atoi(optarg);
            }
            118 => {
                volnam = optarg;
            }
            120 => {
                if cmd.is_null() {
                    cmd = optarg;
                }
            }
            82 => {
                restricted = 1 as libc::c_int;
            }
            86 => {
                if !(i as libc::c_int != 0 || k as libc::c_int != 0) {
                    short_version(
                        version.as_mut_ptr() as *mut u_char,
                        sprintf(
                            &mut *version.as_mut_ptr().offset(0 as libc::c_int as isize)
                                as *mut libc::c_char,
                            b"V\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    printf(
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        version.as_mut_ptr(),
                    );
                    exit(0 as libc::c_int);
                }
            }
            _ => {
                putchar('\n' as i32);
                help();
            }
        }
    }
    argc -= optind;
    argv = argv.offset(optind as isize);
    if argc == 1 as libc::c_int {
        strcpy(file.as_mut_ptr(), *argv);
    } else if !dbfile.is_null() {
        strcpy(file.as_mut_ptr(), dbfile);
    } else {
        if i != 0 {
            info(0 as *mut libc::c_char);
        }
        if k != 0 {
            shutdown(0 as *mut libc::c_char);
        }
        help();
    }
    if i != 0 {
        info(file.as_mut_ptr());
    }
    if k != 0 {
        shutdown(file.as_mut_ptr());
    }
    if !volnam.is_null() {
        exit(
            INIT_Create_File(
                blocks as u_int,
                (bsize * 1024 as libc::c_int) as u_int,
                (map * 1024 as libc::c_int) as u_int,
                volnam,
                env,
                file.as_mut_ptr(),
            ),
        );
    }
    if jobs > 0 as libc::c_int {
        exit(
            INIT_Start(
                file.as_mut_ptr(),
                jobs as u_int,
                gmb as u_int,
                rmb as u_int,
                addmb as u_int,
            ),
        );
    }
    c = INIT_Run(file.as_mut_ptr(), env, cmd);
    if c != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"Error occurred in process - %s\n\0" as *const u8 as *const libc::c_char,
            strerror(c),
        );
    }
    if c == 2 as libc::c_int {
        fprintf(
            __stderrp,
            b"\tRSM database not loaded\n\0" as *const u8 as *const libc::c_char,
        );
    } else if c == 12 as libc::c_int {
        fprintf(
            __stderrp,
            b"\tRSM job table is full\n\0" as *const u8 as *const libc::c_char,
        );
    }
    fclose(__stdinp);
    fclose(__stdoutp);
    fclose(__stderrp);
    exit(c);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
