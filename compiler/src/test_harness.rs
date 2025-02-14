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

#[cfg(test)]
pub mod test {
    use std::{
        ffi::CString,
        sync::{LockResult, Mutex, MutexGuard},
    };

    use ffi::PARTAB;
    static GUARD: Mutex<()> = Mutex::new(());

    pub fn compile_c(src: &str) -> (Vec<u8>, LockResult<MutexGuard<'_, ()>>) {
        use ffi::{comp_ptr, partab, source_ptr, systab, SYSTAB};
        use std::io::Write;
        //TODO this is being leaked.
        let source = CString::new(dbg!(src)).unwrap();
        let source = source.into_raw() as *mut u8;

        //TODO Figure out how these different fields actually work.
        //Zeroing out for now so that I can avoid segmentation faults
        let mut sys_tab = SYSTAB::default();

        //TODO something about the value of 100 can cause the len to be calculated incorrectly.
        //Attributing to some of the unsafe code in this test harness.
        //I think it should be fine for now, after all my end goal is to remove the unsafe C all together.
        const BUFFER_LEN: usize = 600;
        let mut compiled_original = [0u8; BUFFER_LEN];
        let _ = std::io::stdout().flush();
        let lock = GUARD.lock();

        let compile_stack_len = {
            unsafe { source_ptr = source };
            unsafe { comp_ptr = compiled_original.as_mut_ptr() };
            unsafe { partab = PARTAB::default() };
            unsafe { systab = &mut sys_tab as *mut SYSTAB };
            unsafe { ffi::parse() }
            unsafe { comp_ptr.offset_from(compiled_original.as_ptr()) }
        };

        (
            compiled_original[..compile_stack_len as usize].to_vec(),
            lock,
        )
    }
}
