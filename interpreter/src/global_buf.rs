use std::{mem::transmute, ptr::null_mut};

use rsm::bindings::GBD;

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
