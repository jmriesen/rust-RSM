use std::ptr::{from_mut, from_ref};

use libc::{c_int, c_void};
use rsm::bindings::{
    jobtab, locktab, trantab, u_int, u_long, vol_def, DEFAULT_PREC, HISTORIC_DNOK, HISTORIC_EOK,
    HISTORIC_OFFOK, JOBTAB, LOCKTAB, TRANTAB, VAR_U, VOL_DEF,
};

use crate::{alloc::TabLayout, lock_tab, };

#[repr(C, packed(1))]
pub struct SYSTAB {
    /// memory address of *this* system tab
    /// used to verfity that the memeofy segment has been mounted proprely.
    pub address: *mut c_void,
    pub jobtab: *mut jobtab,
    /// maximum jobs permitted
    pub maxjob: u_int,
    /// GBD semaphore id
    pub sem_id: c_int,
    /// bitfield stroing config options
    pub historic: c_int,
    /// decimal precision
    pub precision: c_int,
    /// max TRANTAB used
    pub max_tt: c_int,
    /// translation tables
    pub tt: [trantab; rsm::bindings::MAX_TRANTAB as usize],
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
    pub vol: [*mut vol_def; rsm::bindings::MAX_VOL as usize],
    //This field was being used for alignment shananigans in the old c code.
    //Removing it since I don't want to rely on shananigans.
    //pub last_blk_used: [u_int; 1],
}

pub unsafe fn init(
    jobs: usize,
    volume: *mut VOL_DEF,
    addoff: u64,
    ptr: *mut c_void,
    layout: &TabLayout<SYSTAB, u_int, JOBTAB, LOCKTAB, (), ()>,
) -> *mut SYSTAB {
    let (sys_tab, _, job_tab, lock_tab, _, _, _) = unsafe { layout.calculate_offsets(ptr) };
    let lock_tab = lock_tab::init(lock_tab);
    let sys_tab_description = SYSTAB {
        //This is used to verify the shared memory segment
        //is mapped to the same address space in each process.
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
        locksize: unsafe { *lock_tab }.size,
        lockhead: std::ptr::null_mut(),
        lockfree: lock_tab,
        //TODO look into how this value is used.
        //I feel like passing it in is the wrong call and that I should be able to calculate it.
        //But until I understand more how it is I am going to stick to more or less what the C code does.
        addoff,
        addsize: 0,
        vol: [volume],
    };
    unsafe { sys_tab.ptr.as_mut().unwrap().write(sys_tab_description) };
    sys_tab.ptr.cast::<SYSTAB>()
}

#[cfg(test)]
pub fn assert_sys_tab_eq(left: &SYSTAB, right: &SYSTAB) {
    assert_eq!({ left.maxjob }, { right.maxjob });
    //assert_eq!({left.sem_id}, {right.sem_id});
    assert_eq!({ left.historic }, { right.historic });
    assert_eq!({ left.precision }, { right.precision });
    assert_eq!({ left.max_tt }, { right.max_tt });
    assert_eq!({left.start_user}, {right.start_user});
    assert_eq!({ left.locksize }, { right.locksize });
    assert_eq!({ left.addoff }, { right.addoff });
    assert_eq!({ left.addsize }, { right.addsize });
    //tt
    lock_tab::tests::assert_eq(
        unsafe{left.lockfree.as_ref().unwrap()},
        unsafe{right.lockfree.as_ref().unwrap()}
    );

    //comparing offsets
    assert_eq!(SYSTAB::offsets(left), SYSTAB::offsets(right));
    for i in 0..rsm::bindings::MAX_VOL as usize {
        use crate::vol_def::tests::assert_vol_def_eq;
        assert_vol_def_eq(unsafe {(*left).vol[i].as_ref().unwrap()},unsafe { (*right).vol[i].as_ref().unwrap()});
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
                relitive_ptr((*sys_tab).jobtab, base),
                relitive_ptr((*sys_tab).lockstart, base),
                relitive_ptr((*sys_tab).lockhead, base),
                relitive_ptr((*sys_tab).lockfree, base),
                (*sys_tab).vol.map(|x| relitive_ptr(x, base)),
            )
    }
}
