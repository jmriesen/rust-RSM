/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
use std::{mem::transmute, ptr::null_mut};

use ffi::GBD;

use crate::{shared_seg::alloc::Allocation, units::Bytes};

type GlobalBufferDescriptor = GBD;

#[must_use]
pub fn init_global_buffer_descriptors<'a>(
    descriptors: Allocation<GlobalBufferDescriptor>,
    buffer: &Allocation<u8>,
    block_size: Bytes,
) -> &'a mut [GlobalBufferDescriptor] {
    let descriptors = descriptors.into_slice();
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
