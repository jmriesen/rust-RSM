use crate::{
    systab, UTIL_Share, COMP_VER, DB_VER, VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH,
    VERSION_TEST, VOL_DEF,
};
use std::ffi::{CStr, CString};
use std::fs::OpenOptions;

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
    let vol = unsafe { *vol };
    let vollab = unsafe { *vol.vollab };

    println!("DB File Path:\t\t{file}");
    println!("DB Volume Name:\t\t{}", &vollab.volnam);
    println!("DB Manager UCI Name:\t{}", &vollab.uci[0].name);
    //Checking if the file name is empty.
    let journal_file = unsafe { CStr::from_ptr(vollab.journal_file.as_ptr()) };

    let journal_file: String = if vollab.journal_file[0] != 0 {
        journal_file.to_owned().into_string().unwrap()
    } else {
        "--".into()
    };
    let available = if vollab.journal_available != 0 {
        "ON"
    } else {
        "OFF"
    };

    println!("DB Journal File Path:\t{journal_file} [{available}]");
    let temp = vollab.max_block;
    println!("DB Size in Blocks:\t{temp}");
    println!("DB Map Block Size:\t{}\tKiB", vollab.header_bytes / 1024);
    println!("DB Block Size:\t\t{}\tKiB", vollab.block_size / 1024);

    println!(
        "Global Buffers:\t\t{}\tMiB ({} Buffers)",
        unsafe {
            ((*(*systab).vol[0])
                .zero_block
                .offset_from((*(*systab).vol[0]).global_buf))
                / 1_048_576
        },
        unsafe { (*(*systab).vol[0]).num_gbd }
    );
    let temp = unsafe { vol.rbd_end.offset_from(vol.rbd_head) } / 1_048_576;
    println!("Routine Buffer Space:\t{temp}\tMiB");
    let temp = unsafe { *systab }.locksize / 1024;
    println!("Lock Table Size:\t{temp}\tKiB");
    let temp = unsafe { *systab }.maxjob;
    println!(
        "Job Table Slots:\t{}\tJob{}",
        temp,
        if temp > 1 { "s" } else { "" }
    );
    print!("Daemon Process IDs:\t");

    for pid in vol
        .wd_tab
        .iter()
        .map(|wd_tab| wd_tab.pid)
        .filter(|pid| pid != &0)
    {
        print!("{pid} ");
    }
    unsafe { libc::shmdt(systab as *const libc::c_void) };
    Ok(String::new())
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
