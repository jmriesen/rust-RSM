use crate::alloc::{create_shared_mem, TabLayout};
use crate::bindings::{
    jobtab, locktab, u_int, vol_def, DEFAULT_PREC, GBD, HISTORIC_DNOK, HISTORIC_EOK,
    HISTORIC_OFFOK, MAX_VOL, TRANTAB, VAR_U, VOL_DEF,
};
use crate::global_buf::init_Global_Buffer_Descriptors;
use crate::{
    sys_tab,
    units::{Bytes, Megbibytes, Pages},
};
use crate::{lock_tab, MAX_GLOBAL_BUFFERS, MAX_JOBS, MAX_ROUTINE_BUFFERS};
use core::alloc::Layout;
use std::slice::{from_ptr_range, from_raw_parts_mut};
use libc::c_void;
use rsm::bindings::{label_block, systab, DB_VER, GBD_HASH, LOCKTAB, RBD_HASH};
use std::fs::OpenOptions;
use std::io::Read;
use std::mem::{MaybeUninit};
use std::num::NonZeroU32;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::path::PathBuf;
use std::ptr::{null_mut};
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
    num_global_descriptor: usize,
    routine_buffer: Megbibytes,
    lock_size: Pages,
    label: label_block,
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
        use Error::{InvalidGlobalBufferSize, InvalidNumberOfJobs, InvalidRoutineBufferSize};
        let mut errors = vec![];

        let mut global_buffer =
            global_buffer.unwrap_or(Megbibytes((jobs.get() / 2).max(1) as usize));
        let routine_buffer = routine_buffer.unwrap_or(Megbibytes((jobs.get() / 8).max(1) as usize));

        if MAX_JOBS < jobs.into() {
            errors.push(InvalidNumberOfJobs);
        }
        if Megbibytes(MAX_GLOBAL_BUFFERS as usize) < global_buffer {
            errors.push(InvalidGlobalBufferSize);
        }
        if Megbibytes(MAX_ROUTINE_BUFFERS as usize) < routine_buffer {
            errors.push(InvalidRoutineBufferSize);
        }
        let label_load = Self::load_lable_info(Path::new(&file_name));

        if let Ok(label) = label_load
            && errors.is_empty()
        {
            use rsm::bindings::{LOCKTAB_SIZE, MIN_GBD};
            let min_global_buffer_size =
                Bytes((label.block_size * MIN_GBD) as usize).megbi_round_up();
            global_buffer = global_buffer.max(min_global_buffer_size);
            let num_global_descriptor = Bytes::from(global_buffer).0 / label.block_size as usize;

            Ok(Self {
                file_name,
                jobs: jobs.get(),
                global_buffer,
                routine_buffer,
                lock_size: Bytes((jobs.get() * LOCKTAB_SIZE) as usize).pages_ceil(),
                label,
                num_global_descriptor,
            })
        } else {
            errors.extend(label_load.err());
            Err(errors)
        }
    }

    /// # Errors
    ///
    /// Shared memory initialization error issues will be propagated up to the caller
    /// # Panics
    /// TODO I need to implement proper panic handling
    pub fn setup_shared_mem_segemnt(self) -> Result<*mut sys_tab::SYSTAB, Error> {
        let meta_data_tab = unsafe {
            TabLayout::<sys_tab::SYSTAB, u_int, jobtab, MaybeUninit<LOCKTAB>, (), ()>::new(
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
            TabLayout::<vol_def, c_void, MaybeUninit<GBD>, c_void, c_void, c_void>::new(
                Layout::new::<vol_def>(),
                Layout::array::<u8>(self.label.header_bytes.try_into().unwrap()).unwrap(),
                Layout::array::<GBD>(self.num_global_descriptor).unwrap(),
                Layout::array::<u8>(Bytes::from(self.global_buffer).0).unwrap(),
                Layout::array::<u8>(self.label.block_size.try_into().unwrap()).unwrap(),
                Layout::array::<u8>(Bytes::from(self.routine_buffer).0).unwrap(),
            )
        };

        let share_size = meta_data_tab.size() + volset_layout.size();

        let (shared_mem_segment, _shar_mem_id) = create_shared_mem(share_size.into()).unwrap();
        let (sys_tab, _, job_tab, lock_tab, _, _, volumes_start) =
            unsafe { meta_data_tab.calculate_offsets(shared_mem_segment) };
        let lock_tab = unsafe{lock_tab::init(lock_tab, self.lock_size)};

        let (volume, header, gbd_head, global_buf, zero_block, rbd_head, end) =
            unsafe { volset_layout.calculate_offsets(volumes_start) };
        {
            //Read header section from db file.
            let (vollab, map) = {
                let mut file = OpenOptions::new()
                    .read(true)
                    .open(&self.file_name)
                    .map_err(|_| Error::CouldNotOpenDatabase(self.file_name.clone()))?;
                file.read_exact(unsafe {
                    from_raw_parts_mut(header.cast(), self.label.header_bytes.try_into().unwrap())
                })
                .map_err(|_| Error::CouldNotReadLableSlashMapBlock)?;

                let vollab = header.cast::<label_block>();
                let map = unsafe { vollab.add(1).cast() };
                //NOTE should this be reported as an error?
                if unsafe { (*vollab).clean } == 0 {
                    eprintln!("WARNING: Volume was not dismounted properly!");
                }
                //NOTE the C code says this was to facilitate forking.
                //I am not comfortable really comfortable forking so this may not be needed.
                unsafe {
                    rsm::bindings::partab.vol_fds[0] = file.as_raw_fd();
                }
                (vollab, map)
            };

            let gbd_blocks = unsafe {
                init_Global_Buffer_Descriptors(
                    from_raw_parts_mut(gbd_head,self.num_global_descriptor),
                        global_buf,
                        Bytes(self.label.block_size as usize)
                )};

            let vol_def = VOL_DEF {
                file_name: crate::vol_def::format_name(&self.file_name),
                vollab,

                map,
                map_dirty_flag: 0,
                first_free: map.cast(),

                global_buf,
                zero_block,
                num_gbd: gbd_blocks.len().try_into().unwrap(),
                gbd_head: gbd_blocks.as_mut_ptr(),
                gbd_hash: {
                    let mut temp = [std::ptr::null_mut(); GBD_HASH as usize + 1];
                    temp[GBD_HASH as usize] = gbd_blocks.as_mut_ptr();
                    temp
                },
                rbd_head,
                rbd_end: end,
                rbd_hash: {
                    let mut temp = [std::ptr::null_mut(); RBD_HASH as usize + 1];
                    temp[RBD_HASH as usize] = rbd_head.cast();
                    temp
                },

                shm_id: 0,
                //TODO add test that cover these bounds.
                num_of_daemons: (self.jobs / rsm::bindings::DAEMONS)
                    .clamp(rsm::bindings::MIN_DAEMONS, rsm::bindings::MAX_DAEMONS)
                    .try_into()
                    .unwrap(),
                wd_tab: [rsm::bindings::WD_TAB {
                    pid: 0,
                    doing: 0,
                    currmsg: rsm::bindings::DATA_UNION { intdata: 0 },
                }; 20],
                dismount_flag: 0,
                writelock: 0,
                upto: 0,
                dirtyQ: [std::ptr::null_mut(); 1024],
                dirtyQw: 0,
                dirtyQr: 0,
                garbQ: [0; 8192],
                garbQw: 0,
                garbQr: 0,
                jrn_next: 0,
                stats: rsm::bindings::DB_STAT {
                    dbget: 0,
                    dbset: 0,
                    dbkil: 0,
                    dbdat: 0,
                    dbord: 0,
                    dbqry: 0,
                    lasttry: 0,
                    lastok: 0,
                    logrd: 0,
                    phyrd: 0,
                    logwt: 0,
                    phywt: 0,
                    blkalloc: 0,
                    blkdeall: 0,
                    blkreorg: 0,
                    diskerrors: 0,
                },
            };
            unsafe {
                std::ptr::write(volume, vol_def);
            }
        }

        let sys_tab_description = sys_tab::SYSTAB {
            //This is used to verify the shared memeory segment
            //is mapped to the same address space in each proccess.
            address: shared_mem_segment,
            jobtab: job_tab,
            maxjob: self.jobs,
            sem_id: 0, //TODO
            #[allow(clippy::cast_possible_wrap)]
            historic: (HISTORIC_EOK | HISTORIC_OFFOK | HISTORIC_DNOK) as i32,
            #[allow(clippy::cast_possible_wrap)]
            precision: DEFAULT_PREC as i32,
            max_tt: 0,
            tt: [TRANTAB {
                from_global: VAR_U { var_cu: [0; 32] },
                from_vol: 0,
                from_uci: 0,
                to_global: VAR_U { var_cu: [0; 32] },
                to_vol: 0,
                to_uci: 0,
            }; 8],
            start_user: unsafe { libc::getuid().try_into().unwrap() }, //TODO
            lockstart: lock_tab.cast::<c_void>(),
            #[allow(clippy::cast_possible_wrap)]
            locksize: Bytes::from(self.lock_size).0 as i32,
            lockhead: std::ptr::null_mut(),
            lockfree: lock_tab,
            addoff: Bytes::from(share_size).0 as u64,
            addsize: 0,
            vol: [volume],
        };

        unsafe {
            std::ptr::write(sys_tab, sys_tab_description);
        }

        //TODO Deal with journaling
        assert!(unsafe { (*sys_tab).maxjob } == 1);
        Ok(sys_tab)
    }

    fn load_lable_info(file: &Path) -> Result<label_block, Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(file)
            .map_err(|_| Error::CouldNotOpenDatabase(file.into()))?;

        let mut label = MaybeUninit::<label_block>::zeroed();
        file.read_exact(unsafe { any_as_mut_u8_slice(&mut label) })
            .map_err(|_| Error::CouldNotReadLableBlock)?;
        let label = unsafe { label.assume_init() };
        if label.db_ver == DB_VER as u16 {
            Ok(label)
        } else {
            Err(Error::MissmachedDatabaseVerstions(label.db_ver))
            // TODO C also gives instrcutions on how to update image.
        }
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
        systab = sys_tab.cast();
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn validate_mem_seg_layout() {
        let sys_tab = Config::new(
            "temp".into(),
            NonZeroU32::new(1).unwrap(),
            Some(Megbibytes(1)),
            Some(Megbibytes(1)),
        )
        .unwrap()
        .setup_shared_mem_segemnt()
        .unwrap();

        let code = unsafe {
            rsm::bindings::INIT_Start(CString::new("temp").unwrap().into_raw(), 1, 1, 1, 0)
        };
        //NOTE INIT_start unmounts the shared meme segment after starting demons.
        unsafe {
            rsm::bindings::UTIL_Share(CString::new("temp").unwrap().into_raw());
        }

        println!("code: {code:?}");
        assert!(code == 0);
        unsafe {
            sys_tab::assert_sys_tab_eq(sys_tab, systab.cast::<sys_tab::SYSTAB>());
        }
        let mut sbuf = libc::shmid_ds {
            shm_atime: 0,
            shm_cpid: 0,
            shm_ctime: 0,
            shm_dtime: 0,
            shm_internal: std::ptr::null_mut(),
            shm_lpid: 0,
            shm_nattch: 0,
            shm_perm: libc::ipc_perm {
                _key: 0,
                uid: 0,
                gid: 0,
                cuid: 0,
                cgid: 0,
                mode: 0,
                _seq: 0,
            },
            shm_segsz: 0,
        };
        //TODO see is I should be useing the shutdown function.
        unsafe {
            //signal that the shared mem segment should be destoyed.
            libc::shmctl(libc::shmget(839_184_324, 0, 0), libc::IPC_RMID, &mut sbuf);
            //detaching shared meme segment.
            libc::shmdt(systab.cast::<libc::c_void>());
        }

        /*
        if code == 0 {
            unsafe {
                rsm::bindings::shutdown(CString::new("temp").unwrap().into_raw());
            }
        }
        */
    }
}
