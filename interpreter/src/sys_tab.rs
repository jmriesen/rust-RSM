use std::{fmt::{Display},  ptr::{from_mut}};

use libc::{c_int, c_void};
use ffi::{
    jobtab, locktab, trantab, u_int, u_long, vol_def, DEFAULT_PREC, HISTORIC_DNOK, HISTORIC_EOK, HISTORIC_OFFOK, JOBTAB, LOCKTAB, MAX_TRANTAB, MAX_VOL, TRANTAB, VAR_U, VOL_DEF
};

use crate::{alloc::TabLayout, lock_tab, units::{Bytes, Pages}, };

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
    //This field was being used for alignment shananigans in the old c code.
    //Removing it since I don't want to rely on shananigans.
    //pub last_blk_used: [u_int; 1],
}

impl SYSTAB {
    //Typed wrapper around the field
    fn lock_size(&self)->Bytes{
        Bytes(self.locksize as usize)
    }
    pub fn vols(&self)->impl Iterator<Item = Option<&Volume>>{
        //NOTE into iter copies the array in order to make this iterator.
        //I initially had reservations about copying the data since
        //since I did not want self.vols and the copy to get out of sync
        //However that should not be an issue.
        //Since we are taking and and returning a reference with the same life time
        //The barrow checker will prevent any unexpected mutations
        use ref_cast::RefCast;
        self.vol.into_iter().map(|x|  unsafe{x.as_ref()}.map(Volume::ref_cast))
    }

    #[must_use] pub fn get_env_index(&self,env: &str) -> Option<u8> {
        let env: VAR_U = env.try_into().unwrap();
        self.vols().next().unwrap().unwrap().label().uci()
            .iter()
            .enumerate()
            .find(|(_, uci)| uci.name == env)
            .map(|(i,_)| i as u8)
    }
}

impl Display for SYSTAB{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Job Table Slots:\t{}\tJobs",{self.maxjob})?;
        writeln!(f,"Lock Table Size:\t{}\tKiB\n",self.lock_size().kibi_floor().0)?;
        //TODO printf("Semaphore Array ID:\t%d\n", systab->sem_id);
        for vol in self.vols().flatten(){
            write!(f,"{}",vol)?;
        }
        Ok(())
    }
}



use crate::vol_def::Volume;

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

    for (left, right) in left.vols().zip(right.vols()){
        use crate::vol_def::tests::assert_vol_def_eq;
        match (left,right){
            (Some(left),Some(right))=>assert_vol_def_eq(left,right),
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
                relitive_ptr((*sys_tab).jobtab, base),
                relitive_ptr((*sys_tab).lockstart, base),
                relitive_ptr((*sys_tab).lockhead, base),
                relitive_ptr((*sys_tab).lockfree, base),
                (*sys_tab).vol.map(|x| relitive_ptr(x, base)),
            )
    }
}
