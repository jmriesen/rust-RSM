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
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::all)]
#![allow(dead_code)]

use std::{
    ffi::{CStr, CString},
    fmt,
};
//bingen dose not seem to handle size of properly
pub const MAX_MAP_SIZE: u32 =
    (MAX_DATABASE_BLKS / 8 + (std::mem::size_of::<label_block>() as u32)) / 1024 + 1;
pub const IDX_START: u16 = (std::mem::size_of::<DB_Block>() as u16) / 2;
//TODO note the orignal C code defines this differntely bassed off of cfg values. I have just hard coded the one that works on my machine for now.

include!(concat!(env!("OUT_DIR"), "/opcodes.rs"));
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl fmt::Debug for VAR_U {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for VAR_U {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            CStr::from_bytes_until_nul(unsafe { &self.var_cu })
                .unwrap()
                .to_str()
                .unwrap()
        )
    }
}

impl std::cmp::PartialEq for VAR_U {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.var_cu == other.var_cu }
    }
}

impl TryFrom<&str> for VAR_U {
    type Error = String;
    fn try_from(val: &str) -> Result<Self, String> {
        if val.len() < VAR_LEN as usize {
            let val = CString::new(val).unwrap();
            Ok(Self {
                var_cu: val
                    .as_bytes()
                    .iter()
                    .cloned()
                    .chain(std::iter::repeat(0))
                    .take(VAR_LEN as usize)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            })
        } else {
            Err(format!("must be less then {} characters", VAR_LEN))
        }
    }
}

impl TryFrom<String> for VAR_U {
    type Error = String;
    fn try_from(val: String) -> Result<Self, String> {
        (val.as_str()).try_into()
    }
}

impl PartialEq<LABEL_BLOCK> for LABEL_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic
            && self.max_block == other.max_block
            && self.header_bytes == other.header_bytes
            && self.block_size == other.block_size
            && self.creation_time == other.creation_time
            && self.db_ver == other.db_ver
            && self.volnam == other.volnam
            && self.journal_available == other.journal_available
            && self.journal_requested == other.journal_requested
            && self.clean == other.clean
            && self.journal_file == other.journal_file
        //TODO fix this at some point
        //self.uci == other.uci
    }
}
