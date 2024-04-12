use libc::c_void;

/// calculates the offset from the base pointer, if the ptr is null returns None.
/// this function was created to help facilitate A/B testing the shared memory segment layout.
pub fn relitive_ptr<T>(ptr: *const T, base: *const c_void) -> Option<isize> {
    if ptr.is_null() {
        None
    } else {
        //NOTE I considered using byte_offset_from(base)
        //however in order to use that safely both pointer need to be in the same allocation block
        //since we this is intended to be used while A/B testing the shared memory segment layout, that "should" always hold.
        //But it is simpler to not require that constraint. (otherwise I would need to mark this function as unsafe)
        Some(ptr as isize - base as isize)
    }
}
