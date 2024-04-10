
use rsm::bindings::{LOCKTAB, VAR_U};

use crate::alloc::Allocation;

/// Initialized the a Lock Tab
//NOTE Initializing the block of memory should consume the allocation
#[allow(clippy::needless_pass_by_value)]
#[must_use] pub fn init(alloc:Allocation<LOCKTAB>) -> *mut LOCKTAB{
    unsafe{alloc.ptr.as_mut()}.unwrap().write(LOCKTAB{
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
    alloc.ptr.cast()
}
