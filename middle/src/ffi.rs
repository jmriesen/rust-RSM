use crate::bindings::u_char;
use crate::bindings::u_short;
use core::ffi::CStr;
pub unsafe fn sync_with_c(
    src: *mut *mut u_char,
    comp: *mut *mut u_char,
    offset:usize,
    byte_code:&[u8],
){
    use std::ptr::copy_nonoverlapping;
    // Copy over byte_code
    unsafe {copy_nonoverlapping((*byte_code).as_ptr(),*comp,byte_code.len())}
    unsafe { *comp = (*comp).add(byte_code.len()) };

    //Move source ptr;
    unsafe { (*src) = (*src).add(offset) };

}

pub fn compile_string(value:&CStr)->Vec<u8>{
    use std::iter::once;
    let bytes = value.to_bytes_with_nul();
    once(crate::bindings::OPSTR as u8)
    //length does not include null bite.
        .chain(((bytes.len()-1) as u_short).to_le_bytes())
        .chain(bytes.iter().copied())
        .collect()
}

#[cfg(test)]
pub mod test{
    pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }

}
