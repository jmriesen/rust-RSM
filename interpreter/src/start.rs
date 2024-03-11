use crate::{sys_tab, units::*};
use crate::{MAX_GLOBAL_BUFFERS, MAX_JOBS, MAX_ROUTINE_BUFFERS};
use rsm::bindings::{label_block, DB_VER, RSM_SYSTEM, SHMAT_SEED, systab};
use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::Read;
use std::mem::{size_of, MaybeUninit};
use std::num::NonZeroU32;
use std::path::Path;
use thiserror::Error;

pub unsafe fn any_as_mut_u8_slice<T: Sized>(p: &mut T) -> &mut [u8] {
    ::std::slice::from_raw_parts_mut((p as *mut T) as *mut u8, ::std::mem::size_of::<T>())
}

#[derive(Error, Debug)]
pub enum StartError {
    #[error("Number of jobs must not exceed {}", MAX_JOBS)]
    InvalidNumberOfJobs,
    #[error("Global buffer size must not exceed {} MiB", MAX_GLOBAL_BUFFERS)]
    InvalidGlobalBufferSize,
    #[error("Routine buffer size must not exceed {} MiB", MAX_ROUTINE_BUFFERS)]
    InvalidRoutineBufferSize,
    #[error("open of database {} failed",.0)]
    CouldNotOpenDatabase(String),
    #[error("Read of label block failed")]
    CouldNotReadLableBlock,
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

pub struct StartConfig {
    file_name: String,
    jobs: u32,
    global_buffer: Megbibytes,
    ///The numbber of blocks in the global buffer
    num_global_descriptor: usize,
    routine_buffer: Megbibytes,
    lock_size: Pages,
    label: label_block,
}

impl StartConfig {
    pub fn new(
        file_name: String,
        //TODO think about what 0 jobs would mean.
        jobs: NonZeroU32,
        global_buffer: Option<Megbibytes>,
        routine_buffer: Option<Megbibytes>,
    ) -> Result<StartConfig, Vec<StartError>> {
        let mut errors = vec![];

        let mut global_buffer =
            global_buffer.unwrap_or(Megbibytes((jobs.get() / 2).max(1) as usize));
        let routine_buffer = routine_buffer.unwrap_or(Megbibytes((jobs.get() / 8).max(1) as usize));

        if MAX_JOBS < jobs.into() {
            errors.push(StartError::InvalidNumberOfJobs);
        }
        if Megbibytes(MAX_GLOBAL_BUFFERS as usize) < global_buffer {
            errors.push(StartError::InvalidGlobalBufferSize);
        }
        if Megbibytes(MAX_ROUTINE_BUFFERS as usize) < routine_buffer {
            errors.push(StartError::InvalidRoutineBufferSize);
        }
        let label_load = Self::load_lable_info(&Path::new(&file_name));

        if let Ok(label) = label_load
            && errors.is_empty()
        {
            use rsm::bindings::{MIN_GBD,LOCKTAB_SIZE};
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

    pub fn setup_shared_mem_segemnt(self) -> Result<*mut sys_tab::SYSTAB, StartError> {
        use std::alloc::Layout;
        let systab_layout = Layout::new::<sys_tab::SYSTAB>();
        let todo_what_is_this = Layout::array::<u_int>((self.jobs * MAX_VOL) as usize).unwrap();
        let job_tabs_layout = Layout::array::<jobtab>(self.jobs as usize).unwrap();

        let meta_data_tab_size = Bytes(
            systab_layout.size()
                + todo_what_is_this.size()
                + job_tabs_layout.size()
                + Bytes::from(self.lock_size).0 as usize,
        )
        .pages_ceil();

        let global_buffer_descriptors =
            Layout::array::<GBD>(self.num_global_descriptor as usize).unwrap();
        let volset_size = Bytes(
            size_of::<vol_def>()
                + self.label.header_bytes as usize
                + global_buffer_descriptors.size()
                + Bytes::from(self.global_buffer).0
                + self.label.block_size as usize
                + Bytes::from(self.routine_buffer).0,
        )
        .pages_ceil();

        let share_size = meta_data_tab_size + volset_size;

        /*
            let sem_id = unsafe{semget(
            shar_mem_key,
            rsm::bindings::SEM_MAX as i32,
            SHM_R | SHM_W | (SHM_R >> 3) | (SHM_W >> 3) | IPC_CREAT
        )};
            TODO semephores
            let sbuf :shmid_ds;
            if sem_id < 0 {
            unsafe {shmctl(shar_mem_id, IPC_RMID, &sbuf)};
            return Err(vec![StartError::CouldNotCreateSemaphores])
        }

            let cleared = semctl(sem_id, 0, SETALL, semvals);

            if cleared ==-1{
            shmctl(shar_mem_id, IPC_RMID, &sbuf);
            semctl(sem_id, 0, IPC_RMID, semvals);
            return Err(vec![StartError::CouldNotClearSemaphores])
        }

             */

        /*
        Shared memory segement layout.
        system tab (size of system tab - vol, last_block_used)
        Volume data [mut vol_def;jobs*MAX_VOLS (ie 1)] NOTE space is caluclated uisng last_blk_used I am not 100% sure the c-rust type converstion is right right.
        JobTab [jobtab;jobs]
        LockTab
        Volume Data
         */

        //TODO clean up of shared memory on errors.

        let (shared_mem_segment, shar_mem_id) = self.create_shared_mem(share_size.into())?;
        let job_tab = unsafe {
            shared_mem_segment
                .byte_add(systab_layout.size())
                .byte_add(todo_what_is_this.size()) as *mut jobtab
        };
        let lock_tab = unsafe { job_tab.byte_add(job_tabs_layout.size()) as *mut locktab };
        let volumes_start = unsafe {
            shared_mem_segment.byte_add(Bytes::from(meta_data_tab_size).0) as *mut VOL_DEF
        };
        let sys_tab = shared_mem_segment as *mut sys_tab::SYSTAB;

        use crate::bindings::*;
        use libc::c_void;
        let system_tab = sys_tab::SYSTAB {
            //This is used to verify the shared memeory segment
            //is mapped to the same address space in each proccess.
            address: shared_mem_segment,
            jobtab: job_tab,
            maxjob: self.jobs.into(),
            sem_id: 0, //TODO
            historic: (HISTORIC_EOK | HISTORIC_OFFOK | HISTORIC_EOK) as i32,
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
            start_user: 0, //TODO
            lockstart: lock_tab as *mut c_void,
            locksize: Bytes::from(self.lock_size).0 as i32,
            lockhead: std::ptr::null_mut(),
            lockfree: lock_tab,
            addoff: Bytes::from(share_size).0 as u64,
            addsize: 0,
            vol: [std::ptr::null_mut(); 1],
            //TODO consider switching over to rust verstion of the struct.
        };

        unsafe { (*sys_tab).vol[0] = volumes_start };

        unsafe {
            std::ptr::write(sys_tab, system_tab);
        }

        unsafe {
            std::ptr::write(
                (*sys_tab).lockfree,
                locktab {
                    fwd_link: std::ptr::null_mut(),
                    size: Bytes::from(self.lock_size).0 as i32,
                    job: -1,
                    //not explictly set in the c code
                    //however this entire memory reagon is memset to 0
                    byte_count: 0,
                    key: [0; 256],
                    lock_count: 0,
                    name: VAR_U { var_cu: [0; 32] },
                    uci: 0,
                    vol: 0,
                },
            )
        };

        //TODO factor out.
        use libc::c_char;
        let volume_name: [c_char; VOL_FILENAME_MAX as usize] = {
            let name: Vec<_> = std::fs::canonicalize(&self.file_name)
                //Convert into a string
                //TODO NOTE rust strings must be valid UTF-8.
                //C strings do not have this restriction.
                //I am not going to wory about this right now.
                .unwrap()
                .to_str()
                .unwrap()
                .bytes()
                .map(|x| x as c_char)
                //Take the last N charactors of the path.
                .rev()
                .take(VOL_FILENAME_MAX as usize)
                .rev()
                //Padd out the rest of the array with zeros.
                .chain(std::iter::repeat(0))
                .take(VOL_FILENAME_MAX as usize)
                .collect();
            name.try_into().unwrap()
        };

        unsafe {
            {
                let mut cursor = volumes_start;

                cursor = cursor.byte_add(size_of::<vol_def>());

                let vollab = cursor as *mut LABEL_BLOCK;
                cursor = cursor.byte_add(size_of::<label_block>());

                let map = cursor as *mut c_void;
                let first_free = map;
                let gbd_head = vollab.byte_add(self.label.max_block as usize) as *mut GBD;
                let num_gbd = self.num_global_descriptor;
                let global_buf = gbd_head.add(self.num_global_descriptor as usize) as *mut c_void;
                let zero_block = global_buf.add(Bytes::from(self.global_buffer).0 as usize); //TODO check the math on this.
                let rbd_head = zero_block.add(self.label.header_bytes as usize);
                let rbd_end =sys_tab
                    .byte_add(Bytes::from(share_size).0)
                    .byte_sub((*sys_tab).addsize as usize)
                    as *mut c_void;
                //TODO I have not handled the journaling case yet.
                assert!((*sys_tab).maxjob == 1);
                std::ptr::write(
                    (*sys_tab).vol[0],
                    vol_def {
                        vollab,
                        map,
                        first_free,
                        gbd_head,
                        num_gbd: num_gbd.try_into().unwrap(),
                        global_buf,
                        zero_block,
                        rbd_head,
                        rbd_end,
                        shm_id: shar_mem_id,
                        file_name: volume_name,
                        //TODO fix these values
                        //I am just zeroinging them out for now so I can start runningn some tests.
                        map_dirty_flag: 0,
                        gbd_hash: [std::ptr::null_mut(); 1025],
                        rbd_hash: [std::ptr::null_mut(); 1024],
                        num_of_daemons: 0,
                        wd_tab: [WD_TAB {
                            pid: 0,
                            doing: 0,
                            currmsg: DATA_UNION { intdata: 0 },
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
                        stats: DB_STAT {
                            dbget: 0,
                            dbset: 0,
                            dbkil: 0,
                            dbdat: 0,
                            dbord: 0,
                            dbqry: 0,
                            lasttry:0,
                            lastok:0,
                            logrd: 0,
                            phyrd: 0,
                            logwt: 0,
                            phywt: 0,
                            blkalloc: 0,
                            blkdeall: 0,
                            blkreorg: 0,
                            diskerrors: 0,
                        },
                        //TODO nost specificly called out in C code so they would have been zeroed by memset.
                    },
                );
            }
        }

        //TODO set Vol[0] correctly.

        //TODO Prefome error handleing.
        //TODO Savefilename into the volume
        //create two shared memeory segments.
        //One for header/locks/jobs/Volset
        //One for semephores.
        //TODO not these require clean up/error handleing.

        //debug informaiton printed here.
        //--evertying above this is just verifying we can initat the database.
        /*
            unsafe {
            INIT_Start(
            cfile.into_raw(),
            jobs.into(),
            global_buffer.into(),
            routine_buffer.into(),
            additional_buffer,
        );
        };
             */
        Ok(sys_tab)
    }

    fn load_lable_info(file: &Path) -> Result<label_block, StartError> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(file)
            .map_err(|_| StartError::CouldNotOpenDatabase(file.to_string_lossy().into()))?;

        //TODO this unsafe code needs to be tested.
        let mut label = MaybeUninit::<label_block>::zeroed();
        file.read_exact(unsafe { any_as_mut_u8_slice(&mut label) })
            .map_err(|_| StartError::CouldNotReadLableBlock)?;
        let label = unsafe { label.assume_init() };
        if label.db_ver != DB_VER as u16 {
            Err(StartError::MissmachedDatabaseVerstions(label.db_ver))
            // C also gives instrcutions on how to update image.
        } else {
            Ok(label)
        }
    }

    fn create_shared_mem(&self, size: Bytes) -> Result<(*mut libc::c_void, i32), StartError> {
        use libc::*;
        let cfile = CString::new(self.file_name.clone()).unwrap();

        let shar_mem_key = unsafe { libc::ftok(cfile.as_ptr(), RSM_SYSTEM as i32) }
            .wrap_error()
            .map_err(|_| StartError::CouldNotAccessDatabase(self.file_name.clone()))?;

        //Check that the shared memeory segment has not allready be initialized.
        if unsafe { shmget(shar_mem_key, 0, 0) } == -1 {
            let shar_mem_id = unsafe {
                shmget(
                    shar_mem_key,
                    size.0,
                    SHM_R | SHM_W | (SHM_R >> 3) | (SHM_W >> 3) | IPC_CREAT,
                )
            }
            .wrap_error()
            .map_err(|_| StartError::CouldNotCreateSharedMemorySection)?;

            let address = unsafe { shmat(shar_mem_id, SHMAT_SEED, 0) }
                .wrap_error()
                .map_err(|_| StartError::CouldNotAttachSysTab)?;
            unsafe {
                libc::memset(address, 0, size.0);
            }
            Ok((address, shar_mem_id))
        } else {
            Err(StartError::DatabaseAllreadyInitialized(shar_mem_key))
        }
    }
}

pub fn start(
    file_name: String,
    jobs: NonZeroU32,
    global_buffer: Option<NonZeroU32>,  //MEGBE
    routine_buffer: Option<NonZeroU32>, //MEGBE
) -> Result<(), Vec<StartError>> {
    let sys_tab = StartConfig::new(
        file_name,
        jobs,
        global_buffer.map(|x| Megbibytes(x.get() as usize)),
        routine_buffer.map(|x| Megbibytes(x.get() as usize)),
    )?
    .setup_shared_mem_segemnt()
    .map_err(|x| vec![x])?;

    unsafe{
        systab = core::mem::transmute(sys_tab);
    };
    Ok(())
}

trait CError {
    fn wrap_error(self) -> Result<Self, ()>
    where
        Self: Sized;
}

impl CError for i32 {
    fn wrap_error(self) -> Result<Self, ()> {
        if self as i32 == -1 {
            Err(())
        } else {
            Ok(self)
        }
    }
}

impl CError for *mut libc::c_void {
    fn wrap_error(self) -> Result<Self, ()> {
        if self as i32 == -1 {
            Err(())
        } else {
            Ok(self)
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::units::Megbibytes;

    #[test]
    fn validate_mem_seg_layout(){
        let sys_tab = StartConfig::new(
            "temp".into(),
            NonZeroU32::new(1).unwrap(),
            Some(Megbibytes(1)),
            Some(Megbibytes(1))
        )
            .unwrap()
            .setup_shared_mem_segemnt()
            .unwrap();
        /*
        unsafe{
        rsm::bindings::INIT_Start(
        CString::new("temp").unwrap().into_raw(),
        1,
        1,
        1,
        0
    );
    }
         */
    }
}
