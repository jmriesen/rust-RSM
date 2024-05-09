use crate::{
    alloc::{create_shared_mem, TabLayout},
    label::Label,
    sys_tab::SYSTAB,
};
use crate::{
    sys_tab,
    units::{Bytes, Megbibytes, Pages},
};
use core::alloc::Layout;
use ffi::{jobtab, u_int, vol_def, GBD, MAX_VOL};
use ffi::{systab, DB_VER, LOCKTAB, RBD};
use ffi::{MAX_GLOBAL_BUFFERS, MAX_JOBS, MAX_ROUTINE_BUFFERS};
use std::num::NonZeroU32;
use std::path::Path;
use std::path::PathBuf;
use std::{
    ptr::{from_mut, from_ref},
};
use thiserror::Error;
pub unsafe fn any_as_mut_u8_slice<T: Sized>(p: &mut T) -> &mut [u8] {
    ::std::slice::from_raw_parts_mut(
        std::ptr::from_mut::<T>(p).cast::<u8>(),
        ::std::mem::size_of::<T>(),
    )
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Number of jobs must not exceed {}", MAX_JOBS)]
    InvalidNumberOfJobs,
    #[error("Global buffer size must not exceed {} MiB", MAX_GLOBAL_BUFFERS)]
    InvalidGlobalBufferSize,
    #[error("Routine buffer size must not exceed {} MiB", MAX_ROUTINE_BUFFERS)]
    InvalidRoutineBufferSize,
    #[error("open of database {} failed",.0.display())]
    CouldNotOpenDatabase(PathBuf),
    #[error("Read of label block failed")]
    CouldNotReadLableBlock,
    #[error("Read of label/map block failed")]
    CouldNotReadLableSlashMapBlock,
    #[error("Database is version {}, image requires version {} - start failed!!",DB_VER,.0)]
    MissmachedDatabaseVerstions(u16),
    #[error("Invalid RSM database (wrone magic) - start failed")]
    InvalidMagicNumber,
    #[error("Unable to access database file: {}",.0)]
    CouldNotAccessDatabase(String),
    #[error("RSM environment is already initialized. {}",.0)]
    DatabaseAllreadyInitialized(i32),
    #[error("Unable to create shared memory section")]
    CouldNotCreateSharedMemorySection,
    #[error("Unable to create semaphore set")]
    CouldNotCreateSemaphores,
    #[error("Unable to clear semaphores")]
    CouldNotClearSemaphores,
    #[error("Unable to attach to systab correctly")]
    CouldNotAttachSysTab,
}

pub struct Config {
    file_name: PathBuf,
    jobs: u32,
    global_buffer: Megbibytes,
    ///The numbber of blocks in the global buffer
    routine_buffer: Megbibytes,
    lock_size: Pages,
    label: Label,
}

impl Config {
    /// # Errors
    /// Errors out if the buffer sizes/number of jobs is to large/small
    /// or the Database file we are attempting to open does not exist/is invalid.
    pub fn new(
        file_name: PathBuf,
        //TODO think about what 0 jobs would mean.
        jobs: NonZeroU32,
        global_buffer: Option<Megbibytes>,
        routine_buffer: Option<Megbibytes>,
    ) -> Result<Self, Vec<Error>> {
        use ffi::{LOCKTAB_SIZE, MIN_GBD};
        use Error::{InvalidGlobalBufferSize, InvalidNumberOfJobs, InvalidRoutineBufferSize};
        let mut errors = vec![];

        let routine_buffer = routine_buffer
            .unwrap_or(Megbibytes(jobs.get() as usize / 2))
            .max(Megbibytes(1));

        if MAX_JOBS < jobs.into() {
            errors.push(InvalidNumberOfJobs);
        }
        if Megbibytes(MAX_ROUTINE_BUFFERS as usize) < routine_buffer {
            errors.push(InvalidRoutineBufferSize);
        }
        match Label::load(Path::new(&file_name)) {
            Ok(label) => {
                let global_buffer = global_buffer
                    .unwrap_or(Megbibytes(jobs.get() as usize / 2))
                    .max(Megbibytes(1))
                    .max((label.block_size() * MIN_GBD as usize).megbi_round_up());
                if Megbibytes(MAX_GLOBAL_BUFFERS as usize) < global_buffer {
                    errors.push(InvalidGlobalBufferSize);
                }

                if errors.is_empty() {
                    Ok(Self {
                        file_name,
                        jobs: jobs.get(),
                        global_buffer,
                        routine_buffer,
                        lock_size: Bytes((jobs.get() * LOCKTAB_SIZE) as usize).pages_ceil(),
                        label,
                    })
                } else {
                    Err(errors)
                }
            }
            Err(err) => {
                errors.push(err);
                Err(errors)
            }
        }
    }

