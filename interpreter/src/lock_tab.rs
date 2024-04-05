use std::mem::{transmute, MaybeUninit};

use rsm::bindings::{LOCKTAB, VAR_U};

use crate::units::Bytes;

/// Initialized the a Lock Tab
/// The ptr must point to at least size units of memory.
pub unsafe fn init<T:Into<Bytes>>(ptr:*mut MaybeUninit<LOCKTAB>, size:T) -> *mut LOCKTAB{
    let temp  = LOCKTAB{
        fwd_link: std::ptr::null_mut(),
        #[allow(clippy::cast_possible_wrap)]
        size: size.into().0 as i32,
        job: -1,
        byte_count: 0,
        key: [0; 256],
        lock_count: 0,
        name: VAR_U { var_cu: [0; 32] },
        uci: 0,
        vol: 0,
    };
    unsafe{
        (*ptr).write(temp);
        transmute(ptr)
    }
}
