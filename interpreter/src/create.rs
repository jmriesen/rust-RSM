use crate::units::{Bytes, Kibibytes, Words};
use crate::var_u::AlphaVAR_U;
use ffi::label_block;
use std::{fs::OpenOptions, io::Seek};

use ffi::{current_time, DB_Block, DB_VER, IDX_START, MAX_MAP_SIZE, RSM_MAGIC, TRUE, UCI_TAB};

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        std::ptr::from_ref::<T>(p).cast::<u8>(),
        ::std::mem::size_of::<T>(),
    )
}

///Config for how to create a database file
// TODO consider combining this this with the label block.
pub struct FileConfig {
    name: String,
    volume: AlphaVAR_U,
    env: AlphaVAR_U,
    ///`MAX_DATABASE_BLKS` is `i32::Max` not `u32::Max`
    number_of_blocks: u32,
    block_size: Kibibytes,
    header_size: Kibibytes,
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum FileConfigError {
    ///Database size must be from 100 to [`i32::Max`] blocks
    #[error("Database size must be from 100 to {} blocks", i32::MAX)]
    InvalidDatabaseSize,
    ///Block size must be from 4 to 256 KiB
    #[error("Block size must be from 4 to 256 KiB")]
    InvalidBlockSize,
    ///Map block size must be from 0 to [`MAX_MAP_SIZE`] KiB
    #[error("Map block size must be from 0 to {} KiB", MAX_MAP_SIZE)]
    InvalidMapSize,
    ///Map block size is smaller than required by database size.
    #[error("Map block size of {0} smaller than required by database size")]
    InsufficentMapSize(Kibibytes),
}

impl FileConfig {
    /// The header size will always be at least as large as a `block_size`,
    /// but could larger.
    /// If `reserve_header_kibibytes` is None or to less the the
    /// minimum required header size then minimum head size will be used.
    /// See [`FileConfigError`] for more details on valid values.
    pub fn new(
        name: String,
        volume: AlphaVAR_U,
        env: Option<AlphaVAR_U>,
        mut number_of_blocks: u32,
        block_size: Kibibytes,
        reserve_header: Option<Kibibytes>,
    ) -> Result<Self, Vec<FileConfigError>> {
        let mut errors = vec![];

        //Round up to 8*n-1
        number_of_blocks |= 7;
        if !(100..i32::MAX as u32).contains(&number_of_blocks) {
            errors.push(FileConfigError::InvalidDatabaseSize);
        }
        let header_min_size = Bytes(
            std::mem::size_of::<label_block>()
            //bytes required to track block useage.
                + number_of_blocks.div_ceil(8) as usize + 1,
        );

        let header_size = Kibibytes::max(
            reserve_header.unwrap_or(header_min_size.kibi_round_up()),
            block_size,
        );

        if !(Kibibytes(4)..Kibibytes(256)).contains(&block_size) {
            errors.push(FileConfigError::InvalidBlockSize);
        }
        if header_size > Kibibytes(MAX_MAP_SIZE as usize) {
            errors.push(FileConfigError::InvalidMapSize);
        }
        if header_min_size > header_size.into() {
            errors.push(FileConfigError::InsufficentMapSize(header_size));
        }
        if errors.is_empty() {
            Ok(Self {
                name,
                volume,
                number_of_blocks,
                block_size,
                env: env.unwrap_or_else(|| "MGR".try_into().unwrap()),
                header_size,
            })
        } else {
            Err(errors)
        }
    }

