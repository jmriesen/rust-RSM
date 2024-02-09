//TODO The newtype pattern is farily common. so a lot of the bolier plate could probubly be removed buy using a 3rd party crate.

///interger number of kibiytes.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Kibibytes(pub usize);
///interger number of bytes.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Bytes(pub usize);

impl From<Kibibytes> for Bytes {
    fn from(bytes: Kibibytes) -> Self {
        Self(bytes.0 * 1024)
    }
}

impl Bytes {
    ///Round up to nearest kibi
    pub fn kibi_round_up(self) -> Kibibytes {
        Kibibytes(self.0.div_ceil(1024))
    }
    pub fn words(self) -> usize {
        self.0 / 4
    }
}

// Implement `Display` for `MinMax`.
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

impl std::ops::Mul<u32> for Kibibytes {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0 * rhs as usize)
    }
}
