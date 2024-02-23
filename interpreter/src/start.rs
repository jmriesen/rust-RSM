use crate::{ MAX_GLOBAL_BUFFERS, MAX_JOBS, MAX_ROUTINE_BUFFERS};
use rsm::bindings::{ DB_VER, };
use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::Read;
use std::mem::{size_of, MaybeUninit};
use std::num::NonZeroU32;
use thiserror::Error;

unsafe fn any_as_mut_u8_slice<T: Sized>(p: &mut T) -> &mut [u8] {
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
    #[error("RSM environment is already initialized.")]
    DatabaseAllreadyInitialized,
    #[error("Unable to create shared memory section")]
    CouldNotCreateSharedMemorySection,
    #[error("Unable to create semaphore set")]
    CouldNotCreateSemaphores,
    #[error("Unable to clear semaphores")]
    CouldNotClearSemaphores,
    #[error("Unable to attach to systab correctly")]
    CouldNotAttachSysTab,
}

pub fn start(
    file_name: String,
    jobs: NonZeroU32,
    global_buffer: Option<NonZeroU32>,  //MEGBE
    routine_buffer: Option<NonZeroU32>, //MEGBE
) -> Result<(), Vec<StartError>> {
    let mut errors = vec![];

    let global_buffer = global_buffer
        .or(NonZeroU32::new(jobs.get() / 2))
        .unwrap_or(NonZeroU32::new(1).unwrap());
    let routine_buffer = routine_buffer
        .or(NonZeroU32::new(jobs.get() / 8))
        .unwrap_or(NonZeroU32::new(1).unwrap());

    if MAX_JOBS < jobs.into() {
        errors.push(StartError::InvalidNumberOfJobs);
    }
    if MAX_GLOBAL_BUFFERS < global_buffer.into() {
        errors.push(StartError::InvalidGlobalBufferSize);
    }
    if MAX_ROUTINE_BUFFERS < routine_buffer.into() {
        errors.push(StartError::InvalidRoutineBufferSize);
    }
    //TODO remove sentianal values
    if errors.is_empty() {
        let pagesize = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };

        let locksize = (jobs.get() * LOCKTAB_SIZE).next_multiple_of(pagesize as u32);

        let cfile = CString::new(file_name.clone()).unwrap();
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .read(true)
            .create_new(true)
            .open(file_name.clone())
            .map_err(|_| vec![StartError::CouldNotOpenDatabase(file_name.clone())])?;

        //TODO this unsafe code needs to be tested.
        let mut lable = MaybeUninit::<label_block>::zeroed();
        file.read_exact(unsafe { any_as_mut_u8_slice(&mut lable) })
            .map_err(|_| vec![StartError::CouldNotReadLableBlock])?;
        let lable = unsafe { lable.assume_init() };

        if lable.db_ver != DB_VER as u16 {
            return Err(vec![StartError::MissmachedDatabaseVerstions(lable.db_ver)]);
            // C also gives instrcutions on how to update image.
        }

        let shar_mem_key = unsafe { libc::ftok(cfile.as_ptr(), RSM_SYSTEM as i32) };

        if shar_mem_key == -1 {
            return Err(vec![StartError::CouldNotAccessDatabase(file_name)]);
        }

        let shar_mem_id = unsafe { shmget(shar_mem_key, 0, 0) };

        if shar_mem_id != -1 {
            return Err(vec![StartError::DatabaseAllreadyInitialized]);
        }

        //minimum gloable size
        //there is a minimum of MIN_GBD blocks for the global info.
        //but it also must be an intager number of MBYtes. rounded down.

        //TODO refactor/ clean this up. the semantic meaning is not vary clear.
        let min_global_size = lable.block_size * rsm::bindings::MIN_GBD;
        let global_buffer_size =
            u32::max(min_global_size, global_buffer.get() * MBYTE).next_multiple_of(MBYTE);
        let number_of_global_blocks = global_buffer_size / lable.block_size;

        let system_job_lock_tab_size = (size_of::<symtab_struct>()
            + (size_of::<u32>() * (jobs.get() as usize - 1)) * crate::bindings::MAX_VOL as usize
            + (size_of::<jobtab>()) * jobs.get() as usize
            + locksize as usize)
            .next_multiple_of(pagesize);

        let volset_size = (size_of::<vol_def>()
            + lable.header_bytes as usize
            + number_of_global_blocks as usize * size_of::<crate::bindings::GBD>()
            + global_buffer_size as usize
            + lable.block_size as usize
            + routine_buffer.get() as usize * MBYTE as usize)
            .next_multiple_of(pagesize);

        use libc::*;

        let share_size = system_job_lock_tab_size + volset_size;
        let shar_mem_id = unsafe {
            shmget(
                shar_mem_key,
                share_size,
                SHM_R | SHM_W | (SHM_R >> 3) | (SHM_W >> 3) | IPC_CREAT,
            )
        };

        if shar_mem_id == -1 {
            return Err(vec![StartError::CouldNotCreateSharedMemorySection]);
        }

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

        unsafe {
            systab = shmat(shar_mem_id, SHMAT_SEED, 0) as *mut SYSTAB;
        };
        if unsafe { systab as isize } == -1 {
            /* TODO semaphore
            shmctl(shar_mem_id, IPC_RMID, &sbuf);
            semctl(sem_id, 0, IPC_RMID, semvals);
            */
            return Err(vec![StartError::CouldNotAttachSysTab]);
        }

        unsafe {
            libc::memset(systab as *mut c_void, 0, share_size);
        }

        let sys_tab_address = unsafe{systab};
        let job_tab_address =
            unsafe{
                (sys_tab_address.add(1) as *mut u_int)
                    .sub(1) //address of last_block used.
                    .add((jobs.get() * MAX_VOL) as usize)
                    as *mut jobtab};
        let lock_tab_address =
            unsafe{
            job_tab_address
                    .add(jobs.get() as usize) as *mut locktab
            };

        let volume_zero_address = unsafe{sys_tab_address.byte_add(system_job_lock_tab_size) as *mut VOL_DEF};

        use crate::bindings::*;
        let system_tab =
            SYSTAB {
                //This is used to verify the shared memeory segment
                //is mapped to the same address space in each proccess.
                address: sys_tab_address as *mut c_void,
                jobtab:  job_tab_address,
                maxjob: jobs.into(),
                sem_id: 0, //TODO
                historic: (HISTORIC_EOK | HISTORIC_OFFOK | HISTORIC_EOK) as i32,
                precision: DEFAULT_PREC as i32,
                max_tt: 0,
                tt: [TRANTAB{
                    from_global:VAR_U {
                        var_cu:[0;32]
                    },
                    from_vol:0,
                    from_uci:0,
                    to_global:VAR_U{
                        var_cu:[0;32]
                    },
                    to_vol:0,
                    to_uci:0
                };8],
                start_user: 0,//TODO
                lockstart: lock_tab_address as * mut c_void,
                locksize: locksize as i32,
                lockhead: std::ptr::null_mut(),
                lockfree: lock_tab_address,
                addoff: share_size as u64,
                addsize: 0,
                vol: [std::ptr::null_mut();1],
                last_blk_used: [0;1],
            };

        unsafe {(*systab).vol[0] = volume_zero_address};

        unsafe{
            std::ptr::write(
                systab,
                system_tab,
            );
        }

        unsafe{
            std::ptr::write((*systab).lockfree,locktab{
                fwd_link: std::ptr::null_mut(),
                size:locksize as i32,
                job:-1,
                //not explictly set in the c code
                //however this entire memory reagon is memset to 0
                byte_count:0,
                key:[0;256],
                lock_count:0,
                name:VAR_U {
                    var_cu:[0;32]
                },
                uci:0,
                vol:0,
            })
        };

        let volume_name: [c_char; VOL_FILENAME_MAX as usize] = {
            let name :Vec<_>= std::fs::canonicalize(&file_name)
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
                let vollab =  volume_zero_address.byte_add(std::mem::size_of::<vol_def>()) as *mut LABEL_BLOCK;
                let map = vollab.byte_add(std::mem::size_of::<label_block>()) as *mut c_void;
                let first_free = map;
                let gbd_head = vollab.byte_add(lable.max_block as usize) as *mut GBD;
                let num_gbd = number_of_global_blocks;
                let global_buf = gbd_head.add(number_of_global_blocks as usize) as *mut c_void;
                let zero_block = global_buf.add(global_buffer_size as usize); //TODO check the math on this.
                let rbd_head = zero_block.add(lable.header_bytes as usize);
                let rbd_end = systab.byte_add(share_size).byte_sub((*systab).addsize as usize) as *mut c_void;
                std::ptr::write((*systab).vol[0],vol_def{
                    vollab,
                    map,
                    first_free,
                    gbd_head,
                    num_gbd,
                    global_buf,
                    zero_block,
                    rbd_head,
                    rbd_end,
                    shm_id : shar_mem_id,
                    map_dirty_flag: 0,
                    gbd_hash: todo!(),
                    rbd_hash: todo!(),
                    num_of_daemons: todo!(),
                    wd_tab: todo!(),
                    dismount_flag: todo!(),
                    writelock: todo!(),
                    upto: todo!(),
                    dirtyQ: todo!(),
                    dirtyQw: todo!(),
                    dirtyQr: todo!(),
                    garbQ: todo!(),
                    garbQw: todo!(),
                    garbQr: todo!(),
                    jrn_next: todo!(),
                    file_name: volume_name,
                    stats: todo!(),
                    //TODO nost specificly called out in C code so they would have been zeroed by memset.
                    
                });
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
        Ok(())
    } else {
        Err(errors)
    }
}
