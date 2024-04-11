use crate::units::{Bytes, Pages};
use core::ffi::c_void;
use core::marker::PhantomData;
use std::alloc::Layout;
use std::mem::MaybeUninit;
use std::slice::from_mut_ptr_range;
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

/// Allocates a zeroed block of memory (eventually will be from the shared memory segment see NOTE)
///NOTE Allocation shared memory via C and Rust requires more memory then my computer will allow.
/// There is probably a way around this, however until I get to oxidizing the multi process stuff it the rust code does not really need the a shared memory segment.
/// For now this just allocates a normal block of memory.
#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::result_unit_err)]
pub fn create_shared_mem(size: Bytes) -> Result<(*mut libc::c_void, i32), ()> {
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
    use std::alloc;
    let mem = unsafe { alloc::alloc_zeroed(Layout::array::<u8>(size.0).unwrap()) };
    Ok((mem.cast::<libc::c_void>(), 0))
    /*
    } else {
            Err(StartError::DatabaseAllreadyInitialized(shar_mem_key))
    }
         */
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

/// A chunk of memory returned from a `TabLayout`
/// The memory is guaranteed to have been zeroed.
pub struct Allocation<T> {
    //Consider changing this to a reference
    pub ptr: *mut MaybeUninit<T>,
    pub layout: Layout,
}

impl<T> Allocation<T> {
    /// Creates a new Allocation struct starting a cursor and moves the cursor to the end of the allocation.
    /// # Safety
    /// The cursor must be pointing to a memory regen at least as large as the layout, and that memory region must be zeroed.
    unsafe fn new(cursor: &mut *mut c_void, layout: Layout) -> Self {
        let ptr = cursor.cast();
        *cursor = cursor.byte_add(layout.size());
        Self { ptr, layout }
    }
    #[must_use]
    pub fn to_slice<'a>(self) -> &'a mut [MaybeUninit<T>] {
        unsafe { from_mut_ptr_range(self.ptr..self.ptr.byte_add(self.layout.size())) }
    }
}

impl Allocation<u8> {
    #[must_use]
    pub fn to_void_ptr(self) -> *mut c_void {
        self.ptr.cast::<c_void>()
    }
}

/// This represents the layout for a bunch of types placed one after the other.
/// NOTE This always rounds up to a whole number of page files.
pub struct TabLayout<A, B, C, D, E, F> {
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
    ///constructs a `TabLayout`
    ///The caller needs to guarantee that the provided layouts are large enough for the type parameters.
    #[must_use]
    pub unsafe fn new(
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
            a_phantom: PhantomData,
            b_phantom: PhantomData,
            c_phantom: PhantomData,
            d_phantom: PhantomData,
            e_phantom: PhantomData,
            f_phantom: PhantomData,
        }
    }

    ///Size of the tab.
    #[must_use]
    pub fn size(&self) -> Pages {
        (Bytes(self.a_layout.size())
            + Bytes(self.b_layout.size())
            + Bytes(self.c_layout.size())
            + Bytes(self.d_layout.size())
            + Bytes(self.e_layout.size())
            + Bytes(self.f_layout.size()))
        .pages_ceil()
    }

    /// Calculates where each value should start and where the end of the tab is.
    /// Safety
    /// The caller needs to ensure that the pointer points to large enough region of memory and that the memory has been zeroed.
    #[allow(clippy::many_single_char_names, clippy::type_complexity)]
    pub unsafe fn calculate_offsets(
        &self,
        mut cursor: *mut c_void,
    ) -> (
        Allocation<A>,
        Allocation<B>,
        Allocation<C>,
        Allocation<D>,
        Allocation<E>,
        Allocation<F>,
        *mut c_void,
    ) {
        let end = cursor.byte_add(Bytes::from(self.size()).0);
        (
            Allocation::<A>::new(&mut cursor, self.a_layout),
            Allocation::<B>::new(&mut cursor, self.b_layout),
            Allocation::<C>::new(&mut cursor, self.c_layout),
            Allocation::<D>::new(&mut cursor, self.d_layout),
            Allocation::<E>::new(&mut cursor, self.e_layout),
            Allocation::<F>::new(&mut cursor, self.f_layout),
            //TODO pull out end into its own function.
            end,
        )
    }
}
