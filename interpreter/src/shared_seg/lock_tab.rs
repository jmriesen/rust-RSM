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
use ffi::{LOCKTAB, VAR_U};

use super::alloc::Allocation;

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
    unsafe { alloc.as_mut().assume_init_mut() }
}

#[cfg(test)]
pub mod tests {
    use std::ptr::from_ref;

    use ffi::LOCKTAB;

    use crate::test_helper::relitive_ptr;
    pub fn assert_eq(left: &LOCKTAB, right: &LOCKTAB) {
        let left_base = from_ref(left).cast();
        let right_base = from_ref(right).cast();
        assert_eq!(
            relitive_ptr(left.fwd_link, left_base),
            relitive_ptr(right.fwd_link, right_base)
        );
        assert_eq!({ left.size }, { right.size });
        assert_eq!({ left.job }, { right.job });
        assert_eq!({ left.byte_count }, { right.byte_count });
        assert_eq!(left.key, right.key);
        assert_eq!({ left.lock_count }, { right.lock_count });
        assert_eq!(left.name, right.name);
        assert_eq!(left.uci, right.uci);
        assert_eq!(left.vol, right.vol);
    }
}
