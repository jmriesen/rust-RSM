use crate::{
    shared_seg::{
        alloc::{create_shared_mem, TabLayout},
        sys_tab::SystemTab,
        vol_def::label::Label,
    },
    units::{Bytes, Megbibytes, Pages},
};
use core::alloc::Layout;
use ffi::{jobtab, u_int, vol_def, GBD, MAX_VOL};
use ffi::{systab, DB_VER, LOCKTAB, RBD};
use ffi::{MAX_GLOBAL_BUFFERS, MAX_JOBS, MAX_ROUTINE_BUFFERS};
use std::num::NonZeroU32;
use std::path::Path;
use std::path::PathBuf;
use std::ptr::{from_mut, from_ref};
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
    pub fn setup_shared_mem_segemnt<'a>(self) -> Result<&'a mut SystemTab, Error> {
        //TODO These layouts should be wrapped or abstracted in some way.
        let meta_data_tab = unsafe {
            TabLayout::<SystemTab, u_int, jobtab, (), (), LOCKTAB>::new(
                Layout::new::<SystemTab>(),
                //I am not sure what this u_int section is for.
                Layout::array::<u_int>((self.jobs * MAX_VOL) as usize).unwrap(),
                Layout::array::<jobtab>(self.jobs as usize).unwrap(),
                Layout::new::<()>(),
                Layout::new::<()>(),
                Layout::array::<u8>(Bytes::from(self.lock_size).0).unwrap(),
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
            crate::shared_seg::vol_def::new(
                &self.file_name,
                self.jobs as usize,
                volumes_start,
                &volset_layout,
            )
        }?;

        let sys_tab = unsafe {
            crate::shared_seg::sys_tab::init(
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
    use ffi::util_share;

    use crate::shared_seg::sys_tab::assert_sys_tab_eq;

    use super::*;
    use std::str::FromStr;

    #[test]
    #[ignore]
    fn validate_mem_seg_layout() {
        let file_path = "test_artifacts/temp";

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

        let global_guard = ffi::interface::GlobalGuard::new();
        let result =
            ffi::interface::init::start(file_path.to_str().unwrap(), 1, 1, 1, 0, &global_guard);
        //NOTE the order is important
        //Start UnMounts the shared meme segment and but does not delete it.
        //So we need to need to initialize the _mem_grard before panicking, that way the mem_guards
        //detractor is guarantied to run regardless of if start failed.
        //There has got to be a better way of dealing with this, but this is my current quick fix.
        let _mem_guard = util_share(&file_path);
        result.unwrap();
        let c_sys_tab = SystemTab::from_raw(global_guard.systab().unwrap());

        assert_sys_tab_eq(sys_tab, c_sys_tab);
        sys_tab.assert_eq(c_sys_tab);
    }
}