    /// # Errors
    ///
    /// Shared memory initialization error issues will be propagated up to the caller
    pub fn setup_shared_mem_segemnt<'a>(self) -> Result<&'a mut SYSTAB, Error> {
        //TODO These layouts should be wrapped or abstracted in some way.
        let meta_data_tab = unsafe {
            TabLayout::<sys_tab::SYSTAB, u_int, jobtab, LOCKTAB, (), ()>::new(
                Layout::new::<sys_tab::SYSTAB>(),
                //I am not sure what this u_int section is for.
                Layout::array::<u_int>((self.jobs * MAX_VOL) as usize).unwrap(),
                Layout::array::<jobtab>(self.jobs as usize).unwrap(),
                Layout::array::<u8>(Bytes::from(self.lock_size).0).unwrap(),
                Layout::new::<()>(),
                Layout::new::<()>(),
            )
        };

        let volset_layout = unsafe {
            TabLayout::<vol_def, u8, GBD, u8, u8, RBD>::new(
                Layout::new::<vol_def>(),
                Layout::array::<u8>(self.label.header_size().0).unwrap(),
                Layout::array::<GBD>(Bytes::from(self.global_buffer).0 / self.label.block_size().0)
                    .unwrap(),
                Layout::array::<u8>(Bytes::from(self.global_buffer).0).unwrap(),
                Layout::array::<u8>(self.label.block_size().0).unwrap(),
                Layout::array::<u8>(Bytes::from(self.routine_buffer).0).unwrap(),
            )
        };

        let share_size = meta_data_tab.size() + volset_layout.size();
        let (shared_mem_segment, _shar_mem_id) = create_shared_mem(share_size.into()).unwrap();

        let volumes_start =
            unsafe { shared_mem_segment.byte_add(Bytes::from(meta_data_tab.size()).0) };

        let volume = unsafe {
            crate::vol_def::new(
                &self.file_name,
                self.jobs as usize,
                volumes_start,
                &volset_layout,
            )
        }?;

        let sys_tab = unsafe {
            crate::sys_tab::init(
                self.jobs as usize,
                volume,
                share_size,
                shared_mem_segment,
                &meta_data_tab,
            )
        };
        dbg!(from_ref(sys_tab));

        //TODO Deal with journaling
        assert_eq!({ sys_tab.maxjob }, 1);
        Ok(sys_tab)
    }
}

///# Errors
///
/// There are multiple reasons starting the DB could fail, including invalied configuration, Bad database file, insuffishent shared memeory exetra.
/// Check `StartError` for more details
pub fn start(
    file_name: String,
    jobs: NonZeroU32,
    global_buffer: Option<NonZeroU32>,  //MEGBE
    routine_buffer: Option<NonZeroU32>, //MEGBE
) -> Result<(), Vec<Error>> {
    let sys_tab = Config::new(
        file_name.into(),
        jobs,
        global_buffer.map(|x| Megbibytes(x.get() as usize)),
        routine_buffer.map(|x| Megbibytes(x.get() as usize)),
    )?
    .setup_shared_mem_segemnt()
    .map_err(|x| vec![x])?;

    unsafe {
        systab = from_mut(sys_tab).cast();
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use ffi::{util_share};
    use libc::{c_void};

    use super::*;
    use std::{ffi::CString, mem::size_of, str::FromStr};

    #[test]
    fn validate_mem_seg_layout() {
        let file_path = "test_artifacts/temp";

        let code =
            unsafe { ffi::INIT_Start(CString::new(file_path).unwrap().into_raw(), 1, 1, 1, 0) };

        let file_path = PathBuf::from_str(file_path).unwrap();
        let sys_tab = Config::new(
            file_path.clone(),
            NonZeroU32::new(1).unwrap(),
            Some(Megbibytes(1)),
            Some(Megbibytes(1)),
        )
            .unwrap()
            .setup_shared_mem_segemnt()
            .unwrap();
        //NOTE INIT_start unmounts the shared meme segment after starting demons.
        let _mem_guard = util_share(&file_path);

        println!("code: {code:?}");
        assert!(code == 0);
        unsafe {
            dbg!(from_ref(sys_tab));
            let c_ver = systab.cast::<sys_tab::SYSTAB>().as_ref().unwrap();
            sys_tab::assert_sys_tab_eq(sys_tab, c_ver);
            test_memory_segment_equality(sys_tab.to_slice(), c_ver.to_slice());
        }

        /*
            if code == 0 {
            unsafe {
            rsm::bindings::shutdown(CString::new("temp").unwrap().into_raw());
        }
        }
             */
    }

    fn test_memory_segment_equality(left: &[u8], right: &[u8]) {
        const SIZE: usize = size_of::<*mut c_void>();
        let left_base = std::ptr::from_ref(left).cast::<c_void>() as isize;
        let right_base = std::ptr::from_ref(right).cast::<c_void>() as isize;
        assert_eq!(left.len(), right.len());
        let mut iter = left
            .array_windows::<SIZE>()
            .zip(right.array_windows::<SIZE>())
            .enumerate();
        let mut errors = vec![];
        while let Some((index, (left, right))) = iter.next() {
            let left_ptr = isize::from_le_bytes(*left).checked_sub(left_base);
            let right_ptr = isize::from_le_bytes(*right).checked_sub(right_base);
            if index == 20 {
                //TODO set sem_id properly
                // continue
            } else if index == 22 {
                //TODO set max_tt properly
                // continue
            } else if left_ptr == right_ptr && left_ptr.is_some(){
                //continue
                //advance the iterator and continue.
                for _ in 1..SIZE {
                    iter.next();
                }
            } else if left[0] == right[0] {
                // continue
            } else {
                errors.push((index,left,right,left_ptr,right_ptr));
            }
        }
        for (index,left,right,left_ptr,right_ptr) in &errors{
            println!("{index}");
            println!("left ptr :{left_ptr:?}");
            println!("right ptr:{right_ptr:?}");
            println!();

            println!("left :{left:?}");
            println!("right:{right:?}");

            println!();
            println!("left[0] :{}", left[0]);
            println!("right[0]:{}", right[0]);
        }
        assert!(errors.is_empty());
    }
}
