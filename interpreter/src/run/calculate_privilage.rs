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
    use ffi::{run::Error, CSTRING, LABEL_BLOCK, MAX_GROUPS, PRVGRP, UCIS, UCI_TAB, VAR_U};
    use libc::gid_t;

    use crate::bindings::run::tab_calculate_privilage;

    #[test]
    fn privilaged_if_we_stated_the_database() {
        let user = 5;
        let num_jobs_max = 100;
        assert_eq!(
            Ok(true),
            tab_calculate_privilage(user, user, num_jobs_max, &[])
        );
    }

    #[test]
    fn error_if_max_of_one_job_and_not_starting_user() {
        let user1 = 5;
        let user2 = 10;
        let num_jobs_max = 1;
        assert_eq!(
            Ok(true),
            tab_calculate_privilage(user1, user1, num_jobs_max, &[])
        );
        assert_eq!(
            Err((12, true)),
            tab_calculate_privilage(user1, user2, num_jobs_max, &[])
        );
    }

    #[test]
    fn error_in_get_groups() {
        let user = 5;
        let start_user = 10;
        let num_jobs_max = 100;
        //By setting the groups size to larger then MAX_GROUPS I can get the mock to throw an
        //error.
        let groups = [0; MAX_GROUPS as usize + 1];
        assert_eq!(
            Err((111, false)),
            tab_calculate_privilage(user, start_user, num_jobs_max, &groups)
        );
    }

    #[test]
    fn not_part_of_privliaged_group() {
        let user = 5;
        let start_user = 10;
        let num_jobs_max = 100;
        //By setting the groups size to larger then MAX_GROUPS I can get the mock to throw an
        //error.
        let groups = [0; 10];
        assert_eq!(
            Ok(false),
            tab_calculate_privilage(user, start_user, num_jobs_max, &groups)
        );
    }

    #[test]
    fn part_of_privliaged_group() {
        let user = 5;
        let start_user = 10;
        let num_jobs_max = 100;
        //By setting the groups size to larger then MAX_GROUPS I can get the mock to throw an
        //error.
        let mut groups = [0; 10];
        //NOTE: the value of PRVGRP changes depending on build OS
        groups[4] = PRVGRP;
        assert_eq!(
            Ok(true),
            tab_calculate_privilage(user, start_user, num_jobs_max, &groups)
        );
    }
}
