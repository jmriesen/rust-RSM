use std::{
    fmt::Display,
    ptr::{from_mut, null_mut},
    slice::from_raw_parts_mut,
};

use ffi::{
    jobtab, locktab, trantab, u_int, u_long, vol_def, CleanJob, DB_ViewRel, LCK_Remove,
    Routine_Detach, SQ_Close, ST_KillAll, ST_Restore, ST_newtab, DEFAULT_PREC, DO_FLAG_ATT,
    HISTORIC_DNOK, HISTORIC_EOK, HISTORIC_OFFOK, JOBTAB, LOCKTAB, MAX_SEQ_IO, MAX_TRANTAB, MAX_VOL,
    MVAR, PARTAB, RBD, TRANTAB, UCI_IS_LOCALVAR, VAR_U,
};
use libc::{c_int, c_void};

use super::{alloc::TabLayout, lock_tab, vol_def::Volume};
use crate::units::{Bytes, Pages};

#[repr(C, packed(1))]
pub struct SYSTAB {
    /// memory address of *this* system tab
    /// used to verify that the memory segment has been mounted properly.
    pub address: *mut c_void,
    pub jobtab: *mut jobtab,
    /// maximum jobs permitted
    pub maxjob: u_int,
    /// GBD semaphore id
    pub sem_id: c_int,
    /// bit field storing config options
    pub historic: c_int,
    /// decimal precision
    pub precision: c_int,
    /// max TRANTAB used
    pub max_tt: c_int,
    /// translation tables
    pub tt: [trantab; MAX_TRANTAB as usize],
    pub start_user: c_int,
    /// head of lock table
    pub lockstart: *mut c_void,
    /// size of locktab in bytes
    pub locksize: c_int,
    /// head of used locks
    pub lockhead: *mut locktab,
    /// head of lock free space
    pub lockfree: *mut locktab,
    /// offset from systab to add buff (bytes)
    pub addoff: u_long,
    /// add buffer size
    pub addsize: u_long,
    vol: [*mut vol_def; MAX_VOL as usize],
    //This field was being used for alignment shenanigans in the old c code.
    //Removing it since I don't want to rely on shenanigans.
    //pub last_blk_used: [u_int; 1],
}

impl SYSTAB {
    fn lock_size(&self) -> Bytes {
        Bytes(self.locksize as usize)
    }

    pub fn vols(&self) -> impl Iterator<Item = Option<&Volume>> {
        //NOTE into iter copies the array in order to make this iterator.
        //I initially had reservations about copying the data since
        //since I did not want self.vols and the copy to get out of sync
        //However that should not be an issue.
        //Since we are taking a &self returning a reference with the same life time
        //The barrow checker will prevent any unexpected mutations
        use ref_cast::RefCast;
        self.vol
            .into_iter()
            .map(|x| unsafe { x.as_ref() }.map(Volume::ref_cast))
    }

    pub fn jobs(&mut self) -> &mut [JOBTAB] {
        unsafe { from_raw_parts_mut(self.jobtab, self.maxjob as usize) }
    }

    #[must_use]
    pub fn get_env_index(&self, env: &str) -> Option<u8> {
        let env: VAR_U = env.try_into().unwrap();
        self.vols()
            .next()
            .unwrap()
            .unwrap()
            .label()
            .uci()
            .iter()
            .enumerate()
            .find(|(_, uci)| uci.name == env)
            .map(|(i, _)| i as u8)
    }

    pub fn get_vol(&self, name: &str) -> Option<usize> {
        self.vols()
            .enumerate()
            .filter_map(|(i, x)| x.map(|x| (i, x)))
            .find(|(_, x)| x.file_name() == name)
            .map(|(i, _)| i)
    }

    //Currently use C code so only works for systab.
    unsafe fn clean_jobs(&mut self, exclude_pid: i32) {
        let jobs = unsafe { from_raw_parts_mut(self.jobtab, self.maxjob as usize) };

        let out_dated_indexs = jobs
            .iter()
            .enumerate()
            .filter(|(_, x)| x.pid != 0)
            .filter(|(_, x)| x.pid != exclude_pid)
            .filter(|(_, x)| !is_alive(x.pid))
            .map(|(i, _)| i);

        for index in out_dated_indexs {
            unsafe { CleanJob(index as i32 + 1) };
        }
    }

