use std::{fmt::Display, fs::OpenOptions, io::Read, mem::MaybeUninit, path::Path};

use derive_more::{AsMut,AsRef};
use ffi::{DB_VER, LABEL_BLOCK, UCI_TAB};
use ref_cast::RefCast;

use crate::{
    start::{any_as_mut_u8_slice, Error},
    units::Bytes,
};

#[derive(RefCast, AsMut,AsRef)]
#[repr(transparent)]
pub struct Label(LABEL_BLOCK);

impl Label {
    fn journal_file(&self) -> Option<String> {
        use core::ffi::CStr;
        let journal_file = self.0.journal_file.map(|x| x as u8);
        let journal_file = CStr::from_bytes_until_nul(&journal_file)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if journal_file.is_empty() {
            None
        } else {
            Some(journal_file)
        }
    }

    #[must_use] pub fn header_size(&self) -> Bytes {
        Bytes(self.0.header_bytes as usize)
    }

    #[must_use] pub fn block_size(&self) -> Bytes {
        Bytes(self.0.block_size as usize)
    }

    #[must_use] pub fn uci(&self) -> &[UCI_TAB] {
        &self.0.uci
    }

    #[must_use] pub fn clean(&self) -> bool {
        self.0.clean == 0
    }
    pub fn set_dirty(&mut self, is_dirty: bool) {
        self.0.clean = u8::from(is_dirty);
    }

    pub fn load(file: &Path) -> Result<Label, Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(file)
            .map_err(|_| Error::CouldNotOpenDatabase(file.into()))?;

        let mut label = MaybeUninit::<LABEL_BLOCK>::zeroed();
        file.read_exact(unsafe { any_as_mut_u8_slice(&mut label) })
            .map_err(|_| Error::CouldNotReadLableBlock)?;
        let label = unsafe { label.assume_init() };
        if label.db_ver == DB_VER as u16 {
            Ok(Label(label))
        } else {
            Err(Error::MissmachedDatabaseVerstions(label.db_ver))
            // TODO C also gives instrcutions on how to update image.
        }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DB Volume Name:\t\t{}", self.0.volnam)?;
        writeln!(f, "DB Manager UCI name :\t{}", self.0.uci[0].name)?;

        let journal_file = self.journal_file().unwrap_or("--".into());
        let journal_file_status = if self.0.journal_available != 0 {
            "ON"
        } else {
            "OFF"
        };
        writeln!(
            f,
            "DB Journal File Path:\t{journal_file}[{journal_file_status}]"
        )?;

        writeln!(f, "DB Size in Blocks:\t{}", { self.0.max_block })?;
        writeln!(f, "DB Map Block Size:\t{}", self.header_size().kibi_floor())?;
        writeln!(f, "DB Block Size:\t{}", self.block_size().kibi_floor())?;
        Ok(())
    }
}
