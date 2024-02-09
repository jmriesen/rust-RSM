use crate::label_block;
use crate::var_u::AlphaVAR_U;
use crate::units::DatabaseSize;
use std::{fs::OpenOptions, io::Seek};

use crate::{current_time, DB_Block, DB_VER, IDX_START, MAX_MAP_SIZE, RSM_MAGIC, TRUE, UCI_TAB};

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

///Config for how to create a database file
// TODO consider combining this this with the label block.
pub struct FileConfig{
    name: String,
    volume: AlphaVAR_U,
    env: AlphaVAR_U,
    number_of_blocks: DatabaseSize,
    block_size: u32,//TODO add a unit to this param.
    header_size_bytes: u32,
}
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileConfigError{

    //TODO should I be the type system for this like I am with Database size?
    //Or should I remove the typing from database size.
    ///Block size must be from 4 to 256 KiB
    #[error("Block size must be from 4 to 256 KiB")]
    InvalidBlockSize,
    ///Map block size must be from 0 to [`MAX_MAP_SIZE`] KiB
    #[error("Map block size must be from 0 to {} KiB",MAX_MAP_SIZE)]
    InvalidMapSize,
    ///Map block size is smaller than required by database size.
    #[error("Map block size of {0} KiB smaller than required by database size")]
    InsufficentMapSize(u32),
}

impl FileConfig {
    /// The header size will always be at least as large as a block_size,
    /// but could larger.
    /// If reserve_header_kibibytes is None or to less the the
    /// minimum required header size then minimum head size will be used.
    /// See [`FileConfigError`] for more details on valid values.
    pub fn new(
        name: String,
        volume: AlphaVAR_U,
        env:Option<AlphaVAR_U>,
        number_of_blocks: DatabaseSize,
        block_size: u32,
        reserve_header_kibibytes:Option<u32>,
    )-> Result<Self,FileConfigError>{
        //NOTE this map code is not well tested and may have a bug in it.
        let header_required_size_bytes = number_of_blocks.inner().div_ceil(8) + 1 + (std::mem::size_of::<label_block>() as u32);
        let header_size_bytes = u32::max(
            if let Some (kibibytes) = reserve_header_kibibytes{
                kibibytes*1024
            }else{
                header_required_size_bytes.next_multiple_of(1024)
            }
            ,block_size);

        if !(4..1024).contains(&(block_size / 1024)) {
            Err(FileConfigError::InvalidBlockSize)
        }else if header_size_bytes > rsm::bindings::MAX_MAP_SIZE * 1024{
            Err(FileConfigError::InvalidMapSize)
        }else if header_size_bytes< header_required_size_bytes {
            Err(FileConfigError::InsufficentMapSize(header_size_bytes/1024))
        }else{
            Ok(Self {
                name,
                volume,
                number_of_blocks,
                block_size,
                env:env.unwrap_or_else(|| "MGR".try_into().unwrap()),
                header_size_bytes,
            })
        }
    }

