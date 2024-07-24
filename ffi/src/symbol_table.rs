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
use std::{
    ptr::{from_mut, from_ref},
    sync::{LockResult, Mutex, MutexGuard},
};

use crate::{ST_Get, ST_Init, ST_Kill, ST_Set, CSTRING, MVAR};

///controls access to table globals
static TABLE_MUTEX: Mutex<()> = Mutex::new(());

pub struct Table {
    _guard: LockResult<MutexGuard<'static, ()>>,
}

impl Table {
    pub fn new() -> Self {
        let temp = Self {
            _guard: TABLE_MUTEX.lock(),
        };
        unsafe { ST_Init() };
        temp
    }

    pub fn set(&mut self, var: &MVAR, data: &CSTRING) {
        unsafe { ST_Set(from_ref(var).cast_mut(), from_ref(data).cast_mut()) };
    }

    pub fn get(&self, var: &MVAR) -> Option<CSTRING> {
        let mut buf = [0; 65535];
        let len = unsafe { ST_Get(from_ref(var).cast_mut(), buf.as_mut_ptr()) };

        if len >= 0 {
            Some(CSTRING {
                buf,
                len: len as u16,
            })
        } else {
            None
        }
    }
    pub fn kill(&self, var: &MVAR) {
        unsafe { ST_Kill(from_ref(var).cast_mut()) };
    }
}