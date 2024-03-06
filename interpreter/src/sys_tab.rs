use libc::{c_int, c_void};
use rsm::bindings::{jobtab, locktab, trantab, u_int, u_long, vol_def};

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
    //TODO check if this is acutlly max_vol long or if c was just messing with stuff.
    pub vol: [*mut vol_def; rsm::bindings::MAX_VOL as usize],
    //This field was being used for alignment shananigans in the old c code.
    //Removing it since I don't want to rely on shananigans.
    //pub last_blk_used: [u_int; 1],
}

/*
fn assert_sys_tab_eq(left: *const SYSTAB, right: *const SYSTAB) {
let left = *left;
let right = *right;

println!("left:{:?} right:{:?}",left,right);
assert_eq!(
crate::start::any_as_mut_u8_slice(left),
crate::start::any_as_mut_u8_slice(right),
    );

        assert_eq!((*left).maxjob, (*right).maxjob);
        assert_eq!((*left).sem_id, (*right).sem_id);
        assert_eq!((*left).historic, (*right).historic);
        assert_eq!((*left).precision, (*right).precision);
        assert_eq!((*left).max_tt, (*right).max_tt);
        assert_eq!((*left).start_user, (*right).start_user);
        assert_eq!((*left).lockstart, (*right).lockstart);
        assert_eq!((*left).locksize, (*right).locksize);
        assert_eq!((*left).lockhead, (*right).lockhead);
        assert_eq!((*left).lockfree, (*right).lockfree);
        assert_eq!((*left).addoff, (*right).addoff);
        assert_eq!((*left).addsize, (*right).addsize);
        assert_eq!((*left).vol, (*right).vol);
}
         */