    pub fn create(self) -> std::io::Result<()> {
        use ffi::CSTRING;
        use std::io::SeekFrom::Start;
        use std::io::Write;
        //In source they also restricted permissions.
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create_new(true)
            .open(self.name)
            .unwrap();

        write_zeros(
            &file,
            self.header_size + (self.block_size * self.number_of_blocks),
        )?;

        file.seek(Start(0))?;
        let label = label_block {
            magic: RSM_MAGIC,
            max_block: self.number_of_blocks,
            header_bytes: Bytes::from(self.header_size).0 as u32,
            block_size: Bytes::from(self.block_size).0 as u32,
            clean: 1,
            creation_time: unsafe { current_time(TRUE as i16) as u64 },
            db_ver: DB_VER as u16,
            journal_available: 0,
            journal_file: [0_i8; 227],
            journal_requested: 0,
            uci: {
                let mut uci = [UCI_TAB {
                    global: 0,
                    name: "".try_into().unwrap(),
                }; 64];
                uci[0] = UCI_TAB {
                    global: 1,
                    name: *self.env.inner(),
                };
                uci
            },
            volnam: *self.volume.inner(),
        };

        file.write_all(unsafe { any_as_u8_slice(&label) })?;
        //Writing out that block 0 and 1 have been used.
        //TODO remove raw write I don't want to be relying on int types that carry no context.
        file.write_all(&[3])?;

        //-----------------------------------------------

        file.seek(Start(Bytes::from(self.header_size).0 as u64))?;
        //Make manager block and $GLOBAL record
        let mgrblk = DB_Block {
            type_: 65,
            last_idx: IDX_START,
            last_free: (Words::from(self.block_size).0 - 7) as u16,
            global: "$GLOBAL".try_into().unwrap(),
            flags: 0,
            right_ptr: 0,
            spare: 0,
        };

        file.write_all(unsafe { any_as_u8_slice(&mgrblk) })?;

        //TODO this is vary messy fix this. We should not be reaching into a CSTRING every time I need to make one.
        let block_identifier = CSTRING {
            len: 24,
            buf: {
                let mut buf = [0; 65535];
                buf[1] = 9;
                buf[2] = 128; // "\200" from the C code.
                buf[3..3 + 7].copy_from_slice("$GLOBAL".as_bytes());
                buf[4 * 4 - 2..5 * 4 - 2].copy_from_slice(&1_i32.to_le_bytes());
                buf
            },
        };

        let us = Bytes::from(self.block_size) - Bytes(block_identifier.len.into());
        let us = Words::try_from(us).unwrap();
        file.write_all(&(us.0 as u16).to_le_bytes())?;

        file.seek(Start(
            (Bytes::from(self.header_size) + Bytes::from(us)).0 as u64,
        ))?;

        file.write_all(unsafe { &any_as_u8_slice(&block_identifier)[..24] })?;

        Ok(())
    }
}

///Writes out zeros to a file.
///This is done in 512 kibibyte increments.
fn write_zeros<W: std::io::Write>(mut writer: W, kibibytes: Kibibytes) -> std::io::Result<()> {
    let zero_buffer = vec![0u8; 512 * 1024];
    let bytes = Bytes::from(kibibytes).0;
    for _ in 0..bytes / zero_buffer.len() {
        writer.write_all(&zero_buffer)?;
    }
    writer.write_all(&zero_buffer[0..bytes % zero_buffer.len()])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::INIT_Create_File;
    use rstest::rstest;
    use std::ffi::CString;
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::prelude::*;

    #[ignore]
    #[rstest]
    #[case("TST", 100, 4)]
    fn test(#[case] volume: &str, #[case] number_of_blocks: u32, #[case] block_size: u32) {
        //tests run in parrallel, uuids are used to avoid file conflicts.
        let file_name = uuid::Uuid::new_v4().to_string();

        let c_file_name = format!("{file_name}_c.dat");
        {
            let vol = CString::new(volume).unwrap();
            let file = CString::new(c_file_name.as_str()).unwrap();
            //NOTE this will print to stdout even if test succeeds.
            unsafe {
                INIT_Create_File(
                    number_of_blocks,
                    block_size * 1024,
                    0,
                    vol.into_raw(),
                    std::ptr::null_mut(),
                    file.into_raw(),
                )
            };
        }

        let rust_file_name = format!("{file_name}_rust.dat");
        let _ = FileConfig::new(
            rust_file_name.clone(),
            volume.try_into().unwrap(),
            None,
            number_of_blocks.try_into().unwrap(),
            Kibibytes(block_size as usize),
            None,
        )
        .unwrap()
        .create();

        diff_files(&c_file_name, &rust_file_name);

        remove_file(rust_file_name).unwrap();
        remove_file(c_file_name).unwrap();
    }

    fn diff_files(old: &str, new: &str) {
        let mut old_bytes = Vec::new();
        let mut new_butes = Vec::new();
        // read the whole file
        let _ = File::open(old).unwrap().read_to_end(&mut old_bytes);
        let _ = File::open(new).unwrap().read_to_end(&mut new_butes);

        assert_eq!(old_bytes.len(), new_butes.len());
        for (i, (old, new)) in old_bytes.iter().zip(new_butes.iter()).enumerate() {
            if old != new {
                println!("{i:x}");
                println!("old:{old:x},new:{new:x}");
            }
        }
        assert!(old_bytes == new_butes);
    }
}