    #[cfg(test)]
    #[must_use]
    pub fn to_slice(&self) -> &[u8] {
        use std::ptr::addr_of;

        let diagnostic = |pointer: *mut c_void| {
            format!("{:?}/t:{}", pointer, unsafe {
                pointer.byte_offset_from(self.address)
            })
        };
        dbg!(diagnostic(self.address.cast()));
        dbg!(diagnostic(self.jobtab.cast()));
        dbg!(diagnostic(self.lockfree.cast()));
        for volume in self.vols() {
            let volume = volume.unwrap().as_ref();
            dbg!(diagnostic(volume.vollab.cast()));
            dbg!(diagnostic(volume.map.cast()));
            dbg!(diagnostic(volume.first_free.cast()));
            dbg!(diagnostic(volume.global_buf.cast()));
            dbg!(diagnostic(volume.rbd_head.cast()));
            dbg!(diagnostic(addr_of!(volume.shm_id).cast_mut().cast()));
        }
        dbg!(diagnostic(self.vol[0].cast()));
        dbg!(diagnostic(unsafe { *self.vol[0] }.vollab.cast()));
        dbg!((
            unsafe { self.address.byte_add(self.addoff as usize) },
            self.addoff
        ));

        unsafe {
            ::std::slice::from_raw_parts(
                std::ptr::from_ref(self).cast(),
                self.addoff.try_into().unwrap(),
            )
        }
    }
}

fn is_alive(pid: i32) -> bool {
    !(unsafe { libc::kill(pid, 0) } != 0 && unsafe { *libc::__error() } == libc::ESRCH)
}

impl Display for SYSTAB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Job Table Slots:\t{}\tJobs", { self.maxjob })?;
        writeln!(
            f,
            "Lock Table Size:\t{}\tKiB\n",
            self.lock_size().kibi_floor().0
        )?;
        //TODO printf("Semaphore Array ID:\t%d\n", systab->sem_id);
        for vol in self.vols().flatten() {
            write!(f, "{vol}")?;
        }
        Ok(())
    }
}

fn clean_job(job: Option<usize>, par_tab: &mut PARTAB, sys_tab: &mut SYSTAB) {
    //I don't like this calculation.
    let job_index =
        job.unwrap_or(unsafe { par_tab.jobtab.offset_from(sys_tab.jobtab) } as usize + 1);
    unsafe { LCK_Remove(job_index as i32) }
    let mut job_tab = sys_tab.jobs()[job_index];
    for stack_layer in (1..job_tab.cur_do as usize).rev() {
        let do_frame = &mut job_tab.dostk[stack_layer];
        if job.is_some() {
            let new_tab = do_frame.newtab.cast::<ST_newtab>();
            if !new_tab.is_null() {
                unsafe { ST_Restore(new_tab) }
            }
            if (do_frame.flags & DO_FLAG_ATT as u8) != 0 && !do_frame.symbol.is_null() {
                unsafe {
                    ffi::ST_SymDet(
                        (*do_frame.routine.cast::<RBD>()).num_vars.into(),
                        do_frame.symbol,
                    );
                };
            }
        }
        if (do_frame.flags & DO_FLAG_ATT as u8) != 0 {
            unsafe { Routine_Detach(do_frame.routine.cast::<RBD>()) }
        }
    }

    if job.is_some() {
        unsafe { ST_KillAll(0, null_mut()) };
        par_tab.src_var = MVAR {
            //NOTE the C code leaves this with a value of $ECODE since they reuse the variable
            //TODO check if these get optimized into memcpy?
            name: "$ETRAP".try_into().unwrap(),
            volset: 0,
            slen: 0,
            uci: UCI_IS_LOCALVAR as u8,
            key: par_tab.src_var.key,
        };
        unsafe { ffi::ST_Kill(from_mut(&mut par_tab.src_var)) };
        par_tab.src_var.name = "$ECODE".try_into().unwrap();
        unsafe { ffi::ST_Kill(from_mut(&mut par_tab.src_var)) };
    }
    for i in 0..MAX_VOL as usize {
        if job_tab.view[i].is_null() {
            unsafe { DB_ViewRel(i as u32 + 1, job_tab.view[i]) };
            job_tab.view[i] = null_mut();
        }
    }
    job_tab.cur_do = 0;

    if job.is_some() {
        for i in 1..MAX_SEQ_IO as i32 {
            unsafe { SQ_Close(i) };
        }
        par_tab.jobtab = null_mut();
    }
    //TODO here it memsets JobTab
    //I am not sure what I want to do with this really it seems like it should become a Maybe uninit.
}

