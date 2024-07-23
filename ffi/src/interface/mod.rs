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
use std::sync::{Mutex, MutexGuard};

use crate::systab;

static SINGLETON: Mutex<()> = Mutex::new(());
//This is intended to be a light wriapper around the C api that handles most of the unsafe stuff.
pub struct GlobalGuard {
    _guard: MutexGuard<'static, ()>,
}

impl Default for GlobalGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalGuard {
    pub fn new() -> Self {
        //The mutex becomes poisoned if the thread holding it panics
        //This will happen when tests fail
        SINGLETON.clear_poison();
        Self {
            _guard: SINGLETON.lock().unwrap(),
        }
    }
    pub fn systab(&self) -> Option<&crate::SYSTAB> {
        unsafe { systab.as_ref() }
    }
}

impl Drop for GlobalGuard {
    fn drop(&mut self) {}
}

pub mod init {
    use super::*;
    use crate::{u_int, INIT_Start};
    use std::ffi::{c_int, CString};

    pub fn start(
        file_name: &str,
        jobs: u_int,
        gmb: u_int,
        rmb: u_int,
        add: u_int,
        _: &GlobalGuard,
    ) -> Result<(), c_int> {
        let file_name = CString::new(file_name).unwrap().into_raw();
        match unsafe { INIT_Start(file_name, jobs, gmb, rmb, add) } {
            0 => Ok(()),
            x => Err(x),
        }
    }
}
