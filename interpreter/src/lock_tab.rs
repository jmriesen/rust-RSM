use rsm::bindings::{LOCKTAB, VAR_U};

use crate::alloc::Allocation;

/// Initialized the a Lock Tab
//NOTE Initializing the block of memory should consume the allocation
#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn init<'a>(alloc: Allocation<LOCKTAB>) -> &'a mut LOCKTAB {
    alloc.as_mut().write(LOCKTAB {
        fwd_link: std::ptr::null_mut(),
        #[allow(clippy::cast_possible_wrap)]
        size: alloc.layout.size() as i32,
        job: -1,
        byte_count: 0,
        key: [0; 256],
        lock_count: 0,
        name: VAR_U { var_cu: [0; 32] },
        uci: 0,
        vol: 0,
    });
    unsafe{alloc.as_mut().assume_init_mut()}
}

#[cfg(test)]
pub mod tests{
    use std::ptr::from_ref;

    use rsm::bindings::LOCKTAB;

    use crate::test_helper::relitive_ptr;
    pub fn assert_eq(left:&LOCKTAB,right:&LOCKTAB){
        let left_base = from_ref(left).cast();
        let right_base = from_ref(right).cast();
        assert_eq!(
            relitive_ptr(left.fwd_link, left_base),
            relitive_ptr(right.fwd_link, right_base)
        );
        assert_eq!({left.size},{right.size});
        assert_eq!({left.job},{right.job});
        assert_eq!({left.byte_count},{right.byte_count});
        assert_eq!(left.key,right.key);
        assert_eq!({left.lock_count},{right.lock_count});
        assert_eq!(left.name,right.name);
        assert_eq!(left.uci,right.uci);
        assert_eq!(left.vol,right.vol);
    }
}
