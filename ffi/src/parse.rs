use std::{ffi::CString, sync::Mutex};

use crate::PARTAB;
static PARSE_GUARD: Mutex<()> = Mutex::new(());

pub fn parse(source: &str) -> Vec<u8> {
    use crate::{comp_ptr, partab, source_ptr, systab, SYSTAB};
    let source = CString::new(source).unwrap();
    let mut source = source.as_bytes_with_nul().to_vec();

    let mut sys_tab = SYSTAB::default();
    let par_tab = PARTAB::default();

    //NOTE arbitrarily chosen buffer length.
    //This should be two orders of magnitude more than we need.
    const BUFFER_LEN: usize = 60000;
    let mut output_buffer = [0u8; BUFFER_LEN];

    let _lock = PARSE_GUARD.lock();

    // Saving the original values
    let (original_source_ptr, original_comp_ptr, original_partab_ptr, original_systab_ptr) =
        unsafe { (source_ptr, comp_ptr, partab, systab) };
    //Replacing with mock/temp values
    unsafe {
        source_ptr = source.as_mut_ptr();
        comp_ptr = output_buffer.as_mut_ptr();
        partab = par_tab;
        systab = &mut sys_tab as *mut SYSTAB;
    }

    unsafe { crate::parse() };
    let output_len = unsafe { comp_ptr.offset_from(output_buffer.as_ptr()) };
    //Restore the globals
    unsafe {
        source_ptr = original_source_ptr;
        comp_ptr = original_comp_ptr;
        partab = original_partab_ptr;
        systab = original_systab_ptr;
    }

    output_buffer[..output_len as usize].to_vec()
}
