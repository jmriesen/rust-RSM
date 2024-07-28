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
use super::{Tab, Table};
use std::fmt::Debug;
impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_map();
        builder.entries(
            self.map
                .iter()
                .map(|(k, v)| (k, self.slots[*v].as_ref().unwrap())),
        );
        builder.finish()
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.slots == other.slots && self.map == other.map
    }
}

impl Eq for Table {}

impl Eq for Tab {}

impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        self.fwd_link == other.fwd_link
        && self.usage == other.usage
        //Note data is a pointer 
        //We well need to switch to deep copies at some point.
        && self.data == other.data
        && self.varnam == other.varnam
    }
}

impl Debug for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tab")
            .field("forward_link", &{ self.fwd_link })
            .field("usage", &{ self.usage })
            .field("data", &{ self.data })
            .field("variable name", &self.varnam)
            .finish()
    }
}
