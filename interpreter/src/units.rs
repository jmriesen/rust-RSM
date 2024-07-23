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
use ffi::MBYTE;

use derive_more::Mul;

///integer number of pages
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Pages(pub usize);
///interger number of Megbiytes
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Megbibytes(pub usize);
///interger number of Kibiytes.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Kibibytes(pub usize);
///interger number of Words.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Words(pub usize);
///interger number of Bytes.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Mul)]
pub struct Bytes(pub usize);

impl From<Megbibytes> for Bytes {
    fn from(megbi: Megbibytes) -> Self {
        Self(megbi.0 * MBYTE as usize)
    }
}

impl From<Kibibytes> for Bytes {
    fn from(kibi: Kibibytes) -> Self {
        Self(kibi.0 * 1024)
    }
}

impl From<Words> for Bytes {
    fn from(word: Words) -> Self {
        Self(word.0 * 4)
    }
}

impl From<Pages> for Bytes {
    fn from(pages: Pages) -> Self {
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };
        Self(pages.0 * page_size)
    }
}

impl From<Kibibytes> for Words {
    fn from(kibi: Kibibytes) -> Self {
        Bytes::from(kibi).try_into().unwrap()
    }
}

impl TryFrom<Bytes> for Words {
    type Error = ();
    /// Currently panics on error case
    fn try_from(bytes: Bytes) -> Result<Self, ()> {
        assert!(bytes.0 % 4 == 0);
        Ok(Self(bytes.0 / 4))
    }
}

impl Bytes {
    ///Round up to nearest kibi
    #[must_use]
    pub fn kibi_round_up(self) -> Kibibytes {
        Kibibytes(self.0.div_ceil(1024))
    }
    ///Round up to nearest Megbiyte
    #[must_use]
    pub fn megbi_round_up(self) -> Megbibytes {
        Megbibytes(self.0.div_ceil(MBYTE as usize))
    }
    ///Round up to nearest page file
    #[must_use]
    pub fn pages_ceil(self) -> Pages {
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };
        Pages(self.0.div_ceil(page_size))
    }
    #[must_use]
    pub fn kibi_floor(self) -> Kibibytes {
        Kibibytes(self.0.div_floor(1024))
    }
    #[must_use]
    pub fn megbi_floor(self) -> Megbibytes {
        Megbibytes(self.0.div_floor(MBYTE as usize))
    }
}

impl std::fmt::Display for Kibibytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}KiB)", self.0,)
    }
}

impl std::fmt::Display for Megbibytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}MiB)", self.0,)
    }
}

impl std::ops::Add for Kibibytes {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Add for Pages {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Add for Bytes {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for Bytes {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl std::ops::Mul<u32> for Kibibytes {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0 * rhs as usize)
    }
}
