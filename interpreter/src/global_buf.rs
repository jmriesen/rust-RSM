use std::{mem::{transmute, MaybeUninit}, ptr::null_mut, slice::from_raw_parts_mut};

use libc::c_void;
use rsm::bindings::GBD;

use crate::units::Bytes;


type GlobalBufferDescriptor = GBD;

pub unsafe fn init_Global_Buffer_Descriptors(
    descriptors:&mut [MaybeUninit<GlobalBufferDescriptor>],
    buffer_start:*mut c_void,
    block_size:Bytes,
)->&mut [GlobalBufferDescriptor] {
    for (index,descriptor)in descriptors.iter_mut().enumerate(){
        descriptor.write(GlobalBufferDescriptor{
            block: 0,
            next: null_mut(),
            //TODO mem's value is not initialized.
            mem: unsafe { buffer_start.byte_add(index * block_size.0 ) }
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
