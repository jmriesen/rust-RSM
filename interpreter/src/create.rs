use crate::label_block;
use crate::AlphaVAR_U;
use crate::DatabaseSize;
use std::fs::OpenOptions;

use crate::{current_time, DB_Block, DB_VER, IDX_START, MAX_MAP_SIZE, RSM_MAGIC, TRUE, UCI_TAB};

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

pub fn create_file(
    block_num: DatabaseSize,
    block_size: u32,
    mut map: u32, //TODO I don't know what this is but it is some size of memory.
    volnam: AlphaVAR_U,
    env: Option<AlphaVAR_U>,
    name: String,
) -> Result<(), String> {
    let env = env.unwrap_or_else(|| "MGR".try_into().unwrap());

    if !(4..1024).contains(&(block_size / 1024)) {
        Err(format!("Block size must be from 4 to 256 KiB"))
    } else {
        //NOTE this map code is not well tested and may have a bug in it.
        //TODO factor this out.
        if map == 0 {
            map = (block_num.inner() + 7) / 8 + 1 + (std::mem::size_of::<label_block>() as u32);
        }
        if map & 1023 != 0 {
            map = (map / 1024 + 1) * 1024;
        }
        if map < block_size {
            map = block_size;
        }
        if map < (block_num.inner() + 7) / 8 + 1 + (std::mem::size_of::<label_block>() as u32) {
            Err(format!(
                "Map block size of {} KiB smaller than required by database size\n",
                map / 1024
            ))
        } else if map > (MAX_MAP_SIZE * 1024) {
            Err(format!(
                "Map block size must be from 0 to {} KiB\n",
                MAX_MAP_SIZE
            ))
        } else {
            //In source they also restricted permissions.
            let mut file = OpenOptions::new()
                .truncate(true)
                .write(true)
                .create_new(true)
                .open(name)
                .unwrap();
            let mut uci = [UCI_TAB {
                global: 0,
                name: "".try_into().unwrap(),
            }; 64];
            uci[0] = UCI_TAB {
                global: 1,
                name: *env.inner(),
            };

            let label = label_block {
                magic: RSM_MAGIC,
                max_block: block_num.inner(),
                header_bytes: map,
                block_size,
                clean: 1,
                creation_time: unsafe { current_time(TRUE as i16) as u64 },
                db_ver: DB_VER as u16,
                journal_available: 0,
                journal_file: [0_i8; 227],
                journal_requested: 0,
                uci,
                volnam: *volnam.inner(),
            };
            use std::io::Write;
            file.write(unsafe { any_as_u8_slice(&label) })
                .map_err(|_| "File write failed")?;
            file.write(&[3]).map_err(|_| "File write failed")?;
            let zero_buffer = vec![0u8; 512 * 1024];
            let remaindor = map as usize % (512 * 1024) - std::mem::size_of::<label_block>() - 1;

            file.write(&zero_buffer[0..remaindor])
                .map_err(|_| "File write failed")?;
            for _ in 0..map / zero_buffer.len() as u32 {
                file.write(&zero_buffer).map_err(|_| "File write failed")?;
            }

            let mgrblk = DB_Block {
                type_: 65,
                last_idx: IDX_START,
                last_free: (block_size / 4 - 7) as u16,
                global: "$GLOBAL".try_into().unwrap(),
                flags: 0,
                right_ptr: 0,
                spare: 0,
            };
            let mut buffer = Box::new(std::io::Cursor::new([0u8; 512 * 1024]));
            buffer
                .write(unsafe { any_as_u8_slice(&mgrblk) })
                .map_err(|_| "File write failed")?;
            let us = ((block_size / 4) - 6) as u16;
            buffer
                .write(&us.to_le_bytes())
                .map_err(|_| "File write failed")?;

            buffer.set_position((4 * us).into());
            buffer
                .write(&[0x18, 0, 0, 0x09, 0x80])
                .map_err(|_| "File write failed")?;
            buffer
                .write("$GLOBAL".as_bytes())
                .map_err(|_| "File write failed")?;

            buffer.set_position((4 * (us + 4)).into());
            buffer.write(&[1]).map_err(|_| "File write failed")?;

            //writing out the db_block;
            file.write(&buffer.get_ref()[0..block_size as usize])
                .map_err(|_| "File write failed")?;

            for _ in 1..block_num.inner() {
                file.write(&zero_buffer[0..block_size as usize])
                    .map_err(|_| "File write failed")?;
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::INIT_Create_File;
    use std::ffi::CString;
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::prelude::*;

    #[ignore]
    #[test]
    fn test() {
        let old_verstion = "Test.dat";
        let new_verstion = "foo2.dat";
        {
            let vol = CString::new("TST").unwrap();
            let file = CString::new(old_verstion).unwrap();
            //NOTE this will print to stdout even if test succeeds.
            unsafe {
                INIT_Create_File(
                    100,
                    4 * 1024,
                    0,
                    vol.as_ptr(),
                    std::ptr::null(),
                    file.as_ptr(),
                )
            };
        }

        create_file(
            100.try_into().unwrap(),
            4 * 1024,
            0,
            "TST".try_into().unwrap(),
            None,
            new_verstion.into(),
        )
        .unwrap();

        let mut old = Vec::new();
        let mut new = Vec::new();
        // read the whole file
        File::open(old_verstion)
            .unwrap()
            .read_to_end(&mut old)
            .unwrap();
        File::open(new_verstion)
            .unwrap()
            .read_to_end(&mut new)
            .unwrap();

        assert_eq!(old.len(), new.len());
        for (i, (old, new)) in old.iter().zip(new.iter()).enumerate() {
            if old != new {
                println!("{:x}", i);
                println!("old:{:x},new:{:x}", old, new);
            }
        }
        assert_eq!(true, old == new);
        remove_file(new_verstion).unwrap();
        remove_file(old_verstion).unwrap();
    }
}
