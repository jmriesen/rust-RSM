use ffi::{
    systab, UTIL_Share, COMP_VER, DB_VER, VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH,
    VERSION_TEST, VOL_DEF,
};
use std::ffi::CString;
use std::fs::OpenOptions;

use crate::shared_seg::sys_tab::SYSTAB;

//TODO this is currently only been manually tested.
pub fn info(file: &str) {
    println!("{}", rsm_version());
    println!("Database Version: {DB_VER}\tCompiler Version: {COMP_VER}");
    println!();
    println!("Jacob Riesen");
    println!("Database and Environment Configuration Information:");
    println!();
    match systab_info(file) {
        Ok(sucsses) => println!("{sucsses}"),
        Err(error) => println!("{error}"),
    }
}
//TODO for testability it would probably be best to take in a reference.
//but currently there is no synchronization.
fn systab_info(file: &str) -> Result<String, String> {
    let cfile = CString::new(file.to_string()).unwrap();
    OpenOptions::new()
        .read(true)
        .open(file)
        .map_err(|_| format!("Cannot open database file {file}"))?;
    (unsafe { UTIL_Share(cfile.into_raw()) } == 0)
        .then_some(0)
        .ok_or("Cannot connect to environment.".to_string())?;

    //TODO I am not sure if this is safe.
    //In the C code we use the full path every time.
    let vol = unsafe { (*systab).vol[0] };

    (vol != std::ptr::null::<VOL_DEF>().cast_mut())
        .then_some(0)
        .ok_or("Cannot connect to environment.".to_string())?;

    let sys_tab = unsafe { systab.cast::<SYSTAB>().as_ref() }.unwrap();
    let temp = if sys_tab.vols().next().unwrap().is_none() {
        Err("Cannot connect to environment.".to_string())
    } else {
        Ok(format!("{sys_tab}"))
    };
    //unmounting shared memory segment.
    unsafe { libc::shmdt(systab as *const libc::c_void) };
    temp
}

fn rsm_version() -> String {
    let mut output =
        format!("Reference Standard M V {VERSION_MAJOR}.{VERSION_MINOR}.{VERSION_PATCH} ");
    if VERSION_TEST != 0 {
        output.push_str(&format!("T{VERSION_TEST} "));
    }
    let uname = uname::uname().unwrap();
    output.push_str(&format!("for {} {}", uname.sysname, uname.machine));
    output.push_str(&format!("Built {} at {}", "---", "----"));
    output
}
/*
use for shutdown command.
use std::os::fd::AsRawFd;
let (journal_file,err) =
match OpenOptions::new()
.read(true)
.write(true)
.open(file.clone()){
Ok(fd) => (fd.as_raw_fd(),Ok(())),
//c uses -1 as sentinel value for error.
Err(err) => (-1,Err(err)),
};
    unsafe{partab.jnl_fds[0] = journal_file;}
    err.map_err(|err| format!("Failed to open journal file {} error{}",file,err))?;

    let user = unsafe{libc::getuid()};
     */
