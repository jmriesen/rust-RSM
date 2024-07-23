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
use libc::c_void;

/// calculates the offset from the base pointer, if the ptr is null returns None.
/// this function was created to help facilitate A/B testing the shared memory segment layout.
pub fn relitive_ptr<T>(ptr: *const T, base: *const c_void) -> Option<isize> {
    if ptr.is_null() {
        None
    } else {
        //NOTE I considered using byte_offset_from(base)
        //however in order to use that safely both pointer need to be in the same allocation block
        //since we this is intended to be used while A/B testing the shared memory segment layout, that "should" always hold.
        //But it is simpler to not require that constraint. (otherwise I would need to mark this function as unsafe)
        Some(ptr as isize - base as isize)
    }
}
