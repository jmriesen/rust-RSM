use std::{mem::transmute, ptr::null_mut};

use ffi::GBD;

use crate::{alloc::Allocation, units::Bytes};

type GlobalBufferDescriptor = GBD;

#[must_use]
pub fn init_global_buffer_descriptors<'a>(
    descriptors: Allocation<GlobalBufferDescriptor>,
    buffer: &Allocation<u8>,
    block_size: Bytes,
) -> &'a mut [GlobalBufferDescriptor] {
    let descriptors = descriptors.to_slice();
    //Validate that the buffer is large enough
    assert!(buffer.layout.size() >= descriptors.len() * block_size.0);

    for (index, descriptor) in descriptors.iter_mut().enumerate() {
        descriptor.write(GlobalBufferDescriptor {
            block: 0,
            next: null_mut(),
            mem: unsafe { buffer.ptr.byte_add(index * block_size.0) }.cast(),
            dirty: null_mut(),
            last_accessed: 0,
        });
    }
    // all descriptors have been written to so it is safe to transmute.
    #[allow(clippy::transmute_ptr_to_ptr)]
    let descriptors: &mut [GlobalBufferDescriptor] = unsafe { transmute(descriptors) };

    for i in 0..descriptors.len() - 1 {
        use std::ops::IndexMut;
        descriptors[i].next = descriptors.index_mut(i + 1);
    }
    descriptors
}

#[cfg(test)]
pub mod test {
    use libc::c_void;

    use crate::test_helper::relitive_ptr;

    use super::*;
    pub fn assert_gbd_eq(
        left: &GBD,
        left_base: *const c_void,
        right: &GBD,
        right_base: *const c_void,
    ) {
        assert_eq!(
            relitive_ptr(left, left_base),
            relitive_ptr(right, right_base)
        );
        assert_eq!({ left.block }, { right.block });
        assert_eq!(
            relitive_ptr(left.mem, left_base),
            relitive_ptr(right.mem, right_base)
        );
        assert_eq!(
            relitive_ptr(left.next, left_base),
            relitive_ptr(right.next, right_base)
        );
        assert_eq!(
            relitive_ptr(left.dirty, left_base),
            relitive_ptr(right.dirty, right_base)
        );
        assert_eq!({ left.last_accessed }, { right.last_accessed });
    }
}
