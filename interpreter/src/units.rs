//TODO The newtype pattern is farily common. so a lot of the bolier plate could probubly be removed buy using a 3rd party crate.


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Pages(pub usize);
///interger number of Megbiytes
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Megbibytes(pub usize);
///interger number of kibiytes.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Kibibytes(pub usize);
///interger number of Words.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Words(pub usize);
///interger number of bytes.
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
    pub fn kibi_round_up(self) -> Kibibytes {
        Kibibytes(self.0.div_ceil(1024))
    }
    ///Round up to nearest Megbiyte
    pub fn megbi_round_up(self) -> Megbibytes{
        Megbibytes(self.0.div_ceil(rsm::bindings::MBYTE as usize))
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
