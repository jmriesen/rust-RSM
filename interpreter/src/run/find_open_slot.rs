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
mod tests {
    use std::{mem::MaybeUninit, ptr::null_mut, usize};

    use ffi::jobtab;
    use std::ptr::from_ref;

    use crate::bindings::run::{find_open_slot, StartType};

    #[test]
    fn empty_table_returns_none() {
        assert!(find_open_slot(&mut [], StartType::Run, 0).is_none());
    }

    fn init_job_table<const N: usize>(process_ids: [i32; N]) -> [jobtab; N] {
        let mut tables: [jobtab; N] =
            [MaybeUninit::<jobtab>::zeroed(); N].map(|x| unsafe { x.assume_init() });
        for (job_tab, pid) in tables.iter_mut().zip(process_ids.into_iter()) {
            job_tab.pid = pid;
        }
        tables
    }

    #[test]
    fn run_uses_first_empty_slot() {
        let mut tables = init_job_table([1, 0, 3]);
        let slot =
            find_open_slot(&mut tables, StartType::Run, 12).expect("there should be an open slot");

        assert_eq!({ slot.pid }, 12);
        assert_eq!(from_ref(slot), from_ref(&tables[1]));
    }

    #[test]
    fn job_looks_for_matching_pid() {
        let mut tables = init_job_table([0, 12, 3]);
        let slot =
            find_open_slot(&mut tables, StartType::Job, 12).expect("there should be an open slot");

        assert_eq!({ slot.pid }, 12);
        assert_eq!(from_ref(slot), from_ref(&tables[1]));
    }

    #[test]
    fn return_none_if_table_is_full() {
        let mut tables = init_job_table([1, 2, 3]);
        let slot = find_open_slot(&mut tables, StartType::Run, 4);
        assert!(slot.is_none());
        let slot = find_open_slot(&mut tables, StartType::Job, 4);
        assert!(slot.is_none());
    }

    #[test]
    fn all_other_start_types_return_none() {
        let mut tables = init_job_table([0, 1, 2]);
        assert!(find_open_slot(&mut tables, StartType::Do, 1).is_none());
        assert!(find_open_slot(&mut tables, StartType::Extrinsic, 1).is_none());
        assert!(find_open_slot(&mut tables, StartType::Xecute, 1).is_none());
    }
}
