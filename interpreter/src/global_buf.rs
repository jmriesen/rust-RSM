use std::{mem::{transmute, MaybeUninit}, ptr::null_mut, slice::{from_mut_ptr_range, from_raw_parts_mut}};

use libc::c_void;
use rsm::bindings::GBD;

use crate::{alloc::Allocation, units::Bytes};


type GlobalBufferDescriptor = GBD;

pub fn init_global_buffer_descriptors<'a>(
    descriptors:Allocation<GlobalBufferDescriptor>,
    buffer:&Allocation<u8>,
    block_size:Bytes,
)->&'a mut [GlobalBufferDescriptor] {
    let descriptors =
        unsafe{
            from_mut_ptr_range(
                descriptors.ptr..descriptors.ptr.byte_add(descriptors.layout.size())
            )};
    //Validate that the buffer is large enough
    assert!(buffer.layout.size()>=descriptors.len()*block_size.0);

    for (index,descriptor)in descriptors.iter_mut().enumerate(){
        descriptor.write(GlobalBufferDescriptor{
            block: 0,
            next: null_mut(),
            //TODO mem's value is not initialized.
            mem: unsafe { buffer.ptr.byte_add(index * block_size.0 ) }
            .cast(),
            dirty: null_mut(),
            last_accessed: 0,
        });
    }
    // all descriptors have been written to so it is safe to transmute.
    let descriptors:&mut [GlobalBufferDescriptor] = unsafe{transmute(descriptors)};

    for i in 0..descriptors.len() - 1 {
        use std::ops::IndexMut;
        descriptors[i].next = descriptors.index_mut(i + 1);
    }
    descriptors
}
