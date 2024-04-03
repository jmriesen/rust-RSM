use crate::bindings::{
    jobtab, locktab, u_int, vol_def, DEFAULT_PREC, GBD, HISTORIC_DNOK, HISTORIC_EOK,
    HISTORIC_OFFOK, LABEL_BLOCK, MAX_VOL, TRANTAB, VAR_U, VOL_DEF, VOL_FILENAME_MAX,
};
use crate::{
    sys_tab,
    units::{Bytes, Megbibytes, Pages},
};
use crate::{MAX_GLOBAL_BUFFERS, MAX_JOBS, MAX_ROUTINE_BUFFERS};
use libc::c_char;
use libc::c_void;
use rsm::bindings::{label_block, systab, DB_VER, GBD_HASH, RBD_HASH};
use std::fs::OpenOptions;
use std::io::Read;
use std::mem::{size_of, MaybeUninit};
use std::num::NonZeroU32;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::path::PathBuf;
use std::ptr::{null, null_mut};
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
            TabLayout::<sys_tab::SYSTAB, u_int, jobtab, locktab, (), ()>::new(
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
            TabLayout::<vol_def, c_void, GBD, c_void, c_void, c_void>::new(
                Layout::new::<vol_def>(),
                Layout::array::<u8>(self.label.header_bytes.try_into().unwrap()).unwrap(),
                Layout::array::<GBD>(self.num_global_descriptor).unwrap(),
                Layout::array::<u8>(Bytes::from(self.global_buffer).0).unwrap(),
                Layout::array::<u8>(self.label.block_size.try_into().unwrap()).unwrap(),
                Layout::array::<u8>(Bytes::from(self.routine_buffer).0).unwrap(),
            )
        };

        let share_size = meta_data_tab.size() + volset_layout.size();

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

        let (shared_mem_segment, _shar_mem_id) = Self::create_shared_mem(share_size.into())?;
        let (sys_tab, _, job_tab, lock_tab, _, _, volumes_start) =
            unsafe { meta_data_tab.calculate_offsets(shared_mem_segment) };

        unsafe {
            std::ptr::write(
                sys_tab,
                sys_tab::SYSTAB {
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
                    vol: [std::ptr::null_mut()],
                },
            );
        }

        unsafe {
            std::ptr::write(
                (*sys_tab).lockfree,
                locktab {
                    fwd_link: std::ptr::null_mut(),
                    #[allow(clippy::cast_possible_wrap)]
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
            );
        };
        unsafe {
            {
                //volumes_start|volDef
                //-----------------
                //HEADER SECTION
                //-----------------
                //Vollab|Label_Block
                //Map| rest of section
                //-----------------
                //gbd_head | array of GBDs
                //global_buf | global buffer size
                //zero block | block size
                //rbd_head | routine buffer size
                let (vol_def_ptr, header, gbd_head, global_buf, zero_block, rbd_head, end) =
                    volset_layout.calculate_offsets(volumes_start);

                let vollab = header.cast::<label_block>();
                let map = vollab.add(1).cast();

                std::ptr::write(
                    vol_def_ptr,
                    vol_def {
                        vollab,
                        map,
                        first_free: map.cast(),
                        gbd_head,
                        num_gbd: self.num_global_descriptor.try_into().unwrap(),
                        rbd_end: end,
                        shm_id: 0,
                        file_name: crate::vol_def::format_name(&self.file_name),
                        //TODO fix these values
                        //I am just zeroinging them out for now so I can start runningn some tests.
                        map_dirty_flag: 0,
                        gbd_hash: [std::ptr::null_mut(); 1025],
                        rbd_hash: [std::ptr::null_mut(); 1024],
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
                        global_buf,
                        zero_block,
                        rbd_head,
                        //TODO nost specificly called out in C code so they would have been zeroed by memset.
                    },
                );
                unsafe { (*sys_tab).vol[0] = vol_def_ptr };
                let mut file = OpenOptions::new()
                    .read(true)
                    .open(&self.file_name)
                    .map_err(|_| Error::CouldNotOpenDatabase(self.file_name))?;
                file.read_exact(std::slice::from_raw_parts_mut(
                    (*(*sys_tab).vol[0]).vollab.cast::<u8>(),
                    self.label.header_bytes.try_into().unwrap(),
                ))
                .map_err(|_| Error::CouldNotReadLableSlashMapBlock)?;
                //NOTE should this be reported as an error?
                if (*(*(*sys_tab).vol[0]).vollab).clean == 0 {
                    eprintln!("WARNING: Volume was not dismounted properly!");
                }
                //NOTE the C code says this was to facilitate forking.
                //I am not comfortable really comfortable forking so this may not be needed.
                rsm::bindings::partab.vol_fds[0] = file.as_raw_fd();

                //TODO Deal with journaling
                assert!((*sys_tab).maxjob == 1);
                //TODO I am not sure if this is technically safe, I have not written here yet.
                let gbd_blocks =
                    std::slice::from_raw_parts_mut(unsafe { (*vol_def_ptr) }.gbd_head, unsafe {
                        (*vol_def_ptr).num_gbd as usize
                    });

                let mut cursor = unsafe { (*vol_def_ptr) }.global_buf.cast();

                for db_block in gbd_blocks.iter_mut() {
                    (*db_block).mem = cursor;
                    cursor =
                        cursor.byte_add(unsafe { (*(*vol_def_ptr).vollab).block_size as usize })
                }

                for i in (0..gbd_blocks.len() - 1) {
                    use std::ops::IndexMut;
                    gbd_blocks[i].next = gbd_blocks.index_mut(i + 1);
                }
                if let Some(last) = gbd_blocks.last_mut() {
                    last.next = null_mut();
                }
                (*vol_def_ptr).gbd_hash[GBD_HASH as usize] = (*vol_def_ptr).gbd_head;
                (*vol_def_ptr).rbd_hash[RBD_HASH as usize] = (*vol_def_ptr).rbd_head.cast();
            }
        }
        Ok(sys_tab)
    }

    fn load_lable_info(file: &Path) -> Result<label_block, Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(file)
            .map_err(|_| Error::CouldNotOpenDatabase(file.into()))?;

        //TODO this unsafe code needs to be tested.
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

    #[allow(clippy::unnecessary_wraps)]
    fn create_shared_mem(size: Bytes) -> Result<(*mut libc::c_void, i32), Error> {
        /*
            use libc::*;
            let cfile = CString::new(self.file_name.clone()).unwrap();

            let shar_mem_key = unsafe { libc::ftok(cfile.as_ptr(), RSM_SYSTEM as i32+1) }
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
            TODO implement with shared memory.
             */
        use core::alloc::Layout;
        use std::alloc;
        let mem = unsafe{alloc::alloc(Layout::array::<u8>(size.0).unwrap())};
        #[cfg(test)]
        {
            //NOTE randomizing data so that it is easier to find bugs.
            //We are initializing a lot of stuff to zero.
            //Since by default the allocation is mostly zeros
            //some bugs were being masked.
            use rand::{thread_rng, Rng};
            let mem_slice = unsafe{std::slice::from_raw_parts_mut(mem, size.0)};
            thread_rng().fill(&mut mem_slice[..]);
        }

        Ok((
            mem.cast::<libc::c_void>(),
            0,
        ))
        /*
        } else {
                Err(StartError::DatabaseAllreadyInitialized(shar_mem_key))
        }
             */
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
/*

trait CError {
fn wrap_error(self) -> Result<Self, ()>
where
Self: Sized;
}

impl CError for i32 {
    fn wrap_error(self) -> Result<Self, ()> {
        if self == -1 {
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
*/

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

use core::marker::PhantomData;
use std::alloc::Layout;

/// This represents the layout for a bunch of types placed one after the other.
/// NOTE This always rounds up to a hole number of page files.
struct TabLayout<A, B, C, D, E, F> {
    a_layout: Layout,
    b_layout: Layout,
    c_layout: Layout,
    d_layout: Layout,
    e_layout: Layout,
    f_layout: Layout,
    a_phantom: PhantomData<A>,
    b_phantom: PhantomData<B>,
    c_phantom: PhantomData<C>,
    d_phantom: PhantomData<D>,
    e_phantom: PhantomData<E>,
    f_phantom: PhantomData<F>,
}

impl<A, B, C, D, E, F> TabLayout<A, B, C, D, E, F> {
    ///constructs a TabLayout
    ///The caller needs to guarantee that the provided layouts are large enough for the type parameters.
    unsafe fn new(
        a_layout: Layout,
        b_layout: Layout,
        c_layout: Layout,
        d_layout: Layout,
        e_layout: Layout,
        f_layout: Layout,
    ) -> Self {
        Self {
            a_layout,
            b_layout,
            c_layout,
            d_layout,
            e_layout,
            f_layout,
            a_phantom: Default::default(),
            b_phantom: Default::default(),
            c_phantom: Default::default(),
            d_phantom: Default::default(),
            e_phantom: Default::default(),
            f_phantom: Default::default(),
        }
    }

    ///Size of the tab.
    fn size(&self) -> Pages {
        (Bytes(self.a_layout.size())
            + Bytes(self.b_layout.size())
            + Bytes(self.c_layout.size())
            + Bytes(self.d_layout.size())
            + Bytes(self.e_layout.size())
            + Bytes(self.f_layout.size()))
        .pages_ceil()
    }

    /// Calculates where each value should start and where the end of the tab is.
    /// The caller needs to ensure that the pointer points to large enough region of memory.
    unsafe fn calculate_offsets(
        &self,
        mut cursor: *mut c_void,
    ) -> (*mut A, *mut B, *mut C, *mut D, *mut E, *mut F, *mut c_void) {
        let a = cursor.cast::<A>();
        cursor = cursor.byte_add(self.a_layout.size());
        let b = cursor.cast::<B>();
        cursor = cursor.byte_add(self.b_layout.size());
        let c = cursor.cast::<C>();
        cursor = cursor.byte_add(self.c_layout.size());
        let d = cursor.cast::<D>();
        cursor = cursor.byte_add(self.d_layout.size());
        let e = cursor.cast::<E>();
        cursor = cursor.byte_add(self.e_layout.size());
        let f = cursor.cast::<F>();
        let end = a.cast::<c_void>().byte_add(Bytes::from(self.size()).0);
        (a, b, c, d, e, f, end)
    }
}