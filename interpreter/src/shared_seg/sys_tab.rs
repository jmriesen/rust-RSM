use std::{
    fmt::Display,
    ptr::{from_mut, from_ref, null_mut},
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
pub struct SystemTab {
    /// memory address of *this* system tab
    /// used to verify that the memory segment has been mounted properly.
    pub address: *mut c_void,
    pub job_tab: *mut jobtab,
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
    pub lock_start: *mut c_void,
    /// size of lock_tab in bytes
    pub locksize: c_int,
    /// head of used locks
    pub lock_head: *mut locktab,
    /// head of lock free space
    pub lock_free: *mut locktab,
    /// offset from system tab to add buff (bytes)
    pub add_off: u_long,
    /// add buffer size
    pub add_size: u_long,
    vol: [*mut vol_def; MAX_VOL as usize],
    //This field was being used for alignment shenanigans in the old c code.
    //Removing it since I don't want to rely on shenanigans.
    //pub last_blk_used: [u_int; 1],
}

impl SystemTab {
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
        unsafe { from_raw_parts_mut(self.job_tab, self.maxjob as usize) }
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
        let jobs = unsafe { from_raw_parts_mut(self.job_tab, self.maxjob as usize) };

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
    fn to_slice(&self) -> &[u8] {
        println!("{}", self.debug_layout());
        println!();
        unsafe {
            ::std::slice::from_raw_parts(
                std::ptr::from_ref(self).cast(),
                self.add_off.try_into().unwrap(),
            )
        }
    }
    #[cfg(test)]
    fn debug_layout(&self) -> String {
        let mut layout = format!("label     \tpointer\t\toffset");
        let mut diagnostic = |label: &str, pointer: *mut c_void| {
            layout.push_str(&format!("\n{label:10}\t{:?}\t:{:>7}", pointer, unsafe {
                pointer.byte_offset_from(self.address)
            }))
        };
        diagnostic("address", self.address.cast());
        diagnostic("job tab", self.job_tab.cast());
        diagnostic("lockfree", self.lock_free.cast());
        for (i, volume) in self.vols().enumerate() {
            use std::ptr::addr_of;
            let volume = volume.unwrap().as_ref();
            diagnostic(&format!("Volume {i}"), from_ref(volume).cast_mut().cast());
            diagnostic("label", volume.vollab.cast());
            diagnostic("map", volume.map.cast());
            diagnostic("first free", volume.first_free.cast());
            diagnostic("global buf", volume.global_buf.cast());
            diagnostic("rbd head", volume.rbd_head.cast());
            diagnostic("memory id", addr_of!(volume.shm_id).cast_mut().cast());
        }
        layout
    }
    pub fn from_raw(raw: &ffi::SYSTAB) -> &Self {
        unsafe { from_ref(raw).cast::<Self>().as_ref().unwrap() }
    }

    #[cfg(test)]
    pub fn assert_eq(&self, other: &Self) {
        use super::test_utils::{test_memory_segment_equality, DifferencesList};
        use ffi::VOL_DEF;
        use pretty_assertions::{assert_eq, assert_ne};
        let discrepancies = test_memory_segment_equality(self.to_slice(), other.to_slice());

        let mut expected_discrepancies = DifferencesList::new();
        expected_discrepancies.insert_int(
            self.sem_id,
            other.sem_id,
            std::mem::offset_of!(SystemTab, sem_id),
        );
        for (left, right) in self
            .vols()
            .zip(other.vols())
            .filter(|(x, y)| x.is_some() || y.is_some())
            .map(|(x, y)| (x.unwrap(), y.unwrap()))
        {
            expected_discrepancies.insert_int(
                left.shm_id(),
                right.shm_id(),
                unsafe { from_ref(left).byte_offset_from(from_ref(self)) } as usize
                    + std::mem::offset_of!(VOL_DEF, shm_id),
            );
        }
        assert_eq!(discrepancies, expected_discrepancies)
    }
}

fn is_alive(pid: i32) -> bool {
    !(unsafe { libc::kill(pid, 0) } != 0 && unsafe { *libc::__error() } == libc::ESRCH)
}

impl Display for SystemTab {
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

fn clean_job(job: Option<usize>, par_tab: &mut PARTAB, sys_tab: &mut SystemTab) {
    //I don't like this calculation.
    let job_index =
        job.unwrap_or(unsafe { par_tab.jobtab.offset_from(sys_tab.job_tab) } as usize + 1);
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
    layout: &TabLayout<SystemTab, u_int, JOBTAB, (), (), LOCKTAB>,
) -> &'a mut SystemTab {
    let (sys_tab, _, job_tab, _, _, lock_tab, _) = unsafe { layout.calculate_offsets(ptr) };
    let lock_tab = lock_tab::init(lock_tab);
    let sys_tab_description = SystemTab {
        //address of self used to verify that the shared segment has been mounted correctly.
        address: ptr,
        job_tab: job_tab.ptr.cast::<JOBTAB>(),
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
        }; 64],
        start_user: unsafe { libc::getuid().try_into().unwrap() },
        lock_start: from_mut(lock_tab).cast::<c_void>(),
        locksize: lock_tab.size,
        lock_head: std::ptr::null_mut(),
        lock_free: lock_tab,
        //TODO look into how this value is used.
        //I feel like passing it in is the wrong call and that I should be able to calculate it.
        //But until I understand more how it is I am going to stick to more or less what the C code does.
        add_off: Bytes::from(addoff).0 as u64,
        add_size: 0,
        vol: [from_mut(volume.as_mut())],
    };
    unsafe { sys_tab.ptr.as_mut().unwrap().write(sys_tab_description) };
    sys_tab.ptr.cast::<SystemTab>().as_mut().unwrap()
}

impl Eq for SystemTab {}
impl PartialEq for SystemTab {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[cfg(test)]
pub fn assert_sys_tab_eq(left: &SystemTab, right: &SystemTab) {
    assert_eq!({ left.maxjob }, { right.maxjob });
    //assert_eq!({left.sem_id}, {right.sem_id});
    assert_eq!({ left.historic }, { right.historic });
    assert_eq!({ left.precision }, { right.precision });
    assert_eq!({ left.max_tt }, { right.max_tt });
    assert_eq!({ left.start_user }, { right.start_user });
    assert_eq!({ left.locksize }, { right.locksize });
    assert_eq!({ left.add_off }, { right.add_off });
    assert_eq!({ left.add_size }, { right.add_size });
    //tt
    lock_tab::tests::assert_eq(unsafe { left.lock_free.as_ref().unwrap() }, unsafe {
        right.lock_free.as_ref().unwrap()
    });

    //comparing offsets
    assert_eq!(SystemTab::offsets(left), SystemTab::offsets(right));

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
impl SystemTab {
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
            relitive_ptr(sys_tab.job_tab, base),
            relitive_ptr(sys_tab.lock_start, base),
            relitive_ptr(sys_tab.lock_head, base),
            relitive_ptr(sys_tab.lock_free, base),
            sys_tab.vol.map(|x| relitive_ptr(x, base)),
        )
    }
}
