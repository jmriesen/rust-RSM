use crate::JOBTAB;

#[derive(Clone, Copy)]
pub struct ProcessID(pub std::os::raw::c_int);

pub enum StartType {
    Run = 1,
    Job = 2,
}

pub fn find_open_slot(
    job_table: &mut [JOBTAB],
    pid: ProcessID,
    start_type: StartType,
) -> Option<&mut JOBTAB> {
    unsafe {
        crate::bindings::find_open_slot(
            job_table.as_mut_ptr(),
            job_table.len().try_into().unwrap(),
            pid.0,
            start_type as u8,
        )
        .as_mut()
    }
}