pub unsafe fn init<'a>(
    jobs: usize,
    volume: &mut Volume,
    addoff: Pages,
    ptr: *mut c_void,
    layout: &TabLayout<SYSTAB, u_int, JOBTAB, LOCKTAB, (), ()>,
) -> &'a mut SYSTAB {
    let (sys_tab, _, job_tab, lock_tab, _, _, _) = unsafe { layout.calculate_offsets(ptr) };
    let lock_tab = lock_tab::init(lock_tab);
    let sys_tab_description = SYSTAB {
        //address of self used to verify that the shared segment has been mounted correctly.
        address: ptr,
        jobtab: job_tab.ptr.cast::<JOBTAB>(),
        maxjob: jobs as u32,
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
        start_user: unsafe { libc::getuid().try_into().unwrap() },
        lockstart: from_mut(lock_tab).cast::<c_void>(),
        locksize: lock_tab.size,
        lockhead: std::ptr::null_mut(),
        lockfree: lock_tab,
        //TODO look into how this value is used.
        //I feel like passing it in is the wrong call and that I should be able to calculate it.
        //But until I understand more how it is I am going to stick to more or less what the C code does.
        addoff: Bytes::from(addoff).0 as u64,
        addsize: 0,
        vol: [from_mut(volume.as_mut())],
    };
    unsafe { sys_tab.ptr.as_mut().unwrap().write(sys_tab_description) };
    sys_tab.ptr.cast::<SYSTAB>().as_mut().unwrap()
}

#[cfg(test)]
pub fn assert_sys_tab_eq(left: &SYSTAB, right: &SYSTAB) {
    assert_eq!({ left.maxjob }, { right.maxjob });
    //assert_eq!({left.sem_id}, {right.sem_id});
    assert_eq!({ left.historic }, { right.historic });
    assert_eq!({ left.precision }, { right.precision });
    assert_eq!({ left.max_tt }, { right.max_tt });
    assert_eq!({ left.start_user }, { right.start_user });
    assert_eq!({ left.locksize }, { right.locksize });
    assert_eq!({ left.addoff }, { right.addoff });
    assert_eq!({ left.addsize }, { right.addsize });
    //tt
    lock_tab::tests::assert_eq(unsafe { left.lockfree.as_ref().unwrap() }, unsafe {
        right.lockfree.as_ref().unwrap()
    });

    //comparing offsets
    assert_eq!(SYSTAB::offsets(left), SYSTAB::offsets(right));

    for (left, right) in left.vols().zip(right.vols()) {
        use super::vol_def::tests::assert_vol_def_eq;
        match (left, right) {
            (Some(left), Some(right)) => assert_vol_def_eq(left, right),
            (None, None) => (),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
impl SYSTAB {
    fn offsets(
        sys_tab: &Self,
    ) -> (
        Option<isize>,
        Option<isize>,
        Option<isize>,
        Option<isize>,
        [Option<isize>; 1],
    ) {
        use crate::test_helper::relitive_ptr;
        let base = sys_tab.address;
        (
            relitive_ptr(sys_tab.jobtab, base),
            relitive_ptr(sys_tab.lockstart, base),
            relitive_ptr(sys_tab.lockhead, base),
            relitive_ptr(sys_tab.lockfree, base),
            sys_tab.vol.map(|x| relitive_ptr(x, base)),
        )
    }
}
