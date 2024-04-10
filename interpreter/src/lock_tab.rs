use std::mem::{transmute, MaybeUninit};

use rsm::bindings::{LOCKTAB, VAR_U};

use crate::{alloc::Allocation, units::Bytes};

/// Initialized the a Lock Tab
pub fn init(alloc:Allocation<LOCKTAB>) -> *mut LOCKTAB{
    //TODO I am fairly sure this does not work.
    //The unsafe block tries to copy so we end up writing in the wrong location.
    unsafe{*alloc.ptr}.write(LOCKTAB{
        fwd_link: std::ptr::null_mut(),
        #[allow(clippy::cast_possible_wrap)]
        //TODO this has not been zeroed
        size: alloc.layout.size() as i32,
        job: -1,
        byte_count: 0,
        key: [0; 256],
        lock_count: 0,
        name: VAR_U { var_cu: [0; 32] },
        uci: 0,
        vol: 0,
    });
    unsafe{
        transmute(alloc.ptr)
    }
}
