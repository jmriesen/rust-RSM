use crate::{INIT_Start, LOCKTAB_SIZE, MAX_GLOBAL_BUFFERS, MAX_JOBS, MAX_ROUTINE_BUFFERS};
use std::ffi::CString;
use std::fs::OpenOptions;

pub fn start(
    file: String,
    jobs: u32,
    mut global_buffer: u32,
    mut routine_buffer: u32,
    additional_buffer: u32,
) -> Result<(), String> {
    if !(1..MAX_JOBS).contains(&jobs) {
        Err(format!(
            "Invalid number of jobs {} - must be 1 to {}",
            jobs, MAX_JOBS,
        ))
    } else if global_buffer > MAX_GLOBAL_BUFFERS {
        Err(format!(
            "Global buffer size must not exceed {} MiB",
            MAX_GLOBAL_BUFFERS
        ))
    } else if routine_buffer > MAX_ROUTINE_BUFFERS {
        Err(format!(
            "Routine buffer size must not exceed {} MiB",
            MAX_ROUTINE_BUFFERS
        ))
    } else {
        if global_buffer == 0 {
            global_buffer = jobs / 2;
        }

        if routine_buffer == 0 {
            routine_buffer = jobs / 8;
        }
        let pagesize = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };

        let _locksize = ((jobs * LOCKTAB_SIZE) as f64 / pagesize as f64).ceil() as usize * pagesize; // what we need for locktab

        let cfile = CString::new(file.clone()).unwrap();
        let _file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .read(true)
            .create_new(true)
            .open(file)
            .unwrap();
        unsafe {
            INIT_Start(
                cfile.into_raw(),
                jobs,
                global_buffer,
                routine_buffer,
                additional_buffer,
            );
        };
        Ok(())
    }
}
