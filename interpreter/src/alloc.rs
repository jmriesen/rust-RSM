use crate::units::{Bytes, Pages};
use core::ffi::c_void;
use core::marker::PhantomData;
use std::alloc::Layout;
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

//TODO clean up of shared memory on errors.
///NOTE Allocation shared memory via C and Rust requires more memory then my computer will allow.
/// There is probably a way around this, however until I get to oxidizing the multi process stuff it the rust code does not really need the a shared memory segment.
/// For now this just allocates a normal block of memory.
#[allow(clippy::unnecessary_wraps,clippy::result_unit_err)]
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
    let mem = unsafe { alloc::alloc(Layout::array::<u8>(size.0).unwrap()) };
    #[cfg(test)]
    {
        //NOTE randomizing data so that it is easier to find bugs.
        //We are initializing a lot of stuff to zero.
        //Since by default the allocation is mostly zeros
        //some bugs were being masked.
        use rand::{thread_rng, Rng};
        let mem_slice = unsafe { std::slice::from_raw_parts_mut(mem, size.0) };
        thread_rng().fill(&mut mem_slice[..]);
    }

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

/// This represents the layout for a bunch of types placed one after the other.
/// NOTE This always rounds up to a hole number of page files.
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
    #[must_use] pub unsafe fn new(
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
    #[must_use] pub fn size(&self) -> Pages {
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
    #[allow(clippy::many_single_char_names)]
    pub unsafe fn calculate_offsets(
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
