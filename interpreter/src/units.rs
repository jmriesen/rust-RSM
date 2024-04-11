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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Bytes(pub usize);

impl From<Megbibytes> for Bytes {
    fn from(megbi: Megbibytes) -> Self {
        Self(megbi.0 * rsm::bindings::MBYTE as usize)
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
        Megbibytes(self.0.div_ceil(rsm::bindings::MBYTE as usize))
    }
    ///Round up to nearest page file
    #[must_use]
    pub fn pages_ceil(self) -> Pages {
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };
        Pages(self.0.div_ceil(page_size))
    }
}

impl std::fmt::Display for Kibibytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, KiB)", self.0,)
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
