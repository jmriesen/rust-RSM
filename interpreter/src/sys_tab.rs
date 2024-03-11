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


fn assert_sys_tab_eq(left: *const SYSTAB, right: *const SYSTAB) {

    //primative values.
    assert_eq!(unsafe{(*left).maxjob}, unsafe{(*right).maxjob});
    assert_eq!(unsafe{(*left).sem_id}, unsafe{(*right).sem_id});
    assert_eq!(unsafe{(*left).historic}, unsafe{(*right).historic});
    assert_eq!(unsafe{(*left).precision}, unsafe{(*right).precision});
    assert_eq!(unsafe{(*left).max_tt}, unsafe{(*right).max_tt});
    assert_eq!(unsafe{(*left).start_user}, unsafe{(*right).start_user});
    assert_eq!(unsafe{(*left).locksize}, unsafe{(*right).locksize});
    assert_eq!(unsafe{(*left).lockfree}, unsafe{(*right).lockfree});
    assert_eq!(unsafe{(*left).addoff}, unsafe{(*right).addoff});


    //tt

    //comairing offsets
    assert_eq!(SYSTAB::offsets(left),SYSTAB::offsets(right));
    //let right_jobtab_offset = unsafe{(*right).jobtab - (*right).address};
    //assert_eq!(left_jobtab_offset,right_jobtab_offset);

    //jobtab
    //lockstart
    //lockhead
    //lockfree
    //vol
}

impl SYSTAB {
    fn offsets(sys_tab: *const Self)->(
        isize,
        isize,
        isize,
        isize,
    ){
        unsafe{
            (
                (*sys_tab).jobtab.byte_offset_from((*sys_tab).address),
                (*sys_tab).lockstart.byte_offset_from((*sys_tab).address),
                (*sys_tab).lockhead.byte_offset_from((*sys_tab).address),
                (*sys_tab).lockfree.byte_offset_from((*sys_tab).address),
            )
        }

    }
}