    ///TODO this does not currently verify everthing was written.
    pub fn create(self)->Result<(), String>{
        use std::io::SeekFrom::Start;
        use std::io::Write;
        //In source they also restricted permissions.
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create_new(true)
            .open(self.name)
            .unwrap();

        write_zeros(&file, (self.header_size_bytes + self.block_size*self.number_of_blocks.inner()) as usize);

        file.seek(Start(0));
        let label = label_block {
            magic: RSM_MAGIC,
            max_block: self.number_of_blocks.inner(),
            header_bytes: self.header_size_bytes,
            block_size:self.block_size,
            clean: 1,
            creation_time: unsafe { current_time(TRUE as i16) as u64 },
            db_ver: DB_VER as u16,
            journal_available: 0,
            journal_file: [0_i8; 227],
            journal_requested: 0,
            uci : {
                let mut uci= [UCI_TAB {
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

        file.write(unsafe { any_as_u8_slice(&label) })
            .map_err(|_| "File write failed")?;
        //Writing out that block 0 and 1 have been used.
        //TODO remove raw write I don't want to be relying on int types that carry no context.
        file.write(&[3]).map_err(|_| "File write failed")?;

        //-----------------------------------------------

        file.seek(Start(self.header_size_bytes as u64));
        //Make manager block and $GLOBAL record
        let mgrblk = DB_Block {
            type_: 65,
            last_idx: IDX_START,
            last_free: (self.block_size / 4 - 7) as u16,
            global: "$GLOBAL".try_into().unwrap(),
            flags: 0,
            right_ptr: 0,
            spare: 0,
        };

       file
            .write(unsafe { any_as_u8_slice(&mgrblk) })
            .map_err(|_| "File write failed")?;
        //TODO Remove the 6 litteral.
        let us = ((self.block_size / 4) - 6) as u16;
       file
            .write(&us.to_le_bytes())
            .map_err(|_| "File write failed")?;

        file.seek(Start(self.header_size_bytes  as u64 +(4 * us) as u64));

        //TODO this is vary messy fix this. We should not be reaching into a CSTRING every time I need to make one. 
        use rsm::bindings::CSTRING;
        let block_identifier = CSTRING{
            len:24,
            buf: {
                let mut buf = [0; 65535];
                buf[1]=9;
                buf[2]=128; // "\200" from the C code.
                buf[3..3+7].copy_from_slice("$GLOBAL".as_bytes());
                buf[4*4-2..5*4-2].copy_from_slice(&1_i32.to_le_bytes());
                buf
            }
        };

        file.write(unsafe {
            &any_as_u8_slice(&block_identifier)[..24]
        });

        Ok(())
    }
}

///Writes out zeros to a file.
///This is done in 512 kibibyte increments.
///TODO this does not currently verify everthing was written.
fn write_zeros<W: std::io::Write>(mut writer: W, bytes:usize){
    let zero_buffer = vec![0u8; 512 * 1024];
    for _ in 0..bytes/zero_buffer.len() {
        writer.write(&zero_buffer);
    }
    writer.write(&zero_buffer[0..bytes%zero_buffer.len()]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::INIT_Create_File;
    use std::ffi::CString;
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::prelude::*;

    #[ignore]
    #[rstest]
    #[case("TST", 100, 4 * 1024)]
    fn test(#[case] volume:&str, #[case] number_of_blocks: u32, #[case] block_size: u32) {
        //tests run in parrallel, uuids are used to avoid file conflicts.
        let file_name = uuid::Uuid::new_v4().to_string();

        let c_file_name = format!("{}_c.dat",file_name);
        {
            let vol = CString::new(volume).unwrap();
            let file = CString::new(c_file_name.as_str()).unwrap();
            //NOTE this will print to stdout even if test succeeds.
            unsafe {
                INIT_Create_File(
                    number_of_blocks,
                    block_size,
                    0,
                    vol.into_raw(),
                    std::ptr::null_mut(),
                    file.into_raw(),
                )
            };
        }

        let rust_file_name = format!("{}_rust.dat",file_name);
        FileConfig::new(
            rust_file_name.clone(),
            volume.try_into().unwrap(),
            None,
            number_of_blocks.try_into().unwrap(),
            block_size,
            None
        )
            .unwrap()
            .create();

        diff_files(&c_file_name, &rust_file_name);

        remove_file(rust_file_name).unwrap();
        remove_file(c_file_name).unwrap();
    }

    fn diff_files(old:&str,new:&str){
        let mut old_bytes = Vec::new();
        let mut new_butes = Vec::new();
        // read the whole file
        File::open(old)
            .unwrap()
            .read_to_end(&mut old_bytes)
            .unwrap();
        File::open(new)
            .unwrap()
            .read_to_end(&mut new_butes)
            .unwrap();

        assert_eq!(old_bytes.len(), new_butes.len());
        for (i, (old, new)) in old_bytes.iter().zip(new_butes.iter()).enumerate() {
            if old != new {
                println!("{:x}", i);
                println!("old:{:x},new:{:x}", old, new);
            }
        }
        assert_eq!(true, old_bytes == new_butes);

    }
}
